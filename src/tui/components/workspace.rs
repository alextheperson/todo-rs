use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListState, Paragraph},
};

use crate::tui::components::inspector::Inspector;
use crate::{
    output::RenderFormat,
    todo::{document::Document, list},
    tui::{
        components::{Component, keybinds::Keybind},
        state::{ApplicationState, Area},
    },
};

use crate::output::Render;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub list_state: ListState,
    pub list_items: Vec<ListItem>,

    pub selected_list: Option<Document>,
    pub item_idicies: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub string: String,
    pub indices: Vec<usize>,
}

impl Workspace {
    pub fn new() -> Workspace {
        let mut list_state = ListState::default();
        list_state.select_next();
        Workspace {
            list_state: list_state,
            list_items: vec![],

            selected_list: None,
            item_idicies: None,
        }
    }

    pub fn previous(state: &mut ApplicationState) {
        if state.workspace.list_state.selected().unwrap_or(0) <= 0 {
            return;
        }
        state.workspace.list_state.select_previous();
        if let Some(n) = state.workspace.list_state.selected() {
            if n > 0 {
                state.workspace.item_idicies = Some(
                    state.workspace.list_items[state.workspace.list_state.selected().unwrap()]
                        .indices
                        .clone(),
                );
            } else {
                state.workspace.item_idicies = None;
            }
        }
        Workspace::inspect(state);
    }

    pub fn next(state: &mut ApplicationState) {
        if state.workspace.list_state.selected().unwrap_or(0)
            >= (state.workspace.list_items.len() - 1)
        {
            return;
        }
        state.workspace.list_state.select_next();
        if let Some(n) = state.workspace.list_state.selected() {
            if n > 0 {
                state.workspace.item_idicies = Some(
                    state.workspace.list_items[state.workspace.list_state.selected().unwrap()]
                        .indices
                        .clone(),
                );
            } else {
                state.workspace.item_idicies = None;
            }
        }
        Workspace::inspect(state);
    }

    pub fn inspect(state: &mut ApplicationState) {
        if let Some(n) = state.workspace.list_state.selected() {
            if n > 0 {
                Inspector::inspect_item(state);
            } else {
                Inspector::inspect_document(state);
            }
        }
    }

    pub fn shift_down(state: &mut ApplicationState) {
        if let Some(i) = state.workspace.item_idicies.as_ref() {
            let depth = i.len() - 1;
        }
    }
}

impl Component for Workspace {
    fn render(frame: &mut Frame, area: Rect, state: &mut ApplicationState) -> Result<(), ()> {
        let mut block = Block::bordered().title("Workspace");

        if state.focused_area != Area::Workspace {
            block = block.border_style(Style::new().dim());
        } else {
            block = block.border_style(Style::new().blue());
        };
        frame.render_widget(&block, area);

        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(1)],
        );
        let [list_area, debug_bar] = layout.areas(block.inner(area));

        if let Some(list) = &state.workspace.selected_list {
            state.workspace.list_items = unwrap_list(list);
            let list_items = state
                .workspace
                .list_items
                .clone()
                .into_iter()
                .map(|item| item.string);

            let list_content = List::new(list_items).highlight_style(Style::new().reversed());

            frame.render_stateful_widget(list_content, list_area, &mut state.workspace.list_state);
        } else {
            let help_text = Paragraph::new("Select a .todo file on the left to edit it here.");
            frame.render_widget(help_text, list_area);
        }

        let debug_info = format!(
            "List: {} | Indicies: {:?}",
            if state.workspace.selected_list.is_some() {
                "Some"
            } else {
                "None"
            },
            state.workspace.item_idicies
        );
        frame.render_widget(debug_info, debug_bar);

        Ok(())
    }
    fn handle_keypress(event: KeyEvent, state: &mut ApplicationState) -> Result<(), ()> {
        if let KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code,
            ..
        } = event
        {
            match code {
                KeyCode::Char('d') => {
                    Workspace::shift_down(state);
                    return Ok(());
                }
                KeyCode::Char('u') => {
                    unimplemented!();
                    return Ok(());
                }
                _ => {}
            }
        }

        match event.code {
            KeyCode::Char('j') => {
                Workspace::next(state);
            }
            KeyCode::Char('k') => {
                Workspace::previous(state);
            }
            // Go to the inspector to edit the selected item
            KeyCode::Enter | KeyCode::Char(' ') => {
                Workspace::inspect(state);
                state.focused_area = Area::Inspector;
            }
            _ => {}
        };
        Ok(())
    }

    fn get_keybinds(_: ApplicationState) -> Result<Vec<Keybind>, ()> {
        Ok(vec![
            Keybind::new("j", "down"),
            Keybind::new("k", "up"),
            Keybind::new("CR", "edit"),
            Keybind::new("Space", "edit"),
            Keybind::new("C-d", "Move Up"),
            Keybind::new("C-u", "Move Down"),
        ])
    }
}

pub fn unwrap_list(doc: &Document) -> Vec<ListItem> {
    let strings = doc.format().unwrap().render(&RenderFormat::Plain);
    let start = strings.split("\n").collect::<Vec<&str>>()[0].to_string()
        + &format!(
            "\n│ {} {} {}",
            if doc.archived { "ⓐ" } else { " " },
            doc.priority,
            if let Some(date) = doc.date {
                date.display()
            } else {
                "no date".to_string()
            },
        );
    let mut items = vec![ListItem {
        string: start,
        indices: vec![],
    }];

    items.append(&mut unwrap_items(&doc.items, vec![], vec![]));

    items
}

pub fn unwrap_items(list: &list::List, indices: Vec<usize>, lines: Vec<bool>) -> Vec<ListItem> {
    let mut flat_list = vec![];

    for (i, item) in list.into_iter().enumerate() {
        let mut new_indices = indices.clone();
        new_indices.push(i);

        let at_end = i >= list.len() - 1;

        let rendered_item = item
            .format_overview(false, at_end, lines.clone())
            .unwrap()
            .render(&RenderFormat::Plain);

        flat_list.push(ListItem {
            string: rendered_item,
            indices: new_indices.clone(),
        });

        let mut new_lines = lines.clone();
        new_lines.push(at_end);
        flat_list.append(&mut unwrap_items(&item.items, new_indices, new_lines));
    }

    flat_list
}

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
};
use tui_textarea::{Input, Key, TextArea};

use crate::date::Date;
use crate::{
    todo::list::TodoList,
    tui::{
        components::{Component, keybinds::Keybind, workspace::Workspace},
        state::{ApplicationState, Area},
    },
};

#[derive(Debug, Clone)]
pub enum InspectionState {
    None,
    Item,
    Document,
}

#[derive(Debug, Clone)]
pub struct Inspector<'a> {
    selected_item: Option<usize>,
    text_boxes: Vec<TextArea<'a>>,
    inspection_state: InspectionState,
}

impl<'b> Inspector<'b> {
    pub fn new<'a>() -> Inspector<'a> {
        Inspector {
            selected_item: Some(0),
            text_boxes: vec![],
            inspection_state: InspectionState::None,
        }
    }

    pub fn next_field(&mut self) {
        match self.selected_item {
            Some(i) => {
                self.selected_item = Some(i + 1);

                if i >= self.text_boxes.len() - 1 {
                    self.selected_item = Some(0)
                }
            }
            _ => {
                self.selected_item = Some(0);
            }
        }
    }

    pub fn previous_field(&mut self) {
        match self.selected_item {
            Some(i) => {
                self.selected_item = Some(i - 1);

                if i <= 0 {
                    self.selected_item = Some(self.text_boxes.len() - 1)
                }
            }
            _ => {
                self.selected_item = Some(0);
            }
        }
    }

    pub fn inspect_item(state: &mut ApplicationState) {
        let item = state
            .workspace
            .selected_list
            .as_mut()
            .expect("Something has gone horribly wrong")
            .items
            .get_item_by_indices(state.workspace.item_idicies.as_ref().unwrap().clone());

        let date = if let Some(d) = item.date {
            d.display().to_string()
        } else {
            String::new()
        };

        state.inspector.text_boxes = vec![
            TextArea::new(vec![item.name.clone()]),
            TextArea::new(vec![item.priority.to_string()]),
            TextArea::new(vec![date]),
            TextArea::new(vec![if item.completed {
                "yes".to_string()
            } else {
                "no".to_string()
            }]),
            TextArea::new(vec![if item.archived {
                "yes".to_string()
            } else {
                "no".to_string()
            }]),
        ];
        state.inspector.selected_item = Some(0);
        state.inspector.inspection_state = InspectionState::Item;
    }

    pub fn inspect_document(state: &mut ApplicationState) {
        let document = state.workspace.selected_list.as_ref().unwrap();
        let date = if let Some(d) = document.date {
            d.display().to_string()
        } else {
            String::new()
        };

        state.inspector.text_boxes = vec![
            TextArea::new(vec![document.name.clone()]),
            TextArea::new(vec![document.priority.to_string()]),
            TextArea::new(vec![date]),
            TextArea::new(vec![if document.archived {
                "yes".to_string()
            } else {
                "no".to_string()
            }]),
        ];
        state.inspector.selected_item = Some(0);
        state.inspector.inspection_state = InspectionState::Document;
    }

    pub fn write_to_list(state: &mut ApplicationState) {
        match state.inspector.inspection_state {
            InspectionState::Item => {
                let item = state
                    .workspace
                    .selected_list
                    .as_mut()
                    .expect("Something has gone horribly wrong")
                    .items
                    .get_item_by_indices(state.workspace.item_idicies.as_ref().unwrap().clone());

                let date_string = state.inspector.text_boxes[2].lines()[0].clone();
                let date = if date_string == "" {
                    None
                } else {
                    if let Ok(d) = Date::from(&date_string) {
                        Some(d)
                    } else {
                        None
                    }
                };

                item.name = state.inspector.text_boxes[0].lines()[0].clone();
                item.priority = state.inspector.text_boxes[1].lines()[0]
                    .clone()
                    .parse::<i64>()
                    .unwrap();
                item.date = date;
                item.completed = state.inspector.text_boxes[3].lines()[0].clone() == "yes";
                item.archived = state.inspector.text_boxes[3].lines()[0].clone() == "yes";

                let _ = state.workspace.selected_list.as_ref().unwrap().save();
            }
            InspectionState::Document => {
                let item = state.workspace.selected_list.as_mut().unwrap();

                let date_string = state.inspector.text_boxes[2].lines()[0].clone();
                let date = if date_string == "" {
                    None
                } else {
                    if let Ok(d) = Date::from(&date_string) {
                        Some(d)
                    } else {
                        None
                    }
                };

                item.name = state.inspector.text_boxes[0].lines()[0].clone();
                item.priority = state.inspector.text_boxes[1].lines()[0]
                    .clone()
                    .parse::<i32>()
                    .unwrap();
                item.date = date;
                item.archived = state.inspector.text_boxes[3].lines()[0].clone() == "yes";

                let _ = state.workspace.selected_list.as_ref().unwrap().save();
            }
            InspectionState::None => {}
        }
    }
}

impl<'b> Component for Inspector<'b> {
    fn render(frame: &mut Frame, area: Rect, state: &mut ApplicationState) -> Result<(), ()> {
        let mut block = Block::bordered().title("Inspector");

        if state.focused_area != Area::Inspector {
            block = block.border_style(Style::new().dim());
        } else {
            block = block.border_style(Style::new().blue());
        };

        frame.render_widget(&block, area);

        match state.inspector.inspection_state {
            InspectionState::Item => {
                let layout = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ],
                );
                let areas: [Rect; 5] = layout.areas(block.inner(area));
                let labels = [
                    "Name",
                    "Priority",
                    "Date",
                    "Completed (yes/no)",
                    "Archived (yes/no)",
                ];

                for (i, row) in areas.into_iter().enumerate() {
                    let row_layout = Layout::new(
                        Direction::Horizontal,
                        [Constraint::Min(0), Constraint::Min(1)],
                    );
                    let [label_area, input_area] = row_layout.areas(row);

                    let label = Paragraph::new(labels[i]);
                    let mut input_line = state.inspector.text_boxes[i].clone();
                    if state.inspector.selected_item == Some(i)
                        && state.focused_area == Area::Inspector
                    {
                        input_line.set_style(Style::new().reversed());
                        input_line.set_cursor_style(Style::new().reset().underlined());
                        input_line.set_cursor_line_style(Style::default());
                    } else {
                        input_line.set_cursor_style(Style::default());
                        input_line.set_cursor_line_style(Style::default());
                    }

                    frame.render_widget(label, label_area);
                    frame.render_widget(&input_line, input_area);
                }
            }
            InspectionState::Document => {
                let layout = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ],
                );
                let areas: [Rect; 4] = layout.areas(block.inner(area));
                let labels = ["Name", "Priority", "Date", "Archived (yes/no)"];

                for (i, row) in areas.into_iter().enumerate() {
                    let row_layout = Layout::new(
                        Direction::Horizontal,
                        [Constraint::Min(0), Constraint::Min(1)],
                    );
                    let [label_area, input_area] = row_layout.areas(row);

                    let label = Paragraph::new(labels[i]);
                    let mut input_line = state.inspector.text_boxes[i].clone();
                    if state.inspector.selected_item == Some(i)
                        && state.focused_area == Area::Inspector
                    {
                        input_line.set_style(Style::new().reversed());
                        input_line.set_cursor_style(Style::new().reset().underlined());
                        input_line.set_cursor_line_style(Style::default());
                    } else {
                        input_line.set_cursor_style(Style::default());
                        input_line.set_cursor_line_style(Style::default());
                    }

                    frame.render_widget(label, label_area);
                    frame.render_widget(&input_line, input_area);
                }
            }
            InspectionState::None => {
                let help_text = Paragraph::new("No item or list has been selected yet. Select one to change its properties here.").block(block)
                    ;
                frame.render_widget(help_text, area);
            }
        }

        Ok(())
    }

    fn handle_keypress(event: KeyEvent, state: &mut ApplicationState) -> Result<(), ()> {
        if let KeyEvent {
            modifiers: KeyModifiers::ALT,
            code,
            ..
        } = event
        {
            match code {
                KeyCode::Enter => state.focused_area = Area::Workspace,
                _ => {}
            }
        };

        if let KeyEvent {
            modifiers: KeyModifiers::SHIFT,
            code,
            ..
        } = event
        {
            match code {
                KeyCode::Char('H') => {
                    Workspace::previous(state);
                    return Ok(());
                }
                KeyCode::Char('L') => {
                    Workspace::next(state);
                    return Ok(());
                }
                KeyCode::Char('J') => {
                    state.inspector.next_field();
                    return Ok(());
                }
                KeyCode::Char('K') => {
                    state.inspector.previous_field();
                    return Ok(());
                }
                // If a keybind isn't pressed, pass it on to the selected text_box
                _ => {}
            }
        }
        // We need to ignore keybinds that create newlines
        match event.into() {
            Input {
                key: Key::Char('m'),
                ctrl: true,
                alt: false,
                ..
            }
            | Input {
                key: Key::Enter, ..
            } => state.inspector.next_field(),
            _ => {
                if let Some(i) = state.inspector.selected_item {
                    state.inspector.text_boxes[i].input(event);
                    Inspector::write_to_list(state);
                }
            }
        }

        Ok(())
    }

    fn get_keybinds(_: ApplicationState) -> Result<Vec<Keybind>, ()> {
        Ok(vec![
            Keybind::new("A-CR", "finish"),
            Keybind::new("S-h,k", "edit previous"),
            Keybind::new("S-l,j", "edit next"),
        ])
    }
}

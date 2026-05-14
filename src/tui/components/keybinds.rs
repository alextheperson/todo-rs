use ratatui::crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
};

use crate::tui::{
    components::{Component, file_tree::FileTree, inspector::Inspector, workspace::Workspace},
    state::{ApplicationState, Area},
};

pub struct Keybinds {}

impl Component for Keybinds {
    fn render(frame: &mut Frame, area: Rect, state: &mut ApplicationState) -> Result<(), ()> {
        let block = Block::bordered()
            .title("Keybinds")
            .border_style(Style::new().dim());

        let binding_list = Paragraph::new(
            get_current_keybinds(state.clone())
                .into_iter()
                .map(|k| k.display())
                .collect::<Vec<String>>()
                .join(" "),
        )
        .block(block);

        frame.render_widget(binding_list, area);

        Ok(())
    }

    fn handle_keypress(_: KeyEvent, _: &mut ApplicationState) -> Result<(), ()> {
        Ok(())
    }

    fn get_keybinds(_: ApplicationState) -> Result<Vec<Keybind>, ()> {
        Ok(vec![])
    }
}

pub fn get_current_keybinds(state: ApplicationState) -> Vec<Keybind> {
    let mut keybinds = vec![
        Keybind::new("q", "quit"),
        Keybind::new("C-h,j,k,l", "move focus"),
    ];

    let area_keybinds = match state.focused_area {
        Area::FileExplorer => &mut FileTree::get_keybinds(state).expect("Couldn't get keybinds"),
        Area::Inspector => &mut Inspector::get_keybinds(state).expect("Couldn't get keybinds"),
        Area::Workspace => &mut Workspace::get_keybinds(state).expect("Couldn't get keybinds"),
        _ => unimplemented!(),
    };

    keybinds.append(area_keybinds);

    keybinds
}

pub struct Keybind {
    code: String,
    action: String,
}

impl Keybind {
    pub fn new(code: &str, action: &str) -> Keybind {
        return Keybind {
            code: code.to_string(),
            action: action.to_string(),
        };
    }

    pub fn display(&self) -> String {
        return format!("<{}> {} ", self.code, self.action);
    }
}

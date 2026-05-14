use ratatui::crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::{
    error::{CodeComponent, Error},
    match_result,
    todo::document::Document,
    tui::state::ApplicationState,
};
use keybinds::Keybind;

pub mod file_tree;
pub mod inspector;
pub mod keybinds;
pub mod status_bar;
pub mod workspace;

pub trait Component {
    fn render(frame: &mut Frame, area: Rect, state: &mut ApplicationState) -> Result<(), ()>;
    fn handle_keypress(event: KeyEvent, state: &mut ApplicationState) -> Result<(), ()>;
    fn get_keybinds(state: ApplicationState) -> Result<Vec<Keybind>, ()>;
}

pub fn open_list(path: std::path::PathBuf) -> Result<Document, Error> {
    Ok(Document::from_path(&match_result!(
        std::fs::canonicalize(&path),
        CodeComponent::Main,
        format!("Couldn't open Document at path {}", path.display())
    ))?)
}

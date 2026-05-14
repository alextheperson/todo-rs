use ratatui::crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
};

use chrono;

use crate::tui::{
    components::{Component, keybinds::Keybind},
    state::ApplicationState,
};

pub struct StatusBar {}

impl Component for StatusBar {
    fn render(frame: &mut Frame, area: Rect, state: &mut ApplicationState) -> Result<(), ()> {
        let block = Block::bordered().border_style(Style::new().dim());
        let layout = Layout::horizontal([
            Constraint::Length(10),
            Constraint::Min(10),
            Constraint::Length(19),
        ]);
        let [bin_area, path_area, time_area] = layout.areas(block.inner(area));

        let bin = Paragraph::new("todo-rs");
        let path = Paragraph::new(state.file_list.start.display().to_string());
        let time = Paragraph::new(format!("{}", chrono::offset::Local::now()));

        frame.render_widget(block, area);
        frame.render_widget(bin, bin_area);
        frame.render_widget(path, path_area);
        frame.render_widget(time, time_area);

        Ok(())
    }

    fn handle_keypress(_: KeyEvent, _: &mut ApplicationState) -> Result<(), ()> {
        Ok(())
    }

    fn get_keybinds(_: ApplicationState) -> Result<Vec<Keybind>, ()> {
        Ok(vec![])
    }
}

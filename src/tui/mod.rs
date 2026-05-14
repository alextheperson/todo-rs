use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use std::time::Duration;

use crate::error::Error;
use components::Component;
use state::ApplicationState;
use state::Area;

use components::file_tree::FileTree;
use components::inspector::Inspector;
use components::keybinds::Keybinds;
use components::status_bar::StatusBar;
use components::workspace::Workspace;

mod components;
mod state;

pub fn start_tui() -> Result<(), Error> {
    let mut terminal = ratatui::init();
    let mut application_state = ApplicationState::new();
    loop {
        terminal
            .draw(|f| draw(f, &mut application_state))
            .expect("failed to draw frame");

        if event::poll(Duration::from_secs(0)).expect("Failed to check for an event") == true {
            let event = event::read().expect("failed to read event");

            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
                if handle_focus(key_event, &mut application_state) {
                    continue;
                }
                match application_state.focused_area {
                    Area::FileExplorer => {
                        FileTree::handle_keypress(key_event, &mut application_state)
                    }
                    Area::Workspace => {
                        Workspace::handle_keypress(key_event, &mut application_state)
                    }
                    Area::Inspector => {
                        Inspector::handle_keypress(key_event, &mut application_state)
                    }

                    _ => Ok(()),
                }
                .expect("Could not handle keypress")
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn handle_focus(keypress: KeyEvent, state: &mut ApplicationState) -> bool {
    if let KeyEvent {
        modifiers: KeyModifiers::CONTROL,
        code,
        ..
    } = keypress
    {
        match code {
            KeyCode::Char('h') => {
                if state.focused_area == Area::Workspace || state.focused_area == Area::Inspector {
                    state.focused_area = Area::FileExplorer;
                }
                true
            }
            KeyCode::Char('j') => {
                if state.focused_area == Area::Workspace {
                    state.focused_area = Area::Inspector;
                }
                true
            }
            KeyCode::Char('k') => {
                if state.focused_area == Area::Inspector {
                    state.focused_area = Area::Workspace;
                }
                true
            }
            KeyCode::Char('l') => {
                if state.focused_area == Area::FileExplorer {
                    state.focused_area = Area::Workspace;
                }
                true
            }
            _ => false,
        }
    } else {
        false
    }
}

fn draw(frame: &mut Frame, state: &mut ApplicationState) {
    let main_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ]);
    let [header, body, footer] = main_layout.areas(frame.area());

    let workspace = Layout::horizontal([
        Constraint::Max(30),
        Constraint::Length(1),
        Constraint::Min(0),
    ]);
    let [explorer, _, lists] = workspace.areas(body);

    let area = Layout::vertical([Constraint::Min(0), Constraint::Max(20)]);
    let [list, inspector] = area.areas(lists);

    StatusBar::render(frame, header, state).expect("I ought to be able to render the status bar.");
    Keybinds::render(frame, footer, state).expect("I ought to be able to render the keybinds.");
    FileTree::render(frame, explorer, state).expect("I ought to be able to render the file tree.");
    Inspector::render(frame, inspector, state)
        .expect("I ought to be able to render the inspector.");
    Workspace::render(frame, list, state).expect("I ought to be able to render the workspace.");
}

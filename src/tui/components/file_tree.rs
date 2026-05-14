use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::text::{Line, Text};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListState},
};
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use crate::{
    search_paths::folder_filter,
    search_paths::has_todo_list,
    tui::{
        components::{Component, keybinds::Keybind, open_list},
        state::{ApplicationState, Area},
    },
};

#[derive(Debug, Clone)]
pub struct FileTree {
    pub list_items: Vec<FileTreeEntry>,
    pub list_state: ListState,
    pub start: PathBuf,
}

impl FileTree {
    pub fn new() -> FileTree {
        let mut list_state = ListState::default();
        list_state.select_next();

        FileTree {
            list_items: vec![],
            list_state: list_state,
            start: std::env::current_dir().unwrap(),
        }
    }

    pub fn selected_item(&self) -> FileTreeEntry {
        let selected_index = self.list_state.selected().expect("No index");
        self.list_items[selected_index].clone()
    }

    /// Get a list of the filetree
    /// path: the entry point for the list
    /// max_depth: After this depth, children will render as ellipses (...)
    /// max_size: any directory with more than this number of items will have its children rendered as
    ///           ellipses (...)
    fn get_file_tree(&mut self, path: PathBuf, max_depth: usize, max_size: usize) {
        let name = (*path
            .file_name()
            .expect("It doesn't have a name")
            .to_string_lossy())
        .to_string();

        let mut stack = vec![FileTreeEntry {
            name: name,
            path: path.clone(),
            has_list: has_todo_list(&path).expect("Could not check for a todo list."),
            condensed: false,
            nesting_bars: vec![],
            is_end: false,
            root: true,
        }];

        let read_result = fs::read_dir(&path)
            .expect("Can't read the dir!")
            .filter(|a| folder_filter(a.as_ref().expect("Uh oh")))
            .collect::<Vec<Result<DirEntry, std::io::Error>>>();

        let item_count = read_result.len();

        for (i, entry) in read_result.into_iter().enumerate() {
            let file = entry.expect("Could not read entry in directory");
            let is_last_child = i >= item_count - 1;
            let mut substack = vec![];

            FileTreeEntry::search_down_path(
                &mut substack,
                file.path(),
                &vec![],
                is_last_child,
                max_depth - 1,
                max_size,
            );

            stack.append(&mut substack);
        }

        self.list_items = stack;
    }
}

impl Component for FileTree {
    fn render(frame: &mut Frame, area: Rect, state: &mut ApplicationState) -> Result<(), ()> {
        let mut block = Block::bordered().title("Files");

        if state.focused_area != Area::FileExplorer {
            block = block.border_style(Style::new().dim());
        } else {
            block = block.border_style(Style::new().blue());
        }

        state
            .file_list
            .get_file_tree(state.file_list.start.clone(), 2, 5);

        let file_tree = state
            .file_list
            .list_items
            .iter()
            .map(|a| a.display())
            .collect::<Vec<Line>>();

        let list_content = List::new(file_tree)
            .block(block)
            .highlight_style(Style::new().reversed());

        frame.render_stateful_widget(list_content, area, &mut state.file_list.list_state);

        Ok(())
    }

    fn handle_keypress(event: KeyEvent, state: &mut ApplicationState) -> Result<(), ()> {
        match event.code {
            KeyCode::Char('j') => {
                state.file_list.list_state.select_next();
            }
            KeyCode::Char('k') => {
                state.file_list.list_state.select_previous();
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                state.workspace.selected_list =
                    Some(open_list(state.file_list.selected_item().path).unwrap());
                state.focused_area = Area::Workspace
            }
            KeyCode::Backspace => {
                let path = state.file_list.selected_item().path;
                let ancestors = path.ancestors().collect::<Vec<&Path>>();
                state.file_list.start = ancestors[1].to_path_buf();
                state.file_list.list_state.select(Some(0))
            }
            KeyCode::Char('.') => {
                state.file_list.start = state.file_list.selected_item().path;
                state.file_list.list_state.select(Some(0))
            }
            _ => {}
        };
        Ok(())
    }

    fn get_keybinds(_: ApplicationState) -> Result<Vec<Keybind>, ()> {
        Ok(vec![
            Keybind::new("j", "down"),
            Keybind::new("k", "up"),
            Keybind::new("CR,Space", "open"),
            Keybind::new("Delete", "up one level"),
            Keybind::new(".", "enter"),
        ])
    }
}

/// This is actually only directories
#[derive(Debug, Clone)]
pub struct FileTreeEntry {
    name: String,
    path: PathBuf,
    root: bool,
    has_list: bool,
    condensed: bool,
    /// When displayed as a tree, false corresponds to a vertical bar and true is a space
    nesting_bars: Vec<bool>,
    /// Whether this entry is the last child of its parent, needed for display
    is_end: bool,
}

impl FileTreeEntry {
    pub fn display(&self) -> Line<'_> {
        let style = if self.has_list {
            Style::new()
        } else {
            Style::new().dim()
        };
        if self.root {
            Line::styled(&self.name, style)
        } else {
            Line::styled(
                FileTreeEntry::format_bars(&self.nesting_bars, self.is_end) + &self.name,
                style,
            )
        }
    }
    pub fn format_bars(bars: &Vec<bool>, end: bool) -> String {
        let mut line_string = String::new();

        for level in bars.clone() {
            if level {
                line_string += "  ";
            } else {
                line_string += "│ ";
            }
        }

        if end {
            line_string += "╰ ";
        } else {
            line_string += "├ ";
        }

        line_string
    }

    pub fn search_down_path(
        stack: &mut Vec<FileTreeEntry>,
        path: PathBuf,
        bars: &Vec<bool>,
        is_end: bool,
        max_depth: usize,
        max_size: usize,
    ) {
        let read_result = fs::read_dir(&path)
            .expect("Can't read the dir!")
            .filter(|a| folder_filter(a.as_ref().expect("Uh oh")))
            .collect::<Vec<Result<DirEntry, std::io::Error>>>();
        let item_count = read_result.len();
        let too_big = item_count > max_size;

        let name = (*path
            .file_name()
            .expect("It doesn't have a name")
            .to_string_lossy())
        .to_string();

        let mut substack = vec![FileTreeEntry {
            name: format!("{} ({})", name, item_count),
            path: path.clone(),
            has_list: has_todo_list(&path).expect("Could not check for a todo list."),
            condensed: false,
            nesting_bars: bars.clone(),
            is_end: is_end,
            root: false,
        }];

        let mut new_bars = bars.clone();
        new_bars.push(is_end);

        if item_count <= 0 {
        } else if too_big || max_depth <= 0 {
            substack.push(FileTreeEntry {
                name: "...".to_string(),
                path: path.clone(),
                has_list: has_todo_list(&path).expect("Could not check for a todo list."),
                condensed: true,
                nesting_bars: new_bars,
                is_end: true,
                root: false,
            });
        } else {
            for (i, entry) in read_result.into_iter().enumerate() {
                let file = entry.expect("Could not read entry in directory");
                let is_final_child = i >= item_count - 1;

                FileTreeEntry::search_down_path(
                    &mut substack,
                    file.path(),
                    &new_bars,
                    is_final_child,
                    max_depth - 1,
                    max_size,
                );
            }
        }

        stack.append(&mut substack);
        return;
    }
}

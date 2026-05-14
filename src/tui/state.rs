use crate::tui::components::{file_tree::FileTree, inspector::Inspector, workspace::Workspace};

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Area {
    StatusBar,
    FileExplorer,
    Workspace,
    Inspector,
    Keybinds,
}

#[derive(Debug, Clone)]
pub struct ApplicationState<'a> {
    pub focused_area: Area,

    pub file_list: FileTree,
    pub workspace: Workspace,
    pub inspector: Inspector<'a>,
}

impl<'a> ApplicationState<'a> {
    pub fn new<'b>() -> ApplicationState<'b> {
        ApplicationState {
            focused_area: Area::FileExplorer,

            file_list: FileTree::new(),
            workspace: Workspace::new(),
            inspector: Inspector::new(),
        }
    }
}

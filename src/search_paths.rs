use std::{
    path::Path,
    path::PathBuf,
    fs
};

pub fn has_todo_list(path: &Path) -> bool {
    fs::exists(path.join(".todo")).unwrap_or(false)
}

pub fn search_up() -> std::iter::Rev<std::vec::IntoIter<PathBuf>> {
    let path = std::fs::canonicalize(".").unwrap();

    let mut lists : Vec<PathBuf> = vec![];

    for ancestor in path.clone().ancestors() {
        if has_todo_list(ancestor) {
            lists.push(PathBuf::from(ancestor.join(".todo")));
        }
    }

    lists.into_iter().rev()
}

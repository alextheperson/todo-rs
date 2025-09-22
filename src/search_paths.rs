use std::{fs, path::Path, path::PathBuf};

pub fn has_todo_list(path: &Path) -> bool {
    fs::exists(path.join(".todo")).unwrap_or(false)
}

pub fn search_up(path: PathBuf) -> Vec<PathBuf> {
    let mut lists: Vec<PathBuf> = vec![];

    for ancestor in path.clone().ancestors() {
        if has_todo_list(ancestor) {
            lists.push(PathBuf::from(ancestor.join(".todo")));
        }
    }

    lists.into_iter().rev().collect::<Vec<PathBuf>>()
}

pub fn search_down(path: &PathBuf) -> Vec<PathBuf> {
    let contents = fs::read_dir(path).unwrap();

    let mut lists: Vec<PathBuf> = vec![];

    for item in contents {
        let metadata = fs::metadata(item.as_ref().unwrap().path());
        if metadata.is_err() {
            continue;
        }

        let file_type = metadata.unwrap().file_type();

        if file_type.is_symlink() {
            continue;
        }

        if item.as_ref().unwrap().file_name() == "node_moduled"
            || (!(item.as_ref().unwrap().file_name() == ".todo")
                && item
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                    .starts_with("."))
        {
            continue;
        }

        if file_type.is_dir() {
            lists.append(&mut search_down(&item.unwrap().path()));
        }
    }

    if has_todo_list(path) {
        lists.push(path.join(".todo"));
    }

    lists.into_iter().rev().collect::<Vec<PathBuf>>()
}

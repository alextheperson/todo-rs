use crate::error::{CodeComponent, Error};
use crate::todo::document::Document;
use crate::{match_error, match_result, propagate};

use std::{fs, path::Path, path::PathBuf};

pub fn has_todo_list(path: &Path) -> Result<bool, Error> {
    Ok(match_result!(
        fs::exists(path.join(".todo")),
        CodeComponent::FileSearcher,
        format!(
            "Could not check for '.todo' file at path '{}'.",
            path.display()
        )
    ))
}

/// Search up through the path's ancestors
pub fn search_up(path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut lists: Vec<PathBuf> = vec![];

    for ancestor in path.clone().ancestors() {
        let full_path = match_result!(
            PathBuf::from(ancestor).canonicalize(),
            CodeComponent::FileSearcher,
            format!(
                "Could not get the canonical path of '{}'",
                ancestor.display()
            )
        );
        if match_error!(
            has_todo_list(&full_path),
            CodeComponent::FileSearcher,
            format!("Could not check path '{}'", full_path.display())
        ) {
            lists.push(full_path);
        }
    }

    Ok(lists.into_iter().rev().collect::<Vec<PathBuf>>())
}

/// Search recursively down the file tree.
pub fn search_down(path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    match fs::read_dir(path) {
        Ok(contents) => {
            let mut lists: Vec<PathBuf> = vec![];

            for item in contents {
                if let Ok(item) = item {
                    let metadata = match fs::metadata(item.path()) {
                        Ok(val) => val,
                        Err(_) => {
                            continue;
                        }
                    };

                    let file_type = metadata.file_type();

                    if file_type.is_symlink() {
                        continue;
                    }

                    /*
                    Exclude directories like `.local` or `.config`. Otherwise, the search takes a really
                    long time. I should come up with a better heuristic at some point.
                    */
                    if !(item.file_name() == ".todo")
                        && item
                            .file_name()
                            .into_string()
                            .unwrap_or("".to_string()) // Don't love this, but it works
                            .starts_with(".")
                    {
                        continue;
                    }

                    if file_type.is_dir() {
                        lists.append(&mut match_error!(
                            search_down(&item.path()),
                            CodeComponent::FileSearcher,
                            format!("Could not read dir at path '{}'", item.path().display())
                        ));
                    }
                }
            }

            if match_error!(
                has_todo_list(path),
                CodeComponent::FileSearcher,
                format!("Could not chech for '.todo' at path '{}'.", path.display())
            ) {
                lists.push(path.clone());
            }
            return Ok(lists.into_iter().rev().collect::<Vec<PathBuf>>());
        }
        Err(err) => {
            return Err(propagate!(
                CodeComponent::FileSearcher,
                format!(
                    "Could not read directory '{dir_path}': '{err}'",
                    dir_path = path.display()
                )
            ));
        }
    }
}

pub fn find_list(name: &String, down: bool) -> Result<Document, Error> {
    let search_start = match_result!(
        std::fs::canonicalize(match_result!(
            std::env::current_dir(),
            CodeComponent::FileSearcher,
            format!("Could not get the current directory.")
        )),
        CodeComponent::FileSearcher,
        format!("Could not canonicalize the current dir.")
    );

    let paths: Vec<std::path::PathBuf>;

    if down {
        paths = match_error!(
            search_down(&search_start),
            CodeComponent::FileSearcher,
            format!(
                "Could not search from down from path '{}'.",
                search_start.display()
            )
        );
    } else {
        paths = match_error!(
            search_up(&search_start),
            CodeComponent::FileSearcher,
            format!("Could not search up from path '{}'", search_start.display())
        );
    }

    for path in paths {
        let list = match_error!(
            Document::from_path(&path),
            CodeComponent::FileSearcher,
            format!("Could not parse document at path '{}'", path.display())
        );
        if list.name == *name {
            return Ok(list);
        }
    }

    Err(propagate!(
        CodeComponent::FileSearcher,
        format!("No list called '#{name}'")
    ))
}

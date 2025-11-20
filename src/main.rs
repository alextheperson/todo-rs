use output::Render;
use std::fs;
use std::path::PathBuf;
use todo::document;
use todo::list::ItemList;

pub mod commands;
mod date;
mod output;
mod search_paths;
mod todo;

use crate::output::RenderFormat;
use crate::output::line::OutputLine;
use crate::todo::document::Document;
use crate::todo::item::Item;
use crate::todo::path::ItemPath;

fn main() {
    let command = commands::build();

    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => init(
                sub_matches.get_one::<PathBuf>("FILE_PATH").unwrap_or(&std::env::current_dir().expect("You need to be in a directory.")).canonicalize()
                .expect("There needs to be a directory specified, but there was supposed to be a default value.").to_path_buf()
            ),
        Some(("next", sub_matches)) => next(
                sub_matches.get_one::<PathBuf>("FILE_PATH").unwrap_or(&std::env::current_dir().expect("You need to be in a directory.")).canonicalize()
                    .expect("There needs to be a directory specified, but there was supposed to be a default value.").to_path_buf(), 
                sub_matches.get_flag("children"),
                sub_matches.get_flag("down"),
                sub_matches.get_one::<String>("format")
                    .expect("Format must be specified, but there should have been a default value.")
                    .to_string(),
            ),
        Some(("list", sub_matches)) => list(
                sub_matches.get_flag("down"),
                sub_matches
                    .get_one::<String>("format")
                    .expect("Format must be specified, but there should have been a default value.")
                    .to_string(),
                sub_matches.get_one::<PathBuf>("FILE_PATH").unwrap_or(&std::env::current_dir().expect("You need to be in a directory.")).canonicalize()
                        .expect("There needs to be a directory specified, but there was supposed to be a default value.").to_path_buf(),

                sub_matches.get_flag("archived"),
                !sub_matches.get_flag("completed"),
            ),
        Some(("add", sub_matches)) => add(
                parse_item_path_arg(sub_matches),
                Item {
                    name : sub_matches.get_one::<String>("ITEM_NAME").expect("Expected an item name.").to_string(),
                    date: None,
                    priority: 0,
                    completed: false,
                    archived: false,
                    items: vec![]
                },
                sub_matches.get_flag("down"),
            ),
        Some(("remove", sub_matches)) => remove(
                parse_item_path_arg(sub_matches), 
                sub_matches.get_flag("down"),
            ),
        Some(("prune", sub_matches)) => prune(
                sub_matches.get_one::<PathBuf>("FILE_PATH").unwrap_or(&std::env::current_dir().expect("You need to be in a directory.")).canonicalize()
                    .expect("There needs to be a directory specified, but there was supposed to be a default value.").to_path_buf(),
                sub_matches.get_flag("single"),
                sub_matches.get_flag("down"),
            ),
        Some(("complete", sub_matches)) => complete(
                parse_item_path_arg(sub_matches),
                sub_matches.get_flag("down"),
            ),
        Some(("toggle", sub_matches)) => toggle(
                parse_item_path_arg(sub_matches),
                sub_matches.get_flag("down"),
            ),
        Some(("incomplete", sub_matches)) => incomplete(
                parse_item_path_arg(sub_matches),
                sub_matches.get_flag("down"),
            ),
        Some(("edit", _sub_matches)) => panic!("`todo edit` has not been implemented yet."),
        Some(("get", _sub_matches)) => panic!("`todo get` has not been implemented yet."),
        Some(("move", _sub_matches)) => panic!("`todo move` has not been implemented yet."),
        _ => panic!("The TUI editor has not been implemented yet."),
    }
}

/// This parses the <ITEM_PATH> arg into an ItemPath. I can't do this with clap because I can't
/// import anything into commands.rs because it is include!()ed in build.rs
fn parse_item_path_arg(matches: &clap::ArgMatches) -> ItemPath {
    let provided_path = matches
        .get_one::<String>("ITEM_PATH")
        .expect("Expected an item path.")
        .clone();

    ItemPath::try_from(&provided_path).expect(&format!(
        "Could not parse the item path '{}'.",
        &provided_path
    ))
}

fn init(path: PathBuf) {
    let todo_path = path.join(".todo");
    if fs::exists(&todo_path).unwrap_or(false) {
        println!("[LIST]: '{}' already exists.", todo_path.display());
    } else {
        fs::write(&todo_path, "# New Todo\n\n")
            .expect(&format!("Could not create '{}'.", todo_path.display()));
        println!("[LIST]: Created '{}'.", todo_path.display());
    }
}

fn next(path: PathBuf, show_children: bool, down: bool, format_string: String) {
    let format = match &format_string[..] {
        "html" => RenderFormat::HTML,
        "html-class" => RenderFormat::HtmlClass,
        "pango" => RenderFormat::Pango,
        "plain" => RenderFormat::Plain,
        _ => RenderFormat::ANSI,
    };

    let paths: Vec<PathBuf>;

    if down {
        paths = search_paths::search_down(&path);
    } else {
        paths = search_paths::search_up(path);
    }

    let mut lists = paths
        .into_iter()
        .map(|val| document::Document::from_path(&val))
        .collect::<Vec<document::Document>>();

    // Remove archived lists
    lists = lists
        .into_iter()
        .filter(|a| !a.archived)
        .rev()
        .collect::<Vec<Document>>();

    lists.sort_by(|a, b| b.priority.cmp(&a.priority));
    let mut top_list = lists[0].clone();

    top_list.items.recursive_filter(|item| item.archived);
    top_list.items.sort_by(|a, b| b.priority.cmp(&a.priority));
    let top_item = top_list.items[0].clone();

    println!(
        "{}",
        top_item
            .format_detail(show_children, false, vec![])
            .render(&format)
    );
}

fn list(
    down: bool,
    format_string: String,
    path: PathBuf,
    show_archived: bool,
    show_completed: bool,
) {
    let format = match &format_string[..] {
        "html" => RenderFormat::HTML,
        "html-class" => RenderFormat::HtmlClass,
        "pango" => RenderFormat::Pango,
        "plain" => RenderFormat::Plain,
        _ => RenderFormat::ANSI,
    };

    let search_start = path;

    let paths: Vec<PathBuf>;

    if down {
        paths = search_paths::search_down(&search_start);
    } else {
        paths = search_paths::search_up(search_start);
    }

    let mut lists = paths
        .into_iter()
        .map(|val| document::Document::from_path(&val))
        .collect::<Vec<document::Document>>();

    lists.sort_by(|a, b| b.priority.cmp(&a.priority));

    if !show_archived {
        lists = lists
            .into_iter()
            .filter(|a| !a.archived)
            .collect::<Vec<Document>>();
    }

    for mut list in lists {
        if !show_archived {
            list.items.recursive_filter(|item| item.archived)
        }
        if !show_completed {
            list.items.recursive_filter(|item| item.completed)
        }

        print!("{}", list.clone().format(list.path).render(&format));
        print!("{}", OutputLine::newline(&format));
        print!("{}", OutputLine::newline(&format));
    }
    println!("");
}

fn add(path: ItemPath, item: Item, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    list.items.add_item(item.clone(), path.clone());

    list.clone().save();

    println!(
        "[LIST]: Added '{item_name}' to #{list_name}",
        item_name = item.name,
        list_name = list.name
    );
}

fn complete(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.find(path.clone()).unwrap();
    item.completed = true;

    println!(
        "[LIST]: Completed '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

fn toggle(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.find(path.clone()).unwrap();
    item.completed = !item.completed;

    println!(
        "[LIST]: Toggled '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

fn incomplete(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.find(path.clone()).unwrap();
    item.completed = false;

    println!(
        "[LIST]: marked '{item_name}' in #{list_name} as incomplete.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

fn prune(path: PathBuf, single: bool, down: bool) {
    let search_start = path;

    let paths = if single {
        vec![search_start]
    } else {
        if down {
            search_paths::search_down(&search_start)
        } else {
            search_paths::search_up(search_start)
        }
    };

    for path in paths {
        let mut document = document::Document::from_path(&path);
        document.items.prune();
        document.save();
        println!(
            "[LIST]: Pruned #{list_name} at '{list_path}'",
            list_name = document.name,
            list_path = document.path.display()
        );
    }
}

fn remove(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.remove_by_path(path).unwrap();

    println!(
        "[LIST]: Removed '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

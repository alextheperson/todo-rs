use std::fs;
mod search_paths;
mod todo;
use todo::list::ItemList;
use todo::{document, item};
mod output;
use output::Render;

use crate::output::RenderFormat;

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or(String::from(""));

    match &cmd[..] {
        "new" => new(&mut std::env::args().skip(2)),
        "list" => list(&mut std::env::args().skip(2)),
        "add" => add(&mut std::env::args().skip(2)),
        "remove" => println!("remove"),
        "complete" => complete(&mut std::env::args().skip(2)),
        "toggle" => toggle(&mut std::env::args().skip(2)),
        "" => println!("enter"),
        _ => println!("Command '{}' not found", cmd),
    }
}

/// Create a .todo file in the current directory. Add the -f flag to overwrite an existing .todo file.
fn new(args: &mut std::iter::Skip<std::env::Args>) {
    if fs::exists("./.todo").unwrap_or(false) && args.next().unwrap_or(String::new()) != "-f" {
        println!("'.todo' already exists.");
    } else {
        fs::write("./.todo", "# New Todo\n\n").expect("'.todo' could not be created");
        println!(
            "Create '.todo' at {}",
            fs::canonicalize("./.todo").unwrap().display()
        );
    }
}

/// Display all of the active todo lists
fn list(args: &mut std::iter::Skip<std::env::Args>) {
    let options: Vec<String> = args.collect();

    let search_start = std::fs::canonicalize(".").unwrap();

    let paths: Vec<std::path::PathBuf>;

    if options.contains(&"-d".to_string()) {
        paths = search_paths::search_down(&search_start);
    } else {
        paths = search_paths::search_up(search_start);
    }

    let mut lists = paths
        .into_iter()
        .map(|val| document::Document::from_path(&val))
        .collect::<Vec<document::Document>>();

    lists.sort_by(|a, b| b.priority.cmp(&a.priority));

    let format = if options.contains(&"--html".to_string()) {
        RenderFormat::HTML
    } else if options.contains(&"--html-class".to_string()) {
        RenderFormat::HtmlClass
    } else if options.contains(&"--pango".to_string()) {
        RenderFormat::Pango
    } else if options.contains(&"--plain".to_string()) {
        RenderFormat::Plain
    } else {
        RenderFormat::ANSI
    };

    for list in lists {
        println!("{}", list.clone().format(list.path).render(&format));
    }
}

fn add(args: &mut std::iter::Skip<std::env::Args>) {
    let list_name_input = args.next().unwrap();
    let item_path = args.next().unwrap();
    let options: Vec<String> = args.collect();

    let list_name = if list_name_input.starts_with("#") {
        list_name_input[1..].to_string()
    } else {
        list_name_input
    };

    let item_path_components = item_path.split("/");

    let item_name = item_path_components
        .clone()
        .last()
        .unwrap_or("Unnamed Item");

    let new_item = item::Item {
        name: item_name.to_string(),
        priority: 0,
        date: "".to_string(),
        completed: false,
        items: vec![],
    };

    let mut list =
        search_paths::find_list(list_name.clone(), options.contains(&"-d".to_string())).unwrap();

    list.items
        .add_item(new_item, item_path_components.collect());

    list.save();

    println!("Added {item_path} to #{list_name}");
}

fn complete(args: &mut std::iter::Skip<std::env::Args>) {
    let prefix = args.next().unwrap_or(String::new());
    let options: Vec<String> = args.collect();

    let search_start = std::fs::canonicalize("./").unwrap();

    let paths: Vec<std::path::PathBuf>;

    if options.contains(&"-d".to_string()) {
        paths = search_paths::search_down(&search_start);
    } else {
        paths = search_paths::search_up(search_start);
    }

    for path in paths.into_iter().rev() {
        let mut list = document::Document::from_path(&path);
        let result = list.items.find(vec![&prefix]);

        if result.is_ok() {
            let unwrapped_item = result.unwrap();
            unwrapped_item.completed = true;
            println!(
                "Marked '{name}' in list '{list}' as complete",
                name = unwrapped_item.name,
                list = list.name
            );
            list.save();
            return;
        } else {
            println!(
                "Could not find item '{path}' in list '{list}'.",
                path = prefix.split("/").collect::<Vec<&str>>().join("*/"),
                list = list.name
            );
        }
    }
}

fn toggle(args: &mut std::iter::Skip<std::env::Args>) {
    let prefix = args.next().unwrap_or(String::new());
    let options: Vec<String> = args.collect();

    let search_start = std::fs::canonicalize(".").unwrap();

    let paths: Vec<std::path::PathBuf>;

    if options.contains(&"-d".to_string()) {
        paths = search_paths::search_down(&search_start);
    } else {
        paths = search_paths::search_up(search_start);
    }

    for path in paths.into_iter().rev() {
        let mut list = document::Document::from_path(&path);
        let result = list.items.find(vec![&prefix]);

        if result.is_ok() {
            let unwrapped_item = result.unwrap();
            unwrapped_item.completed = !unwrapped_item.completed;
            println!(
                "Toggled '{name}' in list '{list}' it is now {status}",
                name = unwrapped_item.name,
                list = list.name,
                status = if unwrapped_item.completed {
                    "complete"
                } else {
                    "incomplete"
                }
            );
            list.save();
            return;
        } else {
            println!(
                "Could not find item '{path}' in list '{list}'.",
                path = prefix.split("/").collect::<Vec<&str>>().join("*/"),
                list = list.name
            );
        }
    }
}

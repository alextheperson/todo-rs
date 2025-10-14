use std::{fs, path};
mod search_paths;
mod todo;
use clap::builder::PossibleValue;
use clap::builder::ValueParser;
use todo::document;
use todo::list::ItemList;
mod output;
use output::Render;

use crate::output::RenderFormat;
use crate::todo::item::Item;
use crate::todo::path::ItemPath;

use clap::{ArgAction, Command, arg, value_parser};

fn main() {
    let command = Command::new("todo")
        .version("1.0")
        .about("Manage the items that you need to do.\n\nWith no arguments, it opens a TUI to edit your .todo list")
                .arg(
                    arg!([PATH] "Specify the PATH of the `.todo` to edit.")
                        // .required(false)
                        .value_parser(value_parser!(std::path::PathBuf)),
                )
        .subcommand(
            Command::new("new")
                .about("Create a new `.todo` in the current directory.")
                .arg(
                    arg!([PATH] "Specify an alternate path to create the new todo.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                ),
        )
        .subcommand(
            Command::new("next")
                .about("Create a new `.tood` in the current directory.")
                .arg(
                    arg!([PATH] "An alternate path in which to look for the next todo.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List todo items for the current directory and its parents.")
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-f --format [FORMAT] "Set the output format.")
                        .action(ArgAction::Set)
                        .value_name("format")
                        .default_value("ansi")
                        .value_parser([
                            PossibleValue::new("ansi").help("Use terminal escape codes (default)."),
                            PossibleValue::new("plain").help("Use plaintext."),
                            PossibleValue::new("html").help("Use HTML with inline styles."),
                            PossibleValue::new("html-class").help(
                                "Use HTML, with no porovided styles (bring your own colors).",
                            ),
                            PossibleValue::new("pango").help("Use Pango markup (eg. for waybar)."),
                        ]),
                )
                .arg(
                    arg!([PATH] "Specify an alternate path to create the new todo.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                )
                .arg(
                    arg!(-a --archived "Show archived items.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-c --completed "Hide completed items.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("add")
                .about("Add an item to a todo list.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to add.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                    .arg(arg!(<ITEM_NAME> "The name of the item to add.")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove an item from a todo list.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to remove.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("prune")
                .about("Archive all completed todo items.")
                .arg(
                    arg!(-s --single [PATH] "Prune only a single list.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
        )
        .subcommand(
            Command::new("complete")
                .about("Mark a todo item as completed.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to complete.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("toggle")
                .about("Toggle the completion of a todo item.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to toggle.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("incomplete")
                .about("Mark a todo item as incomplete.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to mark.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("edit")
                .about("Edit the properties of a todo item.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to add.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-n --name "Set the name of the todo item.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-D --date "Set the date of the todo item.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-p --priority "Set the priority of the todo item.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-c --completed "Set whether the item is completed")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-a --archived "Set whether the item is archived")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("get")
                .about("Get info about a specific todo item.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to get.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
        )
        .subcommand(
            Command::new("move")
                .about("Move a todo item to another location.")
                .arg(
                    arg!(<TODO_FROM> "The path of the todo item to move.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                .arg(
                    arg!(<TODO_TO> "The path to move the todo item to.")
                        .value_parser(ValueParser::new(todo::path::ItemPath::parse_item_path)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        );
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => new(sub_matches
            .get_one::<std::path::PathBuf>("PATH").unwrap_or(&std::env::current_dir().expect("You need to be in a directory.")).canonicalize()
                .expect("There needs to be a directory specified, but there was supposed to be a default value.").to_path_buf()),
        Some(("next", _sub_matches)) => panic!("`todo next` has not been implemented yet."),
        Some(("list", sub_matches)) => list(
            sub_matches.get_flag("down"),
            sub_matches
                .get_one::<String>("format")
                .expect("Format must be specified, but there should have been a default value.")
                .to_string(),
            sub_matches.get_one::<std::path::PathBuf>("PATH").unwrap_or(&path::PathBuf::from("./")).canonicalize()
                    .expect("There needs to be a directory specified, but there was supposed to be a default value.").to_path_buf()
        ),
        Some(("add", sub_matches)) => add(
                sub_matches.get_one::<ItemPath>("TODO_PATH").expect("Expected an item path.").clone(),
                Item {
                    name : sub_matches.get_one::<String>("ITEM_NAME").expect("Expected an item name.").to_string(),
                    date: String::new(),
                    priority: 0,
                    completed: false,
                    items: vec![]
                },
                sub_matches.get_flag("down"),
            ),
        Some(("remove", _sub_matches)) => panic!("`todo remove` hos not been implemented yet."),
        Some(("prune", _sub_matches)) => panic!("`todo prune` hos not been implemented yet."),
        Some(("complete", sub_matches)) => complete(
                sub_matches.get_one::<ItemPath>("TODO_PATH").expect("Expected an item path.").clone(),
                sub_matches.get_flag("down"),
            ),
        Some(("toggle", sub_matches)) => toggle(
                sub_matches.get_one::<ItemPath>("TODO_PATH").expect("Expected an item path.").clone(),
                sub_matches.get_flag("down"),
            ),
        Some(("incomplete", sub_matches)) => incomplete(
                sub_matches.get_one::<ItemPath>("TODO_PATH").expect("Expected an item path.").clone(),
                sub_matches.get_flag("down"),
            ),
        Some(("edit", _sub_matches)) => panic!("`todo edit` hos not been implemented yet."),
        Some(("get", _sub_matches)) => panic!("`todo get` hos not been implemented yet."),
        Some(("move", _sub_matches)) => panic!("`todo move` hos not been implemented yet."),
        _ => panic!("The TUI editor has not been implemented yet."),
    }

    // Once I figure out how to do the man pages
    // let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
    // let man = clap_mangen::Man::new(command);
    // let mut buffer: Vec<u8> = Default::default();
    // man.render(&mut buffer)?;
    // std::fs::write(out_dir.join("todo.1"), buffer)?;
}

/// Create a .todo file in the current directory. Add the -f flag to overwrite an existing .todo file.
fn new(path: std::path::PathBuf) {
    let todo_path = path.join(".todo");
    if fs::exists(&todo_path).unwrap_or(false) {
        println!("'{}' already exists.", todo_path.display());
    } else {
        fs::write(&todo_path, "# New Todo\n\n")
            .expect(&format!("Could not create '{}'.", todo_path.display()));
        println!("Created '{}'.", todo_path.display());
    }
}

fn list(down: bool, format_string: String, path: std::path::PathBuf) {
    let format = match &format_string[..] {
        "html" => RenderFormat::HTML,
        "html-class" => RenderFormat::HtmlClass,
        "pango" => RenderFormat::Pango,
        "plain" => RenderFormat::Plain,
        _ => RenderFormat::ANSI,
    };

    let search_start = path;

    let paths: Vec<std::path::PathBuf>;

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

    for list in lists {
        println!("{}", list.clone().format(list.path).render(&format));
    }
}

fn add(path: ItemPath, item: Item, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    list.items.add_item(item.clone(), path.clone());

    list.clone().save();

    println!(
        "Added '{item_name}' to #{list_name}",
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
        "Completed '{item_name}' in #{list_name}.",
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
        "Completed '{item_name}' in #{list_name}.",
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
        "Completed '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

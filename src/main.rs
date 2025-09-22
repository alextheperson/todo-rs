use std::fs;
mod list;
mod search_paths;

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
    let options = args.next().unwrap_or(String::new());

    let search_start = std::fs::canonicalize(".").unwrap();

    let paths: Vec<std::path::PathBuf>;

    if options == "-d" {
        paths = search_paths::search_down(&search_start);
    } else {
        paths = search_paths::search_up(search_start);
    }

    for path in paths {
        println!(
            "{}",
            list::format::format_list(list::parse_list(path.clone()), path)
        );
    }
}

fn add(args: &mut std::iter::Skip<std::env::Args>) {
    let list_name_input = args.next().unwrap();
    let item_path = args.next().unwrap();

    let list_name = if list_name_input.starts_with("#") {
        list_name_input[1..].to_string()
    } else {
        list_name_input
    };

    let item_path_components = item_path.split("/");

    let parents = item_path_components
        .clone()
        .take(item_path_components.clone().count() - 1);
    let item_name = item_path_components.last().unwrap_or("Unnamed Item");

    let new_item = list::Item {
        name: item_name.to_string(),
        priority: 0,
        date: "".to_string(),
        completed: false,
        items: vec![],
    };

    let mut list = list::search::get_list(list_name.clone()).unwrap();

    let result = list::search::add_item(&mut list.items, new_item, parents.collect());

    if result {
        list::save::save_list(list);

        println!("Added {item_path} to #{list_name}");
    }
}

fn complete(args: &mut std::iter::Skip<std::env::Args>) {
    let prefix = args.next().unwrap_or(String::new());
    let search_start = std::fs::canonicalize(".").unwrap();
    let paths = search_paths::search_up(search_start);

    for path in paths {
        let mut list = list::parse_list(path.clone());
        let found = list::search::complete_item(&mut list.items, prefix.clone());

        if found {
            list::save::save_list(list.clone());
            println!("Marked '{name}*' as complete", name = prefix);
        }
    }
}

fn toggle(args: &mut std::iter::Skip<std::env::Args>) {
    let prefix = args.next().unwrap_or(String::new());
    let search_start = std::fs::canonicalize(".").unwrap();
    let paths = search_paths::search_up(search_start);

    for path in paths {
        let mut list = list::parse_list(path.clone());
        let found = list::search::toggle_item(&mut list.items, prefix.clone());

        if found {
            list::save::save_list(list.clone());
            println!("Toggeled the completion of '{name}*'", name = prefix);
        }
    }
}

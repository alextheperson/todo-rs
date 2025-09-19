use std::fs;
mod list;
mod search_paths;

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or(String::from(""));

    match &cmd[..] {
        "new" => new(std::env::args().skip(2)),
        "list" =>  list(std::env::args().skip(2)),
        "add" => println!("add"),
        "remove" => println!("remove"),
        "complete" => println!("complete"),
        "" => println!("enter"),
        _ => println!("Command '{}' not found", cmd),
    }
}

/// Create a .todo file in the current directory. Add the -f flag to overwrite an existing .todo file.
fn new (args: std::iter::Skip<std::env::Args>) {
    if fs::exists("./.todo").unwrap_or(false) && args.take(1).nth(0).unwrap_or(String::new()) != "-f" {
        println!("'.todo' already exists.");
    } else {
        fs::write("./.todo", "# New Todo\n\n").expect("'.todo' could not be created");
        println!("Create '.todo' at {}", fs::canonicalize("./.todo").unwrap().display());
    }
}

/// Display all of the active todo lists
fn list (_args: std::iter::Skip<std::env::Args>) {
    let paths = search_paths::search_up();

    for path in paths {
        list::parse_list(path);
    }
}

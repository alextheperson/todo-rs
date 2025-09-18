use std::fs;
mod list;

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
        fs::write("./.todo", "# Test Todo\n\n- [ ] Item 1\n- [x] Checked item\n - [x] Checked sub-item").expect("'.todo' could not be created");
        println!("Create '.todo' at {}", fs::canonicalize("./.todo").unwrap().display());
    }
}

/// Display all of the active todo lists
fn list (_args: std::iter::Skip<std::env::Args>) {
    let contents = fs::read_to_string("./.todo")
        .expect("Should have been able to read the file");
    
    list::parse_list(contents);
}

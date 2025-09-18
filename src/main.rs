use std::fs;

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or(String::from(""));

    match &cmd[..] {
        "new" => new(std::env::args().skip(2)),
        "add" => println!("add"),
        "remove" => println!("remove"),
        "list" =>  list(std::env::args().skip(2)),
        "" => println!("enter"),
        _ => println!("Command {:?} not found", cmd),
    }
}

/**
*  Create a .todo file in the current directory. Add the -f flag to overwrite an existing .todo file.
*/
fn new (args: std::iter::Skip<std::env::Args>) {
    if fs::exists("./.todo").unwrap_or(false) && args.take(1).nth(0).unwrap_or(String::new()) != "-f" {
        println!("'.todo' already exists.");
    } else {
    fs::write("./.todo", "# Test Todo\n\n- [ ] Item 1\n- [x] Checked item\n - [x] Checked sub-item").expect("'.todo' could not be created");
    println!("Create '.todo' at {:?}", fs::canonicalize("./.todo").unwrap());
    }
}

/**
*  Display all of the active todo lists
*/
fn list (_args: std::iter::Skip<std::env::Args>) {
    let contents = fs::read_to_string("./.todo")
        .expect("Should have been able to read the file");
    
    parse_list(contents);
}

fn parse_list(content: String) {
    let lines = content.lines();

    let name = lines.clone().nth(0).expect("Invalid '.todo'. Expected title.").get(2..).unwrap();

    let mut remaining_lines = lines.clone();
    remaining_lines.nth(1);
    let items = parse_items(remaining_lines, 0);

    println!("{}", format_list(List{
        name: name.to_string(),
        items: items
    }));
}

/**
* This parses the actual items from a list, ignoring the title, etc.
*/
fn parse_items(content: std::str::Lines, depth: u8) -> Vec<Item> {
    let mut items : Vec<Item> = vec![];

    let starting_indentation = content.clone().nth(0).unwrap().chars().count() - content.clone().nth(0).unwrap().trim_start().chars().count();

    for (i, line) in content.clone().enumerate() {
        let current_indentation = line.chars().count() - line.trim_start().chars().count();

        let mut next_indentation = 0;
        if i < content.clone().count() - 1 {
            next_indentation = content.clone().nth(i + 1).unwrap().chars().count() - content.clone().nth(i + 1).unwrap().trim_start().chars().count();
        }


        let mut sub_items: Vec<Item> = vec![];

        // If we rise out of the level that we start at
        if current_indentation < starting_indentation  {
            break;
        }

        // Skip the lower-level lines, they are going ot behandled recursively
        if current_indentation > starting_indentation {
            continue;
        }

        // We need to go deeper
        if current_indentation < next_indentation {
            let mut remaining_lines = content.clone();
            remaining_lines.nth(i);
            if remaining_lines.clone().count() > 0 {
                sub_items = parse_items(remaining_lines, depth + 1);
            }
        }

        let mut sections = line.split("-").skip(1);

        let completed = &sections.next().unwrap()[2..3] == "x";
        let priority: i16 = sections.next().unwrap_or("0").trim().parse().unwrap_or(0);
        let date = sections.next().unwrap().trim().to_string();
        // TODO: Join the rest of the sections with '-'
        let name = sections.next().unwrap().trim().to_string();
        items.push(
            Item {
                name: name,
                priority: priority,
                date: date,
                completed: completed,
                items: sub_items
            }
        );
    }

    items
}

#[derive(Debug, Clone)]
struct Item {
    completed: bool,
    priority: i16,
    date: String,
    name: String,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
struct List {
    name: String,
    items: Vec<Item>
}

fn format_list(list: List) -> String {
    let mut output = String::new();

    output += &format!("# {}\n", list.name)[..];
    output += "\n";
    output += &format!("{}", format_item_vec(list.items.clone(), 0))[..];

    output
}

fn format_item_vec(items: Vec<Item>, depth: usize) -> String {
    let mut output = String::new();

    for (i, item) in items.clone().into_iter().enumerate() {
        output += &format_item(item.clone(), depth, i == items.len() - 1)[..];
    }

    output
}

fn format_item(item: Item, depth: usize, last: bool) -> String {
    let mut output = String::new();

    if last && depth > 0 {
        if depth > 1 {
            output += &"├ ".repeat(depth - 1)[..];
        }
        output += &"╰ "[..];
    } else {
        output += &"├ ".repeat(depth)[..];
    }
    if item.date == "" {
    output += &format!("{} {} {}\n{}", if item.completed { "▣" } else { "□" }, item.priority, item.name, format_item_vec(item.items.clone(), depth + 1))[..];
    } else {
    output += &format!("{} {} {} ({})\n{}", if item.completed { "▣" } else { "□" }, item.priority, item.name, item.date, format_item_vec(item.items.clone(), depth + 1))[..];
    }

    output
}

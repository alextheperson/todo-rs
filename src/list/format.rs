use crate::list::{
    List,
    Item
};

pub fn format_list(list: List, path: std::path::PathBuf) -> String {
    let mut output = String::new();

    output += &format!("╭ # {} ({})\n", list.name, path.as_path().display())[..];
    output += "│\n";
    output += &format!("{}", format_item_vec(list.items.clone(), 1))[..];

    output
}

fn format_item_vec(items: Vec<Item>, depth: usize) -> String {
    let mut output = String::new();

    for item in items {
        output += &format_item(item.clone(), depth)[..];
    }

    output
}

fn format_item(item: Item, depth: usize) -> String {
    let mut output = String::new();

    if depth > 0 {
        // output += &"  "[..];
        if depth > 1 {
            output += &"│ ".repeat(depth - 1)[..];
        }
        output += &"├ "[..];
    }

    if item.date == "" {
        output += &format!("{box} {priority} {name}\n{children}",
            box = if item.completed { "▣" } else { "□" },
            priority = item.priority,
            name = item.name,
            children = format_item_vec(item.items.clone(), depth + 1)
        )[..];
    } else {
        output += &format!("{box} {priority} ({date}) {name}\n{children}",
            box = if item.completed { "▣" } else { "□" },
            priority = item.priority,
            name = item.name,
            date = item.date,
            children = format_item_vec(item.items.clone(), depth + 1)
        )[..];
    }

    output
}

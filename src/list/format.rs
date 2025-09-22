use crate::list::{Item, List};

pub fn format_list(list: List, path: std::path::PathBuf) -> String {
    let mut output = String::new();

    output += &format!("╭ # {} ({})\n", list.name, path.as_path().display())[..];
    output += "│\n";
    output += &format!("{}", format_item_vec(list.items.clone(), vec![]))[..];

    output
}

fn format_item_vec(items: Vec<Item>, lines: Vec<bool>) -> String {
    let mut output = String::new();

    for (i, item) in items.clone().into_iter().enumerate() {
        let is_end = i >= items.len() - 1;
        output += &format_item(item.clone(), is_end, lines.clone());
    }

    output
}

fn format_item(item: Item, end: bool, lines: Vec<bool>) -> String {
    let mut output = String::new();

    for level in lines.clone() {
        if level {
            output += "  ";
        } else {
            output += "│ ";
        }
    }

    if end {
        output += "╰ ";
    } else {
        output += "├ ";
    }

    let mut new_lines = lines.clone();
    new_lines.push(end);

    if item.date == "" {
        output += &format!("{box} {priority} {name}\n{children}",
            box = if item.completed { "▣" } else { "□" },
            priority = item.priority,
            name = item.name,

            children = format_item_vec(item.items.clone(), new_lines)
        )[..];
    } else {
        output += &format!("{box} {priority} ({date}) {name}\n{children}",
            box = if item.completed { "▣" } else { "□" },
            priority = item.priority,
            name = item.name,
            date = item.date,
            children = format_item_vec(item.items.clone(), new_lines)
        )[..];
    }

    output
}

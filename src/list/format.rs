use crate::list::{Item, List};

pub fn format_list(list: List, path: std::path::PathBuf, with_color: bool) -> String {
    let mut output = String::new();

    if with_color {
        output += "\u{001b}[2m╭ #\u{001b}[22m ";
    } else {
        output += "╭ # ";
    }

    if with_color {
        if list.date != "".to_string() {
            output += &format!(
                "{} - {} \u{001b}[2m({})\u{001b}[22m\n",
                list.name,
                list.date,
                path.as_path().display()
            )[..];
        } else {
            output += &format!(
                "{} \u{001b}[2m({})\u{001b}[22m\n",
                list.name,
                path.as_path().display()
            )[..];
        }
    } else {
        if list.date != "".to_string() {
            output += &format!(
                "{} - {} ({})\n",
                list.name,
                list.date,
                path.as_path().display()
            )[..];
        } else {
            output += &format!("{} ({})\n", list.name, path.as_path().display())[..];
        }
    }

    if with_color {
        output += "\u{001b}[2m│\u{001b}[22m\n";
        output += &format!(
            "{}",
            format_item_vec(list.items.clone(), vec![], with_color)
        )[..];
    } else {
        output += "│\n";
        output += &format!(
            "{}",
            format_item_vec(list.items.clone(), vec![], with_color)
        )[..];
    }

    output
}

fn format_item_vec(items: Vec<Item>, lines: Vec<bool>, with_color: bool) -> String {
    let mut output = String::new();

    for (i, item) in items.clone().into_iter().enumerate() {
        let is_end = i >= items.len() - 1;
        output += &format_item(item.clone(), is_end, lines.clone(), with_color);
    }

    output
}

fn format_item(item: Item, end: bool, lines: Vec<bool>, with_color: bool) -> String {
    let mut output = String::new();

    for level in lines.clone() {
        if level {
            output += "  ";
        } else {
            if with_color {
                output += "\u{001b}[2m│\u{001b}[22m ";
            } else {
                output += "│ ";
            }
        }
    }

    if with_color {
        if end {
            output += "\u{001b}[2m╰\u{001b}[22m ";
        } else {
            output += "\u{001b}[2m├\u{001b}[22m ";
        }
    } else {
        if end {
            output += "╰ ";
        } else {
            output += "├ ";
        }
    }

    let mut new_lines = lines.clone();
    new_lines.push(end);

    // Set colors based on the priority
    if with_color {
        match item.priority {
            i16::MIN..=-7 => output += "\u{001b}[32m",
            -6 => output += "\u{001b}[32m",
            -5 => output += "\u{001b}[36m",
            -4 => output += "\u{001b}[36m",
            -3 => output += "\u{001b}[34m",
            -2 => output += "\u{001b}[34m",
            -1 => output += "\u{001b}[37m",
            0 => output += "\u{001b}[37m",
            1 => output += "\u{001b}[37m",
            2 => output += "\u{001b}[33m",
            3 => output += "\u{001b}[33m",
            4 => output += "\u{001b}[35m",
            5 => output += "\u{001b}[35m",
            6 => output += "\u{001b}[31m",
            7 => output += "\u{001b}[31m",
            7..=i16::MAX => output += "\u{001b}[31m",
        }
    }

    if with_color {
        if item.date == "" {
            output += &format!("{box} {priority} {name}\u{001b}[29m\u{001b}[22m\n{children}",
                box = if item.completed { "\u{001b}[2m▣\u{001b}[9m" } else { "□" },
                priority = item.priority,
                name = item.name,

                children = format_item_vec(item.items.clone(), new_lines, with_color)
            )[..];
        } else {
            output += &format!("{box} {priority} ({date}) {name}\u{001b}[29m\u{001b}[22m\n{children}",
                box = if item.completed { "\u{001b}[2m▣\u{001b}[9m" } else { "□" },
                priority = item.priority,
                name = item.name,
                date = item.date,
                children = format_item_vec(item.items.clone(), new_lines, with_color)
            )[..];
        }
    } else {
        if item.date == "" {
            output += &format!("{box} {priority} {name}\n{children}",
                box = if item.completed { "▣" } else { "□" },
                priority = item.priority,
                name = item.name,

                children = format_item_vec(item.items.clone(), new_lines, with_color)
            )[..];
        } else {
            output += &format!("{box} {priority} ({date}) {name}\n{children}",
                box = if item.completed { "▣" } else { "□" },
                priority = item.priority,
                name = item.name,
                date = item.date,
                children = format_item_vec(item.items.clone(), new_lines, with_color)
            )[..];
        }
    }

    if with_color {
        output += "\u{001b}[39m";
    }

    output
}

use crate::list;

pub fn save_list(data: list::List) {
    std::fs::write(data.path.clone().as_path(), list_to_string(data)).expect("should have been able to save the '.todo' file at {path}");
}

fn list_to_string(list: list::List) -> String{
    let mut output = String::new();

    output += &format!("#{title}\n\n", title=list.name)[..];

    output += &items_to_string(list.items, 0)[..];

    output
}

fn items_to_string(items: Vec<list::Item>, depth: usize) -> String {
    let mut output = String::new();

    for item in items {
        let indent = " ".repeat(depth);
        let completed = if item.completed {"x"} else {" "};
        let priority = item.priority;
        let date = item.date;
        let name = item.name;

        if item.items.len() > 0 {
            output += &format!("{indent}- [{completed}] \\{priority}\\{date}\\ {name}\n{children}", children=items_to_string(item.items, depth + 1));
        } else {
            output += &format!("{indent}- [{completed}] \\{priority}\\{date}\\ {name}\n");
        }
    }

    output
}

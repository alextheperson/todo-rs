pub mod list_types;
mod display_list;

use list_types::{
    Item,
    List
};

use display_list::format_list;

pub fn parse_list(path: std::path::PathBuf) {
    let content = std::fs::read_to_string(&path)
        .expect("should have been able to read the file {path}");

    let lines = content.lines();

    let name = lines.clone().nth(0).expect("expected title in {path}").get(2..).unwrap();

    let mut remaining_lines = lines.clone();
    remaining_lines.nth(1);
    let items = parse_items(remaining_lines, 0);

    println!("{}", format_list(List{
        name: name.to_string(),
        items: items
    }, path));
}

/// This parses the actual items from a list, ignoring the title, etc.
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

        let mut sections = line.split("\\");

        let completed = &sections.next().unwrap_or("- [ ]").trim_start()[3..4] == "x";


        let has_priority_value = sections.clone().next().unwrap_or("").parse::<i16>().is_ok();
        let priority: i16 = if has_priority_value {
            sections.next().unwrap_or("0").trim().parse().unwrap_or(0)
        } else {
            0
        };

        let has_date_value = (sections.clone().count() > 1) && sections.clone().next().is_some();
        let date = if has_date_value {
            sections.next().unwrap_or("").trim().to_string()
        } else {
            "".to_string()
        };

        println!("{} {} {} {} {:?}", has_priority_value, has_date_value, priority, date, sections.clone().count());

        // TODO: Join the rest of the sections with '\' in case they have one in the name
        let name = if has_date_value || has_priority_value {
            sections.next().unwrap_or("Unnamed").trim().to_string()
        } else {
            line.trim_start()[6..].to_string()
        };

        sub_items.sort_by(|a, b| b.priority.cmp(&a.priority));

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

    items.sort_by(|a, b| b.priority.cmp(&a.priority));
    items
}

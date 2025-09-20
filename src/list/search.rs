use crate::list;
use crate::search_paths;

pub fn complete_item(list: &mut Vec<list::Item>, prefix: String) -> bool{
    for (i, item) in list.clone().into_iter().enumerate() {
        if item.name.to_ascii_lowercase().starts_with(&prefix.to_ascii_lowercase()) {
            list[i].completed = true;
            return true;
        } else if item.items.len() > 0 {
            let result = complete_item(&mut list[i].items, prefix.clone());
            if result {
                return result;
            }
        }
    }

    false
}

pub fn toggle_item(list: &mut Vec<list::Item>, prefix: String) -> bool {
    for (i, item) in list.clone().into_iter().enumerate() {
        if item.name.to_ascii_lowercase().starts_with(&prefix.to_ascii_lowercase()) {
            list[i].completed = !list[i].completed;
            return true
        } else if item.items.len() > 0 {
            let result = toggle_item(&mut list[i].items, prefix.clone());
            if result {
                return result;
            }
        }
    }

    false
}

pub fn add_item(list: &mut Vec<list::Item>, new_item: list::Item, prefix: Vec<&str>) -> bool{
    if prefix.len() == 0 {
        for item in list.clone() {
            if item.name.starts_with(&new_item.name) {
                println!("Item already exits");
                return false;
            }
        }
        list.push(new_item);
        return true;
    } else {
        for (i, item) in list.clone().into_iter().enumerate() {
            if item.name.starts_with(prefix[0]) {
                return add_item(&mut list[i].items, new_item, prefix[1..].to_vec());
            }
        }
    };

    false
}

pub fn get_list(name: String) -> Result<list::List, String> {
    let paths = search_paths::search_up();

    for path in paths {
        let list = list::parse_list(path);
        if list.name == name {
            return Ok(list)
        }
    }

    Err(format!("No list called '{name}'"))
}

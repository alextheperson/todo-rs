use crate::list;

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

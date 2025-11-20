use crate::output::buffer::OutputBuffer;
use crate::todo::item::Item;
use crate::todo::path::ItemPath;
use std::cmp::Ordering;

pub type List = Vec<Item>;

pub trait ItemList {
    fn parse(file: String) -> List;
    fn to_save(&self) -> String;
    fn find(&mut self, path: ItemPath) -> Result<&mut Item, String>;
    fn add_item(&mut self, item: Item, path: ItemPath);
    fn recursive_filter(&mut self, predicate: fn(&Item) -> bool);
    fn format(&self, lines: Vec<bool>) -> OutputBuffer;
    fn format_overview(&self, lines: Vec<bool>) -> OutputBuffer;
    fn prune(&mut self);
    fn remove_by_path(&mut self, path: ItemPath) -> Result<Item, String>;
}

impl ItemList for List {
    fn parse(file: String) -> List {
        let mut items: Vec<Item> = vec![];

        let content = file.lines();

        if content.clone().count() <= 0 {
            return vec![];
        }

        let starting_indentation = content.clone().nth(0).unwrap().chars().count()
            - content.clone().nth(0).unwrap().trim_start().chars().count();

        for (i, line) in content.clone().enumerate() {
            let current_indentation = line.chars().count() - line.trim_start().chars().count();

            let mut next_indentation = 0;
            if i < content.clone().count() - 1 {
                next_indentation = content.clone().nth(i + 1).unwrap().chars().count()
                    - content
                        .clone()
                        .nth(i + 1)
                        .unwrap()
                        .trim_start()
                        .chars()
                        .count();
            }

            let mut sub_items: Vec<Item> = vec![];

            // If we rise out of the level that we start at
            if current_indentation < starting_indentation {
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
                    sub_items = List::parse(remaining_lines.collect::<Vec<&str>>().join("\n"));
                }
            }

            items.push(Item::from(line.to_string(), sub_items));
        }

        items.sort_by(|a, b| {
            if a.completed ^ b.completed {
                if a.completed && !b.completed {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            } else {
                return b.priority.cmp(&a.priority);
            }
        });

        items
    }

    fn to_save(&self) -> String {
        let mut output = String::new();

        for item in self.clone() {
            output += &item.to_string(0);
        }

        output
    }

    /// Get a mutable reference to an item that matches a certain path.
    fn find(&mut self, path: ItemPath) -> Result<&mut Item, String> {
        let mut matching_itmes = vec![];

        for (i, item) in self.clone().into_iter().enumerate() {
            if path.clone().matches(item.clone()) {
                matching_itmes.push(i);
            }
        }

        // There is probably a better way to do this, but I don't know enough rust for that.
        if path.item_prefixes.len() == 1 && matching_itmes.len() > 0 {
            return Ok(&mut self[matching_itmes[0]]);
        } else {
            for i in matching_itmes {
                let mut cloned_list = self.clone();
                let result = cloned_list[i].items.find(path.clone().shifted());

                if result.is_ok() {
                    return self[i].items.find(path.clone().shifted());
                }
            }
        }

        Err(format!(
            "Could not find an item that started with '{}'",
            path.item_prefixes[0]
        ))
    }

    fn add_item(&mut self, item: Item, path: ItemPath) {
        // If there is no item specified, simply add it to the root of the list.
        if path.item_prefixes.len() == 0 {
            self.push(item.clone());
        } else {
            self.find(path).unwrap().items.push(item);
        }
    }

    fn format(&self, lines: Vec<bool>) -> OutputBuffer {
        let mut output = OutputBuffer::new();

        for (i, item) in self.clone().into_iter().enumerate() {
            let is_end = i >= self.len() - 1;
            output.append(item.clone().format(is_end, lines.clone()));
        }

        output
    }

    fn recursive_filter(&mut self, predicate: fn(&Item) -> bool) {
        // I don't like this, but I don't think there is a particularly better way of doing this.
        let mut removed_items = 0;

        for (i, item) in self.clone().into_iter().enumerate() {
            if predicate(&item) {
                self.remove(i - removed_items);
                removed_items += 1;
            } else {
                self[i - removed_items].items.recursive_filter(predicate);
            }
        }
    }

    fn prune(&mut self) {
        for item in self {
            if item.completed {
                item.archived = true;
            }

            item.items.prune();
        }
    }

    fn remove_by_path(&mut self, path: ItemPath) -> Result<Item, String> {
        let mut matching_itmes = vec![];

        for (i, item) in self.clone().into_iter().enumerate() {
            if path.clone().matches(item.clone()) {
                matching_itmes.push(i);
            }
        }

        // There is probably a better way to do this, but I don't know enough rust for that.
        if path.item_prefixes.len() == 1 && matching_itmes.len() > 0 {
            let item = self[matching_itmes[0]].clone();
            self.remove(matching_itmes[0]);
            return Ok(item);
        } else {
            for i in matching_itmes {
                let mut cloned_list = self.clone();
                let result = cloned_list[i].items.remove_by_path(path.clone().shifted());

                if result.is_ok() {
                    return self[i].items.remove_by_path(path.clone().shifted());
                }
            }
        }

        Err(format!(
            "Could not find an item that started with '{}'",
            path.item_prefixes[0]
        ))
    }

    fn format_overview(&self, lines: Vec<bool>) -> OutputBuffer {
        let mut output = OutputBuffer::new();

        for (i, item) in self.clone().into_iter().enumerate() {
            let is_end = i >= self.len() - 1;
            output.append(item.clone().format_overview(true, is_end, lines.clone()));
        }

        output
    }
}

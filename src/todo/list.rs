use crate::output;
use crate::todo::item::{self, Item};

pub type List = Vec<Item>;

pub trait ItemList {
    fn parse(file: String) -> List;
    fn to_save(&self) -> String;
    fn find(&mut self, path: Vec<&str>) -> Result<&mut item::Item, String>;
    fn add_item(&mut self, item: Item, path: Vec<&str>);
    fn format(&self, lines: Vec<bool>) -> output::buffer::OutputBuffer;
}

impl ItemList for List {
    fn parse(file: String) -> List {
        let mut items: Vec<item::Item> = vec![];

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

            let mut sub_items: Vec<item::Item> = vec![];

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

            items.push(item::Item::from(line.to_string(), sub_items));
        }

        items.sort_by(|a, b| {
            if a.completed ^ b.completed {
                if a.completed && !b.completed {
                    return std::cmp::Ordering::Greater;
                } else {
                    return std::cmp::Ordering::Less;
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

    // Get a mutable reference to an item that matches a certain path.
    fn find(&mut self, path: Vec<&str>) -> Result<&mut item::Item, String> {
        let prefix = &path[0];
        for (i, item) in self.clone().into_iter().enumerate() {
            if item
                .name
                .to_ascii_lowercase()
                .starts_with(&prefix.to_ascii_lowercase())
            {
                if path.len() == 1 {
                    return Ok(&mut self[i]);
                } else {
                    return self[i].items.find(path[1..].to_vec());
                }
            }
        }

        Err(format!(
            "Could not find an item that started with '{}'",
            path[0]
        ))
    }

    fn add_item(&mut self, item: Item, path: Vec<&str>) {
        self.find(path).unwrap().items.push(item);
    }

    fn format(&self, lines: Vec<bool>) -> output::buffer::OutputBuffer {
        let mut output = output::buffer::OutputBuffer::new();

        for (i, item) in self.clone().into_iter().enumerate() {
            let is_end = i >= self.len() - 1;
            output.append(item.clone().format(is_end, lines.clone()));
        }

        output
    }
}

use crate::error::{CodeComponent::ItemList, CodeComponent::ListParser, Error};
use crate::output::buffer::OutputBuffer;
use crate::todo::item::Item;
use crate::todo::path::ItemPath;
use crate::{match_error, match_option, propagate};
use std::cmp::Ordering;

pub type List = Vec<Item>;

pub trait TodoList {
    fn parse(file: String) -> Result<List, Error>;
    fn to_save(&self) -> String;
    fn find(&mut self, path: &ItemPath) -> Result<&mut Item, Error>;
    fn add_item(&mut self, item: Item, path: ItemPath) -> Result<(), Error>;
    fn recursive_filter(&mut self, predicate: fn(&Item) -> bool);
    fn format(&self, lines: Vec<bool>) -> Result<OutputBuffer, Error>;
    fn format_overview(&self, lines: Vec<bool>) -> Result<OutputBuffer, Error>;
    fn prune(&mut self);
    fn remove_by_path(&mut self, path: &ItemPath) -> Result<Item, Error>;
}

impl TodoList for List {
    fn parse(file: String) -> Result<List, Error> {
        let mut items: Vec<Item> = vec![];

        let content = file.lines();

        if content.clone().count() <= 0 {
            return Ok(vec![]);
        }

        let first_row = match_option!(
            content.clone().nth(0),
            ListParser,
            format!("Could not get the first line of the list.")
        );

        let starting_indentation =
            first_row.chars().count() - first_row.trim_start().chars().count();

        for (i, line) in content.clone().enumerate() {
            let current_indentation = line.chars().count() - line.trim_start().chars().count();

            let mut next_indentation = 0;
            if i < content.clone().count() - 1 {
                let next_line = match_option!(
                    content.clone().nth(i + 1),
                    ListParser,
                    format!("Could not get line number {} of the list", i + 2)
                );

                next_indentation =
                    next_line.chars().count() - next_line.trim_start().chars().count();
            }

            let mut sub_items: Vec<Item> = vec![];

            // If we rise out of the level that we start at
            if current_indentation < starting_indentation {
                break;
            }

            // Skip the lower-level lines, they are going to be handled recursively
            if current_indentation > starting_indentation {
                continue;
            }

            // We need to go deeper
            if current_indentation < next_indentation {
                let mut remaining_lines = content.clone();
                remaining_lines.nth(i);
                if remaining_lines.clone().count() > 0 {
                    sub_items = match_error!(
                        List::parse(remaining_lines.collect::<Vec<&str>>().join("\n")),
                        ListParser,
                        format!(
                            "Could not parse list of sub-items at line {index}.",
                            index = i + 1
                        )
                    );
                }
            }

            items.push(match_error!(
                Item::from(line.to_string(), sub_items),
                ListParser,
                format!(
                    "Could not parse item at line {index}: '{line}'.",
                    index = i + 1
                )
            ));
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

        Ok(items)
    }

    fn to_save(&self) -> String {
        let mut output = String::new();

        for item in self.clone() {
            output += &item.to_string(0);
        }

        output
    }

    /// Get a mutable reference to an item that matches a certain path.
    fn find(&mut self, path: &ItemPath) -> Result<&mut Item, Error> {
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
                let result = cloned_list[i].items.find(&path.clone().shifted());

                if result.is_ok() {
                    return self[i].items.find(&path.clone().shifted());
                }
            }
        }

        Err(propagate!(
            ItemList,
            format!(
                "Could not find an item that started with '{}'",
                path.item_prefixes[0]
            )
        ))
    }

    fn add_item(&mut self, item: Item, path: ItemPath) -> Result<(), Error> {
        // If there is no item specified, simply add it to the root of the list.
        if path.item_prefixes.len() == 0 {
            self.push(item.clone());
        } else {
            let found_item = match_error!(
                self.find(&path),
                ItemList,
                format!("Could not find item at path '{}'.", path.display())
            );
            found_item.items.push(item);
        }

        Ok(())
    }

    fn format(&self, lines: Vec<bool>) -> Result<OutputBuffer, Error> {
        let mut output = OutputBuffer::new();

        for (i, item) in self.clone().into_iter().enumerate() {
            let is_end = i >= self.len() - 1;
            output.append(match_error!(
                item.format(is_end, lines.clone()),
                ItemList,
                format!("Could not format item in list.")
            ));
        }

        Ok(output)
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

    fn remove_by_path(&mut self, path: &ItemPath) -> Result<Item, Error> {
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
                let result = cloned_list[i].items.remove_by_path(&path.clone().shifted());

                if result.is_ok() {
                    return self[i].items.remove_by_path(&path.clone().shifted());
                }
            }
        }

        Err(propagate!(
            ItemList,
            format!(
                "Could not find the item at path '{}' to remove.",
                path.display()
            )
        ))
    }

    fn format_overview(&self, lines: Vec<bool>) -> Result<OutputBuffer, Error> {
        let mut output = OutputBuffer::new();

        for (i, item) in self.clone().into_iter().enumerate() {
            let is_end = i >= self.len() - 1;
            output.append(match_error!(
                item.clone().format_overview(true, is_end, lines.clone()),
                ItemList,
                format!("Could not format list overview.")
            ));
        }

        Ok(output)
    }
}

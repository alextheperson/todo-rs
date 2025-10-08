#[derive(Debug, Clone)]
pub struct Item {
    pub completed: bool,
    pub priority: i16,
    pub date: String,
    pub name: String,
    pub items: crate::todo::list::List,
}

impl Item {
    /// Parses a single line from a file to create a todo item. It does not handle parsing
    /// children.
    pub fn from(input: String, sub_items: Vec<Item>) -> Item {
        let mut sections = input.split("\\");

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

        let last_section = sections.collect::<Vec<&str>>().join("\\");

        let name = if has_date_value || has_priority_value {
            last_section.trim().to_string()
        } else {
            input.trim_start()[6..].to_string()
        };

        let mut children = sub_items.clone();
        children.sort_by(|a, b| b.priority.cmp(&a.priority));

        Item {
            name: name,
            priority: priority,
            date: date,
            completed: completed,
            items: children,
        }
    }

    /// This formats it for saving, NOT FOR DISPLAY
    pub fn to_string(&self, depth: usize) -> String {
        let mut output = String::new();

        let indent = " ".repeat(depth);
        let completed = if self.completed { "x" } else { " " };
        let priority = self.priority;
        let date = &self.date;
        let name = &self.name;

        let mut children = String::new();

        for child in self.clone().items {
            children += &child.to_string(depth + 1);
        }

        if self.items.len() > 0 {
            output += &format!(
                "{indent}- [{completed}] \\{priority}\\{date}\\ {name}\n{children}",
                children = children
            );
        } else {
            output += &format!("{indent}- [{completed}] \\{priority}\\{date}\\ {name}\n");
        }

        output
    }
}

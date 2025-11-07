use crate::ItemList;
use crate::date;
use crate::date::Date;
use crate::output;

#[derive(Debug, Clone)]
pub struct Item {
    pub completed: bool,
    pub archived: bool,
    pub priority: i16,
    pub date: Option<date::Date>,
    pub name: String,
    pub items: crate::todo::list::List,
}

impl Item {
    /// Parses a single line from a file to create a todo item. It does not handle parsing
    /// children.
    pub fn from(input: String, sub_items: Vec<Item>) -> Item {
        let mut sections = input.split("\\");

        let archived = &sections.clone().next().unwrap_or("- [ ]").trim_start()[3..4] == "a";
        let completed = &sections.next().unwrap_or("- [ ]").trim_start()[3..4] == "x" || archived;

        let has_priority_value = sections.clone().next().unwrap_or("").parse::<i16>().is_ok();
        let priority: i16 = if has_priority_value {
            sections.next().unwrap_or("0").trim().parse().unwrap_or(0)
        } else {
            0
        };

        let has_date_value = (sections.clone().count() > 1) && sections.clone().next().is_some();
        let date = if has_date_value {
            let section = &sections.next().unwrap().trim().to_string();
            if section != "" {
                let parsed_date = Date::from(section);
                if parsed_date.is_err() {
                    None
                } else {
                    Some(parsed_date.unwrap())
                }
            } else {
                None
            }
        } else {
            None
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
            archived: archived,
            items: children,
        }
    }

    /// This formats it for saving, NOT FOR DISPLAY
    pub fn to_string(&self, depth: usize) -> String {
        let mut output = String::new();

        let indent = " ".repeat(depth);
        let completed = if self.archived {
            "a"
        } else if self.completed {
            "x"
        } else {
            " "
        };
        let priority = self.priority;
        let date = if self.date.is_some() {
            &self.date.clone().unwrap().display()
        } else {
            ""
        };
        let name = &self.name;

        let mut children = String::new();

        for child in self.clone().items {
            children += &child.to_string(depth + 1);
        }

        output += &format!("{indent}- [{completed}] ");
        if self.priority != 0 {
            output += &format!("\\{priority}");
        }
        if self.date.is_some() {
            output += &format!("\\{date}");
        }
        if self.date.is_some() || self.priority != 0 {
            output += &format!("\\ ");
        }
        output += &format!("{name}\n");

        if self.items.len() > 0 {
            output += &children;
        }

        output
    }

    /// This is a number from 0 to 7+, that represents how close today is to the item's date
    /// It is more than 7 if the day has already passed.
    /// It starts ticking up at 7 days until the date
    pub fn urgency(&self) -> Option<i16> {
        if self.date.is_some() {
            let distance = self.date.unwrap().distance(Date::today());
            if distance > 7 {
                return None;
            } else {
                return Some((7 - distance).try_into().unwrap());
            }
        }
        None
    }

    pub fn format(&self, end: bool, lines: Vec<bool>) -> output::buffer::OutputBuffer {
        let mut output = output::buffer::OutputBuffer::new();
        let mut output_line = output::line::OutputLine::new();

        for level in lines.clone() {
            if level {
                output_line.add(output::segment::OutputSegment::new(
                    "  ",
                    output::color::Color::Default,
                    output::style::Style::normal(),
                ));
            } else {
                output_line.add(output::segment::OutputSegment::new(
                    "│ ",
                    output::color::Color::Default,
                    *output::style::Style::new().dim(),
                ));
            }
        }

        if end {
            output_line.add(output::segment::OutputSegment::new(
                "╰ ",
                output::color::Color::Default,
                *output::style::Style::new().dim(),
            ));
        } else {
            output_line.add(output::segment::OutputSegment::new(
                "├ ",
                output::color::Color::Default,
                *output::style::Style::new().dim(),
            ));
        }

        let mut new_lines = lines.clone();
        new_lines.push(end);

        let priority = if self.completed {
            self.priority
        } else {
            self.priority + self.urgency().unwrap_or(0)
        };

        let date = if !self.completed && self.urgency().is_some() {
            &format!(
                "{num} day{s}",
                num = 7 - self.urgency().unwrap(),
                s = if 7 - self.urgency().unwrap() == 1 {
                    ""
                } else {
                    "s"
                }
            )
        } else {
            if self.date.is_some() {
                &self.date.clone().unwrap().display()
            } else {
                ""
            }
        };

        // Set colors based on the priority
        let color = match priority {
            i16::MIN..=-7 => output::color::Color::Green,
            -6 => output::color::Color::Green,
            -5 => output::color::Color::Green,
            -4 => output::color::Color::Blue,
            -3 => output::color::Color::Blue,
            -2 => output::color::Color::Cyan,
            -1 => output::color::Color::Cyan,
            0 => output::color::Color::Default,
            1 => output::color::Color::Yellow,
            2 => output::color::Color::Yellow,
            3 => output::color::Color::Magenta,
            4 => output::color::Color::Magenta,
            5 => output::color::Color::Red,
            6 => output::color::Color::Red,
            7..=i16::MAX => output::color::Color::Red,
        };

        let style = if self.completed {
            *output::style::Style::new().dim().strikethrough()
        } else {
            output::style::Style::normal()
        };

        if self.date.is_none() {
            output_line.add(output::segment::OutputSegment::new(
                &format!("{box} {priority} {name}",
                    box = if self.archived {"\u{24d0}"} else if self.completed { "▣" } else { "□" },
                    priority = priority,
                    name = self.name,
                ),
                color,
                style,
            ));
        } else {
            output_line.add(output::segment::OutputSegment::new(
                &format!("{box} {priority} ({date}) {name}",
                    box = if self.archived {"\u{24d0}"} else if self.completed { "▣" } else { "□" },
                    priority = priority,
                    name = self.name,
                    date = date                ),
                color,
                style,
            ));
        }

        output.add(output_line);

        output.append(self.items.clone().format(new_lines));

        output
    }
}

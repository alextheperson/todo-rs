use crate::ItemList;
use crate::date;
use crate::date::Date;
use crate::output::buffer::OutputBuffer;
use crate::output::color::Color;
use crate::output::line::OutputLine;
use crate::output::segment::OutputSegment;
use crate::output::style::Style;

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

    pub fn format(&self, end: bool, lines: Vec<bool>) -> OutputBuffer {
        let mut output = OutputBuffer::new();
        let mut output_line = OutputLine::new();

        for level in lines.clone() {
            if level {
                output_line.add(OutputSegment::new("  ", Color::Default, Style::normal()));
            } else {
                output_line.add(OutputSegment::new(
                    "│ ",
                    Color::Default,
                    *Style::new().dim(),
                ));
            }
        }

        if end {
            output_line.add(OutputSegment::new(
                "╰ ",
                Color::Default,
                *Style::new().dim(),
            ));
        } else {
            output_line.add(OutputSegment::new(
                "├ ",
                Color::Default,
                *Style::new().dim(),
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
            i16::MIN..=-7 => Color::Green,
            -6 => Color::Green,
            -5 => Color::Green,
            -4 => Color::Blue,
            -3 => Color::Blue,
            -2 => Color::Cyan,
            -1 => Color::Cyan,
            0 => Color::Default,
            1 => Color::Yellow,
            2 => Color::Yellow,
            3 => Color::Magenta,
            4 => Color::Magenta,
            5 => Color::Red,
            6 => Color::Red,
            7..=i16::MAX => Color::Red,
        };

        let style = if self.completed {
            *Style::new().dim().strikethrough()
        } else {
            Style::normal()
        };

        if self.date.is_none() {
            output_line.add(OutputSegment::new(
                &format!("{box} {priority} {name}",
                    box = if self.archived {"\u{24d0}"} else if self.completed { "▣" } else { "□" },
                    priority = priority,
                    name = self.name,
                ),
                color,
                style,
            ));
        } else {
            output_line.add(OutputSegment::new(
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

    pub fn format_detail(&self, show_children: bool) -> OutputBuffer {
        let mut output = OutputBuffer::new();

        let priority = if self.completed {
            self.priority
        } else {
            self.priority + self.urgency().unwrap_or(0)
        };

        let date = if self.date.is_some() {
            &self.date.clone().unwrap().display()
        } else {
            ""
        };

        let relative_date = if self.date.is_some() {
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
            ""
        };

        // Set colors based on the priority
        let color = match priority {
            i16::MIN..=-7 => Color::Green,
            -6 => Color::Green,
            -5 => Color::Green,
            -4 => Color::Blue,
            -3 => Color::Blue,
            -2 => Color::Cyan,
            -1 => Color::Cyan,
            0 => Color::Default,
            1 => Color::Yellow,
            2 => Color::Yellow,
            3 => Color::Magenta,
            4 => Color::Magenta,
            5 => Color::Red,
            6 => Color::Red,
            7..=i16::MAX => Color::Red,
        };

        let mut priority_line = OutputLine::new();
        priority_line.add(OutputSegment::new(
            &format!("Priority: {}", priority),
            color.clone(),
            Style::new(),
        ));

        let mut name_line = OutputLine::new();
        name_line.add(OutputSegment::new(
            &format!("Name: {}", self.name),
            Color::Default,
            Style::new(),
        ));

        let mut date_line = OutputLine::new();
        if date != "" {
            date_line.add(OutputSegment::new(
                &format!("Date: {} ({} days from now)", date, relative_date),
                Color::Default,
                Style::new(),
            ));
        } else {
            date_line.add(OutputSegment::new(
                &format!("Date: None"),
                Color::Default,
                Style::new(),
            ));
        }

        output.add(priority_line);
        output.add(date_line);
        output.add(name_line);

        if show_children {
            output.append(self.items.clone().format_overview(vec![]));
        }

        output
    }

    pub fn format_overview(
        &self,
        show_children: bool,
        end: bool,
        lines: Vec<bool>,
    ) -> OutputBuffer {
        let mut output = OutputBuffer::new();
        let mut output_line = OutputLine::new();

        for level in lines.clone() {
            if level {
                output_line.add(OutputSegment::new("  ", Color::Default, Style::normal()));
            } else {
                output_line.add(OutputSegment::new(
                    "│ ",
                    Color::Default,
                    *Style::new().dim(),
                ));
            }
        }

        if end {
            output_line.add(OutputSegment::new(
                "╰ ",
                Color::Default,
                *Style::new().dim(),
            ));
        } else {
            output_line.add(OutputSegment::new(
                "├ ",
                Color::Default,
                *Style::new().dim(),
            ));
        }

        let mut new_lines = lines.clone();
        new_lines.push(end);

        let priority = if self.completed {
            self.priority
        } else {
            self.priority + self.urgency().unwrap_or(0)
        };

        // Set colors based on the priority
        let color = match priority {
            i16::MIN..=-7 => Color::Green,
            -6 => Color::Green,
            -5 => Color::Green,
            -4 => Color::Blue,
            -3 => Color::Blue,
            -2 => Color::Cyan,
            -1 => Color::Cyan,
            0 => Color::Default,
            1 => Color::Yellow,
            2 => Color::Yellow,
            3 => Color::Magenta,
            4 => Color::Magenta,
            5 => Color::Red,
            6 => Color::Red,
            7..=i16::MAX => Color::Red,
        };

        let style = if self.completed {
            *Style::new().dim().strikethrough()
        } else {
            Style::normal()
        };

        if self.date.is_none() {
            output_line.add(OutputSegment::new(
                &format!("{box} {name}",
                    box = if self.archived {"\u{24d0} "} else if self.completed { "▣" } else { "□" },
                    name = self.name,
                ),
                color,
                style,
            ));
        } else {
            output_line.add(OutputSegment::new(
                &format!("{box} {name}",
                    box = if self.archived {"\u{24d0} "} else if self.completed { "▣" } else { "□" },
                    name = self.name,
                ),
                color,
                style,
            ));
        }

        output.add(output_line);

        if show_children {
            output.append(self.items.clone().format_overview(new_lines));
        }

        output
    }
}

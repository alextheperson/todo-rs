use crate::date;
use crate::date::Date;
use crate::error::{CodeComponent::ItemParser, CodeComponent::TodoItem, Error};
use crate::output::buffer::OutputBuffer;
use crate::output::color::Color;
use crate::output::line::OutputLine;
use crate::output::segment::OutputSegment;
use crate::output::style::Style;
use crate::todo::list::TodoList;
use crate::{match_error, match_option};

#[derive(Debug, Clone)]
pub struct Item {
    pub completed: bool,
    pub archived: bool,
    pub priority: i64,
    pub date: Option<date::Date>,
    pub name: String,
    pub items: crate::todo::list::List,
}

impl Item {
    /// Parses a single line from a file to create a todo item. It does not handle parsing
    /// children.
    pub fn from(input: String, sub_items: Vec<Item>) -> Result<Item, Error> {
        let mut sections = input.split("\\");

        let archived = &sections.clone().next().unwrap_or("- [ ]").trim_start()[3..4] == "a";
        let completed = &sections.next().unwrap_or("- [ ]").trim_start()[3..4] == "x" || archived;

        let has_priority_value = sections.clone().next().unwrap_or("").parse::<i64>().is_ok();
        let priority = if has_priority_value {
            sections.next().unwrap_or("0").trim().parse().unwrap_or(0)
        } else {
            0
        };

        let has_date_value = (sections.clone().count() > 1) && sections.clone().next().is_some();
        let date = if has_date_value {
            let section = &match_option!(
                sections.next(),
                ItemParser,
                format!("Could not parse item when looking for a date.")
            )
            .trim()
            .to_string();
            if section != "" {
                Date::from(section).ok()
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

        Ok(Item {
            name: name,
            priority: priority,
            date: date,
            completed: completed,
            archived: archived,
            items: children,
        })
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
        let date = match self.date {
            Some(val) => &val.display(),
            _ => "",
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
    pub fn urgency(&self) -> Result<Option<i64>, Error> {
        if let Some(date) = self.date {
            let distance = match_error!(
                date.distance(match_error!(
                    Date::today(),
                    TodoItem,
                    format!("Could not get the date today.")
                )),
                TodoItem,
                format!("Could not get the temporal distance to the item.")
            );
            if distance > 7 {
                return Ok(None);
            } else {
                return Ok(Some(7 - distance));
            }
        } else {
            return Ok(None);
        }
    }

    pub fn format(&self, end: bool, lines: Vec<bool>) -> Result<OutputBuffer, Error> {
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

        let urgency = match_error!(
            self.urgency(),
            TodoItem,
            format!("Could not get the item's urgency.")
        );
        let priority = if self.completed {
            self.priority
        } else {
            self.priority + urgency.unwrap_or(0)
        };

        let date = match urgency {
            Some(urgency) => &format!(
                "{num} day{s}",
                num = 7 - urgency,
                s = if 7 - urgency == 1 { "" } else { "s" }
            ),
            _ => {
                if let Some(date) = self.date {
                    &date.display()
                } else {
                    ""
                }
            }
        };

        // Set colors based on the priority
        let color = match priority {
            i64::MIN..=-7 => Color::Green,
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
            7..=i64::MAX => Color::Red,
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

        output.append(match_error!(
            self.items.format(new_lines),
            TodoItem,
            format!("Could not format child items")
        ));

        Ok(output)
    }

    pub fn format_detail(&self, show_children: bool) -> Result<OutputBuffer, Error> {
        let mut output = OutputBuffer::new();

        let urgency = match_error!(
            self.urgency(),
            TodoItem,
            format!("Could not get the item's urgency.")
        );

        let priority = if self.completed {
            self.priority
        } else {
            self.priority + urgency.unwrap_or(0)
        };

        let date = match self.date {
            Some(date) => &date.clone().display(),
            _ => "",
        };

        let relative_date = match self.date {
            Some(date) => {
                let date_distance = match_error!(
                    date.distance(match_error!(
                        Date::today(),
                        TodoItem,
                        format!("Could not get today's date.")
                    )),
                    TodoItem,
                    format!("Could not get the temporal distance to the item.")
                );
                &format!(
                    "{num} day{s}",
                    num = date_distance,
                    s = if date_distance == 1 { "" } else { "s" }
                )
            }
            _ => "",
        };

        // Set colors based on the priority
        let color = match priority {
            i64::MIN..=-7 => Color::Green,
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
            7..=i64::MAX => Color::Red,
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
            output.append(match_error!(
                self.items.clone().format_overview(vec![]),
                TodoItem,
                format!("Could not format overview of the item's children.")
            ));
        }

        Ok(output)
    }

    pub fn format_overview(
        &self,
        show_children: bool,
        end: bool,
        lines: Vec<bool>,
    ) -> Result<OutputBuffer, Error> {
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

        let urgency = match_error!(
            self.urgency(),
            TodoItem,
            format!("Could not get the item's urgency.")
        );

        let priority = if self.completed {
            self.priority
        } else {
            self.priority + urgency.unwrap_or(0)
        };

        // Set colors based on the priority
        let color = match priority {
            i64::MIN..=-7 => Color::Green,
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
            7..=i64::MAX => Color::Red,
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
            output.append(match_error!(
                self.items.clone().format_overview(new_lines),
                TodoItem,
                format!("Could not format the overview of the item's children")
            ));
        }

        Ok(output)
    }
}

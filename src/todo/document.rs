use crate::date::Date;
use crate::error::{CodeComponent, Error};
use crate::output::buffer::OutputBuffer;
use crate::output::color::Color;
use crate::output::line::OutputLine;
use crate::output::segment::OutputSegment;
use crate::output::style::Style;
use crate::todo::list;
use crate::todo::list::TodoList;
use crate::{match_error, match_option, match_result, propagate};

#[derive(Debug, Clone)]
pub struct Document {
    pub name: String,
    pub path: std::path::PathBuf,
    pub priority: i32,
    pub date: Option<Date>,
    pub items: list::List,
    pub archived: bool,
}

impl Document {
    pub fn from(file: String, path: std::path::PathBuf) -> Result<Document, Error> {
        let lines = file.lines();

        let mut name = "Unnamed Todo List".to_string();
        let mut priority = 0;
        let mut date = None;
        let mut archived = false;
        let mut lines_to_skip = 0;

        for (i, line) in lines.clone().enumerate() {
            if !line.starts_with("#") {
                continue;
            }

            lines_to_skip += 1;

            if i == 0 {
                name = line[2..].to_string();
                continue;
            }

            let mut parts = line.split(" ").skip(1);
            // Find the name of the property, if there is one.
            let property_name = match_option!(
                parts.next(),
                CodeComponent::DocumentParser,
                format!("Found malformed metadata line: '{}'", line)
            );
            match property_name {
                "priority" => {
                    let rest = parts.clone().collect::<Vec<&str>>().join(" ");
                    priority = match_result!(
                        rest.parse::<i32>(),
                        CodeComponent::DocumentParser,
                        format!(
                            "Could not parse property value to i32. Got value '{}'.",
                            rest
                        )
                    )
                }
                "date" => {
                    date = Date::from(&parts.clone().collect::<Vec<&str>>().join(" ")).ok();
                }
                "archived" => archived = true,
                _ => {
                    return Err(propagate!(
                        CodeComponent::DocumentParser,
                        format!(
                            "Found unknown property '{}' on line {}.",
                            property_name,
                            i + 1
                        )
                    ));
                }
            }
        }

        let remaining_lines = lines
            .clone()
            .skip(lines_to_skip + 1)
            .collect::<Vec<&str>>()
            .join("\n");
        let items = match_error!(
            list::List::parse(remaining_lines),
            CodeComponent::DocumentParser,
            format!(
                "Could not parse the list of items in the document at path {}",
                path.display()
            )
        );

        Ok(Document {
            name: name,
            path: path,
            priority: priority,
            date: date,
            items: items,
            archived: archived,
        })
    }

    pub fn from_path(path: &std::path::PathBuf) -> Result<Document, Error> {
        let mut normalized_path = match_result!(
            std::fs::canonicalize(&path),
            CodeComponent::DocumentParser,
            format!("Could not normalize the path '{}'.", path.display())
        );
        normalized_path.push(".todo");

        let content = match_result!(
            std::fs::read_to_string(&normalized_path),
            CodeComponent::DocumentParser,
            format!(
                "Could not read from the path '{path}'.",
                path = normalized_path.display()
            )
        );
        Document::from(content, path.clone())
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        output += &format!("# {title}\n", title = &self.name);
        if self.priority != 0 {
            output += &format!("# priority {priority}\n", priority = &self.priority);
        }
        if let Some(date) = self.date {
            output += &format!("# date {date}\n", date = date.display());
        }
        if self.archived {
            output += &format!("# archived\n");
        }

        output += "\n";

        output += &self.items.to_save();

        output
    }

    pub fn save(&self) -> Result<(), Error> {
        match_result!(
            std::fs::write(self.path.as_path().join(".todo"), self.to_string()),
            CodeComponent::Document,
            format!(
                "should have been able to save the '.todo' file at {}",
                self.path.display()
            )
        );

        Ok(())
    }

    pub fn format(&self) -> Result<OutputBuffer, Error> {
        let mut output = OutputBuffer::new();

        let mut first_line = OutputLine::new();

        first_line.add(OutputSegment::new(
            "╭ # ",
            Color::Default,
            *Style::new().dim(),
        ));

        if let Some(date) = self.date {
            first_line.add(OutputSegment::new(
                &format!("{name} - {date} ", name = self.name, date = date.display()),
                Color::Default,
                Style::normal(),
            ));
        } else {
            first_line.add(OutputSegment::new(
                &format!("{name} ", name = self.name),
                Color::Default,
                Style::normal(),
            ));
        }

        first_line.add(OutputSegment::new(
            &format!("({path})", path = self.path.as_path().display()),
            Color::Default,
            *Style::new().dim(),
        ));

        output.add(first_line);

        output.add(
            OutputLine::new()
                .add(OutputSegment::new("│", Color::Default, *Style::new().dim()))
                .clone(),
        );

        output.append(match_error!(
            self.items.format(vec![]),
            CodeComponent::Document,
            format!(
                "Could not format the items of the document at path '{}'",
                self.path.display()
            )
        ));

        Ok(output)
    }
}

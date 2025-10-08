use crate::todo::list;
use crate::todo::list::ItemList;

#[derive(Debug, Clone)]
pub struct Document {
    pub name: String,
    pub path: std::path::PathBuf,
    pub priority: i16,
    pub date: String,
    pub items: list::List,
}

impl Document {
    pub fn from(file: String, path: std::path::PathBuf) -> Document {
        let lines = file.lines();

        let mut name = "Unnamed Todo List".to_string();
        let mut priority = 0;
        let mut date = String::new();
        let mut lines_to_skip = 0;

        for (i, line) in lines.clone().enumerate() {
            if line.starts_with("#") {
                lines_to_skip += 1;
                if i == 0 {
                    name = line[2..].to_string()
                } else {
                    let mut parts = line.split(" ").skip(1);
                    let property = parts.next().unwrap();

                    match property {
                        "priority" => priority = parts.next().unwrap().parse().unwrap(),
                        "date" => date = parts.next().unwrap().to_string(),
                        _ => println!("Unknown property! '{}'", property),
                    }
                }
            }
        }

        let remaining_lines = lines
            .clone()
            .skip(lines_to_skip + 1)
            .collect::<Vec<&str>>()
            .join("\n");
        let items = list::List::parse(remaining_lines);

        Document {
            name: name,
            path: path,
            priority: priority,
            date: date,
            items: items,
        }
    }

    pub fn from_path(path: &std::path::PathBuf) -> Document {
        let mut normalized_path = std::fs::canonicalize(&path).unwrap();
        normalized_path.push(".todo");

        let content = std::fs::read_to_string(&normalized_path).expect(&format!(
            "should have been able to read the file {path}",
            path = normalized_path.display()
        ));
        Document::from(content, path.clone())
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        output += &format!("# {title}\n", title = &self.name);
        output += &format!("# priority {priority}\n", priority = &self.priority);
        output += &format!("# date {date}\n", date = &self.date);

        output += "\n";

        output += &self.items.to_save();

        output
    }

    pub fn save(self) {
        std::fs::write(self.path.as_path(), self.to_string()).expect(&format!(
            "should have been able to save the '.todo' file at {}",
            self.path.display()
        ));
    }
}

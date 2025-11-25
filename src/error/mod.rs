#![allow(dead_code)]

use std::panic;

#[derive(Debug, Copy, Clone)]
pub enum CodeComponent {
    External,

    Main,
    Executor,
    FileSearcher,

    DocumentPath,
    Document,
    DocumentParser,

    ItemList,
    ListParser,

    TodoItem,
    ItemParser,

    Date,
    DateParser,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub component: CodeComponent,
    pub message: String,
    pub line: u32,
    pub file: &'static str,
    pub child: Option<Box<Error>>,
}

impl CodeComponent {
    pub fn format(self) -> String {
        format!("[{name}]", name = self.value())
    }

    fn value(self) -> &'static str {
        match self {
            Self::External => "EXTERNAL",

            Self::Main => "MAIN",
            Self::Executor => "EXECUTOR",
            Self::FileSearcher => "FILE_SEARCH",

            Self::DocumentPath => "DOCUMENT:PATH",
            Self::Document => "DOCUMENT:MAIN",
            Self::DocumentParser => "DOCUMENT:PARSER",

            Self::ItemList => "LIST:MAIN",
            Self::ListParser => "LIST:PARSER",

            Self::TodoItem => "ITEM:MAIN",
            Self::ItemParser => "ITEM:PARSER",

            Self::Date => "DATE:MAIN",
            Self::DateParser => "DATE:PARSER",
        }
    }
}

impl Error {
    pub fn print(self) {
        // We do this so that the default panic message does not show.
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        eprintln!(
            "\u{001b}[31mUnfortunately, an error seems to have occured. Here's what seems to have happened:\u{001b}[39m"
        );
        eprintln!("");
        eprintln!("{}", self.format(0));
        eprintln!("");
        eprintln!(
            "\u{001b}[31mPlease report this issue at [https://github.com/alextheperson/todo-rs/issues],\n otherwise we won't know it happened.\u{001b}[39m"
        );
        panic!();
    }

    pub fn format(&self, indent: usize) -> String {
        let mut output = String::new();
        if indent > 1 {
            output += &"   ".repeat(indent - 1);
        }
        if indent > 0 {
            output += &" ╰ ";
        }
        output += &format!(
            "{component} {message} \u{001b}[2m({file}@{line})\u{001b}[0m\n",
            component = self.component.format(),
            message = self.message,
            file = self.file,
            line = self.line
        );

        if let Some(children) = &self.child {
            output += &children.format(indent + 1);
        }

        output
    }
}

// Make an error, and capture the filename and line number.
#[macro_export]
macro_rules! propagate {
    ($component: expr, $message: expr) => {
        Error {
            component: $component,
            message: $message,
            line: line!(),
            file: file!(),
            child: None,
        }
    };
    ($component: expr, $message: expr, $child: expr) => {
        Error {
            component: $component,
            message: $message,
            line: line!(),
            file: file!(),
            child: Some(Box::new($child)),
        }
    };
}

/// This macro is short for using a match expression when using the value of a function that might
/// be an `Error`.
#[macro_export]
macro_rules! match_error {
    ($val: expr, $component: expr, $message: expr) => {
        match $val {
            Ok(val) => val,
            Err(err) => {
                return Err(Error {
                    component: $component,
                    message: $message,
                    line: line!(),
                    file: file!(),
                    child: Some(Box::new(err)),
                })
            }
        }
    };
}

// Shortcut to match a function that returns a `Result` with a different type of error. It makes a
// child that contains the string content of the other error.
#[macro_export]
macro_rules! match_result {
    ($val: expr, $component: expr, $message: expr) => {
        match $val {
            Ok(val) => val,
            Err(err) => {
                let child = Error {
                    component: CodeComponent::External,
                    message: format!("{}", err),
                    line: line!(),
                    file: file!(),
                    child: None,
                };
                return Err(Error {
                    component: $component,
                    message: $message,
                    line: line!(),
                    file: file!(),
                    child: Some(Box::new(child)),
                });
            }
        }
    };
}

// Match a function that returns an `Option<T>`. Return the`Some()``, but error if it is `None`.
#[macro_export]
macro_rules! match_option {
    ($val: expr, $component: expr, $message: expr) => {
        match $val {
            Some(val) => val,
            _ => {
                return Err(Error {
                    component: $component,
                    message: $message,
                    line: line!(),
                    file: file!(),
                    child: None,
                });
            }
        }
    };
}

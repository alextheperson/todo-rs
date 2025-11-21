use clap::builder::PossibleValue;
use clap::{ArgAction, Command, arg, value_parser};

macro_rules! output_format {
     () => {
        arg!(-f --format [FORMAT] "Set the output format.")
            .action(ArgAction::Set)
            .value_name("format")
            .default_value("ansi")
            .value_parser([
                PossibleValue::new("ansi").help("Use terminal escape codes (default)."),
                PossibleValue::new("plain").help("Use plaintext."),
                PossibleValue::new("html").help("Use HTML with inline styles."),
                PossibleValue::new("html-class").help(
                    "Use HTML, with no porovided styles (bring your own colors).",
                ),
                PossibleValue::new("pango").help("Use Pango markup (eg. for waybar)."),
            ])
     };
}

macro_rules! item_path {
    ($x: expr) => {
        arg!(<ITEM_PATH> $x)
    };
}

macro_rules! file_path {
    ($x: expr) => {
        arg!([FILE_PATH] $x)
            .default_value("./")
            .value_parser(value_parser!(std::path::PathBuf))
    };
}

macro_rules! down_flag {
    () => {
        arg!(-d --down "Search down through files instead of up.")
            .action(ArgAction::SetTrue)
    };
}

pub fn build() -> Command {
    Command::new("todo-rs")
        .bin_name("todo")
        .author("Alex Solis, dev@alexsol.is")
        .version("1.0")
        .about("Manage you todo list with directory-scoped .todo files.")
        .after_long_help(
            "CONCEPT
    Each directory on your system may have at most one todo list, enclosed within
    a '.todo' file. By default, `todo` will seach for all todo lists from your
    current directory up to your home directory. However, with the -d flag, you
    can make it seach downwards (for example, to provide a system-wide overview).

USAGE
    When called with no arguments, `todo` will open a TUI to edit your todo lists.

TIPS
    If you are looking to itegrate todo-rs into something else (like waybar), you
    may want to look at the --format option.

FILE FORMAT
    * Prefix metadata lines with a hash(#).
    * There is one line gap between the metadata and the items.
    * Todo items start with \"- [ ]\", with an \"x\" for a completed item, and an
      \"a\" for an archived item, which is hidden by default.

    Example file:
    ┌──────────────────────────────────────────────────────────────────── .todo ─┐
    │# Todo list title                                                           │
    │# priority 3                                                                │
    │# date june 3 2026                                                          │
    │# archived                                                                  │
    │                                                                            │
    │- [ ] Incomplete todo item                                                  │
    │ - [ ] Child todo item                                                      │
    │- [ ] \\2\\6/7/2026\\ Item with a priority and a date (July 6th, 2026)         │
    │ - [ ] \\5\\ Item with a priority of 5                                        │
    │ - [ ] \\4-aug-2025\\ Item that should be completed by August 4th, 2025       │
    │- [x] A completed todo                                                      │
    │- [a] An archived todo item (hidden by default)                             │
    └────────────────────────────────────────────────────────────────────────────┘
                ",
        )
        .arg(file_path!("Specify an alternate path to open the TUI in."))
        /*
         * Main commands
         */
        .subcommand(
            Command::new("init")
                .about("Create a new `.todo` in the current directory.")
                .arg(file_path!("An alternate directory to initialize.")),
        )
        .subcommand(
            Command::new("next")
                .about(
                    "Show the highest priority uncompleted item (the \"next\" thing to work on).",
                )
                .arg(
                    file_path!("An alternate path in which to look for the next item.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                )
                .arg(down_flag!())
                .arg(arg!(-c --children "Show the item's children.").action(ArgAction::SetTrue))
                .arg(output_format!()),
        )
        .subcommand(
            Command::new("list")
                .about("List todo items for the current directory and its parents.")
                .arg(down_flag!())
                .arg(output_format!())
                .arg(file_path!("Specify an alternate path to search from."))
                .arg(arg!(-a --archived "Show archived items.").action(ArgAction::SetTrue))
                .arg(arg!(-c --completed "Hide completed items.").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("add")
                .about("Add an item to a todo list.")
                .arg(item_path!("The path of the todo item to add."))
                .arg(
                    arg!(<ITEM_NAME> "The name of the item to add.")
                        .value_parser(value_parser!(String)),
                )
                .arg(down_flag!())
                .arg(arg!(-D --date "Give the new item a due date.").action(ArgAction::Set))
                .arg(
                    arg!(-p --priority "Set the priority of the new item.")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i16)),
                ),
        )
        .subcommand(
            Command::new("complete")
                .about("Mark a todo item as completed.")
                .arg(item_path!("The path of the todo item to complete."))
                .arg(down_flag!()),
        )
        /*
         * Maintenance Commands
         */
        .subcommand(
            Command::new("prune")
                .about("Archive all completed todo items.")
                .arg(file_path!("Alternate path to start from."))
                .arg(arg!(-s --single "Prune only a single list.").action(ArgAction::SetTrue))
                .arg(down_flag!()),
        )
        /*
         * Editing Commands
         */
        .subcommand(
            Command::new("remove")
                .about("Remove an item from a todo list.")
                .visible_alias("delete")
                .visible_alias("rm")
                .arg(arg!(<ITEM_PATH> "The path of the todo item to remove."))
                .arg(down_flag!()),
        )
        .subcommand(
            Command::new("toggle")
                .about("Toggle the completion of a todo item.")
                .arg(item_path!("The path of the todo item to toggle."))
                .arg(down_flag!()),
        )
        .subcommand(
            Command::new("incomplete")
                .about("Mark a todo item as incomplete.")
                .arg(item_path!("The path of the todo item to mark."))
                .arg(down_flag!()),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit the properties of a todo item.")
                .arg(item_path!("The path of the todo item to add."))
                .arg(down_flag!())
                .arg(arg!(-n --name "Set the name of the todo item.").action(ArgAction::Set))
                .arg(arg!(-D --date "Set the date of the todo item.").action(ArgAction::Set))
                .arg(
                    arg!(-p --priority "Set the priority of the todo item.").action(ArgAction::Set),
                )
                .arg(
                    arg!(-c --completed "Set whether the item is completed").action(ArgAction::Set),
                )
                .arg(arg!(-a --archived "Set whether the item is archived").action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("get")
                .about("Get the details of a specific todo item or list.")
                .arg(
                    arg!(<ITEM_PATH> "The path of the todo item to get."),
                )
                .arg(output_format!())
                .arg(down_flag!()),
        )
        // TODO:
        .subcommand(
            Command::new("move")
                .about("Move a todo item to another location.")
                .arg(arg!(<TODO_FROM> "The path of the todo item to move."))
                .arg(arg!(<TODO_TO> "The path to move the todo item to."))
                .arg(down_flag!()),
        )
}

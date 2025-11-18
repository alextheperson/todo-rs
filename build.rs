use clap::builder::PossibleValue;
use clap::{ArgAction, Command, ValueEnum, arg, value_parser};
use clap_complete::{Shell, generate_to};
use std::env;
use std::io::Error;

fn main() -> Result<(), Error> {
    let Some(outdir) = env::var_os("OUT_DIR") else {
        return Ok(());
    };

    let mut cmd = build();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "todo", &outdir)?;
    }

    Ok(())
}

fn build() -> Command {
    Command::new("todo")
        .version("1.0")
        .about("Manage the items that you need to do.\n\nWith no arguments, it opens a TUI to edit your .todo list")
                .arg(
                    arg!([PATH] "Specify the PATH of the `.todo` to edit.")
                        .value_parser(value_parser!(std::path::PathBuf)),
                )
        .subcommand(
            Command::new("new")
                .about("Create a new `.todo` in the current directory.")
                .arg(
                    arg!([PATH] "Specify an alternate path to create the new todo.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                ),
        )
        .subcommand(
            Command::new("next")
                .about("Create a new `.tood` in the current directory.")
                .arg(
                    arg!([PATH] "An alternate path in which to look for the next todo.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List todo items for the current directory and its parents.")
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
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
                        ]),
                )
                .arg(
                    arg!([PATH] "Specify an alternate path to search from.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                )
                .arg(
                    arg!(-a --archived "Show archived items.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-c --completed "Hide completed items.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("add")
                .about("Add an item to a todo list.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to add.")
                )
                    .arg(arg!(<ITEM_NAME> "The name of the item to add.")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove an item from a todo list.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to remove.")
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("prune")
                .about("Archive all completed todo items.")
                .arg(
                    arg!([PATH] "Specify an alternate path to search from.")
                        .default_value("./")
                        .value_parser(value_parser!(std::path::PathBuf)),
                )
                .arg(
                    arg!(-s --single "Prune only a single list.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("complete")
                .about("Mark a todo item as completed.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to complete.")
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("toggle")
                .about("Toggle the completion of a todo item.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to toggle.")
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("incomplete")
                .about("Mark a todo item as incomplete.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to mark.")
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("edit")
                .about("Edit the properties of a todo item.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to add.")
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-n --name "Set the name of the todo item.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-D --date "Set the date of the todo item.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-p --priority "Set the priority of the todo item.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-c --completed "Set whether the item is completed")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-a --archived "Set whether the item is archived")
                        .action(ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("get")
                .about("Get info about a specific todo item.")
                .arg(
                    arg!(<TODO_PATH> "The path of the todo item to get.")
                )
        )
        .subcommand(
            Command::new("move")
                .about("Move a todo item to another location.")
                .arg(
                    arg!(<TODO_FROM> "The path of the todo item to move.")
                )
                .arg(
                    arg!(<TODO_TO> "The path to move the todo item to.")
                )
                .arg(
                    arg!(-d --down "Search down through files instead of up.")
                        .action(ArgAction::SetTrue),
                )
        )
}

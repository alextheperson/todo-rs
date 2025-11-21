use output::Render;
use std::path::PathBuf;
use todo::list::ItemList;

mod command_executors;
pub mod commands;
mod date;
mod output;
mod search_paths;
mod todo;

use crate::command_executors::*;
use crate::date::Date;
use crate::output::RenderFormat;
use crate::todo::path::ItemPath;

fn main() {
    let command = commands::build();

    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => init(parse_file_path(sub_matches)),
        Some(("next", sub_matches)) => next(
            parse_file_path(sub_matches),
            sub_matches.get_flag("children"),
            sub_matches.get_flag("down"),
            parse_output_format(sub_matches),
        ),
        Some(("list", sub_matches)) => list(
            sub_matches.get_flag("down"),
            parse_output_format(sub_matches),
            parse_file_path(sub_matches),
            sub_matches.get_flag("archived"),
            !sub_matches.get_flag("completed"),
        ),
        Some(("add", sub_matches)) => add(
            parse_item_path_arg(sub_matches),
            sub_matches
                .get_one::<String>("ITEM_NAME")
                .expect("Expected an item name.")
                .to_string(),
            parse_date(sub_matches),
            sub_matches.get_one::<i16>("priority"),
            sub_matches.get_flag("down"),
        ),
        Some(("remove", sub_matches)) => remove(
            parse_item_path_arg(sub_matches),
            sub_matches.get_flag("down"),
        ),
        Some(("prune", sub_matches)) => prune(
            parse_file_path(sub_matches),
            sub_matches.get_flag("single"),
            sub_matches.get_flag("down"),
        ),
        Some(("complete", sub_matches)) => complete(
            parse_item_path_arg(sub_matches),
            sub_matches.get_flag("down"),
        ),
        Some(("toggle", sub_matches)) => toggle(
            parse_item_path_arg(sub_matches),
            sub_matches.get_flag("down"),
        ),
        Some(("incomplete", sub_matches)) => incomplete(
            parse_item_path_arg(sub_matches),
            sub_matches.get_flag("down"),
        ),
        Some(("edit", sub_matches)) => edit(
            parse_item_path_arg(sub_matches),
            sub_matches.get_flag("down"),
            sub_matches.get_one("name"),
            parse_date(sub_matches),
            sub_matches.get_one("priority"),
            sub_matches.get_one("completed"),
            sub_matches.get_one("archived"),
        ),
        Some(("get", sub_matches)) => get(
            parse_item_path_arg(sub_matches),
            parse_output_format(sub_matches),
            sub_matches.get_flag("down"),
        ),
        Some(("move", sub_matches)) => move_item(
            ItemPath::try_from(
                &sub_matches
                    .get_one::<String>("TODO_FROM")
                    .expect("Expected an item path.")
                    .clone(),
            )
            .expect(&format!(
                "Could not parse the item path '{}'.",
                sub_matches.get_one::<String>("TODO_FROM").unwrap()
            )),
            sub_matches.get_flag("down1"),
            ItemPath::try_from(
                &sub_matches
                    .get_one::<String>("TODO_TO")
                    .expect("Expected an item path.")
                    .clone(),
            )
            .expect(&format!(
                "Could not parse the item path '{}'.",
                sub_matches.get_one::<String>("TODO_TO").unwrap()
            )),
            sub_matches.get_flag("down2"),
        ),
        _ => panic!("The TUI editor has not been implemented yet."),
    }
}

/// This parses the <ITEM_PATH> arg into an ItemPath. I can't do this with clap because I can't
/// import anything into commands.rs because it is include!()ed in build.rs
fn parse_item_path_arg(matches: &clap::ArgMatches) -> ItemPath {
    let provided_path = matches
        .get_one::<String>("ITEM_PATH")
        .expect("Expected an item path.")
        .clone();

    ItemPath::try_from(&provided_path).expect(&format!(
        "Could not parse the item path '{}'.",
        &provided_path
    ))
}

fn parse_output_format(matches: &clap::ArgMatches) -> RenderFormat {
    let format = &matches
        .get_one::<String>("format")
        .expect("Format must be specified, but there should have been a default value.")[..];

    match format {
        "html" => RenderFormat::HTML,
        "html-class" => RenderFormat::HtmlClass,
        "pango" => RenderFormat::Pango,
        "plain" => RenderFormat::Plain,
        "ansi" => RenderFormat::ANSI,
        _ => panic!("Unrecognized vale for --format: '{}'", format),
    }
}

fn parse_date(matches: &clap::ArgMatches) -> Option<Date> {
    let value = matches.get_one::<String>("date");

    if value.is_some() {
        let parsed = Date::from(&value.unwrap());
        if parsed.is_ok() {
            Some(parsed.unwrap())
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_file_path(matches: &clap::ArgMatches) -> PathBuf {
    matches.get_one::<PathBuf>("FILE_PATH")
        .unwrap_or(&std::env::current_dir().expect("You need to be in a directory."))
        .canonicalize()
        .expect("There needs to be a directory specified, but there was supposed to be a default value.")
        .to_path_buf()
}

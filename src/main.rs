use clap::error::Result;
use output::Render;
use std::path::PathBuf;

mod command_executors;
pub mod commands;
mod date;
mod error;
mod output;
mod search_paths;
mod todo;

use crate::command_executors::*;
use crate::date::Date;
use crate::error::{CodeComponent, Error};
use crate::output::RenderFormat;
use crate::todo::path::ItemPath;

fn main() {
    let command = commands::build();

    let matches = command.clone().get_matches();

    let output = match_commands(matches);
    // If there is an error, print it to the user.
    if let Err(err) = output {
        err.print();
    };
}

fn match_commands(matches: clap::ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        Some(("init", sub_matches)) => init(parse_file_path(sub_matches)?),
        Some(("next", sub_matches)) => next(
            parse_file_path(sub_matches)?,
            sub_matches.get_flag("children"),
            sub_matches.get_flag("down"),
            parse_output_format(sub_matches)?,
        ),
        Some(("list", sub_matches)) => list(
            sub_matches.get_flag("down"),
            parse_output_format(sub_matches)?,
            parse_file_path(sub_matches)?,
            sub_matches.get_flag("archived"),
            !sub_matches.get_flag("completed"),
        ),
        Some(("add", sub_matches)) => add(
            parse_item_path_arg(sub_matches)?,
            sub_matches
                .get_one::<String>("ITEM_NAME")
                .expect("Expected an item name.")
                .to_string(),
            parse_date(sub_matches)?,
            sub_matches.get_one::<i64>("priority"),
            sub_matches.get_flag("down"),
        ),
        Some(("remove", sub_matches)) => remove(
            parse_item_path_arg(sub_matches)?,
            sub_matches.get_flag("down"),
        ),
        Some(("prune", sub_matches)) => prune(
            parse_file_path(sub_matches)?,
            sub_matches.get_flag("single"),
            sub_matches.get_flag("down"),
        ),
        Some(("complete", sub_matches)) => complete(
            parse_item_path_arg(sub_matches)?,
            sub_matches.get_flag("down"),
        ),
        Some(("toggle", sub_matches)) => toggle(
            parse_item_path_arg(sub_matches)?,
            sub_matches.get_flag("down"),
        ),
        Some(("incomplete", sub_matches)) => incomplete(
            parse_item_path_arg(sub_matches)?,
            sub_matches.get_flag("down"),
        ),
        Some(("edit", sub_matches)) => edit(
            parse_item_path_arg(sub_matches)?,
            sub_matches.get_flag("down"),
            sub_matches.get_one("name"),
            parse_date(sub_matches)?,
            sub_matches.get_one("priority"),
            sub_matches.get_one("completed"),
            sub_matches.get_one("archived"),
        ),
        Some(("get", sub_matches)) => get(
            parse_item_path_arg(sub_matches)?,
            parse_output_format(sub_matches)?,
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
                sub_matches
                    .get_one::<String>("TODO_FROM")
                    .expect("You must specify an item to move.")
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
                sub_matches
                    .get_one::<String>("TODO_TO")
                    .expect("You must specify a destination.")
            )),
            sub_matches.get_flag("down2"),
        ),
        _ => panic!("The TUI editor has not been implemented yet."),
    }
}

/// This parses the <ITEM_PATH> arg into an ItemPath. I can't do this with clap because I can't
/// import anything into commands.rs because it is include!()ed in build.rs
fn parse_item_path_arg(matches: &clap::ArgMatches) -> Result<ItemPath, Error> {
    let provided_path = matches
        .get_one::<String>("ITEM_PATH")
        .expect("Expected an item path.")
        .clone();

    Ok(match_error!(
        ItemPath::try_from(&provided_path),
        CodeComponent::Main,
        format!("Could not parse the item path '{}'.", &provided_path)
    ))
}

fn parse_output_format(matches: &clap::ArgMatches) -> Result<RenderFormat, Error> {
    let format = &matches
        .get_one::<String>("format")
        .expect("Format must be specified, but there should have been a default value.")[..];

    match format {
        "html" => Ok(RenderFormat::HTML),
        "html-class" => Ok(RenderFormat::HtmlClass),
        "pango" => Ok(RenderFormat::Pango),
        "plain" => Ok(RenderFormat::Plain),
        "ansi" => Ok(RenderFormat::ANSI),
        _ => Err(propagate!(
            CodeComponent::Main,
            format!("Unrecognized vale for --format: '{}'", format)
        )),
    }
}

fn parse_date(matches: &clap::ArgMatches) -> Result<Option<Date>, Error> {
    let value = matches.get_one::<String>("date");

    match value {
        Some(val) => Ok(Some(match_error!(
            Date::from(&val),
            CodeComponent::Main,
            format!("Couldn't parse input as date. Got '{}'", val)
        ))),
        _ => Ok(None),
    }
}

fn parse_file_path(matches: &clap::ArgMatches) -> Result<PathBuf, Error> {
    Ok(matches
        .get_one::<PathBuf>("FILE_PATH")
        .unwrap_or(&match_result!(
            std::env::current_dir(),
            CodeComponent::DocumentPath,
            format!("Couldn't read your current directory.")
        ))
        .to_path_buf())
}

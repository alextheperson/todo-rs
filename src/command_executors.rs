use crate::date::Date;
use crate::error::{CodeComponent, Error};
use crate::output::Render;
use crate::output::RenderFormat;
use crate::output::line::OutputLine;
use crate::todo::document::Document;
use crate::todo::item::Item;
use crate::todo::list::TodoList;
use crate::todo::path::ItemPath;
use crate::{match_error, match_result, propagate, search_paths};

use std::fs;
use std::path::PathBuf;

pub fn init(path: PathBuf) -> Result<(), Error> {
    let todo_path = path.join(".todo");
    if fs::exists(&todo_path).unwrap_or(false) {
        println!("[LIST]: '{}' already exists.", todo_path.display());
    } else {
        match_result!(
            fs::write(&todo_path, "# New Todo\n\n"),
            CodeComponent::Executor,
            format!(
                "Could not write file '.todo' at path '{path}'.",
                path = &todo_path.display(),
            )
        );

        println!("[LIST]: Created '{}'.", todo_path.display());
    }

    Ok(())
}

pub fn next(
    path: PathBuf,
    show_children: bool,
    down: bool,
    format: RenderFormat,
) -> Result<(), Error> {
    let paths: Vec<PathBuf>;

    if down {
        paths = match_error!(
            search_paths::search_down(&path),
            CodeComponent::Executor,
            format!("Could not search down from '{}'.", path.display())
        );
    } else {
        paths = match_error!(
            search_paths::search_up(&path),
            CodeComponent::FileSearcher,
            format!("Could not search up from path '{}'", path.display())
        );
    }

    let mut lists = vec![];
    for path in paths {
        lists.push(match_error!(
            Document::from_path(&path),
            CodeComponent::Executor,
            format!("Could not parse the document at path '{}'", path.display())
        ));
    }

    // Remove archived lists
    lists = lists
        .into_iter()
        .filter(|a| !a.archived)
        .rev()
        .collect::<Vec<Document>>();

    lists.sort_by(|a, b| b.priority.cmp(&a.priority));
    let mut top_list = lists[0].clone();

    top_list.items.recursive_filter(|item| item.archived);
    top_list.items.sort_by(|a, b| b.priority.cmp(&a.priority));
    let top_item = top_list.items[0].clone();

    let output = match_error!(
        top_item.format_detail(show_children),
        CodeComponent::Executor,
        format!("Could not render output.")
    );
    println!("{}", output.render(&format));

    Ok(())
}

pub fn list(
    down: bool,
    format: RenderFormat,
    path: PathBuf,
    show_archived: bool,
    show_completed: bool,
) -> Result<(), Error> {
    let search_start = path;

    let paths: Vec<PathBuf>;

    if down {
        paths = match_error!(
            search_paths::search_down(&search_start),
            CodeComponent::Executor,
            format!("Could not search down from '{}'.", search_start.display())
        );
    } else {
        paths = match_error!(
            search_paths::search_up(&search_start),
            CodeComponent::FileSearcher,
            format!("Could not search up from path '{}'", search_start.display())
        );
    }

    let mut documents = vec![];

    for path in paths {
        documents.push(match_error!(
            Document::from_path(&path),
            CodeComponent::Executor,
            format!("Could not parse the document at path '{}'", path.display())
        ));
    }

    if !show_archived {
        documents = documents
            .into_iter()
            .filter(|a| !a.archived)
            .collect::<Vec<Document>>();
    }

    documents.sort_by(|a, b| b.priority.cmp(&a.priority));

    for mut document in documents {
        if !show_archived {
            document.items.recursive_filter(|item| item.archived)
        }
        if !show_completed {
            document.items.recursive_filter(|item| item.completed)
        }

        print!(
            "{}",
            match_error!(
                document.format(),
                CodeComponent::Executor,
                format!(
                    "Could not format the document '#{}' at path '{}'",
                    document.name,
                    document.path.display()
                )
            )
            .render(&format)
        );
        print!("{}", OutputLine::newline(&format));
        print!("{}", OutputLine::newline(&format));
    }
    println!("");

    Ok(())
}

pub fn add(
    path: ItemPath,
    item_name: String,
    date: Option<Date>,
    priority: Option<&i64>,
    down: bool,
) -> Result<(), Error> {
    let mut list = match_error!(
        search_paths::find_list(&path.document.clone(), down),
        CodeComponent::Executor,
        format!("Could not find list with name '#{}'.", path.document)
    );

    let item = Item {
        name: item_name,
        date: date,
        priority: *priority.unwrap_or(&0_i64),
        completed: false,
        archived: false,
        items: vec![],
    };

    match list.items.add_item(item.clone(), path.clone()) {
        Ok(()) => {}
        Err(err) => {
            return Err(propagate!(
                CodeComponent::Executor,
                format!("Could not add item to path {}", path.display()),
                err
            ));
        }
    };

    match_error!(
        list.clone().save(),
        CodeComponent::Executor,
        format!("Could not same the document")
    );

    println!(
        "[LIST]: Added '{item_name}' to #{list_name}",
        item_name = item.name,
        list_name = list.name
    );

    Ok(())
}

pub fn complete(path: ItemPath, down: bool) -> Result<(), Error> {
    let mut list = match_error!(
        search_paths::find_list(&path.document, down),
        CodeComponent::Executor,
        format!(
            "Could not find a list with the name '{}'",
            path.document.clone()
        )
    );

    let item = match_error!(
        list.items.find(&path.clone()),
        CodeComponent::Executor,
        format!("Could not find item at path '{}'.", path.display())
    );
    item.completed = true;

    println!(
        "[LIST]: Completed '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    match_error!(
        list.clone().save(),
        CodeComponent::Executor,
        format!("Could not same the document")
    );

    Ok(())
}

pub fn toggle(path: ItemPath, down: bool) -> Result<(), Error> {
    let mut list = match_error!(
        search_paths::find_list(&path.document, down),
        CodeComponent::Executor,
        format!(
            "Could not find a list with the name '{}'",
            path.document.clone()
        )
    );

    let item = match_error!(
        list.items.find(&path.clone()),
        CodeComponent::Executor,
        format!("Could not find item at path '{}'.", path.display())
    );
    item.completed = !item.completed;

    println!(
        "[LIST]: Toggled '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    match_error!(
        list.clone().save(),
        CodeComponent::Executor,
        format!("Could not same the document")
    );

    Ok(())
}

pub fn incomplete(path: ItemPath, down: bool) -> Result<(), Error> {
    let mut list = match_error!(
        search_paths::find_list(&path.document, down),
        CodeComponent::Executor,
        format!(
            "Could not find a list with the name '{}'",
            path.document.clone()
        )
    );

    let item = match_error!(
        list.items.find(&path.clone()),
        CodeComponent::Executor,
        format!("Could not find item at path '{}'.", path.display())
    );
    item.completed = false;

    println!(
        "[LIST]: marked '{item_name}' in #{list_name} as incomplete.",
        item_name = item.name,
        list_name = list.name
    );

    match_error!(
        list.clone().save(),
        CodeComponent::Executor,
        format!("Could not same the document")
    );

    Ok(())
}

pub fn prune(path: PathBuf, single: bool, down: bool) -> Result<(), Error> {
    let search_start = path;

    let paths = if single {
        vec![search_start]
    } else {
        if down {
            match_error!(
                search_paths::search_down(&search_start),
                CodeComponent::Executor,
                format!("Could not search down from '{}'.", search_start.display())
            )
        } else {
            match_error!(
                search_paths::search_up(&search_start),
                CodeComponent::FileSearcher,
                format!("Could not search up from path '{}'", search_start.display())
            )
        }
    };

    for path in paths {
        let mut document = match_error!(
            Document::from_path(&path),
            CodeComponent::Executor,
            format!("Could not parse the document at path '{}'", path.display())
        );
        document.items.prune();

        match_error!(
            document.clone().save(),
            CodeComponent::Executor,
            format!("Could not same the document")
        );

        println!(
            "[LIST]: Pruned #{list_name} at '{list_path}'",
            list_name = document.name,
            list_path = document.path.display()
        );
    }

    Ok(())
}

pub fn remove(path: ItemPath, down: bool) -> Result<(), Error> {
    let mut list = match_error!(
        search_paths::find_list(&path.document, down),
        CodeComponent::Executor,
        format!(
            "Could not find a list with the name '{}'",
            path.document.clone()
        )
    );

    let item = match_error!(
        list.items.remove_by_path(&path),
        CodeComponent::Executor,
        format!("Could not remove the item at path '{}'.", path.display())
    );

    println!(
        "[LIST]: Removed '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    match_error!(
        list.clone().save(),
        CodeComponent::Executor,
        format!("Could not same the document")
    );
    Ok(())
}

pub fn get(path: ItemPath, format: RenderFormat, down: bool) -> Result<(), Error> {
    let mut list = match_error!(
        search_paths::find_list(&path.document, down),
        CodeComponent::Executor,
        format!(
            "Could not find a list with the name '{}'",
            path.document.clone()
        )
    );

    if path.item_prefixes.len() == 0 {
        println!(
            "{}",
            match_error!(
                list.format(),
                CodeComponent::Executor,
                format!(
                    "Could not format the document '#{}' at path '{}'",
                    list.name,
                    list.path.display()
                )
            )
            .render(&format)
        );
    } else {
        let item = match_error!(
            list.items.find(&path.clone()),
            CodeComponent::Executor,
            format!("Could not find item at path '{}'", path.display())
        );
        let output = match_error!(
            item.format_detail(true),
            CodeComponent::Executor,
            format!("Could not render output.")
        );
        println!("{}", output.render(&format));
    }

    Ok(())
}

pub fn edit(
    path: ItemPath,
    down: bool,
    name: Option<&String>,
    date: Option<Date>,
    priority: Option<&i64>,
    completed: Option<&bool>,
    archived: Option<&bool>,
) -> Result<(), Error> {
    let mut list = match_error!(
        search_paths::find_list(&path.document, down),
        CodeComponent::Executor,
        format!(
            "Could not find a list with the name '{}'",
            path.document.clone()
        )
    );

    let item = match_error!(
        list.items.find(&path.clone()),
        CodeComponent::Executor,
        format!("Could not find item at path '{}'.", path.display())
    );

    if let Some(name) = name {
        item.name = name.clone();
        println!("Set name to '{}'", name);
    }
    if let Some(date) = date {
        item.date = Some(date);
        println!("Set date to '{}'", date.display());
    }
    if let Some(priority) = priority {
        item.priority = *priority;
        println!("Set name to '{}'", priority);
    }
    if let Some(completed) = completed {
        item.completed = *completed;
        println!("Set name to '{}'", completed);
    }
    if let Some(archived) = archived {
        item.archived = *archived;
        println!("Set name to '{}'", archived);
    }

    println!("\nNew Item Values:");

    let output = match_error!(
        item.format_detail(false),
        CodeComponent::Executor,
        format!("Could not render output.")
    );
    println!("{}", output.render(&RenderFormat::ANSI));

    match_error!(
        list.clone().save(),
        CodeComponent::Executor,
        format!("Could not same the document")
    );

    Ok(())
}

pub fn move_item(
    from_path: ItemPath,
    down1: bool,
    to_path: ItemPath,
    down2: bool,
) -> Result<(), Error> {
    println!("Moving {} -> {}", from_path.display(), to_path.display());
    if from_path == to_path {
        // No-op
        return Ok(());
    }
    let mut list1 = match_error!(
        search_paths::find_list(&from_path.document, down1),
        CodeComponent::Executor,
        format!(
            "Could not find a list named '{}'",
            from_path.document.clone()
        )
    );
    let item1 = match_error!(
        list1.items.remove_by_path(&from_path),
        CodeComponent::Executor,
        format!("Could not remove item at path '{}'.", from_path.display())
    );

    let mut list2 = match_error!(
        search_paths::find_list(&to_path.document, down2),
        CodeComponent::Executor,
        format!("Could not find list with name '#{}'.", to_path.document)
    );

    let add_item_result = list2.items.add_item(item1.clone(), to_path.clone());
    match add_item_result {
        Ok(_) => {
            match_error!(
                list1.clone().save(),
                CodeComponent::Executor,
                format!("Could not same the target document")
            );
            match_error!(
                list2.clone().save(),
                CodeComponent::Executor,
                format!("Could not same the destination document")
            );

            Ok(())
        }
        Err(err) => Err(propagate!(
            CodeComponent::Executor,
            format!(
                "Could not add target item '{target}' to destination '{dest}'",
                target = from_path.display(),
                dest = to_path.display()
            ),
            err
        )),
    }
}

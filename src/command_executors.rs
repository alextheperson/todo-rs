use crate::date::Date;
use crate::output::Render;
use crate::output::RenderFormat;
use crate::output::line::OutputLine;
use crate::search_paths;
use crate::todo::document::Document;
use crate::todo::item::Item;
use crate::todo::list::ItemList;
use crate::todo::path::ItemPath;

use std::fs;
use std::path::PathBuf;

pub fn init(path: PathBuf) {
    let todo_path = path.join(".todo");
    if fs::exists(&todo_path).unwrap_or(false) {
        println!("[LIST]: '{}' already exists.", todo_path.display());
    } else {
        fs::write(&todo_path, "# New Todo\n\n")
            .expect(&format!("Could not create '{}'.", todo_path.display()));
        println!("[LIST]: Created '{}'.", todo_path.display());
    }
}

pub fn next(path: PathBuf, show_children: bool, down: bool, format: RenderFormat) {
    let paths: Vec<PathBuf>;

    if down {
        paths = search_paths::search_down(&path);
    } else {
        paths = search_paths::search_up(path);
    }

    let mut lists = paths
        .into_iter()
        .map(|val| Document::from_path(&val))
        .collect::<Vec<Document>>();

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

    println!("{}", top_item.format_detail(show_children).render(&format));
}

pub fn list(
    down: bool,
    format: RenderFormat,
    path: PathBuf,
    show_archived: bool,
    show_completed: bool,
) {
    let search_start = path;

    let paths: Vec<PathBuf>;

    if down {
        paths = search_paths::search_down(&search_start);
    } else {
        paths = search_paths::search_up(search_start);
    }

    let mut lists = paths
        .into_iter()
        .map(|val| Document::from_path(&val))
        .collect::<Vec<Document>>();

    lists.sort_by(|a, b| b.priority.cmp(&a.priority));

    if !show_archived {
        lists = lists
            .into_iter()
            .filter(|a| !a.archived)
            .collect::<Vec<Document>>();
    }

    for mut list in lists {
        if !show_archived {
            list.items.recursive_filter(|item| item.archived)
        }
        if !show_completed {
            list.items.recursive_filter(|item| item.completed)
        }

        print!("{}", list.clone().format(list.path).render(&format));
        print!("{}", OutputLine::newline(&format));
        print!("{}", OutputLine::newline(&format));
    }
    println!("");
}

pub fn add(
    path: ItemPath,
    item_name: String,
    date: Option<Date>,
    priority: Option<&i16>,
    down: bool,
) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = Item {
        name: item_name,
        date: date,
        priority: *priority.unwrap_or(&0_i16),
        completed: false,
        archived: false,
        items: vec![],
    };

    list.items.add_item(item.clone(), path.clone());

    list.clone().save();

    println!(
        "[LIST]: Added '{item_name}' to #{list_name}",
        item_name = item.name,
        list_name = list.name
    );
}

pub fn complete(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.find(path.clone()).unwrap();
    item.completed = true;

    println!(
        "[LIST]: Completed '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

pub fn toggle(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.find(path.clone()).unwrap();
    item.completed = !item.completed;

    println!(
        "[LIST]: Toggled '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

pub fn incomplete(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.find(path.clone()).unwrap();
    item.completed = false;

    println!(
        "[LIST]: marked '{item_name}' in #{list_name} as incomplete.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

pub fn prune(path: PathBuf, single: bool, down: bool) {
    let search_start = path;

    let paths = if single {
        vec![search_start]
    } else {
        if down {
            search_paths::search_down(&search_start)
        } else {
            search_paths::search_up(search_start)
        }
    };

    for path in paths {
        let mut document = Document::from_path(&path);
        document.items.prune();
        document.save();
        println!(
            "[LIST]: Pruned #{list_name} at '{list_path}'",
            list_name = document.name,
            list_path = document.path.display()
        );
    }
}

pub fn remove(path: ItemPath, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.remove_by_path(path).unwrap();

    println!(
        "[LIST]: Removed '{item_name}' in #{list_name}.",
        item_name = item.name,
        list_name = list.name
    );

    list.save();
}

pub fn get(path: ItemPath, format: RenderFormat, down: bool) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    if path.item_prefixes.len() == 0 {
        println!("{}", list.format(list.path.clone()).render(&format));
    } else {
        let item = list.items.find(path.clone()).unwrap();
        println!("{}", item.format_detail(true).render(&format));
    }
}

pub fn edit(
    path: ItemPath,
    down: bool,
    name: Option<&String>,
    date: Option<Date>,
    priority: Option<&i16>,
    completed: Option<&bool>,
    archived: Option<&bool>,
) {
    let mut list = search_paths::find_list(path.document.clone(), down).expect(&format!(
        "Could not find a list with the name '{}'",
        path.document.clone()
    ));

    let item = list.items.find(path.clone()).unwrap();

    if name.is_some() {
        item.name = name.unwrap().clone();
        println!("Set name to '{}'", name.unwrap());
    }
    if date.is_some() {
        item.date = Some(date.unwrap());
        println!("Set date to '{}'", date.unwrap().display());
    }
    if priority.is_some() {
        item.priority = priority.unwrap().clone();
        println!("Set name to '{}'", priority.unwrap());
    }
    if completed.is_some() {
        item.completed = completed.unwrap().clone();
        println!("Set name to '{}'", completed.unwrap());
    }
    if archived.is_some() {
        item.archived = archived.unwrap().clone();
        println!("Set name to '{}'", archived.unwrap());
    }

    println!("\nNew Item Values:");

    println!("{}", item.format_detail(false).render(&RenderFormat::ANSI));

    list.save();
}

pub fn move_item(from_path: ItemPath, down1: bool, to_path: ItemPath, down2: bool) {
    println!("Moving {} -> {}", from_path.display(), to_path.display());
    if from_path == to_path {
        // No-op
        return;
    }
    let mut list1 = search_paths::find_list(from_path.document.clone(), down1).expect(&format!(
        "Could not find a list with the name '{}'",
        from_path.document.clone()
    ));

    let item1 = list1
        .items
        .remove_by_path(from_path.clone())
        .expect(&format!(
            "Could not remvove item '{}'",
            &from_path.display(),
        ));

    let mut list2 = search_paths::find_list(to_path.document.clone(), down2).expect(&format!(
        "Could not find a list with the name '{}'",
        to_path.document.clone()
    ));

    list2.items.add_item(item1.clone(), to_path.clone());

    list1.save();
    list2.save();
}

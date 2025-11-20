use std::io::{Error, ErrorKind};

use crate::todo::{document, item};

// #list/item1/nesteditem
#[derive(Debug, Clone)]
pub struct ItemPath {
    pub document: String,
    pub item_prefixes: Vec<String>,
}

impl std::convert::TryFrom<&str> for ItemPath {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut segments = value.split("/");
        let mut document = None;

        let first_segment = segments.clone().next();

        if first_segment.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "An Item Path cannot start with a '/'.",
            ));
        };

        if first_segment.unwrap().starts_with("#") {
            document = Some(first_segment.unwrap()[1..].to_string());
            segments.next();
        };

        let mut new_path = ItemPath::new(document, vec![]);

        for piece in segments {
            if piece == "" {
                continue;
            }

            new_path.item_prefixes.push(piece.to_string());
        }

        println!("{:?}", new_path);

        Ok(new_path)
    }
}

impl std::convert::TryFrom<&String> for ItemPath {
    type Error = std::io::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        return ItemPath::try_from(&value[..]);
    }
}

impl ItemPath {
    pub fn new(document: Option<String>, prefixes: Vec<String>) -> ItemPath {
        if document.is_some() {
            ItemPath {
                document: document.unwrap(),
                item_prefixes: prefixes,
            }
        } else {
            ItemPath {
                document: document::Document::from_path(
                    &std::env::current_dir().expect("You need to be in a directory."),
                )
                .name,
                item_prefixes: prefixes,
            }
        }
    }

    pub fn matches(self, item: item::Item) -> bool {
        if self.item_prefixes.len() == 0 {
            return false;
        }

        item.name
            .to_ascii_lowercase()
            .starts_with(&self.item_prefixes[0].to_ascii_lowercase())
    }

    pub fn shifted(self) -> ItemPath {
        ItemPath {
            document: self.document,
            item_prefixes: self.item_prefixes[1..].to_vec(),
        }
    }
}

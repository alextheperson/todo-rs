use crate::{
    error::{CodeComponent, Error},
    match_error, match_result, propagate,
    todo::{document, item},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemPath {
    pub document: String,
    pub item_prefixes: Vec<String>,
}

impl ItemPath {
    pub fn try_from(value: &String) -> Result<Self, Error> {
        let mut segments = value.split("/");
        let mut document = None;

        let first_segment = match segments.clone().next() {
            Some(val) => val,
            _ => {
                return Err(propagate!(
                    CodeComponent::DocumentPath,
                    format!("Couldn't split the string ('{value}') apart. Is it missing slashes?")
                ));
            }
        };

        if first_segment.starts_with("#") {
            document = Some(first_segment[1..].to_string());
            segments.next();
        } else if first_segment == "" {
            // If the first item is blank, that means that the path starts with a slash, so we
            // should infer the document name.
            document = None;
            segments.next();
        };

        let mut prefxes = vec![];

        let segment_count = segments.clone().count();

        for (i, piece) in segments.into_iter().enumerate() {
            if piece == "" {
                if i == segment_count - 1 {
                    // It isn't an error if it ends with a slash
                    continue;
                } else {
                    return Err(propagate!(
                        CodeComponent::DocumentPath,
                        format!("There appears to be an empty section in the path ('{value}').")
                    ));
                }
            } else {
                prefxes.push(piece.to_string());
            }
        }

        let new_path = match_error!(
            ItemPath::new(document, prefxes),
            CodeComponent::DocumentPath,
            format!("Couldn't create the Item Path.")
        );

        Ok(new_path)
    }
}

impl ItemPath {
    pub fn new(document: Option<String>, prefixes: Vec<String>) -> Result<ItemPath, Error> {
        let current_dir = match_result!(
            std::env::current_dir(),
            CodeComponent::DocumentPath,
            "Couldn't read your current directory".to_string()
        );
        let normalized_document = document.unwrap_or(
            match_error!(
                document::Document::from_path(&current_dir),
                CodeComponent::DocumentPath,
                format!(
                    "Could not fetch the document in your current directory ('{}')",
                    current_dir.display()
                )
            )
            .name,
        );

        Ok(ItemPath {
            document: normalized_document,
            item_prefixes: prefixes,
        })
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

    pub fn display(&self) -> String {
        return format!("#{}/{}", self.document, self.item_prefixes.join("/"));
    }
}

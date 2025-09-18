#[derive(Debug, Clone)]
pub struct Item {
    pub completed: bool,
    pub priority: i16,
    pub date: String,
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct List {
    pub name: String,
    pub items: Vec<Item>
}

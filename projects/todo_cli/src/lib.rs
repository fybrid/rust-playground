use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
}

impl TodoList {
    pub fn add(&mut self, title: impl Into<String>) -> u64 {
        let next_id = self.items.iter().map(|item| item.id).max().unwrap_or(0) + 1;
        self.items.push(TodoItem {
            id: next_id,
            title: title.into(),
            completed: false,
        });
        next_id
    }

    pub fn complete(&mut self, id: u64) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.completed = true;
            true
        } else {
            false
        }
    }

    pub fn pending(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| !item.completed).collect()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }
}

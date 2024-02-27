// src/todo_list.rs

use crate::todo_item::TodoItem;

pub struct TodoList {
    pub items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    pub fn add(&mut self, text: String) {
        let id = self.items.len() + 1;
        let item = TodoItem {
            id,
            text,
            status: false,
        };
        self.items.push(item);
    }

    pub fn remove(&mut self, id: usize) -> Option<TodoItem> {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn list(&self) {
        for item in &self.items {
            println!("{:?}", item);
        }
    }

    pub fn toggle_status(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.status = !item.status;
        }
    }
}

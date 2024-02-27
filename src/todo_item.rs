// src/todo_item.rs

#[derive(Debug)]
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub status: bool,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn add(&mut self, title: String) {
        let id = self.next_id();

        let todo = Todo {
            id,
            title,
            completed: false,
            created_at: Utc::now(),
        };

        self.todos.push(todo);
    }

    fn next_id(&self) -> u32 {
        self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    pub fn save_to_file(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("序列化 TodoList 失败");

        fs::write(path, json).expect("写入文件失败");
    }
}

impl TodoList {
    pub fn load_from_file(path: &str) -> Self {
        let content = std::fs::read_to_string(path);

        match content {
            Ok(text) => {
                let list: TodoList = serde_json::from_str(&text).expect("反序列化 JSON 失败");
                list
            }
            Err(_) => {
                // 文件不存在或读取失败，返回空列表
                TodoList::new()
            }
        }
    }
}

impl TodoList {
    pub fn list(&self) -> &Vec<Todo> {
        &self.todos
    }
}

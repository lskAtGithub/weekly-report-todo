#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
    created_at: u64,
}

#[derive(Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    fn add(&mut self, title: String) {
        let id = self.todos.len() as u32 + 1;

        let todo = Todo {
            id,
            title,
            completed: false,
            created_at: current_timestamp(),
        };

        self.todos.push(todo);
    }

    fn list(&self) -> &Vec<Todo> {
        &self.todos
    }
}

fn main() {
    let mut list = TodoList::new();

    list.add("学习 Rust".to_string());
    list.add("写 TodoList".to_string());

    println!("{:#?}", list.list());
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

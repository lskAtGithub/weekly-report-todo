mod todo;

use std::env;
use todo::TodoList;

fn main() {
    command_operation();
}

fn command_operation() {
    let mut list = TodoList::load_from_file("todos.json").unwrap_or_else(|_| TodoList::new());
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let title = args.get(2).expect("请提供 todo 标题");
            list.add(title.to_string());
            list.save_to_file("todos.json");
            println!("Todo 已添加");
        }

        Some("done") => {
            let id: u32 = args
                .get(2)
                .expect("请提供 todo id")
                .parse()
                .expect("id 必须是数字");

            if list.done(id) {
                list.save_to_file("todos.json");
                println!("Todo {} 已完成", id);
            } else {
                println!("未找到 id = {}", id);
            }
        }

        Some("del") => {
            let id: u32 = args
                .get(2)
                .expect("请提供 todo id")
                .parse()
                .expect("id 必须是数字");

            if list.del(id) {
                list.save_to_file("todos.json");
                println!("Todo {} 已删除", id);
            } else {
                println!("未找到 id = {}", id);
            }
        }

        Some("list") => {
            println!("========== Todo List ==========");
            for todo in list.list() {
                let status = if todo.completed { "✓" } else { " " };

                println!("[{}] #{:<3} {}", status, todo.id, todo.title);
            }
            println!("===============================");
        }

        _ => {
            println!("用法:");
            println!("  add <title>");
            println!("  done <id>");
            println!("  remove <id>");
            println!("  list");
        }
    }
}

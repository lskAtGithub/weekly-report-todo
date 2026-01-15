mod todo;

use todo::TodoList;

fn main() {
    let mut list = TodoList::new();

    list.add("学习 Rust".to_string());
    list.add("写 JSON Todo".to_string());

    list.save_to_file("todos.json");

    println!("{:#?}", list);
}

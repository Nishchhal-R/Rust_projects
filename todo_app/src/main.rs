use std::env;

struct TodoItem {
    name: String,
    completion: char
}

struct TodoList{
    list: Vec<TodoItem>
}

impl TodoItem{
    fn new(name: String) -> TodoItem{
        return TodoItem{
            name: name,
            completion: ' '
        }
    }
}
impl TodoList {
    fn new() -> TodoList{
        return TodoList{list: Vec::new()};
    }

    fn add_to_list(&mut self, name: String){
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self){
        for item in &self.list {
            println!("[{}] {}", item.completion, item.name);
        }
    }

}


fn main() {
    let  arguments: Vec<String>  = env::args().collect();
    let command = arguments[1].clone();
    let todo_item_1 = "first item".to_string();
    let todo_item_2 = "second item".to_string();
    let mut todo_list = TodoList::new();

    todo_list.add_to_list(todo_item_1);
    todo_list.add_to_list(todo_item_2);

    if command== "get" {
        todo_list.print();
    }
    else if command == "add"{
        let task=arguments[2].clone();

        todo_list.add_to_list(task);
        todo_list.print();
    }

}
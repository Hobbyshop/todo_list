pub struct Todo {
    pub name: String,
    pub complete: bool
}

impl Todo {

    pub fn new(name: String) -> Todo {
        Todo { name, complete: false }
    }

    pub fn complete(&mut self) {
        self.complete = true;
    }

    pub fn copy(&self) -> Todo {
        Todo { name: self.name.to_string(), complete: self.complete }
    }
    
}

pub static mut TODO_LIST: Vec<Todo> = vec![];

pub fn get_todo_by_name(name: String) -> Option<Todo> {
    for t in unsafe { &TODO_LIST } {
        if name == t.name {
            return Some(t.copy());
        }
    }
    None
}

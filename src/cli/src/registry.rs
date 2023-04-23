use todo_manager::*;

pub struct Action<'a> {
    pub name: &'a str,
    pub arg_count: i32,
    pub function: fn(args: &Vec<String>)
}

pub static ACTIONS: [Action; 7] = [
    Action {
        name: "help",
        arg_count: 0,
        function: |_args| {
            send_help();
        }
    },
    Action {
        name: "list",
        arg_count: 0,
        function: |_args| {
            for (i, t) in unsafe { TODO_LIST.iter().enumerate() } {
                let char = if t.complete {
                    '✓'
                } else {
                    '✗'
                };
                println!("{}. {} | {}", i + 1, t.name, char);
            }
        }
    },
    Action {
        name: "add",
        arg_count: 1,
        function: |args| {
            unsafe {
                TODO_LIST.push(Todo::new(args[0].to_string()));
                println!("Added {} to list!", args[0]);
            };
        }
    },
    Action {
        name: "remove",
        arg_count: 1,
        function: |args| {
            let index: Result<usize, _> = args[0].parse();
            match index {
                Ok(i) => unsafe {
                    if (i - 1) >= TODO_LIST.len() {
                        println!("Please enter a valid index!");
                        return;
                    }
                    TODO_LIST.remove(i - 1);
                    println!("Finished todo at index {}", i);
                },
                Err(_) => println!("Please enter a valid index!")
            }
        }
    },
    Action {
        name: "finish",
        arg_count: 1,
        function: |args| {
            let index: Result<usize, _> = args[0].parse();
            match index {
                Ok(i) => unsafe {
                    if (i - 1) >= TODO_LIST.len() {
                        println!("Please enter a valid index!");
                        return;
                    }
                    TODO_LIST[i - 1].complete();
                    println!("Finished todo at index {}", i);
                },
                Err(_) => println!("Please enter a valid index!")
            }
        }
    },
    Action {
        name: "clean",
        arg_count: 0,
        function: |_args| {
            unsafe {
                for (i, todo) in TODO_LIST.iter().enumerate() {
                    if todo.complete {
                        TODO_LIST.remove(i);
                    }
                }
            }
            println!("Removed all finished todos from list!");
        }
    },
    Action {
        name: "exit",
        arg_count: 0,
        function: |_args| {
            std::process::exit(0);
        }
    }
];

fn send_help() {
    println!("help - Prints this list");
    println!("list - Lists every todo and if it's done or not");
    println!("add <name> - Adds an item with the given name to the list");
    println!("remove <index> - Removes the given item from the list");
    println!("finish <index> - Marks the given item as finished");
    println!("clean - Removes every finshed item from the list");
    println!("exit - Used to exit out of the action prompt");
}

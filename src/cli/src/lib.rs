use std::io::{stdin, stdout, Write};

mod registry;

pub fn perfom_action(args: &Vec<String>) {
    if args.len() == 0 {
        return;
    }

    for a in &registry::ACTIONS {
        if a.name != args[0] {
            continue;
        }
        if args.len() - 1 != a.arg_count as usize {
            println!("Invalid ammount of args, requires {} argument/s", a.arg_count);
            return;
        }
        
        let mut args = args.to_vec();
        args.remove(0);
        (a.function)(&args);
        return;
    }
    println!("Could not find this action!");
    println!("Use 'help' to geta list of all actions.");
}

pub fn run_action_prompt() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split_ascii_whitespace().collect();
        let args = args.iter().map(|s| {
            String::from(*s)
        }).collect();

        perfom_action(&args)
    }
}

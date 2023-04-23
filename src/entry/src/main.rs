fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() != 0 {
        cli::perfom_action(&args);
    } else {
        cli::run_action_prompt();
    }
}

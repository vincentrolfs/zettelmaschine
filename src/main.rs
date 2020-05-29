use std::{env, process};
use zettelmaschine::Zettelmaschine;

const NUMBER_OF_ARGUMENTS: i32 = 1;

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    let args_count = (args.len() as i32) - 1;

    if args_count != NUMBER_OF_ARGUMENTS {
        eprintln!("Application error: Wrong number of arguments. Expected {} arguments, got {}.", NUMBER_OF_ARGUMENTS, args_count);
        process::exit(1);
    }

    let path = args.get(1).unwrap_or_else(|| {
        println!("Could not parse filepath.");
        process::exit(1);
    });

    return path.clone();
}

fn main() {
    let path = get_path();
    Zettelmaschine::new(path);
}

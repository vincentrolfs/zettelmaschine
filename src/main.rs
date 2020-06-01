use std::{env, process};
use zettelmaschine::Zettelmaschine;

const NUMBER_OF_ARGUMENTS: i32 = 1;

fn get_base_dir() -> String {
    let args: Vec<String> = env::args().collect();
    let args_count = (args.len() as i32) - 1;

    if args_count != NUMBER_OF_ARGUMENTS {
        eprintln!("Application error: Wrong number of arguments. Expected {} arguments, got {}.", NUMBER_OF_ARGUMENTS, args_count);
        process::exit(1);
    }

    let base_dir = args.get(1).unwrap_or_else(|| {
        println!("Could not parse base directory.");
        process::exit(1);
    });

    return base_dir.clone();
}

fn main() {
    let base_dir = get_base_dir();
    let maschine = Zettelmaschine::new(base_dir);

    maschine.run();
}

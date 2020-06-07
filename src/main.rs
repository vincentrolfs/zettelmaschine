mod zettelmaschine;
mod zettel_reader;
mod zettel;
mod commands;

use std::{env, process};
use zettelmaschine::Zettelmaschine;

const NUMBER_OF_ARGUMENTS: i32 = 1;

fn get_base_dir() -> String {
    let mut args= env::args();

    // First element of env::args() is the program name, so we subtract by 1
    let program_args_count = (args.len() as i32) - 1;

    if program_args_count != NUMBER_OF_ARGUMENTS {
        eprintln!("Application error: Wrong number of arguments. Expected {} arguments, got {}.", NUMBER_OF_ARGUMENTS, program_args_count);
        process::exit(1);
    }

    let base_dir = args.nth(1).unwrap_or_else(|| {
        println!("Could not parse base directory.");
        process::exit(1);
    });

    return base_dir;
}

fn main() {
    let base_dir = get_base_dir();
    let maschine = Zettelmaschine::new(base_dir);

    maschine.run();
}

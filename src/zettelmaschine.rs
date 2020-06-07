use std::collections::HashSet;
use std::io::{stdin, stdout, Write};

use crate::zettel::Zettel;
use crate::zettel_reader::ZettelReader;
use crate::commands;

pub struct Zettelmaschine {
    zettel: HashSet<Zettel>
}

impl Zettelmaschine {
    pub fn new(base_dir: String) -> Zettelmaschine {
        return Zettelmaschine {
            zettel: ZettelReader::get_zettel(base_dir)
        };
    }

    pub fn run(&self) {
        loop {
            print!("zettelmaschine > ");
            stdout().flush().expect("Could not flush stdout.");

            let input = Self::get_input();

            self.match_input(input);
        }
    }

    fn match_input(&self, input: Option<String>) {
        match input {
            Some(input) => {
                let mut input_iter = input.as_str().trim().split_ascii_whitespace();
                let command = input_iter.next().unwrap_or("");
                let arguments: Vec<&str> = input_iter.collect();

                match command {
                    "tags" => commands::tags(&self.zettel),
                    "zettel" => commands::zettel(&self.zettel, arguments),
                    "" => {}
                    _ => println!("Unknown command: {}", command)
                }
            }
            _ => {
                eprintln!("Could not read input.")
            }
        }
    }

    fn get_input() -> Option<String> {
        let mut input = String::new();

        return match stdin().read_line(&mut input) {
            Ok(_) => Some(input),
            _ => None
        };
    }
}

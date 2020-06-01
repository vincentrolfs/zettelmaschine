mod zettel_reader;
mod zettel;

use std::collections::HashSet;
use std::fs::{read_dir, ReadDir, read_to_string};
use std::path::PathBuf;
use std::{process, io};
use std::hash::{Hash, Hasher};
use regex::{Regex, Match};
use zettel_reader::ZettelReader;
use zettel::Zettel;
use std::iter::FromIterator;
use std::io::{stdin, stdout, Write};

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
        while true {
            print!("zettelmaschine > ");
            let input = Self::get_input();

            self.match_input(input);
        }
    }

    fn match_input(&self, input_opt: Option<String>) {
        match input_opt {
            Some(input) => match input.as_str().trim() {
                "tags" => self.print_all_tags(),
                "" => {},
                _ => println!("Unknown command: {}", input)
            },
            _ => {
                println!("Could not read input.")
            }
        }
    }

    fn get_input() -> Option<String> {
        let mut input = String::new();

        stdout().flush();
        match stdin().read_line(&mut input) {
            Ok(_) => return Some(input),
            _ => return None
        }
    }

    fn print_all_tags(&self) {
        let mut all_tags = HashSet::new();

        for zettel in self.zettel.iter() {
            all_tags.extend(&zettel.tags);
        }

        let mut all_tags_sorted = Vec::from_iter(all_tags);
        all_tags_sorted.sort();

        for tag in all_tags_sorted.iter() {
            println!("{}", tag);
        }
    }
}

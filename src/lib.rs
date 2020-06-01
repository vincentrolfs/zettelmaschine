use std::collections::HashSet;
use std::io::{stdin, stdout, Write};
use std::iter::FromIterator;
use zettel::Zettel;
use zettel_reader::ZettelReader;
mod zettel_reader;
mod zettel;

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
                let input = input.as_str().trim();
                match input {
                    "tags" => self.print_all_tags(),
                    "" => {}
                    _ => println!("Unknown command: {}", input)
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

mod zettel_reader;
mod zettel;

use std::collections::HashSet;
use std::fs::{read_dir, ReadDir, read_to_string};
use std::path::PathBuf;
use std::process;
use std::hash::{Hash, Hasher};
use regex::{Regex, Match};
use zettel_reader::ZettelReader;
use zettel::Zettel;

pub struct Zettelmaschine {
    paths: HashSet<PathBuf>,
    zettel: HashSet<Zettel>
}

impl Zettelmaschine {
    pub fn new(base_dir: String) -> Zettelmaschine {
        return Zettelmaschine {
            paths: ZettelReader::get_paths(base_dir),
            zettel: HashSet::new()
        };
    }

    pub fn run(&mut self) {
        for path in self.paths.iter() {
            match read_to_string(path) {
                Ok(content) => {
                    self.zettel.insert(Zettel::new(content));
                },
                _ => {}
            }
        }
    }
}

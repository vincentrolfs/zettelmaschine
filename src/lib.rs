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
    zettel: HashSet<Zettel>
}

impl Zettelmaschine {
    pub fn new(base_dir: String) -> Zettelmaschine {
        return Zettelmaschine {
            zettel: ZettelReader::get_zettel(base_dir)
        };
    }

    pub fn run(&self) {

    }
}

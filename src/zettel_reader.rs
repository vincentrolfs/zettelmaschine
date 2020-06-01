use std::collections::HashSet;
use std::fs::{read_dir, ReadDir, read_to_string};
use std::process;
use crate::zettel::Zettel;

pub struct ZettelReader;

impl ZettelReader {
    pub fn get_zettel(base_dir: String) -> HashSet<Zettel> {
        let dir_entries = Self::get_dir_iter(base_dir);
        let mut zettel = HashSet::new();

        for dir_entry_result in dir_entries {
            match dir_entry_result {
                Ok(dir_entry) => match read_to_string(dir_entry.path()) {
                    Ok(content) => {
                        zettel.insert(Zettel::new(content, dir_entry.path()));
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        return zettel;
    }

    fn get_dir_iter(base_dir: String) -> ReadDir {
        let dir_iter = read_dir(base_dir).unwrap_or_else(|err| {
            eprintln!("Could not list files in base directory: {}", err);
            process::exit(1);
        });

        return dir_iter;
    }
}

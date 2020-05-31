use std::collections::HashSet;
use std::fs::{read_dir, ReadDir};
use std::process;
use std::path::PathBuf;

pub struct ZettelReader;

impl ZettelReader {
    pub fn get_paths(base_dir: String) -> HashSet<PathBuf> {
        let read_dir_iter = read_dir(base_dir).unwrap_or_else(|err| {
            println!("Could not list files in base directory: {}", err);
            process::exit(1);
        });

        return Self::get_paths_from_iter(read_dir_iter);
    }

    fn get_paths_from_iter(read_dir_iter: ReadDir) -> HashSet<PathBuf> {
        let mut paths = HashSet::new();

        for dir_entry_result in read_dir_iter {
            match dir_entry_result {
                Ok(dir_entry) => {
                    paths.insert(dir_entry.path());
                }
                _ => {}
            }
        }

        paths
    }
}

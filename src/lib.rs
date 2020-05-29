use std::collections::HashSet;
use std::fs::{read_dir, ReadDir, read_to_string};
use std::path::PathBuf;
use std::process;
use std::hash::{Hash, Hasher};
use regex::{Regex, Match};

pub struct Zettelmaschine {
    paths: HashSet<PathBuf>,
    zettel: HashSet<Zettel>
}

impl Zettelmaschine {
    pub fn new(base_dir: String) -> Zettelmaschine {
        return Zettelmaschine {
            paths: ZettelPaths::get_paths(base_dir),
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

#[derive(Eq)]
struct Zettel {
    content: String,
    tags: HashSet<String>
}

impl Zettel {
    fn new(content: String) -> Zettel {
        return Zettel {
            content: content.clone(),
            tags: Self::find_tags(&content)
        }
    }

    fn find_tags(content: &String) -> HashSet<String> {
        let re = Regex::new(r"#(\w+)").unwrap();
        let iter = re.find_iter(&content);
        let mut tags: HashSet<String> = HashSet::new();

        for s in iter {
            tags.insert(s.as_str().parse().unwrap());
        }

        return tags;
    }
}

impl PartialEq for Zettel {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Hash for Zettel {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.content.hash(hasher);
    }
}

struct ZettelPaths;

impl ZettelPaths {
    fn get_paths(base_dir: String) -> HashSet<PathBuf> {
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

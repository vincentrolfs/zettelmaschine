use std::collections::HashSet;
use regex::Regex;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Eq, PartialEq)]
pub struct Zettel {
    content: String,
    path: PathBuf,
    tags: HashSet<String>
}

impl Zettel {
    pub fn new(content: String, path: PathBuf) -> Zettel {
        return Zettel {
            tags: Self::find_tags(&content),
            content,
            path,
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

impl Hash for Zettel {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.content.hash(hasher);
    }
}

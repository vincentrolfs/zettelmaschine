use std::collections::HashSet;
use regex::Regex;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Eq, PartialEq)]
pub struct Zettel {
    content: String,
    path: PathBuf,
    pub tags: HashSet<String>
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
        let iter = re.captures_iter(&content);
        let mut tags: HashSet<String> = HashSet::new();

        for captures in iter {
            match captures.get(1) {
                Some(m) => {
                    tags.insert(m.as_str().to_string());
                },
                _ => {}
            }
        }

        return tags;
    }
}

impl Hash for Zettel {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.content.hash(hasher);
    }
}

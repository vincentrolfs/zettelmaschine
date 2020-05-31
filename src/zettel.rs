use std::collections::HashSet;
use regex::Regex;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq)]
pub struct Zettel {
    content: String,
    tags: HashSet<String>
}

impl Zettel {
    pub fn new(content: String) -> Zettel {
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

impl Hash for Zettel {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.content.hash(hasher);
    }
}

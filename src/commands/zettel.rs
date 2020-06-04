use std::collections::HashSet;

use crate::zettel::Zettel;
use std::iter::FromIterator;

pub fn run(zettel: &HashSet<Zettel>, tags: Vec<&str>) {
    let tags: HashSet<&&str> = HashSet::from_iter(tags.iter());

    for one_zettel in zettel.iter() {}
}

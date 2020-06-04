use std::collections::HashSet;
use crate::zettel::Zettel;
use std::iter::FromIterator;

pub fn run(zettel: &HashSet<Zettel>) {
    let mut all_tags = HashSet::new();

    for one_zettel in zettel.iter() {
        all_tags.extend(&one_zettel.tags);
    }

    let mut all_tags_sorted = Vec::from_iter(all_tags);
    all_tags_sorted.sort();

    for tag in all_tags_sorted.iter() {
        println!("{}", tag);
    }
}

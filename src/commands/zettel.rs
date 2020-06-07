use std::collections::HashSet;

use crate::zettel::Zettel;
use std::iter::FromIterator;

pub fn run(zettel: &HashSet<Zettel>) {
    let mut zettel_names: Vec<&str> = zettel
        .iter()
        .filter_map( |zettel| zettel.path.components().last()?.as_os_str().to_str() )
        .collect();

    zettel_names.sort();

    for name in zettel_names.iter() {
        println!("{}", name);
    }
}

use std::collections::HashSet;

use crate::zettel::Zettel;
use std::iter::FromIterator;
use std::ffi::OsStr;

pub fn run(zettel: &HashSet<Zettel>, arguments: Vec<&str>) {
    let tags = HashSet::from_iter(arguments.iter().map( |a| a.to_string() ));

    let mut zettel_names: Vec<&OsStr> = zettel
        .iter()
        .filter( |zettel|
            tags.len() == 0 ||
            ! zettel.tags.is_disjoint(&tags)
        )
        .filter_map( |zettel|
            zettel.path
            .components()
            .last()
            .and_then(|c| Some(c.as_os_str()) )
        )
        .collect();

    zettel_names.sort();

    zettel_names.iter().for_each( |name|
        println!("{}", name.to_string_lossy())
    );
}

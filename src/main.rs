use std::{collections::HashSet, fs::File};
use crate::csv::build_tree_from_csv;

mod csv;
mod tree_node;
fn main() {
    let bytes= std::fs::File::open("./input.csv").unwrap();
    let tree = build_tree_from_csv(&bytes).unwrap();
    dbg!(&tree);
    let mut nodes = HashSet::new();
    let mut output = File::create("output.sql").unwrap();

    tree.collect_and_generate_cypher(&mut nodes, &mut output, None)
        .unwrap();
}

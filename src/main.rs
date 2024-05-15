use std::{collections::HashSet, fs::File};
use crate::csv::build_tree_from_csv;

mod csv;
mod tree_node;
fn main() {
    let data: &str = "\
    name,attribute,parent\n\
    root,RootAttr,\n\
    child1,Child1Attr,root\n\
    grandchild1,GrandChild1Attr,child1\n";    
    let tree = build_tree_from_csv(data.as_bytes()).unwrap();
    dbg!(&tree);
    let mut nodes = HashSet::new();
    let mut output = File::create("output.sql").unwrap();

    tree.collect_and_generate_cypher(&mut nodes, &mut output, None)
        .unwrap();
}

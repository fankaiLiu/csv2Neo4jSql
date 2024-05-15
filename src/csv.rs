use serde::Deserialize;
use std::io::Read;

use crate::tree_node::TreeNode;

#[derive(Debug, Deserialize, Clone)]
pub struct CsvNode {
    name: String,
    attribute: String,
    parent: Option<String>,
}

pub fn build_tree_from_csv<R: Read>(reader: R) -> Option<TreeNode> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut tree_nodes=vec![];

    // Read CSV file and populate nodes and parentage
    for result in rdr.deserialize() {
        let record: CsvNode = result.expect("CSV parsing error");
        let node = TreeNode {
            name: record.name.clone(),
            attribute: record.attribute,
            parent: match record.parent {
                Some(p) => Some(p.to_string()),
                None =>None,
            },
            children: Vec::new(),
        };
        tree_nodes.push(node);
    }
    let root=build_tree(tree_nodes);
    root
 }


 pub fn build_tree(nodes: Vec<TreeNode>) -> Option<TreeNode> {
    use std::collections::HashMap;

    // Create a HashMap to group all nodes by their parent
    let mut map: HashMap<Option<String>, Vec<TreeNode>> = HashMap::new();
    for node in nodes {
        map.entry(node.parent.clone())
            .or_default()
            .push(TreeNode { children: vec![], ..node });
    }

    // Recursively build the tree
    fn build_subtree(map: &HashMap<Option<String>, Vec<TreeNode>>, parent_name: Option<String>) -> Vec<TreeNode> {
        map.get(&parent_name).map_or_else(Vec::new, |nodes| {
            nodes.iter().map(|node| {
                let mut new_node = node.clone(); // Clone basic info
                new_node.children = build_subtree(map, Some(node.name.clone())); // Recursively build children
                new_node
            }).collect() // Collect all child nodes into a Vec<TreeNode>
        })
    }
    // Build the root node where the parent is None
    build_subtree(&map, None).into_iter().next()
}

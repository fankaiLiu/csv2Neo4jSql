use serde::Deserialize;
use std::collections::HashMap;
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
    let mut node_map = HashMap::new();
    let mut children_map: HashMap<String, Vec<TreeNode>> = HashMap::new();

    // Read CSV file and build a map of CsvNodes
    for result in rdr.deserialize() {
        let record: CsvNode = result.expect("CSV parsing error");
        let node = TreeNode {
            name: record.name.clone(),
            attribute: record.attribute,
            children: vec![],
        };
        // Initialize the children vector for each node
        children_map.entry(record.name.clone()).or_insert(vec![]);
        if let Some(parent) = record.parent {
            children_map.entry(parent).or_insert(vec![]).push(node);
        } else {
            // This node is the root node
            node_map.insert(record.name.clone(), node);
        }
    }

    // Now build the TreeNode structure
    for (name, mut node) in node_map.iter_mut() {
        if let Some(children) = children_map.remove(name) {
            node.children = children;
        }
    }

    // Assuming there is only one root node
    node_map.into_iter().next().map(|(_, node)| node)
}

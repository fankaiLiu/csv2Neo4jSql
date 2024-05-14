use std::collections::HashSet;

struct TreeNode {
    name: String,
    attribute: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(name: &str, attribute: &str) -> Self {
        TreeNode {
            name: name.to_string(),
            attribute: attribute.to_string(),
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    // 遍历树，收集所有节点，并生成节点和关系的创建语句
    fn collect_and_generate_cypher(&self, nodes: &mut HashSet<String>, relations: &mut Vec<String>, parent_name: Option<&String>) {
        // 插入当前节点的名字，HashSet 自动处理重复项
        if nodes.insert(self.name.clone()) {
            // 如果节点是新添加的，生成节点的创建语句
            let node_statement = format!("CREATE ({}:Node {{name: '{}'}});", self.name.replace(" ", "_"), self.name);
            relations.push(node_statement);
        }

        // 如果存在父节点，生成关系创建语句
        if let Some(parent) = parent_name {
            let relation = format!("MATCH (a:Node {{name: '{}'}}), (b:Node {{name: '{}'}}) CREATE (a)-[:{}]->(b);", parent, self.name,self.attribute);
            relations.push(relation);
        }

        // 递归收集所有子节点并为每个子节点生成关系
        for child in &self.children {
            child.collect_and_generate_cypher(nodes, relations, Some(&self.name));
        }
    }

    // 生成所有节点和关系的 Cypher 创建语句
    fn generate_all_cypher_statements(&self) -> Vec<String> {
        let mut nodes = HashSet::new();
        let mut statements = Vec::new();
        self.collect_and_generate_cypher(&mut nodes, &mut statements, None);
        statements
    }
}

fn main() {
    let mut root = TreeNode::new("Root", "attr1");
    let mut child1 = TreeNode::new("Child1", "attr2");
    let child2 = TreeNode::new("Child2", "attr3");

    child1.add_child(TreeNode::new("SubChild1", "attr4"));
    root.add_child(child1);
    root.add_child(child2);

    let statements = root.generate_all_cypher_statements();
    for statement in statements {
        println!("{}", statement);
    }
}
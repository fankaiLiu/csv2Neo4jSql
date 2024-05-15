use std::{
    collections::HashSet,
    fs::File,
    io::{self, Write},
};

#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub attribute: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(name: &str, attribute: &str) -> Self {
        TreeNode {
            name: name.to_string(),
            attribute: attribute.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    // 遍历树，收集所有节点，并生成节点和关系的创建语句
    // 修改函数签名来接收一个可变引用到文件
    pub fn collect_and_generate_cypher(
        &self,
        nodes: &mut HashSet<String>,
        output: &mut File,
        parent_name: Option<&String>,
    ) -> io::Result<()> {
        // 插入当前节点的名字，HashSet 自动处理重复项
        if nodes.insert(self.name.clone()) {
            // 如果节点是新添加的，生成节点的创建语句
            let node_statement = format!(
                "CREATE ({}:Node {{name: '{}'}});\n",
                self.name.replace(" ", "_"),
                self.name
            );
            output.write_all(node_statement.as_bytes())?;
        }

        // 如果存在父节点，生成关系创建语句
        if let Some(parent) = parent_name {
            let relation = format!(
                "MATCH (a:Node {{name: '{}'}}), (b:Node {{name: '{}'}}) CREATE (a)-[:{}]->(b);\n",
                parent, self.name, self.attribute
            );
            output.write_all(relation.as_bytes())?;
        }

        // 递归收集所有子节点并为每个子节点生成关系
        for child in &self.children {
            child.collect_and_generate_cypher(nodes, output, Some(&self.name))?;
        }
        Ok(())
    }
}
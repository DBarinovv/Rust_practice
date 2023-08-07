#[derive(PartialEq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn serialize(root: Option<Box<TreeNode>>) -> String {
    let mut res = String::new();
    serialize_rec(&root, &mut res);
    res.pop(); // Remove the trailing comma
    res
}

fn serialize_rec(root: &Option<Box<TreeNode>>, res: &mut String) {
    if let Some(node) = root {
        res.push_str(&node.val.to_string());
        res.push(',');
        serialize_rec(&node.left, res);
        serialize_rec(&node.right, res);
    } else {
        res.push_str("#,");
    }
}

fn deserialize_rec(nodes: &[&str], idx: &mut usize) -> Option<Box<TreeNode>> {
    if *idx < nodes.len() {
        let node = nodes[*idx];
        *idx += 1;
        if node == "#" {
            return None;
        }

        let val = node.parse::<i32>().unwrap_or(0);
        Some(Box::new(TreeNode {
            val,
            left: deserialize_rec(nodes, idx),
            right: deserialize_rec(nodes, idx),
        }))
    } else {
        None
    }
}

pub fn deserialize(data: String) -> Option<Box<TreeNode>> {
    let nodes = data.split(',').collect::<Vec<_>>();
    let mut idx = 0;
    deserialize_rec(&nodes, &mut idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: Option<Box<TreeNode>> = None;
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized.clone());
        let serialized2 = serialize(deserialized);
        assert_eq!(serialized2, serialized);
    }

    #[test]
    fn test_single_node_tree() {
        let tree = Some(Box::new(TreeNode::new(5)));
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_full_binary_tree() {
        let mut tree = Some(Box::new(TreeNode::new(1)));
        tree.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
        tree.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
        tree.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
        tree.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized.clone());
        let serialized2 = serialize(deserialized.clone());
        // println!("{} -- {}", serialized, serialized2);
        assert_eq!(serialized, serialized2);
    }

    #[test]
    fn test_unbalanced_tree() {
        let mut tree = Some(Box::new(TreeNode::new(1)));
        tree.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));
        tree.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_tree_with_negative_values() {
        let mut tree = Some(Box::new(TreeNode::new(-1)));
        tree.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
        tree.as_mut().unwrap().right = Some(Box::new(TreeNode::new(-3)));
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }
}

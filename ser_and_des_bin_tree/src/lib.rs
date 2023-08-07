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
    unimplemented!();
    // Your code here
}

pub fn deserialize(data: String) -> Option<Box<TreeNode>> {
    unimplemented!();
    // Your code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Some(Box::new(TreeNode::new(1)));
        let left_child = Some(Box::new(TreeNode::new(2)));
        let right_child = Some(Box::new(TreeNode::new(3)));
        root.as_mut().unwrap().left = left_child;
        root.as_mut().unwrap().right = right_child;
        
        let serialized = serialize(root.clone());
        let deserialized = deserialize(serialized);
        
        // deserialized should be equal to root
    }
}

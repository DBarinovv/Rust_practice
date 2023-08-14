### Serialize and Deserialize a Binary Tree

You'll implement the serialization and deserialization of a binary tree. Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer or transmitted across a network connection. Deserialization is the opposite process.

#### Definition for a binary tree node:
```rust
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
```

#### Implement the following two functions:

1. **serialize**: Convert the binary tree to a string. You can decide the format of the string as long as you can successfully deserialize it later.

```rust
pub fn serialize(root: Option<Box<TreeNode>>) -> String {
    // Your code here
}
```

2. **deserialize**: Convert the string back to the binary tree.

```rust
pub fn deserialize(data: String) -> Option<Box<TreeNode>> {
    // Your code here
}
```

#### Constraints:

- The number of nodes in the tree is in the range `[0, 1000]`.
- `-1000 <= Node.val <= 1000`.

#### Example:

```rust
let root = Some(Box::new(TreeNode::new(1)));
let left_child = Some(Box::new(TreeNode::new(2)));
let right_child = Some(Box::new(TreeNode::new(3)));
root.as_mut().unwrap().left = left_child;
root.as_mut().unwrap().right = right_child;

let serialized = serialize(root.clone());
let deserialized = deserialize(serialized);

// deserialized should be equal to root
```

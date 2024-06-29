use std::io;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.borrow().left.clone());
            let right_depth = max_depth(node.borrow().right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Read input to build the binary tree
    println!("Enter the values of the tree nodes in level order, separated by spaces. Use 'null' for empty nodes:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let values: Vec<Option<i32>> = input.trim().split_whitespace().map(|s| {
        if s == "null" {
            None
        } else {
            Some(s.parse().expect("Please enter valid integers"))
        }
    }).collect();

    let root = build_tree(&values);

    let depth = max_depth(root);
    println!("The maximum depth of the tree is {}", depth);
}

fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() {
        return None;
    }

    let root = TreeNode::new(values[0].unwrap());
    let mut queue = vec![root.clone()];

    let mut i = 1;
    while i < values.len() {
        let current = queue.remove(0);

        if let Some(val) = values[i] {
            let left = TreeNode::new(val);
            current.borrow_mut().left = Some(left.clone());
            queue.push(left);
        }
        i += 1;

        if i < values.len() {
            if let Some(val) = values[i] {
                let right = TreeNode::new(val);
                current.borrow_mut().right = Some(right.clone());
                queue.push(right);
            }
            i += 1;
        }
    }

    Some(root)
}

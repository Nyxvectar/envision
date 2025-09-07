/**
 * Author:  Nyxvectar Yan
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left,
        right,
    })));

    let result = Solution::check_tree(root);
    println!("{:?}", result);
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root_node) = root {
            let root_value = root_node.borrow().val;
            let left_value = if let Some(left_node) = &root_node.borrow().left {
                left_node.borrow().val
            } else {
                0
            };
            let right_value = if let Some(right_node) = &root_node.borrow().right {
                right_node.borrow().val
            } else {
                0
            };
            return root_value == left_value + right_value;
        }
        false
    }
}

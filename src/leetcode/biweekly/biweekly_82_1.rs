use leetcode_prelude::{btree, TreeNode};
use crate::leetcode::Solution;

#[test]
fn biweekly_82_1_test() {
    assert_eq!(Solution::evaluate_tree(btree![2,1,3,null,null,0,1]), true);
    assert_eq!(Solution::evaluate_tree(btree![]), false);
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::get_sum(&root)
    }

    fn get_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = node {
            let v = n.borrow();
            if v.left.is_none() && v.right.is_none() {
                v.val == 1
            } else {
                let left = Solution::get_sum(&v.left);
                let right = Solution::get_sum(&v.right);
                if v.val == 2 {
                    left || right
                } else {
                    left && right
                }
            }
        } else {
            false
        }
    }
}
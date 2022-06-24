//给定一棵二叉树的根节点 root ，请找出该二叉树中每一层的最大值。 
//
// 
//
// 示例1： 
//
// 
//
// 
//输入: root = [1,3,2,5,3,null,9]
//输出: [1,3,9]
// 
//
// 示例2： 
//
// 
//输入: root = [1,2,3]
//输出: [1,3]
// 
//
// 
//
// 提示： 
//
// 
// 二叉树的节点个数的范围是 [0,10⁴] 
// -2³¹ <= Node.val <= 2³¹ - 1 
// 
//
// 
// Related Topics 树 深度优先搜索 广度优先搜索 二叉树 👍 249 👎 0

use leetcode_prelude::{btree, TreeNode};
use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut nodes = VecDeque::new();
        let mut max_nodes = vec![];
        let mut current_level = 0;
        let mut current_level_max = i32::MIN;
        nodes.push_back((Rc::clone(&root.unwrap()), 0));
        while !nodes.is_empty() {
            if let Some((n, level)) = nodes.pop_front() {
                let node = n.as_ref().borrow();
                if level > current_level {
                    max_nodes.push(current_level_max);
                    current_level_max = node.val;
                    current_level = level;
                } else {
                    current_level_max = current_level_max.max(node.val);
                }
                if let Some(left) = &node.left {
                    nodes.push_back((Rc::clone(left), level + 1));
                }
                if let Some(right) = &node.right {
                    nodes.push_back((Rc::clone(right), level + 1));
                }
            }
        }
        max_nodes.push(current_level_max);
        max_nodes
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_515_test() {
    assert_eq!(Solution::largest_values(btree![1,2,3]), vec![1,3]);
    assert_eq!(Solution::largest_values(btree![1,3,2,5,3,null,9]), vec![1,3,9]);
}
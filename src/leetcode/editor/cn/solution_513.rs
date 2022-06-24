//给定一个二叉树的 根节点 root，请找出该二叉树的 最底层 最左边 节点的值。 
//
// 假设二叉树中至少有一个节点。 
//
// 
//
// 示例 1: 
//
// 
//
// 
//输入: root = [2,1,3]
//输出: 1
// 
//
// 示例 2: 
//
// 
//
// 
//输入: [1,2,3,4,null,5,6,null,null,7]
//输出: 7
// 
//
// 
//
// 提示: 
//
// 
// 二叉树的节点个数的范围是 [1,10⁴] 
// -2³¹ <= Node.val <= 2³¹ - 1 
// 
// Related Topics 树 深度优先搜索 广度优先搜索 二叉树 👍 338 👎 0

use std::borrow::Borrow;
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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut node_queue = VecDeque::new();
        node_queue.push_back((Rc::clone(&root.unwrap()), 0));
        let mut current_level = 0;
        let mut result = node_queue.front().unwrap().0.as_ref().borrow().val;
        while !node_queue.is_empty() {
            if let Some((v, level)) = node_queue.pop_front() {
                let node = v.as_ref().borrow();
                if level > current_level {
                    current_level = level;
                    result = node.val;
                }
                if let Some(left) = &node.left {
                    node_queue.push_back((Rc::clone(left), level + 1));
                }
                if let Some(right) = &node.right {
                    node_queue.push_back((Rc::clone(right), level + 1));
                }
            }
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_513_test() {
    // let tree: Option<Rc<RefCell<TreeNode>>> = btree![2,1,3];
    // assert_eq!(Solution::find_bottom_left_value(tree), 1);
    // assert_eq!(Solution::find_bottom_left_value(btree![1,2,3,4,null,5,6,null,null,7]), 7);
}
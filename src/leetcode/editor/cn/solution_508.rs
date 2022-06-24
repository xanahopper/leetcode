//给你一个二叉树的根结点 root ，请返回出现次数最多的子树元素和。如果有多个元素出现的次数相同，返回所有出现次数最多的子树元素和（不限顺序）。 
//
// 一个结点的 「子树元素和」 定义为以该结点为根的二叉树上所有结点的元素之和（包括结点本身）。 
//
// 
//
// 示例 1： 
//
// 
//
// 
//输入: root = [5,2,-3]
//输出: [2,-3,4]
// 
//
// 示例 2： 
//
// 
//
// 
//输入: root = [5,2,-5]
//输出: [2]
// 
//
// 
//
// 提示: 
//
// 
// 节点数在 [1, 10⁴] 范围内 
// -10⁵ <= Node.val <= 10⁵ 
// 
// Related Topics 树 深度优先搜索 哈希表 二叉树 👍 163 👎 0

use crate::leetcode::Solution;
use leetcode_prelude::{btree, TreeNode};

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
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut sum = HashMap::new();
        Solution::travel_tree_sum(&mut sum, &root);
        if let Some((_, &count)) = sum.iter().max_by_key(|(&k, &v)| v) {
            sum.iter().filter(|(_, &v)| v == count)
                .map(|(&k, &v)| k)
                .collect()
        } else {
            vec![]
        }
    }

    fn travel_tree_sum(sum: &mut HashMap<i32, i32>, root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(mid) = root {
            let root = mid.borrow();
            let left_sum = Solution::travel_tree_sum(sum, &root.left);
            let right_sum = Solution::travel_tree_sum(sum, &root.right);
            let root_sum: i32 = left_sum.unwrap_or(0) + right_sum.unwrap_or(0) + root.val;
            let sum_entry = sum.entry(root_sum).or_insert(0);
            *sum_entry += 1;
            Some(root_sum)
        } else {
            None
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

use std::iter::FromIterator;
#[test]
fn solution_508_test() {
    assert_eq!(HashSet::from_iter(Solution::find_frequent_tree_sum(btree![5, 2, -3])), HashSet::from([2, -3, 4]));
    assert_eq!(Solution::find_frequent_tree_sum(btree![5, 2, -5]), vec![2]);
}

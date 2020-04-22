//Add Two Numbers
//You are given two non-empty linked lists representing two non-negative integer
//s. The digits are stored in reverse order and each of their nodes contain a sing
//le digit. Add the two numbers and return it as a linked list. 
//
// You may assume the two numbers do not contain any leading zero, except the nu
//mber 0 itself. 
//
// Example: 
//
// 
//Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
//Output: 7 -> 0 -> 8
//Explanation: 342 + 465 = 807.
// 
// Related Topics Linked List Math


use crate::leetcode::Solution;
use leetcode_prelude::ListNode;

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, remain: i32) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val + remain;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add(n1.next, n2.next, sum / 10)
                }))
            }
            (Some(n1), None) => {
                let sum = n1.val + remain;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add(n1.next, None, sum / 10),
                }))
            }
            (None, Some(n2)) => {
                let sum = n2.val + remain;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add(None, n2.next, sum / 10),
                }))
            }
            (None, None) => {
                if remain != 0 {
                    Some(Box::new(ListNode::new(remain)))
                } else {
                    None
                }
            }
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::add(l1, l2, 0)
    }
}
//leetcode submit region end(Prohibit modification and deletion)

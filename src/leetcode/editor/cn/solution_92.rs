//反转链表 II
//给你单链表的头节点 head 和两个整数 left 和 right ，其中 left <= right 。请你反转从位置 left 到位置 right 的链
//表节点，返回 反转后的链表 。
// 
//
// 示例 1： 
//
// 
//输入：head = [1,2,3,4,5], left = 2, right = 4
//输出：[1,4,3,2,5]
// 
//
// 示例 2： 
//
// 
//输入：head = [5], left = 1, right = 1
//输出：[5]
// 
//
// 
//
// 提示： 
//
// 
// 链表中节点数目为 n 
// 1 <= n <= 500 
// -500 <= Node.val <= 500 
// 1 <= left <= right <= n 
// 
//
// 
//
// 进阶： 你可以使用一趟扫描完成反转吗？ 
// Related Topics 链表 
// 👍 812 👎 0


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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut root = ListNode { val: 0, next: head };
        {
            let mut current = &mut root;
            let mut i = 1;
            while current.next.is_some() {
                if i >= left {
                    let mut nodes = vec![];
                    let mut mid = ListNode { val: 0, next: current.next.take() };
                    let mut mid_current = &mut mid;
                    while i <= right {
                        if let Some(node) = mid_current.next.take() {
                            nodes.push(node);
                            mid_current = nodes.last_mut().unwrap();
                        } else {
                            break;
                        }
                        i += 1;
                    }

                    let rear = mid_current.next.take();
                    mid = ListNode::new(0);
                    mid_current = &mut mid;
                    for node in nodes.into_iter().rev() {
                        mid_current.next = Some(node);
                        mid_current = mid_current.next.as_mut().unwrap();
                    }

                    mid_current.next = rear;
                    current.next = mid.next;
                    break;
                } else {
                    i += 1;
                    current = current.next.as_mut().unwrap();
                }
            }
        }
        root.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)

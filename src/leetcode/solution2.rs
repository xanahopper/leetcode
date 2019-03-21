pub mod solution2 {
    extern crate leetcode_prelude;

    use leetcode_prelude::*;
    use crate::leetcode::Solution;

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
}
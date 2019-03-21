mod leetcode {
    pub struct Solution {

    }

    pub mod solution1 {
        use crate::leetcode::Solution;

        impl Solution {
            pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
                use std::collections::HashMap;
                let mut num_hash = HashMap::new();
                for (i, num) in nums.iter().enumerate() {
                    let other = target - num;
                    if let Some(&other_i) = num_hash.get(&other) {
                        return vec![other_i, i as i32];
                    }
                    num_hash.insert(num, i as i32);
                }
                return vec![0, 1];
            }
        }
    }

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn n1_test() {
        use crate::leetcode::Solution;
        assert_eq!(Solution::two_sum(vec![1, 2, 3], 5), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn n2_test() {
        extern crate leetcode_prelude;
        use crate::leetcode::Solution;
        use leetcode_prelude::*;

        let l1 = linkedlist![2, 4, 3];
        let l2 = linkedlist![5, 6, 4];

        assert_eq!(Solution::add_two_numbers(l1, l2), linkedlist![7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(linkedlist![5], linkedlist![5]), linkedlist![0, 1]);
    }
}

mod leetcode {
    pub mod solution1 {
        pub fn n1_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

    pub mod solution2 {
        extern crate leetcode_prelude;

        use leetcode_prelude::*;

        pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            fn add(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, remain: i32) -> Option<Box<ListNode>> {
                if *l1 != None && *l2 != None {
                    let node1 = l1.as_ref().unwrap();
                    let node2 = l2.as_ref().unwrap();
                    let sum = node1.val + node2.val + remain;
                    Some(Box::new(ListNode {
                        val: sum % 10,
                        next: add(&node1.next, &node2.next, sum / 10),
                    }))
                } else if let Some(ref node) = l1 {
                    Some(Box::new(ListNode {
                        val: (node.val + remain) % 10,
                        next: add(&node.next, l2, (node.val + remain) / 10),
                    }))
                } else if let Some(ref node) = l2 {
                    Some(Box::new(ListNode {
                        val: (node.val + remain) % 10,
                        next: add(l1, &node.next, (node.val + remain) / 10),
                    }))
                } else if remain != 0 {
                    Some(Box::new(ListNode {
                        val: remain,
                        next: None
                    }))
                } else { None }
            }
            add(&l1, &l2, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn n1_test() {
        use super::leetcode::solution1::n1_two_sum;
        assert_eq!(n1_two_sum(vec![1, 2, 3], 5), vec![1, 2]);
        assert_eq!(n1_two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn n2_test() {
        extern crate leetcode_prelude;
        use super::leetcode::solution2::add_two_numbers;
        use leetcode_prelude::*;

        let l1 = linkedlist![2, 4, 3];
        let l2 = linkedlist![5, 6, 4];

        assert_eq!(add_two_numbers(l1, l2), linkedlist![7, 0, 8]);
        assert_eq!(add_two_numbers(linkedlist![5], linkedlist![5]), linkedlist![0, 1]);
    }
}

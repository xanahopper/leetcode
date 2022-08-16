use std::collections::HashSet;
use crate::leetcode::Solution;

#[test]
fn weekly_304_1_test() {
    assert_eq!(Solution::minimum_operations(vec![1,5,0,3,5]), 3);
    assert_eq!(Solution::minimum_operations(vec![0]), 0);
}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let counter = nums.iter().fold(HashSet::new(), |mut r, &x| {
            if x != 0 {
                r.insert(x);
            }
            r
        });
        counter.len() as i32
    }
}
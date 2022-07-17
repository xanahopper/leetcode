use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn weekly_302_1_test() {
    assert_eq!(Solution::number_of_pairs(vec![1,3,2,1,3,2,2]), vec![3,1]);
    assert_eq!(Solution::number_of_pairs(vec![1,1]), vec![1,0]);
    assert_eq!(Solution::number_of_pairs(vec![0]), vec![0,1]);
}

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let count = nums.iter().fold(HashMap::new(), |mut res, &x| {
            let entry = res.entry(x).or_insert(0);
            *entry += 1;
            res
        });
        let (pair, remain) = count.values().fold((0, 0), |(pair, remain), &x| (pair + x / 2, remain + x % 2));
        vec![pair, remain]
    }
}
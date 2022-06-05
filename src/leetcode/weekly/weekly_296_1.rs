use std::cmp::{max, min};
use crate::leetcode::Solution;

#[test]
fn weekly_296_1_test() {
    assert_eq!(Solution::min_max_game(vec![1,3,5,2,4,8,2,2]), 1);
    assert_eq!(Solution::min_max_game(vec![2,3,5,1,4,8,2,2]), 2);
    assert_eq!(Solution::min_max_game(vec![3]), 3);
}

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut len = nums.len();
        while len > 1 {
            let mut new_nums = vec![0; len / 2];
            for i in 0..(len / 2) {
                if i % 2 == 0 {
                    new_nums[i] = min(nums[i * 2], nums[i * 2 + 1]);
                } else {
                    new_nums[i] = max(nums[i * 2], nums[i * 2 + 1]);
                }
            }
            nums = new_nums;
            len = nums.len();
        }
        *nums.first().unwrap()
    }
}
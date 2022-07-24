use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::leetcode::Solution;

#[test]
fn weekly_297_4_test() {
    assert_eq!(Solution::bit_count(15), 4);
    assert_eq!(Solution::count_excellent_pairs(vec![1,2,3,1], 3), 5);
    assert_eq!(Solution::count_excellent_pairs(vec![5,1,1], 10), 0);
}

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let nums: HashSet<i32> = HashSet::from_iter(nums);
        let mut count = nums.iter().map(|&x| Solution::bit_count(x as u32))
            .fold(vec![0; 150], |mut r, c| {
                r[c as usize] += 1;
                r
            });
        let mut res = 0;
        for i in 1..=64usize {
            for j in 1..=64usize {
                if i + j >= k as usize {
                    res += (count[i] as i64) * (count[j] as i64);
                }
            }
        }
        res
    }

    fn bit_count(n: u32) -> u32 {
        let t = n - ((n >> 1) & 0o33333333333) - ((n >> 2) & 0o11111111111);
        ((t + (t >> 3)) & 0o30707070707) % 63
    }
}
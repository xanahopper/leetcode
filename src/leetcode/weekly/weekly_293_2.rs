use std::cmp::max;
use crate::leetcode::Solution;

#[test]
fn weekly_293_2_test() {
    assert_eq!(Solution::max_consecutive(2, 9, vec![4, 6]), 3);
    assert_eq!(Solution::max_consecutive(6, 8, vec![7, 6, 8]), 0);
}

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut sp = vec![bottom - 1];
        sp.extend(special);
        sp.push(top + 1);
        sp.sort();
        let mut maximum = 0;
        for range in sp.windows(2) {
            let last: i32 = range[0] + 1;
            let next: i32 = range[1] - 1;
            if last <= next {
                maximum = max(maximum, next - last + 1);
            }
        }
        maximum
    }
}
use crate::leetcode::Solution;

#[test]
fn biweekly_81_3_test() {
    assert_eq!(Solution::maximum_xor(vec![3,2,4,6]), 7);
    assert_eq!(Solution::maximum_xor(vec![1,2,3,9,2]), 11);
}

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |res, &x| res | x)
    }
}
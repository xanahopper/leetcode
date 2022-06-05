use crate::leetcode::Solution;

#[test]
fn weekly_296_2_test() {
    assert_eq!(Solution::partition_array(vec![3,6,1,2,5], 2), 2);
    assert_eq!(Solution::partition_array(vec![1,2,3], 1), 2);
    assert_eq!(Solution::partition_array(vec![2,2,4,5], 0), 3);
}

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a));
        nums.iter().fold((1i32, *nums.first().unwrap()), |(count, last_start), &n| {
            if last_start - n > k {
                (count + 1, n)
            } else {
                (count, last_start)
            }
        }).0
    }
}
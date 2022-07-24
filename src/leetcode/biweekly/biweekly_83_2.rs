use crate::leetcode::Solution;

#[test]
fn biweekly_83_2_test() {
    assert_eq!(Solution::zero_filled_subarray(vec![1,3,0,0,2,0,0,4]), 6);
    assert_eq!(Solution::zero_filled_subarray(vec![0,0,0,2,0,0]), 9);
    assert_eq!(Solution::zero_filled_subarray(vec![2,10,2019]), 0);
}

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        nums.split(|&x| x != 0)
            .filter_map(|x| if x.len() == 0 { None } else { Some(x.len() as i64) })
            .map(|x| x * (x + 1) / 2)
            .sum()
    }
}
use crate::leetcode::Solution;

#[test]
fn weekly_305_3_test() {
}

const SINGLE: i32 = 0;
const EQUAL: i32 = 1;
const ASC: i32 = 2;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        // let mut dp = vec![0; 3];
        // let mut last = nums[0];
        // for (idx, num) in nums[1..nums.len()].iter().enumerate() {
        //     let s = dp[idx % 3];
        //     if last == num {
        //         if s != 0 && s != 1 {
        //             return false;
        //         }
        //     } else if num - last == 1 {
        //
        //     }
        // }
        // nums.zip().enumerate().fold(vec![0; 3], |mut dp, (idx, num)| {
        //     let n1 = num.0;
        //     let n2 = num.1;
        //     if n1 == n2 {
        //     }
        //     dp
        // });
        true
    }
}
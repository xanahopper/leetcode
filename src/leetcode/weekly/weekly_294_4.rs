use crate::leetcode::Solution;

const MOD: i32 = 1E9 as i32 + 7;

impl Solution {
    pub fn total_strength(strength: Vec<i32>) -> i32 {
        let sum = strength.iter().enumerate()
            .fold(vec![0; strength.len()], |mut sum, (index, &s)| {
            sum[index] = (sum[index - 1] + s) % MOD;
            sum
        });
        0
    }
}
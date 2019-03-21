pub mod solution1 {
    use crate::leetcode::Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            use std::collections::HashMap;
            let mut num_hash = HashMap::new();
            for (i, num) in nums.iter().enumerate() {
                let other = target - num;
                if let Some(&other_i) = num_hash.get(&other) {
                    return vec![other_i, i as i32];
                }
                num_hash.insert(num, i as i32);
            }
            return vec![0, 1];
        }
    }
}
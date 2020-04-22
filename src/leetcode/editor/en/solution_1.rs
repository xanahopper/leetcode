//Two Sum
//Given an array of integers, return indices of the two numbers such that they a
//dd up to a specific target. 
//
// You may assume that each input would have exactly one solution, and you may n
//ot use the same element twice. 
//
// Example: 
//
// 
//Given nums = [2, 7, 11, 15], target = 9,
//
//Because nums[0] + nums[1] = 2 + 7 = 9,
//return [0, 1].
// 
// Related Topics Array Hash Table


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut num_hash = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let other = target - num;
            if let Some(&other_i) = num_hash.get(&other) {
                return vec![other_i, i as i32];
            }
            num_hash.insert(num, i as i32);
        }
        return vec![0, 1];
    }
}
//leetcode submit region end(Prohibit modification and deletion)

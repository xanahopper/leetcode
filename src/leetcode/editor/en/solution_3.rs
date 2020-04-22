//Longest Substring Without Repeating Characters
//Given a string, find the length of the longest substring without repeating cha
//racters. 
//
// 
// Example 1: 
//
// 
//Input: "abcabcbb"
//Output: 3 
//Explanation: The answer is "abc", with the length of 3. 
// 
//
// 
// Example 2: 
//
// 
//Input: "bbbbb"
//Output: 1
//Explanation: The answer is "b", with the length of 1.
// 
//
// 
// Example 3: 
//
// 
//Input: "pwwkew"
//Output: 3
//Explanation: The answer is "wke", with the length of 3. 
//             Note that the answer must be a substring, "pwke" is a subsequence
// and not a substring.
// 
// 
// 
// 
// Related Topics Hash Table Two Pointers String Sliding Window


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;

        let mut max_length = 0;
        let mut current_length = 0;
        let mut start_index = 0usize;
        for (end_index, c) in s.chars().enumerate() {
            match &s[start_index..end_index].find(c) {
                None => current_length += 1,
                Some(index) => {
                    max_length = max(current_length, max_length);
                    current_length -= index;
                    start_index = start_index + index + 1;
                }
            }
        }
        max(current_length, max_length) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

//Longest Common Prefix
//Write a function to find the longest common prefix string amongst an array of 
//strings. 
//
// If there is no common prefix, return an empty string "". 
//
// Example 1: 
//
// 
//Input: ["flower","flow","flight"]
//Output: "fl"
// 
//
// Example 2: 
//
// 
//Input: ["dog","racecar","car"]
//Output: ""
//Explanation: There is no common prefix among the input strings.
// 
//
// Note: 
//
// All given inputs are in lowercase letters a-z. 
// Related Topics String


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let (first, rest) = match strs.split_first() {
            Some(x) => x,
            None => return String::new()
        };
        let mut iters = rest.iter().map(|s| s.chars()).collect::<Vec<_>>();
        first
            .chars()
            .take_while(|&c|
                iters.iter_mut().all(|i| i.next() == Some(c))
            )
            .collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

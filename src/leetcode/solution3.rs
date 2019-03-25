use crate::leetcode::Solution;

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
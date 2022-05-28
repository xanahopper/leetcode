use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn biweekly_79_1_test() {
    assert_eq!(Solution::digit_count("1210".to_string()), true);
    assert_eq!(Solution::digit_count("030".to_string()), false);
}

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let nums: Vec<u8> = num.into_bytes().into_iter().map(|x| x - ('0' as u8)).collect();
        let mut counts: HashMap<u8, usize> = nums.iter().fold(HashMap::new(), |mut counts, x| {
            let item = counts.entry(*x).or_insert(0);
            *item += 1;
            counts
        });
        nums.iter().enumerate()
            .fold(true, |res, (index, &x)|
                res && *(counts.entry(index as u8).or_insert(0)) == x as usize)
    }
}
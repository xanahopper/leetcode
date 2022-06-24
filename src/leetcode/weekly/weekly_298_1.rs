use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn weekly_298_1_test() {
    assert_eq!(Solution::greatest_letter("lEeTcOdE".to_string()), "E".to_string());
    assert_eq!(Solution::greatest_letter("arRAzFif".to_string()), "R".to_string());
    assert_eq!(Solution::greatest_letter("AbCdEfGhIjK".to_string()), "".to_string());
}

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut upper = HashMap::new();
        let mut lower = HashMap::new();
        for c  in s.chars() {
            if c.is_lowercase() {
                lower.insert(c, true);
            } else if c.is_uppercase() {
                upper.insert(c, true);
            }
        }
        let mut res = String::new();
        for c in 'A'..='Z' {
            let low_c = c.to_lowercase().collect::<Vec<_>>()[0];
            if upper.contains_key(&c) && lower.contains_key(&low_c) {
                res = c.to_string();
            }
        }
        res
    }
}
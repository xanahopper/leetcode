use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn weekly_300_1_test() {
    assert_eq!(Solution::decode_message("the quick brown fox jumps over the lazy dog".to_string(), "vkbs bs t suepuv".to_string()), "this is a secret".to_string());
}

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut code = HashMap::new();
        let mut index = 0;
        for &c in key.as_bytes().iter() {
            if !code.contains_key(&c) && c != ' ' as u8 {
                code.entry(c).or_insert(index);
                index += 1;
            }
            if code.len() >= 26 {
                break;
            }
        }
        String::from_utf8(message.as_bytes().iter().map(|&x| if x == ' ' as u8 {
            x
        } else {
            'a' as u8 + code[&x]
        }).collect()).unwrap()
    }
}
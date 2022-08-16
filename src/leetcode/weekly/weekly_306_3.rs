use std::collections::VecDeque;
use crate::leetcode::Solution;

#[test]
fn weekly_306_3_test() {
    assert_eq!(Solution::smallest_number("IIIDIDDD".into()), "123549876");
    assert_eq!(Solution::smallest_number("DDD".into()), "4321");
}

#[derive(Copy, Clone, PartialEq)]
enum DiPattern {
    Increase,
    Decrease
}

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut pattern: Vec<_> = pattern.as_bytes().iter()
            .map(|&x| if x == 'I' as u8 { DiPattern::Increase } else { DiPattern::Decrease }).collect();
        let mut stack = VecDeque::new();
        let mut result = vec![];
        let mut last = pattern[0];
        for (idx, &p) in pattern.iter().enumerate() {
            match p {
                DiPattern::Increase => {
                    result.push((idx + 1) as i32);
                    if last == DiPattern::Decrease {
                        while let Some(x) = stack.pop_back() {
                            result.push(x);
                        }
                    }
                }
                DiPattern::Decrease => {
                    stack.push_back((idx + 1) as i32)
                }
            }
            last = p;
        }
        if last == DiPattern::Decrease {
            stack.push_back((pattern.len() + 1) as i32)
        } else {
            result.push((pattern.len() + 1) as i32)
        }
        while let Some(x) = stack.pop_back() {
            result.push(x);
        }
        String::from_utf8(result.into_iter().map(|x| '0' as u8 + x as u8).collect::<Vec<_>>()).unwrap()
    }
}
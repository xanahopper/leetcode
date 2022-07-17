use std::collections::VecDeque;
use crate::leetcode::Solution;

#[test]
fn weekly_300_4_test() {
    assert_eq!(Solution::can_change("_L__R__R_".to_string(), "L______RR".to_string()), true);
    assert_eq!(Solution::can_change("R_L_".to_string(), "__LR".to_string()), false);
    assert_eq!(Solution::can_change("_R".to_string(), "R_".to_string()), false);
    assert_eq!(Solution::can_change("R_L__R__R_".to_string(), "_L______RR".to_string()), false);
    assert_eq!(Solution::can_change("_R__L__R_".to_string(), "_RLLL__RR".to_string()), false);

}

const R: u8 = 'R' as u8;
const L: u8 = 'L' as u8;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let n = start.len();
        let mut stack = VecDeque::new();
        let mut left_stack = VecDeque::new();
        let start = start.as_bytes();
        let target = target.as_bytes();
        if start.iter().filter(|&x| *x == L).count() != target.iter().filter(|&x| *x == L).count() ||
            start.iter().filter(|&x| *x == R).count() != target.iter().filter(|&x| *x == R).count() {
            return false;
        }
        let mut i = 0;
        while i < n {
            if start[i] == R {
                stack.push_back(R);
            }
            if target[i] == L {
                left_stack.push_back(L);
            }
            if target[i] == R {
                match stack.back() {
                    None => return false,
                    Some(&x) => if x != R {
                        return false;
                    }
                }
            } else if target[i] == L {
                stack.clear();
            }
            if start[i] == R {
                left_stack.clear();
            } else if start[i] == L {
                match left_stack.back() {
                    None => return false,
                    Some(&x) => if x != L {
                        return false;
                    }
                }
            }
            i += 1
        }
        left_stack.is_empty()
    }
}
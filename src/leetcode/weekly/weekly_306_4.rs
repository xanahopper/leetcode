use crate::leetcode::Solution;
use std::collections::{HashMap, VecDeque};

#[test]
fn weekly_306_4_test() {
    assert_eq!(Solution::count_special_numbers(5), 5);
    assert_eq!(Solution::count_special_numbers(20), 19);
    assert_eq!(Solution::count_special_numbers(135), 110);
}

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let mut nums = VecDeque::new();
        let mut num = n;
        while num > 0 {
            nums.push_front(num % 10);
            num = num / 10;
        }
        let len = nums.len();
        let mut dp: Vec<HashMap<u32, i32>> = vec![HashMap::new(); len];
        Solution::count_special_numbers_f(&mut dp, &nums, 0, 0, true, false)
    }

    fn count_special_numbers_f(dp: &mut Vec<HashMap<u32, i32>>, nums: &VecDeque<i32>, i: usize, mask: u32, is_limit: bool, is_num: bool) -> i32 {
        if i == nums.len() {
            return if is_num { 1 } else { 0 }
        }
        match dp[i].get(&mask) {
            Some(&x) if !is_limit && is_num => x,
            _ => {
                let start = if is_num { 0 } else { 1 };
                let up = if is_limit { nums[i] } else { 9 };
                let res = (start..=up).fold(0, |sum, d| {
                    if mask >> d & 1 == 0 {
                        sum + Solution::count_special_numbers_f(dp, nums, i + 1, mask | (1 << d), is_limit && d == up, true)
                    } else {
                        sum
                    }
                }) + if !is_num { Solution::count_special_numbers_f(dp, nums, i + 1, mask, false, false) } else { 0 };
                if !is_limit && is_num {
                    dp[i].insert(mask, res);
                }
                res
            }
        }
    }
}

use std::cmp::max;
use std::collections::VecDeque;
use crate::leetcode::Solution;

#[test]
fn weekly_295_3_test() {
    assert_eq!(Solution::total_steps(vec![5,3,4,4,7,3,6,11,8,5,11]), 3);
    assert_eq!(Solution::total_steps(vec![4,5,7,7,13]), 0);
    assert_eq!(Solution::total_steps(vec![10,1,2,3,4,5,6,1,2,3]), 6);
    assert_eq!(Solution::total_steps(vec![7,11,1]), 1);
    assert_eq!(Solution::total_steps(vec![7,14,4,14,13,2,6,13]), 3);
}

struct Step {
    step: i32,
    val: i32
}

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        nums.iter().fold((VecDeque::new(), 0i32), |(mut stack, max_step): (VecDeque<Step>, i32), &n| {
            let mut step = 0;
            while stack.len() > 0 && stack.back().unwrap().val <= n {
                step = max(step, stack.back().unwrap().step);
                stack.pop_back();
            };
            if stack.len() > 0 {
                step += 1;
            };
            let max_step = max(max_step, step);
            stack.push_back(Step { step, val: n });
            (stack, max_step)
        }).1

    }
}
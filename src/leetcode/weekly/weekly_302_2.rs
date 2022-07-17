use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn weekly_300_4_test() {
    assert_eq!(Solution::maximum_sum(vec![18,43,36,13,7]), 54);
    assert_eq!(Solution::maximum_sum(vec![10,12,19,14]), -1);
}

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sums: HashMap<i32, (i32, i32)> = HashMap::new();
        for num in nums {
            let s = Solution::get_num_sum(num);
            let entry = sums.entry(s).or_insert((0, 0));
            let (a, b) = *entry;
            if num > a {
                *entry = (num, a)
            } else if num > b {
                *entry = (a, num)
            }
        }
        sums.values().map(|&(a, b)| if a > 0 && b > 0 { a + b } else { -1 }).max().unwrap_or(-1)
    }

    fn get_num_sum(num: i32) -> i32 {
        let mut x = 0;
        let mut num = num;
        while num > 0 {
            x += num % 10;
            num = num / 10;
        }
        x
    }
}
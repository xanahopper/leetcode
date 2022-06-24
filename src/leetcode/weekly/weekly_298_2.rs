use crate::leetcode::Solution;

#[test]
fn weekly_298_2_test() {
    assert_eq!(Solution::minimum_numbers(58, 9), 2);
    assert_eq!(Solution::minimum_numbers(60, 5), 2);
    assert_eq!(Solution::minimum_numbers(37, 2), -1);
    assert_eq!(Solution::minimum_numbers(0, 7), 0);
    assert_eq!(Solution::minimum_numbers(10, 1), 10);
    assert_eq!(Solution::minimum_numbers(10, 0), 1);
    assert_eq!(Solution::minimum_numbers(4, 0), -1);
}

impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let last = num % 10;
        for i in 1..=10 {
            if i * k % 10 == last && i * k <= num {
                return i
            }
        }
        -1
    }
}
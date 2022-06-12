use std::cmp::min;
use crate::leetcode::Solution;

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut tax = 0.0;
        let mut last_upper = 0;
        for bracket in brackets {
            let upper = min(income, bracket[0]);
            let percent = bracket[1];
            if last_upper >= income {
                break
            }
            tax += (upper - last_upper) as f64 * percent as f64 / 100f64;
            last_upper = upper;
        }
        tax
    }
}

#[test]
fn weekly_297_1_test() {
    assert_eq!(Solution::calculate_tax(vec![vec![3,50],vec![7,10],vec![12,25]], 10), 2.65000);
    assert_eq!(Solution::calculate_tax(vec![vec![1,0],vec![4,25],vec![5,50]], 2), 0.25000);
    assert_eq!(Solution::calculate_tax(vec![vec![2,50]], 0), 0.000);
}
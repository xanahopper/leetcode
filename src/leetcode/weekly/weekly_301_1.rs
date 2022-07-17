use crate::leetcode::Solution;

#[test]
fn weekly_301_1_test() {
    assert_eq!(Solution::fill_cups(vec![1,4,2]), 4);
    assert_eq!(Solution::fill_cups(vec![5,4,4]), 7);
    assert_eq!(Solution::fill_cups(vec![5,0,0]), 5);
}

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_by(|a,b| b.cmp(a));
        let mut a1 = amount[0];
        let mut a2 = amount[1];
        let mut a3 = amount[2];
        let mut res = 0;
        while amount.iter().any(|&x| x > 0) {
            amount[0] -= 1;
            if amount[1] > 0 {
                amount[1] -= 1;
            }
            res += 1;
            amount.sort_by(|a,b| b.cmp(a));
        }
        res
    }
}
use crate::leetcode::Solution;

#[test]
fn weekly_304_1_test() {
    assert_eq!(Solution::maximum_groups(vec![10,6,12,7,3,5,]), 3);
    assert_eq!(Solution::maximum_groups(vec![8,8]), 1);
}

impl Solution {
    pub fn maximum_groups(mut grades: Vec<i32>) -> i32 {
        grades.sort();
        let n = grades.len();
        let mut last_len = 1;
        let mut last_sum = grades[0];
        let mut len = 0;
        let mut sum = 0;
        let mut count = 1;
        for &grade in &grades[1..n] {
            len += 1;
            sum += grade;
            if len > last_len && sum > last_sum {
                count += 1;
                last_len = len;
                last_sum = sum;
                len = 0;
                sum = 0;
            }
        }
        count
    }
}
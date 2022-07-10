use crate::leetcode::Solution;

#[test]
fn weekly_300_3_test() {
    assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
    assert_eq!(Solution::people_aware_of_secret(5, 1, 3), 10);
    assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
}

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let len = forget - delay;
        let mut count = vec![0; n as usize + 1];
        count[0] = 1;
        for i in 1..n as usize {
            let forget_index = i as i32 - forget + 1;
            let forget_count = if forget_index >= 0 { count[forget_index as usize] } else { 0 };
            let aware_index = (i as i32 - delay) as usize;
            let aware_count: i32 = count[aware_index.max(0)..(i - 1)].iter().sum();
            count[i] = (count[i - 1] - forget_count + aware_count) % 1000000007;
        }
        count[n as usize - 1]
    }
}
use crate::leetcode::Solution;

#[test]
fn weekly_300_4_test() {
    assert_eq!(Solution::ideal_arrays(2, 5), 10);
    assert_eq!(Solution::ideal_arrays(5, 3), 11);
}

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let mut count = vec![vec![0; max_value as usize + 1]; n as usize + 1];
        for i in 1..=max_value {
            count[1][i as usize] = 1;
        }
        for i in 2..=n as usize {
            for j in 1..=max_value as usize {
                for k in 1..=j {
                    if j % k == 0 {
                        count[i][j] += count[i - 1][k];
                    }
                }
            }
        }
        count[n as usize].iter().sum()
    }
}
use crate::leetcode::Solution;

#[test]
fn weekly_299_2_test() {
    assert_eq!(Solution::count_house_placements(1), 4);
    assert_eq!(Solution::count_house_placements(2), 9);
    assert_eq!(Solution::count_house_placements(3), 25);
    assert_eq!(Solution::count_house_placements(4), 64);
    assert_eq!(Solution::count_house_placements(1000), 500478595);
}

impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0i64; 2]; n as usize];
        dp[0][0] = 1;
        dp[0][1] = 1;
        for i in 1..n as usize {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) % 1_000_000_007i64;
            dp[i][1] = dp[i - 1][0] % 1_000_000_007i64;
        }
        let max: i64 = dp[n - 1][0] as i64 + dp[n - 1][1] as i64;
        (max * max % 1_000_000_007i64) as i32
    }
}
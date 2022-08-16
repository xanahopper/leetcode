use crate::leetcode::Solution;

#[test]
fn weekly_306_1_test() {
    assert_eq!(Solution::largest_local(vec![vec![9,9,8,1],vec![5,6,2,6],vec![8,2,6,4],vec![6,2,2,2]]), vec![vec![9,9],vec![8,6]]);
}

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut res = vec![vec![0; n - 2]; n - 2];
        for i in 0..n - 2 {
            for j in 0..n - 2 {
                res[i][j] = grid[i][j].max(grid[i][j + 1]).max(grid[i][j + 2])
                    .max(grid[i + 1][j]).max(grid[i + 1][j + 1]).max(grid[i + 1][j + 2])
                    .max(grid[i + 2][j]).max(grid[i + 2][j + 1]).max(grid[i + 2][j + 2]);
            }
        }
        res
    }
}
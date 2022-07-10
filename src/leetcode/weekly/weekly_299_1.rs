use crate::leetcode::Solution;

#[test]
fn weekly_299_1_test() {
    assert_eq!(Solution::check_x_matrix(vec![vec![2,0,0,1],vec![0,3,1,0],vec![0,5,2,0],vec![4,0,0,2]]), true);
}

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for i in 0..n {
            for j in 0..n {
                if i == j || i + j == n - 1 {
                    if grid[i][j] == 0 {
                        return false;
                    }
                } else if grid[i][j] != 0 {
                    return false;
                }
            }
        }
        true
    }
}
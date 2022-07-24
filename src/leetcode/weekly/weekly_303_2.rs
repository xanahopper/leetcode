use crate::leetcode::Solution;

#[test]
fn weekly_303_2_test() {

}

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                let mut equal = true;
                for k in 0..n {
                    if grid[i][k] != grid[k][j] {
                        equal = false;
                        break;
                    }
                }
                if equal {
                    res += 1;
                }
            }
        }
        res
    }
}
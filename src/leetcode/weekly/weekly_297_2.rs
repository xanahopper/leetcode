use std::cmp::min;
use crate::leetcode::Solution;

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();
        let first: Vec<i32> = grid.first().unwrap().clone();
        let cost = grid.windows(2).enumerate()
            .fold(vec![vec![0; n], first], |mut res, (index, line)| {
                let i = index % 2;
                let i_1 = if index > 0 { (index - 1) % 2 } else { 1 };
                for j in 0..n {
                    res[i][j] = i32::MAX;
                }
                for (k, &v) in line[0].iter().enumerate() {
                    let v_k = v as usize;
                    for j in 0..n {
                        res[i][j] = res[i][j].min(res[i_1][k] + move_cost[v_k][j] + line[1][j])
                    }
                }
                res
            });
        *cost[(m - 2) % 2].iter().min().unwrap()
    }
}

#[test]
fn weekly_297_2_test() {
    assert_eq!(Solution::min_path_cost(vec![vec![5,3],vec![4,0],vec![2,1]], vec![vec![9,8],vec![1,5],vec![10,12],vec![18,6],vec![2,4],vec![14,3]]), 17);
    assert_eq!(Solution::min_path_cost(vec![vec![5,1,2],vec![4,0,3]], vec![vec![12,10,15],vec![20,23,8],vec![21,7,1],vec![8,1,13],vec![9,10,25],vec![5,3,2]]), 6);
}
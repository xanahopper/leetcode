use crate::leetcode::Solution;

#[test]
fn weekly_306_2_test() {
    assert_eq!(Solution::edge_score(vec![1,0,0,0,0,7,7,5]), 7);
    assert_eq!(Solution::edge_score(vec![2,0,0,2]), 0);
}

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let counter = edges.iter().enumerate().fold(vec![0i64; edges.len()], |mut counter, (idx , &edge)| {
            let index = idx as i64;
            counter[edge as usize] += index;
            counter
        });
        let mut res = 0;
        let mut max = counter[0];
        for (i, &v) in counter.iter().enumerate() {
            if v > max {
                res = i as i32;
                max = v;
            }
        }
        res
    }
}
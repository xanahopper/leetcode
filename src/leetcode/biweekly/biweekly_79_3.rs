use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn biweekly_79_3_test() {
    assert_eq!(Solution::maximum_importance(5, vec![vec![0,1],vec![1,2],vec![2,3],vec![0,2],vec![1,3],vec![2,4]]), 43);
    assert_eq!(Solution::maximum_importance(5, vec![vec![0,3],vec![2,4],vec![1,3]]), 20);
    assert_eq!(Solution::maximum_importance(5, vec![vec![0,1]]), 9);
    assert_eq!(Solution::maximum_importance(5, vec![vec![0,1], vec![1,2]]), 17);
}

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut counts: Vec<_> = roads.iter().fold(HashMap::new(), |mut res, r| {
            let p1 = r[0];
            let p2 = r[1];
            *res.entry(p1).or_insert(0) += 1;
            *res.entry(p2).or_insert(0) += 1;
            res
        }).into_iter().collect();
        counts.sort_by(|x, y| y.1.cmp(&x.1));
        counts.iter().enumerate().fold(0, |res, (index, (_, count))| {
            res + ((n - index as i32) as i64 * (*count as i64))
        })
    }
}
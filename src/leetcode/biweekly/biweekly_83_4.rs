use std::collections::HashSet;
use crate::leetcode::Solution;

#[test]
fn biweekly_83_4_test() {
    assert_eq!(Solution::shortest_sequence(vec![4,2,1,2,3,3,2,4,1], 4), 3);
    assert_eq!(Solution::shortest_sequence(vec![1,1,2,2], 2), 2);
    assert_eq!(Solution::shortest_sequence(vec![1,1,3,2,2,2,3,3], 4), 1);
}

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut counter = HashSet::new();
        let mut res = 1;
        for r in rolls {
            counter.insert(r);
            if counter.len() as i32 >= k {
                res += 1;
                counter.clear();
            }
        }
        res
    }
}
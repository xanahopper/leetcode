use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn weekly_296_3_test() {
    assert_eq!(Solution::array_change(vec![1,2,4,6], vec![vec![1,3],vec![4,7],vec![6,1]]), vec![3,2,7,1]);
    assert_eq!(Solution::array_change(vec![1,2], vec![vec![1,3],vec![2,1],vec![3,2]]), vec![2,1])
}

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let overrides: HashMap<i32, i32> = operations.iter().fold((HashMap::new(), HashMap::new()), |(mut r, mut rev), op| {
            let from = op[0];
            let target = op[1];
            if let Some(origin) = rev.remove(&from) {
                rev.insert(target, origin);
                r.insert(origin, target);
            } else {
                rev.insert(target, from);
                r.insert(from, target);
            }
            (r, rev)
        }).0;
        nums.iter().map(|x| *overrides.get(x).unwrap_or(x)).collect()
    }
}
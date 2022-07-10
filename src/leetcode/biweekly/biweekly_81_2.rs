use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn biweekly_81_2_test() {
    assert_eq!(Solution::count_pairs(3, vec![vec![0,1],vec![0,2],vec![1,2]]), 0);
    assert_eq!(Solution::count_pairs(1, vec![]), 0);
    assert_eq!(Solution::count_pairs(7, vec![vec![0,2],vec![5,4],vec![4,2],vec![1,6],vec![2,0]]), 14);
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut v = (0..n as usize).collect();
        for e in edges {
            let e1 = e[0] as usize;
            let e2 = e[1] as usize;
            let a = Solution::find_root(&mut v, e1);
            let b = Solution::find_root(&mut v, e2);
            if a != b {
                v[a] = b;
            }
        }
        let mut group = HashMap::new();
        let mut result = 0;
        let mut sum = 0;
        for i in 0..n as usize {
            let parent = Solution::find_root(&mut v, i);
            if group.contains_key(&parent) {
                result += (sum - group[&parent]);
                group.entry(parent).and_modify(|x| *x += 1);
            } else {
                let entry = group.entry(parent).or_insert(1i64);
                result += sum;
            }
            sum += 1;
        }
        result
    }

    fn find_root(v: &mut Vec<usize>, i: usize) -> usize {
        if i == v[i] {
            return v[i];
        }
        v[i] = Solution::find_root(v, v[i]);
        v[i]
    }
}
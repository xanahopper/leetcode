use std::collections::{HashMap, HashSet, VecDeque};
use crate::leetcode::Solution;

#[test]
fn weekly_305_2_test() {
    assert_eq!(Solution::reachable_nodes(7, vec![vec![0,1],vec![1,2],vec![3,1],vec![4,0],vec![0,5],vec![5,6]], vec![4,5]), 4);
    assert_eq!(Solution::reachable_nodes(7, vec![vec![0,1],vec![0,2],vec![0,5],vec![0,4],vec![3,2],vec![6,5]], vec![4, 2, 1]), 3);
    assert_eq!(Solution::reachable_nodes(10, vec![vec![4,1],vec![1,3],vec![1,5],vec![0,5],vec![3,6],vec![8,4],vec![5,7],vec![6,9],vec![3,2]], vec![2,7]), 8);
}

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let restricted_set: HashSet<_> = restricted.into_iter().collect();
        let mut parents = edges.iter().fold(HashMap::new(), |mut parent, edge| {
            let a = edge[0];
            let b = edge[1];
            let (a, b) = (a.max(b), a.min(b));
            if !restricted_set.contains(&a) && !restricted_set.contains(&b) {
                let entry_a = parent.entry(a).or_insert(vec![]);
                (*entry_a).push(b);
                let entry_b = parent.entry(b).or_insert(vec![]);
                (*entry_b).push(a);
            }
            parent
        });
        let mut group = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while !queue.is_empty() {
            if let Some(x) = queue.pop_front() {
                group.insert(x);
                if let Some(children) = parents.remove(&x) {
                    for t in children {
                        queue.push_back(t);
                    }
                }
            }
        }
        group.len() as i32
    }
}
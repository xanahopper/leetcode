use std::cmp::Ordering;
use std::collections::HashSet;
use crate::leetcode::Solution;

#[test]
fn weekly_304_1_test() {
    // assert_eq!(Solution::closest_meeting_node(vec![2,2,3,-1], 0, 1), 2);
    // assert_eq!(Solution::closest_meeting_node(vec![1,2,-1], 0, 2), 2);
    assert_eq!(Solution::closest_meeting_node(vec![2,0,0], 2, 0), 2);
}

#[derive(Copy, Clone)]
struct Parent {
    node: i32,
    distance: i32
}

impl PartialOrd for Parent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl PartialEq<Self> for Parent {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut parent_set1 = HashSet::new();
        let mut parent_set2 = HashSet::new();
        let mut p1 = node1;
        let mut p2 = node2;
        while p1 != p2 && p1 != -1 || p2 != -1 {
            parent_set1.insert(p1);
            p1 = Solution::get_parent(&edges, p1);
            parent_set2.insert(p2);
            p2 = Solution::get_parent(&edges, p2);
            let intersect = parent_set1.intersection(&parent_set2);
            for &x in intersect {
                if x != -1 {
                    return x;
                }
            }
        }
        if p1 == p2 { p1 } else { -1 }
    }

    fn get_parent(edges: &Vec<i32>, p: i32) -> i32 {
        if p == -1 {
            -1
        } else {
            edges[p as usize]
        }
    }
}
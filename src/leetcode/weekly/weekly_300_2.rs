use leetcode_prelude::{linkedlist, ListNode};
use crate::leetcode::Solution;

#[test]
fn weekly_300_2_test() {
    assert_eq!(Solution::spiral_matrix(3, 5, linkedlist![3,0,2,6,8,1,7,9,4,2,5,5,0]), vec![vec![3,0,2,6,8],vec![5,0,-1,-1,1],vec![5,2,4,9,7]]);
    assert_eq!(Solution::spiral_matrix(1, 4, linkedlist![0, 1, 2]), vec![vec![0, 1,2,-1]]);
}

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![-1; n as usize]; m as usize];
        let mut dx = 1;
        let mut dy = 0;
        let mut max_x = n;
        let mut min_x = 0;
        let mut max_y = m;
        let mut min_y = 0;
        let mut i = 0i32;
        let mut j = 0i32;
        let mut current = head;
        while current.is_some() {
            let node = current.unwrap();
            res[j as usize][i as usize] = node.val;
            if i + dx >= max_x {
                dx = 0;
                dy = 1;
                min_y += 1;
            } else if i + dx < min_x {
                dx = 0;
                dy = -1;
                max_y -= 1;
            }
            if j + dy >= max_y {
                dy = 0;
                dx = -1;
                max_x -= 1;
            } else if j + dy < min_y {
                dy = 0;
                dx = 1;
                min_x += 1;
            }
            i += dx;
            j += dy;
            current = node.next;
        }
        res
    }
}
use std::collections::BTreeMap;
use crate::leetcode::Solution;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct StrKey<'a> {
    s: &'a str,
    index: usize
}

#[test]
fn weekly_300_4_test() {
    assert_eq!(Solution::smallest_trimmed_numbers(
        vec!["102","473","251","814"].iter().map(|&s| s.to_owned()).collect(),
        vec![vec![1,1],vec![2,3],vec![4,2],vec![1,2]]
    ), vec![2,2,1,0]);

    assert_eq!(Solution::smallest_trimmed_numbers(
        vec!["24","37","96","04"].iter().map(|&s| s.to_owned()).collect(),
        vec![vec![2,1],vec![2,2]]
    ), vec![3,0]);
}

impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.iter().map(|q| {
            let k = q[0];
            let trim = q[1];
            Solution::trimmed_nums(&nums, trim, k)
        }).collect()
    }

    fn trimmed_nums(nums: &Vec<String>, trim: i32, k: i32) -> i32 {
        let n = nums[0].len();
        let start = n - trim as usize;
        let mut trimmed = nums.iter().enumerate().map(|(index, s)| (index, &s[start..]))
            .fold(BTreeMap::new(), |mut tree, (index, s)| {
                tree.insert(StrKey { index, s }, index);
                tree
            });
        let mut i = 0;
        for x in trimmed {
            i += 1;
            if i == k {
                return x.1 as i32
            }
        }
        return 0
    }
}
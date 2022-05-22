use crate::leetcode::Solution;

#[test]
fn weekly_294_2_test() {
    assert_eq!(Solution::maximum_bags(vec![2,3,4,5], vec![1,2,4,4], 2), 3);
    assert_eq!(Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100), 3);
}

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remains: Vec<i32> = capacity.iter().zip(rocks.iter()).map(|(&c, &r)| c - r).collect();
        remains.sort();
        remains.iter().fold((0, additional_rocks), |(count, r), &remain| {
            if r >= remain {
                (count + 1, r - remain)
            } else {
                (count, r)
            }
        }).0
    }
}
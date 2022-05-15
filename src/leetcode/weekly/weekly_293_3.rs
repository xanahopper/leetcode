use crate::leetcode::Solution;

#[test]
fn weekly_293_3_test() {
    assert_eq!(Solution::largest_combination(vec![16,17,71,62,12,24,14]), 4);
    assert_eq!(Solution::largest_combination(vec![8, 8]), 2);
    assert_eq!(Solution::largest_combination(vec![84,40,66,44,91,90,1,14,73,51,47,35,18,46,18,65,55,18,16,45,43,58,90,92,91,43,44,76,85,72,24,89,60,94,81,90,86,79,84,41,41,28,44]), 28);

}

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        (0..=30).map(|i| {
            candidates.iter().filter(|&c| *c >> i & 1  == 1).count() as i32
        })
            .max().unwrap()
    }
}
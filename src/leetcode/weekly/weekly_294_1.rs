use crate::leetcode::Solution;

#[test]
fn weekly_294_1_test() {
    assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
    assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    assert_eq!(Solution::percentage_letter("vmvvvvvzrvvpvdvvvvyfvdvvvvpkvvbvvkvvfkvvvkvbvvnvvomvzvvvdvvvkvvvvvvvvvlvcvilaqvvhoevvlmvhvkvtgwfvvzy".to_string(), 'v'), 59);
}

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let count = s.chars().filter(|&x| x == letter).count();
        (count as f32 * 100.0 / s.len() as f32) as i32
    }
}
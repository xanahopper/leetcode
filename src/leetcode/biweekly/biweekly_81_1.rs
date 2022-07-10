use crate::leetcode::Solution;

#[test]
fn biweekly_81_1_test() {
    assert_eq!(Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()), 2);
    assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
    assert_eq!(Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()), 5);
}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.as_bytes().iter().fold((false, 0), |(in_pair, count), &c| {
            if c == '|' as u8 {
                (!in_pair, count)
            } else if c == '*' as u8 {
                (in_pair, count + if in_pair { 0 } else { 1 })
            } else {
                (in_pair, count)
            }
        }).1
    }
}
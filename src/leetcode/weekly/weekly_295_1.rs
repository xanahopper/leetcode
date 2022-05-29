use crate::leetcode::Solution;

#[test]
fn weekly_295_1_test() {
    assert_eq!(Solution::rearrange_characters("ilovecodingonleetcode".to_string(), "code".to_string()), 2);
    assert_eq!(Solution::rearrange_characters("abcba".to_string(), "abc".to_string()), 1);
    assert_eq!(Solution::rearrange_characters("abbaccaddaeea".to_string(), "aaaaa".to_string()), 1);
    assert_eq!(Solution::rearrange_characters("abc".to_string(), "abcd".to_string()), 0);
    assert_eq!(Solution::rearrange_characters("xys".to_string(), "adk".to_string()), 0);
    assert_eq!(Solution::rearrange_characters("wvu".to_string(), "tu".to_string()), 0);
}

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        if s.len() < target.len() {
            return 0
        }
        let s_chars = s.into_bytes()
            .iter()
            .map(|x| x - 'a' as u8)
            .fold(vec![0i32; 26], |mut counter, c| {
                counter[c as usize] += 1;
                counter
            });;
        let target_counter: Vec<_> = target.into_bytes()
            .iter()
            .map(|x| x - 'a' as u8)
            .fold(vec![0i32; 26], |mut counter, c| {
                counter[c as usize] += 1;
                counter
            });
        s_chars.iter().zip(target_counter.iter())
            .map(|(&sc, &tc)| if tc > 0 { sc / tc } else { i32::MAX })
            .min().unwrap_or(0)
    }
}
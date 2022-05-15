use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn weekly_293_1_test() {
    assert_eq!(
        Solution::remove_anagrams(vec!["abba".to_string(),"baba".to_string(),"bbaa".to_string(),"cd".to_string(),"cd".to_string()]),
        vec!["abba","cd"]
    );
}

struct Result {
    vector: Vec<String>,
    previous_map: HashMap<u8, u16>
}

impl Solution {

    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        res.extend(words);
        res.into_iter().map(|x| x.into_bytes())
            .fold( Result { vector: vec![],  previous_map: HashMap::new() }, |mut res, ch| {
                let current_map = Solution::calc_map(&ch);
                if current_map != res.previous_map {
                    res.vector.push(String::from_utf8(ch).unwrap());
                    res.previous_map = current_map;
                }
                res
            }).vector
    }

    fn calc_map(ch: &[u8]) -> HashMap<u8, u16> {
        ch.into_iter().fold(HashMap::new(), |mut res, c| {
            let count =res.entry(*c).or_insert(0);
            *count += 1;
            res
        })
    }
}
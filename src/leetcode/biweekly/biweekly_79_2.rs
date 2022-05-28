use std::collections::HashMap;
use crate::leetcode::Solution;

#[test]
fn biweekly_79_2_test() {
    assert_eq!(Solution::largest_word_count(
        vec!["Hello userTwooo".to_string(),"Hi userThree".to_string(),"Wonderful day Alice".to_string(),"Nice day userThree".to_string()],
        vec!["Alice".to_string(),"userTwo".to_string(),"userThree".to_string(),"Alice".to_string()]),
               "Alice");
}

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {

        let mut counts = messages.iter().zip(senders.iter())
            .fold(HashMap::new(), |mut res, (msg, s)| {
                let count = msg.split(" ").count();
                let mut counter = res.entry(s).or_insert(0usize);
                *counter += count;
                res
            });
        let mut v: Vec<_> = counts.into_iter().collect();
        v.sort_by(|x, y| y.1.cmp(&x.1).then(y.0.cmp(&x.0)));
        v.first().unwrap().0.into()
    }
}
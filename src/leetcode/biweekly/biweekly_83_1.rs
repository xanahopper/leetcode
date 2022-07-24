use crate::leetcode::Solution;

#[test]
fn biweekly_83_1_test() {
    assert_eq!(Solution::best_hand(vec![13,2,3,1,9], vec!['a', 'a', 'a', 'a', 'a']), "Flush");
    assert_eq!(Solution::best_hand(vec![4,4,2,4,4], vec!['d', 'a', 'a', 'b', 'c']), "Three of a Kind");
    assert_eq!(Solution::best_hand(vec![10,10,2,12,9], vec!['a', 'b', 'c', 'a', 'd']), "Pair");
}

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.iter().all(|&x| x == suits[0]) {
            "Flush"
        } else {
            let mut counter = ranks.iter().fold(vec![0; 13], |mut c, &x| {
                c[(x - 1) as usize] += 1;
                c
            });
            let max = *counter.iter().max().unwrap();
            if max >= 3 {
                "Three of a Kind"
            } else if max == 2 {
                "Pair"
            } else {
                "High Card"
            }
        }.to_string()
    }
}
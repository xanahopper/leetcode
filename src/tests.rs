#[test]
fn n1_test() {
    use crate::leetcode::Solution;
    assert_eq!(Solution::two_sum(vec![1, 2, 3], 5), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn n2_test() {
    extern crate leetcode_prelude;
    use crate::leetcode::Solution;
    use leetcode_prelude::*;

    let l1 = linkedlist![2, 4, 3];
    let l2 = linkedlist![5, 6, 4];

    assert_eq!(Solution::add_two_numbers(l1, l2), linkedlist![7, 0, 8]);
    assert_eq!(Solution::add_two_numbers(linkedlist![5], linkedlist![5]), linkedlist![0, 1]);
}

#[test]
fn n3_test() {
    use crate::leetcode::Solution;
    assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
}
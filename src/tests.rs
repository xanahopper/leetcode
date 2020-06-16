use crate::leetcode::Solution;

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

#[test]
fn n6_test() {
    use crate::leetcode::Solution;
    assert_eq!(Solution::convert(String::from("PAYPALISHIRING"), 3), String::from("PAHNAPLSIIGYIR"));
    assert_eq!(Solution::convert(String::from("PAYPALISHIRING"), 4), String::from("PINALSIGYAHRPI"));
}

#[test]
fn n7_test() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(i32::MIN), 0);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
}

#[test]
fn n8_test() {
    assert_eq!(Solution::my_atoi(String::from("42")), 42);
    assert_eq!(Solution::my_atoi(String::from("+1")), 1);
    assert_eq!(Solution::my_atoi(String::from("   +0 123")), 0);
    assert_eq!(Solution::my_atoi(String::from("   -42")), -42);
    assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
    assert_eq!(Solution::my_atoi(String::from("words and 987")), 0);
    assert_eq!(Solution::my_atoi(String::from("-91283472332")), -2147483648);
    assert_eq!(Solution::my_atoi(String::from("2147483648")), 2147483647);
    assert_eq!(Solution::my_atoi(String::from("-   234")), 0);
    assert_eq!(Solution::my_atoi(String::from("-5-")), -5);
}

#[test]
fn n14_test() {
    assert_eq!(Solution::longest_common_prefix(
        vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]),
        "fl".to_string());
    assert_eq!(Solution::longest_common_prefix(
        vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
    ),
               String::new());
    assert_eq!(Solution::longest_common_prefix(
        vec!["c".to_string(), "c".to_string()]
    ),
        "c".to_string());
}

#[test]
fn n28_test() {
    assert_eq!(Solution::str_str(String::from("hello"), String::from("ll")), 2);
    assert_eq!(Solution::str_str(String::from("aaaaa"), String::from("bba")), -1);
}

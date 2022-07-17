use crate::leetcode::Solution;

#[test]
fn weekly_302_4_test() {
    assert_eq!(Solution::min_operations(vec![2,3,2,4,3], vec![9,6,9,3,15]), 2);
    assert_eq!(Solution::min_operations(vec![4,3,6], vec![8,2,6,10]), -1);
    assert_eq!(Solution::min_operations(vec![3,2,6,2,35,5,35,2,5,8,7,3,4],
                                        vec![105,70,70,175,105,105,105]), 6);
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {

        let division = nums_divide[1..].iter().fold(nums_divide[0], |target, &x| Solution::gcd(target, x));
        let mut nums = nums;
        nums.sort();
        match nums.iter().enumerate().find(|&(i, &x)| division % x == 0) {
            Some((idx, _)) => idx as i32,
            None => -1
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            let r = a % b;
            let a = b;
            let b = r;
            Solution::gcd(a, b)
        }
    }
}
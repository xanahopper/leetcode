use crate::leetcode::Solution;

#[test]
fn biweekly_82_3_test() {
    assert_eq!(Solution::min_sum_square_diff(vec![1,2,3,4], vec![2,10,20,19], 0, 0), 579);
    assert_eq!(Solution::min_sum_square_diff(vec![1,4,10,12], vec![5,8,6,9], 1, 1), 43);
}

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut nums = nums1.iter().zip(nums2.iter())
            .map(|(&n1, &n2)| (n1 as i64 + n2 as i64).abs())
            .collect::<Vec<_>>();
        nums.sort_by(|a, b| b.cmp(a));
        let mut count = k1 as i64 + k2 as i64;
        let len = nums.len();
        let mut i = 0;
        while count > 0 && i < len {
            if nums[i] > count {
                nums[i] -= count;
                count = 0;
            } else {
                count -= nums[i];
                nums[i] = 0;
            }
        }
        nums.iter().map(|&x| x * x).sum()
    }
}
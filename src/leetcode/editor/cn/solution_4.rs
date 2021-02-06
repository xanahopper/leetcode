//Median of Two Sorted Arrays
//There are two sorted arrays nums1 and nums2 of size m and n respectively. 
//
// Find the median of the two sorted arrays. The overall run time complexity sho
//uld be O(log (m+n)). 
//
// You may assume nums1 and nums2 cannot be both empty. 
//
// Example 1: 
//
// 
//nums1 = [1, 3]
//nums2 = [2]
//
//The median is 2.0
// 
//
// Example 2: 
//
// 
//nums1 = [1, 2]
//nums2 = [3, 4]
//
//The median is (2 + 3)/2 = 2.5
// 
// Related Topics Array Binary Search Divide and Conquer


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::i32::MAX;
use std::cmp::Ordering;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mid = (nums1.len() + nums2.len()) as f32 / 2.0;
        let mut n1_iter = nums1.iter();
        let mut n2_iter = nums2.iter();
        let mut n1 = n1_iter.next().unwrap_or(&MAX);
        let mut n2 = n2_iter.next().unwrap_or(&MAX);

        let mut count = 0;
        let mut result = Vec::<i32>::new();
        loop {
            if (n1 == &MAX && n2 == &MAX) || count > mid as i32 {
                break;
            }
            match n1.cmp(&n2) {
                Ordering::Less => {
                    result.push(*n1);
                    n1 = n1_iter.next().unwrap_or(&MAX);
                }
                _ => {
                    result.push(*n2);
                    n2 = n2_iter.next().unwrap_or(&MAX);
                }
            }
            count += 1;
        }
        let left = mid as usize;
        let right = mid.ceil() as usize;
        match left.cmp(&right) {
            Ordering::Equal => (result[left - 1] + result[right]) as f64 / 2.0,
            _ => result[left] as f64
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

//数对 (a,b) 由整数 a 和 b 组成，其数对距离定义为 a 和 b 的绝对差值。 
//
// 给你一个整数数组 nums 和一个整数 k ，数对由 nums[i] 和 nums[j] 组成且满足 0 <= i < j < nums.length 。
//返回 所有数对距离中 第 k 小的数对距离。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [1,3,1], k = 1
//输出：0
//解释：数对和对应的距离如下：
//(1,3) -> 2
//(1,1) -> 0
//(3,1) -> 2
//距离第 1 小的数对是 (1,1) ，距离为 0 。
// 
//
// 示例 2： 
//
// 
//输入：nums = [1,1,1], k = 2
//输出：0
// 
//
// 示例 3： 
//
// 
//输入：nums = [1,6,1], k = 3
//输出：5
// 
//
// 
//
// 提示： 
//
// 
// n == nums.length 
// 2 <= n <= 10⁴ 
// 0 <= nums[i] <= 10⁶ 
// 1 <= k <= n * (n - 1) / 2 
// 
// Related Topics 数组 双指针 二分查找 排序 👍 352 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let len = nums.len();
        let mut left = 0;
        let mut right = nums[len - 1] - nums[0];
        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 0usize;
            let mut i = 0;
            while i < len {
                let mut j = 1;
                while j < len && nums[j] - nums[i] <= mid {
                    j += 1;
                }
                count += (j - i - 1);
                i += 1;
            }
            if count >= k as usize {
                right = mid
            } else {
                left = mid + 1
            }
        }
        right
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_719_test() {
    assert_eq!(Solution::smallest_distance_pair(vec![1,3,1], 1), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1,1,1], 2), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1,6,1], 3), 5);
}
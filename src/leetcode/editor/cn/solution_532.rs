//给你一个整数数组 nums 和一个整数 k，请你在数组中找出 不同的 k-diff 数对，并返回不同的 k-diff 数对 的数目。 
//
// k-diff 数对定义为一个整数对 (nums[i], nums[j]) ，并满足下述全部条件： 
//
// 
// 0 <= i, j < nums.length 
// i != j 
// nums[i] - nums[j] == k 
// 
//
// 注意，|val| 表示 val 的绝对值。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [3, 1, 4, 1, 5], k = 2
//输出：2
//解释：数组中有两个 2-diff 数对, (1, 3) 和 (3, 5)。
//尽管数组中有两个 1 ，但我们只应返回不同的数对的数量。
// 
//
// 示例 2： 
//
// 
//输入：nums = [1, 2, 3, 4, 5], k = 1
//输出：4
//解释：数组中有四个 1-diff 数对, (1, 2), (2, 3), (3, 4) 和 (4, 5) 。
// 
//
// 示例 3： 
//
// 
//输入：nums = [1, 3, 1, 5, 4], k = 0
//输出：1
//解释：数组中只有一个 0-diff 数对，(1, 1) 。
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 10⁴ 
// -10⁷ <= nums[i] <= 10⁷ 
// 0 <= k <= 10⁷ 
// 
// Related Topics 数组 哈希表 双指针 二分查找 排序 👍 218 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;
use crate::algorithm::binary_search::{LowerBoundBinarySearch, UpperBoundBinarySearch};

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut count = 0;
        let mut i = 0;
        while i < len {
            let next_part = &nums[i + 1..len];
            let target = nums[i] + k;
            let lower = next_part.lower_bound(&target);
            let upper = next_part.upper_bound(&target);
            if lower < upper {
                count += 1;
            }
            i = next_part.upper_bound(&nums[i]);
        }
        count
    }

    fn find_pairs_worse(nums: Vec<i32>, k: i32) -> i32 {
        let mut visited = HashSet::new();
        let mut res = HashSet::new();
        for num in nums {
            if visited.contains(&(num - k)) {
                res.insert(num - k);
            }
            if visited.contains(&(num + k)) {
                res.insert(num);
            }
            visited.insert(num);
        }
        res.len() as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_532_test() {
    assert_eq!(Solution::find_pairs(vec![3,1,4,1,5], 2), 2);
    assert_eq!(Solution::find_pairs(vec![1,2,3,4,5], 1), 4);
    assert_eq!(Solution::find_pairs(vec![1,3,1,5,4], 0), 1);
}
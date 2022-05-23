/// 三数之和
////给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有和为 0 且
//不重
////复的三元组。 
////
//// 注意：答案中不可以包含重复的三元组。 
////
//// 
////
//// 示例 1： 
////
//// 
////输入：nums = [-1,0,1,2,-1,-4]
////输出：[[-1,-1,2],[-1,0,1]]
//// 
////
//// 示例 2： 
////
//// 
////输入：nums = []
////输出：[]
//// 
////
//// 示例 3： 
////
//// 
////输入：nums = [0]
////输出：[]
//// 
////
//// 
////
//// 提示： 
////
//// 
//// 0 <= nums.length <= 3000 
//// -10⁵ <= nums[i] <= 10⁵ 
//// 
//// Related Topics 数组 双指针 排序 👍 4800 👎 0
//
use crate::leetcode::Solution;
#[test]
fn solution_15_test() {
    assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
}


//leetcode submit region begin(Prohibit modification and deletion)
/// ```rust
/// use leetcode::leetcode::Solution;
/// use self::*;
/// assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]]);
/// ```
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut res = vec!();
        for (i, &n) in nums.iter().enumerate() {
            if n > 0 {
                break
            }
            if i > 0 && nums[i - 1] == n {
                continue
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = n + nums[left] + nums[right];
                if sum == 0 {
                    res.push(vec![n, nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1
                    }
                    while left <right && nums[right] == nums[right - 1] {
                        right -= 1
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1
                } else if sum > 0 {
                    right -= 1
                }
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)

//区域和检索 - 数组不可变
//给定一个整数数组 nums，求出数组从索引 i 到 j（i ≤ j）范围内元素的总和，包含 i、j 两点。 
//
// 
// 
// 实现 NumArray 类： 
//
// 
// NumArray(int[] nums) 使用数组 nums 初始化对象 
// int sumRange(int i, int j) 返回数组 nums 从索引 i 到 j（i ≤ j）范围内元素的总和，包含 i、j 两点（也就是 s
//um(nums[i], nums[i + 1], ... , nums[j])） 
// 
//
// 
//
// 示例： 
//
// 
//输入：
//["NumArray", "sumRange", "sumRange", "sumRange"]
//[[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
//输出：
//[null, 1, -1, -3]
//
//解释：
//NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
//numArray.sumRange(0, 2); // return 1 ((-2) + 0 + 3)
//numArray.sumRange(2, 5); // return -1 (3 + (-5) + 2 + (-1)) 
//numArray.sumRange(0, 5); // return -3 ((-2) + 0 + 3 + (-5) + 2 + (-1))
// 
//
// 
//
// 提示： 
//
// 
// 0 <= nums.length <= 104 
// -105 <= nums[i] <= 105 
// 0 <= i <= j < nums.length 
// 最多调用 104 次 sumRange 方法 
// 
// 
// 
// Related Topics 动态规划 
// 👍 277 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
pub struct NumArray {
    data: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    pub fn new(nums: Vec<i32>) -> Self {
        NumArray {
            data: nums
        }
    }
    
    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.data[i as usize..=j as usize].iter().sum()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

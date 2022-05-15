//给你一个数组 nums ，请你完成两类查询。 
//
// 
// 其中一类查询要求 更新 数组 nums 下标对应的值 
// 另一类查询要求返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 ，其中 left <= right 
// 
//
// 实现 NumArray 类： 
//
// 
// NumArray(int[] nums) 用整数数组 nums 初始化对象 
// void update(int index, int val) 将 nums[index] 的值 更新 为 val 
// int sumRange(int left, int right) 返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元
//素的 和 （即，nums[left] + nums[left + 1], ..., nums[right]） 
// 
//
// 
//
// 示例 1： 
//
// 
//输入：
//["NumArray", "sumRange", "update", "sumRange"]
//[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
//输出：
//[null, 9, null, 8]
//
//解释：
//NumArray numArray = new NumArray([1, 3, 5]);
//numArray.sumRange(0, 2); // 返回 1 + 3 + 5 = 9
//numArray.update(1, 2);   // nums = [1,2,5]
//numArray.sumRange(0, 2); // 返回 1 + 2 + 5 = 8
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 3 * 10⁴ 
// -100 <= nums[i] <= 100 
// 0 <= index < nums.length 
// -100 <= val <= 100 
// 0 <= left <= right < nums.length 
// 调用 update 和 sumRange 方法次数不大于 3 * 10⁴ 
// 
// Related Topics 设计 树状数组 线段树 数组 👍 451 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
struct NumArray {
    buf: Vec<i32>,
    n: i32
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    pub fn new(nums: Vec<i32>) -> Self {
        let n: usize = nums.len();
        let mut buf = vec![0 as i32; 2*n];
        buf[n..2*n].copy_from_slice(&nums);
        for i in (1..n).rev() {
            buf[i] = buf[2*i] + buf[2*i+1];
        }
        NumArray {buf, n: n as i32}
    }

    fn update(&mut self, index: i32, val: i32) {
        let mut idx = index + self.n;
        self.buf[idx as usize] = val;
        idx /= 2;

        while idx != 0 {
            self.buf[idx as usize] = self.buf[2*idx as usize] + self.buf[(2*idx+1) as usize];
            idx /= 2;
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut l = left + self.n;
        let mut r = right + self.n;
        let mut ans = 0;
        while l <= r {
            if l % 2 == 1 {
                ans += self.buf[l as usize];
                l += 1;
            }
            if r % 2 == 0 {
                ans += self.buf[r as usize];
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        return ans;
    }
}
//leetcode submit region end(Prohibit modification and deletion)

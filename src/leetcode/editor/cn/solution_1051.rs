//学校打算为全体学生拍一张年度纪念照。根据要求，学生需要按照 非递减 的高度顺序排成一行。 
//
// 排序后的高度情况用整数数组 expected 表示，其中 expected[i] 是预计排在这一行中第 i 位的学生的高度（下标从 0 开始）。 
//
// 给你一个整数数组 heights ，表示 当前学生站位 的高度情况。heights[i] 是这一行中第 i 位学生的高度（下标从 0 开始）。 
//
// 返回满足 heights[i] != expected[i] 的 下标数量 。 
//
// 
//
// 示例： 
//
// 
//输入：heights = [1,1,4,2,1,3]
//输出：3 
//解释：
//高度：[1,1,4,2,1,3]
//预期：[1,1,1,2,3,4]
//下标 2 、4 、5 处的学生高度不匹配。 
//
// 示例 2： 
//
// 
//输入：heights = [5,1,2,3,4]
//输出：5
//解释：
//高度：[5,1,2,3,4]
//预期：[1,2,3,4,5]
//所有下标的对应学生高度都不匹配。 
//
// 示例 3： 
//
// 
//输入：heights = [1,2,3,4,5]
//输出：0
//解释：
//高度：[1,2,3,4,5]
//预期：[1,2,3,4,5]
//所有下标的对应学生高度都匹配。 
//
// 
//
// 提示： 
//
// 
// 1 <= heights.length <= 100 
// 1 <= heights[i] <= 100 
// 
// Related Topics 数组 计数排序 排序 👍 139 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let height_count = heights.iter().fold(vec![0; 101usize], |mut count, &h| {
            count[h as usize] += 1;
            count
        });
        height_count.iter().enumerate().fold((0, 0), |(total, acc_index), (height, &count)| {
            let height = height as i32;
            let expect = heights[acc_index..acc_index + count].iter().filter(|&&x| x != height).count() as i32;
            (total + expect, acc_index + count)
        }).0
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_1051_test() {
    assert_eq!(Solution::height_checker(vec![1,1,4,2,1,3]), 3);
    assert_eq!(Solution::height_checker(vec![5,1,2,3,4]), 5);
    assert_eq!(Solution::height_checker(vec![100,5,1,2,3,4]), 6);
    assert_eq!(Solution::height_checker(vec![1,2,3,4,5]), 0);
    assert_eq!(Solution::height_checker(vec![1,2,3,4,5,100]), 0);
}

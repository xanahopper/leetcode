//给你一个大小为 m x n 的矩阵 mat ，请以对角线遍历的顺序，用一个数组返回这个矩阵中的所有元素。 
//
// 
//
// 示例 1： 
//
// 
//输入：mat = [[1,2,3],[4,5,6],[7,8,9]]
//输出：[1,2,4,7,5,3,6,8,9]
// 
//
// 示例 2： 
//
// 
//输入：mat = [[1,2],[3,4]]
//输出：[1,2,3,4]
// 
//
// 
//
// 提示： 
//
// 
// m == mat.length 
// n == mat[i].length 
// 1 <= m, n <= 10⁴ 
// 1 <= m * n <= 10⁴ 
// -10⁵ <= mat[i][j] <= 10⁵ 
// 
// Related Topics 数组 矩阵 模拟 👍 356 👎 0


use std::ops::Range;
use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
trait Clamp {
    type Item;
    fn clamp(&self, val: Self::Item) -> Self::Item;
}

impl Clamp for Range<usize> {
    type Item = usize;

    fn clamp(&self, val: usize) -> usize {
        if val < self.start {
            self.start
        } else if val >= self.end {
            self.end - 1
        } else {
            val
        }
    }
}

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat.first().unwrap().len();
        let i_range = 0..m;
        let j_range = 0..n;
        let mut res = vec![0; m * n];
        let mut index = 0usize;
        for line in 0..(m + n - 1) {
            if line % 2 == 0 {
                for i in (0..=line).rev() {
                    let j = line - i;
                    if i_range.contains(&i) && j_range.contains(&j) {
                        res[index] = mat[i][j];
                        index += 1;
                    }
                }
            } else {
                for j in (0..=line).rev() {
                    let i = line - j;
                    if i_range.contains(&i) && j_range.contains(&j) {
                        res[index] = mat[i][j];
                        index += 1;
                    }
                }
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_498_test() {
    assert_eq!(Solution::find_diagonal_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), vec![1,2,4,7,5,3,6,8,9]);
    assert_eq!(Solution::find_diagonal_order(vec![vec![1,2],vec![3,4]]), vec![1,2,3,4]);
}

//给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列的所有元素都设为 0 。请使用 原地 算法。 
//
// 进阶： 
//
// 
// 一个直观的解决方案是使用 O(mn) 的额外空间，但这并不是一个好的解决方案。 
// 一个简单的改进方案是使用 O(m + n) 的额外空间，但这仍然不是最好的解决方案。 
// 你能想出一个仅使用常量空间的解决方案吗？ 
// 
//
// 
//
// 示例 1： 
//
// 
//输入：matrix = [[1,1,1],[1,0,1],[1,1,1]]
//输出：[[1,0,1],[0,0,0],[1,0,1]]
// 
//
// 示例 2： 
//
// 
//输入：matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
//输出：[[0,0,0,0],[0,4,5,0],[0,3,1,0]]
// 
//
// 
//
// 提示： 
//
// 
// m == matrix.length 
// n == matrix[0].length 
// 1 <= m, n <= 200 
// -231 <= matrix[i][j] <= 231 - 1 
// 
// Related Topics 数组 
// 👍 451 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut extra= Vec::new();
        matrix
            .iter()
            .enumerate()
            .for_each(|cur|
                extra.append(
                    &mut cur
                        .1
                        .iter()
                        .enumerate()
                        .filter(|v|*v.1==0)
                        .map(|e|(cur.0,e.0))
                        .collect::<Vec<(usize,usize)>>()));
        extra
            .iter()
            .map(|e|e.0)
            .for_each(|i|
                (0..matrix[0]
                    .len())
                    .for_each(|j|
                        matrix[i][j]=0));
        extra
            .iter()
            .map(|e|e.1)
            .for_each(|j|
                (0..matrix
                    .len())
                    .for_each(|i|
                        matrix[i][j]=0));
    }
}
//leetcode submit region end(Prohibit modification and deletion)

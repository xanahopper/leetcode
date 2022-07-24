//最大正方形
//在一个由 '0' 和 '1' 组成的二维矩阵内，找到只包含 '1' 的最大正方形，并返回其面积。 
//
// 
//
// 示例 1： 
//
// 
//输入：matrix = [['1','0','1','0','0'],['1','0','1','1','1'],['1','1','1','1','1']
//,['1','0','0','1','0']]
//输出：4
// 
//
// 示例 2： 
//
// 
//输入：matrix = [['0','1'],['1','0']]
//输出：1
// 
//
// 示例 3： 
//
// 
//输入：matrix = [['0']]
//输出：0
// 
//
// 
//
// 提示： 
//
// 
// m == matrix.length 
// n == matrix[i].length 
// 1 <= m, n <= 300 
// matrix[i][j] 为 '0' 或 '1' 
// 
// Related Topics 数组 动态规划 矩阵 👍 1145 👎 0

use crate::leetcode::Solution;

#[test]
fn solution_221_test() {
    assert_eq!(Solution::maximal_square(
        vec![
            vec!['1','0','1','0','0'],
            vec!['1','0','1','1','1'],
            vec!['1','1','1','1','1'],
            vec!['1','0','0','1','0']]
    ), 4);
    assert_eq!(Solution::maximal_square(
        vec![
            vec!['0', '1'],
            vec!['1', '0']
        ]
    ), 1);
    assert_eq!(Solution::maximal_square(
        vec![vec!['0']]
    ), 0)
}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        // let n = matrix.len();
        let m = matrix.first().unwrap().len();
        let mut dp = vec![vec![0; m + 1]; 2];
        let mut result = 0;
        for (i, line) in matrix.iter().enumerate() {
            let x = i % 2;
            for (j, &c) in line.iter().enumerate() {
                dp[x][j] = if c == '0' {
                    0
                } else if i == 0 || j == 0 {
                    if c == '1' { 1 } else { 0 }
                } else {
                    let prev_x = (i - 1) % 2;
                    dp[prev_x][j].min(dp[prev_x][j - 1]).min(dp[x][j-1]) + 1
                };
                result = result.max(dp[x][j]);
            }
        }
        result * result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

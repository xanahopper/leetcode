//螺旋矩阵
//给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。 
//
// 
//
// 示例 1： 
//
// 
//输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
//输出：[1,2,3,6,9,8,7,4,5]
// 
//
// 示例 2： 
//
// 
//输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
//输出：[1,2,3,4,8,12,11,10,9,5,6,7]
// 
//
// 
//
// 提示： 
//
// 
// m == matrix.length 
// n == matrix[i].length 
// 1 <= m, n <= 10 
// -100 <= matrix[i][j] <= 100 
// 
// Related Topics 数组 
// 👍 694 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut x = 0;
        let mut y = 0;
        let mut dx: i32 = 1;
        let mut dy: i32 = 0;
        let mut result = vec![];
        let n = matrix.len();
        let m = matrix.first().unwrap().len();
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = m as i32;
        let mut max_y = n as i32;
        while result.len() < n * m {
            result.push(matrix[y as usize][x as usize]);
            if x + dx >= max_x || x + dx < min_x {
                if dx > 0 {
                    dy = 1;
                    min_y += 1;
                } else if dx < 0 {
                    dy = -1;
                    max_y -= 1;
                }
                dx = 0;
            } else if y + dy >= max_y || y + dy < min_y {
                if dy > 0 {
                    dx = -1;
                    max_x -= 1;
                } else if dy < 0 {
                    dx = 1;
                    min_x += 1;
                }
                dy = 0
            }
            x += dx;
            y += dy;
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

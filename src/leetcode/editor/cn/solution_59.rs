//给你一个正整数 n ，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。 
//
// 
//
// 示例 1： 
//
// 
//输入：n = 3
//输出：[[1,2,3],[8,9,4],[7,6,5]]
// 
//
// 示例 2： 
//
// 
//输入：n = 1
//输出：[[1]]
// 
//
// 
//
// 提示： 
//
// 
// 1 <= n <= 20 
// 
// Related Topics 数组 
// 👍 324 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut x = 0;
        let mut y = 0;
        let mut dx = 1;
        let mut dy = 0;
        let mut min_x = 0;
        let mut max_x = n;
        let mut min_y = 0;
        let mut max_y = n;
        let mut result = vec![vec![0; n as usize]; n as usize];
        (1..=n * n).for_each(|i| {
            result[y as usize][x as usize] = i;
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
                dy = 0;
            }
            x += dx;
            y += dy;
        });
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

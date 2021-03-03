//二维区域和检索 - 矩阵不可变
//给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (row1, col1) ，右下角为 (row2, col2) 。 
//
// 
//上图子矩阵左上角 (row1, col1) = (2, 1) ，右下角(row2, col2) = (4, 3)，该子矩形内元素的总和为 8。 
//
// 
//
// 示例： 
//
// 
//给定 matrix = [
//  [3, 0, 1, 4, 2],
//  [5, 6, 3, 2, 1],
//  [1, 2, 0, 1, 5],
//  [4, 1, 0, 1, 7],
//  [1, 0, 3, 0, 5]
//]
//
//sumRegion(2, 1, 4, 3) -> 8
//sumRegion(1, 1, 2, 2) -> 11
//sumRegion(1, 2, 2, 4) -> 12
// 
//
// 
//
// 提示： 
//
// 
// 你可以假设矩阵不可变。 
// 会多次调用 sumRegion 方法。 
// 你可以假设 row1 ≤ row2 且 col1 ≤ col2 。 
// 
// Related Topics 动态规划 
// 👍 203 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
struct NumMatrix {
    sum_matrix: Vec<Vec<i32>>
}

impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sum_matrix: Vec<Vec<i32>> = Vec::new();
        for (row_index, row) in matrix.into_iter().enumerate() {
            let mut current_row = Vec::new();
            for (col_index, col) in row.into_iter().enumerate() {
                if row_index == 0 && col_index == 0{
                    current_row.push(col)
                }else if row_index == 0 {
                    current_row.push(current_row[col_index-1] + col)
                }else if col_index == 0 {
                    current_row.push(sum_matrix[row_index-1][col_index] + col)
                }else{
                    current_row.push(current_row[col_index-1] + sum_matrix[row_index-1][col_index] + col - sum_matrix[row_index-1][col_index-1])
                }
            }
            sum_matrix.push(current_row);
        }
        NumMatrix{
            sum_matrix
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        if row1 == 0 && col1 == 0 {
            return self.sum_matrix[row2 as usize][col2 as usize]
        }else if row1 == 0 {
            return self.sum_matrix[row2 as usize][col2 as usize]- self.sum_matrix[row2 as usize][col1 as usize-1]
        }else if col1 == 0 {
            return self.sum_matrix[row2 as usize][col2 as usize]- self.sum_matrix[row1 as usize - 1][col2 as usize]
        }
        self.sum_matrix[row2 as usize][col2 as usize] + self.sum_matrix[row1 as usize - 1][col1 as usize - 1] - self.sum_matrix[row2 as usize][col1 as usize - 1] - self.sum_matrix[row1 as usize -1][col2 as usize]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
//leetcode submit region end(Prohibit modification and deletion)

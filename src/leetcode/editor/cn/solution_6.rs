//ZigZag Conversion
//The string "PAYPALISHIRING" is written in a zigzag pattern on a given number o
//f rows like this: (you may want to display this pattern in a fixed font for bett
//er legibility) 
//
// 
//P   A   H   N
//A P L S I I G
//Y   I   R
// 
//
// And then read line by line: "PAHNAPLSIIGYIR" 
//
// Write the code that will take a string and make this conversion given a numbe
//r of rows: 
//
// 
//string convert(string s, int numRows); 
//
// Example 1: 
//
// 
//Input: s = "PAYPALISHIRING", numRows = 3
//Output: "PAHNAPLSIIGYIR"
// 
//
// Example 2: 
//
// 
//Input: s = "PAYPALISHIRING", numRows = 4
//Output: "PINALSIGYAHRPI"
//Explanation:
//
//P     I    N
//A   L S  I G
//Y A   H R
//P     I 
// Related Topics String


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num = num_rows as usize;
        let line_num = 2 * num - 2;
        let content = s.into_bytes();
        let length = content.len();
        let mut result: Vec<u8> = Vec::new();
        for index in 0..num {
            let mut offset = 0usize;
            while offset + index < length {
                result.push(content[offset + index]);
                if index != 0 && index != num - 1 && offset + line_num - index < length {
                    result.push(content[offset + line_num - index]);
                }
                offset += line_num;
            }
        }
        return String::from_utf8(result).unwrap();
    }
}
//leetcode submit region end(Prohibit modification and deletion)

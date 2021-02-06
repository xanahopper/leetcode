//Reverse Integer
//Given a 32-bit signed integer, reverse digits of an integer. 
//
// Example 1: 
//
// 
//Input: 123
//Output: 321
// 
//
// Example 2: 
//
// 
//Input: -123
//Output: -321
// 
//
// Example 3: 
//
// 
//Input: 120
//Output: 21
// 
//
// Note: 
//Assume we are dealing with an environment which could only store integers with
//in the 32-bit signed integer range: [−231, 231 − 1]. For the purpose of this pro
//blem, assume that your function returns 0 when the reversed integer overflows. 
// Related Topics Math


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 || x == i32::min_value() {
            return 0;
        }
        let mut num: i32 = x;
        let mut result: i32 = 0;
        while num.abs() > 0 {
            let (c, n) = if let Some(mul) = result.checked_mul(10) {
                (mul + (num % 10), num / 10)
            } else {
                (0i32, 0i32)
            };
            result = c;
            num = n;
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

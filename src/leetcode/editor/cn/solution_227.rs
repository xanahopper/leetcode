//给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。 
//
// 整数除法仅保留整数部分。 
//
// 
// 
// 
//
// 示例 1： 
//
// 
//输入：s = "3+2*2"
//输出：7
// 
//
// 示例 2： 
//
// 
//输入：s = " 3/2 "
//输出：1
// 
//
// 示例 3： 
//
// 
//输入：s = " 3+5 / 2 "
//输出：5
// 
//
// 
//
// 提示： 
//
// 
// 1 <= s.length <= 3 * 105 
// s 由整数和算符 ('+', '-', '*', '/') 组成，中间由一些空格隔开 
// s 表示一个 有效表达式 
// 表达式中的所有整数都是非负整数，且在范围 [0, 231 - 1] 内 
// 题目数据保证答案是一个 32-bit 整数 
// 
// 
// 
// Related Topics 栈 字符串 
// 👍 348 👎 0


use crate::leetcode::Solution;
//leetcode submit region begin(Prohibit modification and deletion)
macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut dict = std::collections::HashMap::new();
        $( dict.insert($key, $val); )*
        dict
    }};
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut nums = vec![];
        let mut ops = vec![];
        let mut num = None;
        let pri = map!(b'-' => 1, b'+' => 1, b'*' => 2, b'/' => 2, b'('=> 10, b')' => 0);

        for ch in s.bytes().chain(")".bytes()).filter(|x| *x != b' ') {
            if ch.is_ascii_digit() {
                let n = num.get_or_insert(0);
                *n = 10 * *n + (ch - b'0') as i32;
                continue;
            }
            num.take().map(|x| nums.push(x));
            while ops
                .last()
                .map_or(false, |x| *x != b'(' && pri[x] >= pri[&ch])
            {
                ops.pop().map(|x| Self::_calc(&mut nums, x));
            }

            if ch != b')' {
                ops.push(ch);
            } else {
                ops.pop();
            }
            // println!("{:?} {:?}", nums, String::from_utf8_lossy(&ops));
        }

        nums.pop().unwrap_or(0)
    }

    #[inline]
    fn _calc(nums: &mut Vec<i32>, op: u8) {
        let r = nums.pop().unwrap_or(0);
        let l = nums.pop().unwrap_or(0);
        nums.push(match op {
            b'+' => l + r,
            b'-' => l - r,
            b'*' => l * r,
            b'/' => l / r,
            _ => panic!(),
        });
    }
}
//leetcode submit region end(Prohibit modification and deletion)

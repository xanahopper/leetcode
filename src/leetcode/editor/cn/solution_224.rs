//给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。 
//
// 
//
// 示例 1： 
//
// 
//输入：s = "1 + 1"
//输出：2
// 
//
// 示例 2： 
//
// 
//输入：s = " 2-1 + 2 "
//输出：3
// 
//
// 示例 3： 
//
// 
//输入：s = "(1+(4+5+2)-3)+(6+8)"
//输出：23
// 
//
// 
//
// 提示： 
//
// 
// 1 <= s.length <= 3 * 105 
// s 由数字、'+'、'-'、'('、')'、和 ' ' 组成 
// s 表示一个有效的表达式 
// 
// Related Topics 栈 数学 
// 👍 485 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn calculate(s: String) -> i32 {
        #[derive(Copy, Clone)]
        enum Token {
            Number(i32),
            Add,
            Minus,
            Quote,
            End,
            Ignore
        }
        let mut stack = s.chars()
            .fold(Vec::<Token>::new(), |mut acc, x| {
                let token = match x {
                    '0'..='9' => Token::Number(x - b'0'),
                    '+' => Token::Add,
                    '-' => Token::Minus,
                    '(' => Token::Quote,
                    ')' => Token::End,
                    _ => Token::Ignore
                };
                if token != Token::Ignore {
                    acc.push(token)
                } else if token == Token::End {
                    let mut expr = vec![];

                }
                acc
            });
        if let Token::Number(x) = stack[0] {
            x
        } else {
            0
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

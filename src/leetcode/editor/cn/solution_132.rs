//给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文。 
//
// 返回符合要求的 最少分割次数 。 
//
// 
// 
// 
//
// 示例 1： 
//
// 
//输入：s = "aab"
//输出：1
//解释：只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。
// 
//
// 示例 2： 
//
// 
//输入：s = "a"
//输出：0
// 
//
// 示例 3： 
//
// 
//输入：s = "ab"
//输出：1
// 
//
// 
//
// 提示： 
//
// 
// 1 <= s.length <= 2000 
// s 仅由小写英文字母组成 
// 
// 
// 
// Related Topics 动态规划 
// 👍 371 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::min;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let size = s.len();
        let mut count = vec![vec![true; size]; size];

        // dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
        for i in (0..size).rev() {
            for j in i+1..size {
                count[i][j] = chars[i] == chars[j] && count[i + 1][j - 1];
            }
        }

        // res[i] = min(res[i], res[j] + 1) , j <= i ;
        let mut res = vec![0 as i32; size];
        for i in 0..size {
            res[i] = i as i32;
            for j in 0..=i {
                if count[j][i] {
                    if j == 0 {
                        res[i] = 0;
                    } else {
                        res[i] = min(res[i], res[j - 1] + 1);
                    }
                }
            }
        }
        return res[size - 1];
    }
}
//leetcode submit region end(Prohibit modification and deletion)

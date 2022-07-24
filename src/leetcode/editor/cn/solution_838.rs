//n 张多米诺骨牌排成一行，将每张多米诺骨牌垂直竖立。在开始时，同时把一些多米诺骨牌向左或向右推。 
//
// 每过一秒，倒向左边的多米诺骨牌会推动其左侧相邻的多米诺骨牌。同样地，倒向右边的多米诺骨牌也会推动竖立在其右侧的相邻多米诺骨牌。 
//
// 如果一张垂直竖立的多米诺骨牌的两侧同时有多米诺骨牌倒下时，由于受力平衡， 该骨牌仍然保持不变。 
//
// 就这个问题而言，我们会认为一张正在倒下的多米诺骨牌不会对其它正在倒下或已经倒下的多米诺骨牌施加额外的力。 
//
// 给你一个字符串 dominoes 表示这一行多米诺骨牌的初始状态，其中： 
//
// 
// dominoes[i] = 'L'，表示第 i 张多米诺骨牌被推向左侧， 
// dominoes[i] = 'R'，表示第 i 张多米诺骨牌被推向右侧， 
// dominoes[i] = '.'，表示没有推动第 i 张多米诺骨牌。 
// 
//
// 返回表示最终状态的字符串。 
// 
//
// 示例 1： 
//
// 
//输入：dominoes = "RR.L"
//输出："RR.L"
//解释：第一张多米诺骨牌没有给第二张施加额外的力。
// 
//
// 示例 2： 
//
// 
//输入：dominoes = ".L.R...LR..L.."
//输出："LL.RR.LLRRLL.."
// 
//
// 
//
// 提示： 
//
// 
// n == dominoes.length 
// 1 <= n <= 10⁵ 
// dominoes[i] 为 'L'、'R' 或 '.' 
// 
// Related Topics 双指针 字符串 动态规划 👍 234 👎 0



use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
struct Result {
    chars: Vec<char>,
    last_index: i32,
    last_char: char,
}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dom = dominoes.into_bytes();
        let mut start = 0;
        let mut lst_r = 0;
        let mut has_r = false;
        for i in 0..dom.len() {
            match dom[i] {
                b'L' => {
                    if has_r {
                        let mut x = lst_r;
                        let mut y = i;
                        while x < y {
                            dom[x] = b'R';
                            dom[y] = b'L';
                            x += 1;
                            y -= 1;
                        }
                        has_r = false;
                    } else {
                        for j in start..i {
                            dom[j] = b'L';
                        }
                    }
                    start = i + 1;
                },
                b'R' => {
                    if has_r {
                        for j in lst_r..i {
                            dom[j] = b'R';
                        }
                    }
                    has_r = true;
                    lst_r = i;
                },
                _ => ()
            }
        }
        if has_r {for i in lst_r..dom.len() {dom[i] = b'R'}}
        dom.into_iter().map(|c| c as char).collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

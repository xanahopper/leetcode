//给你一个二维整数数组 envelopes ，其中 envelopes[i] = [wi, hi] ，表示第 i 个信封的宽度和高度。 
//
// 当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。 
//
// 请计算 最多能有多少个 信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。 
//
// 注意：不允许旋转信封。 
// 
//
// 示例 1： 
//
// 
//输入：envelopes = [[5,4],[6,4],[6,7],[2,3]]
//输出：3
//解释：最多信封的个数为 3, 组合为: [2,3] => [5,4] => [6,7]。 
//
// 示例 2： 
//
// 
//输入：envelopes = [[1,1],[1,1],[1,1]]
//输出：1
// 
//
// 
//
// 提示： 
//
// 
// 1 <= envelopes.length <= 5000 
// envelopes[i].length == 2 
// 1 <= wi, hi <= 104 
// 
// Related Topics 二分查找 动态规划 
// 👍 446 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering;
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() == 0 {
            return 0;
        }

        let mut envelopes = envelopes;

        //先排序，按照 宽w进行排序。如果宽w相同，就不能互相套，所以要把高h大的放前面
        envelopes.sort_unstable_by(|a, b| {
            if a[0].cmp(&b[0]) == Ordering::Equal {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });


        //接下来转为求 高h 的最长递增子序列
        let mut hs = Vec::new();
        for e in &envelopes{
            let idx =  hs.binary_search(&e[1]).unwrap_or_else(|x|x);
            if idx == hs.len(){
                hs.push(e[1]);
            }else{
                hs[idx] = e[1];
            }

        }

        hs.len() as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

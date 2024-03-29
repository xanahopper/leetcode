//给你一个整数数组 nums 。如果 nums 的一个子集中，所有元素的乘积可以表示为一个或多个 互不相同的质数 的乘积，那么我们称它为 好子集 。 
//
// 
// 比方说，如果 nums = [1, 2, 3, 4] ：
//
// 
// [2, 3] ，[1, 2, 3] 和 [1, 3] 是 好 子集，乘积分别为 6 = 2*3 ，6 = 2*3 和 3 = 3 。 
// [1, 4] 和 [4] 不是 好 子集，因为乘积分别为 4 = 2*2 和 4 = 2*2 。 
// 
// 
// 
//
// 请你返回 nums 中不同的 好 子集的数目对 10⁹ + 7 取余 的结果。 
//
// nums 中的 子集 是通过删除 nums 中一些（可能一个都不删除，也可能全部都删除）元素后剩余元素组成的数组。如果两个子集删除的下标不同，那么它们被视
//为不同的子集。 
//
// 
//
// 示例 1： 
//
// 
//输入：nums = [1,2,3,4]
//输出：6
//解释：好子集为：
//- [1,2]：乘积为 2 ，可以表示为质数 2 的乘积。
//- [1,2,3]：乘积为 6 ，可以表示为互不相同的质数 2 和 3 的乘积。
//- [1,3]：乘积为 3 ，可以表示为质数 3 的乘积。
//- [2]：乘积为 2 ，可以表示为质数 2 的乘积。
//- [2,3]：乘积为 6 ，可以表示为互不相同的质数 2 和 3 的乘积。
//- [3]：乘积为 3 ，可以表示为质数 3 的乘积。
// 
//
// 示例 2： 
//
// 
//输入：nums = [4,2,3,15]
//输出：5
//解释：好子集为：
//- [2]：乘积为 2 ，可以表示为质数 2 的乘积。
//- [2,3]：乘积为 6 ，可以表示为互不相同质数 2 和 3 的乘积。
//- [2,15]：乘积为 30 ，可以表示为互不相同质数 2，3 和 5 的乘积。
//- [3]：乘积为 3 ，可以表示为质数 3 的乘积。
//- [15]：乘积为 15 ，可以表示为互不相同质数 3 和 5 的乘积。
// 
//
// 
//
// 提示： 
//
// 
// 1 <= nums.length <= 10⁵ 
// 1 <= nums[i] <= 30 
// 
// Related Topics 位运算 数组 数学 动态规划 状态压缩 👍 106 👎 0


use crate::leetcode::Solution;
/// 思路
/// num <= 30，即一个数字最多有 2 3 5 7 11 13 17 19 23 29 10 个质因子的可能，使用一个 u16 来表示
/// 先计算 1-30 的「好数字」，因为只有好数字的组成的集合才能是「好子集」
/// F(n)(m) 前 n 个数字中，使用了质数状态为 m 的「好子集」总数的余数
/// good(m)
/// F(n)(m) = (m & good(n) == 0 ? F(n-1)(m & ^good(n)) : 0) + F(n-1)(m)

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const STATES: [usize; 31] = [
            0, 0, 1, 2, 0, 4, 3, 8, 0, 0, 5, 16, 0, 32, 9, 6, 0, 64, 0, 128, 0, 10, 17, 256, 0, 0,
            33, 0, 0, 512, 7,
        ];

        let mut dp = vec![0; 1024];
        dp[0] = 1;

        let mut freq = vec![0i64; STATES.len()];
        nums.into_iter().for_each(|n| {
            if n == 1 {
                dp[0] = (dp[0] << 1) % MOD;
            } else if STATES[n as usize] > 0 {
                freq[n as usize] += 1;
            }
        });

        freq.into_iter()
            .enumerate()
            .filter_map(|(i, c)| if c > 0 { Some((STATES[i], c)) } else { None })
            .for_each(|(sta, cnt)| {
                (sta..1024)
                    .rev()
                    .filter(|s| (s & sta) == sta)
                    .for_each(|s| {
                        dp[s] = (dp[s] + dp[s ^ sta] * cnt % MOD) % MOD;
                    });
            });

        // if sum == 0, all nums are 1, which is invalid set.
        dp[1..].iter().fold(0, |res, v| (res + v) % MOD) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

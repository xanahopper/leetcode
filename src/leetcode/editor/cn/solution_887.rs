//你将获得 K 个鸡蛋，并可以使用一栋从 1 到 N 共有 N 层楼的建筑。 
//
// 每个蛋的功能都是一样的，如果一个蛋碎了，你就不能再把它掉下去。 
//
// 你知道存在楼层 F ，满足 0 <= F <= N 任何从高于 F 的楼层落下的鸡蛋都会碎，从 F 楼层或比它低的楼层落下的鸡蛋都不会破。 
//
// 每次移动，你可以取一个鸡蛋（如果你有完整的鸡蛋）并把它从任一楼层 X 扔下（满足 1 <= X <= N）。 
//
// 你的目标是确切地知道 F 的值是多少。 
//
// 无论 F 的初始值如何，你确定 F 的值的最小移动次数是多少？ 
//
// 
//
// 
// 
//
// 示例 1： 
//
// 输入：K = 1, N = 2
//输出：2
//解释：
//鸡蛋从 1 楼掉落。如果它碎了，我们肯定知道 F = 0 。
//否则，鸡蛋从 2 楼掉落。如果它碎了，我们肯定知道 F = 1 。
//如果它没碎，那么我们肯定知道 F = 2 。
//因此，在最坏的情况下我们需要移动 2 次以确定 F 是多少。
// 
//
// 示例 2： 
//
// 输入：K = 2, N = 6
//输出：3
// 
//
// 示例 3： 
//
// 输入：K = 3, N = 14
//输出：4
// 
//
// 
//
// 提示： 
//
// 
// 1 <= K <= 100 
// 1 <= N <= 10000 
// 
// Related Topics 数学 二分查找 动态规划 
// 👍 561 👎 0

use crate::leetcode::Solution;

/// f(K, K) 是 K 个鸡蛋 N 层楼所需要的最少实验次数，所以
/// f(K, N) = min{ f(K - 1, x), f(K, N-x) } 1 <= x <= N

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::{min, max};
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut f = vec![vec![(n + 1) as usize; (n + 1) as usize]; (k + 1) as usize];
        Solution::search_result(&mut f, k as usize, n as usize) as i32
    }

    fn search_result(f: &mut Vec<Vec<usize>>, k: usize, n: usize) -> usize {
        if k == 1 || n == 1 || n == 0 {
            n
        } else {
            let mut min_f = n;
            for x in 1..=n {
                f[k - 1][x - 1] = if f[k - 1][x - 1] == n {
                    min(f[k - 1][x - 1] - 1, Solution::search_result(f,k - 1, x - 1))
                } else {
                    f[k - 1][x - 1]
                };
                f[k][n - x] = if f[k][n - x] == n {
                    min(f[k][n - x] - 1, Solution::search_result(f, k, n - x))
                } else {
                    f[k][n - x]
                };
                min_f = min(min_f, 1 + max(f[k - 1][x - 1], f[k][n - x]));
            }
            min_f
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

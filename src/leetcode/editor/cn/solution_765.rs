//N 对情侣坐在连续排列的 2N 个座位上，想要牵到对方的手。 计算最少交换座位的次数，以便每对情侣可以并肩坐在一起。 一次交换可选择任意两人，让他们站起来交
//换座位。 
//
// 人和座位用 0 到 2N-1 的整数表示，情侣们按顺序编号，第一对是 (0, 1)，第二对是 (2, 3)，以此类推，最后一对是 (2N-2, 2N-1)
//。 
//
// 这些情侣的初始座位 row[i] 是由最初始坐在第 i 个座位上的人决定的。 
//
// 示例 1: 
//
// 
//输入: row = [0, 2, 1, 3]
//输出: 1
//解释: 我们只需要交换row[1]和row[2]的位置即可。
// 
//
// 示例 2: 
//
// 
//输入: row = [3, 2, 0, 1]
//输出: 0
//解释: 无需交换座位，所有的情侣都已经可以手牵手了。
// 
//
// 说明: 
//
// 
// len(row) 是偶数且数值在 [4, 60]范围内。 
// 可以保证row 是序列 0...len(row)-1 的一个全排列。 
// 
// Related Topics 贪心算法 并查集 图 
// 👍 240 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let mut len: i32 = row.len() as i32;
        let pair: i32 = len / 2;
        let parent: Vec<i32> = (0..pair).map(|idx| idx).collect();
        let mut uf = UnionFind::new(pair, parent);
        for i in (0..len).filter(|x| x % 2 == 0) {
            uf.union(row[i as usize] / 2, row[(i + 1) as usize] / 2);
        }
        pair - uf.count
    }
}

struct UnionFind {
    parent: Vec<i32>,
    pub count: i32,
}

impl UnionFind {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        UnionFind {
            count: n,
            parent,
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        if x == self.parent[x as usize] {
            return x
        }
        self.parent[x as usize] = self.find(self.parent[x as usize]);
        self.parent[x as usize]
    }
    fn union(&mut self, x: i32, y: i32) {
        let mut fx = self.find(x) as usize;
        let mut fy = self.find(y) as usize;
        if fx != fy {
            self.parent[fx] = fy as i32;
            self.count -= 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

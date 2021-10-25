//不使用任何内建的哈希表库设计一个哈希集合（HashSet）。 
//
// 实现 MyHashSet 类： 
//
// 
// void add(key) 向哈希集合中插入值 key 。 
// bool contains(key) 返回哈希集合中是否存在这个值 key 。 
// void remove(key) 将给定值 key 从哈希集合中删除。如果哈希集合中没有这个值，什么也不做。 
// 
// 
//
// 示例： 
//
// 
//输入：
//["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove
//", "contains"]
//[[], [1], [2], [1], [3], [2], [2], [2], [2]]
//输出：
//[null, null, null, true, false, null, true, null, false]
//
//解释：
//MyHashSet myHashSet = new MyHashSet();
//myHashSet.add(1);      // set = [1]
//myHashSet.add(2);      // set = [1, 2]
//myHashSet.contains(1); // 返回 True
//myHashSet.contains(3); // 返回 False ，（未找到）
//myHashSet.add(2);      // set = [1, 2]
//myHashSet.contains(2); // 返回 True
//myHashSet.remove(2);   // set = [1]
//myHashSet.contains(2); // 返回 False ，（已移除） 
//
// 
//
// 提示： 
//
// 
// 0 <= key <= 10^6
// 最多调用 10^4 次 add、remove 和 contains 。
// 
//
// 
//
// 进阶：你可以不使用内建的哈希集合库解决此问题吗？ 
// Related Topics 设计 哈希表 
// 👍 107 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
struct MyHashSet {
    data: Vec<u32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            data: vec![0; 500_000]
        }
    }
    
    fn add(&mut self, key: i32) {
        self.data[MyHashSet::hash(key)] |= 1 << (key % 20);
    }
    
    fn remove(&mut self, key: i32) {
        self.data[MyHashSet::hash(key)] &= !(1 << (key % 20));
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.data[MyHashSet::hash(key)] & (1 << (key % 20)) != 0
    }

    fn hash(key: i32) -> usize {
        key as usize / 20
    }
}
//leetcode submit region end(Prohibit modification and deletion)

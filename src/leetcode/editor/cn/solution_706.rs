//不使用任何内建的哈希表库设计一个哈希映射（HashMap）。 
//
// 实现 MyHashMap 类： 
//
// 
// MyHashMap() 用空映射初始化对象 
// void put(int key, int value) 向 HashMap 插入一个键值对 (key, value) 。如果 key 已经存在于映射中，
//则更新其对应的值 value 。 
// int get(int key) 返回特定的 key 所映射的 value ；如果映射中不包含 key 的映射，返回 -1 。 
// void remove(key) 如果映射中存在 key 的映射，则移除 key 和它所对应的 value 。 
// 
//
// 
//
// 示例： 
//
// 
//输入：
//["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
//[[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
//输出：
//[null, null, null, 1, -1, null, 1, null, -1]
//
//解释：
//MyHashMap myHashMap = new MyHashMap();
//myHashMap.put(1, 1); // myHashMap 现在为 [[1,1]]
//myHashMap.put(2, 2); // myHashMap 现在为 [[1,1], [2,2]]
//myHashMap.get(1);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,2]]
//myHashMap.get(3);    // 返回 -1（未找到），myHashMap 现在为 [[1,1], [2,2]]
//myHashMap.put(2, 1); // myHashMap 现在为 [[1,1], [2,1]]（更新已有的值）
//myHashMap.get(2);    // 返回 1 ，myHashMap 现在为 [[1,1], [2,1]]
//myHashMap.remove(2); // 删除键为 2 的数据，myHashMap 现在为 [[1,1]]
//myHashMap.get(2);    // 返回 -1（未找到），myHashMap 现在为 [[1,1]]
// 
//
// 
//
// 提示： 
//
// 
// 0 <= key, value <= 106 
// 最多调用 104 次 put、get 和 remove 方法 
// 
//
// 
//
// 进阶：你能否不使用内置的 HashMap 库解决此问题？ 
// Related Topics 设计 哈希表 
// 👍 161 👎 0


//leetcode submit region begin(Prohibit modification and deletion)

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
pub struct MyHashMap {
    hasher_builder: RandomState,
    buckets: Vec<Vec<(i32, i32)>>,
    capacity: usize,
    size: usize
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            hasher_builder: RandomState::new(),
            buckets: vec![vec![]; 56],
            capacity: 56,
            size: 0
        }
    }
    
    /** value will always be non-negative. */
    pub fn put(&mut self, key: i32, value: i32) {
        let i = self.hash(key);
        if self.size == self.capacity >> 1 {
            self.buckets.resize(self.capacity * 2, vec![]);
        }

        if let Some(bucket) = self.buckets.get_mut(i % self.capacity) {
            if let Some((k, v)) = bucket.iter_mut().find(|&&mut (k, v)| k == key) {
                *v = value;
            } else {
                bucket.push((key, value));
            }
        }
        self.size += 1;
    }
    
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    pub fn get(&self, key: i32) -> i32 {
        let i = self.hash(key);

        if let Some(bucket) = self.buckets.get(i % self.capacity) {
            bucket.iter().find(|&(k, _)| *k == key).unwrap_or(&(-1, -1)).1
        } else {
            -1
        }
    }

    fn hash(&self, key: i32) -> usize {
        let mut hasher = self.hasher_builder.build_hasher();
        hasher.write_i32(key);
        hasher.finish() as usize
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    pub fn remove(&mut self, key: i32) {
        let i = self.hash(key);
        self.size -= 1;
        if let Some(bucket) = self.buckets.get_mut(i % self.capacity) {
            bucket.retain(|(k, _)| *k != key);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

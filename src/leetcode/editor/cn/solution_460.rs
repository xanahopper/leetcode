//请你为 最不经常使用（LFU）缓存算法设计并实现数据结构。 
//
// 实现 LFUCache 类： 
//
// 
// LFUCache(int capacity) - 用数据结构的容量 capacity 初始化对象 
// int get(int key) - 如果键存在于缓存中，则获取键的值，否则返回 -1。 
// void put(int key, int value) - 如果键已存在，则变更其值；如果键不存在，请插入键值对。当缓存达到其容量时，则应该在插入新项之
//前，使最不经常使用的项无效。在此问题中，当存在平局（即两个或更多个键具有相同使用频率）时，应该去除 最久未使用 的键。 
// 
//
// 注意「项的使用次数」就是自插入该项以来对其调用 get 和 put 函数的次数之和。使用次数会在对应项被移除后置为 0 。 
//
// 为了确定最不常使用的键，可以为缓存中的每个键维护一个 使用计数器 。使用计数最小的键是最久未使用的键。 
//
// 当一个键首次插入到缓存中时，它的使用计数器被设置为 1 (由于 put 操作)。对缓存中的键执行 get 或 put 操作，使用计数器的值将会递增。 
//
// 
//
// 示例： 
//
// 
//输入：
//["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "g
//et"]
//[[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
//输出：
//[null, null, null, 1, null, -1, 3, null, -1, 3, 4]
//
//解释：
//// cnt(x) = 键 x 的使用计数
//// cache=[] 将显示最后一次使用的顺序（最左边的元素是最近的）
//LFUCache lFUCache = new LFUCache(2);
//lFUCache.put(1, 1);   // cache=[1,_], cnt(1)=1
//lFUCache.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
//lFUCache.get(1);      // 返回 1
//                      // cache=[1,2], cnt(2)=1, cnt(1)=2
//lFUCache.put(3, 3);   // 去除键 2 ，因为 cnt(2)=1 ，使用计数最小
//                      // cache=[3,1], cnt(3)=1, cnt(1)=2
//lFUCache.get(2);      // 返回 -1（未找到）
//lFUCache.get(3);      // 返回 3
//                      // cache=[3,1], cnt(3)=2, cnt(1)=2
//lFUCache.put(4, 4);   // 去除键 1 ，1 和 3 的 cnt 相同，但 1 最久未使用
//                      // cache=[4,3], cnt(4)=1, cnt(3)=2
//lFUCache.get(1);      // 返回 -1（未找到）
//lFUCache.get(3);      // 返回 3
//                      // cache=[3,4], cnt(4)=1, cnt(3)=3
//lFUCache.get(4);      // 返回 4
//                      // cache=[3,4], cnt(4)=2, cnt(3)=3 
//
// 
//
// 提示： 
//
// 
// 0 <= capacity, key, value <= 104 
// 最多调用 105 次 get 和 put 方法 
// 
//
// 
//
// 进阶：你可以为这两种操作设计时间复杂度为 O(1) 的实现吗？ 
// Related Topics 设计 
// 👍 333 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{HashMap, BTreeSet};
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    key: i32,
    value: i32,
    freq: i32,
    id: i32
}

pub struct LFUCache {
    data: HashMap<i32, Node>,
    capacity: usize,
    order_set: BTreeSet<Node>,
    primary_index: i32
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Ordering::Equal => self.id.cmp(&other.id),
            _o => _o,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn new(id: i32, key: i32, val: i32) -> Self {
        Node {
            key,
            value: val,
            freq: 1,
            id
        }
    }
}

impl LFUCache {

    pub fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            data: HashMap::new(),
            order_set: Default::default(),
            primary_index: 1
        }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        match self.data.get_mut(&key) {
            Some(t) => {
                self.order_set.remove(t);
                self.primary_index += 1;
                t.freq += 1;
                t.id = self.primary_index;
                self.order_set.insert(*t);
                t.value
            }
            None => -1
        }
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        match self.data.get_mut(&key) {
            Some(t) => {
                self.order_set.remove(t);
                self.primary_index += 1;
                t.freq += 1;
                t.id = self.primary_index;
                t.value = value;
                self.order_set.insert(*t);
            }
            None => {
                self.primary_index += 1;
                if self.capacity == self.data.len() {
                    let remove_node = *self.order_set.iter().next().unwrap();
                    println!("remove node {}: {}, freq: {}", &remove_node.key, &remove_node.value, &remove_node.freq);
                    self.data.remove(&remove_node.key);
                    self.order_set.remove(&remove_node);
                }
                let node = Node::new(self.primary_index, key, value);
                self.order_set.insert(node);
                self.data.insert(key, node);
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

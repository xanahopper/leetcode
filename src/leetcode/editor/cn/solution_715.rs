//Range模块是跟踪数字范围的模块。设计一个数据结构来跟踪表示为 半开区间 的范围并查询它们。 
//
// 半开区间 [left, right) 表示所有 left <= x < right 的实数 x 。 
//
// 实现 RangeModule 类: 
//
// 
// RangeModule() 初始化数据结构的对象。 
// void addRange(int left, int right) 添加 半开区间 [left, right)，跟踪该区间中的每个实数。添加与当前跟踪的
//数字部分重叠的区间时，应当添加在区间 [left, right) 中尚未跟踪的任何数字到该区间中。
// boolean queryRange(int left, int right) 只有在当前正在跟踪区间 [left, right) 中的每一个实数时，才返
//回 true ，否则返回 false 。 
// void removeRange(int left, int right) 停止跟踪 半开区间 [left, right) 中当前正在跟踪的每个实数。 
// 
//
// 
//
// 示例 1： 
//
// 
//输入
//["RangeModule", "addRange", "removeRange", "queryRange", "queryRange", 
//"queryRange"]
//[[], [10, 20], [14, 16], [10, 14], [13, 15], [16, 17]]
//输出
//[null, null, null, true, false, true]
//
//解释
//RangeModule rangeModule = new RangeModule();
//rangeModule.addRange(10, 20);
//rangeModule.removeRange(14, 16);
//rangeModule.queryRange(10, 14); 返回 true （区间 [10, 14) 中的每个数都正在被跟踪）
//rangeModule.queryRange(13, 15); 返回 false（未跟踪区间 [13, 15) 中像 14, 14.03, 14.17 这样
//的数字）
//rangeModule.queryRange(16, 17); 返回 true （尽管执行了删除操作，区间 [16, 17) 中的数字 16 仍然会被跟踪）
//
// 
//
// 
//
// 提示： 
//
// 
// 1 <= left < right <= 10⁹ 
// 在单个测试用例中，对 addRange 、 queryRange 和 removeRange 的调用总数不超过 10⁴ 次 
// 
// Related Topics 设计 线段树 有序集合 👍 162 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct RangeNode {
    range: Range<i32>,
    track: bool,
    add: bool,
    left: Option<Rc<RefCell<RangeNode>>>,
    right: Option<Rc<RefCell<RangeNode>>>
}

struct RangeModule {
    root: Option<Rc<RefCell<RangeNode>>>
}

pub trait RangeOps<T> {
    fn mid(&self) -> T;
}

impl RangeOps<i32> for Range<i32> {
    fn mid(&self) -> i32 {
        self.start + (self.end - self.start) / 2
    }
}

impl RangeNode {
    fn split(&mut self) {
        let mid = self.range.mid();
        if self.left.is_none() {
            self.left = Some(Rc::new(RefCell::new(RangeNode {
                range: self.range.start..mid,
                track: self.track,
                add: true,
                left: None,
                right: None
            })));
        } else if self.add {
            self.left.as_ref().unwrap().as_ref().borrow_mut().track = self.track;
        }
        if self.right.is_none() {
            self.right = Some(Rc::new(RefCell::new(RangeNode {
                range: mid + 1..self.range.end,
                track: self.track,
                add: true,
                left: None,
                right: None
            })));
        } else if self.add {
            self.right.as_ref().unwrap().as_ref().borrow_mut().track = self.track;
        }
        if !self.add {
            return;
        }
        self.add = false;
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {

    fn new() -> Self {
        RangeModule {
            root: Some(Rc::new(RefCell::new(RangeNode {
                range: 1..1_000_000_000,
                track: false,
                add: false,
                left: None,
                right: None
            })))
        }
    }
    
    fn add_range(&mut self, left: i32, right: i32) {
        RangeModule::range_update(&mut self.root, &(left..right - 1), true);
    }
    
    fn query_range(&mut self, left: i32, right: i32) -> bool {
        RangeModule::query_range_node(&self.root, &(left..right - 1)).unwrap()
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        RangeModule::range_update(&mut self.root, &(left..right - 1), false);
    }

    fn query_range_node(node: &Option<Rc<RefCell<RangeNode>>>, query_range: &Range<i32>) -> Option<bool> {
        if let Some(v) = node {
            let mut node = v.as_ref().borrow_mut();
            if query_range.start <= node.range.start && node.range.end <= query_range.end {
                Some(node.track)
            } else {
                node.split();
                let mid = node.range.mid();
                let left_track = if query_range.start <= mid {
                    RangeModule::query_range_node(&node.left, query_range)
                } else { Some(true) };
                let right_track = if query_range.end > mid {
                    RangeModule::query_range_node(&node.right, query_range)
                } else { Some(true) };
                Some(left_track.unwrap() && right_track.unwrap())
            }
        } else {
            None
        }
    }

    fn range_update(node: &Option<Rc<RefCell<RangeNode>>>, update_range: &Range<i32>, track: bool) {
        if let Some(v) = node {
            let mut node = v.as_ref().borrow_mut();
            if update_range.start > node.range.end || update_range.end < node.range.start {
                return;
            }
            if update_range.start <= node.range.start && update_range.end >= node.range.end {
                node.track = track;
                node.add = true;
                return;
            }
            let mid = node.range.mid();
            node.split();
            if update_range.start <= mid {
                RangeModule::range_update(&node.left, update_range, track)
            }
            if mid < update_range.end {
                RangeModule::range_update(&node.right, update_range, track)
            }
            let mut t = node.track;
            if let Some(lt) = &node.left {
                if let Some(rt) = &node.right {
                    let left_track = lt.as_ref().borrow();
                    let right_track = rt.as_ref().borrow();
                    t = left_track.track && right_track.track;
                }
            }
            node.track = t;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_715_test() {
    let mut range_module = RangeModule::new();
    range_module.add_range(10, 20);
    range_module.remove_range(14, 16);
    assert_eq!(range_module.query_range(10, 14), true);
    assert_eq!(range_module.query_range(13, 15), false);
    assert_eq!(range_module.query_range(16, 17), true);

    let mut r2 = RangeModule::new();
    r2.add_range(6, 8);
    r2.remove_range(7, 8);
    r2.remove_range(8, 9);
    r2.add_range(8, 9);
    r2.remove_range(1, 3);
    r2.add_range(1, 8);
    assert_eq!(r2.query_range(2, 4), true);
    assert_eq!(r2.query_range(2, 9), true);
    assert_eq!(r2.query_range(4, 6), true);

    // :["RangeModule","removeRange","addRange","queryRange","queryRange","queryRange","addRange","addRange","removeRange","removeRange"] [[],[7,8],[4,10],[7,8],[5,7],[1,8],[2,6],[4,6],[3,4],[5,7]] Output:[null,null,null,false,true,false,null,null,null,null] Expected:[null,null,null,true,true,false,null,null,null,null] stdout:
}

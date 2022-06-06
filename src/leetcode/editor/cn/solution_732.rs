////当 k 个日程安排有一些时间上的交叉时（例如 k 个日程安排都在同一时间内），就会产生 k 次预订。 
////
//// 给你一些日程安排 [start, end) ，请你在每个日程安排添加后，返回一个整数 k ，表示所有先前日程安排会产生的最大 k 次预订。 
////
//// 实现一个 MyCalendarThree 类来存放你的日程安排，你可以一直添加新的日程安排。 
////
//// 
//// MyCalendarThree() 初始化对象。 
//// int book(int start, int end) 返回一个整数 k ，表示日历中存在的 k 次预订的最大值。 
//// 
////
//// 
////
//// 示例： 
////
//// 
////输入：
////["MyCalendarThree", "book", "book", "book", "book", "book", "book"]
////[[], [10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]]
////输出：
////[null, 1, 1, 2, 3, 3, 3]
////
////解释：
////MyCalendarThree myCalendarThree = new MyCalendarThree();
////myCalendarThree.book(10, 20); // 返回 1 ，第一个日程安排可以预订并且不存在相交，所以最大 k 次预订是 1 次预订。
//
////myCalendarThree.book(50, 60); // 返回 1 ，第二个日程安排可以预订并且不存在相交，所以最大 k 次预订是 1 次预订。
//
////myCalendarThree.book(10, 40); // 返回 2 ，第三个日程安排 [10, 40) 与第一个日程安排相交，所以最大 k 次预
//订是
//// 2 次预订。
////myCalendarThree.book(5, 15); // 返回 3 ，剩下的日程安排的最大 k 次预订是 3 次预订。
////myCalendarThree.book(5, 10); // 返回 3
////myCalendarThree.book(25, 55); // 返回 3
//// 
////
//// 
////
//// 提示： 
////
//// 
//// 0 <= start < end <= 10⁹ 
//// 每个测试用例，调用 book 函数最多不超过 400次 
//// 
//// Related Topics 设计 线段树 有序集合 👍 157 👎 0
//

#[test]
fn solution_732_test() {
    let mut book = MyCalendarThree::new();
    assert_eq!(book.book(10, 20), 1);
    assert_eq!(book.book(50, 60), 1);
    assert_eq!(book.book(10, 40), 2);
    assert_eq!(book.book(5, 15), 3);
    assert_eq!(book.book(5, 10), 3);
    assert_eq!(book.book(25, 66), 3);
}


//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;
use std::collections::HashMap;

struct BookNode {
    book_count: i32,
    max_book_count: i32
}

impl Default for BookNode {
    fn default() -> Self {
        BookNode {
            book_count: 0,
            max_book_count: 0
        }
    }
}

struct MyCalendarThree {
    data: HashMap<i32, BookNode>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        MyCalendarThree {
            data: HashMap::new()
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.update(start, end - 1, 0, 1_000_000_000, 1);
        self.data.get(&1).unwrap_or(&BookNode::default()).max_book_count
    }

    fn update(&mut self, start: i32, end: i32, l: i32, r: i32, index: i32) {
        if start > r || end < l {
            return;
        }
        if start <= l && end >= r {
            let entry = self.data.entry(index).or_insert(BookNode::default());
            entry.book_count += 1;
            entry.max_book_count += 1;
        } else {
            let mid = (l + r) / 2;
            self.update(start, end, l, mid, index * 2);
            self.update(start, end, mid + 1, r, index * 2 + 1);
            let left = self.data.get(&(index * 2)).map_or(0, |x| x.max_book_count);
            let right = self.data.get(&(index * 2 + 1)).map_or(0, |x| x.max_book_count);
            let entry = self.data.entry(index).or_insert(BookNode::default());
            entry.max_book_count = entry.book_count + max(left, right);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

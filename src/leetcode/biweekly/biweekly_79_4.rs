use std::cmp::min;
use std::ops::Range;
use crate::algorithm::segment_tree::{Operation, RangeOps, SegmentTree};

struct BookMyShow {
    book_tree: SegmentTree<BookInfo>,
    n: i32,
    m: i32
}

#[derive(Debug, Copy, Clone)]
struct BookInfo {
    min: i32,
    sum: i32
}

impl Operation<BookInfo> for BookInfo {
    fn merge(&self, a: &Self, b: &Self) -> Self {
        BookInfo { min: min(self.min, b.min), sum: self.sum + b.sum }
    }
}

impl Default for BookInfo {
    fn default() -> Self {
        BookInfo { min: 0, sum: 0 }
    }
}

trait BookOperation {
    fn index(&self, root: usize, range: Range<usize>, right: usize, val: i32) -> i32;

    fn query_sum(&self, query_range: Range<usize>) -> i32;

    fn add(&mut self, root: usize, range: Range<usize>, index: usize, val: i32);
}

impl BookOperation for SegmentTree<BookInfo> {
    fn index(&self, root: usize, range: Range<usize>, right: usize, val: i32) -> i32 {
        if let Some(r) = &self.tree[root] {
            if r.min > val {
                return 0;
            }
        }
        if range.len() == 0 {
            return 0;
        }
        let mid = range.mid();
        let left_index = SegmentTree::<BookInfo>::left_child(root);
        let right_index = SegmentTree::<BookInfo>::right_child(root);
        if let Some(r) = &self.tree[left_index] {
            if r.min <= val {
                return self.index(left_index, range.start..mid, right, val);
            }
        }
        if mid < right {
            return self.index(right_index, mid + 1..range.end, right_index, val)
        }
        -1
    }

    fn query_sum(&self, query_range: Range<usize>) -> i32 {
        self.query(query_range).unwrap_or(BookInfo::default()).sum
    }

    fn add(&mut self, root: usize, range: Range<usize>, index: usize, val: i32) {
        if range.len() == 0 {
            if let Some(mut r) = self.tree[root] {
                r.min += val;
                r.sum += val;
                return;
            }
        }
        let mid = range.mid();
        let left_index = SegmentTree::<BookInfo>::left_child(root);
        let right_index = SegmentTree::<BookInfo>::right_child(root);
        if index <= mid {
            self.add(left_index, range.start..mid, index, val);
        } else {
            self.add(right_index, mid + 1..range.end, index, val);
        }
        self.tree[root] = self.tree[left_index].merge(&self.tree[left_index], &self.tree[right_index]);
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {

    fn new(n: i32, m: i32) -> Self {
        BookMyShow {
            book_tree: SegmentTree::<BookInfo>::new((0..n).map(|_| BookInfo::default()).collect()),
            n,
            m
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let i = self.book_tree.index(0, 0..self.book_tree.len - 1, (max_row + 1) as usize, self.m - k);
        if i < 0 {
            vec![]
        } else {
            let seats = self.book_tree.query_sum(i as usize..i as usize);
            self.book_tree.add(0, 0..self.book_tree.len - 1, i as usize, k);
            return vec![i, seats]
        }
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {

        false
    }
}
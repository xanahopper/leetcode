use std::ops::Range;

pub trait Operation<T : Copy> {
    fn merge(&self, b: &Self) -> Self;
}

#[derive(Debug)]
pub struct SegmentTree<T> {
    data: Vec<T>,
    tree: Vec<Option<T>>,
    len: usize
}

trait RangeOps<T> {
    fn mid(&self) -> T;
}

impl RangeOps<usize> for Range<usize> {
    fn mid(&self) -> usize {
        self.start + self.len() / 2
    }
}

impl<T : Copy> Operation<T> for Option<T> where T : Operation<T> {
    fn merge(&self, b: &Self) -> Self {
        self.and_then(|a| b.and_then(|b| Some(a.merge(&b))))
    }
}

impl<T> SegmentTree<T> where T : Copy + Operation<T> + Default {
    fn build(&mut self, root: usize, range: Range<usize>) {
        if range.len() == 0 {
            self.tree[root] = Some(self.data[range.start]);
            return;
        }
        let left_index = SegmentTree::<T>::left_child(root);
        let right_index = SegmentTree::<T>::right_child(root);
        let mid = range.mid();
        self.build(left_index, range.start..mid);
        self.build(right_index, mid + 1..range.end);
        self.tree[root] = self.tree[left_index].or(Some(T::default())).merge(&self.tree[right_index].or(Some(T::default())));
    }

    fn left_child(root: usize) -> usize {
        root * 2 + 1
    }

    fn right_child(root: usize) -> usize {
        SegmentTree::<T>::left_child(root) + 1
    }

    pub fn get(&self, index: usize) -> Option<T> {
        Some(self.data[index])
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new(data: Vec<T>) -> Self {
        let len = data.len();
        let mut tree: Vec<Option<T>> = vec![None; len * 4];
        let mut t = Self {
            data,
            tree,
            len
        };
        t.build(0, 0..len - 1);
        t
    }

    pub fn query(&self, range: Range<usize>) -> Option<T> {
        self.recursion_query(0, 0..self.len() - 1, range)
    }

    fn recursion_query(&self, root: usize, range: Range<usize>, query_range: Range<usize>) -> Option<T> {
        if range == query_range {
            return self.tree[root].or(Some(T::default()));
        }
        let mid = range.mid();
        let left_index = SegmentTree::<T>::left_child(root);
        let right_index = SegmentTree::<T>::right_child(root);
        if query_range.start > mid {
            return self.recursion_query(right_index, mid + 1..range.end, query_range);
        } else if query_range.end <= mid {
            return self.recursion_query(left_index, range.start..mid, query_range);
        }
        let left_res = self.recursion_query(left_index, range.start..mid, query_range.start..mid);
        let right_res = self.recursion_query(right_index, mid + 1..range.end, mid + 1..query_range.end);

        left_res.merge(&right_res)
    }

    pub fn set(&mut self, index: usize, e: T) {
        if index >= self.len {
            return;
        }
        self.tree[index] = Some(e);
        self.recursion_set(0, 0..self.len() - 1, index, e);
    }

    fn recursion_set(&mut self, root_index: usize, range: Range<usize>, index: usize, e: T) {
        if range.len() == 0 {
            self.tree[root_index] = Some(e);
            return;
        }
        let mid = range.mid();
        let left_index = SegmentTree::<T>::left_child(root_index);
        let right_index = SegmentTree::<T>::right_child(root_index);
        if index > mid {
            self.recursion_set(right_index, mid + 1..range.end, index, e)
        } else {
            self.recursion_set(left_index, range.start..mid, index, e)
        }
        self.tree[root_index] = self.tree[left_index].merge(&self.tree[right_index]);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Ticket {
    cost: i32,
}

impl Operation<Ticket> for Ticket {
    fn merge(&self, b: &Self) -> Self {
        Ticket { cost: self.cost + b.cost }
    }
}

impl Default for Ticket {
    fn default() -> Self {
        Ticket { cost: 0 }
    }
}

#[test]
fn it_works() {
    let data = vec![Ticket { cost: 2 }, Ticket { cost: 0 }, Ticket { cost: -3 }, Ticket { cost: 55 }, Ticket { cost: 2 }, Ticket { cost: -1 }];
    let mut tree = SegmentTree::new(data);
    assert_eq!(tree.query(0..2), Some(Ticket { cost: -1 }));
    assert_eq!(tree.query(2..5), Some(Ticket { cost: 53 }));
    assert_eq!(tree.query(0..5), Some(Ticket { cost: 55 }));
    tree.set(1, Ticket { cost: 1});
    assert_eq!(tree.query(0..2), Some(Ticket { cost: 0 }));
}
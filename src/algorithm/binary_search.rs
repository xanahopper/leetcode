use std::cmp::Ordering;

pub trait LowerBoundBinarySearch<T> {

    fn lower_bound(&self, val: &T) -> usize;

    fn lower_bound_by<Prediction>(&self, p: Prediction) -> usize
        where Prediction: Fn(&T) -> Ordering;

}

pub trait UpperBoundBinarySearch<T> {
    fn upper_bound(&self, val: &T) -> usize;

    fn upper_bound_by<Prediction>(&self, p: Prediction) -> usize
        where Prediction: Fn(&T) -> Ordering;
}

pub trait LowerBoundExcludeBinarySearch<T> {
    fn lower_bound_exclude(&self, val: &T) -> usize;

    fn lower_bound_exclude_by<Prediction>(&self, p: Prediction) -> usize
        where Prediction: Fn(&T) -> Ordering;
}

pub trait UpperBoundExcludeBinarySearch<T> {
    fn upper_bound_exclude(&self, val: &T) -> usize;

    fn upper_bound_exclude_by<Prediction>(&self, p: Prediction) -> usize
        where Prediction: Fn(&T) -> Ordering;
}

impl<T: PartialOrd + Ord> LowerBoundBinarySearch<T> for &[T] {

    fn lower_bound(&self, val: &T) -> usize{
        self.lower_bound_by(|x| x.cmp(val))
    }

    fn lower_bound_by<Prediction>(&self, p: Prediction) -> usize
        where Prediction: Fn(&T) -> Ordering
    {
        let len = self.len();
        // [start, end)
        let mut start = 0usize;
        let mut end = len;
        while start < end {
            let mid = start + (end - start) / 2;
            if p(&self[mid]) == Ordering::Less {
                start = mid + 1
            } else {
                end = mid
            }
        }
        start
    }
}

impl<T: Ord> UpperBoundBinarySearch<T> for &[T] {

    fn upper_bound(&self, val: &T) -> usize {
        self.upper_bound_by(|x| x.cmp(val))
    }

    fn upper_bound_by<Prediction>(&self, p: Prediction) -> usize
        where Prediction: Fn(&T) -> Ordering
    {
        let len = self.len();
        let mut start = 0usize;
        let mut end = len;
        while start < end {
            let mid = start + (end - start) / 2;
            if p(&self[mid]) == Ordering::Greater {
                end = mid
            } else {
                start = mid + 1
            }
        }
        start
    }
}

#[test]
fn binary_search_lower_bound_test() {
    let data = vec![-1, 0, 0, 3, 3, 3, 7, 8, 9];
    assert_eq!(data.as_slice().lower_bound(&-2), 0usize);
    assert_eq!(data.as_slice().lower_bound(&-1), 0usize);
    assert_eq!(data.as_slice().lower_bound(&0), 1usize);
    assert_eq!(data.as_slice().lower_bound(&1), 3usize);
    assert_eq!(data.as_slice().lower_bound(&2), 3usize);
    assert_eq!(data.as_slice().lower_bound(&3), 3usize);
    assert_eq!(data.as_slice().lower_bound(&4), 6usize);
    assert_eq!(data.as_slice().lower_bound(&5), 6usize);
    assert_eq!(data.as_slice().lower_bound(&6), 6usize);
    assert_eq!(data.as_slice().lower_bound(&7), 6usize);
    assert_eq!(data.as_slice().lower_bound(&8), 7usize);
    assert_eq!(data.as_slice().lower_bound(&9), 8usize);
    assert_eq!(data.as_slice().lower_bound(&10), 9usize);

    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&-2)), 0usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&-1)), 0usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&0)), 1usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&1)), 3usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&2)), 3usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&3)), 3usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&4)), 6usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&5)), 6usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&6)), 6usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&7)), 6usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&8)), 7usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&9)), 8usize);
    assert_eq!(data.as_slice().lower_bound_by(|x| x.cmp(&10)), 9usize);
}

#[test]
fn binary_search_upper_bound_test() {
    let data = vec![-1, 0, 0, 3, 3, 3, 7, 8, 9];
    assert_eq!(data.as_slice().upper_bound(&-2), 0usize);
    assert_eq!(data.as_slice().upper_bound(&-1), 1usize);
    assert_eq!(data.as_slice().upper_bound(&0), 3usize);
    assert_eq!(data.as_slice().upper_bound(&1), 3usize);
    assert_eq!(data.as_slice().upper_bound(&2), 3usize);
    assert_eq!(data.as_slice().upper_bound(&3), 6usize);
    assert_eq!(data.as_slice().upper_bound(&4), 6usize);
    assert_eq!(data.as_slice().upper_bound(&5), 6usize);
    assert_eq!(data.as_slice().upper_bound(&6), 6usize);
    assert_eq!(data.as_slice().upper_bound(&7), 7usize);
    assert_eq!(data.as_slice().upper_bound(&8), 8usize);
    assert_eq!(data.as_slice().upper_bound(&9), 9usize);
    assert_eq!(data.as_slice().upper_bound(&10), 9usize);
}
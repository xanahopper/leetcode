use std::collections::BTreeSet;
use std::iter::FromIterator;

#[test]
fn weekly_300_4_test() {
    let mut set = SmallestInfiniteSet::new();
    set.add_back(2);
    assert_eq!(set.pop_smallest(), 1);
    assert_eq!(set.pop_smallest(), 2);
    assert_eq!(set.pop_smallest(), 3);
    set.add_back(1);
    assert_eq!(set.pop_smallest(), 1);
    assert_eq!(set.pop_smallest(), 4);
    assert_eq!(set.pop_smallest(), 5);

}

struct SmallestInfiniteSet {
    nums: Vec<bool>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        SmallestInfiniteSet {
            nums: vec![true; 1000]
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        let index = self.nums.iter().enumerate().find(|(i, &x)| x).unwrap().0;
        self.nums[index] = false;
        index as i32 + 1
    }

    fn add_back(&mut self, num: i32) {
        let n = num as usize;
        self.nums[n - 1] |= true;
    }
}
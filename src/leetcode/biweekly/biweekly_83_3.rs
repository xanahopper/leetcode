use std::collections::{BTreeSet, HashMap};

#[test]
fn biweekly_83_3_test() {
    let mut nc = NumberContainers::new();
    assert_eq!(nc.find(10), -1);
    nc.change(2, 10);
    nc.change(1, 10);
    nc.change(3, 10);
    nc.change(5, 10);
    assert_eq!(nc.find(10), 1);
    nc.change(1, 20);
    assert_eq!(nc.find(10), 2);
    assert_eq!(nc.find(20), 1);
}

struct NumberContainers {
    nums: HashMap<i32, i32>,
    positions: HashMap<i32, BTreeSet<i32>>
}

impl NumberContainers {

    fn new() -> Self {
        Self {
            nums: HashMap::new(),
            positions: HashMap::new()
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if self.nums.contains_key(&index) {
            let n = self.nums[&index];
            // remove n's index from it index set
            let entry = self.positions.entry(n);
            entry.and_modify(|x| { x.remove(&index); });
        }
        self.nums.insert(index, number);
        let entry = self.positions.entry(number).or_insert(BTreeSet::default());
        (*entry).insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        self.positions.get(&number).map_or(-1, |x| {
            let mut res = None;
            for &c in x {
                res = Some(c);
                break;
            }
            res.unwrap_or(-1)
        })
    }
}
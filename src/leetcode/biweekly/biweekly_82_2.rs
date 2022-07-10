use crate::leetcode::Solution;

#[test]
fn biweekly_82_2_test() {
    assert_eq!(Solution::latest_time_catch_the_bus(vec![10, 20], vec![2,17,18,19], 2), 16);
    assert_eq!(Solution::latest_time_catch_the_bus(vec![20,30,10], vec![19,13,26,4,25,11,21], 2), 20);
    assert_eq!(Solution::latest_time_catch_the_bus(vec![3], vec![2,4], 2), 3);
    assert_eq!(Solution::latest_time_catch_the_bus(vec![3], vec![2,3], 2), 1);
    assert_eq!(Solution::latest_time_catch_the_bus(vec![6,8,18,17], vec![6,8,17], 1), 18);
}

impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let n = buses.len();
        let m = passengers.len();
        let mut buses = buses;
        let mut passengers = passengers;
        buses.sort();
        passengers.sort();
        let mut i = 0;
        let mut j = 0;
        let mut res = 1;
        let mut last = 0;
        let mut last_bus = 0;
        let mut last_count = 0;
        while i < n && j <= m {
            let mut count = 0;
            let t = buses[i];
            while j < m && passengers[j] <= t && count < capacity {
                if passengers[j] - last > 1 {
                    res = passengers[j] - 1;
                }
                last = passengers[j];
                count += 1;
                j += 1;
            }
            last_bus = t;
            last_count = count;
            if count < capacity && last != t {
                res = t;
            }
            i += 1;
        }
        res
    }
}
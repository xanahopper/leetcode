use crate::leetcode::Solution;

#[test]
fn biweekly_80_2_test() {
    assert_eq!(Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7i64), vec![4, 0, 3]);
    assert_eq!(Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16i64), vec![2, 0, 2]);


    assert_eq!(Solution::successful_pairs(
        vec![36, 36, 22, 11, 35, 21, 4, 25, 30, 35, 31, 10, 8, 39, 7, 22, 18, 9, 23, 30, 9, 37, 22, 7, 36, 40, 17, 37, 38, 27, 6, 15, 1, 15, 7, 31, 36, 29, 9, 15, 3, 37, 15, 17, 25, 35, 9, 21, 5, 17, 25, 8, 18, 25, 7, 19, 4, 33, 9, 5, 29, 13, 9, 18, 5, 10, 31, 6, 7, 24, 13, 11, 8, 19, 2],
        vec![30, 11, 5, 20, 19, 36, 39, 24, 20, 37, 33, 22, 32, 28, 36, 24, 40, 27, 36, 37, 38, 23, 39, 11, 40, 19, 37, 32, 25, 29, 28, 37, 31, 36, 32, 40, 38, 22, 17, 38, 20, 33, 29, 17, 36, 33, 35, 25, 28, 18, 17, 19, 40, 27, 40, 28, 40, 40, 40, 39, 17, 34, 36, 11, 22, 29, 22, 35, 35, 22, 18, 34],
        135i64),
               vec![72, 72, 71, 68, 72, 71, 29, 71, 72, 72, 72, 68, 68, 72, 59, 71, 71, 68, 71, 72, 68, 72, 71, 59, 72, 72, 71, 72, 72, 72, 51, 71, 0, 71, 59, 72, 72, 72, 68, 71, 0, 72, 71, 71, 71, 72, 68, 71, 46, 71, 71, 68, 71, 71, 59, 71, 29, 72, 68, 46, 72, 71, 68, 71, 46, 68, 72, 51, 59, 71, 71, 68, 68, 71, 0]);
}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut spells = spells;
        let mut potions = potions;
        potions.sort();
        let len = potions.len() as i32;
        spells.into_iter().map(|s| {
            let potions: Vec<i64> = potions.iter().map(|&a| a as i64 * s as i64).collect();
            let min = potions[0];
            let max = *potions.last().unwrap();
            if success > max {
                0
            } else if success <= min {
                len
            } else {
                let r = Solution::binary_search(&potions.as_slice(), &success, 0, potions.len() - 1);
                match r {
                    Ok(index) => {
                        len - index as i32
                    }
                    Err(index) => {
                        if potions[index] > success {
                            len - index as i32
                        } else {
                            len - index as i32 - 1
                        }
                    }
                }
            }
        }).collect()
    }

    fn binary_search(data: &[i64], val: &i64, start: usize, end: usize) -> Result<usize, usize> {
        let len = end - start;
        if len <= 1 {
            if *val == data[start] {
                Ok(start)
            } else if *val > data[start] {
                Err(end)
            } else {
                Err(start)
            }
        } else {
            let mid = start + len / 2;
            if *val == data[mid] {
                Ok(mid)
            } else if *val == data[end - 1] {
                Solution::binary_search(data, val, start, end - 1)
            } else if *val > data[mid] {
                Solution::binary_search(data, val, mid + 1, end)
            } else {
                Solution::binary_search(data, val, start, mid - 1)
            }
        }
    }
}
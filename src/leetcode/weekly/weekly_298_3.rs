use crate::leetcode::Solution;

#[test]
fn weekly_298_3_test() {
    assert_eq!(Solution::longest_subsequence("1001010".to_string(), 5), 5);
    assert_eq!(Solution::longest_subsequence("00101001".to_string(), 2), 6);
    // "001010101011010100010101101010010"
    // 93951055
    assert_eq!(Solution::longest_subsequence("001010101011010100010101101010010".to_string(), 93951055), 22);
    // "0111011001"
    // 620776083
    assert_eq!(Solution::longest_subsequence("0111011001".to_string(), 620776083), 10);
}

const ZERO: u8 = '0' as u8;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let nums: Vec<_> = s.as_bytes().iter().map(|&x| (x - ZERO) as i32).collect();
        let n = nums.len();
        let mut length = vec![0usize; n];
        let mut data = vec![0; n];
        length[n - 1] = 1;
        data[n - 1] = nums[n - 1];
        for i in (0..n-1).rev() {
            if nums[i] == 0 {
                length[i] = length[i + 1] + 1;
                data[i] = data[i + 1];
            } else {
                length[i] = length[i + 1];
                data[i] = data[i + 1];
                for j in i + 1..n {
                    let append = nums[i] << (length[j]);
                    let new_data = data[j] + append;
                    if data[j] + append < k && new_data <= data[i] && length[j] + 1 > length[i] {
                        length[i] = length[j] + 1;
                        data[i] = new_data;
                    }
                }

            }
        }
        *length.iter().max().unwrap() as i32
    }
}
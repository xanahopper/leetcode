use crate::leetcode::Solution;

impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut cookies = cookies;
        cookies.sort();
        cookies.reverse();
        let mut res = i32::MAX;
        Solution::dispatch_cookies(&mut vec![0; k as usize], &cookies, 0, &mut res);
        res
    }

    fn dispatch_cookies(buckets: &mut Vec<i32>, cookies: &Vec<i32>, start: usize, res: &mut i32) {
        let k = buckets.len();
        let len = cookies.len();
        if start == len {
            *res = (*res).min(*buckets.iter().max().unwrap());
            return;
        }
        // 剪枝 1：不够分
        if buckets.iter().filter(|&&x| x == 0).count() > len - start {
            return;
        }
        // 剪枝 2：已经比现在答案还多了，就没必要继续下去了
        if buckets.iter().any(|x| *x > *res) {
            return;
        }
        for i in 0..k {
            if i > 0 && start == 0 {
                return;
            }
            buckets[i] += cookies[start];
            Solution::dispatch_cookies(buckets, cookies, start + 1, res);
            buckets[i] -= cookies[start];
        }
    }
}

#[test]
fn weekly_297_3_test() {
    assert_eq!(Solution::distribute_cookies(vec![8,15,10,20,8], 2), 31);
    assert_eq!(Solution::distribute_cookies(vec![6,1,3,2,2,4,1,2], 3), 7);
}
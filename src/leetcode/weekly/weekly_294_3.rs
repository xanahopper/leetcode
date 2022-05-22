use crate::leetcode::Solution;

#[test]
fn weekly_294_3_test() {
    assert_eq!(Solution::minimum_lines(vec![vec![0, 0]]), 0);
    assert_eq!(Solution::minimum_lines(vec![vec![1, 7], vec![2, 6], vec![3, 5], vec![4, 4], vec![5, 4], vec![6, 3], vec![7, 2], vec![8, 1]]), 3);
    assert_eq!(Solution::minimum_lines(vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]]), 1);
    assert_eq!(Solution::minimum_lines(vec![vec![83,35],vec![79,51],vec![61,48],vec![54,87],vec![44,93],vec![22,5],vec![87,28],vec![64,8],vec![89,78],vec![62,83],vec![58,72],vec![48,7],vec![97,16],vec![27,100],vec![65,48],vec![11,31],vec![29,76],vec![93,29],vec![72,59],vec![73,74],vec![9,90],vec![66,81],vec![12,8],vec![86,80],vec![84,43],vec![36,63],vec![80,45],vec![81,88],vec![95,5],vec![40,59]]), 29);
    assert_eq!(Solution::minimum_lines(vec![vec![1,1000000000],vec![1000000000,1000000000],vec![999999999,1],vec![2,999999999]]), 3);
}

#[derive(PartialOrd, PartialEq)]
struct Ratio {
    x: i32,
    y: i32,
}

fn get_contract(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        let r = a % b;
        let a = b;
        let b = r;
        get_contract(a, b)
    }
}

impl Ratio {
    fn new(x: i32, y: i32) -> Self {
        let contract = get_contract(y, x);
        if contract != 0 {
            Ratio { x: x / contract, y: y / contract}
        } else {
            Ratio { x, y }
        }
    }
}

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        let mut p = stock_prices;
        p.sort_by_key(|x| x[0]);
        p
            .windows(2)
            .into_iter()
            .fold((Ratio { x: 0, y: 0 }, 0), |(ratio, count), vector| {
                let p1 = &vector[0];
                let p2 = &vector[1];
                let r = Ratio::new(p2[0] - p1[0], p2[1] - p1[1]);
                if ratio == r {
                    (ratio, count)
                } else {
                    (r, count + 1)
                }
            }).1
    }
}
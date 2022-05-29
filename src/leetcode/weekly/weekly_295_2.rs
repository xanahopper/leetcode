use std::fmt::format;
use crate::leetcode::Solution;

#[test]
fn weekly_295_2_test() {
    assert_eq!(Solution::discount_prices("there are $1 $2 and 5$ candies in the shop".to_string(), 50), "there are $0.50 $1.00 and 5$ candies in the shop");
    assert_eq!(Solution::discount_prices("1 2 $3 4 $5 $6 7 8$ $9 $10$".to_string(), 100), "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$");
    assert_eq!(Solution::discount_prices("$76111 ab $6 $".to_string(), 48), "$39577.72 ab $3.12 $");
    assert_eq!(Solution::discount_prices("706hzu76jjh7yufr5x9ot60v149k5 $7651913186 pw2o $6".to_string(), 28), "706hzu76jjh7yufr5x9ot60v149k5 $5509377493.92 pw2o $4.32");
    assert_eq!(Solution::discount_prices(
        "f32eir5f6hlmmtnlq$zno3zbl5pr26b1xmet6q3rjzs422zqzsezpgi4jqx3h0olb428pk95qndkfz8hereio$2ewx0cnqlvnb6nl$$8iny7t4aemhnqzz6971rnq7pha97e9lf16227j5l2033pnddk $3513024 $516863 $604 $9128265 $945728 $nbf 5az21pm0tj $".to_string(), 26),
               "f32eir5f6hlmmtnlq$zno3zbl5pr26b1xmet6q3rjzs422zqzsezpgi4jqx3h0olb428pk95qndkfz8hereio$2ewx0cnqlvnb6nl$$8iny7t4aemhnqzz6971rnq7pha97e9lf16227j5l2033pnddk $2599637.76 $382478.62 $446.96 $6754916.10 $699838.72 $nbf 5az21pm0tj $");
}

const ZERO: u8 = '0' as u8;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        sentence.split(" ").map(|t| if let Some(price) = Solution::get_price(t) {
            format!("${0:.2}", price as f64 * (100.0f64 - discount as f64) / 100.0f64)
        } else { t.to_string() }).collect::<Vec<_>>().join(" ")
    }

    fn get_price(s: &str) -> Option<i64> {
        if s.starts_with("$") && s.as_bytes()[1..s.len()].iter().all(|&x| x >= '0' as u8 && x <= '9' as u8) && s.len() > 1 {
            Some(s.as_bytes()[1..s.len()]
                .iter()
                .fold(0i64, |res, &x|
                    res * 10 + (x - ZERO) as i64)
            )
        } else { None }
    }
}
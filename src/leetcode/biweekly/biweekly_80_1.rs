use crate::leetcode::Solution;

#[test]
fn biweekly_80_1_test() {
    assert_eq!(Solution::strong_password_checker_ii("IloveLe3tcode!".to_string()), true);
    assert_eq!(Solution::strong_password_checker_ii("Me+You--IsMyDream".to_string()), false);
    assert_eq!(Solution::strong_password_checker_ii("1aB!".to_string()), false);
}

const UPPER: u8 = 'A' as u8;
const LOWER: u8 = 'a' as u8;
const NUMBER: u8 = '0' as u8;

struct StrongPasswordFlag {
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
    same: bool
}

impl Solution {

    pub fn strong_password_checker_ii(password: String) -> bool {
        let symbols: Vec<u8> = Vec::from("!@#$%^&*()-+");
        if password.len() < 8 {
            return false;
        }
        let flag = password.as_bytes().iter().fold((StrongPasswordFlag {
            upper: false,
            lower: false,
            number: false,
            symbol: false,
            same: false
        }, None), |(mut res, last_char), &c| {
            res.upper |= c >= UPPER && c < UPPER + 26;
            res.lower |= c >= LOWER && c < LOWER + 26;
            res.number |= c >= NUMBER && c < NUMBER + 10;
            res.symbol |= symbols.contains(&c);
            if let Some(lc) = last_char {
                res.same |= c == lc;
            }
            (res, Some(c))
        }).0;
        flag.upper && flag.lower && flag.number && flag.symbol && !flag.same
    }
}
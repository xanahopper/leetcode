mod kmp;
pub mod segment_tree;
pub mod binary_search;

pub mod basic {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            let r = a % b;
            let a = b;
            let b = r;
            gcd(a, b)
        }
    }
}
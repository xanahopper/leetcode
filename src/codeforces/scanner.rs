use std::io::{Read, stdin};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    // fn next<T : std::str::FromStr>(&mut self) -> T {
    //     loop {
    //         if let Some(token) = self.buffer.pop() {
    //             token.parse().ok().expect("Failed parse")
    //         } else {
    //             let mut input = String::new();
    //             stdin().read_line(&mut input).expect("Failed read");
    //             self.buffer = input.split_whitespace().rev().map(String::from).collect();
    //         }
    //     }
    // }
}
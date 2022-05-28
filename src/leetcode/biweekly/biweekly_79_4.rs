use std::ops::Range;

struct BookMyShow {
    rows: Vec<Vec<Range<i32>>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {

    fn new(n: i32, m: i32) -> Self {
        BookMyShow {
            rows: vec![vec![0..m]; n as usize]
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        for (index, mut row) in self.rows[0..=max_row as usize].iter().enumerate() {
            if let Some(position) = row.iter().enumerate().filter(|r| r.len() >= k as usize).min_by_key(|r| r.1.len()) {
                let r = row.remove(position.0);
                if r.len() > k as usize {
                    row.push(r.start + k .. r.end)
                }
                return vec![index as i32, r.start]
            }
        }
        vec![]
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        if self.rows[0..=max_row as usize].iter().fold(0, |res, r| {
            res + r.iter().map(|x| x.len()).sum()
        }) >= k {
            // self.rows[0..=max_row as usize].iter().enumerate().fold((true, k), |(res, remain), (index, range)| {
            //     if let Some((position, r)) = range.iter().enumerate().min_by_key(|r| r.1.len()) {
            //         if r.len() > remain as usize {
            //
            //         }
            //     }
            // });
            true
        } else { false }

        // for (index, mut row) in self.rows[0..=max_row as usize].iter().enumerate() {
        //     if let Some(position) = row.iter().enumerate().min_by_key(|r| r.1.len()) {
        //         let r = row.remove(position.0);
        //         if r.len() > remain as usize {
        //             row.push(r.start + remain .. r.end)
        //         } else {
        //
        //         }
        //         return vec![index as i32, r.start]
        //     }
        // }
        false
    }
}
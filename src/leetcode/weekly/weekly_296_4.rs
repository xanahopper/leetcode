use std::cmp::{max, min};
use std::collections::{LinkedList, VecDeque};

#[test]
fn weekly_296_4_test() {
    let mut editor = TextEditor::new();
    editor.add_text("leetcode".to_string());
    assert_eq!(editor.delete_text(4), 4);
    editor.add_text("practice".to_string());
    assert_eq!(editor.cursor_right(3), "etpractice");
    assert_eq!(editor.cursor_left(8), "leet");
    editor.delete_text(10);
    assert_eq!(editor.cursor_left(2), "");
    assert_eq!(editor.cursor_right(6), "practi");
}

struct TextEditor {
    before: Vec<u8>,
    after: Vec<u8>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {

    fn new() -> Self {
        TextEditor {
            before: Vec::new(),
            after: Vec::new()
        }
    }

    fn add_text(&mut self, text: String) {
        self.before.append(&mut text.into_bytes())
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = min(k as usize, self.before.len());
        for i in 0..k {
            self.before.pop();
        }
        k as i32
    }

    fn cursor_left(&mut self, k: i32) -> String {
        let k = min(k as usize, self.before.len());
        for i in 0..k {
            self.after.push(self.before.pop().unwrap());
        }
        let len = self.before.len();
        String::from_utf8_lossy(&self.before[len - min(10usize, len)..len]).to_string()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let k = min(k as usize, self.after.len());
        for i in 0..k {
            self.before.push(self.after.pop().unwrap());
        }
        let len = self.before.len();
        String::from_utf8_lossy(&self.before[len - min(10usize, len)..len]).to_string()
    }
}
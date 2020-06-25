pub trait KmpSearch {
    type T;

    fn index_of(&self, pattern: &Self::T) -> Option<usize>;
}

impl KmpSearch for str {
    type T = String;

    fn index_of(&self, pattern: &String) -> Option<usize> {
        None
    }
}

use std::ops::Range;

pub trait Scanner {
    fn scan(&self, s: &str, trigger_index: usize) -> Option<Range<usize>>;
}

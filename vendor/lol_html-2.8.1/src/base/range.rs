use super::Align;

// NOTE: std::ops::Range implements iterator and, thus, doesn't implement Copy.
// See: https://github.com/rust-lang/rust/pull/27186
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Range {
    pub start: usize,
    pub end: usize,
}

impl Align for Range {
    #[inline]
    fn align(&mut self, offset: usize) {
        self.start.align(offset);
        self.end.align(offset);
    }
}

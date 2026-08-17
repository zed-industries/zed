use proc_macro2::Span;

#[derive(Copy, Clone)]
pub struct Spans {
    pub start: Span,
    pub end: Span,
}

impl Spans {
    pub fn joined(self) -> Span {
        self.start.join(self.end).unwrap_or(self.start)
    }
}

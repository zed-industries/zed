use crate::arena::Arena;

/// Helper class to emit expressions
#[derive(Default, Debug)]
pub struct Emitter {
    start_len: Option<usize>,
}

impl Emitter {
    pub fn start(&mut self, arena: &Arena<crate::Expression>) {
        if self.start_len.is_some() {
            unreachable!("Emitting has already started!");
        }
        self.start_len = Some(arena.len());
    }
    pub const fn is_running(&self) -> bool {
        self.start_len.is_some()
    }
    #[must_use]
    pub fn finish(
        &mut self,
        arena: &Arena<crate::Expression>,
    ) -> Option<(crate::Statement, crate::span::Span)> {
        let start_len = self.start_len.take().unwrap();
        if start_len != arena.len() {
            let mut span = crate::span::Span::default();
            let range = arena.range_from(start_len);
            for handle in range.clone() {
                span.subsume(arena.get_span(handle))
            }
            Some((crate::Statement::Emit(range), span))
        } else {
            None
        }
    }
}

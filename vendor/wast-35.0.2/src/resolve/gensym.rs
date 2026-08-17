use crate::ast::{Id, Span};
use std::cell::Cell;

thread_local!(static NEXT: Cell<u32> = Cell::new(0));

pub fn reset() {
    NEXT.with(|c| c.set(0));
}

pub fn gen(span: Span) -> Id<'static> {
    NEXT.with(|next| {
        let gen = next.get() + 1;
        next.set(gen);
        Id::gensym(span, gen)
    })
}

pub fn fill<'a>(span: Span, slot: &mut Option<Id<'a>>) -> Id<'a> {
    *slot.get_or_insert_with(|| gen(span))
}

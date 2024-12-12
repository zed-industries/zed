use gpui::{AppContext, Model, ModelContext};
use language::Buffer;
use std::ops::Range;

// TODO: Find a better home for `Direction`.
//
// This should live in an ancestor crate of `editor` and `inline_completion`,
// but at time of writing there isn't an obvious spot.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

#[derive(Clone)]
pub struct InlineCompletion {
    pub edits: Vec<(Range<language::Anchor>, String)>,
}

pub trait InlineCompletionProvider: 'static + Sized {
    fn name() -> &'static str;
    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool;
    fn refresh(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut ModelContext<Self>,
    );
    fn cycle(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut ModelContext<Self>,
    );
    fn accept(&mut self, cx: &mut ModelContext<Self>);
    fn discard(&mut self, cx: &mut ModelContext<Self>);
    fn suggest(
        &mut self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Option<InlineCompletion>;
}

pub trait InlineCompletionProviderHandle {
    fn name(&self) -> &'static str;
    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool;
    fn refresh(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut AppContext,
    );
    fn cycle(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut AppContext,
    );
    fn accept(&self, cx: &mut AppContext);
    fn discard(&self, cx: &mut AppContext);
    fn suggest(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut AppContext,
    ) -> Option<InlineCompletion>;
}

impl<T> InlineCompletionProviderHandle for Model<T>
where
    T: InlineCompletionProvider,
{
    fn name(&self) -> &'static str {
        T::name()
    }

    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool {
        self.read(cx).is_enabled(buffer, cursor_position, cx)
    }

    fn refresh(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut AppContext,
    ) {
        self.update(cx, |this, cx| {
            this.refresh(buffer, cursor_position, debounce, cx)
        })
    }

    fn cycle(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut AppContext,
    ) {
        self.update(cx, |this, cx| {
            this.cycle(buffer, cursor_position, direction, cx)
        })
    }

    fn accept(&self, cx: &mut AppContext) {
        self.update(cx, |this, cx| this.accept(cx))
    }

    fn discard(&self, cx: &mut AppContext) {
        self.update(cx, |this, cx| this.discard(cx))
    }

    fn suggest(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut AppContext,
    ) -> Option<InlineCompletion> {
        self.update(cx, |this, cx| this.suggest(buffer, cursor_position, cx))
    }
}

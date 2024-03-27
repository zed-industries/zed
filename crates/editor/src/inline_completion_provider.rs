use crate::Direction;
use gpui::{AppContext, Model, ModelContext};
use language::Buffer;

pub trait InlineCompletionProvider: 'static + Sized {
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
    fn active_completion_text(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> Option<String>;
}

pub trait InlineCompletionProviderHandle {
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
    fn active_completion_text<'a>(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &'a AppContext,
    ) -> Option<String>;
}

impl<T> InlineCompletionProviderHandle for Model<T>
where
    T: InlineCompletionProvider,
{
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

    fn active_completion_text<'a>(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &'a AppContext,
    ) -> Option<String> {
        self.read(cx)
            .active_completion_text(buffer, cursor_position, cx)
    }
}

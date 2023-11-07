use editor::Editor;
use gpui::{
    div, uniform_list, Component, Div, ParentElement, Render, StatelessInteractive, Styled,
    UniformListScrollHandle, View, ViewContext, VisualContext,
};
use std::cmp;

pub struct Picker<D: PickerDelegate> {
    pub delegate: D,
    scroll_handle: UniformListScrollHandle,
    editor: View<Editor>,
}

pub trait PickerDelegate: Sized + 'static {
    type ListItem: Component<Picker<Self>>;

    fn match_count(&self) -> usize;
    fn selected_index(&self) -> usize;
    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>);

    // fn placeholder_text(&self) -> Arc<str>;
    // fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()>;

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>);
    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>);

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Self::ListItem;
}

impl<D: PickerDelegate> Picker<D> {
    pub fn new(delegate: D, cx: &mut ViewContext<Self>) -> Self {
        Self {
            delegate,
            scroll_handle: UniformListScrollHandle::new(),
            editor: cx.build_view(|cx| Editor::single_line(cx)),
        }
    }

    fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = cmp::min(index + 1, count - 1);
            self.delegate.set_selected_index(ix, cx);
            self.scroll_handle.scroll_to_item(ix);
        }
    }

    fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = index.saturating_sub(1);
            self.delegate.set_selected_index(ix, cx);
            self.scroll_handle.scroll_to_item(ix);
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.delegate.set_selected_index(0, cx);
            self.scroll_handle.scroll_to_item(0);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.delegate.set_selected_index(count - 1, cx);
            self.scroll_handle.scroll_to_item(count - 1);
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        self.delegate.dismissed(cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        self.delegate.confirm(false, cx);
    }

    fn secondary_confirm(&mut self, _: &menu::SecondaryConfirm, cx: &mut ViewContext<Self>) {
        self.delegate.confirm(true, cx);
    }
}

impl<D: PickerDelegate> Render for Picker<D> {
    type Element = Div<Self>;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .size_full()
            .context("picker")
            .on_action(Self::select_next)
            .on_action(Self::select_prev)
            .on_action(Self::select_first)
            .on_action(Self::select_last)
            .on_action(Self::cancel)
            .on_action(Self::confirm)
            .on_action(Self::secondary_confirm)
            .child(self.editor.clone())
            .child(
                uniform_list("candidates", self.delegate.match_count(), {
                    move |this: &mut Self, visible_range, cx| {
                        let selected_ix = this.delegate.selected_index();
                        visible_range
                            .map(|ix| this.delegate.render_match(ix, ix == selected_ix, cx))
                            .collect()
                    }
                })
                .track_scroll(self.scroll_handle.clone())
                .size_full(),
            )
    }
}

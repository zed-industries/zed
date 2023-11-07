use gpui::{
    div, uniform_list, Component, ElementId, FocusHandle, ParentElement, StatelessInteractive,
    Styled, UniformListScrollHandle, ViewContext,
};
use std::cmp;

#[derive(Component)]
pub struct Picker<V: PickerDelegate> {
    id: ElementId,
    focus_handle: FocusHandle,
    phantom: std::marker::PhantomData<V>,
}

pub trait PickerDelegate: Sized + 'static {
    type ListItem: Component<Self>;

    fn match_count(&self, picker_id: ElementId) -> usize;
    fn selected_index(&self, picker_id: ElementId) -> usize;
    fn set_selected_index(&mut self, ix: usize, picker_id: ElementId, cx: &mut ViewContext<Self>);

    // fn placeholder_text(&self) -> Arc<str>;
    // fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()>;

    fn confirm(&mut self, secondary: bool, picker_id: ElementId, cx: &mut ViewContext<Self>);
    fn dismissed(&mut self, picker_id: ElementId, cx: &mut ViewContext<Self>);

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        picker_id: ElementId,
        cx: &mut ViewContext<Self>,
    ) -> Self::ListItem;
}

impl<V: PickerDelegate> Picker<V> {
    pub fn new(id: impl Into<ElementId>, focus_handle: FocusHandle) -> Self {
        Self {
            id: id.into(),
            focus_handle,
            phantom: std::marker::PhantomData,
        }
    }

    fn bind_actions<T: StatelessInteractive<V>>(
        div: T,
        id: ElementId,
        scroll_handle: &UniformListScrollHandle,
    ) -> T {
        div.on_action({
            let id = id.clone();
            let scroll_handle = scroll_handle.clone();
            move |view: &mut V, _: &menu::SelectNext, cx| {
                let count = view.match_count(id.clone());
                if count > 0 {
                    let index = view.selected_index(id.clone());
                    let ix = cmp::min(index + 1, count - 1);
                    view.set_selected_index(ix, id.clone(), cx);
                    scroll_handle.scroll_to_item(ix);
                }
            }
        })
        .on_action({
            let id = id.clone();
            let scroll_handle = scroll_handle.clone();
            move |view, _: &menu::SelectPrev, cx| {
                let count = view.match_count(id.clone());
                if count > 0 {
                    let index = view.selected_index(id.clone());
                    let ix = index.saturating_sub(1);
                    view.set_selected_index(ix, id.clone(), cx);
                    scroll_handle.scroll_to_item(ix);
                }
            }
        })
        .on_action({
            let id = id.clone();
            let scroll_handle = scroll_handle.clone();
            move |view: &mut V, _: &menu::SelectFirst, cx| {
                let count = view.match_count(id.clone());
                if count > 0 {
                    view.set_selected_index(0, id.clone(), cx);
                    scroll_handle.scroll_to_item(0);
                }
            }
        })
        .on_action({
            let id = id.clone();
            let scroll_handle = scroll_handle.clone();
            move |view: &mut V, _: &menu::SelectLast, cx| {
                let count = view.match_count(id.clone());
                if count > 0 {
                    view.set_selected_index(count - 1, id.clone(), cx);
                    scroll_handle.scroll_to_item(count - 1);
                }
            }
        })
        .on_action({
            let id = id.clone();
            move |view: &mut V, _: &menu::Cancel, cx| {
                view.dismissed(id.clone(), cx);
            }
        })
        .on_action({
            let id = id.clone();
            move |view: &mut V, _: &menu::Confirm, cx| {
                view.confirm(false, id.clone(), cx);
            }
        })
        .on_action({
            let id = id.clone();
            move |view: &mut V, _: &menu::SecondaryConfirm, cx| {
                view.confirm(true, id.clone(), cx);
            }
        })
    }
}

impl<V: 'static + PickerDelegate> Picker<V> {
    pub fn render(self, view: &mut V, _cx: &mut ViewContext<V>) -> impl Component<V> {
        let id = self.id.clone();
        let scroll_handle = UniformListScrollHandle::new();
        Self::bind_actions(
            div()
                .id(self.id.clone())
                .size_full()
                .track_focus(&self.focus_handle)
                .context("picker")
                .child(
                    uniform_list(
                        "candidates",
                        view.match_count(self.id.clone()),
                        move |view: &mut V, visible_range, cx| {
                            let selected_ix = view.selected_index(self.id.clone());
                            visible_range
                                .map(|ix| {
                                    view.render_match(ix, ix == selected_ix, self.id.clone(), cx)
                                })
                                .collect()
                        },
                    )
                    .track_scroll(scroll_handle.clone())
                    .size_full(),
                ),
            id,
            &scroll_handle,
        )
    }
}

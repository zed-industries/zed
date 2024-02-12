use std::sync::Arc;

use gpui::{
    actions, rems, DismissEvent, EventEmitter, FocusableView, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WindowContext,
};
use picker::{Picker, PickerDelegate};
use ui::{v_flex, ListItem};
use workspace::ModalView;

actions!(runnables, [New]);
/// A modal used to spawn new runnables.
pub(crate) struct RunnablesModalDelegate {
    candidates: Vec<Model<Box<dyn Runnable>>>,
}

pub(crate) struct RunnablesModal(View<Picker<RunnablesModalDelegate>>);

impl RunnablesModal {
    pub(crate) fn new(cx: &mut WindowContext) -> Self {
        Self(cx.new_view(|cx| Picker::new(RunnablesModalDelegate, cx)))
    }
}
impl Render for RunnablesModal {
    fn render(
        &mut self,
        cx: &mut ui::prelude::ViewContext<Self>,
    ) -> impl gpui::prelude::IntoElement {
        v_flex()
            .w(rems(20.))
            .child(self.0.clone())
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                this.0.update(cx, |this, cx| {
                    this.cancel(&Default::default(), cx);
                })
            }))
    }
}

impl EventEmitter<DismissEvent> for RunnablesModal {}
impl FocusableView for RunnablesModal {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.0.read(cx).focus_handle(cx)
    }
}
impl ModalView for RunnablesModal {}

impl PickerDelegate for RunnablesModalDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        2
    }

    fn selected_index(&self) -> usize {
        0
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        cx: &mut ui::prelude::ViewContext<picker::Picker<Self>>,
    ) {
    }

    fn placeholder_text(&self) -> Arc<str> {
        Arc::from("Select a runnable")
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<picker::Picker<Self>>) {}

    fn dismissed(&mut self, cx: &mut ViewContext<picker::Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ui::prelude::ViewContext<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(ListItem::new(ix).child("A"))
    }
}

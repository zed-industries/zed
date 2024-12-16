use std::rc::Rc;

use gpui::{View, WeakModel, WeakView};
use ui::{prelude::*, IconButtonShape, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::Workspace;

use crate::context::{Context, ContextId, ContextKind};
use crate::context_picker::ContextPicker;
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;

pub struct ContextStrip {
    context: Vec<Context>,
    next_context_id: ContextId,
    context_picker: View<ContextPicker>,
    pub(crate) context_picker_handle: PopoverMenuHandle<ContextPicker>,
}

impl ContextStrip {
    pub fn new(
        workspace: WeakView<Workspace>,
        thread_store: WeakModel<ThreadStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let weak_self = cx.view().downgrade();

        Self {
            context: Vec::new(),
            next_context_id: ContextId(0),
            context_picker: cx.new_view(|cx| {
                ContextPicker::new(workspace.clone(), thread_store.clone(), weak_self, cx)
            }),
            context_picker_handle: PopoverMenuHandle::default(),
        }
    }

    pub fn drain(&mut self) -> Vec<Context> {
        self.context.drain(..).collect()
    }

    pub fn insert_context(
        &mut self,
        kind: ContextKind,
        name: impl Into<SharedString>,
        text: impl Into<SharedString>,
    ) {
        self.context.push(Context {
            id: self.next_context_id.post_inc(),
            name: name.into(),
            kind,
            text: text.into(),
        });
    }
}

impl Render for ContextStrip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let context_picker = self.context_picker.clone();

        h_flex()
            .flex_wrap()
            .gap_2()
            .child(
                PopoverMenu::new("context-picker")
                    .menu(move |_cx| Some(context_picker.clone()))
                    .trigger(
                        IconButton::new("add-context", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small),
                    )
                    .attach(gpui::AnchorCorner::TopLeft)
                    .anchor(gpui::AnchorCorner::BottomLeft)
                    .offset(gpui::Point {
                        x: px(0.0),
                        y: px(-16.0),
                    })
                    .with_handle(self.context_picker_handle.clone()),
            )
            .children(self.context.iter().map(|context| {
                ContextPill::new(context.clone()).on_remove({
                    let context = context.clone();
                    Rc::new(cx.listener(move |this, _event, cx| {
                        this.context.retain(|other| other.id != context.id);
                        cx.notify();
                    }))
                })
            }))
            .when(!self.context.is_empty(), |parent| {
                parent.child(
                    IconButton::new("remove-all-context", IconName::Eraser)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .tooltip(move |cx| Tooltip::text("Remove All Context", cx))
                        .on_click(cx.listener(|this, _event, cx| {
                            this.context.clear();
                            cx.notify();
                        })),
                )
            })
    }
}

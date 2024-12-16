use std::rc::Rc;

use gpui::{Model, View, WeakModel, WeakView};
use ui::{prelude::*, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::Workspace;

use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;

pub struct ContextStrip {
    context_store: Model<ContextStore>,
    context_picker: View<ContextPicker>,
    pub(crate) context_picker_handle: PopoverMenuHandle<ContextPicker>,
}

impl ContextStrip {
    pub fn new(
        context_store: Model<ContextStore>,
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            context_store: context_store.clone(),
            context_picker: cx.new_view(|cx| {
                ContextPicker::new(
                    workspace.clone(),
                    thread_store.clone(),
                    context_store.downgrade(),
                    cx,
                )
            }),
            context_picker_handle: PopoverMenuHandle::default(),
        }
    }
}

impl Render for ContextStrip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let context = self.context_store.read(cx).context();
        let context_picker = self.context_picker.clone();

        h_flex()
            .flex_wrap()
            .gap_1()
            .child(
                PopoverMenu::new("context-picker")
                    .menu(move |_cx| Some(context_picker.clone()))
                    .trigger(
                        IconButton::new("add-context", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .style(ui::ButtonStyle::Filled),
                    )
                    .attach(gpui::AnchorCorner::TopLeft)
                    .anchor(gpui::AnchorCorner::BottomLeft)
                    .offset(gpui::Point {
                        x: px(0.0),
                        y: px(-16.0),
                    })
                    .with_handle(self.context_picker_handle.clone()),
            )
            .children(context.iter().map(|context| {
                ContextPill::new(context.clone()).on_remove({
                    let context = context.clone();
                    let context_store = self.context_store.clone();
                    Rc::new(cx.listener(move |_this, _event, cx| {
                        context_store.update(cx, |this, _cx| {
                            this.remove_context(&context.id);
                        });
                        cx.notify();
                    }))
                })
            }))
            .when(!context.is_empty(), |parent| {
                parent.child(
                    IconButton::new("remove-all-context", IconName::Eraser)
                        .icon_size(IconSize::Small)
                        .tooltip(move |cx| Tooltip::text("Remove All Context", cx))
                        .on_click({
                            let context_store = self.context_store.clone();
                            cx.listener(move |_this, _event, cx| {
                                context_store.update(cx, |this, _cx| this.clear());
                                cx.notify();
                            })
                        }),
                )
            })
    }
}

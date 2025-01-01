use std::rc::Rc;

use gpui::{FocusHandle, Model, View, WeakModel, WeakView};
use ui::{prelude::*, PopoverMenu, PopoverMenuHandle, Tooltip};
use workspace::Workspace;

use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;
use crate::ToggleContextPicker;
use settings::Settings;

pub struct ContextStrip {
    context_store: Model<ContextStore>,
    context_picker: View<ContextPicker>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    focus_handle: FocusHandle,
}

impl ContextStrip {
    pub fn new(
        context_store: Model<ContextStore>,
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        focus_handle: FocusHandle,
        context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            context_store: context_store.clone(),
            context_picker: cx.new_view(|cx| {
                ContextPicker::new(
                    workspace.clone(),
                    thread_store.clone(),
                    context_store.downgrade(),
                    ConfirmBehavior::KeepOpen,
                    cx,
                )
            }),
            context_picker_menu_handle,
            focus_handle,
        }
    }
}

impl Render for ContextStrip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let context = self.context_store.read(cx).context().clone();
        let context_picker = self.context_picker.clone();
        let focus_handle = self.focus_handle.clone();

        h_flex()
            .flex_wrap()
            .gap_1()
            .child(
                PopoverMenu::new("context-picker")
                    .menu(move |_cx| Some(context_picker.clone()))
                    .trigger(
                        IconButton::new("add-context", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .style(ui::ButtonStyle::Filled)
                            .tooltip(move |cx| {
                                Tooltip::for_action_in(
                                    "Add Context",
                                    &ToggleContextPicker,
                                    &focus_handle,
                                    cx,
                                )
                            }),
                    )
                    .attach(gpui::Corner::TopLeft)
                    .anchor(gpui::Corner::BottomLeft)
                    .offset(gpui::Point {
                        x: px(0.0),
                        y: px(-16.0),
                    })
                    .with_handle(self.context_picker_menu_handle.clone()),
            )
            .when(context.is_empty(), {
                |parent| {
                    parent.child(
                        h_flex()
                            .id("no-content-info")
                            .ml_1p5()
                            .gap_2()
                            .font(theme::ThemeSettings::get_global(cx).buffer_font.clone())
                            .text_size(TextSize::Small.rems(cx))
                            .text_color(cx.theme().colors().text_muted)
                            .child("Add Context")
                            .children(
                                ui::KeyBinding::for_action_in(
                                    &ToggleContextPicker,
                                    &self.focus_handle,
                                    cx,
                                )
                                .map(|binding| binding.into_any_element()),
                            )
                            .opacity(0.5),
                    )
                }
            })
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
            .when(!context.is_empty(), {
                move |parent| {
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
                }
            })
    }
}

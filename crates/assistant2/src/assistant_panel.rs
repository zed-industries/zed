use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use client::zed_urls;
use gpui::{
    prelude::*, px, svg, Action, AnyElement, AppContext, AsyncWindowContext, EventEmitter,
    FocusHandle, FocusableView, FontWeight, Model, Pixels, Task, View, ViewContext, WeakView,
    WindowContext,
};
use language::LanguageRegistry;
use language_model::LanguageModelRegistry;
use language_model_selector::LanguageModelSelector;
use ui::{prelude::*, ButtonLike, Divider, IconButtonShape, KeyBinding, ListItem, Tab, Tooltip};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

use crate::active_thread::ActiveThread;
use crate::message_editor::MessageEditor;
use crate::thread::{Thread, ThreadError, ThreadId};
use crate::thread_store::ThreadStore;
use crate::{NewThread, OpenHistory, ToggleFocus, ToggleModelSelector};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<AssistantPanel>(cx);
            });
        },
    )
    .detach();
}

pub struct AssistantPanel {
    workspace: WeakView<Workspace>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Model<ThreadStore>,
    thread: Option<View<ActiveThread>>,
    message_editor: View<MessageEditor>,
    tools: Arc<ToolWorkingSet>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let tools = Arc::new(ToolWorkingSet::default());
            let thread_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    ThreadStore::new(project, tools.clone(), cx)
                })?
                .await?;

            workspace.update(&mut cx, |workspace, cx| {
                cx.new_view(|cx| Self::new(workspace, thread_store, tools, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Model<ThreadStore>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let thread = thread_store.update(cx, |this, cx| this.create_thread(cx));

        Self {
            workspace: workspace.weak_handle(),
            language_registry: workspace.project().read(cx).languages().clone(),
            thread_store,
            thread: None,
            message_editor: cx.new_view(|cx| MessageEditor::new(thread, cx)),
            tools,
        }
    }

    fn new_thread(&mut self, cx: &mut ViewContext<Self>) {
        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(cx));

        self.thread = Some(cx.new_view(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.tools.clone(),
                cx,
            )
        }));
        self.message_editor = cx.new_view(|cx| MessageEditor::new(thread, cx));
        self.message_editor.focus_handle(cx).focus(cx);
    }

    fn open_thread(&mut self, thread_id: &ThreadId, cx: &mut ViewContext<Self>) {
        let Some(thread) = self
            .thread_store
            .update(cx, |this, cx| this.open_thread(thread_id, cx))
        else {
            return;
        };

        self.thread = Some(cx.new_view(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.tools.clone(),
                cx,
            )
        }));
        self.message_editor = cx.new_view(|cx| MessageEditor::new(thread, cx));
        self.message_editor.focus_handle(cx).focus(cx);
    }
}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel2"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, _position: DockPosition, _cx: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &WindowContext) -> Pixels {
        px(640.)
    }

    fn set_size(&mut self, _size: Option<Pixels>, _cx: &mut ViewContext<Self>) {}

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl AssistantPanel {
    fn render_toolbar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        h_flex()
            .id("assistant-toolbar")
            .justify_between()
            .gap(DynamicSpacing::Base08.rems(cx))
            .h(Tab::container_height(cx))
            .px(DynamicSpacing::Base08.rems(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(h_flex().child(Label::new("Thread Title Goes Here")))
            .child(
                h_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .child(self.render_language_model_selector(cx))
                    .child(Divider::vertical())
                    .child(
                        IconButton::new("new-thread", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "New Thread",
                                        &NewThread,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click(move |_event, cx| {
                                cx.dispatch_action(NewThread.boxed_clone());
                            }),
                    )
                    .child(
                        IconButton::new("open-history", IconName::HistoryRerun)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "Open History",
                                        &OpenHistory,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click(move |_event, cx| {
                                cx.dispatch_action(OpenHistory.boxed_clone());
                            }),
                    )
                    .child(
                        IconButton::new("configure-assistant", IconName::Settings)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip(move |cx| Tooltip::text("Configure Assistant", cx))
                            .on_click(move |_event, _cx| {
                                println!("Configure Assistant");
                            }),
                    ),
            )
    }

    fn render_language_model_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_provider = LanguageModelRegistry::read_global(cx).active_provider();
        let active_model = LanguageModelRegistry::read_global(cx).active_model();

        LanguageModelSelector::new(
            |model, _cx| {
                println!("Selected {:?}", model.name());
            },
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .w_full()
                        .gap_0p5()
                        .child(
                            div()
                                .overflow_x_hidden()
                                .flex_grow()
                                .whitespace_nowrap()
                                .child(match (active_provider, active_model) {
                                    (Some(provider), Some(model)) => h_flex()
                                        .gap_1()
                                        .child(
                                            Icon::new(
                                                model.icon().unwrap_or_else(|| provider.icon()),
                                            )
                                            .color(Color::Muted)
                                            .size(IconSize::XSmall),
                                        )
                                        .child(
                                            Label::new(model.name().0)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .into_any_element(),
                                    _ => Label::new("No model selected")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element(),
                                }),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        ),
                )
                .tooltip(move |cx| Tooltip::for_action("Change Model", &ToggleModelSelector, cx)),
        )
    }

    fn render_active_thread_or_empty_state(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let Some(thread) = self.thread.as_ref() else {
            return self.render_thread_empty_state(cx).into_any_element();
        };

        if thread.read(cx).is_empty() {
            return self.render_thread_empty_state(cx).into_any_element();
        }

        thread.clone().into_any()
    }

    fn render_thread_empty_state(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let recent_threads = self
            .thread_store
            .update(cx, |this, cx| this.recent_threads(3, cx));

        v_flex()
            .gap_2()
            .mx_auto()
            .child(
                v_flex().w_full().child(
                    svg()
                        .path("icons/logo_96.svg")
                        .text_color(cx.theme().colors().text)
                        .w(px(40.))
                        .h(px(40.))
                        .mx_auto()
                        .mb_4(),
                ),
            )
            .child(v_flex())
            .child(
                h_flex()
                    .w_full()
                    .justify_center()
                    .child(Label::new("Context Examples:").size(LabelSize::Small)),
            )
            .child(
                h_flex()
                    .gap_2()
                    .justify_center()
                    .child(
                        h_flex()
                            .gap_1()
                            .p_0p5()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                Icon::new(IconName::Terminal)
                                    .size(IconSize::Small)
                                    .color(Color::Disabled),
                            )
                            .child(Label::new("Terminal").size(LabelSize::Small)),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .p_0p5()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                Icon::new(IconName::Folder)
                                    .size(IconSize::Small)
                                    .color(Color::Disabled),
                            )
                            .child(Label::new("/src/components").size(LabelSize::Small)),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_center()
                    .child(Label::new("Recent Threads:").size(LabelSize::Small)),
            )
            .child(
                v_flex().gap_2().children(
                    recent_threads
                        .into_iter()
                        .map(|thread| self.render_past_thread(thread, cx)),
                ),
            )
            .child(
                h_flex().w_full().justify_center().child(
                    Button::new("view-all-past-threads", "View All Past Threads")
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small)
                        .key_binding(KeyBinding::for_action_in(
                            &OpenHistory,
                            &self.focus_handle(cx),
                            cx,
                        ))
                        .on_click(move |_event, cx| {
                            cx.dispatch_action(OpenHistory.boxed_clone());
                        }),
                ),
            )
    }

    fn render_past_thread(
        &self,
        thread: Model<Thread>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let id = thread.read(cx).id().clone();

        ListItem::new(("past-thread", thread.entity_id()))
            .start_slot(Icon::new(IconName::MessageBubbles))
            .child(Label::new(format!("Thread {id}")))
            .end_slot(
                h_flex()
                    .gap_2()
                    .child(Label::new("1 hour ago").color(Color::Disabled))
                    .child(
                        IconButton::new("delete", IconName::TrashAlt)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small),
                    ),
            )
            .on_click(cx.listener(move |this, _event, cx| {
                this.open_thread(&id, cx);
            }))
    }

    fn render_last_error(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        let last_error = self.thread.as_ref()?.read(cx).last_error()?;

        Some(
            div()
                .absolute()
                .right_3()
                .bottom_12()
                .max_w_96()
                .py_2()
                .px_3()
                .elevation_2(cx)
                .occlude()
                .child(match last_error {
                    ThreadError::PaymentRequired => self.render_payment_required_error(cx),
                    ThreadError::MaxMonthlySpendReached => {
                        self.render_max_monthly_spend_reached_error(cx)
                    }
                    ThreadError::Message(error_message) => {
                        self.render_error_message(&error_message, cx)
                    }
                })
                .into_any(),
        )
    }

    fn render_payment_required_error(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        const ERROR_MESSAGE: &str = "Free tier exceeded. Subscribe and add payment to continue using Zed LLMs. You'll be billed at cost for tokens used.";

        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new("Free Usage Exceeded").weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(ERROR_MESSAGE)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("subscribe", "Subscribe").on_click(cx.listener(
                        |this, _, cx| {
                            if let Some(thread) = this.thread.as_ref() {
                                thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });
                            }

                            cx.open_url(&zed_urls::account_url(cx));
                            cx.notify();
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            if let Some(thread) = this.thread.as_ref() {
                                thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });
                            }

                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_max_monthly_spend_reached_error(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        const ERROR_MESSAGE: &str = "You have reached your maximum monthly spend. Increase your spend limit to continue using Zed LLMs.";

        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new("Max Monthly Spend Reached").weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(ERROR_MESSAGE)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(
                        Button::new("subscribe", "Update Monthly Spend Limit").on_click(
                            cx.listener(|this, _, cx| {
                                if let Some(thread) = this.thread.as_ref() {
                                    thread.update(cx, |this, _cx| {
                                        this.clear_last_error();
                                    });
                                }

                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            }),
                        ),
                    )
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            if let Some(thread) = this.thread.as_ref() {
                                thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });
                            }

                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_error_message(
        &self,
        error_message: &SharedString,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(
                        Label::new("Error interacting with language model")
                            .weight(FontWeight::MEDIUM),
                    ),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_32()
                    .overflow_y_scroll()
                    .child(Label::new(error_message.clone())),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            if let Some(thread) = this.thread.as_ref() {
                                thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });
                            }

                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AssistantPanel2")
            .justify_between()
            .size_full()
            .on_action(cx.listener(|this, _: &NewThread, cx| {
                this.new_thread(cx);
            }))
            .on_action(cx.listener(|_this, _: &OpenHistory, _cx| {
                println!("Open History");
            }))
            .child(self.render_toolbar(cx))
            .child(self.render_active_thread_or_empty_state(cx))
            .child(
                h_flex()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.message_editor.clone()),
            )
            .children(self.render_last_error(cx))
    }
}

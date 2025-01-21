use std::sync::Arc;

use anyhow::Result;
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_tool::ToolWorkingSet;
use client::zed_urls;
use fs::Fs;
use gpui::{
    prelude::*, px, svg, Action, AnyElement, AppContext, AsyncWindowContext, EventEmitter,
    FocusHandle, FocusableView, FontWeight, Model, Pixels, Task, View, ViewContext, WeakView,
    WindowContext,
};
use language::LanguageRegistry;
use settings::Settings;
use time::UtcOffset;
use ui::{prelude::*, KeyBinding, Tab, Tooltip};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

use crate::active_thread::ActiveThread;
use crate::message_editor::MessageEditor;
use crate::thread::{Thread, ThreadError, ThreadId};
use crate::thread_history::{PastThread, ThreadHistory};
use crate::thread_store::ThreadStore;
use crate::{NewThread, OpenHistory, ToggleFocus};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    workspace.toggle_panel_focus::<AssistantPanel>(cx);
                })
                .register_action(|workspace, _: &NewThread, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.new_thread(cx));
                        workspace.focus_panel::<AssistantPanel>(cx);
                    }
                })
                .register_action(|workspace, _: &OpenHistory, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(cx);
                        panel.update(cx, |panel, cx| panel.open_history(cx));
                    }
                });
        },
    )
    .detach();
}

enum ActiveView {
    Thread,
    History,
}

pub struct AssistantPanel {
    workspace: WeakView<Workspace>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Model<ThreadStore>,
    thread: View<ActiveThread>,
    message_editor: View<MessageEditor>,
    tools: Arc<ToolWorkingSet>,
    local_timezone: UtcOffset,
    active_view: ActiveView,
    history: View<ThreadHistory>,
    width: Option<Pixels>,
    height: Option<Pixels>,
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
        let fs = workspace.app_state().fs.clone();
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace = workspace.weak_handle();
        let weak_self = cx.view().downgrade();

        let message_editor = cx.new_view(|cx| {
            MessageEditor::new(
                fs.clone(),
                workspace.clone(),
                thread_store.downgrade(),
                thread.clone(),
                cx,
            )
        });

        Self {
            active_view: ActiveView::Thread,
            workspace: workspace.clone(),
            fs: fs.clone(),
            language_registry: language_registry.clone(),
            thread_store: thread_store.clone(),
            thread: cx.new_view(|cx| {
                ActiveThread::new(
                    thread.clone(),
                    workspace,
                    language_registry,
                    tools.clone(),
                    cx,
                )
            }),
            message_editor,
            tools,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            history: cx.new_view(|cx| ThreadHistory::new(weak_self, thread_store, cx)),
            width: None,
            height: None,
        }
    }

    pub(crate) fn local_timezone(&self) -> UtcOffset {
        self.local_timezone
    }

    pub(crate) fn thread_store(&self) -> &Model<ThreadStore> {
        &self.thread_store
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        self.thread
            .update(cx, |thread, cx| thread.cancel_last_completion(cx));
    }

    fn new_thread(&mut self, cx: &mut ViewContext<Self>) {
        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(cx));

        self.active_view = ActiveView::Thread;
        self.thread = cx.new_view(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.tools.clone(),
                cx,
            )
        });
        self.message_editor = cx.new_view(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                self.thread_store.downgrade(),
                thread,
                cx,
            )
        });
        self.message_editor.focus_handle(cx).focus(cx);
    }

    fn open_history(&mut self, cx: &mut ViewContext<Self>) {
        self.active_view = ActiveView::History;
        self.history.focus_handle(cx).focus(cx);
        cx.notify();
    }

    pub(crate) fn open_thread(&mut self, thread_id: &ThreadId, cx: &mut ViewContext<Self>) {
        let Some(thread) = self
            .thread_store
            .update(cx, |this, cx| this.open_thread(thread_id, cx))
        else {
            return;
        };

        self.active_view = ActiveView::Thread;
        self.thread = cx.new_view(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.tools.clone(),
                cx,
            )
        });
        self.message_editor = cx.new_view(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                self.thread_store.downgrade(),
                thread,
                cx,
            )
        });
        self.message_editor.focus_handle(cx).focus(cx);
    }

    pub(crate) fn active_thread(&self, cx: &AppContext) -> Model<Thread> {
        self.thread.read(cx).thread.clone()
    }

    pub(crate) fn delete_thread(&mut self, thread_id: &ThreadId, cx: &mut ViewContext<Self>) {
        self.thread_store
            .update(cx, |this, cx| this.delete_thread(thread_id, cx));
    }
}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        match self.active_view {
            ActiveView::Thread => self.message_editor.focus_handle(cx),
            ActiveView::History => self.history.focus_handle(cx),
        }
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

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<AssistantSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left => AssistantDockPosition::Left,
                    DockPosition::Bottom => AssistantDockPosition::Bottom,
                    DockPosition::Right => AssistantDockPosition::Right,
                };
                settings.set_dock(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        let settings = AssistantSettings::get_global(cx);
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
    }

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled || !settings.button {
            return None;
        }

        Some(IconName::ZedAssistant2)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        3
    }
}

impl AssistantPanel {
    fn render_toolbar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        let thread = self.thread.read(cx);

        let title = if thread.is_empty() {
            thread.summary_or_default(cx)
        } else {
            thread
                .summary(cx)
                .unwrap_or_else(|| SharedString::from("Loading Summary…"))
        };

        h_flex()
            .id("assistant-toolbar")
            .px(DynamicSpacing::Base08.rems(cx))
            .h(Tab::container_height(cx))
            .flex_none()
            .justify_between()
            .gap(DynamicSpacing::Base08.rems(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(h_flex().child(Label::new(title)))
            .child(
                h_flex()
                    .h_full()
                    .pl_1p5()
                    .border_l_1()
                    .border_color(cx.theme().colors().border)
                    .gap(DynamicSpacing::Base02.rems(cx))
                    .child(
                        IconButton::new("new-thread", IconName::Plus)
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
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip(move |cx| Tooltip::text("Configure Assistant", cx))
                            .on_click(move |_event, _cx| {
                                println!("Configure Assistant");
                            }),
                    ),
            )
    }

    fn render_active_thread_or_empty_state(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        if self.thread.read(cx).is_empty() {
            return self.render_thread_empty_state(cx).into_any_element();
        }

        self.thread.clone().into_any()
    }

    fn render_thread_empty_state(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let recent_threads = self
            .thread_store
            .update(cx, |this, cx| this.recent_threads(3, cx));

        v_flex()
            .gap_2()
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
            .when(!recent_threads.is_empty(), |parent| {
                parent
                    .child(
                        h_flex().w_full().justify_center().child(
                            Label::new("Recent Threads:")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(v_flex().mx_auto().w_4_5().gap_2().children(
                        recent_threads.into_iter().map(|thread| {
                            // TODO: keyboard navigation
                            PastThread::new(thread, cx.view().downgrade(), false)
                        }),
                    ))
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
            })
    }

    fn render_last_error(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        let last_error = self.thread.read(cx).last_error()?;

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
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

                            cx.open_url(&zed_urls::account_url(cx));
                            cx.notify();
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

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
                                this.thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });

                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            }),
                        ),
                    )
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

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
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

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
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|this, _: &NewThread, cx| {
                this.new_thread(cx);
            }))
            .on_action(cx.listener(|this, _: &OpenHistory, cx| {
                this.open_history(cx);
            }))
            .child(self.render_toolbar(cx))
            .map(|parent| match self.active_view {
                ActiveView::Thread => parent
                    .child(self.render_active_thread_or_empty_state(cx))
                    .child(
                        h_flex()
                            .border_t_1()
                            .border_color(cx.theme().colors().border)
                            .child(self.message_editor.clone()),
                    )
                    .children(self.render_last_error(cx)),
                ActiveView::History => parent.child(self.history.clone()),
            })
    }
}

use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use client::zed_urls;
use fs::Fs;
use gpui::{Window, ModelContext, 
    prelude::*, px, svg, Action, AnyElement, AppContext, AsyncWindowContext, EventEmitter,
    FocusHandle, FocusableView, FontWeight, Model, Pixels, Task,   WeakView,
    
};
use language::LanguageRegistry;
use settings::Settings;
use time::UtcOffset;
use ui::{prelude::*, KeyBinding, Tab, Tooltip};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

use crate::active_thread::ActiveThread;
use crate::assistant_settings::{AssistantDockPosition, AssistantSettings};
use crate::message_editor::MessageEditor;
use crate::thread::{ThreadError, ThreadId};
use crate::thread_history::{PastThread, ThreadHistory};
use crate::thread_store::ThreadStore;
use crate::{NewThread, OpenHistory, ToggleFocus};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _window: &mut Window, _cx: &mut ModelContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, window, cx| {
                    workspace.toggle_panel_focus::<AssistantPanel>(window, cx);
                })
                .register_action(|workspace, _: &NewThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(window, cx) {
                        panel.update(cx, |panel, cx| panel.new_thread(window, cx));
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                    }
                })
                .register_action(|workspace, _: &OpenHistory, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(window, cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_history(window, cx));
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
    thread: Model<ActiveThread>,
    message_editor: Model<MessageEditor>,
    tools: Arc<ToolWorkingSet>,
    local_timezone: UtcOffset,
    active_view: ActiveView,
    history: Model<ThreadHistory>,
    width: Option<Pixels>,
    height: Option<Pixels>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        window: &mut Window, cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            let tools = Arc::new(ToolWorkingSet::default());
            let thread_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    ThreadStore::new(project, tools.clone(), cx)
                })?
                .await?;

            workspace.update(&mut cx, |workspace, cx| {
                window.new_view(cx, |cx| Self::new(workspace, thread_store, tools, window, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Model<ThreadStore>,
        tools: Arc<ToolWorkingSet>,
        window: &mut Window, cx: &mut ModelContext<Self>,
    ) -> Self {
        let thread = thread_store.update(cx, |this, cx| this.create_thread(cx));
        let fs = workspace.app_state().fs.clone();
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace = workspace.weak_handle();
        let weak_self = cx.view().downgrade();

        Self {
            active_view: ActiveView::Thread,
            workspace: workspace.clone(),
            fs: fs.clone(),
            language_registry: language_registry.clone(),
            thread_store: thread_store.clone(),
            thread: window.new_view(cx, |cx| {
                ActiveThread::new(
                    thread.clone(),
                    workspace.clone(),
                    language_registry,
                    tools.clone(),
                    window, cx,
                )
            }),
            message_editor: window.new_view(cx, |cx| {
                MessageEditor::new(
                    fs.clone(),
                    workspace,
                    thread_store.downgrade(),
                    thread.clone(),
                    window, cx,
                )
            }),
            tools,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            history: window.new_view(cx, |cx| ThreadHistory::new(weak_self, thread_store, window, cx)),
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

    fn new_thread(&mut self, window: &mut Window, cx: &mut ModelContext<Self>) {
        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(cx));

        self.active_view = ActiveView::Thread;
        self.thread = window.new_view(cx, |cx| {
            ActiveThread::new(
                thread.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.tools.clone(),
                window, cx,
            )
        });
        self.message_editor = window.new_view(cx, |cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                self.thread_store.downgrade(),
                thread,
                window, cx,
            )
        });
        self.message_editor.focus_handle(cx).focus(window);
    }

    fn open_history(&mut self, window: &mut Window, cx: &mut ModelContext<Self>) {
        self.active_view = ActiveView::History;
        self.history.focus_handle(cx).focus(window);
        cx.notify();
    }

    pub(crate) fn open_thread(&mut self, thread_id: &ThreadId, window: &mut Window, cx: &mut ModelContext<Self>) {
        let Some(thread) = self
            .thread_store
            .update(cx, |this, cx| this.open_thread(thread_id, cx))
        else {
            return;
        };

        self.active_view = ActiveView::Thread;
        self.thread = window.new_view(cx, |cx| {
            ActiveThread::new(
                thread.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.tools.clone(),
                window, cx,
            )
        });
        self.message_editor = window.new_view(cx, |cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                self.thread_store.downgrade(),
                thread,
                window, cx,
            )
        });
        self.message_editor.focus_handle(cx).focus(window);
    }

    pub(crate) fn delete_thread(&mut self, thread_id: &ThreadId, window: &mut Window, cx: &mut ModelContext<Self>) {
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

    fn position(&self, _window: &mut Window, _cx: &mut AppContext) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, window: &mut Window, cx: &mut ModelContext<Self>) {
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

    fn size(&self, window: &mut Window, cx: &mut AppContext) -> Pixels {
        let settings = AssistantSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut ModelContext<Self>) {
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
    }

    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut ModelContext<Self>) {}

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _window: &mut Window, _cx: &mut AppContext) -> Option<IconName> {
        Some(IconName::ZedAssistant2)
    }

    fn icon_tooltip(&self, _window: &mut Window, _cx: &mut AppContext) -> Option<&'static str> {
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
    fn render_toolbar(&self, window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        h_flex()
            .id("assistant-toolbar")
            .justify_between()
            .gap(DynamicSpacing::Base08.rems(cx))
            .h(Tab::container_height(window, cx))
            .px(DynamicSpacing::Base08.rems(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(h_flex().children(self.thread.read(cx).summary(cx).map(Label::new)))
            .child(
                h_flex()
                    .h_full()
                    .pl_1()
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
                                        window, cx,
                                    )
                                }
                            })
                            .on_click(move |_event, window, cx| {
                                window.dispatch_action(NewThread.boxed_clone(), cx);
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
                                        window, cx,
                                    )
                                }
                            })
                            .on_click(move |_event, window, cx| {
                                window.dispatch_action(OpenHistory.boxed_clone(), cx);
                            }),
                    )
                    .child(
                        IconButton::new("configure-assistant", IconName::Settings)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip(move |window, cx| Tooltip::text("Configure Assistant", window, cx))
                            .on_click(move |_event, _window, _cx| {
                                println!("Configure Assistant");
                            }),
                    ),
            )
    }

    fn render_active_thread_or_empty_state(&self, window: &mut Window, cx: &mut ModelContext<Self>) -> AnyElement {
        if self.thread.read(cx).is_empty() {
            return self.render_thread_empty_state(window, cx).into_any_element();
        }

        self.thread.clone().into_any()
    }

    fn render_thread_empty_state(&self, window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
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
                    .child(
                        v_flex().mx_auto().w_4_5().gap_2().children(
                            recent_threads
                                .into_iter()
                                .map(|thread| PastThread::new(thread, cx.view().downgrade())),
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
                                    window, cx,
                                ))
                                .on_click(move |_event, window, cx| {
                                    window.dispatch_action(OpenHistory.boxed_clone(), cx);
                                }),
                        ),
                    )
            })
    }

    fn render_last_error(&self, window: &mut Window, cx: &mut ModelContext<Self>) -> Option<AnyElement> {
        let last_error = self.thread.read(cx).last_error()?;

        Some(
            div()
                .absolute()
                .right_3()
                .bottom_12()
                .max_w_96()
                .py_2()
                .px_3()
                .elevation_2(window, cx)
                .occlude()
                .child(match last_error {
                    ThreadError::PaymentRequired => self.render_payment_required_error(window, cx),
                    ThreadError::MaxMonthlySpendReached => {
                        self.render_max_monthly_spend_reached_error(window, cx)
                    }
                    ThreadError::Message(error_message) => {
                        self.render_error_message(&error_message, window, cx)
                    }
                })
                .into_any(),
        )
    }

    fn render_payment_required_error(&self, window: &mut Window, cx: &mut ModelContext<Self>) -> AnyElement {
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
                        |this, _, window, cx| {
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

                            cx.open_url(&zed_urls::account_url(cx));
                            cx.notify();
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, window, cx| {
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_max_monthly_spend_reached_error(&self, window: &mut Window, cx: &mut ModelContext<Self>) -> AnyElement {
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
                            cx.listener(|this, _, window, cx| {
                                this.thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });

                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            }),
                        ),
                    )
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, window, cx| {
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
        window: &mut Window, cx: &mut ModelContext<Self>,
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
                        |this, _, window, cx| {
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
    fn render(&mut self, window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AssistantPanel2")
            .justify_between()
            .size_full()
            .on_action(cx.listener(|this, _: &NewThread, window, cx| {
                this.new_thread(window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenHistory, window, cx| {
                this.open_history(window, cx);
            }))
            .child(self.render_toolbar(window, cx))
            .map(|parent| match self.active_view {
                ActiveView::Thread => parent
                    .child(self.render_active_thread_or_empty_state(window, cx))
                    .child(
                        h_flex()
                            .border_t_1()
                            .border_color(cx.theme().colors().border)
                            .child(self.message_editor.clone()),
                    )
                    .children(self.render_last_error(window, cx)),
                ActiveView::History => parent.child(self.history.clone()),
            })
    }
}

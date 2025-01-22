use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_context_editor::{
    make_lsp_adapter_delegate, AssistantPanelDelegate, ContextEditor, ContextHistory,
};
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_tool::ToolWorkingSet;
use client::zed_urls;
use fs::Fs;
use gpui::{
    prelude::*, px, svg, Action, AnyElement, AppContext, AsyncWindowContext, Corner, EventEmitter,
    FocusHandle, FocusableView, FontWeight, Model, Pixels, Task, View, ViewContext, WeakView,
    WindowContext,
};
use language::LanguageRegistry;
use project::Project;
use prompt_library::PromptBuilder;
use settings::Settings;
use time::UtcOffset;
use ui::{prelude::*, ContextMenu, KeyBinding, PopoverMenu, PopoverMenuHandle, Tab, Tooltip};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;
use zed_actions::assistant::ToggleFocus;

use crate::active_thread::ActiveThread;
use crate::message_editor::MessageEditor;
use crate::thread::{Thread, ThreadError, ThreadId};
use crate::thread_history::{PastThread, ThreadHistory};
use crate::thread_store::ThreadStore;
use crate::{NewPromptEditor, NewThread, OpenHistory, OpenPromptEditorHistory};

pub fn init(cx: &mut AppContext) {
    <dyn AssistantPanelDelegate>::set_global(Arc::new(ConcreteAssistantPanelDelegate), cx);
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
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
                })
                .register_action(|workspace, _: &NewPromptEditor, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(cx);
                        panel.update(cx, |panel, cx| panel.new_prompt_editor(cx));
                    }
                })
                .register_action(|workspace, _: &OpenPromptEditorHistory, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(cx);
                        panel.update(cx, |panel, cx| panel.open_prompt_editor_history(cx));
                    }
                });
        },
    )
    .detach();
}

enum ActiveView {
    Thread,
    PromptEditor,
    History,
    PromptEditorHistory,
}

pub struct AssistantPanel {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Model<ThreadStore>,
    thread: View<ActiveThread>,
    message_editor: View<MessageEditor>,
    context_store: Model<assistant_context_editor::ContextStore>,
    context_editor: Option<View<ContextEditor>>,
    context_history: Option<View<ContextHistory>>,
    tools: Arc<ToolWorkingSet>,
    local_timezone: UtcOffset,
    active_view: ActiveView,
    history: View<ThreadHistory>,
    new_item_context_menu_handle: PopoverMenuHandle<ContextMenu>,
    open_history_context_menu_handle: PopoverMenuHandle<ContextMenu>,
    width: Option<Pixels>,
    height: Option<Pixels>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
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

            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            let context_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    assistant_context_editor::ContextStore::new(
                        project,
                        prompt_builder.clone(),
                        slash_commands,
                        tools.clone(),
                        cx,
                    )
                })?
                .await?;

            workspace.update(&mut cx, |workspace, cx| {
                cx.new_view(|cx| Self::new(workspace, thread_store, context_store, tools, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Model<ThreadStore>,
        context_store: Model<assistant_context_editor::ContextStore>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let thread = thread_store.update(cx, |this, cx| this.create_thread(cx));
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let language_registry = project.read(cx).languages().clone();
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
            project,
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
            context_store,
            context_editor: None,
            context_history: None,
            tools,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            history: cx.new_view(|cx| ThreadHistory::new(weak_self, thread_store, cx)),
            new_item_context_menu_handle: PopoverMenuHandle::default(),
            open_history_context_menu_handle: PopoverMenuHandle::default(),
            width: None,
            height: None,
        }
    }

    pub fn toggle_focus(
        workspace: &mut Workspace,
        _: &ToggleFocus,
        cx: &mut ViewContext<Workspace>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        workspace.toggle_panel_focus::<Self>(cx);
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

    fn new_prompt_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.active_view = ActiveView::PromptEditor;

        let context = self
            .context_store
            .update(cx, |context_store, cx| context_store.create(cx));
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        self.context_editor = Some(cx.new_view(|cx| {
            ContextEditor::for_context(
                context,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                cx,
            )
        }));

        if let Some(context_editor) = self.context_editor.as_ref() {
            context_editor.focus_handle(cx).focus(cx);
        }
    }

    fn open_history(&mut self, cx: &mut ViewContext<Self>) {
        self.active_view = ActiveView::History;
        self.history.focus_handle(cx).focus(cx);
        cx.notify();
    }

    fn open_prompt_editor_history(&mut self, cx: &mut ViewContext<Self>) {
        self.active_view = ActiveView::PromptEditorHistory;
        self.context_history = Some(cx.new_view(|cx| {
            ContextHistory::new(
                self.project.clone(),
                self.context_store.clone(),
                self.workspace.clone(),
                cx,
            )
        }));

        if let Some(context_history) = self.context_history.as_ref() {
            context_history.focus_handle(cx).focus(cx);
        }

        cx.notify();
    }

    fn open_saved_prompt_editor(
        &mut self,
        path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let context = self
            .context_store
            .update(cx, |store, cx| store.open_local_context(path.clone(), cx));
        let fs = self.fs.clone();
        let project = self.project.clone();
        let workspace = self.workspace.clone();

        let lsp_adapter_delegate = make_lsp_adapter_delegate(&project, cx).log_err().flatten();

        cx.spawn(|this, mut cx| async move {
            let context = context.await?;
            this.update(&mut cx, |this, cx| {
                let editor = cx.new_view(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        project,
                        lsp_adapter_delegate,
                        cx,
                    )
                });
                this.active_view = ActiveView::PromptEditor;
                this.context_editor = Some(editor);

                anyhow::Ok(())
            })??;
            Ok(())
        })
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
            ActiveView::PromptEditor => {
                if let Some(context_editor) = self.context_editor.as_ref() {
                    context_editor.focus_handle(cx)
                } else {
                    cx.focus_handle()
                }
            }
            ActiveView::PromptEditorHistory => {
                if let Some(context_history) = self.context_history.as_ref() {
                    context_history.focus_handle(cx)
                } else {
                    cx.focus_handle()
                }
            }
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

        Some(IconName::ZedAssistant)
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
        let thread = self.thread.read(cx);

        let title = match self.active_view {
            ActiveView::Thread => {
                if thread.is_empty() {
                    thread.summary_or_default(cx)
                } else {
                    thread
                        .summary(cx)
                        .unwrap_or_else(|| SharedString::from("Loading Summary…"))
                }
            }
            ActiveView::PromptEditor => self
                .context_editor
                .as_ref()
                .map(|context_editor| {
                    SharedString::from(context_editor.read(cx).title(cx).to_string())
                })
                .unwrap_or_else(|| SharedString::from("Loading Summary…")),
            ActiveView::History => "History / Thread".into(),
            ActiveView::PromptEditorHistory => "History / Prompt Editor".into(),
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
                        PopoverMenu::new("assistant-toolbar-new-popover-menu")
                            .trigger(
                                IconButton::new("new", IconName::Plus)
                                    .icon_size(IconSize::Small)
                                    .style(ButtonStyle::Subtle)
                                    .tooltip(|cx| Tooltip::text("New…", cx)),
                            )
                            .anchor(Corner::TopRight)
                            .with_handle(self.new_item_context_menu_handle.clone())
                            .menu(move |cx| {
                                Some(ContextMenu::build(cx, |menu, _| {
                                    menu.action("New Thread", NewThread.boxed_clone())
                                        .action("New Prompt Editor", NewPromptEditor.boxed_clone())
                                }))
                            }),
                    )
                    .child(
                        PopoverMenu::new("assistant-toolbar-history-popover-menu")
                            .trigger(
                                IconButton::new("open-history", IconName::HistoryRerun)
                                    .icon_size(IconSize::Small)
                                    .style(ButtonStyle::Subtle)
                                    .tooltip(|cx| Tooltip::text("History…", cx)),
                            )
                            .anchor(Corner::TopRight)
                            .with_handle(self.open_history_context_menu_handle.clone())
                            .menu(move |cx| {
                                Some(ContextMenu::build(cx, |menu, _| {
                                    menu.action("Thread History", OpenHistory.boxed_clone())
                                        .action(
                                            "Prompt Editor History",
                                            OpenPromptEditorHistory.boxed_clone(),
                                        )
                                }))
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
                ActiveView::PromptEditor => parent.children(self.context_editor.clone()),
                ActiveView::PromptEditorHistory => parent.children(self.context_history.clone()),
            })
    }
}

struct ConcreteAssistantPanelDelegate;

impl AssistantPanelDelegate for ConcreteAssistantPanelDelegate {
    fn active_context_editor(
        &self,
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<View<ContextEditor>> {
        let panel = workspace.panel::<AssistantPanel>(cx)?;
        panel.update(cx, |panel, _cx| panel.context_editor.clone())
    }

    fn open_saved_context(
        &self,
        workspace: &mut Workspace,
        path: std::path::PathBuf,
        cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<()>> {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return Task::ready(Err(anyhow!("Assistant panel not found")));
        };

        panel.update(cx, |panel, cx| panel.open_saved_prompt_editor(path, cx))
    }

    fn open_remote_context(
        &self,
        _workspace: &mut Workspace,
        _context_id: assistant_context_editor::ContextId,
        _cx: &mut ViewContext<Workspace>,
    ) -> Task<Result<View<ContextEditor>>> {
        Task::ready(Err(anyhow!("opening remote context not implemented")))
    }

    fn quote_selection(
        &self,
        _workspace: &mut Workspace,
        _creases: Vec<(String, String)>,
        _cx: &mut ViewContext<Workspace>,
    ) {
    }
}

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Result, anyhow};
use assistant_context_editor::{
    AssistantPanelDelegate, ConfigurationError, ContextEditor, SlashCommandCompletionProvider,
    make_lsp_adapter_delegate, render_remaining_tokens,
};
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_tool::ToolWorkingSet;

use client::zed_urls;
use editor::{Editor, MultiBuffer};
use fs::Fs;
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Corner, Entity, EventEmitter, FocusHandle,
    Focusable, FontWeight, KeyContext, Pixels, Subscription, Task, UpdateGlobal, WeakEntity,
    action_with_deprecated_aliases, prelude::*,
};
use language::LanguageRegistry;
use language_model::{LanguageModelProviderTosView, LanguageModelRegistry};
use language_model_selector::ToggleModelSelector;
use project::Project;
use prompt_library::{PromptLibrary, open_prompt_library};
use prompt_store::PromptBuilder;
use settings::{Settings, update_settings_file};
use time::UtcOffset;
use ui::{
    Banner, ContextMenu, KeyBinding, PopoverMenu, PopoverMenuHandle, Tab, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::Workspace;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use zed_actions::assistant::ToggleFocus;

use crate::active_thread::ActiveThread;
use crate::assistant_configuration::{AssistantConfiguration, AssistantConfigurationEvent};
use crate::history_store::{HistoryEntry, HistoryStore};
use crate::message_editor::MessageEditor;
use crate::thread::{Thread, ThreadError, ThreadId};
use crate::thread_history::{PastContext, PastThread, ThreadHistory};
use crate::thread_store::ThreadStore;
use crate::{
    AgentDiff, InlineAssistant, NewPromptEditor, NewThread, OpenActiveThreadAsMarkdown,
    OpenAgentDiff, OpenConfiguration, OpenHistory, ToggleContextPicker,
};

action_with_deprecated_aliases!(
    assistant,
    OpenPromptLibrary,
    ["assistant::DeployPromptLibrary"]
);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, action: &NewThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.new_thread(action, window, cx));
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                    }
                })
                .register_action(|workspace, _: &OpenHistory, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_history(window, cx));
                    }
                })
                .register_action(|workspace, _: &OpenConfiguration, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_configuration(window, cx));
                    }
                })
                .register_action(|workspace, _: &NewPromptEditor, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.new_prompt_editor(window, cx));
                    }
                })
                .register_action(|workspace, _: &OpenPromptLibrary, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.deploy_prompt_library(&OpenPromptLibrary, window, cx)
                        });
                    }
                })
                .register_action(|workspace, _: &OpenAgentDiff, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.open_agent_diff(&OpenAgentDiff, window, cx);
                        });
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
    Configuration,
}

pub struct AssistantPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    thread: Entity<ActiveThread>,
    message_editor: Entity<MessageEditor>,
    context_store: Entity<assistant_context_editor::ContextStore>,
    context_editor: Option<Entity<ContextEditor>>,
    configuration: Option<Entity<AssistantConfiguration>>,
    configuration_subscription: Option<Subscription>,
    local_timezone: UtcOffset,
    active_view: ActiveView,
    history_store: Entity<HistoryStore>,
    history: Entity<ThreadHistory>,
    assistant_dropdown_menu_handle: PopoverMenuHandle<ContextMenu>,
    width: Option<Pixels>,
    height: Option<Pixels>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            let tools = Arc::new(ToolWorkingSet::default());
            let thread_store = workspace.update(cx, |workspace, cx| {
                let project = workspace.project().clone();
                ThreadStore::new(project, tools.clone(), prompt_builder.clone(), cx)
            })??;

            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            let context_store = workspace
                .update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    assistant_context_editor::ContextStore::new(
                        project,
                        prompt_builder.clone(),
                        slash_commands,
                        cx,
                    )
                })?
                .await?;

            workspace.update_in(cx, |workspace, window, cx| {
                cx.new(|cx| Self::new(workspace, thread_store, context_store, window, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Entity<ThreadStore>,
        context_store: Entity<assistant_context_editor::ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let thread = thread_store.update(cx, |this, cx| this.create_thread(cx));
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let language_registry = project.read(cx).languages().clone();
        let workspace = workspace.weak_handle();
        let weak_self = cx.entity().downgrade();

        let message_editor_context_store = cx.new(|_cx| {
            crate::context_store::ContextStore::new(
                workspace.clone(),
                Some(thread_store.downgrade()),
            )
        });

        let message_editor = cx.new(|cx| {
            MessageEditor::new(
                fs.clone(),
                workspace.clone(),
                message_editor_context_store.clone(),
                thread_store.downgrade(),
                thread.clone(),
                window,
                cx,
            )
        });

        let history_store =
            cx.new(|cx| HistoryStore::new(thread_store.clone(), context_store.clone(), cx));

        let thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                thread_store.clone(),
                language_registry.clone(),
                message_editor_context_store.clone(),
                workspace.clone(),
                window,
                cx,
            )
        });

        Self {
            active_view: ActiveView::Thread,
            workspace,
            project: project.clone(),
            fs: fs.clone(),
            language_registry,
            thread_store: thread_store.clone(),
            thread,
            message_editor,
            context_store,
            context_editor: None,
            configuration: None,
            configuration_subscription: None,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            history_store: history_store.clone(),
            history: cx.new(|cx| ThreadHistory::new(weak_self, history_store, cx)),
            assistant_dropdown_menu_handle: PopoverMenuHandle::default(),
            width: None,
            height: None,
        }
    }

    pub fn toggle_focus(
        workspace: &mut Workspace,
        _: &ToggleFocus,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if workspace
            .panel::<Self>(cx)
            .is_some_and(|panel| panel.read(cx).enabled(cx))
        {
            workspace.toggle_panel_focus::<Self>(window, cx);
        }
    }

    pub(crate) fn local_timezone(&self) -> UtcOffset {
        self.local_timezone
    }

    pub(crate) fn thread_store(&self) -> &Entity<ThreadStore> {
        &self.thread_store
    }

    fn cancel(
        &mut self,
        _: &editor::actions::Cancel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread
            .update(cx, |thread, cx| thread.cancel_last_completion(cx));
    }

    fn new_thread(&mut self, action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(cx));

        self.active_view = ActiveView::Thread;

        let message_editor_context_store = cx.new(|_cx| {
            crate::context_store::ContextStore::new(
                self.workspace.clone(),
                Some(self.thread_store.downgrade()),
            )
        });

        if let Some(other_thread_id) = action.from_thread_id.clone() {
            let other_thread_task = self
                .thread_store
                .update(cx, |this, cx| this.open_thread(&other_thread_id, cx));

            cx.spawn({
                let context_store = message_editor_context_store.clone();

                async move |_panel, cx| {
                    let other_thread = other_thread_task.await?;

                    context_store.update(cx, |this, cx| {
                        this.add_thread(other_thread, false, cx);
                    })?;
                    anyhow::Ok(())
                }
            })
            .detach_and_log_err(cx);
        }

        self.thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.thread_store.clone(),
                self.language_registry.clone(),
                message_editor_context_store.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        });
        self.message_editor = cx.new(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                message_editor_context_store,
                self.thread_store.downgrade(),
                thread,
                window,
                cx,
            )
        });
        self.message_editor.focus_handle(cx).focus(window);
    }

    fn new_prompt_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active_view = ActiveView::PromptEditor;

        let context = self
            .context_store
            .update(cx, |context_store, cx| context_store.create(cx));
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        self.context_editor = Some(cx.new(|cx| {
            let mut editor = ContextEditor::for_context(
                context,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                window,
                cx,
            );
            editor.insert_default_prompt(window, cx);
            editor
        }));

        if let Some(context_editor) = self.context_editor.as_ref() {
            context_editor.focus_handle(cx).focus(window);
        }
    }

    fn deploy_prompt_library(
        &mut self,
        _: &OpenPromptLibrary,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        open_prompt_library(
            self.language_registry.clone(),
            Box::new(PromptLibraryInlineAssist::new(self.workspace.clone())),
            Arc::new(|| {
                Box::new(SlashCommandCompletionProvider::new(
                    Arc::new(SlashCommandWorkingSet::default()),
                    None,
                    None,
                ))
            }),
            cx,
        )
        .detach_and_log_err(cx);
    }

    fn open_history(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.thread_store
            .update(cx, |thread_store, cx| thread_store.reload(cx))
            .detach_and_log_err(cx);
        self.active_view = ActiveView::History;
        self.history.focus_handle(cx).focus(window);
        cx.notify();
    }

    pub(crate) fn open_saved_prompt_editor(
        &mut self,
        path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let context = self
            .context_store
            .update(cx, |store, cx| store.open_local_context(path.clone(), cx));
        let fs = self.fs.clone();
        let project = self.project.clone();
        let workspace = self.workspace.clone();

        let lsp_adapter_delegate = make_lsp_adapter_delegate(&project, cx).log_err().flatten();

        cx.spawn_in(window, async move |this, cx| {
            let context = context.await?;
            this.update_in(cx, |this, window, cx| {
                let editor = cx.new(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        project,
                        lsp_adapter_delegate,
                        window,
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

    pub(crate) fn open_thread(
        &mut self,
        thread_id: &ThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let open_thread_task = self
            .thread_store
            .update(cx, |this, cx| this.open_thread(thread_id, cx));

        cx.spawn_in(window, async move |this, cx| {
            let thread = open_thread_task.await?;
            this.update_in(cx, |this, window, cx| {
                this.active_view = ActiveView::Thread;
                let message_editor_context_store = cx.new(|_cx| {
                    crate::context_store::ContextStore::new(
                        this.workspace.clone(),
                        Some(this.thread_store.downgrade()),
                    )
                });
                this.thread = cx.new(|cx| {
                    ActiveThread::new(
                        thread.clone(),
                        this.thread_store.clone(),
                        this.language_registry.clone(),
                        message_editor_context_store.clone(),
                        this.workspace.clone(),
                        window,
                        cx,
                    )
                });
                this.message_editor = cx.new(|cx| {
                    MessageEditor::new(
                        this.fs.clone(),
                        this.workspace.clone(),
                        message_editor_context_store,
                        this.thread_store.downgrade(),
                        thread,
                        window,
                        cx,
                    )
                });
                this.message_editor.focus_handle(cx).focus(window);
            })
        })
    }

    pub fn open_agent_diff(
        &mut self,
        _: &OpenAgentDiff,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread = self.thread.read(cx).thread().clone();
        AgentDiff::deploy(thread, self.workspace.clone(), window, cx).log_err();
    }

    pub(crate) fn open_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let context_server_manager = self.thread_store.read(cx).context_server_manager();
        let tools = self.thread_store.read(cx).tools();
        let fs = self.fs.clone();

        self.active_view = ActiveView::Configuration;
        self.configuration =
            Some(cx.new(|cx| {
                AssistantConfiguration::new(fs, context_server_manager, tools, window, cx)
            }));

        if let Some(configuration) = self.configuration.as_ref() {
            self.configuration_subscription = Some(cx.subscribe_in(
                configuration,
                window,
                Self::handle_assistant_configuration_event,
            ));

            configuration.focus_handle(cx).focus(window);
        }
    }

    pub(crate) fn open_active_thread_as_markdown(
        &mut self,
        _: &OpenActiveThreadAsMarkdown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self
            .workspace
            .upgrade()
            .ok_or_else(|| anyhow!("workspace dropped"))
            .log_err()
        else {
            return;
        };

        let markdown_language_task = workspace
            .read(cx)
            .app_state()
            .languages
            .language_for_name("Markdown");
        let thread = self.active_thread(cx);
        cx.spawn_in(window, async move |_this, cx| {
            let markdown_language = markdown_language_task.await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let thread = thread.read(cx);
                let markdown = thread.to_markdown(cx)?;
                let thread_summary = thread
                    .summary()
                    .map(|summary| summary.to_string())
                    .unwrap_or_else(|| "Thread".to_string());

                let project = workspace.project().clone();
                let buffer = project.update(cx, |project, cx| {
                    project.create_local_buffer(&markdown, Some(markdown_language), cx)
                });
                let buffer = cx.new(|cx| {
                    MultiBuffer::singleton(buffer, cx).with_title(thread_summary.clone())
                });

                workspace.add_item_to_active_pane(
                    Box::new(cx.new(|cx| {
                        let mut editor =
                            Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                        editor.set_breadcrumb_header(thread_summary);
                        editor
                    })),
                    None,
                    true,
                    window,
                    cx,
                );

                anyhow::Ok(())
            })
        })
        .detach_and_log_err(cx);
    }

    fn handle_assistant_configuration_event(
        &mut self,
        _entity: &Entity<AssistantConfiguration>,
        event: &AssistantConfigurationEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            AssistantConfigurationEvent::NewThread(provider) => {
                if LanguageModelRegistry::read_global(cx)
                    .active_provider()
                    .map_or(true, |active_provider| {
                        active_provider.id() != provider.id()
                    })
                {
                    if let Some(model) = provider.default_model(cx) {
                        update_settings_file::<AssistantSettings>(
                            self.fs.clone(),
                            cx,
                            move |settings, _| settings.set_model(model),
                        );
                    }
                }

                self.new_thread(&NewThread::default(), window, cx);
            }
        }
    }

    pub(crate) fn active_thread(&self, cx: &App) -> Entity<Thread> {
        self.thread.read(cx).thread().clone()
    }

    pub(crate) fn delete_thread(&mut self, thread_id: &ThreadId, cx: &mut Context<Self>) {
        self.thread_store
            .update(cx, |this, cx| this.delete_thread(thread_id, cx))
            .detach_and_log_err(cx);
    }

    pub(crate) fn active_context_editor(&self) -> Option<Entity<ContextEditor>> {
        self.context_editor.clone()
    }

    pub(crate) fn delete_context(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.context_store
            .update(cx, |this, cx| this.delete_local_context(path, cx))
            .detach_and_log_err(cx);
    }
}

impl Focusable for AssistantPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
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
            ActiveView::Configuration => {
                if let Some(configuration) = self.configuration.as_ref() {
                    configuration.focus_handle(cx)
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
        "AgentPanel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        match AssistantSettings::get_global(cx).dock {
            AssistantDockPosition::Left => DockPosition::Left,
            AssistantDockPosition::Bottom => DockPosition::Bottom,
            AssistantDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
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

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = AssistantSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
    }

    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        (self.enabled(cx) && AssistantSettings::get_global(cx).button)
            .then_some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Agent Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        3
    }

    fn enabled(&self, cx: &App) -> bool {
        AssistantSettings::get_global(cx).enabled
    }
}

impl AssistantPanel {
    fn render_toolbar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let thread = self.thread.read(cx);
        let is_empty = thread.is_empty();

        let thread_id = thread.thread().read(cx).id().clone();
        let focus_handle = self.focus_handle(cx);

        let title = match self.active_view {
            ActiveView::Thread => {
                if is_empty {
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
            ActiveView::History => "History".into(),
            ActiveView::Configuration => "Settings".into(),
        };

        h_flex()
            .id("assistant-toolbar")
            .h(Tab::container_height(cx))
            .flex_none()
            .justify_between()
            .gap(DynamicSpacing::Base08.rems(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .id("title")
                    .overflow_x_scroll()
                    .px(DynamicSpacing::Base08.rems(cx))
                    .child(Label::new(title).truncate()),
            )
            .child(
                h_flex()
                    .h_full()
                    .pl_2()
                    .gap_2()
                    .bg(cx.theme().colors().tab_bar_background)
                    .children(if matches!(self.active_view, ActiveView::PromptEditor) {
                        self.context_editor
                            .as_ref()
                            .and_then(|editor| render_remaining_tokens(editor, cx))
                    } else {
                        None
                    })
                    .child(
                        h_flex()
                            .h_full()
                            .px(DynamicSpacing::Base08.rems(cx))
                            .border_l_1()
                            .border_color(cx.theme().colors().border)
                            .gap(DynamicSpacing::Base02.rems(cx))
                            .child(
                                IconButton::new("new", IconName::Plus)
                                    .icon_size(IconSize::Small)
                                    .style(ButtonStyle::Subtle)
                                    .tooltip(move |window, cx| {
                                        Tooltip::for_action_in(
                                            "New Thread",
                                            &NewThread::default(),
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                    })
                                    .on_click(move |_event, window, cx| {
                                        window.dispatch_action(
                                            NewThread::default().boxed_clone(),
                                            cx,
                                        );
                                    }),
                            )
                            .child(
                                PopoverMenu::new("assistant-menu")
                                    .trigger_with_tooltip(
                                        IconButton::new("new", IconName::Ellipsis)
                                            .icon_size(IconSize::Small)
                                            .style(ButtonStyle::Subtle),
                                        Tooltip::text("Toggle Agent Menu"),
                                    )
                                    .anchor(Corner::TopRight)
                                    .with_handle(self.assistant_dropdown_menu_handle.clone())
                                    .menu(move |window, cx| {
                                        Some(ContextMenu::build(
                                            window,
                                            cx,
                                            |menu, _window, _cx| {
                                                menu.action(
                                                    "New Thread",
                                                    Box::new(NewThread {
                                                        from_thread_id: None,
                                                    }),
                                                )
                                                .action(
                                                    "New Prompt Editor",
                                                    NewPromptEditor.boxed_clone(),
                                                )
                                                .when(!is_empty, |menu| {
                                                    menu.action(
                                                        "Continue in New Thread",
                                                        Box::new(NewThread {
                                                            from_thread_id: Some(thread_id.clone()),
                                                        }),
                                                    )
                                                })
                                                .separator()
                                                .action("History", OpenHistory.boxed_clone())
                                                .action("Settings", OpenConfiguration.boxed_clone())
                                            },
                                        ))
                                    }),
                            ),
                    ),
            )
    }

    fn render_active_thread_or_empty_state(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if self.thread.read(cx).is_empty() {
            return self
                .render_thread_empty_state(window, cx)
                .into_any_element();
        }

        self.thread.clone().into_any_element()
    }

    fn configuration_error(&self, cx: &App) -> Option<ConfigurationError> {
        let Some(provider) = LanguageModelRegistry::read_global(cx).active_provider() else {
            return Some(ConfigurationError::NoProvider);
        };

        if !provider.is_authenticated(cx) {
            return Some(ConfigurationError::ProviderNotAuthenticated);
        }

        if provider.must_accept_terms(cx) {
            return Some(ConfigurationError::ProviderPendingTermsAcceptance(provider));
        }

        None
    }

    fn render_thread_empty_state(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let recent_history = self
            .history_store
            .update(cx, |this, cx| this.recent_entries(6, cx));

        let configuration_error = self.configuration_error(cx);
        let no_error = configuration_error.is_none();
        let focus_handle = self.focus_handle(cx);

        v_flex()
            .size_full()
            .when(recent_history.is_empty(), |this| {
                let configuration_error_ref = &configuration_error;
                this.child(
                    v_flex()
                        .size_full()
                        .max_w_80()
                        .mx_auto()
                        .justify_center()
                        .items_center()
                        .gap_1()
                        .child(
                            h_flex().child(
                                Headline::new("Welcome to the Agent Panel")
                            ),
                        )
                        .when(no_error, |parent| {
                            parent
                                .child(
                                    h_flex().child(
                                        Label::new("Ask and build anything.")
                                            .color(Color::Muted)
                                            .mb_2p5(),
                                    ),
                                )
                                .child(
                                    Button::new("new-thread", "Start New Thread")
                                        .icon(IconName::Plus)
                                        .icon_position(IconPosition::Start)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .full_width()
                                        .key_binding(KeyBinding::for_action_in(
                                            &NewThread::default(),
                                            &focus_handle,
                                            window,
                                            cx,
                                        ))
                                        .on_click(|_event, window, cx| {
                                            window.dispatch_action(NewThread::default().boxed_clone(), cx)
                                        }),
                                )
                                .child(
                                    Button::new("context", "Add Context")
                                        .icon(IconName::FileCode)
                                        .icon_position(IconPosition::Start)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .full_width()
                                        .key_binding(KeyBinding::for_action_in(
                                            &ToggleContextPicker,
                                            &focus_handle,
                                            window,
                                            cx,
                                        ))
                                        .on_click(|_event, window, cx| {
                                            window.dispatch_action(ToggleContextPicker.boxed_clone(), cx)
                                        }),
                                )
                                .child(
                                    Button::new("mode", "Switch Model")
                                        .icon(IconName::DatabaseZap)
                                        .icon_position(IconPosition::Start)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .full_width()
                                        .key_binding(KeyBinding::for_action_in(
                                            &ToggleModelSelector,
                                            &focus_handle,
                                            window,
                                            cx,
                                        ))
                                        .on_click(|_event, window, cx| {
                                            window.dispatch_action(ToggleModelSelector.boxed_clone(), cx)
                                        }),
                                )
                                .child(
                                    Button::new("settings", "View Settings")
                                        .icon(IconName::Settings)
                                        .icon_position(IconPosition::Start)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .full_width()
                                        .key_binding(KeyBinding::for_action_in(
                                            &OpenConfiguration,
                                            &focus_handle,
                                            window,
                                            cx,
                                        ))
                                        .on_click(|_event, window, cx| {
                                            window.dispatch_action(OpenConfiguration.boxed_clone(), cx)
                                        }),
                                )
                        })
                        .map(|parent| {
                            match configuration_error_ref {
                                Some(ConfigurationError::ProviderNotAuthenticated)
                                | Some(ConfigurationError::NoProvider) => {
                                    parent
                                        .child(
                                            h_flex().child(
                                                Label::new("To start using the agent, configure at least one LLM provider.")
                                                    .color(Color::Muted)
                                                    .mb_2p5()
                                            )
                                        )
                                        .child(
                                            Button::new("settings", "Configure a Provider")
                                                .icon(IconName::Settings)
                                                .icon_position(IconPosition::Start)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Muted)
                                                .full_width()
                                                .key_binding(KeyBinding::for_action_in(
                                                    &OpenConfiguration,
                                                    &focus_handle,
                                                    window,
                                                    cx,
                                                ))
                                                .on_click(|_event, window, cx| {
                                                    window.dispatch_action(OpenConfiguration.boxed_clone(), cx)
                                                }),
                                        )
                                }
                                Some(ConfigurationError::ProviderPendingTermsAcceptance(provider)) => {
                                    parent.children(
                                        provider.render_accept_terms(
                                            LanguageModelProviderTosView::ThreadFreshStart,
                                            cx,
                                        ),
                                    )
                                }
                                None => parent,
                            }
                        })
                )
            })
            .when(!recent_history.is_empty(), |parent| {
                let focus_handle = focus_handle.clone();
                let configuration_error_ref = &configuration_error;

                parent
                    .p_1p5()
                    .justify_end()
                    .gap_1()
                    .child(
                        h_flex()
                            .pl_1p5()
                            .pb_1()
                            .w_full()
                            .justify_between()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                Label::new("Past Interactions")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                Button::new("view-history", "View All")
                                    .style(ButtonStyle::Subtle)
                                    .label_size(LabelSize::Small)
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &OpenHistory,
                                            &self.focus_handle(cx),
                                            window,
                                            cx,
                                        ).map(|kb| kb.size(rems_from_px(12.))),
                                    )
                                    .on_click(move |_event, window, cx| {
                                        window.dispatch_action(OpenHistory.boxed_clone(), cx);
                                    }),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .children(
                                recent_history.into_iter().map(|entry| {
                                    // TODO: Add keyboard navigation.
                                    match entry {
                                        HistoryEntry::Thread(thread) => {
                                            PastThread::new(thread, cx.entity().downgrade(), false)
                                                .into_any_element()
                                        }
                                        HistoryEntry::Context(context) => {
                                            PastContext::new(context, cx.entity().downgrade(), false)
                                                .into_any_element()
                                        }
                                    }
                                }),
                            )
                    )
                    .map(|parent| {
                        match configuration_error_ref {
                            Some(ConfigurationError::ProviderNotAuthenticated)
                            | Some(ConfigurationError::NoProvider) => {
                                parent
                                    .child(
                                        Banner::new()
                                            .severity(ui::Severity::Warning)
                                            .children(
                                                Label::new(
                                                    "Configure at least one LLM provider to start using the panel.",
                                                )
                                                .size(LabelSize::Small),
                                            )
                                            .action_slot(
                                                Button::new("settings", "Configure Provider")
                                                    .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                                                    .label_size(LabelSize::Small)
                                                    .key_binding(
                                                        KeyBinding::for_action_in(
                                                            &OpenConfiguration,
                                                            &focus_handle,
                                                            window,
                                                            cx,
                                                        )
                                                        .map(|kb| kb.size(rems_from_px(12.))),
                                                    )
                                                    .on_click(|_event, window, cx| {
                                                        window.dispatch_action(
                                                            OpenConfiguration.boxed_clone(),
                                                            cx,
                                                        )
                                                    }),
                                            ),
                                    )
                            }
                            Some(ConfigurationError::ProviderPendingTermsAcceptance(provider)) => {
                                parent
                                    .child(
                                        Banner::new()
                                            .severity(ui::Severity::Warning)
                                            .children(
                                                h_flex()
                                                    .w_full()
                                                    .children(
                                                        provider.render_accept_terms(
                                                            LanguageModelProviderTosView::ThreadtEmptyState,
                                                            cx,
                                                        ),
                                                    ),
                                            ),
                                    )
                            }
                            None => parent,
                        }
                    })
            })
    }

    fn render_last_error(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
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
                    ThreadError::Message { header, message } => {
                        self.render_error_message(header, message, cx)
                    }
                })
                .into_any(),
        )
    }

    fn render_payment_required_error(&self, cx: &mut Context<Self>) -> AnyElement {
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
                        |this, _, _, cx| {
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

                            cx.open_url(&zed_urls::account_url(cx));
                            cx.notify();
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_max_monthly_spend_reached_error(&self, cx: &mut Context<Self>) -> AnyElement {
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
                            cx.listener(|this, _, _, cx| {
                                this.thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });

                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            }),
                        ),
                    )
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, _, cx| {
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
        header: SharedString,
        message: SharedString,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new(header).weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_32()
                    .overflow_y_scroll()
                    .child(Label::new(message)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.thread.update(cx, |this, _cx| {
                                this.clear_last_error();
                            });

                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AgentPanel");
        if matches!(self.active_view, ActiveView::PromptEditor) {
            key_context.add("prompt_editor");
        }
        key_context
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context(self.key_context())
            .justify_between()
            .size_full()
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|this, action: &NewThread, window, cx| {
                this.new_thread(action, window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenHistory, window, cx| {
                this.open_history(window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenConfiguration, window, cx| {
                this.open_configuration(window, cx);
            }))
            .on_action(cx.listener(Self::open_active_thread_as_markdown))
            .on_action(cx.listener(Self::deploy_prompt_library))
            .on_action(cx.listener(Self::open_agent_diff))
            .child(self.render_toolbar(window, cx))
            .map(|parent| match self.active_view {
                ActiveView::Thread => parent
                    .child(self.render_active_thread_or_empty_state(window, cx))
                    .child(h_flex().child(self.message_editor.clone()))
                    .children(self.render_last_error(cx)),
                ActiveView::History => parent.child(self.history.clone()),
                ActiveView::PromptEditor => parent.children(self.context_editor.clone()),
                ActiveView::Configuration => parent.children(self.configuration.clone()),
            })
    }
}

struct PromptLibraryInlineAssist {
    workspace: WeakEntity<Workspace>,
}

impl PromptLibraryInlineAssist {
    pub fn new(workspace: WeakEntity<Workspace>) -> Self {
        Self { workspace }
    }
}

impl prompt_library::InlineAssistDelegate for PromptLibraryInlineAssist {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        _initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut Context<PromptLibrary>,
    ) {
        InlineAssistant::update_global(cx, |assistant, cx| {
            assistant.assist(&prompt_editor, self.workspace.clone(), None, window, cx)
        })
    }

    fn focus_assistant_panel(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        workspace
            .focus_panel::<AssistantPanel>(window, cx)
            .is_some()
    }
}

pub struct ConcreteAssistantPanelDelegate;

impl AssistantPanelDelegate for ConcreteAssistantPanelDelegate {
    fn active_context_editor(
        &self,
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<ContextEditor>> {
        let panel = workspace.panel::<AssistantPanel>(cx)?;
        panel.update(cx, |panel, _cx| panel.context_editor.clone())
    }

    fn open_saved_context(
        &self,
        workspace: &mut Workspace,
        path: std::path::PathBuf,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return Task::ready(Err(anyhow!("Agent panel not found")));
        };

        panel.update(cx, |panel, cx| {
            panel.open_saved_prompt_editor(path, window, cx)
        })
    }

    fn open_remote_context(
        &self,
        _workspace: &mut Workspace,
        _context_id: assistant_context_editor::ContextId,
        _window: &mut Window,
        _cx: &mut Context<Workspace>,
    ) -> Task<Result<Entity<ContextEditor>>> {
        Task::ready(Err(anyhow!("opening remote context not implemented")))
    }

    fn quote_selection(
        &self,
        _workspace: &mut Workspace,
        _creases: Vec<(String, String)>,
        _window: &mut Window,
        _cx: &mut Context<Workspace>,
    ) {
    }
}

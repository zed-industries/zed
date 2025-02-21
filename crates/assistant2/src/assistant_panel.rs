use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_context_editor::{
    make_lsp_adapter_delegate, render_remaining_tokens, AssistantPanelDelegate, ConfigurationError,
    ContextEditor, SlashCommandCompletionProvider,
};
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_tool::ToolWorkingSet;

use client::zed_urls;
use editor::Editor;
use fs::Fs;
use gpui::{
    prelude::*, px, svg, Action, AnyElement, App, AsyncWindowContext, Corner, Entity, EventEmitter,
    FocusHandle, Focusable, FontWeight, Pixels, Subscription, Task, UpdateGlobal, WeakEntity,
};
use language::LanguageRegistry;
use language_model::{LanguageModelProviderTosView, LanguageModelRegistry};
use project::Project;
use prompt_library::{open_prompt_library, PromptBuilder, PromptLibrary};
use settings::{update_settings_file, Settings};
use time::UtcOffset;
use ui::{prelude::*, ContextMenu, KeyBinding, PopoverMenu, PopoverMenuHandle, Tab, Tooltip};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;
use zed_actions::assistant::{DeployPromptLibrary, ToggleFocus};

use crate::active_thread::ActiveThread;
use crate::assistant_configuration::{AssistantConfiguration, AssistantConfigurationEvent};
use crate::history_store::{HistoryEntry, HistoryStore};
use crate::message_editor::MessageEditor;
use crate::thread::{Thread, ThreadError, ThreadId};
use crate::thread_history::{PastContext, PastThread, ThreadHistory};
use crate::thread_store::ThreadStore;
use crate::{InlineAssistant, NewPromptEditor, NewThread, OpenConfiguration, OpenHistory};

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, _: &NewThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.new_thread(window, cx));
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                    }
                })
                .register_action(|workspace, _: &OpenHistory, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_history(window, cx));
                    }
                })
                .register_action(|workspace, _: &NewPromptEditor, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.new_prompt_editor(window, cx));
                    }
                })
                .register_action(|workspace, _: &OpenConfiguration, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_configuration(window, cx));
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
    tools: Arc<ToolWorkingSet>,
    local_timezone: UtcOffset,
    active_view: ActiveView,
    history_store: Entity<HistoryStore>,
    history: Entity<ThreadHistory>,
    new_item_context_menu_handle: PopoverMenuHandle<ContextMenu>,
    width: Option<Pixels>,
    height: Option<Pixels>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(|mut cx| async move {
            let tools = Arc::new(ToolWorkingSet::default());
            log::info!("[assistant2-debug] initializing ThreadStore");
            let thread_store = workspace.update(&mut cx, |workspace, cx| {
                let project = workspace.project().clone();
                ThreadStore::new(project, tools.clone(), cx)
            })??;
            log::info!("[assistant2-debug] finished initializing ThreadStore");

            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            log::info!("[assistant2-debug] initializing ContextStore");
            let context_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    assistant_context_editor::ContextStore::new(
                        project,
                        prompt_builder.clone(),
                        slash_commands,
                        cx,
                    )
                })?
                .await?;
            log::info!("[assistant2-debug] finished initializing ContextStore");

            workspace.update_in(&mut cx, |workspace, window, cx| {
                cx.new(|cx| Self::new(workspace, thread_store, context_store, tools, window, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Entity<ThreadStore>,
        context_store: Entity<assistant_context_editor::ContextStore>,
        tools: Arc<ToolWorkingSet>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        log::info!("[assistant2-debug] AssistantPanel::new");
        let thread = thread_store.update(cx, |this, cx| this.create_thread(cx));
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let language_registry = project.read(cx).languages().clone();
        let workspace = workspace.weak_handle();
        let weak_self = cx.entity().downgrade();

        let message_editor = cx.new(|cx| {
            MessageEditor::new(
                fs.clone(),
                workspace.clone(),
                thread_store.downgrade(),
                thread.clone(),
                window,
                cx,
            )
        });

        let history_store =
            cx.new(|cx| HistoryStore::new(thread_store.clone(), context_store.clone(), cx));

        Self {
            active_view: ActiveView::Thread,
            workspace: workspace.clone(),
            project,
            fs: fs.clone(),
            language_registry: language_registry.clone(),
            thread_store: thread_store.clone(),
            thread: cx.new(|cx| {
                ActiveThread::new(
                    thread.clone(),
                    thread_store.clone(),
                    workspace,
                    language_registry,
                    tools.clone(),
                    window,
                    cx,
                )
            }),
            message_editor,
            context_store,
            context_editor: None,
            configuration: None,
            configuration_subscription: None,
            tools,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            history_store: history_store.clone(),
            history: cx.new(|cx| ThreadHistory::new(weak_self, history_store, cx)),
            new_item_context_menu_handle: PopoverMenuHandle::default(),
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
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        workspace.toggle_panel_focus::<Self>(window, cx);
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

    fn new_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(cx));

        self.active_view = ActiveView::Thread;
        self.thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.thread_store.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.tools.clone(),
                window,
                cx,
            )
        });
        self.message_editor = cx.new(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
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
        _: &DeployPromptLibrary,
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

        cx.spawn_in(window, |this, mut cx| async move {
            let context = context.await?;
            this.update_in(&mut cx, |this, window, cx| {
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

        cx.spawn_in(window, |this, mut cx| async move {
            let thread = open_thread_task.await?;
            this.update_in(&mut cx, |this, window, cx| {
                this.active_view = ActiveView::Thread;
                this.thread = cx.new(|cx| {
                    ActiveThread::new(
                        thread.clone(),
                        this.thread_store.clone(),
                        this.workspace.clone(),
                        this.language_registry.clone(),
                        this.tools.clone(),
                        window,
                        cx,
                    )
                });
                this.message_editor = cx.new(|cx| {
                    MessageEditor::new(
                        this.fs.clone(),
                        this.workspace.clone(),
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

    pub(crate) fn open_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active_view = ActiveView::Configuration;
        self.configuration = Some(cx.new(|cx| AssistantConfiguration::new(window, cx)));

        if let Some(configuration) = self.configuration.as_ref() {
            self.configuration_subscription = Some(cx.subscribe_in(
                configuration,
                window,
                Self::handle_assistant_configuration_event,
            ));

            configuration.focus_handle(cx).focus(window);
        }
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
                    if let Some(model) = provider.provided_models(cx).first().cloned() {
                        update_settings_file::<AssistantSettings>(
                            self.fs.clone(),
                            cx,
                            move |settings, _| settings.set_model(model),
                        );
                    }
                }

                self.new_thread(window, cx);
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
        "AssistantPanel2"
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
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled || !settings.button {
            return None;
        }

        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
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
    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            ActiveView::History => "History".into(),
            ActiveView::Configuration => "Assistant Settings".into(),
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
            .child(
                h_flex()
                    .w_full()
                    .gap_1()
                    .justify_between()
                    .child(Label::new(title))
                    .children(if matches!(self.active_view, ActiveView::PromptEditor) {
                        self.context_editor
                            .as_ref()
                            .and_then(|editor| render_remaining_tokens(editor, cx))
                    } else {
                        None
                    }),
            )
            .child(
                h_flex()
                    .h_full()
                    .pl_1p5()
                    .border_l_1()
                    .border_color(cx.theme().colors().border)
                    .gap(DynamicSpacing::Base02.rems(cx))
                    .child(
                        PopoverMenu::new("assistant-toolbar-new-popover-menu")
                            .trigger_with_tooltip(
                                IconButton::new("new", IconName::Plus)
                                    .icon_size(IconSize::Small)
                                    .style(ButtonStyle::Subtle),
                                Tooltip::text("New…"),
                            )
                            .anchor(Corner::TopRight)
                            .with_handle(self.new_item_context_menu_handle.clone())
                            .menu(move |window, cx| {
                                Some(ContextMenu::build(window, cx, |menu, _window, _cx| {
                                    menu.action("New Thread", NewThread.boxed_clone())
                                        .action("New Prompt Editor", NewPromptEditor.boxed_clone())
                                }))
                            }),
                    )
                    .child(
                        IconButton::new("open-history", IconName::HistoryRerun)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip({
                                let focus_handle = self.focus_handle(cx);
                                move |window, cx| {
                                    Tooltip::for_action_in(
                                        "History",
                                        &OpenHistory,
                                        &focus_handle,
                                        window,
                                        cx,
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
                            .tooltip(Tooltip::text("Assistant Settings"))
                            .on_click(move |_event, window, cx| {
                                window.dispatch_action(OpenConfiguration.boxed_clone(), cx);
                            }),
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
            .update(cx, |this, cx| this.recent_entries(3, cx));

        let create_welcome_heading = || {
            h_flex()
                .w_full()
                .justify_center()
                .child(Headline::new("Welcome to the Assistant Panel").size(HeadlineSize::Small))
        };

        let configuration_error = self.configuration_error(cx);
        let no_error = configuration_error.is_none();

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
            .map(|parent| {
                match configuration_error {
                    Some(ConfigurationError::ProviderNotAuthenticated)
                    | Some(ConfigurationError::NoProvider) => {
                        parent.child(
                            v_flex()
                                .gap_0p5()
                                .child(create_welcome_heading())
                                .child(
                                    h_flex().mb_2().w_full().justify_center().child(
                                        Label::new(
                                            "To start using the assistant, configure at least one LLM provider.",
                                        )
                                        .color(Color::Muted),
                                    ),
                                )
                                .child(
                                    h_flex().w_full().justify_center().child(
                                        Button::new("open-configuration", "Configure a Provider")
                                            .size(ButtonSize::Compact)
                                            .icon(Some(IconName::Sliders))
                                            .icon_size(IconSize::Small)
                                            .icon_position(IconPosition::Start)
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.open_configuration(window, cx);
                                            })),
                                    ),
                                ),
                        )
                    }
                    Some(ConfigurationError::ProviderPendingTermsAcceptance(provider)) => parent
                        .child(v_flex().gap_0p5().child(create_welcome_heading()).children(
                            provider.render_accept_terms(
                                LanguageModelProviderTosView::ThreadEmptyState,
                                cx,
                            ),
                        )),
                    None => parent,
                }
            })
            .when(recent_history.is_empty() && no_error, |parent| {
                parent.child(v_flex().gap_0p5().child(create_welcome_heading()).child(
                    h_flex().w_full().justify_center().child(
                        Label::new("Start typing to chat with your codebase").color(Color::Muted),
                    ),
                ))
            })
            .when(!recent_history.is_empty(), |parent| {
                parent
                    .child(
                        h_flex().w_full().justify_center().child(
                            Label::new("Recent Threads:")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(v_flex().mx_auto().w_4_5().gap_2().children(
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
                    ))
                    .child(
                        h_flex().w_full().justify_center().child(
                            Button::new("view-all-past-threads", "View All Past Threads")
                                .style(ButtonStyle::Subtle)
                                .label_size(LabelSize::Small)
                                .key_binding(KeyBinding::for_action_in(
                                    &OpenHistory,
                                    &self.focus_handle(cx),
                                    window,
                                    cx,
                                ))
                                .on_click(move |_event, window, cx| {
                                    window.dispatch_action(OpenHistory.boxed_clone(), cx);
                                }),
                        ),
                    )
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
                    ThreadError::Message(error_message) => {
                        self.render_error_message(&error_message, cx)
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
        error_message: &SharedString,
        cx: &mut Context<Self>,
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
}

impl Render for AssistantPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AssistantPanel2")
            .justify_between()
            .size_full()
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|this, _: &NewThread, window, cx| {
                this.new_thread(window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenHistory, window, cx| {
                this.open_history(window, cx);
            }))
            .on_action(cx.listener(Self::deploy_prompt_library))
            .child(self.render_toolbar(cx))
            .map(|parent| match self.active_view {
                ActiveView::Thread => parent
                    .child(self.render_active_thread_or_empty_state(window, cx))
                    .child(
                        h_flex()
                            .border_t_1()
                            .border_color(cx.theme().colors().border)
                            .child(self.message_editor.clone()),
                    )
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
            return Task::ready(Err(anyhow!("Assistant panel not found")));
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

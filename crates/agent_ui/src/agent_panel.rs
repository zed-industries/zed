use std::cell::RefCell;
use std::ops::{Not, Range};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use agent_servers::AgentServer;
use db::kvp::{Dismissable, KEY_VALUE_STORE};
use serde::{Deserialize, Serialize};

use crate::NewExternalAgentThread;
use crate::agent_diff::AgentDiffThread;
use crate::message_editor::{MAX_EDITOR_LINES, MIN_EDITOR_LINES};
use crate::ui::NewThreadButton;
use crate::{
    AddContextServer, AgentDiffPane, ContinueThread, ContinueWithBurnMode,
    DeleteRecentlyOpenThread, ExpandMessageEditor, Follow, InlineAssistant, NewTextThread,
    NewThread, OpenActiveThreadAsMarkdown, OpenAgentDiff, OpenHistory, ResetTrialEndUpsell,
    ResetTrialUpsell, ToggleBurnMode, ToggleContextPicker, ToggleNavigationMenu, ToggleOptionsMenu,
    acp::AcpThreadView,
    active_thread::{self, ActiveThread, ActiveThreadEvent},
    agent_configuration::{AgentConfiguration, AssistantConfigurationEvent},
    agent_diff::AgentDiff,
    message_editor::{MessageEditor, MessageEditorEvent},
    slash_command::SlashCommandCompletionProvider,
    text_thread_editor::{
        AgentPanelDelegate, TextThreadEditor, humanize_token_count, make_lsp_adapter_delegate,
        render_remaining_tokens,
    },
    thread_history::{HistoryEntryElement, ThreadHistory},
    ui::{AgentOnboardingModal, EndTrialUpsell},
};
use agent::{
    Thread, ThreadError, ThreadEvent, ThreadId, ThreadSummary, TokenUsageRatio,
    context_store::ContextStore,
    history_store::{HistoryEntryId, HistoryStore},
    thread_store::{TextThreadStore, ThreadStore},
};
use agent_settings::{AgentDockPosition, AgentSettings, CompletionMode, DefaultView};
use ai_onboarding::AgentPanelOnboarding;
use anyhow::{Result, anyhow};
use assistant_context::{AssistantContext, ContextEvent, ContextSummary};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_tool::ToolWorkingSet;
use client::{UserStore, zed_urls};
use cloud_llm_client::{CompletionIntent, Plan, UsageLimit};
use editor::{Anchor, AnchorRangeExt as _, Editor, EditorEvent, MultiBuffer};
use feature_flags::{self, FeatureFlagAppExt};
use fs::Fs;
use gpui::{
    Action, Animation, AnimationExt as _, AnyElement, App, AsyncWindowContext, ClipboardItem,
    Corner, DismissEvent, Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable, Hsla,
    KeyContext, Pixels, Subscription, Task, UpdateGlobal, WeakEntity, prelude::*,
    pulsating_between,
};
use language::LanguageRegistry;
use language_model::{
    ConfigurationError, ConfiguredModel, LanguageModelProviderTosView, LanguageModelRegistry,
};
use project::{DisableAiSettings, Project, ProjectPath, Worktree};
use prompt_store::{PromptBuilder, PromptStore, UserPromptId};
use rules_library::{RulesLibrary, open_rules_library};
use search::{BufferSearchBar, buffer_search};
use settings::{Settings, update_settings_file};
use theme::ThemeSettings;
use time::UtcOffset;
use ui::utils::WithRemSize;
use ui::{
    Banner, Callout, ContextMenu, ContextMenuEntry, ElevationIndex, KeyBinding, PopoverMenu,
    PopoverMenuHandle, ProgressBar, Tab, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{
    CollaboratorId, DraggedSelection, DraggedTab, ToggleZoom, ToolbarItemView, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};
use zed_actions::{
    DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize,
    agent::{OpenOnboardingModal, OpenSettings, ResetOnboarding, ToggleModelSelector},
    assistant::{OpenRulesLibrary, ToggleFocus},
};

const AGENT_PANEL_KEY: &str = "agent_panel";

#[derive(Serialize, Deserialize)]
struct SerializedAgentPanel {
    width: Option<Pixels>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, action: &NewThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.new_thread(action, window, cx));
                        workspace.focus_panel::<AgentPanel>(window, cx);
                    }
                })
                .register_action(|workspace, _: &OpenHistory, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_history(window, cx));
                    }
                })
                .register_action(|workspace, _: &OpenSettings, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_configuration(window, cx));
                    }
                })
                .register_action(|workspace, _: &NewTextThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.new_prompt_editor(window, cx));
                    }
                })
                .register_action(|workspace, action: &NewExternalAgentThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.new_external_thread(action.agent, window, cx)
                        });
                    }
                })
                .register_action(|workspace, action: &OpenRulesLibrary, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.deploy_rules_library(action, window, cx)
                        });
                    }
                })
                .register_action(|workspace, _: &OpenAgentDiff, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        match &panel.read(cx).active_view {
                            ActiveView::Thread { thread, .. } => {
                                let thread = thread.read(cx).thread().clone();
                                AgentDiffPane::deploy_in_workspace(thread, workspace, window, cx);
                            }
                            ActiveView::ExternalAgentThread { .. }
                            | ActiveView::TextThread { .. }
                            | ActiveView::History
                            | ActiveView::Configuration => {}
                        }
                    }
                })
                .register_action(|workspace, _: &Follow, window, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .register_action(|workspace, _: &ExpandMessageEditor, window, cx| {
                    let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                        return;
                    };
                    workspace.focus_panel::<AgentPanel>(window, cx);
                    panel.update(cx, |panel, cx| {
                        if let Some(message_editor) = panel.active_message_editor() {
                            message_editor.update(cx, |editor, cx| {
                                editor.expand_message_editor(&ExpandMessageEditor, window, cx);
                            });
                        }
                    });
                })
                .register_action(|workspace, _: &ToggleNavigationMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_navigation_menu(&ToggleNavigationMenu, window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &ToggleOptionsMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_options_menu(&ToggleOptionsMenu, window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &OpenOnboardingModal, window, cx| {
                    AgentOnboardingModal::toggle(workspace, window, cx)
                })
                .register_action(|_workspace, _: &ResetOnboarding, window, cx| {
                    window.dispatch_action(workspace::RestoreBanner.boxed_clone(), cx);
                    window.refresh();
                })
                .register_action(|_workspace, _: &ResetTrialUpsell, _window, cx| {
                    OnboardingUpsell::set_dismissed(false, cx);
                })
                .register_action(|_workspace, _: &ResetTrialEndUpsell, _window, cx| {
                    TrialEndUpsell::set_dismissed(false, cx);
                });
        },
    )
    .detach();
}

enum ActiveView {
    Thread {
        thread: Entity<ActiveThread>,
        change_title_editor: Entity<Editor>,
        message_editor: Entity<MessageEditor>,
        _subscriptions: Vec<gpui::Subscription>,
    },
    ExternalAgentThread {
        thread_view: Entity<AcpThreadView>,
    },
    TextThread {
        context_editor: Entity<TextThreadEditor>,
        title_editor: Entity<Editor>,
        buffer_search_bar: Entity<BufferSearchBar>,
        _subscriptions: Vec<gpui::Subscription>,
    },
    History,
    Configuration,
}

enum WhichFontSize {
    AgentFont,
    BufferFont,
    None,
}

impl ActiveView {
    pub fn which_font_size_used(&self) -> WhichFontSize {
        match self {
            ActiveView::Thread { .. }
            | ActiveView::ExternalAgentThread { .. }
            | ActiveView::History => WhichFontSize::AgentFont,
            ActiveView::TextThread { .. } => WhichFontSize::BufferFont,
            ActiveView::Configuration => WhichFontSize::None,
        }
    }

    pub fn thread(
        active_thread: Entity<ActiveThread>,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<AgentPanel>,
    ) -> Self {
        let summary = active_thread.read(cx).summary(cx).or_default();

        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(summary.clone(), window, cx);
            editor
        });

        let subscriptions = vec![
            cx.subscribe(&message_editor, |this, _, event, cx| match event {
                MessageEditorEvent::Changed | MessageEditorEvent::EstimatedTokenCount => {
                    cx.notify();
                }
                MessageEditorEvent::ScrollThreadToBottom => match &this.active_view {
                    ActiveView::Thread { thread, .. } => {
                        thread.update(cx, |thread, cx| {
                            thread.scroll_to_bottom(cx);
                        });
                    }
                    ActiveView::ExternalAgentThread { .. } => {}
                    ActiveView::TextThread { .. }
                    | ActiveView::History
                    | ActiveView::Configuration => {}
                },
            }),
            window.subscribe(&editor, cx, {
                {
                    let thread = active_thread.clone();
                    move |editor, event, window, cx| match event {
                        EditorEvent::BufferEdited => {
                            let new_summary = editor.read(cx).text(cx);

                            thread.update(cx, |thread, cx| {
                                thread.thread().update(cx, |thread, cx| {
                                    thread.set_summary(new_summary, cx);
                                });
                            })
                        }
                        EditorEvent::Blurred => {
                            if editor.read(cx).text(cx).is_empty() {
                                let summary = thread.read(cx).summary(cx).or_default();

                                editor.update(cx, |editor, cx| {
                                    editor.set_text(summary, window, cx);
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }),
            cx.subscribe(&active_thread, |_, _, event, cx| match &event {
                ActiveThreadEvent::EditingMessageTokenCountChanged => {
                    cx.notify();
                }
            }),
            cx.subscribe_in(&active_thread.read(cx).thread().clone(), window, {
                let editor = editor.clone();
                move |_, thread, event, window, cx| match event {
                    ThreadEvent::SummaryGenerated => {
                        let summary = thread.read(cx).summary().or_default();

                        editor.update(cx, |editor, cx| {
                            editor.set_text(summary, window, cx);
                        })
                    }
                    ThreadEvent::MessageAdded(_) => {
                        cx.notify();
                    }
                    _ => {}
                }
            }),
        ];

        Self::Thread {
            change_title_editor: editor,
            thread: active_thread,
            message_editor: message_editor,
            _subscriptions: subscriptions,
        }
    }

    pub fn prompt_editor(
        context_editor: Entity<TextThreadEditor>,
        history_store: Entity<HistoryStore>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let title = context_editor.read(cx).title(cx).to_string();

        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(title, window, cx);
            editor
        });

        // This is a workaround for `editor.set_text` emitting a `BufferEdited` event, which would
        // cause a custom summary to be set. The presence of this custom summary would cause
        // summarization to not happen.
        let mut suppress_first_edit = true;

        let subscriptions = vec![
            window.subscribe(&editor, cx, {
                {
                    let context_editor = context_editor.clone();
                    move |editor, event, window, cx| match event {
                        EditorEvent::BufferEdited => {
                            if suppress_first_edit {
                                suppress_first_edit = false;
                                return;
                            }
                            let new_summary = editor.read(cx).text(cx);

                            context_editor.update(cx, |context_editor, cx| {
                                context_editor
                                    .context()
                                    .update(cx, |assistant_context, cx| {
                                        assistant_context.set_custom_summary(new_summary, cx);
                                    })
                            })
                        }
                        EditorEvent::Blurred => {
                            if editor.read(cx).text(cx).is_empty() {
                                let summary = context_editor
                                    .read(cx)
                                    .context()
                                    .read(cx)
                                    .summary()
                                    .or_default();

                                editor.update(cx, |editor, cx| {
                                    editor.set_text(summary, window, cx);
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }),
            window.subscribe(&context_editor.read(cx).context().clone(), cx, {
                let editor = editor.clone();
                move |assistant_context, event, window, cx| match event {
                    ContextEvent::SummaryGenerated => {
                        let summary = assistant_context.read(cx).summary().or_default();

                        editor.update(cx, |editor, cx| {
                            editor.set_text(summary, window, cx);
                        })
                    }
                    ContextEvent::PathChanged { old_path, new_path } => {
                        history_store.update(cx, |history_store, cx| {
                            if let Some(old_path) = old_path {
                                history_store
                                    .replace_recently_opened_text_thread(old_path, new_path, cx);
                            } else {
                                history_store.push_recently_opened_entry(
                                    HistoryEntryId::Context(new_path.clone()),
                                    cx,
                                );
                            }
                        });
                    }
                    _ => {}
                }
            }),
        ];

        let buffer_search_bar =
            cx.new(|cx| BufferSearchBar::new(Some(language_registry), window, cx));
        buffer_search_bar.update(cx, |buffer_search_bar, cx| {
            buffer_search_bar.set_active_pane_item(Some(&context_editor), window, cx)
        });

        Self::TextThread {
            context_editor,
            title_editor: editor,
            buffer_search_bar,
            _subscriptions: subscriptions,
        }
    }
}

pub struct AgentPanel {
    workspace: WeakEntity<Workspace>,
    user_store: Entity<UserStore>,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    _default_model_subscription: Subscription,
    context_store: Entity<TextThreadStore>,
    prompt_store: Option<Entity<PromptStore>>,
    inline_assist_context_store: Entity<ContextStore>,
    configuration: Option<Entity<AgentConfiguration>>,
    configuration_subscription: Option<Subscription>,
    local_timezone: UtcOffset,
    active_view: ActiveView,
    acp_message_history:
        Rc<RefCell<crate::acp::MessageHistory<Vec<agent_client_protocol::ContentBlock>>>>,
    previous_view: Option<ActiveView>,
    history_store: Entity<HistoryStore>,
    history: Entity<ThreadHistory>,
    hovered_recent_history_item: Option<usize>,
    new_thread_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_panel_menu_handle: PopoverMenuHandle<ContextMenu>,
    assistant_navigation_menu_handle: PopoverMenuHandle<ContextMenu>,
    assistant_navigation_menu: Option<Entity<ContextMenu>>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    zoomed: bool,
    pending_serialization: Option<Task<Result<()>>>,
    onboarding: Entity<AgentPanelOnboarding>,
}

impl AgentPanel {
    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        self.pending_serialization = Some(cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(
                    AGENT_PANEL_KEY.into(),
                    serde_json::to_string(&SerializedAgentPanel { width })?,
                )
                .await?;
            anyhow::Ok(())
        }));
    }
    pub fn load(
        workspace: WeakEntity<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
        mut cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        let prompt_store = cx.update(|_window, cx| PromptStore::global(cx));
        cx.spawn(async move |cx| {
            let prompt_store = match prompt_store {
                Ok(prompt_store) => prompt_store.await.ok(),
                Err(_) => None,
            };
            let tools = cx.new(|_| ToolWorkingSet::default())?;
            let thread_store = workspace
                .update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    ThreadStore::load(
                        project,
                        tools.clone(),
                        prompt_store.clone(),
                        prompt_builder.clone(),
                        cx,
                    )
                })?
                .await?;

            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            let context_store = workspace
                .update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    assistant_context::ContextStore::new(
                        project,
                        prompt_builder.clone(),
                        slash_commands,
                        cx,
                    )
                })?
                .await?;

            let serialized_panel = if let Some(panel) = cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(AGENT_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedAgentPanel>(&panel)?)
            } else {
                None
            };

            let panel = workspace.update_in(cx, |workspace, window, cx| {
                let panel = cx.new(|cx| {
                    Self::new(
                        workspace,
                        thread_store,
                        context_store,
                        prompt_store,
                        window,
                        cx,
                    )
                });
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width.map(|w| w.round());
                        cx.notify();
                    });
                }
                panel
            })?;

            Ok(panel)
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Entity<ThreadStore>,
        context_store: Entity<TextThreadStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let thread = thread_store.update(cx, |this, cx| this.create_thread(cx));
        let fs = workspace.app_state().fs.clone();
        let user_store = workspace.app_state().user_store.clone();
        let project = workspace.project();
        let language_registry = project.read(cx).languages().clone();
        let client = workspace.client().clone();
        let workspace = workspace.weak_handle();
        let weak_self = cx.entity().downgrade();

        let message_editor_context_store =
            cx.new(|_cx| ContextStore::new(project.downgrade(), Some(thread_store.downgrade())));
        let inline_assist_context_store =
            cx.new(|_cx| ContextStore::new(project.downgrade(), Some(thread_store.downgrade())));

        let thread_id = thread.read(cx).id().clone();

        let history_store = cx.new(|cx| {
            HistoryStore::new(
                thread_store.clone(),
                context_store.clone(),
                [HistoryEntryId::Thread(thread_id)],
                cx,
            )
        });

        let message_editor = cx.new(|cx| {
            MessageEditor::new(
                fs.clone(),
                workspace.clone(),
                message_editor_context_store.clone(),
                prompt_store.clone(),
                thread_store.downgrade(),
                context_store.downgrade(),
                Some(history_store.downgrade()),
                thread.clone(),
                window,
                cx,
            )
        });

        cx.observe(&history_store, |_, _, cx| cx.notify()).detach();

        let active_thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                thread_store.clone(),
                context_store.clone(),
                message_editor_context_store.clone(),
                language_registry.clone(),
                workspace.clone(),
                window,
                cx,
            )
        });

        let panel_type = AgentSettings::get_global(cx).default_view;
        let active_view = match panel_type {
            DefaultView::Thread => ActiveView::thread(active_thread, message_editor, window, cx),
            DefaultView::TextThread => {
                let context =
                    context_store.update(cx, |context_store, cx| context_store.create(cx));
                let lsp_adapter_delegate = make_lsp_adapter_delegate(&project.clone(), cx).unwrap();
                let context_editor = cx.new(|cx| {
                    let mut editor = TextThreadEditor::for_context(
                        context,
                        fs.clone(),
                        workspace.clone(),
                        project.clone(),
                        lsp_adapter_delegate,
                        window,
                        cx,
                    );
                    editor.insert_default_prompt(window, cx);
                    editor
                });
                ActiveView::prompt_editor(
                    context_editor,
                    history_store.clone(),
                    language_registry.clone(),
                    window,
                    cx,
                )
            }
        };

        AgentDiff::set_active_thread(&workspace, thread.clone(), window, cx);

        let weak_panel = weak_self.clone();

        window.defer(cx, move |window, cx| {
            let panel = weak_panel.clone();
            let assistant_navigation_menu =
                ContextMenu::build_persistent(window, cx, move |mut menu, _window, cx| {
                    if let Some(panel) = panel.upgrade() {
                        menu = Self::populate_recently_opened_menu_section(menu, panel, cx);
                    }
                    menu.action("View All", Box::new(OpenHistory))
                        .end_slot_action(DeleteRecentlyOpenThread.boxed_clone())
                        .fixed_width(px(320.).into())
                        .keep_open_on_confirm(false)
                        .key_context("NavigationMenu")
                });
            weak_panel
                .update(cx, |panel, cx| {
                    cx.subscribe_in(
                        &assistant_navigation_menu,
                        window,
                        |_, menu, _: &DismissEvent, window, cx| {
                            menu.update(cx, |menu, _| {
                                menu.clear_selected();
                            });
                            cx.focus_self(window);
                        },
                    )
                    .detach();
                    panel.assistant_navigation_menu = Some(assistant_navigation_menu);
                })
                .ok();
        });

        let _default_model_subscription = cx.subscribe(
            &LanguageModelRegistry::global(cx),
            |this, _, event: &language_model::Event, cx| match event {
                language_model::Event::DefaultModelChanged => match &this.active_view {
                    ActiveView::Thread { thread, .. } => {
                        thread
                            .read(cx)
                            .thread()
                            .clone()
                            .update(cx, |thread, cx| thread.get_or_init_configured_model(cx));
                    }
                    ActiveView::ExternalAgentThread { .. }
                    | ActiveView::TextThread { .. }
                    | ActiveView::History
                    | ActiveView::Configuration => {}
                },
                _ => {}
            },
        );

        let onboarding = cx.new(|cx| {
            AgentPanelOnboarding::new(
                user_store.clone(),
                client,
                |_window, cx| {
                    OnboardingUpsell::set_dismissed(true, cx);
                },
                cx,
            )
        });

        Self {
            active_view,
            workspace,
            user_store,
            project: project.clone(),
            fs: fs.clone(),
            language_registry,
            thread_store: thread_store.clone(),
            _default_model_subscription,
            context_store,
            prompt_store,
            configuration: None,
            configuration_subscription: None,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            inline_assist_context_store,
            previous_view: None,
            acp_message_history: Default::default(),
            history_store: history_store.clone(),
            history: cx.new(|cx| ThreadHistory::new(weak_self, history_store, window, cx)),
            hovered_recent_history_item: None,
            new_thread_menu_handle: PopoverMenuHandle::default(),
            agent_panel_menu_handle: PopoverMenuHandle::default(),
            assistant_navigation_menu_handle: PopoverMenuHandle::default(),
            assistant_navigation_menu: None,
            width: None,
            height: None,
            zoomed: false,
            pending_serialization: None,
            onboarding,
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
            && !DisableAiSettings::get_global(cx).disable_ai
        {
            workspace.toggle_panel_focus::<Self>(window, cx);
        }
    }

    pub(crate) fn local_timezone(&self) -> UtcOffset {
        self.local_timezone
    }

    pub(crate) fn prompt_store(&self) -> &Option<Entity<PromptStore>> {
        &self.prompt_store
    }

    pub(crate) fn inline_assist_context_store(&self) -> &Entity<ContextStore> {
        &self.inline_assist_context_store
    }

    pub(crate) fn thread_store(&self) -> &Entity<ThreadStore> {
        &self.thread_store
    }

    pub(crate) fn text_thread_store(&self) -> &Entity<TextThreadStore> {
        &self.context_store
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        match &self.active_view {
            ActiveView::Thread { thread, .. } => {
                thread.update(cx, |thread, cx| thread.cancel_last_completion(window, cx));
            }
            ActiveView::ExternalAgentThread { thread_view, .. } => {
                thread_view.update(cx, |thread_element, cx| thread_element.cancel(cx));
            }
            ActiveView::TextThread { .. } | ActiveView::History | ActiveView::Configuration => {}
        }
    }

    fn active_message_editor(&self) -> Option<&Entity<MessageEditor>> {
        match &self.active_view {
            ActiveView::Thread { message_editor, .. } => Some(message_editor),
            ActiveView::ExternalAgentThread { .. }
            | ActiveView::TextThread { .. }
            | ActiveView::History
            | ActiveView::Configuration => None,
        }
    }

    fn new_thread(&mut self, action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        // Preserve chat box text when using creating new thread
        let preserved_text = self
            .active_message_editor()
            .map(|editor| editor.read(cx).get_text(cx).trim().to_string());

        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(cx));

        let context_store = cx.new(|_cx| {
            ContextStore::new(
                self.project.downgrade(),
                Some(self.thread_store.downgrade()),
            )
        });

        if let Some(other_thread_id) = action.from_thread_id.clone() {
            let other_thread_task = self.thread_store.update(cx, |this, cx| {
                this.open_thread(&other_thread_id, window, cx)
            });

            cx.spawn({
                let context_store = context_store.clone();

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

        let active_thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.thread_store.clone(),
                self.context_store.clone(),
                context_store.clone(),
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        });

        let message_editor = cx.new(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                context_store.clone(),
                self.prompt_store.clone(),
                self.thread_store.downgrade(),
                self.context_store.downgrade(),
                Some(self.history_store.downgrade()),
                thread.clone(),
                window,
                cx,
            )
        });

        if let Some(text) = preserved_text {
            message_editor.update(cx, |editor, cx| {
                editor.set_text(text, window, cx);
            });
        }

        message_editor.focus_handle(cx).focus(window);

        let thread_view = ActiveView::thread(active_thread.clone(), message_editor, window, cx);
        self.set_active_view(thread_view, window, cx);

        AgentDiff::set_active_thread(&self.workspace, thread.clone(), window, cx);
    }

    fn new_prompt_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let context = self
            .context_store
            .update(cx, |context_store, cx| context_store.create(cx));
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        let context_editor = cx.new(|cx| {
            let mut editor = TextThreadEditor::for_context(
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
        });

        self.set_active_view(
            ActiveView::prompt_editor(
                context_editor.clone(),
                self.history_store.clone(),
                self.language_registry.clone(),
                window,
                cx,
            ),
            window,
            cx,
        );
        context_editor.focus_handle(cx).focus(window);
    }

    fn new_external_thread(
        &mut self,
        agent_choice: Option<crate::ExternalAgent>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let message_history = self.acp_message_history.clone();

        const LAST_USED_EXTERNAL_AGENT_KEY: &str = "agent_panel__last_used_external_agent";

        #[derive(Default, Serialize, Deserialize)]
        struct LastUsedExternalAgent {
            agent: crate::ExternalAgent,
        }

        cx.spawn_in(window, async move |this, cx| {
            let server: Rc<dyn AgentServer> = match agent_choice {
                Some(agent) => {
                    cx.background_spawn(async move {
                        if let Some(serialized) =
                            serde_json::to_string(&LastUsedExternalAgent { agent }).log_err()
                        {
                            KEY_VALUE_STORE
                                .write_kvp(LAST_USED_EXTERNAL_AGENT_KEY.to_string(), serialized)
                                .await
                                .log_err();
                        }
                    })
                    .detach();

                    agent.server()
                }
                None => cx
                    .background_spawn(async move {
                        KEY_VALUE_STORE.read_kvp(LAST_USED_EXTERNAL_AGENT_KEY)
                    })
                    .await
                    .log_err()
                    .flatten()
                    .and_then(|value| {
                        serde_json::from_str::<LastUsedExternalAgent>(&value).log_err()
                    })
                    .unwrap_or_default()
                    .agent
                    .server(),
            };

            this.update_in(cx, |this, window, cx| {
                let thread_view = cx.new(|cx| {
                    crate::acp::AcpThreadView::new(
                        server,
                        workspace.clone(),
                        project,
                        message_history,
                        MIN_EDITOR_LINES,
                        Some(MAX_EDITOR_LINES),
                        window,
                        cx,
                    )
                });

                this.set_active_view(ActiveView::ExternalAgentThread { thread_view }, window, cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn deploy_rules_library(
        &mut self,
        action: &OpenRulesLibrary,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        open_rules_library(
            self.language_registry.clone(),
            Box::new(PromptLibraryInlineAssist::new(self.workspace.clone())),
            Rc::new(|| {
                Rc::new(SlashCommandCompletionProvider::new(
                    Arc::new(SlashCommandWorkingSet::default()),
                    None,
                    None,
                ))
            }),
            action
                .prompt_to_select
                .map(|uuid| UserPromptId(uuid).into()),
            cx,
        )
        .detach_and_log_err(cx);
    }

    fn open_history(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.active_view, ActiveView::History) {
            if let Some(previous_view) = self.previous_view.take() {
                self.set_active_view(previous_view, window, cx);
            }
        } else {
            self.thread_store
                .update(cx, |thread_store, cx| thread_store.reload(cx))
                .detach_and_log_err(cx);
            self.set_active_view(ActiveView::History, window, cx);
        }
        cx.notify();
    }

    pub(crate) fn open_saved_prompt_editor(
        &mut self,
        path: Arc<Path>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let context = self
            .context_store
            .update(cx, |store, cx| store.open_local_context(path, cx));
        cx.spawn_in(window, async move |this, cx| {
            let context = context.await?;
            this.update_in(cx, |this, window, cx| {
                this.open_prompt_editor(context, window, cx);
            })
        })
    }

    pub(crate) fn open_prompt_editor(
        &mut self,
        context: Entity<AssistantContext>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project.clone(), cx)
            .log_err()
            .flatten();
        let editor = cx.new(|cx| {
            TextThreadEditor::for_context(
                context,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                window,
                cx,
            )
        });
        self.set_active_view(
            ActiveView::prompt_editor(
                editor.clone(),
                self.history_store.clone(),
                self.language_registry.clone(),
                window,
                cx,
            ),
            window,
            cx,
        );
    }

    pub(crate) fn open_thread_by_id(
        &mut self,
        thread_id: &ThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let open_thread_task = self
            .thread_store
            .update(cx, |this, cx| this.open_thread(thread_id, window, cx));
        cx.spawn_in(window, async move |this, cx| {
            let thread = open_thread_task.await?;
            this.update_in(cx, |this, window, cx| {
                this.open_thread(thread, window, cx);
                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    pub(crate) fn open_thread(
        &mut self,
        thread: Entity<Thread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_store = cx.new(|_cx| {
            ContextStore::new(
                self.project.downgrade(),
                Some(self.thread_store.downgrade()),
            )
        });

        let active_thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.thread_store.clone(),
                self.context_store.clone(),
                context_store.clone(),
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        });

        let message_editor = cx.new(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                context_store,
                self.prompt_store.clone(),
                self.thread_store.downgrade(),
                self.context_store.downgrade(),
                Some(self.history_store.downgrade()),
                thread.clone(),
                window,
                cx,
            )
        });
        message_editor.focus_handle(cx).focus(window);

        let thread_view = ActiveView::thread(active_thread.clone(), message_editor, window, cx);
        self.set_active_view(thread_view, window, cx);
        AgentDiff::set_active_thread(&self.workspace, thread.clone(), window, cx);
    }

    pub fn go_back(&mut self, _: &workspace::GoBack, window: &mut Window, cx: &mut Context<Self>) {
        match self.active_view {
            ActiveView::Configuration | ActiveView::History => {
                if let Some(previous_view) = self.previous_view.take() {
                    self.active_view = previous_view;

                    match &self.active_view {
                        ActiveView::Thread { message_editor, .. } => {
                            message_editor.focus_handle(cx).focus(window);
                        }
                        ActiveView::ExternalAgentThread { thread_view } => {
                            thread_view.focus_handle(cx).focus(window);
                        }
                        ActiveView::TextThread { context_editor, .. } => {
                            context_editor.focus_handle(cx).focus(window);
                        }
                        ActiveView::History | ActiveView::Configuration => {}
                    }
                }
                cx.notify();
            }
            _ => {}
        }
    }

    pub fn toggle_navigation_menu(
        &mut self,
        _: &ToggleNavigationMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.assistant_navigation_menu_handle.toggle(window, cx);
    }

    pub fn toggle_options_menu(
        &mut self,
        _: &ToggleOptionsMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.agent_panel_menu_handle.toggle(window, cx);
    }

    pub fn increase_font_size(
        &mut self,
        action: &IncreaseBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_font_size_action(action.persist, px(1.0), cx);
    }

    pub fn decrease_font_size(
        &mut self,
        action: &DecreaseBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_font_size_action(action.persist, px(-1.0), cx);
    }

    fn handle_font_size_action(&mut self, persist: bool, delta: Pixels, cx: &mut Context<Self>) {
        match self.active_view.which_font_size_used() {
            WhichFontSize::AgentFont => {
                if persist {
                    update_settings_file::<ThemeSettings>(
                        self.fs.clone(),
                        cx,
                        move |settings, cx| {
                            let agent_font_size =
                                ThemeSettings::get_global(cx).agent_font_size(cx) + delta;
                            let _ = settings
                                .agent_font_size
                                .insert(theme::clamp_font_size(agent_font_size).0);
                        },
                    );
                } else {
                    theme::adjust_agent_font_size(cx, |size| {
                        *size += delta;
                    });
                }
            }
            WhichFontSize::BufferFont => {
                // Prompt editor uses the buffer font size, so allow the action to propagate to the
                // default handler that changes that font size.
                cx.propagate();
            }
            WhichFontSize::None => {}
        }
    }

    pub fn reset_font_size(
        &mut self,
        action: &ResetBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if action.persist {
            update_settings_file::<ThemeSettings>(self.fs.clone(), cx, move |settings, _| {
                settings.agent_font_size = None;
            });
        } else {
            theme::reset_agent_font_size(cx);
        }
    }

    pub fn toggle_zoom(&mut self, _: &ToggleZoom, window: &mut Window, cx: &mut Context<Self>) {
        if self.zoomed {
            cx.emit(PanelEvent::ZoomOut);
        } else {
            if !self.focus_handle(cx).contains_focused(window, cx) {
                cx.focus_self(window);
            }
            cx.emit(PanelEvent::ZoomIn);
        }
    }

    pub fn open_agent_diff(
        &mut self,
        _: &OpenAgentDiff,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.active_view {
            ActiveView::Thread { thread, .. } => {
                let thread = thread.read(cx).thread().clone();
                self.workspace
                    .update(cx, |workspace, cx| {
                        AgentDiffPane::deploy_in_workspace(
                            AgentDiffThread::Native(thread),
                            workspace,
                            window,
                            cx,
                        )
                    })
                    .log_err();
            }
            ActiveView::ExternalAgentThread { .. }
            | ActiveView::TextThread { .. }
            | ActiveView::History
            | ActiveView::Configuration => {}
        }
    }

    pub(crate) fn open_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let context_server_store = self.project.read(cx).context_server_store();
        let tools = self.thread_store.read(cx).tools();
        let fs = self.fs.clone();

        self.set_active_view(ActiveView::Configuration, window, cx);
        self.configuration = Some(cx.new(|cx| {
            AgentConfiguration::new(
                fs,
                context_server_store,
                tools,
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        }));

        if let Some(configuration) = self.configuration.as_ref() {
            self.configuration_subscription = Some(cx.subscribe_in(
                configuration,
                window,
                Self::handle_agent_configuration_event,
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
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        match &self.active_view {
            ActiveView::Thread { thread, .. } => {
                active_thread::open_active_thread_as_markdown(
                    thread.read(cx).thread().clone(),
                    workspace,
                    window,
                    cx,
                )
                .detach_and_log_err(cx);
            }
            ActiveView::ExternalAgentThread { thread_view } => {
                thread_view
                    .update(cx, |thread_view, cx| {
                        thread_view.open_thread_as_markdown(workspace, window, cx)
                    })
                    .detach_and_log_err(cx);
            }
            ActiveView::TextThread { .. } | ActiveView::History | ActiveView::Configuration => {}
        }
    }

    fn handle_agent_configuration_event(
        &mut self,
        _entity: &Entity<AgentConfiguration>,
        event: &AssistantConfigurationEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            AssistantConfigurationEvent::NewThread(provider) => {
                if LanguageModelRegistry::read_global(cx)
                    .default_model()
                    .map_or(true, |model| model.provider.id() != provider.id())
                {
                    if let Some(model) = provider.default_model(cx) {
                        update_settings_file::<AgentSettings>(
                            self.fs.clone(),
                            cx,
                            move |settings, _| settings.set_model(model),
                        );
                    }
                }

                self.new_thread(&NewThread::default(), window, cx);
                if let Some((thread, model)) =
                    self.active_thread(cx).zip(provider.default_model(cx))
                {
                    thread.update(cx, |thread, cx| {
                        thread.set_configured_model(
                            Some(ConfiguredModel {
                                provider: provider.clone(),
                                model,
                            }),
                            cx,
                        );
                    });
                }
            }
        }
    }

    pub(crate) fn active_thread(&self, cx: &App) -> Option<Entity<Thread>> {
        match &self.active_view {
            ActiveView::Thread { thread, .. } => Some(thread.read(cx).thread().clone()),
            _ => None,
        }
    }

    pub(crate) fn delete_thread(
        &mut self,
        thread_id: &ThreadId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.thread_store
            .update(cx, |this, cx| this.delete_thread(thread_id, cx))
    }

    fn continue_conversation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let ActiveView::Thread { thread, .. } = &self.active_view else {
            return;
        };

        let thread_state = thread.read(cx).thread().read(cx);
        if !thread_state.tool_use_limit_reached() {
            return;
        }

        let model = thread_state.configured_model().map(|cm| cm.model.clone());
        if let Some(model) = model {
            thread.update(cx, |active_thread, cx| {
                active_thread.thread().update(cx, |thread, cx| {
                    thread.insert_invisible_continue_message(cx);
                    thread.advance_prompt_id();
                    thread.send_to_model(
                        model,
                        CompletionIntent::UserPrompt,
                        Some(window.window_handle()),
                        cx,
                    );
                });
            });
        } else {
            log::warn!("No configured model available for continuation");
        }
    }

    fn toggle_burn_mode(
        &mut self,
        _: &ToggleBurnMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ActiveView::Thread { thread, .. } = &self.active_view else {
            return;
        };

        thread.update(cx, |active_thread, cx| {
            active_thread.thread().update(cx, |thread, _cx| {
                let current_mode = thread.completion_mode();

                thread.set_completion_mode(match current_mode {
                    CompletionMode::Burn => CompletionMode::Normal,
                    CompletionMode::Normal => CompletionMode::Burn,
                });
            });
        });
    }

    pub(crate) fn active_context_editor(&self) -> Option<Entity<TextThreadEditor>> {
        match &self.active_view {
            ActiveView::TextThread { context_editor, .. } => Some(context_editor.clone()),
            _ => None,
        }
    }

    pub(crate) fn delete_context(
        &mut self,
        path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.context_store
            .update(cx, |this, cx| this.delete_local_context(path, cx))
    }

    fn set_active_view(
        &mut self,
        new_view: ActiveView,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let current_is_history = matches!(self.active_view, ActiveView::History);
        let new_is_history = matches!(new_view, ActiveView::History);

        let current_is_config = matches!(self.active_view, ActiveView::Configuration);
        let new_is_config = matches!(new_view, ActiveView::Configuration);

        let current_is_special = current_is_history || current_is_config;
        let new_is_special = new_is_history || new_is_config;

        match &self.active_view {
            ActiveView::Thread { thread, .. } => {
                let thread = thread.read(cx);
                if thread.is_empty() {
                    let id = thread.thread().read(cx).id().clone();
                    self.history_store.update(cx, |store, cx| {
                        store.remove_recently_opened_thread(id, cx);
                    });
                }
            }
            _ => {}
        }

        match &new_view {
            ActiveView::Thread { thread, .. } => self.history_store.update(cx, |store, cx| {
                let id = thread.read(cx).thread().read(cx).id().clone();
                store.push_recently_opened_entry(HistoryEntryId::Thread(id), cx);
            }),
            ActiveView::TextThread { context_editor, .. } => {
                self.history_store.update(cx, |store, cx| {
                    if let Some(path) = context_editor.read(cx).context().read(cx).path() {
                        store.push_recently_opened_entry(HistoryEntryId::Context(path.clone()), cx)
                    }
                })
            }
            ActiveView::ExternalAgentThread { .. } => {}
            ActiveView::History | ActiveView::Configuration => {}
        }

        if current_is_special && !new_is_special {
            self.active_view = new_view;
        } else if !current_is_special && new_is_special {
            self.previous_view = Some(std::mem::replace(&mut self.active_view, new_view));
        } else {
            if !new_is_special {
                self.previous_view = None;
            }
            self.active_view = new_view;
        }

        self.acp_message_history.borrow_mut().reset_position();

        self.focus_handle(cx).focus(window);
    }

    fn populate_recently_opened_menu_section(
        mut menu: ContextMenu,
        panel: Entity<Self>,
        cx: &mut Context<ContextMenu>,
    ) -> ContextMenu {
        let entries = panel
            .read(cx)
            .history_store
            .read(cx)
            .recently_opened_entries(cx);

        if entries.is_empty() {
            return menu;
        }

        menu = menu.header("Recently Opened");

        for entry in entries {
            let title = entry.title().clone();
            let id = entry.id();

            menu = menu.entry_with_end_slot_on_hover(
                title,
                None,
                {
                    let panel = panel.downgrade();
                    let id = id.clone();
                    move |window, cx| {
                        let id = id.clone();
                        panel
                            .update(cx, move |this, cx| match id {
                                HistoryEntryId::Thread(id) => this
                                    .open_thread_by_id(&id, window, cx)
                                    .detach_and_log_err(cx),
                                HistoryEntryId::Context(path) => this
                                    .open_saved_prompt_editor(path.clone(), window, cx)
                                    .detach_and_log_err(cx),
                            })
                            .ok();
                    }
                },
                IconName::Close,
                "Close Entry".into(),
                {
                    let panel = panel.downgrade();
                    let id = id.clone();
                    move |_window, cx| {
                        panel
                            .update(cx, |this, cx| {
                                this.history_store.update(cx, |history_store, cx| {
                                    history_store.remove_recently_opened_entry(&id, cx);
                                });
                            })
                            .ok();
                    }
                },
            );
        }

        menu = menu.separator();

        menu
    }
}

impl Focusable for AgentPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.active_view {
            ActiveView::Thread { message_editor, .. } => message_editor.focus_handle(cx),
            ActiveView::ExternalAgentThread { thread_view, .. } => thread_view.focus_handle(cx),
            ActiveView::History => self.history.focus_handle(cx),
            ActiveView::TextThread { context_editor, .. } => context_editor.focus_handle(cx),
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

fn agent_panel_dock_position(cx: &App) -> DockPosition {
    match AgentSettings::get_global(cx).dock {
        AgentDockPosition::Left => DockPosition::Left,
        AgentDockPosition::Bottom => DockPosition::Bottom,
        AgentDockPosition::Right => DockPosition::Right,
    }
}

impl EventEmitter<PanelEvent> for AgentPanel {}

impl Panel for AgentPanel {
    fn persistent_name() -> &'static str {
        "AgentPanel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        agent_panel_dock_position(cx)
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position != DockPosition::Bottom
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file::<AgentSettings>(self.fs.clone(), cx, move |settings, _| {
            let dock = match position {
                DockPosition::Left => AgentDockPosition::Left,
                DockPosition::Bottom => AgentDockPosition::Bottom,
                DockPosition::Right => AgentDockPosition::Right,
            };
            settings.set_dock(dock);
        });
    }

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = AgentSettings::get_global(cx);
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
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        (self.enabled(cx) && AgentSettings::get_global(cx).button).then_some(IconName::ZedAssistant)
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
        DisableAiSettings::get_global(cx).disable_ai.not() && AgentSettings::get_global(cx).enabled
    }

    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        self.zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomed = zoomed;
        cx.notify();
    }
}

impl AgentPanel {
    fn render_title_view(&self, _window: &mut Window, cx: &Context<Self>) -> AnyElement {
        const LOADING_SUMMARY_PLACEHOLDER: &str = "Loading Summary…";

        let content = match &self.active_view {
            ActiveView::Thread {
                thread: active_thread,
                change_title_editor,
                ..
            } => {
                let state = {
                    let active_thread = active_thread.read(cx);
                    if active_thread.is_empty() {
                        &ThreadSummary::Pending
                    } else {
                        active_thread.summary(cx)
                    }
                };

                match state {
                    ThreadSummary::Pending => Label::new(ThreadSummary::DEFAULT.clone())
                        .truncate()
                        .into_any_element(),
                    ThreadSummary::Generating => Label::new(LOADING_SUMMARY_PLACEHOLDER)
                        .truncate()
                        .into_any_element(),
                    ThreadSummary::Ready(_) => div()
                        .w_full()
                        .child(change_title_editor.clone())
                        .into_any_element(),
                    ThreadSummary::Error => h_flex()
                        .w_full()
                        .child(change_title_editor.clone())
                        .child(
                            ui::IconButton::new("retry-summary-generation", IconName::RotateCcw)
                                .on_click({
                                    let active_thread = active_thread.clone();
                                    move |_, _window, cx| {
                                        active_thread.update(cx, |thread, cx| {
                                            thread.regenerate_summary(cx);
                                        });
                                    }
                                })
                                .tooltip(move |_window, cx| {
                                    cx.new(|_| {
                                        Tooltip::new("Failed to generate title")
                                            .meta("Click to try again")
                                    })
                                    .into()
                                }),
                        )
                        .into_any_element(),
                }
            }
            ActiveView::ExternalAgentThread { thread_view } => {
                Label::new(thread_view.read(cx).title(cx))
                    .truncate()
                    .into_any_element()
            }
            ActiveView::TextThread {
                title_editor,
                context_editor,
                ..
            } => {
                let summary = context_editor.read(cx).context().read(cx).summary();

                match summary {
                    ContextSummary::Pending => Label::new(ContextSummary::DEFAULT)
                        .truncate()
                        .into_any_element(),
                    ContextSummary::Content(summary) => {
                        if summary.done {
                            div()
                                .w_full()
                                .child(title_editor.clone())
                                .into_any_element()
                        } else {
                            Label::new(LOADING_SUMMARY_PLACEHOLDER)
                                .truncate()
                                .into_any_element()
                        }
                    }
                    ContextSummary::Error => h_flex()
                        .w_full()
                        .child(title_editor.clone())
                        .child(
                            ui::IconButton::new("retry-summary-generation", IconName::RotateCcw)
                                .on_click({
                                    let context_editor = context_editor.clone();
                                    move |_, _window, cx| {
                                        context_editor.update(cx, |context_editor, cx| {
                                            context_editor.regenerate_summary(cx);
                                        });
                                    }
                                })
                                .tooltip(move |_window, cx| {
                                    cx.new(|_| {
                                        Tooltip::new("Failed to generate title")
                                            .meta("Click to try again")
                                    })
                                    .into()
                                }),
                        )
                        .into_any_element(),
                }
            }
            ActiveView::History => Label::new("History").truncate().into_any_element(),
            ActiveView::Configuration => Label::new("Settings").truncate().into_any_element(),
        };

        h_flex()
            .key_context("TitleEditor")
            .id("TitleEditor")
            .flex_grow()
            .w_full()
            .max_w_full()
            .overflow_x_scroll()
            .child(content)
            .into_any()
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let user_store = self.user_store.read(cx);
        let usage = user_store.model_request_usage();

        let account_url = zed_urls::account_url(cx);

        let focus_handle = self.focus_handle(cx);

        let go_back_button = div().child(
            IconButton::new("go-back", IconName::ArrowLeft)
                .icon_size(IconSize::Small)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.go_back(&workspace::GoBack, window, cx);
                }))
                .tooltip({
                    let focus_handle = focus_handle.clone();
                    move |window, cx| {
                        Tooltip::for_action_in(
                            "Go Back",
                            &workspace::GoBack,
                            &focus_handle,
                            window,
                            cx,
                        )
                    }
                }),
        );

        let recent_entries_menu = div().child(
            PopoverMenu::new("agent-nav-menu")
                .trigger_with_tooltip(
                    IconButton::new("agent-nav-menu", IconName::MenuAlt)
                        .icon_size(IconSize::Small)
                        .style(ui::ButtonStyle::Subtle),
                    {
                        let focus_handle = focus_handle.clone();
                        move |window, cx| {
                            Tooltip::for_action_in(
                                "Toggle Panel Menu",
                                &ToggleNavigationMenu,
                                &focus_handle,
                                window,
                                cx,
                            )
                        }
                    },
                )
                .anchor(Corner::TopLeft)
                .with_handle(self.assistant_navigation_menu_handle.clone())
                .menu({
                    let menu = self.assistant_navigation_menu.clone();
                    move |window, cx| {
                        if let Some(menu) = menu.as_ref() {
                            menu.update(cx, |_, cx| {
                                cx.defer_in(window, |menu, window, cx| {
                                    menu.rebuild(window, cx);
                                });
                            })
                        }
                        menu.clone()
                    }
                }),
        );

        let full_screen_label = if self.is_zoomed(window, cx) {
            "Disable Full Screen"
        } else {
            "Enable Full Screen"
        };

        let active_thread = match &self.active_view {
            ActiveView::Thread { thread, .. } => Some(thread.read(cx).thread().clone()),
            ActiveView::ExternalAgentThread { .. }
            | ActiveView::TextThread { .. }
            | ActiveView::History
            | ActiveView::Configuration => None,
        };

        let new_thread_menu = PopoverMenu::new("new_thread_menu")
            .trigger_with_tooltip(
                IconButton::new("new_thread_menu_btn", IconName::Plus).icon_size(IconSize::Small),
                Tooltip::text("New Thread…"),
            )
            .anchor(Corner::TopRight)
            .with_handle(self.new_thread_menu_handle.clone())
            .menu({
                let focus_handle = focus_handle.clone();
                move |window, cx| {
                    let active_thread = active_thread.clone();
                    Some(ContextMenu::build(window, cx, |mut menu, _window, cx| {
                        menu = menu
                            .context(focus_handle.clone())
                            .when(cx.has_flag::<feature_flags::AcpFeatureFlag>(), |this| {
                                this.header("Zed Agent")
                            })
                            .when_some(active_thread, |this, active_thread| {
                                let thread = active_thread.read(cx);

                                if !thread.is_empty() {
                                    let thread_id = thread.id().clone();
                                    this.item(
                                        ContextMenuEntry::new("New From Summary")
                                            .icon(IconName::ThreadFromSummary)
                                            .icon_color(Color::Muted)
                                            .handler(move |window, cx| {
                                                window.dispatch_action(
                                                    Box::new(NewThread {
                                                        from_thread_id: Some(thread_id.clone()),
                                                    }),
                                                    cx,
                                                );
                                            }),
                                    )
                                } else {
                                    this
                                }
                            })
                            .item(
                                ContextMenuEntry::new("New Thread")
                                    .icon(IconName::Thread)
                                    .icon_color(Color::Muted)
                                    .action(NewThread::default().boxed_clone())
                                    .handler(move |window, cx| {
                                        window.dispatch_action(
                                            NewThread::default().boxed_clone(),
                                            cx,
                                        );
                                    }),
                            )
                            .item(
                                ContextMenuEntry::new("New Text Thread")
                                    .icon(IconName::TextThread)
                                    .icon_color(Color::Muted)
                                    .action(NewTextThread.boxed_clone())
                                    .handler(move |window, cx| {
                                        window.dispatch_action(NewTextThread.boxed_clone(), cx);
                                    }),
                            )
                            .when(cx.has_flag::<feature_flags::AcpFeatureFlag>(), |this| {
                                this.separator()
                                    .header("External Agents")
                                    .item(
                                        ContextMenuEntry::new("New Gemini Thread")
                                            .icon(IconName::AiGemini)
                                            .icon_color(Color::Muted)
                                            .handler(move |window, cx| {
                                                window.dispatch_action(
                                                    NewExternalAgentThread {
                                                        agent: Some(crate::ExternalAgent::Gemini),
                                                    }
                                                    .boxed_clone(),
                                                    cx,
                                                );
                                            }),
                                    )
                                    .item(
                                        ContextMenuEntry::new("New Claude Code Thread")
                                            .icon(IconName::AiClaude)
                                            .icon_color(Color::Muted)
                                            .handler(move |window, cx| {
                                                window.dispatch_action(
                                                    NewExternalAgentThread {
                                                        agent: Some(
                                                            crate::ExternalAgent::ClaudeCode,
                                                        ),
                                                    }
                                                    .boxed_clone(),
                                                    cx,
                                                );
                                            }),
                                    )
                                    .item(
                                        ContextMenuEntry::new("New Native Agent Thread")
                                            .icon(IconName::ZedAssistant)
                                            .icon_color(Color::Muted)
                                            .handler(move |window, cx| {
                                                window.dispatch_action(
                                                    NewExternalAgentThread {
                                                        agent: Some(
                                                            crate::ExternalAgent::NativeAgent,
                                                        ),
                                                    }
                                                    .boxed_clone(),
                                                    cx,
                                                );
                                            }),
                                    )
                            });
                        menu
                    }))
                }
            });

        let agent_panel_menu = PopoverMenu::new("agent-options-menu")
            .trigger_with_tooltip(
                IconButton::new("agent-options-menu", IconName::Ellipsis)
                    .icon_size(IconSize::Small),
                {
                    let focus_handle = focus_handle.clone();
                    move |window, cx| {
                        Tooltip::for_action_in(
                            "Toggle Agent Menu",
                            &ToggleOptionsMenu,
                            &focus_handle,
                            window,
                            cx,
                        )
                    }
                },
            )
            .anchor(Corner::TopRight)
            .with_handle(self.agent_panel_menu_handle.clone())
            .menu({
                let focus_handle = focus_handle.clone();
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |mut menu, _window, _| {
                        menu = menu.context(focus_handle.clone());
                        if let Some(usage) = usage {
                            menu = menu
                                .header_with_link("Prompt Usage", "Manage", account_url.clone())
                                .custom_entry(
                                    move |_window, cx| {
                                        let used_percentage = match usage.limit {
                                            UsageLimit::Limited(limit) => {
                                                Some((usage.amount as f32 / limit as f32) * 100.)
                                            }
                                            UsageLimit::Unlimited => None,
                                        };

                                        h_flex()
                                            .flex_1()
                                            .gap_1p5()
                                            .children(used_percentage.map(|percent| {
                                                ProgressBar::new("usage", percent, 100., cx)
                                            }))
                                            .child(
                                                Label::new(match usage.limit {
                                                    UsageLimit::Limited(limit) => {
                                                        format!("{} / {limit}", usage.amount)
                                                    }
                                                    UsageLimit::Unlimited => {
                                                        format!("{} / ∞", usage.amount)
                                                    }
                                                })
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                            )
                                            .into_any_element()
                                    },
                                    move |_, cx| cx.open_url(&zed_urls::account_url(cx)),
                                )
                                .separator()
                        }

                        menu = menu
                            .header("MCP Servers")
                            .action(
                                "View Server Extensions",
                                Box::new(zed_actions::Extensions {
                                    category_filter: Some(
                                        zed_actions::ExtensionCategoryFilter::ContextServers,
                                    ),
                                    id: None,
                                }),
                            )
                            .action("Add Custom Server…", Box::new(AddContextServer))
                            .separator();

                        menu = menu
                            .action("Rules…", Box::new(OpenRulesLibrary::default()))
                            .action("Settings", Box::new(OpenSettings))
                            .separator()
                            .action(full_screen_label, Box::new(ToggleZoom));
                        menu
                    }))
                }
            });

        h_flex()
            .id("assistant-toolbar")
            .h(Tab::container_height(cx))
            .max_w_full()
            .flex_none()
            .justify_between()
            .gap_2()
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .size_full()
                    .pl_1()
                    .gap_1()
                    .child(match &self.active_view {
                        ActiveView::History | ActiveView::Configuration => go_back_button,
                        _ => recent_entries_menu,
                    })
                    .child(self.render_title_view(window, cx)),
            )
            .child(
                h_flex()
                    .h_full()
                    .gap_2()
                    .children(self.render_token_count(cx))
                    .child(
                        h_flex()
                            .h_full()
                            .gap(DynamicSpacing::Base02.rems(cx))
                            .px(DynamicSpacing::Base08.rems(cx))
                            .border_l_1()
                            .border_color(cx.theme().colors().border)
                            .child(new_thread_menu)
                            .child(agent_panel_menu),
                    ),
            )
    }

    fn render_token_count(&self, cx: &App) -> Option<AnyElement> {
        match &self.active_view {
            ActiveView::Thread {
                thread,
                message_editor,
                ..
            } => {
                let active_thread = thread.read(cx);
                let message_editor = message_editor.read(cx);

                let editor_empty = message_editor.is_editor_fully_empty(cx);

                if active_thread.is_empty() && editor_empty {
                    return None;
                }

                let thread = active_thread.thread().read(cx);
                let is_generating = thread.is_generating();
                let conversation_token_usage = thread.total_token_usage()?;

                let (total_token_usage, is_estimating) =
                    if let Some((editing_message_id, unsent_tokens)) =
                        active_thread.editing_message_id()
                    {
                        let combined = thread
                            .token_usage_up_to_message(editing_message_id)
                            .add(unsent_tokens);

                        (combined, unsent_tokens > 0)
                    } else {
                        let unsent_tokens =
                            message_editor.last_estimated_token_count().unwrap_or(0);
                        let combined = conversation_token_usage.add(unsent_tokens);

                        (combined, unsent_tokens > 0)
                    };

                let is_waiting_to_update_token_count =
                    message_editor.is_waiting_to_update_token_count();

                if total_token_usage.total == 0 {
                    return None;
                }

                let token_color = match total_token_usage.ratio() {
                    TokenUsageRatio::Normal if is_estimating => Color::Default,
                    TokenUsageRatio::Normal => Color::Muted,
                    TokenUsageRatio::Warning => Color::Warning,
                    TokenUsageRatio::Exceeded => Color::Error,
                };

                let token_count = h_flex()
                    .id("token-count")
                    .flex_shrink_0()
                    .gap_0p5()
                    .when(!is_generating && is_estimating, |parent| {
                        parent
                            .child(
                                h_flex()
                                    .mr_1()
                                    .size_2p5()
                                    .justify_center()
                                    .rounded_full()
                                    .bg(cx.theme().colors().text.opacity(0.1))
                                    .child(
                                        div().size_1().rounded_full().bg(cx.theme().colors().text),
                                    ),
                            )
                            .tooltip(move |window, cx| {
                                Tooltip::with_meta(
                                    "Estimated New Token Count",
                                    None,
                                    format!(
                                        "Current Conversation Tokens: {}",
                                        humanize_token_count(conversation_token_usage.total)
                                    ),
                                    window,
                                    cx,
                                )
                            })
                    })
                    .child(
                        Label::new(humanize_token_count(total_token_usage.total))
                            .size(LabelSize::Small)
                            .color(token_color)
                            .map(|label| {
                                if is_generating || is_waiting_to_update_token_count {
                                    label
                                        .with_animation(
                                            "used-tokens-label",
                                            Animation::new(Duration::from_secs(2))
                                                .repeat()
                                                .with_easing(pulsating_between(0.6, 1.)),
                                            |label, delta| label.alpha(delta),
                                        )
                                        .into_any()
                                } else {
                                    label.into_any_element()
                                }
                            }),
                    )
                    .child(Label::new("/").size(LabelSize::Small).color(Color::Muted))
                    .child(
                        Label::new(humanize_token_count(total_token_usage.max))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any();

                Some(token_count)
            }
            ActiveView::TextThread { context_editor, .. } => {
                let element = render_remaining_tokens(context_editor, cx)?;

                Some(element.into_any_element())
            }
            ActiveView::ExternalAgentThread { .. }
            | ActiveView::History
            | ActiveView::Configuration => {
                return None;
            }
        }
    }

    fn should_render_trial_end_upsell(&self, cx: &mut Context<Self>) -> bool {
        if TrialEndUpsell::dismissed() {
            return false;
        }

        match &self.active_view {
            ActiveView::Thread { thread, .. } => {
                if thread
                    .read(cx)
                    .thread()
                    .read(cx)
                    .configured_model()
                    .map_or(false, |model| {
                        model.provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
                    })
                {
                    return false;
                }
            }
            ActiveView::TextThread { .. } => {
                if LanguageModelRegistry::global(cx)
                    .read(cx)
                    .default_model()
                    .map_or(false, |model| {
                        model.provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
                    })
                {
                    return false;
                }
            }
            ActiveView::ExternalAgentThread { .. }
            | ActiveView::History
            | ActiveView::Configuration => return false,
        }

        let plan = self.user_store.read(cx).plan();
        let has_previous_trial = self.user_store.read(cx).trial_started_at().is_some();

        matches!(plan, Some(Plan::ZedFree)) && has_previous_trial
    }

    fn should_render_onboarding(&self, cx: &mut Context<Self>) -> bool {
        if OnboardingUpsell::dismissed() {
            return false;
        }

        match &self.active_view {
            ActiveView::Thread { .. } | ActiveView::TextThread { .. } => {
                let history_is_empty = self
                    .history_store
                    .update(cx, |store, cx| store.recent_entries(1, cx).is_empty());

                let has_configured_non_zed_providers = LanguageModelRegistry::read_global(cx)
                    .providers()
                    .iter()
                    .any(|provider| {
                        provider.is_authenticated(cx)
                            && provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
                    });

                history_is_empty || !has_configured_non_zed_providers
            }
            ActiveView::ExternalAgentThread { .. }
            | ActiveView::History
            | ActiveView::Configuration => false,
        }
    }

    fn render_onboarding(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_onboarding(cx) {
            return None;
        }

        let thread_view = matches!(&self.active_view, ActiveView::Thread { .. });
        let text_thread_view = matches!(&self.active_view, ActiveView::TextThread { .. });

        Some(
            div()
                .when(thread_view, |this| {
                    this.size_full().bg(cx.theme().colors().panel_background)
                })
                .when(text_thread_view, |this| {
                    this.bg(cx.theme().colors().editor_background)
                })
                .child(self.onboarding.clone()),
        )
    }

    fn render_backdrop(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .absolute()
            .inset_0()
            .bg(cx.theme().colors().panel_background)
            .opacity(0.8)
            .block_mouse_except_scroll()
    }

    fn render_trial_end_upsell(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_trial_end_upsell(cx) {
            return None;
        }

        Some(
            v_flex()
                .absolute()
                .inset_0()
                .size_full()
                .bg(cx.theme().colors().panel_background)
                .opacity(0.85)
                .block_mouse_except_scroll()
                .child(EndTrialUpsell::new(Arc::new({
                    let this = cx.entity();
                    move |_, cx| {
                        this.update(cx, |_this, cx| {
                            TrialEndUpsell::set_dismissed(true, cx);
                            cx.notify();
                        });
                    }
                }))),
        )
    }

    fn render_empty_state_section_header(
        &self,
        label: impl Into<SharedString>,
        action_slot: Option<AnyElement>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .mt_2()
            .pl_1p5()
            .pb_1()
            .w_full()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                Label::new(label.into())
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .children(action_slot)
    }

    fn render_thread_empty_state(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let recent_history = self
            .history_store
            .update(cx, |this, cx| this.recent_entries(6, cx));

        let model_registry = LanguageModelRegistry::read_global(cx);

        let configuration_error =
            model_registry.configuration_error(model_registry.default_model(), cx);

        let no_error = configuration_error.is_none();
        let focus_handle = self.focus_handle(cx);

        v_flex()
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .when(recent_history.is_empty(), |this| {
                this.child(
                    v_flex()
                        .size_full()
                        .mx_auto()
                        .justify_center()
                        .items_center()
                        .gap_1()
                        .child(h_flex().child(Headline::new("Welcome to the Agent Panel")))
                        .when(no_error, |parent| {
                            parent
                                .child(h_flex().child(
                                    Label::new("Ask and build anything.").color(Color::Muted),
                                ))
                                .child(
                                    v_flex()
                                        .mt_2()
                                        .gap_1()
                                        .max_w_48()
                                        .child(
                                            Button::new("context", "Add Context")
                                                .label_size(LabelSize::Small)
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
                                                    window.dispatch_action(
                                                        ToggleContextPicker.boxed_clone(),
                                                        cx,
                                                    )
                                                }),
                                        )
                                        .child(
                                            Button::new("mode", "Switch Model")
                                                .label_size(LabelSize::Small)
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
                                                    window.dispatch_action(
                                                        ToggleModelSelector.boxed_clone(),
                                                        cx,
                                                    )
                                                }),
                                        )
                                        .child(
                                            Button::new("settings", "View Settings")
                                                .label_size(LabelSize::Small)
                                                .icon(IconName::Settings)
                                                .icon_position(IconPosition::Start)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Muted)
                                                .full_width()
                                                .key_binding(KeyBinding::for_action_in(
                                                    &OpenSettings,
                                                    &focus_handle,
                                                    window,
                                                    cx,
                                                ))
                                                .on_click(|_event, window, cx| {
                                                    window.dispatch_action(
                                                        OpenSettings.boxed_clone(),
                                                        cx,
                                                    )
                                                }),
                                        ),
                                )
                        })
                        .when_some(configuration_error.as_ref(), |this, err| {
                            this.child(self.render_configuration_error(
                                err,
                                &focus_handle,
                                window,
                                cx,
                            ))
                        }),
                )
            })
            .when(!recent_history.is_empty(), |parent| {
                let focus_handle = focus_handle.clone();
                parent
                    .overflow_hidden()
                    .p_1p5()
                    .justify_end()
                    .gap_1()
                    .child(
                        self.render_empty_state_section_header(
                            "Recent",
                            Some(
                                Button::new("view-history", "View All")
                                    .style(ButtonStyle::Subtle)
                                    .label_size(LabelSize::Small)
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &OpenHistory,
                                            &self.focus_handle(cx),
                                            window,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                    )
                                    .on_click(move |_event, window, cx| {
                                        window.dispatch_action(OpenHistory.boxed_clone(), cx);
                                    })
                                    .into_any_element(),
                            ),
                            cx,
                        ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .children(recent_history.into_iter().enumerate().map(
                                |(index, entry)| {
                                    // TODO: Add keyboard navigation.
                                    let is_hovered =
                                        self.hovered_recent_history_item == Some(index);
                                    HistoryEntryElement::new(entry.clone(), cx.entity().downgrade())
                                        .hovered(is_hovered)
                                        .on_hover(cx.listener(
                                            move |this, is_hovered, _window, cx| {
                                                if *is_hovered {
                                                    this.hovered_recent_history_item = Some(index);
                                                } else if this.hovered_recent_history_item
                                                    == Some(index)
                                                {
                                                    this.hovered_recent_history_item = None;
                                                }
                                                cx.notify();
                                            },
                                        ))
                                        .into_any_element()
                                },
                            )),
                    )
                    .child(self.render_empty_state_section_header("Start", None, cx))
                    .child(
                        v_flex()
                            .p_1()
                            .gap_2()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_2()
                                    .child(
                                        NewThreadButton::new(
                                            "new-thread-btn",
                                            "New Thread",
                                            IconName::Thread,
                                        )
                                        .keybinding(KeyBinding::for_action_in(
                                            &NewThread::default(),
                                            &self.focus_handle(cx),
                                            window,
                                            cx,
                                        ))
                                        .on_click(
                                            |window, cx| {
                                                window.dispatch_action(
                                                    NewThread::default().boxed_clone(),
                                                    cx,
                                                )
                                            },
                                        ),
                                    )
                                    .child(
                                        NewThreadButton::new(
                                            "new-text-thread-btn",
                                            "New Text Thread",
                                            IconName::TextThread,
                                        )
                                        .keybinding(KeyBinding::for_action_in(
                                            &NewTextThread,
                                            &self.focus_handle(cx),
                                            window,
                                            cx,
                                        ))
                                        .on_click(
                                            |window, cx| {
                                                window.dispatch_action(Box::new(NewTextThread), cx)
                                            },
                                        ),
                                    ),
                            )
                            .when(cx.has_flag::<feature_flags::AcpFeatureFlag>(), |this| {
                                this.child(
                                    h_flex()
                                        .w_full()
                                        .gap_2()
                                        .child(
                                            NewThreadButton::new(
                                                "new-gemini-thread-btn",
                                                "New Gemini Thread",
                                                IconName::AiGemini,
                                            )
                                            // .keybinding(KeyBinding::for_action_in(
                                            //     &OpenHistory,
                                            //     &self.focus_handle(cx),
                                            //     window,
                                            //     cx,
                                            // ))
                                            .on_click(
                                                |window, cx| {
                                                    window.dispatch_action(
                                                        Box::new(NewExternalAgentThread {
                                                            agent: Some(
                                                                crate::ExternalAgent::Gemini,
                                                            ),
                                                        }),
                                                        cx,
                                                    )
                                                },
                                            ),
                                        )
                                        .child(
                                            NewThreadButton::new(
                                                "new-claude-thread-btn",
                                                "New Claude Code Thread",
                                                IconName::AiClaude,
                                            )
                                            // .keybinding(KeyBinding::for_action_in(
                                            //     &OpenHistory,
                                            //     &self.focus_handle(cx),
                                            //     window,
                                            //     cx,
                                            // ))
                                            .on_click(
                                                |window, cx| {
                                                    window.dispatch_action(
                                                        Box::new(NewExternalAgentThread {
                                                            agent: Some(
                                                                crate::ExternalAgent::ClaudeCode,
                                                            ),
                                                        }),
                                                        cx,
                                                    )
                                                },
                                            ),
                                        )
                                        .child(
                                            NewThreadButton::new(
                                                "new-native-agent-thread-btn",
                                                "New Native Agent Thread",
                                                IconName::ZedAssistant,
                                            )
                                            // .keybinding(KeyBinding::for_action_in(
                                            //     &OpenHistory,
                                            //     &self.focus_handle(cx),
                                            //     window,
                                            //     cx,
                                            // ))
                                            .on_click(
                                                |window, cx| {
                                                    window.dispatch_action(
                                                        Box::new(NewExternalAgentThread {
                                                            agent: Some(
                                                                crate::ExternalAgent::NativeAgent,
                                                            ),
                                                        }),
                                                        cx,
                                                    )
                                                },
                                            ),
                                        ),
                                )
                            }),
                    )
                    .when_some(configuration_error.as_ref(), |this, err| {
                        this.child(self.render_configuration_error(err, &focus_handle, window, cx))
                    })
            })
    }

    fn render_configuration_error(
        &self,
        configuration_error: &ConfigurationError,
        focus_handle: &FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        match configuration_error {
            ConfigurationError::ModelNotFound
            | ConfigurationError::ProviderNotAuthenticated(_)
            | ConfigurationError::NoProvider => Banner::new()
                .severity(ui::Severity::Warning)
                .child(Label::new(configuration_error.to_string()))
                .action_slot(
                    Button::new("settings", "Configure Provider")
                        .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                        .label_size(LabelSize::Small)
                        .key_binding(
                            KeyBinding::for_action_in(&OpenSettings, &focus_handle, window, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_event, window, cx| {
                            window.dispatch_action(OpenSettings.boxed_clone(), cx)
                        }),
                ),
            ConfigurationError::ProviderPendingTermsAcceptance(provider) => {
                Banner::new().severity(ui::Severity::Warning).child(
                    h_flex().w_full().children(
                        provider.render_accept_terms(
                            LanguageModelProviderTosView::ThreadEmptyState,
                            cx,
                        ),
                    ),
                )
            }
        }
    }

    fn render_tool_use_limit_reached(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let active_thread = match &self.active_view {
            ActiveView::Thread { thread, .. } => thread,
            ActiveView::ExternalAgentThread { .. } => {
                return None;
            }
            ActiveView::TextThread { .. } | ActiveView::History | ActiveView::Configuration => {
                return None;
            }
        };

        let thread = active_thread.read(cx).thread().read(cx);

        let tool_use_limit_reached = thread.tool_use_limit_reached();
        if !tool_use_limit_reached {
            return None;
        }

        let model = thread.configured_model()?.model;

        let focus_handle = self.focus_handle(cx);

        let banner = Banner::new()
            .severity(ui::Severity::Info)
            .child(Label::new("Consecutive tool use limit reached.").size(LabelSize::Small))
            .action_slot(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("continue-conversation", "Continue")
                            .layer(ElevationIndex::ModalSurface)
                            .label_size(LabelSize::Small)
                            .key_binding(
                                KeyBinding::for_action_in(
                                    &ContinueThread,
                                    &focus_handle,
                                    window,
                                    cx,
                                )
                                .map(|kb| kb.size(rems_from_px(10.))),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.continue_conversation(window, cx);
                            })),
                    )
                    .when(model.supports_burn_mode(), |this| {
                        this.child(
                            Button::new("continue-burn-mode", "Continue with Burn Mode")
                                .style(ButtonStyle::Filled)
                                .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                .layer(ElevationIndex::ModalSurface)
                                .label_size(LabelSize::Small)
                                .key_binding(
                                    KeyBinding::for_action_in(
                                        &ContinueWithBurnMode,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(10.))),
                                )
                                .tooltip(Tooltip::text("Enable Burn Mode for unlimited tool use."))
                                .on_click({
                                    let active_thread = active_thread.clone();
                                    cx.listener(move |this, _, window, cx| {
                                        active_thread.update(cx, |active_thread, cx| {
                                            active_thread.thread().update(cx, |thread, _cx| {
                                                thread.set_completion_mode(CompletionMode::Burn);
                                            });
                                        });
                                        this.continue_conversation(window, cx);
                                    })
                                }),
                        )
                    }),
            );

        Some(div().px_2().pb_2().child(banner).into_any_element())
    }

    fn create_copy_button(&self, message: impl Into<String>) -> impl IntoElement {
        let message = message.into();

        IconButton::new("copy", IconName::Copy)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .tooltip(Tooltip::text("Copy Error Message"))
            .on_click(move |_, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(message.clone()))
            })
    }

    fn dismiss_error_button(
        &self,
        thread: &Entity<ActiveThread>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        IconButton::new("dismiss", IconName::Close)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .tooltip(Tooltip::text("Dismiss Error"))
            .on_click(cx.listener({
                let thread = thread.clone();
                move |_, _, _, cx| {
                    thread.update(cx, |this, _cx| {
                        this.clear_last_error();
                    });

                    cx.notify();
                }
            }))
    }

    fn upgrade_button(
        &self,
        thread: &Entity<ActiveThread>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Button::new("upgrade", "Upgrade")
            .label_size(LabelSize::Small)
            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
            .on_click(cx.listener({
                let thread = thread.clone();
                move |_, _, _, cx| {
                    thread.update(cx, |this, _cx| {
                        this.clear_last_error();
                    });

                    cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx));
                    cx.notify();
                }
            }))
    }

    fn error_callout_bg(&self, cx: &Context<Self>) -> Hsla {
        cx.theme().status().error.opacity(0.08)
    }

    fn render_payment_required_error(
        &self,
        thread: &Entity<ActiveThread>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        const ERROR_MESSAGE: &str =
            "You reached your free usage limit. Upgrade to Zed Pro for more prompts.";

        let icon = Icon::new(IconName::XCircle)
            .size(IconSize::Small)
            .color(Color::Error);

        div()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                Callout::new()
                    .icon(icon)
                    .title("Free Usage Exceeded")
                    .description(ERROR_MESSAGE)
                    .tertiary_action(self.upgrade_button(thread, cx))
                    .secondary_action(self.create_copy_button(ERROR_MESSAGE))
                    .primary_action(self.dismiss_error_button(thread, cx))
                    .bg_color(self.error_callout_bg(cx)),
            )
            .into_any_element()
    }

    fn render_model_request_limit_reached_error(
        &self,
        plan: Plan,
        thread: &Entity<ActiveThread>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let error_message = match plan {
            Plan::ZedPro => "Upgrade to usage-based billing for more prompts.",
            Plan::ZedProTrial | Plan::ZedFree => "Upgrade to Zed Pro for more prompts.",
        };

        let icon = Icon::new(IconName::XCircle)
            .size(IconSize::Small)
            .color(Color::Error);

        div()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                Callout::new()
                    .icon(icon)
                    .title("Model Prompt Limit Reached")
                    .description(error_message)
                    .tertiary_action(self.upgrade_button(thread, cx))
                    .secondary_action(self.create_copy_button(error_message))
                    .primary_action(self.dismiss_error_button(thread, cx))
                    .bg_color(self.error_callout_bg(cx)),
            )
            .into_any_element()
    }

    fn render_error_message(
        &self,
        header: SharedString,
        message: SharedString,
        thread: &Entity<ActiveThread>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let message_with_header = format!("{}\n{}", header, message);

        let icon = Icon::new(IconName::XCircle)
            .size(IconSize::Small)
            .color(Color::Error);

        let retry_button = Button::new("retry", "Retry")
            .icon(IconName::RotateCw)
            .icon_position(IconPosition::Start)
            .icon_size(IconSize::Small)
            .label_size(LabelSize::Small)
            .on_click({
                let thread = thread.clone();
                move |_, window, cx| {
                    thread.update(cx, |thread, cx| {
                        thread.clear_last_error();
                        thread.thread().update(cx, |thread, cx| {
                            thread.retry_last_completion(Some(window.window_handle()), cx);
                        });
                    });
                }
            });

        div()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                Callout::new()
                    .icon(icon)
                    .title(header)
                    .description(message.clone())
                    .primary_action(retry_button)
                    .secondary_action(self.dismiss_error_button(thread, cx))
                    .tertiary_action(self.create_copy_button(message_with_header))
                    .bg_color(self.error_callout_bg(cx)),
            )
            .into_any_element()
    }

    fn render_retryable_error(
        &self,
        message: SharedString,
        can_enable_burn_mode: bool,
        thread: &Entity<ActiveThread>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let icon = Icon::new(IconName::XCircle)
            .size(IconSize::Small)
            .color(Color::Error);

        let retry_button = Button::new("retry", "Retry")
            .icon(IconName::RotateCw)
            .icon_position(IconPosition::Start)
            .icon_size(IconSize::Small)
            .label_size(LabelSize::Small)
            .on_click({
                let thread = thread.clone();
                move |_, window, cx| {
                    thread.update(cx, |thread, cx| {
                        thread.clear_last_error();
                        thread.thread().update(cx, |thread, cx| {
                            thread.retry_last_completion(Some(window.window_handle()), cx);
                        });
                    });
                }
            });

        let mut callout = Callout::new()
            .icon(icon)
            .title("Error")
            .description(message.clone())
            .bg_color(self.error_callout_bg(cx))
            .primary_action(retry_button);

        if can_enable_burn_mode {
            let burn_mode_button = Button::new("enable_burn_retry", "Enable Burn Mode and Retry")
                .icon(IconName::ZedBurnMode)
                .icon_position(IconPosition::Start)
                .icon_size(IconSize::Small)
                .label_size(LabelSize::Small)
                .on_click({
                    let thread = thread.clone();
                    move |_, window, cx| {
                        thread.update(cx, |thread, cx| {
                            thread.clear_last_error();
                            thread.thread().update(cx, |thread, cx| {
                                thread.enable_burn_mode_and_retry(Some(window.window_handle()), cx);
                            });
                        });
                    }
                });
            callout = callout.secondary_action(burn_mode_button);
        }

        div()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(callout)
            .into_any_element()
    }

    fn render_prompt_editor(
        &self,
        context_editor: &Entity<TextThreadEditor>,
        buffer_search_bar: &Entity<BufferSearchBar>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let mut registrar = buffer_search::DivRegistrar::new(
            |this, _, _cx| match &this.active_view {
                ActiveView::TextThread {
                    buffer_search_bar, ..
                } => Some(buffer_search_bar.clone()),
                _ => None,
            },
            cx,
        );
        BufferSearchBar::register(&mut registrar);
        registrar
            .into_div()
            .size_full()
            .relative()
            .map(|parent| {
                buffer_search_bar.update(cx, |buffer_search_bar, cx| {
                    if buffer_search_bar.is_dismissed() {
                        return parent;
                    }
                    parent.child(
                        div()
                            .p(DynamicSpacing::Base08.rems(cx))
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .bg(cx.theme().colors().editor_background)
                            .child(buffer_search_bar.render(window, cx)),
                    )
                })
            })
            .child(context_editor.clone())
            .child(self.render_drag_target(cx))
    }

    fn render_drag_target(&self, cx: &Context<Self>) -> Div {
        let is_local = self.project.read(cx).is_local();
        div()
            .invisible()
            .absolute()
            .top_0()
            .right_0()
            .bottom_0()
            .left_0()
            .bg(cx.theme().colors().drop_target_background)
            .drag_over::<DraggedTab>(|this, _, _, _| this.visible())
            .drag_over::<DraggedSelection>(|this, _, _, _| this.visible())
            .when(is_local, |this| {
                this.drag_over::<ExternalPaths>(|this, _, _, _| this.visible())
            })
            .on_drop(cx.listener(move |this, tab: &DraggedTab, window, cx| {
                let item = tab.pane.read(cx).item_for_index(tab.ix);
                let project_paths = item
                    .and_then(|item| item.project_path(cx))
                    .into_iter()
                    .collect::<Vec<_>>();
                this.handle_drop(project_paths, vec![], window, cx);
            }))
            .on_drop(
                cx.listener(move |this, selection: &DraggedSelection, window, cx| {
                    let project_paths = selection
                        .items()
                        .filter_map(|item| this.project.read(cx).path_for_entry(item.entry_id, cx))
                        .collect::<Vec<_>>();
                    this.handle_drop(project_paths, vec![], window, cx);
                }),
            )
            .on_drop(cx.listener(move |this, paths: &ExternalPaths, window, cx| {
                let tasks = paths
                    .paths()
                    .into_iter()
                    .map(|path| {
                        Workspace::project_path_for_path(this.project.clone(), &path, false, cx)
                    })
                    .collect::<Vec<_>>();
                cx.spawn_in(window, async move |this, cx| {
                    let mut paths = vec![];
                    let mut added_worktrees = vec![];
                    let opened_paths = futures::future::join_all(tasks).await;
                    for entry in opened_paths {
                        if let Some((worktree, project_path)) = entry.log_err() {
                            added_worktrees.push(worktree);
                            paths.push(project_path);
                        }
                    }
                    this.update_in(cx, |this, window, cx| {
                        this.handle_drop(paths, added_worktrees, window, cx);
                    })
                    .ok();
                })
                .detach();
            }))
    }

    fn handle_drop(
        &mut self,
        paths: Vec<ProjectPath>,
        added_worktrees: Vec<Entity<Worktree>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.active_view {
            ActiveView::Thread { thread, .. } => {
                let context_store = thread.read(cx).context_store().clone();
                context_store.update(cx, move |context_store, cx| {
                    let mut tasks = Vec::new();
                    for project_path in &paths {
                        tasks.push(context_store.add_file_from_path(
                            project_path.clone(),
                            false,
                            cx,
                        ));
                    }
                    cx.background_spawn(async move {
                        futures::future::join_all(tasks).await;
                        // Need to hold onto the worktrees until they have already been used when
                        // opening the buffers.
                        drop(added_worktrees);
                    })
                    .detach();
                });
            }
            ActiveView::ExternalAgentThread { thread_view } => {
                thread_view.update(cx, |thread_view, cx| {
                    thread_view.insert_dragged_files(paths, added_worktrees, window, cx);
                });
            }
            ActiveView::TextThread { context_editor, .. } => {
                context_editor.update(cx, |context_editor, cx| {
                    TextThreadEditor::insert_dragged_files(
                        context_editor,
                        paths,
                        added_worktrees,
                        window,
                        cx,
                    );
                });
            }
            ActiveView::History | ActiveView::Configuration => {}
        }
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AgentPanel");
        match &self.active_view {
            ActiveView::ExternalAgentThread { .. } => key_context.add("external_agent_thread"),
            ActiveView::TextThread { .. } => key_context.add("prompt_editor"),
            ActiveView::Thread { .. } | ActiveView::History | ActiveView::Configuration => {}
        }
        key_context
    }
}

impl Render for AgentPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // WARNING: Changes to this element hierarchy can have
        // non-obvious implications to the layout of children.
        //
        // If you need to change it, please confirm:
        // - The message editor expands (cmd-option-esc) correctly
        // - When expanded, the buttons at the bottom of the panel are displayed correctly
        // - Font size works as expected and can be changed with cmd-+/cmd-
        // - Scrolling in all views works as expected
        // - Files can be dropped into the panel
        let content = v_flex()
            .relative()
            .size_full()
            .justify_between()
            .key_context(self.key_context())
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|this, action: &NewThread, window, cx| {
                this.new_thread(action, window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenHistory, window, cx| {
                this.open_history(window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenSettings, window, cx| {
                this.open_configuration(window, cx);
            }))
            .on_action(cx.listener(Self::open_active_thread_as_markdown))
            .on_action(cx.listener(Self::deploy_rules_library))
            .on_action(cx.listener(Self::open_agent_diff))
            .on_action(cx.listener(Self::go_back))
            .on_action(cx.listener(Self::toggle_navigation_menu))
            .on_action(cx.listener(Self::toggle_options_menu))
            .on_action(cx.listener(Self::increase_font_size))
            .on_action(cx.listener(Self::decrease_font_size))
            .on_action(cx.listener(Self::reset_font_size))
            .on_action(cx.listener(Self::toggle_zoom))
            .on_action(cx.listener(|this, _: &ContinueThread, window, cx| {
                this.continue_conversation(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ContinueWithBurnMode, window, cx| {
                match &this.active_view {
                    ActiveView::Thread { thread, .. } => {
                        thread.update(cx, |active_thread, cx| {
                            active_thread.thread().update(cx, |thread, _cx| {
                                thread.set_completion_mode(CompletionMode::Burn);
                            });
                        });
                        this.continue_conversation(window, cx);
                    }
                    ActiveView::ExternalAgentThread { .. } => {}
                    ActiveView::TextThread { .. }
                    | ActiveView::History
                    | ActiveView::Configuration => {}
                }
            }))
            .on_action(cx.listener(Self::toggle_burn_mode))
            .child(self.render_toolbar(window, cx))
            .children(self.render_onboarding(window, cx))
            .map(|parent| match &self.active_view {
                ActiveView::Thread {
                    thread,
                    message_editor,
                    ..
                } => parent
                    .child(
                        if thread.read(cx).is_empty() && !self.should_render_onboarding(cx) {
                            self.render_thread_empty_state(window, cx)
                                .into_any_element()
                        } else {
                            thread.clone().into_any_element()
                        },
                    )
                    .children(self.render_tool_use_limit_reached(window, cx))
                    .when_some(thread.read(cx).last_error(), |this, last_error| {
                        this.child(
                            div()
                                .child(match last_error {
                                    ThreadError::PaymentRequired => {
                                        self.render_payment_required_error(thread, cx)
                                    }
                                    ThreadError::ModelRequestLimitReached { plan } => self
                                        .render_model_request_limit_reached_error(plan, thread, cx),
                                    ThreadError::Message { header, message } => {
                                        self.render_error_message(header, message, thread, cx)
                                    }
                                    ThreadError::RetryableError {
                                        message,
                                        can_enable_burn_mode,
                                    } => self.render_retryable_error(
                                        message,
                                        can_enable_burn_mode,
                                        thread,
                                        cx,
                                    ),
                                })
                                .into_any(),
                        )
                    })
                    .child(h_flex().relative().child(message_editor.clone()).when(
                        !LanguageModelRegistry::read_global(cx).has_authenticated_provider(cx),
                        |this| this.child(self.render_backdrop(cx)),
                    ))
                    .child(self.render_drag_target(cx)),
                ActiveView::ExternalAgentThread { thread_view, .. } => parent
                    .child(thread_view.clone())
                    .child(self.render_drag_target(cx)),
                ActiveView::History => parent.child(self.history.clone()),
                ActiveView::TextThread {
                    context_editor,
                    buffer_search_bar,
                    ..
                } => {
                    let model_registry = LanguageModelRegistry::read_global(cx);
                    let configuration_error =
                        model_registry.configuration_error(model_registry.default_model(), cx);
                    parent
                        .map(|this| {
                            if !self.should_render_onboarding(cx)
                                && let Some(err) = configuration_error.as_ref()
                            {
                                this.child(
                                    div().bg(cx.theme().colors().editor_background).p_2().child(
                                        self.render_configuration_error(
                                            err,
                                            &self.focus_handle(cx),
                                            window,
                                            cx,
                                        ),
                                    ),
                                )
                            } else {
                                this
                            }
                        })
                        .child(self.render_prompt_editor(
                            context_editor,
                            buffer_search_bar,
                            window,
                            cx,
                        ))
                }
                ActiveView::Configuration => parent.children(self.configuration.clone()),
            })
            .children(self.render_trial_end_upsell(window, cx));

        match self.active_view.which_font_size_used() {
            WhichFontSize::AgentFont => {
                WithRemSize::new(ThemeSettings::get_global(cx).agent_font_size(cx))
                    .size_full()
                    .child(content)
                    .into_any()
            }
            _ => content.into_any(),
        }
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

impl rules_library::InlineAssistDelegate for PromptLibraryInlineAssist {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut Context<RulesLibrary>,
    ) {
        InlineAssistant::update_global(cx, |assistant, cx| {
            let Some(project) = self
                .workspace
                .upgrade()
                .map(|workspace| workspace.read(cx).project().downgrade())
            else {
                return;
            };
            let prompt_store = None;
            let thread_store = None;
            let text_thread_store = None;
            let context_store = cx.new(|_| ContextStore::new(project.clone(), None));
            assistant.assist(
                &prompt_editor,
                self.workspace.clone(),
                context_store,
                project,
                prompt_store,
                thread_store,
                text_thread_store,
                initial_prompt,
                window,
                cx,
            )
        })
    }

    fn focus_agent_panel(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        workspace.focus_panel::<AgentPanel>(window, cx).is_some()
    }
}

pub struct ConcreteAssistantPanelDelegate;

impl AgentPanelDelegate for ConcreteAssistantPanelDelegate {
    fn active_context_editor(
        &self,
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<TextThreadEditor>> {
        let panel = workspace.panel::<AgentPanel>(cx)?;
        panel.read(cx).active_context_editor()
    }

    fn open_saved_context(
        &self,
        workspace: &mut Workspace,
        path: Arc<Path>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
            return Task::ready(Err(anyhow!("Agent panel not found")));
        };

        panel.update(cx, |panel, cx| {
            panel.open_saved_prompt_editor(path, window, cx)
        })
    }

    fn open_remote_context(
        &self,
        _workspace: &mut Workspace,
        _context_id: assistant_context::ContextId,
        _window: &mut Window,
        _cx: &mut Context<Workspace>,
    ) -> Task<Result<Entity<TextThreadEditor>>> {
        Task::ready(Err(anyhow!("opening remote context not implemented")))
    }

    fn quote_selection(
        &self,
        workspace: &mut Workspace,
        selection_ranges: Vec<Range<Anchor>>,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
            return;
        };

        if !panel.focus_handle(cx).contains_focused(window, cx) {
            workspace.toggle_panel_focus::<AgentPanel>(window, cx);
        }

        panel.update(cx, |_, cx| {
            // Wait to create a new context until the workspace is no longer
            // being updated.
            cx.defer_in(window, move |panel, window, cx| {
                if let Some(message_editor) = panel.active_message_editor() {
                    message_editor.update(cx, |message_editor, cx| {
                        message_editor.context_store().update(cx, |store, cx| {
                            let buffer = buffer.read(cx);
                            let selection_ranges = selection_ranges
                                .into_iter()
                                .flat_map(|range| {
                                    let (start_buffer, start) =
                                        buffer.text_anchor_for_position(range.start, cx)?;
                                    let (end_buffer, end) =
                                        buffer.text_anchor_for_position(range.end, cx)?;
                                    if start_buffer != end_buffer {
                                        return None;
                                    }
                                    Some((start_buffer, start..end))
                                })
                                .collect::<Vec<_>>();

                            for (buffer, range) in selection_ranges {
                                store.add_selection(buffer, range, cx);
                            }
                        })
                    })
                } else if let Some(context_editor) = panel.active_context_editor() {
                    let snapshot = buffer.read(cx).snapshot(cx);
                    let selection_ranges = selection_ranges
                        .into_iter()
                        .map(|range| range.to_point(&snapshot))
                        .collect::<Vec<_>>();

                    context_editor.update(cx, |context_editor, cx| {
                        context_editor.quote_ranges(selection_ranges, snapshot, window, cx)
                    });
                }
            });
        });
    }
}

struct OnboardingUpsell;

impl Dismissable for OnboardingUpsell {
    const KEY: &'static str = "dismissed-trial-upsell";
}

struct TrialEndUpsell;

impl Dismissable for TrialEndUpsell {
    const KEY: &'static str = "dismissed-trial-end-upsell";
}

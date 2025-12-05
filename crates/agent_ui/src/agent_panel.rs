use std::{ops::Range, path::Path, rc::Rc, sync::Arc, time::Duration};

use acp_thread::AcpThread;
use agent_client_protocol as acp;
use agent::{ContextServerRegistry, DbThreadMetadata, HistoryEntry, HistoryStore};
use db::kvp::{Dismissable, KEY_VALUE_STORE};
use project::{
    ExternalAgentServerName,
    agent_server_store::{CLAUDE_CODE_NAME, CODEX_NAME, GEMINI_NAME},
};
use serde::{Deserialize, Serialize};
use settings::{
    DefaultAgentView as DefaultView, LanguageModelProviderSetting, LanguageModelSelection,
};

use zed_actions::agent::{OpenClaudeCodeOnboardingModal, ReauthenticateAgent};

use crate::ManageProfiles;
use crate::ui::{AcpOnboardingModal, ClaudeCodeOnboardingModal};
use crate::{
    ActivateNextThread, ActivatePreviousThread, AddContextServer, AgentDiffPane, 
    CloseActiveThread, Follow, InlineAssistant, NewTextThread, NewThread,
    OpenActiveThreadAsMarkdown, OpenAgentDiff, OpenHistory, ResetTrialEndUpsell, ResetTrialUpsell,
    ToggleNavigationMenu, ToggleNewThreadMenu, ToggleOptionsMenu,
    acp::AcpThreadView,
    agent_configuration::{AgentConfiguration, AssistantConfigurationEvent},
    slash_command::SlashCommandCompletionProvider,
    text_thread_editor::{AgentPanelDelegate, TextThreadEditor, make_lsp_adapter_delegate},
    ui::{AgentOnboardingModal, EndTrialUpsell},
};
use crate::{
    ExpandMessageEditor,
    acp::{AcpThreadHistory, ThreadHistoryEvent},
};
use crate::{ExternalAgent, NewExternalAgentThread, NewNativeAgentThreadFromSummary};
use agent_settings::AgentSettings;
use ai_onboarding::AgentPanelOnboarding;
use anyhow::{Result, anyhow};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_text_thread::{TextThread, TextThreadEvent, TextThreadSummary};
use client::{UserStore, zed_urls};
use cloud_llm_client::{Plan, PlanV1, PlanV2, UsageLimit};
use editor::{Anchor, AnchorRangeExt as _, Editor, EditorEvent, MultiBuffer};
use extension::ExtensionEvents;
use extension_host::ExtensionStore;
use fs::Fs;
use gpui::{
    div, px, Action, Animation, AnimationExt, AnyElement, App, AsyncWindowContext,
    Context, Corner, DismissEvent, Entity, EntityId, EventEmitter, ExternalPaths,
    FocusHandle, Focusable,  KeyContext, InteractiveElement, IntoElement, KeyContext,
    ParentElement, Pixels, Render, ScrollHandle, SharedString, Styled, Subscription, Task,
    UpdateGlobal, WeakEntity, Window, prelude::*, pulsating_between,
};
use language::LanguageRegistry;
use language_model::{ConfigurationError, LanguageModelRegistry};
use project::{Project, ProjectPath, Worktree};
use prompt_store::{PromptBuilder, PromptStore, UserPromptId};
use rules_library::{RulesLibrary, open_rules_library};
use search::{BufferSearchBar, buffer_search};
use settings::{Settings, update_settings_file};
use theme::ThemeSettings;
use ui::{
    Callout, ContextMenu, ContextMenuEntry, KeyBinding, PopoverMenu, PopoverMenuHandle,
    ProgressBar, Tab, Tooltip, prelude::*, utils::WithRemSize,
};
use util::ResultExt as _;
use workspace::{
    CollaboratorId, DraggedSelection, DraggedTab, ToggleZoom, ToolbarItemView, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};
use zed_actions::{
    DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize,
    agent::{
        OpenAcpOnboardingModal, OpenOnboardingModal, OpenSettings, ResetAgentZoom, ResetOnboarding,
    },
    assistant::{OpenRulesLibrary, ToggleFocus},
};

const AGENT_PANEL_KEY: &str = "agent_panel";

#[derive(Serialize, Deserialize, Debug, Clone)]
enum SerializedThread {
    Acp { session_id: String, agent_type: AgentType },
    Text { path: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentPanel {
    width: Option<Pixels>,
    selected_agent: Option<AgentType>,
    #[serde(default)]
    threads: Vec<SerializedThread>,
    #[serde(default)]
    active_thread_index: usize,
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
                .register_action(
                    |workspace, action: &NewNativeAgentThreadFromSummary, window, cx| {
                        if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                            panel.update(cx, |panel, cx| {
                                panel.new_native_agent_thread_from_summary(action, window, cx)
                            });
                            workspace.focus_panel::<AgentPanel>(window, cx);
                        }
                    },
                )
                .register_action(|workspace, _: &ExpandMessageEditor, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.expand_message_editor(window, cx));
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
                        panel.update(cx, |panel, cx| panel.new_text_thread(window, cx));
                    }
                })
                .register_action(|workspace, action: &NewExternalAgentThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.external_thread(action.agent.clone(), None, None, window, cx)
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
                .register_action(|workspace, _: &Follow, window, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .register_action(|workspace, _: &OpenAgentDiff, window, cx| {
                    let thread = workspace
                        .panel::<AgentPanel>(cx)
                        .and_then(|panel| panel.read(cx).active_thread_view().cloned())
                        .and_then(|thread_view| thread_view.read(cx).thread().cloned());

                    if let Some(thread) = thread {
                        AgentDiffPane::deploy_in_workspace(thread, workspace, window, cx);
                    }
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
                .register_action(|workspace, _: &ToggleNewThreadMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_new_thread_menu(&ToggleNewThreadMenu, window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &OpenOnboardingModal, window, cx| {
                    AgentOnboardingModal::toggle(workspace, window, cx)
                })
                .register_action(|workspace, _: &OpenAcpOnboardingModal, window, cx| {
                    AcpOnboardingModal::toggle(workspace, window, cx)
                })
                .register_action(|workspace, _: &OpenClaudeCodeOnboardingModal, window, cx| {
                    ClaudeCodeOnboardingModal::toggle(workspace, window, cx)
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
                })
                .register_action(|workspace, _: &ResetAgentZoom, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.reset_agent_zoom(window, cx);
                        });
                    }
                });
        },
    )
    .detach();
}


enum ThreadView {
    Acp(Entity<AcpThreadView>),
    Text {
        text_thread_editor: Entity<TextThreadEditor>,
        title_editor: Entity<Editor>,
        buffer_search_bar: Entity<BufferSearchBar>,
        _subscriptions: Vec<gpui::Subscription>,
    },
}

impl ThreadView {
    #[allow(dead_code)]
    fn which_font_size_used(&self) -> WhichFontSize {
        match self {
            ThreadView::Acp(_) => WhichFontSize::AgentFont,
            ThreadView::Text { .. } => WhichFontSize::BufferFont,
        }
    }
}

impl ThreadView {
    fn entity_id(&self) -> EntityId {
        match self {
            Self::Acp(view) => view.entity_id(),
            Self::Text { text_thread_editor, .. } => text_thread_editor.entity_id(),
        }
    }

    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self {
            Self::Acp(view) => view.read(cx).focus_handle(cx),
            Self::Text { text_thread_editor, .. } => text_thread_editor.read(cx).focus_handle(cx),
        }
    }
}

#[allow(dead_code)]
struct ThreadTab {
    tab_id: EntityId,
    view: ThreadView,
    agent_type: AgentType,
    title: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl ThreadTab {
    fn new(
        view: ThreadView,
        agent_type: AgentType,
        _window: &mut Window,
        cx: &mut Context<AgentPanel>,
    ) -> Self {
        let tab_id = view.entity_id();
        let mut subscriptions = Vec::new();
        
        match &view {
            ThreadView::Acp(thread_view) => {
                subscriptions.push(cx.observe(thread_view, |_, _, cx| cx.notify()));
                // subscriptions.push(cx.subscribe_in(thread_view, window, AgentPanel::handle_thread_view_event));
            }
            ThreadView::Text { text_thread_editor, .. } => {
                subscriptions.push(cx.observe(text_thread_editor, |_, _, cx| cx.notify()));
                // Add text thread specific subscriptions if needed
            }
        }

        Self {
            tab_id,
            view,
            agent_type,
            title: None,
            _subscriptions: subscriptions,
        }
    }

    #[allow(dead_code)]
    fn update_title(&mut self, cx: &App) {
        match &self.view {
            ThreadView::Acp(thread_view) => {
                if let Some(thread) = thread_view.read(cx).thread() {
                    self.title = thread.read(cx).title().map(|s| s.into());
                }
            }
            ThreadView::Text { text_thread_editor, .. } => {
                self.title = Some(text_thread_editor.read(cx).title(cx).to_string().into());
            }
        }
    }
}

enum SpecialView {
    History,
    Configuration,
}

#[allow(dead_code)]
enum WhichFontSize {
    AgentFont,
    BufferFont,
    None,
    Default,
}

// TODO unify this with ExternalAgent
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentType {
    #[default]
    NativeAgent,
    TextThread,
    Gemini,
    ClaudeCode,
    Codex,
    Custom {
        name: SharedString,
    },
}

impl AgentType {
    fn label(&self) -> SharedString {
        match self {
            Self::NativeAgent | Self::TextThread => "Zed Agent".into(),
            Self::Gemini => "Gemini CLI".into(),
            Self::ClaudeCode => "Claude Code".into(),
            Self::Codex => "Codex".into(),
            Self::Custom { name, .. } => name.into(),
        }
    }

    fn icon(&self) -> Option<IconName> {
        match self {
            Self::NativeAgent | Self::TextThread => None,
            Self::Gemini => Some(IconName::AiGemini),
            Self::ClaudeCode => Some(IconName::AiClaude),
            Self::Codex => Some(IconName::AiOpenAi),
            Self::Custom { .. } => Some(IconName::Terminal),
        }
    }

    fn into_external_agent(self) -> Option<ExternalAgent> {
        match self {
            Self::NativeAgent => Some(ExternalAgent::NativeAgent),
            Self::Gemini => Some(ExternalAgent::Gemini),
            Self::ClaudeCode => Some(ExternalAgent::ClaudeCode),
            Self::Codex => Some(ExternalAgent::Codex),
            Self::Custom { name } => Some(ExternalAgent::Custom { name }),
            Self::TextThread => None,
        }
    }
}

impl From<ExternalAgent> for AgentType {
    fn from(value: ExternalAgent) -> Self {
        match value {
            ExternalAgent::Gemini => Self::Gemini,
            ExternalAgent::ClaudeCode => Self::ClaudeCode,
            ExternalAgent::Codex => Self::Codex,
            ExternalAgent::Custom { name } => Self::Custom { name },
            ExternalAgent::NativeAgent => Self::NativeAgent,
        }
    }
}


pub struct AgentPanel {
    workspace: WeakEntity<Workspace>,
    loading: bool,
    user_store: Entity<UserStore>,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    acp_history: Entity<AcpThreadHistory>,
    history_store: Entity<agent::HistoryStore>,
    text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
    prompt_store: Option<Entity<PromptStore>>,
    context_server_registry: Entity<ContextServerRegistry>,
    configuration: Option<Entity<AgentConfiguration>>,
    configuration_subscription: Option<Subscription>,
    
    // Collection of all open threads
    threads: Vec<ThreadTab>,
    active_thread_index: usize,

    // Special views (History, Configuration) stored separately
    special_view: Option<SpecialView>,
    previous_thread_index: Option<usize>,  // For returning from special views
    
    // Tab UI state
    tab_bar_scroll_handle: ScrollHandle,

    new_thread_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_panel_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_navigation_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_navigation_menu: Option<Entity<ContextMenu>>,
    _extension_subscription: Option<Subscription>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    zoomed: bool,
    pending_serialization: Option<Task<Result<()>>>,
    onboarding: Entity<AgentPanelOnboarding>,
    selected_agent: AgentType,
}

impl AgentPanel {
    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let selected_agent = self.selected_agent.clone();
        let active_thread_index = self.active_thread_index;
        
        let threads: Vec<SerializedThread> = self.threads.iter().filter_map(|tab| {
            match &tab.view {
                ThreadView::Acp(thread_view) => {
                    thread_view.read(cx).thread().map(|thread| {
                        SerializedThread::Acp {
                            session_id: thread.read(cx).session_id().to_string(),
                            agent_type: tab.agent_type.clone(),
                        }
                    })
                }
                ThreadView::Text { text_thread_editor, .. } => {
                    text_thread_editor.read(cx).text_thread().read(cx).path().map(|path| {
                        SerializedThread::Text {
                            path: path.to_string_lossy().to_string(),
                        }
                    })
                }
            }
        }).collect();
        
        self.pending_serialization = Some(cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(
                    AGENT_PANEL_KEY.into(),
                    serde_json::to_string(&SerializedAgentPanel {
                        width,
                        selected_agent: Some(selected_agent),
                        threads,
                        active_thread_index,
                    })?,
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
            let serialized_panel = if let Some(panel) = cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(AGENT_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                serde_json::from_str::<SerializedAgentPanel>(&panel).log_err()
            } else {
                None
            };

            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            let text_thread_store = workspace
                .update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    assistant_text_thread::TextThreadStore::new(
                        project,
                        prompt_builder,
                        slash_commands,
                        cx,
                    )
                })?
                .await?;

            let panel = workspace.update_in(cx, |workspace, window, cx| {
                let panel =
                    cx.new(|cx| Self::new(workspace, text_thread_store, prompt_store, window, cx));

                panel.as_mut(cx).loading = true;
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width.map(|w| w.round());
                        if let Some(selected_agent) = serialized_panel.selected_agent.clone() {
                            panel.selected_agent = selected_agent;
                        }
                        
                        // Restore threads from serialized data
                        if !serialized_panel.threads.is_empty() {
                            for serialized_thread in &serialized_panel.threads {
                                match serialized_thread {
                                    SerializedThread::Acp { session_id, agent_type } => {
                                        panel.load_agent_thread_by_session_id(
                                            session_id.clone(),
                                            agent_type.clone(),
                                            window,
                                            cx,
                                        );
                                    }
                                    SerializedThread::Text { path } => {
                                        let path = std::path::PathBuf::from(path);
                                        panel.open_text_thread_by_path(path, window, cx);
                                    }
                                }
                            }
                            // Restore active thread index, clamping to valid range
                            panel.active_thread_index = serialized_panel.active_thread_index
                                .min(panel.threads.len().saturating_sub(1));
                        } else {
                            // Fall back to creating a new thread with selected agent
                            let agent_type = serialized_panel.selected_agent
                                .unwrap_or(AgentType::NativeAgent);
                            panel.new_agent_thread(agent_type, window, cx);
                        }
                        cx.notify();
                    });
                } else {
                    panel.update(cx, |panel, cx| {
                        panel.new_agent_thread(AgentType::NativeAgent, window, cx);
                    });
                }
                panel.as_mut(cx).loading = false;
                panel
            })?;

            Ok(panel)
        })
    }

    fn new(
        workspace: &Workspace,
        text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let fs = workspace.app_state().fs.clone();
        let user_store = workspace.app_state().user_store.clone();
        let project = workspace.project();
        let language_registry = project.read(cx).languages().clone();
        let client = workspace.client().clone();
        let workspace = workspace.weak_handle();

        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));

        let history_store = cx.new(|cx| agent::HistoryStore::new(text_thread_store.clone(), cx));
        let acp_history = cx.new(|cx| AcpThreadHistory::new(history_store.clone(), window, cx));
        cx.subscribe_in(
            &acp_history,
            window,
            |this, _, event, window, cx| match event {
                ThreadHistoryEvent::Open(HistoryEntry::AcpThread(thread)) => {
                    this.external_thread(
                        Some(crate::ExternalAgent::NativeAgent),
                        Some(thread.clone()),
                        None,
                        window,
                        cx,
                    );
                }
                ThreadHistoryEvent::Open(HistoryEntry::TextThread(thread)) => {
                    this.open_saved_text_thread(thread.path.clone(), window, cx)
                        .detach_and_log_err(cx);
                }
                ThreadHistoryEvent::ThreadRemoved(id) => {
                    let mut index_to_close = None;
                    for (i, thread) in this.threads.iter().enumerate() {
                        match &thread.view {
                            ThreadView::Acp(view) => {
                                if let Some(thread) = view.read(cx).thread() {
                                    if thread.read(cx).session_id().to_string() == *id {
                                        index_to_close = Some(i);
                                        break;
                                    }
                                }
                            }
                            ThreadView::Text { text_thread_editor, .. } => {
                                let thread_id = text_thread_editor.read(cx).text_thread().read(cx).id().clone();
                                if thread_id == assistant_text_thread::TextThreadId::from_proto(id.clone()) {
                                    index_to_close = Some(i);
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(index) = index_to_close {
                        this.close_thread(index, window, cx);
                    }
                }
            },
        )
        .detach();

        let weak_panel = cx.entity().downgrade();

        window.defer(cx, move |window, cx| {
            let panel = weak_panel.clone();
            let agent_navigation_menu =
                ContextMenu::build_persistent(window, cx, move |mut menu, _window, cx| {
                    if let Some(panel) = panel.upgrade() {
                        menu = Self::populate_recently_opened_menu_section(menu, panel, cx);
                    }

                    menu = menu
                        .action("View All", Box::new(OpenHistory))
                        .fixed_width(px(320.).into())
                        .keep_open_on_confirm(false)
                        .key_context("NavigationMenu");

                    menu
                });
            weak_panel
                .update(cx, |panel, cx| {
                    cx.subscribe_in(
                        &agent_navigation_menu,
                        window,
                        |_, menu, _: &DismissEvent, window, cx| {
                            menu.update(cx, |menu, _| {
                                menu.clear_selected();
                            });
                            cx.focus_self(window);
                        },
                    )
                    .detach();
                    panel.agent_navigation_menu = Some(agent_navigation_menu);
                })
                .ok();
        });

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

        // Subscribe to extension events to sync agent servers when extensions change
        let extension_subscription = if let Some(extension_events) = ExtensionEvents::try_global(cx)
        {
            Some(
                cx.subscribe(&extension_events, |this, _source, event, cx| match event {
                    extension::Event::ExtensionInstalled(_)
                    | extension::Event::ExtensionUninstalled(_)
                    | extension::Event::ExtensionsInstalledChanged => {
                        this.sync_agent_servers_from_extensions(cx);
                    }
                    _ => {}
                }),
            )
        } else {
            None
        };

        let mut panel = Self {
            workspace,
            user_store,
            project: project.clone(),
            fs: fs.clone(),
            language_registry,
            text_thread_store,
            prompt_store,
            configuration: None,
            configuration_subscription: None,
            context_server_registry,
            new_thread_menu_handle: PopoverMenuHandle::default(),
            agent_panel_menu_handle: PopoverMenuHandle::default(),
            agent_navigation_menu_handle: PopoverMenuHandle::default(),
            agent_navigation_menu: None,
            _extension_subscription: extension_subscription,
            width: None,
            height: None,
            zoomed: false,
            pending_serialization: None,
            onboarding,
            acp_history,
            history_store,
            selected_agent: AgentType::default(),
            loading: false,
            threads: Vec::new(),
            active_thread_index: 0,
            special_view: None,
            previous_thread_index: None,
            tab_bar_scroll_handle: ScrollHandle::new(),
        };

        // Initial sync of agent servers from extensions
        panel.sync_agent_servers_from_extensions(cx);

        // Create initial thread based on default view
        let panel_type = AgentSettings::get_global(cx).default_view;
        match panel_type {
            DefaultView::Thread => {
                panel.add_new_thread(AgentType::NativeAgent, window, cx);
            }
            DefaultView::TextThread => {
                panel.add_new_thread(AgentType::TextThread, window, cx);
            }
        }

        panel
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

    pub(crate) fn prompt_store(&self) -> &Option<Entity<PromptStore>> {
        &self.prompt_store
    }

    pub(crate) fn thread_store(&self) -> &Entity<HistoryStore> {
        &self.history_store
    }

    pub(crate) fn context_server_registry(&self) -> &Entity<ContextServerRegistry> {
        &self.context_server_registry
    }

    pub fn is_hidden(workspace: &Entity<Workspace>, cx: &App) -> bool {
        let workspace_read = workspace.read(cx);

        workspace_read
            .panel::<AgentPanel>(cx)
            .map(|panel| {
                let panel_id = Entity::entity_id(&panel);

                let is_visible = workspace_read.all_docks().iter().any(|dock| {
                    dock.read(cx)
                        .visible_panel()
                        .is_some_and(|visible_panel| visible_panel.panel_id() == panel_id)
                });

                !is_visible
            })
            .unwrap_or(true)
    }

    fn active_thread_view(&self) -> Option<&Entity<AcpThreadView>> {
        self.threads.get(self.active_thread_index).and_then(|tab| {
            match &tab.view {
                ThreadView::Acp(view) => Some(view),
                _ => None,
            }
        })
    }

    fn add_new_thread(
        &mut self,
        agent_type: AgentType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let view = self.create_thread_view(agent_type.clone(), None, None, window, cx);
        let tab = ThreadTab::new(view, agent_type, window, cx);

        self.threads.push(tab);
        self.active_thread_index = self.threads.len() - 1;
        self.special_view = None;

        // Scroll to show new tab
        self.tab_bar_scroll_handle.scroll_to_item(self.active_thread_index);

        cx.notify();
    }

    fn create_thread_view(
        &mut self,
        agent_type: AgentType,
        resume_thread: Option<DbThreadMetadata>,
        summarize_thread: Option<DbThreadMetadata>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ThreadView {
        match agent_type {
            AgentType::TextThread => {
                 let context = self.text_thread_store.update(cx, |store, cx| store.create(cx));
                 let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx).log_err().flatten();
                 let text_thread_editor = cx.new(|cx| {
                    let mut editor = TextThreadEditor::for_text_thread(
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
                
                // We need to create the title editor and search bar too, similar to ActiveView::text_thread
                // This logic is duplicated from ActiveView::text_thread, we should probably refactor it later
                // For now, I'll inline it to make it work.
                
                let title = text_thread_editor.read(cx).title(cx).to_string();
                let editor = cx.new(|cx| {
                    let mut editor = Editor::single_line(window, cx);
                    editor.set_text(title, window, cx);
                    editor
                });
                
                let mut suppress_first_edit = true;
                let subscriptions = vec![
                    window.subscribe(&editor, cx, {
                        {
                            let text_thread_editor = text_thread_editor.clone();
                            move |editor, event, window, cx| match event {
                                EditorEvent::BufferEdited => {
                                    if suppress_first_edit {
                                        suppress_first_edit = false;
                                        return;
                                    }
                                    let new_summary = editor.read(cx).text(cx);

                                    text_thread_editor.update(cx, |text_thread_editor, cx| {
                                        text_thread_editor
                                            .text_thread()
                                            .update(cx, |text_thread, cx| {
                                                text_thread.set_custom_summary(new_summary, cx);
                                            })
                                    })
                                }
                                EditorEvent::Blurred => {
                                    if editor.read(cx).text(cx).is_empty() {
                                        let summary = text_thread_editor
                                            .read(cx)
                                            .text_thread()
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
                    window.subscribe(&text_thread_editor.read(cx).text_thread().clone(), cx, {
                        let editor = editor.clone();
                        let acp_history_store = self.history_store.clone();
                        move |text_thread, event, window, cx| match event {
                            TextThreadEvent::SummaryGenerated => {
                                let summary = text_thread.read(cx).summary().or_default();

                                editor.update(cx, |editor, cx| {
                                    editor.set_text(summary, window, cx);
                                })
                            }
                            TextThreadEvent::PathChanged { old_path, new_path } => {
                                acp_history_store.update(cx, |history_store, cx| {
                                    if let Some(old_path) = old_path {
                                        history_store
                                            .replace_recently_opened_text_thread(old_path, new_path, cx);
                                    } else {
                                        history_store.push_recently_opened_entry(
                                            agent::HistoryEntryId::TextThread(new_path.clone()),
                                            cx,
                                        );
                                    }
                                });
                            }
                            _ => {}
                        }
                    }),
                ];

                let buffer_search_bar = cx.new(|cx| BufferSearchBar::new(Some(self.language_registry.clone()), window, cx));
                buffer_search_bar.update(cx, |buffer_search_bar, cx| {
                    buffer_search_bar.set_active_pane_item(Some(&text_thread_editor), window, cx)
                });

                ThreadView::Text {
                    text_thread_editor,
                    title_editor: editor,
                    buffer_search_bar,
                    _subscriptions: subscriptions,
                }
            }
            _ => {
                 let view = cx.new(|cx| {
                    crate::acp::AcpThreadView::new(
                        agent_type.clone().into_external_agent().unwrap_or(ExternalAgent::NativeAgent).server(self.fs.clone(), self.history_store.clone()),
                        resume_thread,
                        summarize_thread,
                        self.workspace.clone(),
                        self.project.clone(),
                        self.history_store.clone(),
                        self.prompt_store.clone(),
                        window,
                        cx,
                    )
                });
                ThreadView::Acp(view)
            }
        }
    }
    
    #[allow(dead_code)]
    fn activate_thread(
        &mut self,
        index: usize,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if index >= self.threads.len() {
            return;
        }

        if index != self.active_thread_index {
            self.active_thread_index = index;
            self.tab_bar_scroll_handle.scroll_to_item(index);
            cx.notify();
        }

        if focus {
            self.threads[index].view.focus_handle(cx).focus(window);
        }
    }

    fn close_thread(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if index >= self.threads.len() {
            return;
        }

        // Remove the thread
        self.threads.remove(index);

        // Adjust active index
        if self.threads.is_empty() {
            // Create a new thread to avoid empty state
            self.add_new_thread(AgentType::NativeAgent, window, cx);
        } else if index <= self.active_thread_index && self.active_thread_index > 0 {
            self.active_thread_index -= 1;
        }

        cx.notify();
    }
    
    fn activate_next_thread(&mut self, _: &ActivateNextThread, window: &mut Window, cx: &mut Context<Self>) {
        if self.threads.is_empty() {
            return;
        }
        let next_index = (self.active_thread_index + 1) % self.threads.len();
        self.activate_thread(next_index, true, window, cx);
    }
    
    fn activate_previous_thread(&mut self, _: &ActivatePreviousThread, window: &mut Window, cx: &mut Context<Self>) {
        if self.threads.is_empty() {
            return;
        }
        let prev_index = if self.active_thread_index == 0 {
            self.threads.len() - 1
        } else {
            self.active_thread_index - 1
        };
        self.activate_thread(prev_index, true, window, cx);
    }
    
    fn close_active_thread(&mut self, _: &CloseActiveThread, window: &mut Window, cx: &mut Context<Self>) {
        if self.threads.len() <= 1 {
            return;
        }
        self.close_thread(self.active_thread_index, window, cx);
    }
    

    fn new_thread(&mut self, _action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        self.new_agent_thread(AgentType::NativeAgent, window, cx);
    }

    fn new_native_agent_thread_from_summary(
        &mut self,
        action: &NewNativeAgentThreadFromSummary,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self
            .history_store
            .read(cx)
            .thread_from_session_id(&action.from_session_id)
        else {
            return;
        };

        self.external_thread(
            Some(ExternalAgent::NativeAgent),
            None,
            Some(thread.clone()),
            window,
            cx,
        );
    }

    fn new_text_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        telemetry::event!("Agent Thread Started", agent = "zed-text");

        let context = self
            .text_thread_store
            .update(cx, |context_store, cx| context_store.create(cx));
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        let text_thread_editor = cx.new(|cx| {
            let mut editor = TextThreadEditor::for_text_thread(
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

        if self.selected_agent != AgentType::TextThread {
            self.selected_agent = AgentType::TextThread;
            self.serialize(cx);
        }

        let title = text_thread_editor.read(cx).title(cx).to_string();
        let title_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(title, window, cx);
            editor
        });

        let subscriptions = vec![
            cx.subscribe(&text_thread_editor, |_, _, _event: &EditorEvent, cx| {
                // TODO: Handle TextThreadEditorEvent if needed
                cx.notify();
            }),
        ];

        let buffer_search_bar = cx.new(|cx| {
            let mut bar = BufferSearchBar::new(
                Some(self.language_registry.clone()),
                window,
                cx,
            );
            bar.set_active_pane_item(Some(&text_thread_editor), window, cx);
            bar
        });

        let thread_view = ThreadView::Text {
            text_thread_editor: text_thread_editor.clone(),
            title_editor,
            buffer_search_bar,
            _subscriptions: subscriptions,
        };

        let tab = ThreadTab::new(thread_view, AgentType::TextThread, window, cx);
        self.threads.push(tab);
        self.active_thread_index = self.threads.len() - 1;
        self.special_view = None;
        self.tab_bar_scroll_handle.scroll_to_item(self.active_thread_index);
        cx.notify();
        text_thread_editor.focus_handle(cx).focus(window);
    }

    fn external_thread(
        &mut self,
        agent_choice: Option<crate::ExternalAgent>,
        resume_thread: Option<DbThreadMetadata>,
        summarize_thread: Option<DbThreadMetadata>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let _workspace = self.workspace.clone();
        let _project = self.project.clone();
        let fs = self.fs.clone();
        let is_via_collab = self.project.read(cx).is_via_collab();

        const LAST_USED_EXTERNAL_AGENT_KEY: &str = "agent_panel__last_used_external_agent";

        #[derive(Serialize, Deserialize)]
        struct LastUsedExternalAgent {
            agent: crate::ExternalAgent,
        }

        let loading = self.loading;
        let history = self.history_store.clone();

        cx.spawn_in(window, async move |this, cx| {
            let ext_agent = match agent_choice {
                Some(agent) => {
                    cx.background_spawn({
                        let agent = agent.clone();
                        async move {
                            if let Some(serialized) =
                                serde_json::to_string(&LastUsedExternalAgent { agent }).log_err()
                            {
                                KEY_VALUE_STORE
                                    .write_kvp(LAST_USED_EXTERNAL_AGENT_KEY.to_string(), serialized)
                                    .await
                                    .log_err();
                            }
                        }
                    })
                    .detach();

                    agent
                }
                None => {
                    if is_via_collab {
                        ExternalAgent::NativeAgent
                    } else {
                        cx.background_spawn(async move {
                            KEY_VALUE_STORE.read_kvp(LAST_USED_EXTERNAL_AGENT_KEY)
                        })
                        .await
                        .log_err()
                        .flatten()
                        .and_then(|value| {
                            serde_json::from_str::<LastUsedExternalAgent>(&value).log_err()
                        })
                        .map(|agent| agent.agent)
                        .unwrap_or(ExternalAgent::NativeAgent)
                    }
                }
            };

            let server = ext_agent.server(fs, history);

            if !loading {
                telemetry::event!("Agent Thread Started", agent = server.telemetry_id());
            }

            this.update_in(cx, |this, window, cx| {
                let selected_agent = ext_agent.into();
                if this.selected_agent != selected_agent {
                    this.selected_agent = selected_agent.clone();
                    this.serialize(cx);
                }

                let thread_view = this.create_thread_view(
                    selected_agent.clone(),
                    resume_thread,
                    summarize_thread,
                    window,
                    cx
                );
                
                let tab = ThreadTab::new(thread_view, selected_agent, window, cx);
                this.threads.push(tab);
                this.active_thread_index = this.threads.len() - 1;
                this.special_view = None;
                this.tab_bar_scroll_handle.scroll_to_item(this.active_thread_index);
                cx.notify();
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

    fn expand_message_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(thread_view) = self.active_thread_view() {
            thread_view.update(cx, |view, cx| {
                view.expand_message_editor(&ExpandMessageEditor, window, cx);
                view.focus_handle(cx).focus(window);
            });
        }
    }

    fn open_history(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.special_view, Some(SpecialView::History)) {
            self.hide_special_view(cx);
        } else {
            self.show_special_view(SpecialView::History, cx);
        }
        cx.focus_self(window);
    }
    pub(crate) fn open_saved_text_thread(
        &mut self,
        path: Arc<Path>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let text_thread_task = self
            .history_store
            .update(cx, |store, cx| store.load_text_thread(path, cx));
        cx.spawn_in(window, async move |this, cx| {
            let text_thread = text_thread_task.await?;
            this.update_in(cx, |this, window, cx| {
                this.open_text_thread(text_thread, window, cx);
            })
        })
    }

    pub(crate) fn open_text_thread(
        &mut self,
        text_thread: Entity<TextThread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project.clone(), cx)
            .log_err()
            .flatten();
        let editor = cx.new(|cx| {
            TextThreadEditor::for_text_thread(
                text_thread,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                window,
                cx,
            )
        });

        if self.selected_agent != AgentType::TextThread {
            self.selected_agent = AgentType::TextThread;
            self.serialize(cx);
        }

        // For TextThread, we already have the editor, so we can't use create_thread_view directly
        // or we need to modify create_thread_view to accept an optional editor.
        // Given the current structure, it's easier to construct the ThreadView::Text manually here
        // since we already have the editor constructed.
        
        let title = editor.read(cx).title(cx).to_string();
        let title_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(title, window, cx);
            editor
        });
        
        // Subscriptions logic...
        let subscriptions = vec![
            cx.subscribe(&editor, |_this, _, _event: &EditorEvent, cx| {
                // TODO: Handle TextThreadEditorEvent if needed, or define AgentPanelEvent
                // For now, just notify
                cx.notify();
            }),
        ];

        let buffer_search_bar = cx.new(|cx| {
            let mut bar = BufferSearchBar::new(
                Some(self.language_registry.clone()),
                window,
                cx,
            );
            bar.set_active_pane_item(Some(&editor), window, cx);
            bar
        });
        
        let thread_view = ThreadView::Text {
            text_thread_editor: editor,
            title_editor,
            buffer_search_bar,
            _subscriptions: subscriptions,
        };

        let tab = ThreadTab::new(thread_view, AgentType::TextThread, window, cx);
        self.threads.push(tab);
        self.active_thread_index = self.threads.len() - 1;
        self.special_view = None;
        self.tab_bar_scroll_handle.scroll_to_item(self.active_thread_index);
        cx.notify();
    }

    pub fn go_back(&mut self, _: &workspace::GoBack, window: &mut Window, cx: &mut Context<Self>) {
        if self.special_view.is_some() {
            self.hide_special_view(cx);
            if let Some(tab) = self.threads.get(self.active_thread_index) {
                tab.view.focus_handle(cx).focus(window);
            }
        }
    }

    pub fn toggle_navigation_menu(
        &mut self,
        _: &ToggleNavigationMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.agent_navigation_menu_handle.toggle(window, cx);
    }

    pub fn toggle_options_menu(
        &mut self,
        _: &ToggleOptionsMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.agent_panel_menu_handle.toggle(window, cx);
    }

    pub fn toggle_new_thread_menu(
        &mut self,
        _: &ToggleNewThreadMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.new_thread_menu_handle.toggle(window, cx);
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
        let font_size = if let Some(tab) = self.threads.get(self.active_thread_index) {
             match &tab.view {
                 ThreadView::Acp(_) => WhichFontSize::AgentFont,
                 ThreadView::Text { .. } => WhichFontSize::BufferFont,
             }
        } else {
            WhichFontSize::Default
        };

        match font_size {
            WhichFontSize::AgentFont => {
                if persist {
                    update_settings_file(self.fs.clone(), cx, move |settings, cx| {
                        let agent_ui_font_size =
                            ThemeSettings::get_global(cx).agent_ui_font_size(cx) + delta;
                        let agent_buffer_font_size =
                            ThemeSettings::get_global(cx).agent_buffer_font_size(cx) + delta;

                        let _ = settings
                            .theme
                            .agent_ui_font_size
                            .insert(theme::clamp_font_size(agent_ui_font_size).into());
                        let _ = settings
                            .theme
                            .agent_buffer_font_size
                            .insert(theme::clamp_font_size(agent_buffer_font_size).into());
                    });
                } else {
                    theme::adjust_agent_ui_font_size(cx, |size| size + delta);
                    theme::adjust_agent_buffer_font_size(cx, |size| size + delta);
                }
            }
            WhichFontSize::BufferFont => {
                // Prompt editor uses the buffer font size, so allow the action to propagate to the
                // default handler that changes that font size.
                cx.propagate();
            }
            WhichFontSize::None | WhichFontSize::Default => {}
        }
    }

    pub fn reset_font_size(
        &mut self,
        action: &ResetBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if action.persist {
            update_settings_file(self.fs.clone(), cx, move |settings, _| {
                settings.theme.agent_ui_font_size = None;
                settings.theme.agent_buffer_font_size = None;
            });
        } else {
            theme::reset_agent_ui_font_size(cx);
            theme::reset_agent_buffer_font_size(cx);
        }
    }

    pub fn reset_agent_zoom(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        theme::reset_agent_ui_font_size(cx);
        theme::reset_agent_buffer_font_size(cx);
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

    pub(crate) fn open_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let context_server_store = self.project.read(cx).context_server_store();
        let fs = self.fs.clone();

        self.show_special_view(SpecialView::Configuration, cx);
        self.configuration = Some(cx.new(|cx| {
            AgentConfiguration::new(
                fs,
                agent_server_store,
                context_server_store,
                self.context_server_registry.clone(),
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

        if let Some(thread_view) = self.active_thread_view() {
            thread_view
                .update(cx, |thread_view, cx| {
                    thread_view.open_thread_as_markdown(workspace, window, cx)
                })
                .detach_and_log_err(cx);
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
                    .is_none_or(|model| model.provider.id() != provider.id())
                    && let Some(model) = provider.default_model(cx)
                {
                    update_settings_file(self.fs.clone(), cx, move |settings, _| {
                        let provider = model.provider_id().0.to_string();
                        let model = model.id().0.to_string();
                        settings
                            .agent
                            .get_or_insert_default()
                            .set_model(LanguageModelSelection {
                                provider: LanguageModelProviderSetting(provider),
                                model,
                            })
                    });
                }

                self.new_thread(&NewThread, window, cx);
                // AcpThread uses the global default model, so we don't need to set it explicitly on the thread instance.
            }
        }
    }

    pub fn active_agent_thread(&self, cx: &App) -> Option<Entity<AcpThread>> {
        self.active_thread_view()
            .and_then(|view| view.read(cx).thread().cloned())
    }


    pub fn active_text_thread_editor(&self) -> Option<Entity<TextThreadEditor>> {
        self.threads.get(self.active_thread_index).and_then(|tab| {
            match &tab.view {
                ThreadView::Text { text_thread_editor, .. } => Some(text_thread_editor.clone()),
                _ => None,
            }
        })
    }

    pub fn active_title_editor(&self) -> Option<Entity<Editor>> {
        self.threads.get(self.active_thread_index).and_then(|tab| {
            match &tab.view {
                ThreadView::Text { title_editor, .. } => Some(title_editor.clone()),
                _ => None,
            }
        })
    }

    fn show_special_view(&mut self, view: SpecialView, cx: &mut Context<Self>) {
        if self.special_view.is_none() {
            self.previous_thread_index = Some(self.active_thread_index);
        }
        self.special_view = Some(view);
        cx.notify();
    }

    fn hide_special_view(&mut self, cx: &mut Context<Self>) {
        if self.special_view.is_some() {
            self.special_view = None;
            if let Some(index) = self.previous_thread_index {
                if index < self.threads.len() {
                    self.active_thread_index = index;
                    self.tab_bar_scroll_handle.scroll_to_item(index);
                }
            }
            cx.notify();
        }
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

            menu = menu.entry_with_end_slot_on_hover(
                title,
                None,
                {
                    let panel = panel.downgrade();
                    let entry = entry.clone();
                    move |window, cx| {
                        let entry = entry.clone();
                        panel
                            .update(cx, move |this, cx| match &entry {
                                agent::HistoryEntry::AcpThread(entry) => this.external_thread(
                                    Some(ExternalAgent::NativeAgent),
                                    Some(entry.clone()),
                                    None,
                                    window,
                                    cx,
                                ),
                                agent::HistoryEntry::TextThread(entry) => this
                                    .open_saved_text_thread(entry.path.clone(), window, cx)
                                    .detach_and_log_err(cx),
                            })
                            .ok();
                    }
                },
                IconName::Close,
                "Close Entry".into(),
                {
                    let panel = panel.downgrade();
                    let id = entry.id();
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

    pub fn selected_agent(&self) -> AgentType {
        self.selected_agent.clone()
    }

    fn sync_agent_servers_from_extensions(&mut self, cx: &mut Context<Self>) {
        if let Some(extension_store) = ExtensionStore::try_global(cx) {
            let (manifests, extensions_dir) = {
                let store = extension_store.read(cx);
                let installed = store.installed_extensions();
                let manifests: Vec<_> = installed
                    .iter()
                    .map(|(id, entry)| (id.clone(), entry.manifest.clone()))
                    .collect();
                let extensions_dir = paths::extensions_dir().join("installed");
                (manifests, extensions_dir)
            };

            self.project.update(cx, |project, cx| {
                project.agent_server_store().update(cx, |store, cx| {
                    let manifest_refs: Vec<_> = manifests
                        .iter()
                        .map(|(id, manifest)| (id.as_ref(), manifest.as_ref()))
                        .collect();
                    store.sync_extension_agents(manifest_refs, extensions_dir, cx);
                });
            });
        }
    }

    pub fn new_agent_thread(
        &mut self,
        agent: AgentType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match agent {
            AgentType::TextThread => {
                window.dispatch_action(NewTextThread.boxed_clone(), cx);
            }
            AgentType::NativeAgent => self.external_thread(
                Some(crate::ExternalAgent::NativeAgent),
                None,
                None,
                window,
                cx,
            ),
            AgentType::Gemini => {
                self.external_thread(Some(crate::ExternalAgent::Gemini), None, None, window, cx)
            }
            AgentType::ClaudeCode => {
                self.selected_agent = AgentType::ClaudeCode;
                self.serialize(cx);
                self.external_thread(
                    Some(crate::ExternalAgent::ClaudeCode),
                    None,
                    None,
                    window,
                    cx,
                )
            }
            AgentType::Codex => {
                self.selected_agent = AgentType::Codex;
                self.serialize(cx);
                self.external_thread(Some(crate::ExternalAgent::Codex), None, None, window, cx)
            }
            AgentType::Custom { name } => self.external_thread(
                Some(crate::ExternalAgent::Custom { name }),
                None,
                None,
                window,
                cx,
            ),
        }
    }

    pub fn load_agent_thread(
        &mut self,
        thread: DbThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.external_thread(
            Some(ExternalAgent::NativeAgent),
            Some(thread),
            None,
            window,
            cx,
        );
    }
    
    fn load_agent_thread_by_session_id(
        &mut self,
        session_id: String,
        agent_type: AgentType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let history_store = self.history_store.clone();
        let session_id_for_lookup = acp::SessionId(session_id.clone().into());
        
        cx.spawn_in(window, async move |this, cx| {
            // Find the thread metadata matching the session ID
            let thread_metadata = this.update(cx, |_this, cx| {
                let history = history_store.read(cx);
                history.thread_from_session_id(&session_id_for_lookup).cloned()
            })?;
            
            if let Some(metadata) = thread_metadata {
                // TextThread should not come through this path
                if agent_type == AgentType::TextThread {
                    log::warn!("Unexpected TextThread in load_agent_thread_by_session_id");
                    return anyhow::Ok(());
                }
                
                this.update_in(cx, |this, window, cx| {
                    // Map agent_type to ExternalAgent
                    let external_agent = match &agent_type {
                        AgentType::NativeAgent => ExternalAgent::NativeAgent,
                        AgentType::Gemini => ExternalAgent::Gemini,
                        AgentType::ClaudeCode => ExternalAgent::ClaudeCode,
                        AgentType::Codex => ExternalAgent::Codex,
                        AgentType::Custom { name } => ExternalAgent::Custom { name: name.clone() },
                        AgentType::TextThread => unreachable!(),
                    };
                    this.external_thread(
                        Some(external_agent),
                        Some(metadata),
                        None,
                        window,
                        cx,
                    );
                })?;
            } else {
                log::warn!("Could not find thread with session_id: {}", session_id);
            }
            anyhow::Ok(())
        }).detach_and_log_err(cx);
    }
    
    fn open_text_thread_by_path(
        &mut self,
        path: std::path::PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let history_store = self.history_store.clone();
        let path_arc: Arc<std::path::Path> = path.into();
        
        cx.spawn_in(window, async move |this, cx| {
            let text_thread = history_store.update_in(cx, |store, _window, cx| {
                store.load_text_thread(path_arc.clone(), cx)
            })?.await?;
            
            this.update_in(cx, |this, window, cx| {
                this.open_text_thread(text_thread, window, cx);
            })?;
            anyhow::Ok(())
        }).detach_and_log_err(cx);
    }
}

impl Focusable for AgentPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if let Some(special) = &self.special_view {
            match special {
                SpecialView::History => self.acp_history.focus_handle(cx),
                SpecialView::Configuration => {
                    if let Some(configuration) = self.configuration.as_ref() {
                        configuration.focus_handle(cx)
                    } else {
                        cx.focus_handle()
                    }
                }
            }
        } else if let Some(tab) = self.threads.get(self.active_thread_index) {
            tab.view.focus_handle(cx)
        } else {
            // Fallback to panel's own focus handle if no threads (e.g. empty state)
            // But AgentPanel doesn't have a focus_handle field.
            // Wait, Panel trait implies it?
            // No, Focusable trait requires it.
            // If AgentPanel doesn't have one, how did it work before?
            // It delegated to active_view.
            // If active_view was None (impossible before), it would panic or something.
            // But now threads can be empty.
            // I should probably add a focus_handle to AgentPanel struct if I want to support empty state focus.
            // Or just return cx.focus_handle() which is a dummy?
            // Actually, `cx.focus_handle()` creates a NEW handle.
            // I should probably use `self.new_thread_menu_handle` or something if I have to.
            // Or just create a focus handle in `AgentPanel` struct.
            // The plan didn't mention adding focus_handle to AgentPanel.
            // But `empty state` UI will need focus.
            // Let's assume for now threads are not empty (I ensure that in close_thread).
            // But `add_new_thread` might not be called yet during initialization?
            // `new` calls `add_new_thread`.
            // So threads should not be empty.
            // But let's be safe.
            cx.focus_handle()
        }
    }
}

fn agent_panel_dock_position(cx: &App) -> DockPosition {
    AgentSettings::get_global(cx).dock.into()
}

impl EventEmitter<PanelEvent> for AgentPanel {}

impl Panel for AgentPanel {
    fn persistent_name() -> &'static str {
        "AgentPanel"
    }

    fn panel_key() -> &'static str {
        AGENT_PANEL_KEY
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        agent_panel_dock_position(cx)
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position != DockPosition::Bottom
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings
                .agent
                .get_or_insert_default()
                .set_dock(position.into());
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
        AgentSettings::get_global(cx).enabled(cx)
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
        
        if let Some(special) = &self.special_view {
            return match special {
                SpecialView::History => Label::new("History").truncate().into_any_element(),
                SpecialView::Configuration => Label::new("Settings").truncate().into_any_element(),
            };
        }

        let content = if let Some(tab) = self.threads.get(self.active_thread_index) {
            match &tab.view {
                ThreadView::Acp(thread_view) => {
                    if let Some(title_editor) = thread_view.read(cx).title_editor() {
                        div()
                            .w_full()
                            .on_action({
                                let thread_view = thread_view.downgrade();
                                move |_: &menu::Confirm, window, cx| {
                                    if let Some(thread_view) = thread_view.upgrade() {
                                        thread_view.focus_handle(cx).focus(window);
                                    }
                                }
                            })
                            .on_action({
                                let thread_view = thread_view.downgrade();
                                move |_: &editor::actions::Cancel, window, cx| {
                                    if let Some(thread_view) = thread_view.upgrade() {
                                        thread_view.focus_handle(cx).focus(window);
                                    }
                                }
                            })
                            .child(title_editor)
                            .into_any_element()
                    } else {
                        Label::new(thread_view.read(cx).title(cx))
                            .color(Color::Muted)
                            .truncate()
                            .into_any_element()
                    }
                }
                ThreadView::Text {
                    title_editor,
                    text_thread_editor,
                    ..
                } => {
                    let summary = text_thread_editor.read(cx).text_thread().read(cx).summary();

                    match summary {
                        TextThreadSummary::Pending => Label::new(TextThreadSummary::DEFAULT)
                            .color(Color::Muted)
                            .truncate()
                            .into_any_element(),
                        TextThreadSummary::Content(summary) => {
                            if summary.done {
                                div()
                                    .w_full()
                                    .child(title_editor.clone())
                                    .into_any_element()
                            } else {
                                Label::new(LOADING_SUMMARY_PLACEHOLDER)
                                    .truncate()
                                    .color(Color::Muted)
                                    .into_any_element()
                            }
                        }
                        TextThreadSummary::Error => h_flex()
                            .w_full()
                            .child(title_editor.clone())
                            .child(
                                IconButton::new("retry-summary-generation", IconName::RotateCcw)
                                    .icon_size(IconSize::Small)
                                    .on_click({
                                        let text_thread_editor = text_thread_editor.clone();
                                        move |_, _window, cx| {
                                            text_thread_editor.update(cx, |text_thread_editor, cx| {
                                                text_thread_editor.regenerate_summary(cx);
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
            }
        } else {
             div().into_any_element()
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

    fn render_panel_options_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let user_store = self.user_store.read(cx);
        let usage = user_store.model_request_usage();
        let account_url = zed_urls::account_url(cx);

        let focus_handle = self.focus_handle(cx);

        let full_screen_label = if self.is_zoomed(window, cx) {
            "Disable Full Screen"
        } else {
            "Enable Full Screen"
        };

        let selected_agent = self.selected_agent.clone();

        PopoverMenu::new("agent-options-menu")
            .trigger_with_tooltip(
                IconButton::new("agent-options-menu", IconName::Ellipsis)
                    .icon_size(IconSize::Small),
                {
                    let focus_handle = focus_handle.clone();
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "Toggle Agent Menu",
                            &ToggleOptionsMenu,
                            &focus_handle,
                            cx,
                        )
                    }
                },
            )
            .anchor(Corner::TopRight)
            .with_handle(self.agent_panel_menu_handle.clone())
            .menu({
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
                            .separator()
                            .action("Rules", Box::new(OpenRulesLibrary::default()))
                            .action("Profiles", Box::new(ManageProfiles::default()))
                            .action("Settings", Box::new(OpenSettings))
                            .separator()
                            .action(full_screen_label, Box::new(ToggleZoom));

                        if selected_agent == AgentType::Gemini {
                            menu = menu.action("Reauthenticate", Box::new(ReauthenticateAgent))
                        }

                        menu
                    }))
                }
            })
    }

    fn render_recent_entries_menu(
        &self,
        icon: IconName,
        corner: Corner,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        PopoverMenu::new("agent-nav-menu")
            .trigger_with_tooltip(
                IconButton::new("agent-nav-menu", icon).icon_size(IconSize::Small),
                {
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "Toggle Recent Threads",
                            &ToggleNavigationMenu,
                            &focus_handle,
                            cx,
                        )
                    }
                },
            )
            .anchor(corner)
            .with_handle(self.agent_navigation_menu_handle.clone())
            .menu({
                let menu = self.agent_navigation_menu.clone();
                move |window, cx| {
                    telemetry::event!("View Thread History Clicked");

                    if let Some(menu) = menu.as_ref() {
                        menu.update(cx, |_, cx| {
                            cx.defer_in(window, |menu, window, cx| {
                                menu.rebuild(window, cx);
                            });
                        })
                    }
                    menu.clone()
                }
            })
    }

    fn render_toolbar_back_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        IconButton::new("go-back", IconName::ArrowLeft)
            .icon_size(IconSize::Small)
            .on_click(cx.listener(|this, _, window, cx| {
                this.go_back(&workspace::GoBack, window, cx);
            }))
            .tooltip({
                move |_window, cx| {
                    Tooltip::for_action_in("Go Back", &workspace::GoBack, &focus_handle, cx)
                }
            })
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let focus_handle = self.focus_handle(cx);

        // Get custom icon path for selected agent before building menu (to avoid borrow issues)
        let selected_agent_custom_icon =
            if let AgentType::Custom { name, .. } = &self.selected_agent {
                agent_server_store
                    .read(cx)
                    .agent_icon(&ExternalAgentServerName(name.clone()))
            } else {
                None
            };

        let active_thread = self.active_agent_thread(cx);

        let new_thread_menu = PopoverMenu::new("new_thread_menu")
            .trigger_with_tooltip(
                IconButton::new("new_thread_menu_btn", IconName::Plus).icon_size(IconSize::Small),
                {
                    let focus_handle = focus_handle.clone();
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "New Thread…",
                            &ToggleNewThreadMenu,
                            &focus_handle,
                            cx,
                        )
                    }
                },
            )
            .anchor(Corner::TopRight)
            .with_handle(self.new_thread_menu_handle.clone())
            .menu({
                let selected_agent = self.selected_agent.clone();
                let is_agent_selected = move |agent_type: AgentType| selected_agent == agent_type;

                let workspace = self.workspace.clone();
                let is_via_collab = workspace
                    .update(cx, |workspace, cx| {
                        workspace.project().read(cx).is_via_collab()
                    })
                    .unwrap_or_default();

                move |window, cx| {
                    telemetry::event!("New Thread Clicked");

                    let active_thread = active_thread.clone();
                    Some(ContextMenu::build(window, cx, |menu, _window, cx| {
                        menu.context(focus_handle.clone())
                            .when_some(active_thread, |this, active_thread| {
                                let thread = active_thread.read(cx);

                                if !thread.is_empty() {
                                    let session_id = thread.session_id().clone();
                                    this.item(
                                        ContextMenuEntry::new("New From Summary")
                                            .icon(IconName::ThreadFromSummary)
                                            .icon_color(Color::Muted)
                                            .handler(move |window, cx| {
                                                window.dispatch_action(
                                                    Box::new(NewNativeAgentThreadFromSummary {
                                                        from_session_id: session_id.clone(),
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
                                ContextMenuEntry::new("Zed Agent")
                                    .when(is_agent_selected(AgentType::NativeAgent) | is_agent_selected(AgentType::TextThread) , |this| {
                                        this.action(Box::new(NewExternalAgentThread { agent: None }))
                                    })
                                    .icon(IconName::ZedAgent)
                                    .icon_color(Color::Muted)
                                    .handler({
                                        let workspace = workspace.clone();
                                        move |window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                workspace.update(cx, |workspace, cx| {
                                                    if let Some(panel) =
                                                        workspace.panel::<AgentPanel>(cx)
                                                    {
                                                        panel.update(cx, |panel, cx| {
                                                            panel.new_agent_thread(
                                                                AgentType::NativeAgent,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                });
                                            }
                                        }
                                    }),
                            )
                            .item(
                                ContextMenuEntry::new("Text Thread")
                                    .action(NewTextThread.boxed_clone())
                                    .icon(IconName::TextThread)
                                    .icon_color(Color::Muted)
                                    .handler({
                                        let workspace = workspace.clone();
                                        move |window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                workspace.update(cx, |workspace, cx| {
                                                    if let Some(panel) =
                                                        workspace.panel::<AgentPanel>(cx)
                                                    {
                                                        panel.update(cx, |panel, cx| {
                                                            panel.new_agent_thread(
                                                                AgentType::TextThread,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                });
                                            }
                                        }
                                    }),
                            )
                            .separator()
                            .header("External Agents")
                            .item(
                                ContextMenuEntry::new("Claude Code")
                                    .when(is_agent_selected(AgentType::ClaudeCode), |this| {
                                        this.action(Box::new(NewExternalAgentThread { agent: None }))
                                    })
                                    .icon(IconName::AiClaude)
                                    .disabled(is_via_collab)
                                    .icon_color(Color::Muted)
                                    .handler({
                                        let workspace = workspace.clone();
                                        move |window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                workspace.update(cx, |workspace, cx| {
                                                    if let Some(panel) =
                                                        workspace.panel::<AgentPanel>(cx)
                                                    {
                                                        panel.update(cx, |panel, cx| {
                                                            panel.new_agent_thread(
                                                                AgentType::ClaudeCode,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                });
                                            }
                                        }
                                    }),
                            )
                            .item(
                                ContextMenuEntry::new("Codex CLI")
                                    .when(is_agent_selected(AgentType::Codex), |this| {
                                        this.action(Box::new(NewExternalAgentThread { agent: None }))
                                    })
                                    .icon(IconName::AiOpenAi)
                                    .disabled(is_via_collab)
                                    .icon_color(Color::Muted)
                                    .handler({
                                        let workspace = workspace.clone();
                                        move |window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                workspace.update(cx, |workspace, cx| {
                                                    if let Some(panel) =
                                                        workspace.panel::<AgentPanel>(cx)
                                                    {
                                                        panel.update(cx, |panel, cx| {
                                                            panel.new_agent_thread(
                                                                AgentType::Codex,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                });
                                            }
                                        }
                                    }),
                            )
                            .item(
                                ContextMenuEntry::new("Gemini CLI")
                                    .when(is_agent_selected(AgentType::Gemini), |this| {
                                        this.action(Box::new(NewExternalAgentThread { agent: None }))
                                    })
                                    .icon(IconName::AiGemini)
                                    .icon_color(Color::Muted)
                                    .disabled(is_via_collab)
                                    .handler({
                                        let workspace = workspace.clone();
                                        move |window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                workspace.update(cx, |workspace, cx| {
                                                    if let Some(panel) =
                                                        workspace.panel::<AgentPanel>(cx)
                                                    {
                                                        panel.update(cx, |panel, cx| {
                                                            panel.new_agent_thread(
                                                                AgentType::Gemini,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                });
                                            }
                                        }
                                    }),
                            )
                            .map(|mut menu| {
                                let agent_server_store = agent_server_store.read(cx);
                                let agent_names = agent_server_store
                                    .external_agents()
                                    .filter(|name| {
                                        name.0 != GEMINI_NAME
                                            && name.0 != CLAUDE_CODE_NAME
                                            && name.0 != CODEX_NAME
                                    })
                                    .cloned()
                                    .collect::<Vec<_>>();

                                for agent_name in agent_names {
                                    let icon_path = agent_server_store.agent_icon(&agent_name);

                                    let mut entry = ContextMenuEntry::new(agent_name.clone());

                                    if let Some(icon_path) = icon_path {
                                        entry = entry.custom_icon_svg(icon_path);
                                    } else {
                                        entry = entry.icon(IconName::Terminal);
                                    }
                                    entry = entry
                                        .when(
                                            is_agent_selected(AgentType::Custom {
                                                name: agent_name.0.clone(),
                                            }),
                                            |this| {
                                                this.action(Box::new(NewExternalAgentThread { agent: None }))
                                            },
                                        )
                                        .icon_color(Color::Muted)
                                        .disabled(is_via_collab)
                                        .handler({
                                            let workspace = workspace.clone();
                                            let agent_name = agent_name.clone();
                                            move |window, cx| {
                                                if let Some(workspace) = workspace.upgrade() {
                                                    workspace.update(cx, |workspace, cx| {
                                                        if let Some(panel) =
                                                            workspace.panel::<AgentPanel>(cx)
                                                        {
                                                            panel.update(cx, |panel, cx| {
                                                                panel.new_agent_thread(
                                                                    AgentType::Custom {
                                                                        name: agent_name
                                                                            .clone()
                                                                            .into(),
                                                                    },
                                                                    window,
                                                                    cx,
                                                                );
                                                            });
                                                        }
                                                    });
                                                }
                                            }
                                        });

                                    menu = menu.item(entry);
                                }

                                menu
                            })
                            .separator()
                            .item(
                                ContextMenuEntry::new("Add More Agents")
                                    .icon(IconName::Plus)
                                    .icon_color(Color::Muted)
                                    .handler({
                                        move |window, cx| {
                                            window.dispatch_action(Box::new(zed_actions::Extensions {
                                                category_filter: Some(
                                                    zed_actions::ExtensionCategoryFilter::AgentServers,
                                                ),
                                                id: None,
                                            }), cx)
                                        }
                                    }),
                            )
                    }))
                }
            });

        let selected_agent_label = self.selected_agent.label();

        let is_thread_loading = self
            .active_thread_view()
            .map(|thread| thread.read(cx).is_loading())
            .unwrap_or(false);

        let has_custom_icon = selected_agent_custom_icon.is_some();

        let selected_agent = div()
            .id("selected_agent_icon")
            .when_some(selected_agent_custom_icon, |this, icon_path| {
                this.px_1()
                    .child(Icon::from_external_svg(icon_path).color(Color::Muted))
            })
            .when(!has_custom_icon, |this| {
                this.when_some(self.selected_agent.icon(), |this, icon| {
                    this.px_1().child(Icon::new(icon).color(Color::Muted))
                })
            })
            .tooltip(move |_, cx| {
                Tooltip::with_meta(selected_agent_label.clone(), None, "Selected Agent", cx)
            });

        let selected_agent = if is_thread_loading {
            selected_agent
                .with_animation(
                    "pulsating-icon",
                    Animation::new(Duration::from_secs(1))
                        .repeat()
                        .with_easing(pulsating_between(0.2, 0.6)),
                    |icon, delta| icon.opacity(delta),
                )
                .into_any_element()
        } else {
            selected_agent.into_any_element()
        };

        h_flex()
            .id("agent-panel-toolbar")
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
                    .gap(DynamicSpacing::Base04.rems(cx))
                    .pl(DynamicSpacing::Base04.rems(cx))
                    .child(if self.special_view.is_some() {
                        self.render_toolbar_back_button(cx).into_any_element()
                    } else {
                        selected_agent.into_any_element()
                    })
                    .child(self.render_title_view(window, cx)),
            )
            .child(
                h_flex()
                    .flex_none()
                    .gap(DynamicSpacing::Base02.rems(cx))
                    .pl(DynamicSpacing::Base04.rems(cx))
                    .pr(DynamicSpacing::Base06.rems(cx))
                    .child(new_thread_menu)
                    .child(self.render_recent_entries_menu(
                        IconName::MenuAltTemp,
                        Corner::TopRight,
                        cx,
                    ))
                    .child(self.render_panel_options_menu(window, cx)),
            )
    }

    fn should_render_trial_end_upsell(&self, cx: &mut Context<Self>) -> bool {
        if TrialEndUpsell::dismissed() {
            return false;
        }

        if self.special_view.is_some() {
            return false;
        }

        if let Some(tab) = self.threads.get(self.active_thread_index) {
            match &tab.view {
                ThreadView::Text { .. } => {
                     if LanguageModelRegistry::global(cx)
                        .read(cx)
                        .default_model()
                        .is_some_and(|model| {
                            model.provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
                        })
                    {
                        return false;
                    }
                }
                _ => return false,
            }
        } else {
            return false;
        }

        let plan = self.user_store.read(cx).plan();
        let has_previous_trial = self.user_store.read(cx).trial_started_at().is_some();

        matches!(
            plan,
            Some(Plan::V1(PlanV1::ZedFree) | Plan::V2(PlanV2::ZedFree))
        ) && has_previous_trial
    }

    fn should_render_onboarding(&self, cx: &mut Context<Self>) -> bool {
        if OnboardingUpsell::dismissed() {
            return false;
        }

        let user_store = self.user_store.read(cx);

        if user_store
            .plan()
            .is_some_and(|plan| matches!(plan, Plan::V1(PlanV1::ZedPro) | Plan::V2(PlanV2::ZedPro)))
            && user_store
                .subscription_period()
                .and_then(|period| period.0.checked_add_days(chrono::Days::new(1)))
                .is_some_and(|date| date < chrono::Utc::now())
        {
            OnboardingUpsell::set_dismissed(true, cx);
            return false;
        }

        if self.special_view.is_some() {
            return false;
        }

        if let Some(tab) = self.threads.get(self.active_thread_index) {
            if let ThreadView::Acp(view) = &tab.view {
                if view.read(cx).as_native_thread(cx).is_none() {
                    return false;
                }
            }
        } else {
            return false;
        }

        let history_is_empty = self.history_store.read(cx).is_empty(cx);

        let has_configured_non_zed_providers = LanguageModelRegistry::read_global(cx)
            .providers()
            .iter()
            .any(|provider| {
                provider.is_authenticated(cx)
                    && provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
            });

        history_is_empty || !has_configured_non_zed_providers
    }

    fn render_onboarding(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_onboarding(cx) {
            return None;
        }

        let text_thread_view = self.active_text_thread_editor().is_some();

        Some(
            div()
                .when(text_thread_view, |this| {
                    this.bg(cx.theme().colors().editor_background)
                })
                .child(self.onboarding.clone()),
        )
    }

    fn render_trial_end_upsell(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_trial_end_upsell(cx) {
            return None;
        }

        let plan = self.user_store.read(cx).plan()?;

        Some(
            v_flex()
                .absolute()
                .inset_0()
                .size_full()
                .bg(cx.theme().colors().panel_background)
                .opacity(0.85)
                .block_mouse_except_scroll()
                .child(EndTrialUpsell::new(
                    plan,
                    Arc::new({
                        let this = cx.entity();
                        move |_, cx| {
                            this.update(cx, |_this, cx| {
                                TrialEndUpsell::set_dismissed(true, cx);
                                cx.notify();
                            });
                        }
                    }),
                )),
        )
    }

    #[allow(dead_code)]
    fn render_configuration_error(
        &self,
        border_bottom: bool,
        configuration_error: &ConfigurationError,
        _focus_handle: &FocusHandle,
        cx: &mut App,
    ) -> impl IntoElement {
        let zed_provider_configured = AgentSettings::get_global(cx)
            .default_model
            .as_ref()
            .is_some_and(|selection| selection.provider.0.as_str() == "zed.dev");

        let callout = if zed_provider_configured {
            Callout::new()
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .when(border_bottom, |this| {
                    this.border_position(ui::BorderPosition::Bottom)
                })
                .title("Sign in to continue using Zed as your LLM provider.")
                .actions_slot(
                    Button::new("sign_in", "Sign In")
                        .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                        .label_size(LabelSize::Small)
                        .on_click({
                            let workspace = self.workspace.clone();
                            move |_, _, cx| {
                                let Ok(client) =
                                    workspace.update(cx, |workspace, _| workspace.client().clone())
                                else {
                                    return;
                                };

                                cx.spawn(async move |cx| {
                                    client.sign_in_with_optional_connect(true, cx).await
                                })
                                .detach_and_log_err(cx);
                            }
                        }),
                )
        } else {
            Callout::new()
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .when(border_bottom, |this| {
                    this.border_position(ui::BorderPosition::Bottom)
                })
                .title(configuration_error.to_string())
                .actions_slot(
                    Button::new("settings", "Configure")
                        .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                        .label_size(LabelSize::Small)
                        .key_binding(
                            ui::KeyBinding::for_action(&OpenSettings, cx),
                        )
                        .on_click(|_event, window, cx| {
                            window.dispatch_action(OpenSettings.boxed_clone(), cx)
                        }),
                )
        };

        match configuration_error {
            ConfigurationError::ModelNotFound
            | ConfigurationError::ProviderNotAuthenticated(_)
            | ConfigurationError::NoProvider => callout.into_any_element(),
        }
    }

    #[allow(dead_code)]
    fn render_text_thread(
        &self,
        text_thread_editor: &Entity<TextThreadEditor>,
        buffer_search_bar: &Entity<BufferSearchBar>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let mut registrar = buffer_search::DivRegistrar::new(
            |this, _, _cx| {
                if let Some(tab) = this.threads.get(this.active_thread_index) {
                     if let ThreadView::Text { buffer_search_bar, .. } = &tab.view {
                         return Some(buffer_search_bar.clone());
                     }
                }
                None
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
            .child(text_thread_editor.clone())
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
                    .iter()
                    .map(|path| {
                        Workspace::project_path_for_path(this.project.clone(), path, false, cx)
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
        if self.special_view.is_some() {
            return;
        }

        if let Some(tab) = self.threads.get(self.active_thread_index) {
            match &tab.view {
                ThreadView::Acp(thread_view) => {
                    thread_view.update(cx, |thread_view, cx| {
                        thread_view.insert_dragged_files(paths, added_worktrees, window, cx);
                    });
                }
                ThreadView::Text {
                    text_thread_editor, ..
                } => {
                    text_thread_editor.update(cx, |text_thread_editor, cx| {
                        TextThreadEditor::insert_dragged_files(
                            text_thread_editor,
                            paths,
                            added_worktrees,
                            window,
                            cx,
                        );
                    });
                }
            }
        }
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AgentPanel");
        if let Some(tab) = self.threads.get(self.active_thread_index) {
            match &tab.view {
                ThreadView::Acp(_) => key_context.add("acp_thread"),
                ThreadView::Text { .. } => key_context.add("text_thread"),
            }
        }
        key_context
    }
    
    fn render_tab_bar(&self, _window: &mut Window, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        // Only show tab bar when there are 2+ threads and no special view is active
        let show_tab_bar = self.threads.len() >= 2 && self.special_view.is_none();
        
        if !show_tab_bar {
            return None;
        }
        
        Some(
            h_flex()
                .id("agent_thread_tab_bar")
                .w_full()
                .h(Tab::container_height(cx))
                .bg(cx.theme().colors().tab_bar_background)
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .overflow_x_scroll()
                .track_scroll(&self.tab_bar_scroll_handle)
                .children(
                    self.threads.iter().enumerate().map(|(index, tab)| {
                        self.render_tab(index, tab, cx)
                    })
                )
        )
    }
    
    fn render_tab(&self, index: usize, tab: &ThreadTab, cx: &Context<Self>) -> impl IntoElement {
        let is_active = index == self.active_thread_index;
        
        // Get thread title
        let title: SharedString = match &tab.view {
            ThreadView::Acp(thread_view) => {
                thread_view.read(cx).thread()
                    .and_then(|t| t.read(cx).title().map(|s| s.into()))
                    .unwrap_or_else(|| "New Thread".into())
            }
            ThreadView::Text { text_thread_editor, .. } => {
                text_thread_editor.read(cx).title(cx).to_string().into()
            }
        };
        
        Tab::new(("thread_tab", index))
            .toggle_state(is_active)
            .on_click(cx.listener(move |this, _, window, cx| {
                this.activate_thread(index, true, window, cx);
            }))
            .end_slot(
                IconButton::new(("close_tab", index), IconName::Close)
                    .icon_size(IconSize::XSmall)
                    .visible_on_hover("")
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.close_thread(index, window, cx);
                    }))
            )
            .child(title)
    }

    fn render_active_content(&mut self, cx: &mut Context<Self>) -> AnyElement {
        if let Some(special) = &self.special_view {
            match special {
                SpecialView::History => self.acp_history.clone().into_any_element(),
                SpecialView::Configuration => {
                     if let Some(configuration) = &self.configuration {
                         configuration.clone().into_any_element()
                     } else {
                         div().into_any_element()
                     }
                }
            }
        } else if let Some(tab) = self.threads.get(self.active_thread_index) {
            match &tab.view {
                ThreadView::Acp(view) => {
                     div()
                        .size_full()
                        .child(view.clone())
                        .child(self.render_drag_target(cx))
                        .into_any_element()
                }
                ThreadView::Text { text_thread_editor, buffer_search_bar, .. } => {
                     let model_registry = LanguageModelRegistry::read_global(cx);
                     let configuration_error = model_registry.configuration_error(model_registry.default_model(), cx);
                     
                     div()
                        .size_full()
                        .child(text_thread_editor.clone())
                        .children(configuration_error.map(|error| {
                            Label::new(error.to_string())
                                .color(Color::Error)
                                .into_any_element()
                        }))
                        .child(buffer_search_bar.clone())
                        .into_any_element()
                }
            }
        } else {
            div().into_any_element()
        }
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
            .on_action(cx.listener(Self::go_back))
            .on_action(cx.listener(Self::toggle_navigation_menu))
            .on_action(cx.listener(Self::toggle_options_menu))
            .on_action(cx.listener(Self::increase_font_size))
            .on_action(cx.listener(Self::decrease_font_size))
            .on_action(cx.listener(Self::reset_font_size))
            .on_action(cx.listener(Self::toggle_zoom))
            .on_action(cx.listener(|this, _: &ReauthenticateAgent, window, cx| {
                if let Some(thread_view) = this.active_thread_view() {
                    thread_view.update(cx, |thread_view, cx| thread_view.reauthenticate(window, cx))
                }
            }))
            .on_action(cx.listener(Self::activate_next_thread))
            .on_action(cx.listener(Self::activate_previous_thread))
            .on_action(cx.listener(Self::close_active_thread))
            .child(self.render_toolbar(window, cx))
            .children(self.render_tab_bar(window, cx))
            .children(self.render_onboarding(window, cx))
            .child(self.render_active_content(cx))
            .children(self.render_trial_end_upsell(window, cx));

        let font_size = if let Some(tab) = self.threads.get(self.active_thread_index) {
             match &tab.view {
                 ThreadView::Acp(_) => WhichFontSize::AgentFont,
                 _ => WhichFontSize::Default,
             }
        } else {
            WhichFontSize::Default
        };

        match font_size {
            WhichFontSize::AgentFont => {
                WithRemSize::new(ThemeSettings::get_global(cx).agent_ui_font_size(cx))
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
            let Some(workspace) = self.workspace.upgrade() else {
                return;
            };
            let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
                return;
            };
            let project = workspace.read(cx).project().downgrade();
            let thread_store = panel.read(cx).thread_store().clone();
            assistant.assist(
                prompt_editor,
                self.workspace.clone(),
                project,
                thread_store,
                None,
                initial_prompt,
                window,
                cx,
            );
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
    fn active_text_thread_editor(
        &self,
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<TextThreadEditor>> {
        let panel = workspace.panel::<AgentPanel>(cx)?;
        panel.read(cx).active_text_thread_editor()
    }

    fn open_local_text_thread(
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
            panel.open_saved_text_thread(path, window, cx)
        })
    }

    fn open_remote_text_thread(
        &self,
        _workspace: &mut Workspace,
        _text_thread_id: assistant_text_thread::TextThreadId,
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
                if let Some(thread_view) = panel.active_thread_view() {
                    thread_view.update(cx, |thread_view, cx| {
                        thread_view.insert_selections(window, cx);
                    });
                } else if let Some(text_thread_editor) = panel.active_text_thread_editor() {
                    let snapshot = buffer.read(cx).snapshot(cx);
                    let selection_ranges = selection_ranges
                        .into_iter()
                        .map(|range| range.to_point(&snapshot))
                        .collect::<Vec<_>>();

                    text_thread_editor.update(cx, |text_thread_editor, cx| {
                        text_thread_editor.quote_ranges(selection_ranges, snapshot, window, cx)
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

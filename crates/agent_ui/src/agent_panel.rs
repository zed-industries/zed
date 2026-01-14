use std::{ops::Range, path::Path, rc::Rc, sync::Arc, time::Duration};

use acp_thread::{AcpThread, AgentSessionInfo};
use agent::{ContextServerRegistry, ThreadStore};
use agent_servers::AgentServer;
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
    AddContextServer, AgentDiffPane, Follow, InlineAssistant, NewTextThread, NewThread,
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
    text_thread_history::{TextThreadHistory, TextThreadHistoryEvent},
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
    Action, Animation, AnimationExt, AnyElement, App, AsyncWindowContext, Corner, DismissEvent,
    Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable, KeyContext, Pixels, Subscription,
    Task, UpdateGlobal, WeakEntity, prelude::*, pulsating_between,
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
const RECENTLY_UPDATED_MENU_LIMIT: usize = 6;
const DEFAULT_THREAD_TITLE: &str = "New Thread";

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentPanel {
    width: Option<Pixels>,
    selected_agent: Option<AgentType>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HistoryKind {
    AgentThreads,
    TextThreads,
}

enum ActiveView {
    ExternalAgentThread {
        thread_view: Entity<AcpThreadView>,
    },
    TextThread {
        text_thread_editor: Entity<TextThreadEditor>,
        title_editor: Entity<Editor>,
        buffer_search_bar: Entity<BufferSearchBar>,
        _subscriptions: Vec<gpui::Subscription>,
    },
    History {
        kind: HistoryKind,
    },
    Configuration,
}

enum WhichFontSize {
    AgentFont,
    BufferFont,
    None,
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
            Self::Custom { .. } => Some(IconName::Sparkle),
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

impl ActiveView {
    pub fn which_font_size_used(&self) -> WhichFontSize {
        match self {
            ActiveView::ExternalAgentThread { .. } | ActiveView::History { .. } => {
                WhichFontSize::AgentFont
            }
            ActiveView::TextThread { .. } => WhichFontSize::BufferFont,
            ActiveView::Configuration => WhichFontSize::None,
        }
    }

    fn native_agent(
        fs: Arc<dyn Fs>,
        prompt_store: Option<Entity<PromptStore>>,
        thread_store: Entity<ThreadStore>,
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        history: Entity<AcpThreadHistory>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let thread_view = cx.new(|cx| {
            crate::acp::AcpThreadView::new(
                ExternalAgent::NativeAgent.server(fs, thread_store.clone()),
                None,
                None,
                workspace,
                project,
                Some(thread_store),
                prompt_store,
                history,
                false,
                window,
                cx,
            )
        });

        Self::ExternalAgentThread { thread_view }
    }

    pub fn text_thread(
        text_thread_editor: Entity<TextThreadEditor>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let title = text_thread_editor.read(cx).title(cx).to_string();

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
                move |text_thread, event, window, cx| match event {
                    TextThreadEvent::SummaryGenerated => {
                        let summary = text_thread.read(cx).summary().or_default();

                        editor.update(cx, |editor, cx| {
                            editor.set_text(summary, window, cx);
                        })
                    }
                    TextThreadEvent::PathChanged { .. } => {}
                    _ => {}
                }
            }),
        ];

        let buffer_search_bar =
            cx.new(|cx| BufferSearchBar::new(Some(language_registry), window, cx));
        buffer_search_bar.update(cx, |buffer_search_bar, cx| {
            buffer_search_bar.set_active_pane_item(Some(&text_thread_editor), window, cx)
        });

        Self::TextThread {
            text_thread_editor,
            title_editor: editor,
            buffer_search_bar,
            _subscriptions: subscriptions,
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
    text_thread_history: Entity<TextThreadHistory>,
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
    prompt_store: Option<Entity<PromptStore>>,
    context_server_registry: Entity<ContextServerRegistry>,
    configuration: Option<Entity<AgentConfiguration>>,
    configuration_subscription: Option<Subscription>,
    active_view: ActiveView,
    previous_view: Option<ActiveView>,
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
    show_trust_workspace_message: bool,
}

impl AgentPanel {
    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let selected_agent = self.selected_agent.clone();
        self.pending_serialization = Some(cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(
                    AGENT_PANEL_KEY.into(),
                    serde_json::to_string(&SerializedAgentPanel {
                        width,
                        selected_agent: Some(selected_agent),
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
                        if let Some(selected_agent) = serialized_panel.selected_agent {
                            panel.selected_agent = selected_agent.clone();
                            panel.new_agent_thread(selected_agent, window, cx);
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

        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        let acp_history = cx.new(|cx| AcpThreadHistory::new(None, window, cx));
        let text_thread_history =
            cx.new(|cx| TextThreadHistory::new(text_thread_store.clone(), window, cx));
        cx.subscribe_in(
            &acp_history,
            window,
            |this, _, event, window, cx| match event {
                ThreadHistoryEvent::Open(thread) => {
                    this.external_thread(
                        Some(crate::ExternalAgent::NativeAgent),
                        Some(thread.clone()),
                        None,
                        window,
                        cx,
                    );
                }
            },
        )
        .detach();
        cx.subscribe_in(
            &text_thread_history,
            window,
            |this, _, event, window, cx| match event {
                TextThreadHistoryEvent::Open(thread) => {
                    this.open_saved_text_thread(thread.path.clone(), window, cx)
                        .detach_and_log_err(cx);
                }
            },
        )
        .detach();

        let panel_type = AgentSettings::get_global(cx).default_view;
        let active_view = match panel_type {
            DefaultView::Thread => ActiveView::native_agent(
                fs.clone(),
                prompt_store.clone(),
                thread_store.clone(),
                project.clone(),
                workspace.clone(),
                acp_history.clone(),
                window,
                cx,
            ),
            DefaultView::TextThread => {
                let context = text_thread_store.update(cx, |store, cx| store.create(cx));
                let lsp_adapter_delegate = make_lsp_adapter_delegate(&project.clone(), cx).unwrap();
                let text_thread_editor = cx.new(|cx| {
                    let mut editor = TextThreadEditor::for_text_thread(
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
                ActiveView::text_thread(text_thread_editor, language_registry.clone(), window, cx)
            }
        };

        let weak_panel = cx.entity().downgrade();

        window.defer(cx, move |window, cx| {
            let panel = weak_panel.clone();
            let agent_navigation_menu =
                ContextMenu::build_persistent(window, cx, move |mut menu, _window, cx| {
                    if let Some(panel) = panel.upgrade() {
                        if let Some(kind) = panel.read(cx).history_kind_for_selected_agent(cx) {
                            menu =
                                Self::populate_recently_updated_menu_section(menu, panel, kind, cx);
                            menu = menu.action("View All", Box::new(OpenHistory));
                        }
                    }

                    menu = menu
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
            active_view,
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
            previous_view: None,
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
            text_thread_history,
            thread_store,
            selected_agent: AgentType::default(),
            loading: false,
            show_trust_workspace_message: false,
        };

        // Initial sync of agent servers from extensions
        panel.sync_agent_servers_from_extensions(cx);
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

    pub fn thread_store(&self) -> &Entity<ThreadStore> {
        &self.thread_store
    }

    pub fn history(&self) -> &Entity<AcpThreadHistory> {
        &self.acp_history
    }

    pub fn open_thread(
        &mut self,
        thread: AgentSessionInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.external_thread(
            Some(crate::ExternalAgent::NativeAgent),
            Some(thread),
            None,
            window,
            cx,
        );
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

    pub(crate) fn active_thread_view(&self) -> Option<&Entity<AcpThreadView>> {
        match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view, .. } => Some(thread_view),
            ActiveView::TextThread { .. }
            | ActiveView::History { .. }
            | ActiveView::Configuration => None,
        }
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
            .acp_history
            .read(cx)
            .session_for_id(&action.from_session_id)
        else {
            return;
        };

        self.external_thread(
            Some(ExternalAgent::NativeAgent),
            None,
            Some(thread),
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

        self.set_active_view(
            ActiveView::text_thread(
                text_thread_editor.clone(),
                self.language_registry.clone(),
                window,
                cx,
            ),
            true,
            window,
            cx,
        );
        text_thread_editor.focus_handle(cx).focus(window, cx);
    }

    fn external_thread(
        &mut self,
        agent_choice: Option<crate::ExternalAgent>,
        resume_thread: Option<AgentSessionInfo>,
        summarize_thread: Option<AgentSessionInfo>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let fs = self.fs.clone();
        let is_via_collab = self.project.read(cx).is_via_collab();

        const LAST_USED_EXTERNAL_AGENT_KEY: &str = "agent_panel__last_used_external_agent";

        #[derive(Serialize, Deserialize)]
        struct LastUsedExternalAgent {
            agent: crate::ExternalAgent,
        }

        let loading = self.loading;
        let thread_store = self.thread_store.clone();

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

            let server = ext_agent.server(fs, thread_store);
            this.update_in(cx, |agent_panel, window, cx| {
                agent_panel._external_thread(
                    server,
                    resume_thread,
                    summarize_thread,
                    workspace,
                    project,
                    loading,
                    ext_agent,
                    window,
                    cx,
                );
            })?;

            anyhow::Ok(())
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
                view.focus_handle(cx).focus(window, cx);
            });
        }
    }

    fn history_kind_for_selected_agent(&self, cx: &App) -> Option<HistoryKind> {
        match self.selected_agent {
            AgentType::NativeAgent => Some(HistoryKind::AgentThreads),
            AgentType::TextThread => Some(HistoryKind::TextThreads),
            AgentType::Gemini
            | AgentType::ClaudeCode
            | AgentType::Codex
            | AgentType::Custom { .. } => {
                if self.acp_history.read(cx).has_session_list() {
                    Some(HistoryKind::AgentThreads)
                } else {
                    None
                }
            }
        }
    }

    fn open_history(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(kind) = self.history_kind_for_selected_agent(cx) else {
            return;
        };

        if let ActiveView::History { kind: active_kind } = self.active_view {
            if active_kind == kind {
                if let Some(previous_view) = self.previous_view.take() {
                    self.set_active_view(previous_view, true, window, cx);
                }
                return;
            }
        }

        self.set_active_view(ActiveView::History { kind }, true, window, cx);
        cx.notify();
    }

    pub(crate) fn open_saved_text_thread(
        &mut self,
        path: Arc<Path>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let text_thread_task = self
            .text_thread_store
            .update(cx, |store, cx| store.open_local(path, cx));
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

        self.set_active_view(
            ActiveView::text_thread(editor, self.language_registry.clone(), window, cx),
            true,
            window,
            cx,
        );
    }

    pub fn go_back(&mut self, _: &workspace::GoBack, window: &mut Window, cx: &mut Context<Self>) {
        match self.active_view {
            ActiveView::Configuration | ActiveView::History { .. } => {
                if let Some(previous_view) = self.previous_view.take() {
                    self.active_view = previous_view;

                    match &self.active_view {
                        ActiveView::ExternalAgentThread { thread_view } => {
                            thread_view.focus_handle(cx).focus(window, cx);
                        }
                        ActiveView::TextThread {
                            text_thread_editor, ..
                        } => {
                            text_thread_editor.focus_handle(cx).focus(window, cx);
                        }
                        ActiveView::History { .. } | ActiveView::Configuration => {}
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
        if self.history_kind_for_selected_agent(cx).is_none() {
            return;
        }
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
        match self.active_view.which_font_size_used() {
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

        self.set_active_view(ActiveView::Configuration, true, window, cx);
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

            configuration.focus_handle(cx).focus(window, cx);
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
            ActiveView::ExternalAgentThread { thread_view } => {
                thread_view
                    .update(cx, |thread_view, cx| {
                        thread_view.open_thread_as_markdown(workspace, window, cx)
                    })
                    .detach_and_log_err(cx);
            }
            ActiveView::TextThread { .. }
            | ActiveView::History { .. }
            | ActiveView::Configuration => {}
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
                if let Some((thread, model)) = self
                    .active_native_agent_thread(cx)
                    .zip(provider.default_model(cx))
                {
                    thread.update(cx, |thread, cx| {
                        thread.set_model(model, cx);
                    });
                }
            }
        }
    }

    pub(crate) fn active_agent_thread(&self, cx: &App) -> Option<Entity<AcpThread>> {
        match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view, .. } => {
                thread_view.read(cx).thread().cloned()
            }
            _ => None,
        }
    }

    pub(crate) fn active_native_agent_thread(&self, cx: &App) -> Option<Entity<agent::Thread>> {
        match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view, .. } => {
                thread_view.read(cx).as_native_thread(cx)
            }
            _ => None,
        }
    }

    pub(crate) fn active_text_thread_editor(&self) -> Option<Entity<TextThreadEditor>> {
        match &self.active_view {
            ActiveView::TextThread {
                text_thread_editor, ..
            } => Some(text_thread_editor.clone()),
            _ => None,
        }
    }

    fn set_active_view(
        &mut self,
        new_view: ActiveView,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let current_is_history = matches!(self.active_view, ActiveView::History { .. });
        let new_is_history = matches!(new_view, ActiveView::History { .. });

        let current_is_config = matches!(self.active_view, ActiveView::Configuration);
        let new_is_config = matches!(new_view, ActiveView::Configuration);

        let current_is_special = current_is_history || current_is_config;
        let new_is_special = new_is_history || new_is_config;

        match &new_view {
            ActiveView::TextThread { .. } => {}
            ActiveView::ExternalAgentThread { .. } => {}
            ActiveView::History { .. } | ActiveView::Configuration => {}
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

        if focus {
            self.focus_handle(cx).focus(window, cx);
        }
    }

    fn populate_recently_updated_menu_section(
        mut menu: ContextMenu,
        panel: Entity<Self>,
        kind: HistoryKind,
        cx: &mut Context<ContextMenu>,
    ) -> ContextMenu {
        match kind {
            HistoryKind::AgentThreads => {
                let entries = panel
                    .read(cx)
                    .acp_history
                    .read(cx)
                    .sessions()
                    .iter()
                    .take(RECENTLY_UPDATED_MENU_LIMIT)
                    .cloned()
                    .collect::<Vec<_>>();

                if entries.is_empty() {
                    return menu;
                }

                menu = menu.header("Recently Updated");

                for entry in entries {
                    let title = entry
                        .title
                        .as_ref()
                        .filter(|title| !title.is_empty())
                        .cloned()
                        .unwrap_or_else(|| SharedString::new_static(DEFAULT_THREAD_TITLE));

                    menu = menu.entry(title, None, {
                        let panel = panel.downgrade();
                        let entry = entry.clone();
                        move |window, cx| {
                            let entry = entry.clone();
                            panel
                                .update(cx, move |this, cx| {
                                    this.external_thread(
                                        Some(ExternalAgent::NativeAgent),
                                        Some(entry.clone()),
                                        None,
                                        window,
                                        cx,
                                    );
                                })
                                .ok();
                        }
                    });
                }
            }
            HistoryKind::TextThreads => {
                let entries = panel
                    .read(cx)
                    .text_thread_store
                    .read(cx)
                    .ordered_text_threads()
                    .take(RECENTLY_UPDATED_MENU_LIMIT)
                    .cloned()
                    .collect::<Vec<_>>();

                if entries.is_empty() {
                    return menu;
                }

                menu = menu.header("Recently Updated");

                for entry in entries {
                    let title = if entry.title.is_empty() {
                        SharedString::new_static(DEFAULT_THREAD_TITLE)
                    } else {
                        entry.title.clone()
                    };

                    menu = menu.entry(title, None, {
                        let panel = panel.downgrade();
                        let entry = entry.clone();
                        move |window, cx| {
                            let path = entry.path.clone();
                            panel
                                .update(cx, move |this, cx| {
                                    this.open_saved_text_thread(path.clone(), window, cx)
                                        .detach_and_log_err(cx);
                                })
                                .ok();
                        }
                    });
                }
            }
        }

        menu.separator()
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
        thread: AgentSessionInfo,
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

    fn _external_thread(
        &mut self,
        server: Rc<dyn AgentServer>,
        resume_thread: Option<AgentSessionInfo>,
        summarize_thread: Option<AgentSessionInfo>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        loading: bool,
        ext_agent: ExternalAgent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_agent = AgentType::from(ext_agent);
        if self.selected_agent != selected_agent {
            self.selected_agent = selected_agent;
            self.serialize(cx);
        }
        let thread_store = server
            .clone()
            .downcast::<agent::NativeAgentServer>()
            .is_some()
            .then(|| self.thread_store.clone());

        let thread_view = cx.new(|cx| {
            crate::acp::AcpThreadView::new(
                server,
                resume_thread,
                summarize_thread,
                workspace.clone(),
                project,
                thread_store,
                self.prompt_store.clone(),
                self.acp_history.clone(),
                !loading,
                window,
                cx,
            )
        });

        self.set_active_view(
            ActiveView::ExternalAgentThread { thread_view },
            !loading,
            window,
            cx,
        );
    }
}

impl Focusable for AgentPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view, .. } => thread_view.focus_handle(cx),
            ActiveView::History { kind } => match kind {
                HistoryKind::AgentThreads => self.acp_history.focus_handle(cx),
                HistoryKind::TextThreads => self.text_thread_history.focus_handle(cx),
            },
            ActiveView::TextThread {
                text_thread_editor, ..
            } => text_thread_editor.focus_handle(cx),
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

        let content = match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view } => {
                let is_generating_title = thread_view
                    .read(cx)
                    .as_native_thread(cx)
                    .map_or(false, |t| t.read(cx).is_generating_title());

                if let Some(title_editor) = thread_view.read(cx).title_editor() {
                    let container = div()
                        .w_full()
                        .on_action({
                            let thread_view = thread_view.downgrade();
                            move |_: &menu::Confirm, window, cx| {
                                if let Some(thread_view) = thread_view.upgrade() {
                                    thread_view.focus_handle(cx).focus(window, cx);
                                }
                            }
                        })
                        .on_action({
                            let thread_view = thread_view.downgrade();
                            move |_: &editor::actions::Cancel, window, cx| {
                                if let Some(thread_view) = thread_view.upgrade() {
                                    thread_view.focus_handle(cx).focus(window, cx);
                                }
                            }
                        })
                        .child(title_editor);

                    if is_generating_title {
                        container
                            .with_animation(
                                "generating_title",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 0.8)),
                                |div, delta| div.opacity(delta),
                            )
                            .into_any_element()
                    } else {
                        container.into_any_element()
                    }
                } else {
                    Label::new(thread_view.read(cx).title(cx))
                        .color(Color::Muted)
                        .truncate()
                        .into_any_element()
                }
            }
            ActiveView::TextThread {
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
                                .with_animation(
                                    "generating_title",
                                    Animation::new(Duration::from_secs(2))
                                        .repeat()
                                        .with_easing(pulsating_between(0.4, 0.8)),
                                    |label, delta| label.alpha(delta),
                                )
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
            ActiveView::History { kind } => {
                let title = match kind {
                    HistoryKind::AgentThreads => "History",
                    HistoryKind::TextThreads => "Text Threads",
                };
                Label::new(title).truncate().into_any_element()
            }
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

    fn handle_regenerate_thread_title(thread_view: Entity<AcpThreadView>, cx: &mut App) {
        thread_view.update(cx, |thread_view, cx| {
            if let Some(thread) = thread_view.as_native_thread(cx) {
                thread.update(cx, |thread, cx| {
                    thread.generate_title(cx);
                });
            }
        });
    }

    fn handle_regenerate_text_thread_title(
        text_thread_editor: Entity<TextThreadEditor>,
        cx: &mut App,
    ) {
        text_thread_editor.update(cx, |text_thread_editor, cx| {
            text_thread_editor.regenerate_summary(cx);
        });
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

        let text_thread_view = match &self.active_view {
            ActiveView::TextThread {
                text_thread_editor, ..
            } => Some(text_thread_editor.clone()),
            _ => None,
        };
        let text_thread_with_messages = match &self.active_view {
            ActiveView::TextThread {
                text_thread_editor, ..
            } => text_thread_editor
                .read(cx)
                .text_thread()
                .read(cx)
                .messages(cx)
                .any(|message| message.role == language_model::Role::Assistant),
            _ => false,
        };

        let thread_view = match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view } => Some(thread_view.clone()),
            _ => None,
        };
        let thread_with_messages = match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view } => {
                thread_view.read(cx).has_user_submitted_prompt(cx)
            }
            _ => false,
        };

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

                        if thread_with_messages | text_thread_with_messages {
                            menu = menu.header("Current Thread");

                            if let Some(text_thread_view) = text_thread_view.as_ref() {
                                menu = menu
                                    .entry("Regenerate Thread Title", None, {
                                        let text_thread_view = text_thread_view.clone();
                                        move |_, cx| {
                                            Self::handle_regenerate_text_thread_title(
                                                text_thread_view.clone(),
                                                cx,
                                            );
                                        }
                                    })
                                    .separator();
                            }

                            if let Some(thread_view) = thread_view.as_ref() {
                                menu = menu
                                    .entry("Regenerate Thread Title", None, {
                                        let thread_view = thread_view.clone();
                                        move |_, cx| {
                                            Self::handle_regenerate_thread_title(
                                                thread_view.clone(),
                                                cx,
                                            );
                                        }
                                    })
                                    .separator();
                            }
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
                            "Toggle Recently Updated Threads",
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

        let (selected_agent_custom_icon, selected_agent_label) =
            if let AgentType::Custom { name, .. } = &self.selected_agent {
                let store = agent_server_store.read(cx);
                let icon = store.agent_icon(&ExternalAgentServerName(name.clone()));

                let label = store
                    .agent_display_name(&ExternalAgentServerName(name.clone()))
                    .unwrap_or_else(|| self.selected_agent.label());
                (icon, label)
            } else {
                (None, self.selected_agent.label())
            };

        let active_thread = match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view } => {
                thread_view.read(cx).as_native_thread(cx)
            }
            ActiveView::TextThread { .. }
            | ActiveView::History { .. }
            | ActiveView::Configuration => None,
        };

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
                                    let session_id = thread.id().clone();
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
                                    let display_name = agent_server_store
                                        .agent_display_name(&agent_name)
                                        .unwrap_or_else(|| agent_name.0.clone());

                                    let mut entry = ContextMenuEntry::new(display_name);

                                    if let Some(icon_path) = icon_path {
                                        entry = entry.custom_icon_svg(icon_path);
                                    } else {
                                        entry = entry.icon(IconName::Sparkle);
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

        let show_history_menu = self.history_kind_for_selected_agent(cx).is_some();

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
                    .child(match &self.active_view {
                        ActiveView::History { .. } | ActiveView::Configuration => {
                            self.render_toolbar_back_button(cx).into_any_element()
                        }
                        _ => selected_agent.into_any_element(),
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
                    .when(show_history_menu, |this| {
                        this.child(self.render_recent_entries_menu(
                            IconName::MenuAltTemp,
                            Corner::TopRight,
                            cx,
                        ))
                    })
                    .child(self.render_panel_options_menu(window, cx)),
            )
    }

    fn should_render_trial_end_upsell(&self, cx: &mut Context<Self>) -> bool {
        if TrialEndUpsell::dismissed() {
            return false;
        }

        match &self.active_view {
            ActiveView::TextThread { .. } => {
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
            ActiveView::ExternalAgentThread { .. }
            | ActiveView::History { .. }
            | ActiveView::Configuration => return false,
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

        match &self.active_view {
            ActiveView::History { .. } | ActiveView::Configuration => false,
            ActiveView::ExternalAgentThread { thread_view, .. }
                if thread_view.read(cx).as_native_thread(cx).is_none() =>
            {
                false
            }
            _ => {
                let history_is_empty = self.acp_history.read(cx).is_empty();

                let has_configured_non_zed_providers = LanguageModelRegistry::read_global(cx)
                    .visible_providers()
                    .iter()
                    .any(|provider| {
                        provider.is_authenticated(cx)
                            && provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
                    });

                history_is_empty || !has_configured_non_zed_providers
            }
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

        let text_thread_view = matches!(&self.active_view, ActiveView::TextThread { .. });

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

    fn render_configuration_error(
        &self,
        border_bottom: bool,
        configuration_error: &ConfigurationError,
        focus_handle: &FocusHandle,
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
                            KeyBinding::for_action_in(&OpenSettings, focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
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

    fn render_text_thread(
        &self,
        text_thread_editor: &Entity<TextThreadEditor>,
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
        match &self.active_view {
            ActiveView::ExternalAgentThread { thread_view } => {
                thread_view.update(cx, |thread_view, cx| {
                    thread_view.insert_dragged_files(paths, added_worktrees, window, cx);
                });
            }
            ActiveView::TextThread {
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
            ActiveView::History { .. } | ActiveView::Configuration => {}
        }
    }

    fn render_workspace_trust_message(&self, cx: &Context<Self>) -> Option<impl IntoElement> {
        if !self.show_trust_workspace_message {
            return None;
        }

        let description = "To protect your system, third-party code—like MCP servers—won't run until you mark this workspace as safe.";

        Some(
            Callout::new()
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .border_position(ui::BorderPosition::Bottom)
                .title("You're in Restricted Mode")
                .description(description)
                .actions_slot(
                    Button::new("open-trust-modal", "Configure Project Trust")
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Outlined)
                        .on_click({
                            cx.listener(move |this, _, window, cx| {
                                this.workspace
                                    .update(cx, |workspace, cx| {
                                        workspace
                                            .show_worktree_trust_security_modal(true, window, cx)
                                    })
                                    .log_err();
                            })
                        }),
                ),
        )
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AgentPanel");
        match &self.active_view {
            ActiveView::ExternalAgentThread { .. } => key_context.add("acp_thread"),
            ActiveView::TextThread { .. } => key_context.add("text_thread"),
            ActiveView::History { .. } | ActiveView::Configuration => {}
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
            .child(self.render_toolbar(window, cx))
            .children(self.render_workspace_trust_message(cx))
            .children(self.render_onboarding(window, cx))
            .map(|parent| match &self.active_view {
                ActiveView::ExternalAgentThread { thread_view, .. } => parent
                    .child(thread_view.clone())
                    .child(self.render_drag_target(cx)),
                ActiveView::History { kind } => match kind {
                    HistoryKind::AgentThreads => parent.child(self.acp_history.clone()),
                    HistoryKind::TextThreads => parent.child(self.text_thread_history.clone()),
                },
                ActiveView::TextThread {
                    text_thread_editor,
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
                                this.child(self.render_configuration_error(
                                    true,
                                    err,
                                    &self.focus_handle(cx),
                                    cx,
                                ))
                            } else {
                                this
                            }
                        })
                        .child(self.render_text_thread(
                            text_thread_editor,
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
            let panel = panel.read(cx);
            let thread_store = panel.thread_store().clone();
            let history = panel.history().downgrade();
            assistant.assist(
                prompt_editor,
                self.workspace.clone(),
                project,
                thread_store,
                None,
                history,
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

#[cfg(feature = "test-support")]
impl AgentPanel {
    /// Opens an external thread using an arbitrary AgentServer.
    ///
    /// This is a test-only helper that allows visual tests and integration tests
    /// to inject a stub server without modifying production code paths.
    /// Not compiled into production builds.
    pub fn open_external_thread_with_server(
        &mut self,
        server: Rc<dyn AgentServer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        let project = self.project.clone();

        let ext_agent = ExternalAgent::Custom {
            name: server.name(),
        };

        self._external_thread(
            server, None, None, workspace, project, false, ext_agent, window, cx,
        );
    }

    /// Returns the currently active thread view, if any.
    ///
    /// This is a test-only accessor that exposes the private `active_thread_view()`
    /// method for test assertions. Not compiled into production builds.
    pub fn active_thread_view_for_tests(&self) -> Option<&Entity<AcpThreadView>> {
        self.active_thread_view()
    }
}

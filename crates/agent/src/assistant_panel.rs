use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use db::kvp::KEY_VALUE_STORE;
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use serde::{Deserialize, Serialize};

use anyhow::{Result, anyhow};
use assistant_context_editor::{
    AssistantContext, AssistantPanelDelegate, ConfigurationError, ContextEditor, ContextEvent,
    SlashCommandCompletionProvider, humanize_token_count, make_lsp_adapter_delegate,
    render_remaining_tokens,
};
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_tool::ToolWorkingSet;

use client::{UserStore, zed_urls};
use editor::{Anchor, AnchorRangeExt as _, Editor, EditorEvent, MultiBuffer};
use fs::Fs;
use gpui::{
    Action, Animation, AnimationExt as _, AnyElement, App, AsyncWindowContext, ClipboardItem,
    Corner, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, KeyContext,
    Pixels, Subscription, Task, UpdateGlobal, WeakEntity, prelude::*, pulsating_between,
};
use language::LanguageRegistry;
use language_model::{LanguageModelProviderTosView, LanguageModelRegistry};
use language_model_selector::ToggleModelSelector;
use project::Project;
use prompt_store::{PromptBuilder, PromptStore, UserPromptId};
use proto::Plan;
use rules_library::{RulesLibrary, open_rules_library};
use search::{BufferSearchBar, buffer_search::DivRegistrar};
use settings::{Settings, update_settings_file};
use time::UtcOffset;
use ui::utils::WithRemSize;
use ui::{
    Banner, CheckboxWithLabel, ContextMenu, KeyBinding, PopoverMenu, PopoverMenuHandle,
    ProgressBar, Tab, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{CollaboratorId, ToolbarItemView, Workspace};
use zed_actions::agent::OpenConfiguration;
use zed_actions::assistant::{OpenRulesLibrary, ToggleFocus};
use zed_llm_client::UsageLimit;

use crate::active_thread::{ActiveThread, ActiveThreadEvent, default_markdown_style};
use crate::agent_diff::AgentDiff;
use crate::assistant_configuration::{AssistantConfiguration, AssistantConfigurationEvent};
use crate::history_store::{HistoryEntry, HistoryStore, RecentEntry};
use crate::message_editor::{MessageEditor, MessageEditorEvent};
use crate::thread::{Thread, ThreadError, ThreadId, TokenUsageRatio};
use crate::thread_history::{PastContext, PastThread, ThreadHistory};
use crate::thread_store::ThreadStore;
use crate::{
    AddContextServer, AgentDiffPane, DeleteRecentlyOpenThread, ExpandMessageEditor, Follow,
    InlineAssistant, NewTextThread, NewThread, OpenActiveThreadAsMarkdown, OpenAgentDiff,
    OpenHistory, ResetTrialUpsell, ThreadEvent, ToggleContextPicker, ToggleNavigationMenu,
    ToggleOptionsMenu,
};

const AGENT_PANEL_KEY: &str = "agent_panel";

#[derive(Serialize, Deserialize)]
struct SerializedAssistantPanel {
    width: Option<Pixels>,
}

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
                .register_action(|workspace, _: &NewTextThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.new_prompt_editor(window, cx));
                    }
                })
                .register_action(|workspace, action: &OpenRulesLibrary, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.deploy_rules_library(action, window, cx)
                        });
                    }
                })
                .register_action(|workspace, _: &OpenAgentDiff, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        let thread = panel.read(cx).thread.read(cx).thread().clone();
                        AgentDiffPane::deploy_in_workspace(thread, workspace, window, cx);
                    }
                })
                .register_action(|workspace, _: &Follow, window, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .register_action(|workspace, _: &ExpandMessageEditor, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.message_editor.update(cx, |editor, cx| {
                                editor.expand_message_editor(&ExpandMessageEditor, window, cx);
                            });
                        });
                    }
                })
                .register_action(|workspace, _: &ToggleNavigationMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_navigation_menu(&ToggleNavigationMenu, window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &ToggleOptionsMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_options_menu(&ToggleOptionsMenu, window, cx);
                        });
                    }
                })
                .register_action(|_workspace, _: &ResetTrialUpsell, _window, cx| {
                    set_trial_upsell_dismissed(false, cx);
                });
        },
    )
    .detach();
}

enum ActiveView {
    Thread {
        change_title_editor: Entity<Editor>,
        thread: WeakEntity<Thread>,
        _subscriptions: Vec<gpui::Subscription>,
    },
    PromptEditor {
        context_editor: Entity<ContextEditor>,
        title_editor: Entity<Editor>,
        buffer_search_bar: Entity<BufferSearchBar>,
        _subscriptions: Vec<gpui::Subscription>,
    },
    History,
    Configuration,
}

impl ActiveView {
    pub fn thread(thread: Entity<Thread>, window: &mut Window, cx: &mut App) -> Self {
        let summary = thread.read(cx).summary_or_default();

        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(summary.clone(), window, cx);
            editor
        });

        let subscriptions = vec![
            window.subscribe(&editor, cx, {
                {
                    let thread = thread.clone();
                    move |editor, event, window, cx| match event {
                        EditorEvent::BufferEdited => {
                            let new_summary = editor.read(cx).text(cx);

                            thread.update(cx, |thread, cx| {
                                thread.set_summary(new_summary, cx);
                            })
                        }
                        EditorEvent::Blurred => {
                            if editor.read(cx).text(cx).is_empty() {
                                let summary = thread.read(cx).summary_or_default();

                                editor.update(cx, |editor, cx| {
                                    editor.set_text(summary, window, cx);
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }),
            window.subscribe(&thread, cx, {
                let editor = editor.clone();
                move |thread, event, window, cx| match event {
                    ThreadEvent::SummaryGenerated => {
                        let summary = thread.read(cx).summary_or_default();

                        editor.update(cx, |editor, cx| {
                            editor.set_text(summary, window, cx);
                        })
                    }
                    _ => {}
                }
            }),
        ];

        Self::Thread {
            change_title_editor: editor,
            thread: thread.downgrade(),
            _subscriptions: subscriptions,
        }
    }

    pub fn prompt_editor(
        context_editor: Entity<ContextEditor>,
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
                                    .summary_or_default();

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
                        let summary = assistant_context.read(cx).summary_or_default();

                        editor.update(cx, |editor, cx| {
                            editor.set_text(summary, window, cx);
                        })
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

        Self::PromptEditor {
            context_editor,
            title_editor: editor,
            buffer_search_bar,
            _subscriptions: subscriptions,
        }
    }
}

pub struct AssistantPanel {
    workspace: WeakEntity<Workspace>,
    user_store: Entity<UserStore>,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    thread: Entity<ActiveThread>,
    message_editor: Entity<MessageEditor>,
    _active_thread_subscriptions: Vec<Subscription>,
    _default_model_subscription: Subscription,
    context_store: Entity<assistant_context_editor::ContextStore>,
    prompt_store: Option<Entity<PromptStore>>,
    configuration: Option<Entity<AssistantConfiguration>>,
    configuration_subscription: Option<Subscription>,
    local_timezone: UtcOffset,
    active_view: ActiveView,
    previous_view: Option<ActiveView>,
    history_store: Entity<HistoryStore>,
    history: Entity<ThreadHistory>,
    assistant_dropdown_menu_handle: PopoverMenuHandle<ContextMenu>,
    assistant_navigation_menu_handle: PopoverMenuHandle<ContextMenu>,
    assistant_navigation_menu: Option<Entity<ContextMenu>>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    pending_serialization: Option<Task<Result<()>>>,
    hide_trial_upsell: bool,
    trial_markdown: Entity<Markdown>,
}

impl AssistantPanel {
    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        self.pending_serialization = Some(cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(
                    AGENT_PANEL_KEY.into(),
                    serde_json::to_string(&SerializedAssistantPanel { width })?,
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
                    assistant_context_editor::ContextStore::new(
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
                Some(serde_json::from_str::<SerializedAssistantPanel>(&panel)?)
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
        context_store: Entity<assistant_context_editor::ContextStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let thread = thread_store.update(cx, |this, cx| this.create_thread(cx));
        let fs = workspace.app_state().fs.clone();
        let user_store = workspace.app_state().user_store.clone();
        let project = workspace.project();
        let language_registry = project.read(cx).languages().clone();
        let workspace = workspace.weak_handle();
        let weak_self = cx.entity().downgrade();

        let message_editor_context_store = cx.new(|_cx| {
            crate::context_store::ContextStore::new(
                project.downgrade(),
                Some(thread_store.downgrade()),
            )
        });

        let message_editor = cx.new(|cx| {
            MessageEditor::new(
                fs.clone(),
                workspace.clone(),
                user_store.clone(),
                message_editor_context_store.clone(),
                prompt_store.clone(),
                thread_store.downgrade(),
                thread.clone(),
                window,
                cx,
            )
        });

        let message_editor_subscription =
            cx.subscribe(&message_editor, |_, _, event, cx| match event {
                MessageEditorEvent::Changed | MessageEditorEvent::EstimatedTokenCount => {
                    cx.notify();
                }
            });

        let thread_id = thread.read(cx).id().clone();
        let history_store = cx.new(|cx| {
            HistoryStore::new(
                thread_store.clone(),
                context_store.clone(),
                [RecentEntry::Thread(thread_id, thread.clone())],
                cx,
            )
        });

        cx.observe(&history_store, |_, _, cx| cx.notify()).detach();

        let active_view = ActiveView::thread(thread.clone(), window, cx);
        let thread_subscription = cx.subscribe(&thread, |_, _, event, cx| {
            if let ThreadEvent::MessageAdded(_) = &event {
                // needed to leave empty state
                cx.notify();
            }
        });
        let active_thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                thread_store.clone(),
                message_editor_context_store.clone(),
                language_registry.clone(),
                workspace.clone(),
                window,
                cx,
            )
        });
        AgentDiff::set_active_thread(&workspace, &thread, window, cx);

        let active_thread_subscription =
            cx.subscribe(&active_thread, |_, _, event, cx| match &event {
                ActiveThreadEvent::EditingMessageTokenCountChanged => {
                    cx.notify();
                }
            });

        let weak_panel = weak_self.clone();

        window.defer(cx, move |window, cx| {
            let panel = weak_panel.clone();
            let assistant_navigation_menu =
                ContextMenu::build_persistent(window, cx, move |mut menu, _window, cx| {
                    let recently_opened = panel
                        .update(cx, |this, cx| {
                            this.history_store.update(cx, |history_store, cx| {
                                history_store.recently_opened_entries(cx)
                            })
                        })
                        .unwrap_or_default();

                    if !recently_opened.is_empty() {
                        menu = menu.header("Recently Opened");

                        for entry in recently_opened.iter() {
                            let summary = entry.summary(cx);

                            menu = menu.entry_with_end_slot_on_hover(
                                summary,
                                None,
                                {
                                    let panel = panel.clone();
                                    let entry = entry.clone();
                                    move |window, cx| {
                                        panel
                                            .update(cx, {
                                                let entry = entry.clone();
                                                move |this, cx| match entry {
                                                    RecentEntry::Thread(_, thread) => {
                                                        this.open_thread(thread, window, cx)
                                                    }
                                                    RecentEntry::Context(context) => {
                                                        let Some(path) = context.read(cx).path()
                                                        else {
                                                            return;
                                                        };
                                                        this.open_saved_prompt_editor(
                                                            path.clone(),
                                                            window,
                                                            cx,
                                                        )
                                                        .detach_and_log_err(cx)
                                                    }
                                                }
                                            })
                                            .ok();
                                    }
                                },
                                IconName::Close,
                                "Close Entry".into(),
                                {
                                    let panel = panel.clone();
                                    let entry = entry.clone();
                                    move |_window, cx| {
                                        panel
                                            .update(cx, |this, cx| {
                                                this.history_store.update(
                                                    cx,
                                                    |history_store, cx| {
                                                        history_store.remove_recently_opened_entry(
                                                            &entry, cx,
                                                        );
                                                    },
                                                );
                                            })
                                            .ok();
                                    }
                                },
                            );
                        }

                        menu = menu.separator();
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
                language_model::Event::DefaultModelChanged => {
                    this.thread
                        .read(cx)
                        .thread()
                        .clone()
                        .update(cx, |thread, cx| thread.get_or_init_configured_model(cx));
                }
                _ => {}
            },
        );

        let trial_markdown = cx.new(|cx| {
            Markdown::new(
                include_str!("trial_markdown.md").into(),
                Some(language_registry.clone()),
                None,
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
            thread: active_thread,
            message_editor,
            _active_thread_subscriptions: vec![
                thread_subscription,
                active_thread_subscription,
                message_editor_subscription,
            ],
            _default_model_subscription,
            context_store,
            prompt_store,
            configuration: None,
            configuration_subscription: None,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            previous_view: None,
            history_store: history_store.clone(),
            history: cx.new(|cx| ThreadHistory::new(weak_self, history_store, window, cx)),
            assistant_dropdown_menu_handle: PopoverMenuHandle::default(),
            assistant_navigation_menu_handle: PopoverMenuHandle::default(),
            assistant_navigation_menu: None,
            width: None,
            height: None,
            pending_serialization: None,
            hide_trial_upsell: false,
            trial_markdown,
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

    pub(crate) fn prompt_store(&self) -> &Option<Entity<PromptStore>> {
        &self.prompt_store
    }

    pub(crate) fn thread_store(&self) -> &Entity<ThreadStore> {
        &self.thread_store
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        self.thread
            .update(cx, |thread, cx| thread.cancel_last_completion(window, cx));
    }

    fn new_thread(&mut self, action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(cx));

        let thread_view = ActiveView::thread(thread.clone(), window, cx);
        self.set_active_view(thread_view, window, cx);

        let context_store = cx.new(|_cx| {
            crate::context_store::ContextStore::new(
                self.project.downgrade(),
                Some(self.thread_store.downgrade()),
            )
        });

        if let Some(other_thread_id) = action.from_thread_id.clone() {
            let other_thread_task = self
                .thread_store
                .update(cx, |this, cx| this.open_thread(&other_thread_id, cx));

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

        let thread_subscription = cx.subscribe(&thread, |_, _, event, cx| {
            if let ThreadEvent::MessageAdded(_) = &event {
                // needed to leave empty state
                cx.notify();
            }
        });

        self.thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.thread_store.clone(),
                context_store.clone(),
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        });
        AgentDiff::set_active_thread(&self.workspace, &thread, window, cx);

        let active_thread_subscription =
            cx.subscribe(&self.thread, |_, _, event, cx| match &event {
                ActiveThreadEvent::EditingMessageTokenCountChanged => {
                    cx.notify();
                }
            });

        self.message_editor = cx.new(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                self.user_store.clone(),
                context_store,
                self.prompt_store.clone(),
                self.thread_store.downgrade(),
                thread,
                window,
                cx,
            )
        });
        self.message_editor.focus_handle(cx).focus(window);

        let message_editor_subscription =
            cx.subscribe(&self.message_editor, |_, _, event, cx| match event {
                MessageEditorEvent::Changed | MessageEditorEvent::EstimatedTokenCount => {
                    cx.notify();
                }
            });

        self._active_thread_subscriptions = vec![
            thread_subscription,
            active_thread_subscription,
            message_editor_subscription,
        ];
    }

    fn new_prompt_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let context = self
            .context_store
            .update(cx, |context_store, cx| context_store.create(cx));
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        let context_editor = cx.new(|cx| {
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
        });

        self.set_active_view(
            ActiveView::prompt_editor(
                context_editor.clone(),
                self.language_registry.clone(),
                window,
                cx,
            ),
            window,
            cx,
        );
        context_editor.focus_handle(cx).focus(window);
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
            Arc::new(|| {
                Box::new(SlashCommandCompletionProvider::new(
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

                this.set_active_view(
                    ActiveView::prompt_editor(
                        editor.clone(),
                        this.language_registry.clone(),
                        window,
                        cx,
                    ),
                    window,
                    cx,
                );

                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    pub(crate) fn open_thread_by_id(
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
        let thread_view = ActiveView::thread(thread.clone(), window, cx);
        self.set_active_view(thread_view, window, cx);
        let context_store = cx.new(|_cx| {
            crate::context_store::ContextStore::new(
                self.project.downgrade(),
                Some(self.thread_store.downgrade()),
            )
        });
        let thread_subscription = cx.subscribe(&thread, |_, _, event, cx| {
            if let ThreadEvent::MessageAdded(_) = &event {
                // needed to leave empty state
                cx.notify();
            }
        });

        self.thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                self.thread_store.clone(),
                context_store.clone(),
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        });
        AgentDiff::set_active_thread(&self.workspace, &thread, window, cx);

        let active_thread_subscription =
            cx.subscribe(&self.thread, |_, _, event, cx| match &event {
                ActiveThreadEvent::EditingMessageTokenCountChanged => {
                    cx.notify();
                }
            });

        self.message_editor = cx.new(|cx| {
            MessageEditor::new(
                self.fs.clone(),
                self.workspace.clone(),
                self.user_store.clone(),
                context_store,
                self.prompt_store.clone(),
                self.thread_store.downgrade(),
                thread,
                window,
                cx,
            )
        });
        self.message_editor.focus_handle(cx).focus(window);

        let message_editor_subscription =
            cx.subscribe(&self.message_editor, |_, _, event, cx| match event {
                MessageEditorEvent::Changed | MessageEditorEvent::EstimatedTokenCount => {
                    cx.notify();
                }
            });

        self._active_thread_subscriptions = vec![
            thread_subscription,
            active_thread_subscription,
            message_editor_subscription,
        ];
    }

    pub fn go_back(&mut self, _: &workspace::GoBack, window: &mut Window, cx: &mut Context<Self>) {
        match self.active_view {
            ActiveView::Configuration | ActiveView::History => {
                self.active_view =
                    ActiveView::thread(self.thread.read(cx).thread().clone(), window, cx);
                self.message_editor.focus_handle(cx).focus(window);
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
        self.assistant_dropdown_menu_handle.toggle(window, cx);
    }

    pub fn open_agent_diff(
        &mut self,
        _: &OpenAgentDiff,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread = self.thread.read(cx).thread().clone();
        self.workspace
            .update(cx, |workspace, cx| {
                AgentDiffPane::deploy_in_workspace(thread, workspace, window, cx)
            })
            .log_err();
    }

    pub(crate) fn open_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let context_server_manager = self.thread_store.read(cx).context_server_manager();
        let tools = self.thread_store.read(cx).tools();
        let fs = self.fs.clone();

        self.set_active_view(ActiveView::Configuration, window, cx);
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
                    .default_model()
                    .map_or(true, |model| model.provider.id() != provider.id())
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

    pub(crate) fn delete_thread(
        &mut self,
        thread_id: &ThreadId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.thread_store
            .update(cx, |this, cx| this.delete_thread(thread_id, cx))
    }

    pub(crate) fn has_active_thread(&self) -> bool {
        matches!(self.active_view, ActiveView::Thread { .. })
    }

    pub(crate) fn active_context_editor(&self) -> Option<Entity<ContextEditor>> {
        match &self.active_view {
            ActiveView::PromptEditor { context_editor, .. } => Some(context_editor.clone()),
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

        match &self.active_view {
            ActiveView::Thread { thread, .. } => self.history_store.update(cx, |store, cx| {
                if let Some(thread) = thread.upgrade() {
                    if thread.read(cx).is_empty() {
                        let id = thread.read(cx).id().clone();
                        store.remove_recently_opened_thread(id, cx);
                    }
                }
            }),
            _ => {}
        }

        match &new_view {
            ActiveView::Thread { thread, .. } => self.history_store.update(cx, |store, cx| {
                if let Some(thread) = thread.upgrade() {
                    let id = thread.read(cx).id().clone();
                    store.push_recently_opened_entry(RecentEntry::Thread(id, thread), cx);
                }
            }),
            ActiveView::PromptEditor { context_editor, .. } => {
                self.history_store.update(cx, |store, cx| {
                    let context = context_editor.read(cx).context().clone();
                    store.push_recently_opened_entry(RecentEntry::Context(context), cx)
                })
            }
            _ => {}
        }

        if current_is_history && !new_is_history {
            self.active_view = new_view;
        } else if !current_is_history && new_is_history {
            self.previous_view = Some(std::mem::replace(&mut self.active_view, new_view));
        } else {
            if !new_is_history {
                self.previous_view = None;
            }
            self.active_view = new_view;
        }

        self.focus_handle(cx).focus(window);
    }
}

impl Focusable for AssistantPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.active_view {
            ActiveView::Thread { .. } => self.message_editor.focus_handle(cx),
            ActiveView::History => self.history.focus_handle(cx),
            ActiveView::PromptEditor { context_editor, .. } => context_editor.focus_handle(cx),
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
        self.serialize(cx);
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
    fn render_title_view(&self, _window: &mut Window, cx: &Context<Self>) -> AnyElement {
        const LOADING_SUMMARY_PLACEHOLDER: &str = "Loading Summary…";

        let content = match &self.active_view {
            ActiveView::Thread {
                change_title_editor,
                ..
            } => {
                let active_thread = self.thread.read(cx);
                let is_empty = active_thread.is_empty();

                let summary = active_thread.summary(cx);

                if is_empty {
                    Label::new(Thread::DEFAULT_SUMMARY.clone())
                        .truncate()
                        .into_any_element()
                } else if summary.is_none() {
                    Label::new(LOADING_SUMMARY_PLACEHOLDER)
                        .truncate()
                        .into_any_element()
                } else {
                    div()
                        .w_full()
                        .child(change_title_editor.clone())
                        .into_any_element()
                }
            }
            ActiveView::PromptEditor {
                title_editor,
                context_editor,
                ..
            } => {
                let context_editor = context_editor.read(cx);
                let summary = context_editor.context().read(cx).summary();

                match summary {
                    None => Label::new(AssistantContext::DEFAULT_SUMMARY.clone())
                        .truncate()
                        .into_any_element(),
                    Some(summary) => {
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
        let active_thread = self.thread.read(cx);
        let thread = active_thread.thread().read(cx);
        let thread_id = thread.id().clone();
        let is_empty = active_thread.is_empty();
        let last_usage = active_thread.thread().read(cx).last_usage();
        let account_url = zed_urls::account_url(cx);

        let show_token_count = match &self.active_view {
            ActiveView::Thread { .. } => !is_empty,
            ActiveView::PromptEditor { .. } => true,
            _ => false,
        };

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

        let agent_extra_menu = PopoverMenu::new("agent-options-menu")
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
            .with_handle(self.assistant_dropdown_menu_handle.clone())
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, |mut menu, _window, _cx| {
                    menu = menu
                        .action("New Thread", NewThread::default().boxed_clone())
                        .action("New Text Thread", NewTextThread.boxed_clone())
                        .when(!is_empty, |menu| {
                            menu.action(
                                "New From Summary",
                                Box::new(NewThread {
                                    from_thread_id: Some(thread_id.clone()),
                                }),
                            )
                        })
                        .separator();

                    menu = menu
                        .header("MCP Servers")
                        .action(
                            "View Server Extensions",
                            Box::new(zed_actions::Extensions {
                                category_filter: Some(
                                    zed_actions::ExtensionCategoryFilter::ContextServers,
                                ),
                            }),
                        )
                        .action("Add Custom Server…", Box::new(AddContextServer))
                        .separator();

                    if let Some(usage) = last_usage {
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
                        .action("Rules…", Box::new(OpenRulesLibrary::default()))
                        .action("Settings", Box::new(OpenConfiguration));
                    menu
                }))
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
                    .when(show_token_count, |parent| {
                        parent.children(self.render_token_count(&thread, cx))
                    })
                    .child(
                        h_flex()
                            .h_full()
                            .gap(DynamicSpacing::Base02.rems(cx))
                            .px(DynamicSpacing::Base08.rems(cx))
                            .border_l_1()
                            .border_color(cx.theme().colors().border)
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
                            .child(agent_extra_menu),
                    ),
            )
    }

    fn render_token_count(&self, thread: &Thread, cx: &App) -> Option<AnyElement> {
        let is_generating = thread.is_generating();
        let message_editor = self.message_editor.read(cx);

        let conversation_token_usage = thread.total_token_usage()?;

        let (total_token_usage, is_estimating) = if let Some((editing_message_id, unsent_tokens)) =
            self.thread.read(cx).editing_message_id()
        {
            let combined = thread
                .token_usage_up_to_message(editing_message_id)
                .add(unsent_tokens);

            (combined, unsent_tokens > 0)
        } else {
            let unsent_tokens = message_editor.last_estimated_token_count().unwrap_or(0);
            let combined = conversation_token_usage.add(unsent_tokens);

            (combined, unsent_tokens > 0)
        };

        let is_waiting_to_update_token_count = message_editor.is_waiting_to_update_token_count();

        match &self.active_view {
            ActiveView::Thread { .. } => {
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
            ActiveView::PromptEditor { context_editor, .. } => {
                let element = render_remaining_tokens(context_editor, cx)?;

                Some(element.into_any_element())
            }
            _ => None,
        }
    }

    fn should_render_upsell(&self, _cx: &mut Context<Self>) -> bool {
        if self.hide_trial_upsell || dismissed_trial_upsell() {
            return false;
        }

        // let plan = self.user_store.read(cx).current_plan();
        // if matches!(plan, Some(Plan::ZedPro | Plan::ZedProTrial)) {
        //     return false;
        // }

        // let has_previous_trial = self.user_store.read(cx).trial_started_at().is_some();
        // if has_previous_trial {
        //     return false;
        // }

        true
    }

    fn render_trial_upsell(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_upsell(cx) {
            return None;
        }

        let default_md_style = default_markdown_style(window, cx);

        let mut text_style = default_md_style.base_text_style;
        text_style.font_size = px(4.0).into();

        let md_style = MarkdownStyle {
            base_text_style: text_style,
            ..default_markdown_style(window, cx)
        };

        let checkbox = CheckboxWithLabel::new(
            "dont-show-again",
            Label::new("Don't show again").color(Color::Muted),
            ToggleState::Unselected,
            move |toggle_state, _window, cx| {
                let toggle_state_bool = toggle_state.selected();

                set_trial_upsell_dismissed(toggle_state_bool, cx);
            },
        );

        Some(
            div().p_4().child(
                v_flex()
                    .w_full()
                    .elevation_2(cx)
                    .bg(cx.theme().colors().background.alpha(0.5))
                    .p_4()
                    .gap_6()
                    .child(
                        WithRemSize::new(14.)
                            .max_w(px(540.))
                            .child(MarkdownElement::new(self.trial_markdown.clone(), md_style)),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .px_neg_2()
                            .justify_between()
                            .items_center()
                            .child(h_flex().items_center().gap_1().child(checkbox))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("dismiss-button", "No Thanks")
                                            .style(ButtonStyle::Subtle)
                                            .color(Color::Muted)
                                            .on_click({
                                                let assistant_panel = cx.entity();
                                                move |_, _, cx| {
                                                    assistant_panel.update(cx, |this, cx| {
                                                        let hidden = this.hide_trial_upsell;
                                                        println!("hidden: {}", hidden);
                                                        this.hide_trial_upsell = true;
                                                        let new_hidden = this.hide_trial_upsell;
                                                        println!("new_hidden: {}", new_hidden);

                                                        cx.notify();
                                                    });
                                                }
                                            }),
                                    )
                                    .child(
                                        Button::new("cta-button", "Upgrade Now")
                                            .style(ButtonStyle::Filled)
                                            .on_click(|_, _, cx| {
                                                cx.open_url(&zed_urls::account_url(cx))
                                            }),
                                    ),
                            ),
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
        let Some(model) = LanguageModelRegistry::read_global(cx).default_model() else {
            return Some(ConfigurationError::NoProvider);
        };

        if !model.provider.is_authenticated(cx) {
            return Some(ConfigurationError::ProviderNotAuthenticated);
        }

        if model.provider.must_accept_terms(cx) {
            return Some(ConfigurationError::ProviderPendingTermsAcceptance(
                model.provider,
            ));
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
                    .overflow_hidden()
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
                                            PastThread::new(thread, cx.entity().downgrade(), false, vec![])
                                                .into_any_element()
                                        }
                                        HistoryEntry::Context(context) => {
                                            PastContext::new(context, cx.entity().downgrade(), false, vec![])
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
                                            .child(
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
                                            .child(
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

    fn render_tool_use_limit_reached(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let tool_use_limit_reached = self
            .thread
            .read(cx)
            .thread()
            .read(cx)
            .tool_use_limit_reached();
        if !tool_use_limit_reached {
            return None;
        }

        let model = self
            .thread
            .read(cx)
            .thread()
            .read(cx)
            .configured_model()?
            .model;

        let max_mode_upsell = if model.supports_max_mode() {
            " Enable max mode for unlimited tool use."
        } else {
            ""
        };

        Some(
            Banner::new()
                .severity(ui::Severity::Info)
                .child(h_flex().child(Label::new(format!(
                    "Consecutive tool use limit reached.{max_mode_upsell}"
                ))))
                .into_any_element(),
        )
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
                    ThreadError::ModelRequestLimitReached { plan } => {
                        self.render_model_request_limit_reached_error(plan, cx)
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
                    .gap_1()
                    .child(self.create_copy_button(ERROR_MESSAGE))
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
                    .gap_1()
                    .child(self.create_copy_button(ERROR_MESSAGE))
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

    fn render_model_request_limit_reached_error(
        &self,
        plan: Plan,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let error_message = match plan {
            Plan::ZedPro => {
                "Model request limit reached. Upgrade to usage-based billing for more requests."
            }
            Plan::ZedProTrial => {
                "Model request limit reached. Upgrade to Zed Pro for more requests."
            }
            Plan::Free => "Model request limit reached. Upgrade to Zed Pro for more requests.",
        };
        let call_to_action = match plan {
            Plan::ZedPro => "Upgrade to usage-based billing",
            Plan::ZedProTrial => "Upgrade to Zed Pro",
            Plan::Free => "Upgrade to Zed Pro",
        };

        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new("Model Request Limit Reached").weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(error_message)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .gap_1()
                    .child(self.create_copy_button(error_message))
                    .child(
                        Button::new("subscribe", call_to_action).on_click(cx.listener(
                            |this, _, _, cx| {
                                this.thread.update(cx, |this, _cx| {
                                    this.clear_last_error();
                                });

                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            },
                        )),
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
        let message_with_header = format!("{}\n{}", header, message);
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
                    .child(Label::new(message.clone())),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .gap_1()
                    .child(self.create_copy_button(message_with_header))
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

    fn create_copy_button(&self, message: impl Into<String>) -> impl IntoElement {
        let message = message.into();
        IconButton::new("copy", IconName::Copy)
            .on_click(move |_, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(message.clone()))
            })
            .tooltip(Tooltip::text("Copy Error Message"))
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AgentPanel");
        if matches!(self.active_view, ActiveView::PromptEditor { .. }) {
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
            .on_action(cx.listener(Self::deploy_rules_library))
            .on_action(cx.listener(Self::open_agent_diff))
            .on_action(cx.listener(Self::go_back))
            .on_action(cx.listener(Self::toggle_navigation_menu))
            .on_action(cx.listener(Self::toggle_options_menu))
            .child(self.render_toolbar(window, cx))
            .children(self.render_trial_upsell(window, cx))
            .map(|parent| match &self.active_view {
                ActiveView::Thread { .. } => parent
                    .child(self.render_active_thread_or_empty_state(window, cx))
                    .children(self.render_tool_use_limit_reached(cx))
                    .child(h_flex().child(self.message_editor.clone()))
                    .children(self.render_last_error(cx)),
                ActiveView::History => parent.child(self.history.clone()),
                ActiveView::PromptEditor {
                    context_editor,
                    buffer_search_bar,
                    ..
                } => {
                    let mut registrar = DivRegistrar::new(
                        |this, _, _cx| match &this.active_view {
                            ActiveView::PromptEditor {
                                buffer_search_bar, ..
                            } => Some(buffer_search_bar.clone()),
                            _ => None,
                        },
                        cx,
                    );
                    BufferSearchBar::register(&mut registrar);
                    parent.child(
                        registrar
                            .into_div()
                            .size_full()
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
                            .child(context_editor.clone()),
                    )
                }
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

impl rules_library::InlineAssistDelegate for PromptLibraryInlineAssist {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        _initial_prompt: Option<String>,
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
            assistant.assist(
                &prompt_editor,
                self.workspace.clone(),
                project,
                prompt_store,
                thread_store,
                window,
                cx,
            )
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
        panel.read(cx).active_context_editor()
    }

    fn open_saved_context(
        &self,
        workspace: &mut Workspace,
        path: Arc<Path>,
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
        workspace: &mut Workspace,
        selection_ranges: Vec<Range<Anchor>>,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        if !panel.focus_handle(cx).contains_focused(window, cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(window, cx);
        }

        panel.update(cx, |_, cx| {
            // Wait to create a new context until the workspace is no longer
            // being updated.
            cx.defer_in(window, move |panel, window, cx| {
                if panel.has_active_thread() {
                    panel.message_editor.update(cx, |message_editor, cx| {
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

const DISMISSED_TRIAL_UPSELL_KEY: &str = "dismissed-trial-upsell";

fn dismissed_trial_upsell() -> bool {
    db::kvp::KEY_VALUE_STORE
        .read_kvp(DISMISSED_TRIAL_UPSELL_KEY)
        .log_err()
        .map_or(false, |s| s.is_some())
}

fn set_trial_upsell_dismissed(is_dismissed: bool, cx: &mut App) {
    db::write_and_log(cx, move || async move {
        if is_dismissed {
            db::kvp::KEY_VALUE_STORE
                .write_kvp(DISMISSED_TRIAL_UPSELL_KEY.into(), "1".into())
                .await
        } else {
            db::kvp::KEY_VALUE_STORE
                .delete_kvp(DISMISSED_TRIAL_UPSELL_KEY.into())
                .await
        }
    })
}

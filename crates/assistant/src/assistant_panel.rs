use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    humanize_token_count,
    prompt_library::open_prompt_library,
    slash_command::{
        default_command::DefaultSlashCommand,
        docs_command::{DocsSlashCommand, DocsSlashCommandArgs},
        SlashCommandCompletionProvider, SlashCommandRegistry,
    },
    terminal_inline_assistant::TerminalInlineAssistant,
    Assist, ConfirmCommand, Context, ContextEvent, ContextId, ContextStore, CycleMessageRole,
    DebugEditSteps, DeployHistory, DeployPromptLibrary, EditStep, EditStepOperations,
    EditSuggestionGroup, InlineAssist, InlineAssistId, InlineAssistant, InsertIntoEditor,
    MessageStatus, ModelSelector, PendingSlashCommand, PendingSlashCommandStatus, QuoteSelection,
    RemoteContextMetadata, ResetKey, SavedContextMetadata, Split, ToggleFocus, ToggleModelSelector,
};
use anyhow::{anyhow, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutputSection};
use breadcrumbs::Breadcrumbs;
use client::proto;
use collections::{BTreeSet, HashMap, HashSet};
use completion::CompletionProvider;
use editor::{
    actions::{FoldAt, MoveToEndOfLine, Newline, ShowCompletions, UnfoldAt},
    display_map::{
        BlockDisposition, BlockProperties, BlockStyle, Crease, CustomBlockId, RenderBlock,
        ToDisplayPoint,
    },
    scroll::{Autoscroll, AutoscrollStrategy, ScrollAnchor},
    Anchor, Editor, EditorEvent, ExcerptRange, MultiBuffer, RowExt, ToOffset as _, ToPoint,
};
use editor::{display_map::CreaseId, FoldPlaceholder};
use fs::Fs;
use gpui::{
    div, percentage, point, Action, Animation, AnimationExt, AnyElement, AnyView, AppContext,
    AsyncWindowContext, ClipboardItem, Context as _, DismissEvent, Empty, Entity, EventEmitter,
    FocusHandle, FocusableView, InteractiveElement, IntoElement, Model, ParentElement, Pixels,
    Render, SharedString, StatefulInteractiveElement, Styled, Subscription, Task, Transformation,
    UpdateGlobal, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use indexed_docs::IndexedDocsStore;
use language::{
    language_settings::SoftWrap, Buffer, Capability, LanguageRegistry, LspAdapterDelegate, Point,
    ToOffset,
};
use language_model::Role;
use multi_buffer::MultiBufferRow;
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectLspAdapterDelegate};
use search::{buffer_search::DivRegistrar, BufferSearchBar};
use settings::Settings;
use std::{
    cmp::{self, Ordering},
    fmt::Write,
    ops::Range,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use theme::ThemeSettings;
use ui::{
    prelude::*,
    utils::{format_distance_from_now, DateTimeType},
    Avatar, AvatarShape, ButtonLike, ContextMenu, Disclosure, ElevationIndex, KeyBinding, ListItem,
    ListItemSpacing, PopoverMenu, PopoverMenuHandle, Tooltip,
};
use util::ResultExt;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::{self, BreadcrumbText, FollowableItem, Item, ItemHandle},
    notifications::NotifyTaskExt,
    pane::{self, SaveIntent},
    searchable::{SearchEvent, SearchableItem},
    Pane, Save, ToggleZoom, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};
use workspace::{searchable::SearchableItemHandle, NewFile};

pub fn init(cx: &mut AppContext) {
    workspace::FollowableViewRegistry::register::<ContextEditor>(cx);
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    let settings = AssistantSettings::get_global(cx);
                    if !settings.enabled {
                        return;
                    }

                    workspace.toggle_panel_focus::<AssistantPanel>(cx);
                })
                .register_action(AssistantPanel::inline_assist)
                .register_action(ContextEditor::quote_selection)
                .register_action(ContextEditor::insert_selection);
        },
    )
    .detach();
}

pub enum AssistantPanelEvent {
    ContextEdited,
}

pub struct AssistantPanel {
    pane: View<Pane>,
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    project: Model<Project>,
    context_store: Model<ContextStore>,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    subscriptions: Vec<Subscription>,
    authentication_prompt: Option<AnyView>,
    model_selector_menu_handle: PopoverMenuHandle<ContextMenu>,
}

#[derive(Clone)]
enum ContextMetadata {
    Remote(RemoteContextMetadata),
    Saved(SavedContextMetadata),
}

struct SavedContextPickerDelegate {
    store: Model<ContextStore>,
    project: Model<Project>,
    matches: Vec<ContextMetadata>,
    selected_index: usize,
}

enum SavedContextPickerEvent {
    Confirmed(ContextMetadata),
}

enum InlineAssistTarget {
    Editor(View<Editor>, bool),
    Terminal(View<TerminalView>),
}

impl EventEmitter<SavedContextPickerEvent> for Picker<SavedContextPickerDelegate> {}

impl SavedContextPickerDelegate {
    fn new(project: Model<Project>, store: Model<ContextStore>) -> Self {
        Self {
            project,
            store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for SavedContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search = self.store.read(cx).search(query, cx);
        cx.spawn(|this, mut cx| async move {
            let matches = search.await;
            this.update(&mut cx, |this, cx| {
                let host_contexts = this.delegate.store.read(cx).host_contexts();
                this.delegate.matches = host_contexts
                    .iter()
                    .cloned()
                    .map(ContextMetadata::Remote)
                    .chain(matches.into_iter().map(ContextMetadata::Saved))
                    .collect();
                this.delegate.selected_index = 0;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(metadata) = self.matches.get(self.selected_index) {
            cx.emit(SavedContextPickerEvent::Confirmed(metadata.clone()));
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let context = self.matches.get(ix)?;
        let item = match context {
            ContextMetadata::Remote(context) => {
                let host_user = self.project.read(cx).host().and_then(|collaborator| {
                    self.project
                        .read(cx)
                        .user_store()
                        .read(cx)
                        .get_cached_user(collaborator.user_id)
                });
                div()
                    .flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex().flex_1().overflow_x_hidden().child(
                            Label::new(context.summary.clone().unwrap_or("New Context".into()))
                                .size(LabelSize::Small),
                        ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .children(if let Some(host_user) = host_user {
                                vec![
                                    Avatar::new(host_user.avatar_uri.clone())
                                        .shape(AvatarShape::Circle)
                                        .into_any_element(),
                                    Label::new(format!("Shared by @{}", host_user.github_login))
                                        .color(Color::Muted)
                                        .size(LabelSize::Small)
                                        .into_any_element(),
                                ]
                            } else {
                                vec![Label::new("Shared by host")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small)
                                    .into_any_element()]
                            }),
                    )
            }
            ContextMetadata::Saved(context) => div()
                .flex()
                .w_full()
                .justify_between()
                .gap_2()
                .child(
                    h_flex()
                        .flex_1()
                        .child(Label::new(context.title.clone()).size(LabelSize::Small))
                        .overflow_x_hidden(),
                )
                .child(
                    Label::new(format_distance_from_now(
                        DateTimeType::Local(context.mtime),
                        false,
                        true,
                        true,
                    ))
                    .color(Color::Muted)
                    .size(LabelSize::Small),
                ),
        };
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(item),
        )
    }
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let context_store = workspace
                .update(&mut cx, |workspace, cx| {
                    ContextStore::new(workspace.project().clone(), cx)
                })?
                .await?;
            workspace.update(&mut cx, |workspace, cx| {
                // TODO: deserialize state.
                cx.new_view(|cx| Self::new(workspace, context_store, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        context_store: Model<ContextStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let model_selector_menu_handle = PopoverMenuHandle::default();
        let pane = cx.new_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                None,
                NewFile.boxed_clone(),
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(true, cx);
            pane.display_nav_history_buttons(None);
            pane.set_should_display_tab_bar(|_| true);
            pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                h_flex()
                    .gap(Spacing::Small.rems(cx))
                    .child(
                        IconButton::new("menu", IconName::Menu)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|pane, _, cx| {
                                let zoom_label = if pane.is_zoomed() {
                                    "Zoom Out"
                                } else {
                                    "Zoom In"
                                };
                                let menu = ContextMenu::build(cx, |menu, cx| {
                                    menu.context(pane.focus_handle(cx))
                                        .action("New Context", Box::new(NewFile))
                                        .action("History", Box::new(DeployHistory))
                                        .action("Prompt Library", Box::new(DeployPromptLibrary))
                                        .action(zoom_label, Box::new(ToggleZoom))
                                });
                                cx.subscribe(&menu, |pane, _, _: &DismissEvent, _| {
                                    pane.new_item_menu = None;
                                })
                                .detach();
                                pane.new_item_menu = Some(menu);
                            })),
                    )
                    .when_some(pane.new_item_menu.as_ref(), |el, new_item_menu| {
                        el.child(Pane::render_menu_overlay(new_item_menu))
                    })
                    .into_any_element()
            });
            pane.toolbar().update(cx, |toolbar, cx| {
                toolbar.add_item(cx.new_view(|_| Breadcrumbs::new()), cx);
                toolbar.add_item(
                    cx.new_view(|_| {
                        ContextEditorToolbarItem::new(workspace, model_selector_menu_handle.clone())
                    }),
                    cx,
                );
                toolbar.add_item(cx.new_view(BufferSearchBar::new), cx)
            });
            pane
        });

        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe(&pane, Self::handle_pane_event),
            cx.observe_global::<CompletionProvider>({
                let mut prev_settings_version = CompletionProvider::global(cx).settings_version();
                move |this, cx| {
                    this.completion_provider_changed(prev_settings_version, cx);
                    prev_settings_version = CompletionProvider::global(cx).settings_version();
                }
            }),
        ];

        Self {
            pane,
            workspace: workspace.weak_handle(),
            width: None,
            height: None,
            project: workspace.project().clone(),
            context_store,
            languages: workspace.app_state().languages.clone(),
            fs: workspace.app_state().fs.clone(),
            subscriptions,
            authentication_prompt: None,
            model_selector_menu_handle,
        }
    }

    fn handle_pane_event(
        &mut self,
        pane: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::Remove => cx.emit(PanelEvent::Close),
            pane::Event::ZoomIn => cx.emit(PanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(PanelEvent::ZoomOut),

            pane::Event::AddItem { item } => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, self.pane.clone(), cx)
                    })
                    .ok();
            }

            pane::Event::ActivateItem { local } => {
                if *local {
                    self.workspace
                        .update(cx, |workspace, cx| {
                            workspace.unfollow_in_pane(&pane, cx);
                        })
                        .ok();
                }
                cx.emit(AssistantPanelEvent::ContextEdited);
            }

            pane::Event::RemoveItem { .. } => {
                cx.emit(AssistantPanelEvent::ContextEdited);
            }

            _ => {}
        }
    }

    fn completion_provider_changed(
        &mut self,
        prev_settings_version: usize,
        cx: &mut ViewContext<Self>,
    ) {
        if self.is_authenticated(cx) {
            self.authentication_prompt = None;

            if let Some(editor) = self.active_context_editor(cx) {
                editor.update(cx, |active_context, cx| {
                    active_context
                        .context
                        .update(cx, |context, cx| context.completion_provider_changed(cx))
                })
            }

            if self.active_context_editor(cx).is_none() {
                self.new_context(cx);
            }
            cx.notify();
        } else if self.authentication_prompt.is_none()
            || prev_settings_version != CompletionProvider::global(cx).settings_version()
        {
            self.authentication_prompt =
                Some(cx.update_global::<CompletionProvider, _>(|provider, cx| {
                    provider.authentication_prompt(cx)
                }));
            cx.notify();
        }
    }

    pub fn inline_assist(
        workspace: &mut Workspace,
        action: &InlineAssist,
        cx: &mut ViewContext<Workspace>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        let Some(assistant_panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        let Some(inline_assist_target) =
            Self::resolve_inline_assist_target(workspace, &assistant_panel, cx)
        else {
            return;
        };

        let initial_prompt = action.prompt.clone();
        if assistant_panel.update(cx, |assistant, cx| assistant.is_authenticated(cx)) {
            match inline_assist_target {
                InlineAssistTarget::Editor(active_editor, include_context) => {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_editor,
                            Some(cx.view().downgrade()),
                            include_context.then_some(&assistant_panel),
                            initial_prompt,
                            cx,
                        )
                    })
                }
                InlineAssistTarget::Terminal(active_terminal) => {
                    TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_terminal,
                            Some(cx.view().downgrade()),
                            Some(&assistant_panel),
                            initial_prompt,
                            cx,
                        )
                    })
                }
            }
        } else {
            let assistant_panel = assistant_panel.downgrade();
            cx.spawn(|workspace, mut cx| async move {
                assistant_panel
                    .update(&mut cx, |assistant, cx| assistant.authenticate(cx))?
                    .await?;
                if assistant_panel.update(&mut cx, |panel, cx| panel.is_authenticated(cx))? {
                    cx.update(|cx| match inline_assist_target {
                        InlineAssistTarget::Editor(active_editor, include_context) => {
                            let assistant_panel = if include_context {
                                assistant_panel.upgrade()
                            } else {
                                None
                            };
                            InlineAssistant::update_global(cx, |assistant, cx| {
                                assistant.assist(
                                    &active_editor,
                                    Some(workspace),
                                    assistant_panel.as_ref(),
                                    initial_prompt,
                                    cx,
                                )
                            })
                        }
                        InlineAssistTarget::Terminal(active_terminal) => {
                            TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                                assistant.assist(
                                    &active_terminal,
                                    Some(workspace),
                                    assistant_panel.upgrade().as_ref(),
                                    initial_prompt,
                                    cx,
                                )
                            })
                        }
                    })?
                } else {
                    workspace.update(&mut cx, |workspace, cx| {
                        workspace.focus_panel::<AssistantPanel>(cx)
                    })?;
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx)
        }
    }

    fn resolve_inline_assist_target(
        workspace: &mut Workspace,
        assistant_panel: &View<AssistantPanel>,
        cx: &mut WindowContext,
    ) -> Option<InlineAssistTarget> {
        if let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx) {
            if terminal_panel
                .read(cx)
                .focus_handle(cx)
                .contains_focused(cx)
            {
                use feature_flags::FeatureFlagAppExt;
                if !cx.has_flag::<feature_flags::TerminalInlineAssist>() {
                    return None;
                }

                if let Some(terminal_view) = terminal_panel
                    .read(cx)
                    .pane()
                    .read(cx)
                    .active_item()
                    .and_then(|t| t.downcast::<TerminalView>())
                {
                    return Some(InlineAssistTarget::Terminal(terminal_view));
                }
            }
        }
        let context_editor =
            assistant_panel
                .read(cx)
                .active_context_editor(cx)
                .and_then(|editor| {
                    let editor = &editor.read(cx).editor;
                    if editor.read(cx).is_focused(cx) {
                        Some(editor.clone())
                    } else {
                        None
                    }
                });

        if let Some(context_editor) = context_editor {
            Some(InlineAssistTarget::Editor(context_editor, false))
        } else if let Some(workspace_editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            Some(InlineAssistTarget::Editor(workspace_editor, true))
        } else {
            None
        }
    }

    fn new_context(&mut self, cx: &mut ViewContext<Self>) -> Option<View<ContextEditor>> {
        let context = self.context_store.update(cx, |store, cx| store.create(cx));
        let workspace = self.workspace.upgrade()?;
        let lsp_adapter_delegate = workspace.update(cx, |workspace, cx| {
            make_lsp_adapter_delegate(workspace.project(), cx).log_err()
        });

        let assistant_panel = cx.view().downgrade();
        let editor = cx.new_view(|cx| {
            let mut editor = ContextEditor::for_context(
                context,
                self.fs.clone(),
                workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                assistant_panel,
                cx,
            );
            editor.insert_default_prompt(cx);
            editor
        });

        self.show_context(editor.clone(), cx);
        Some(editor)
    }

    fn show_context(&mut self, context_editor: View<ContextEditor>, cx: &mut ViewContext<Self>) {
        let focus = self.focus_handle(cx).contains_focused(cx);
        let prev_len = self.pane.read(cx).items_len();
        self.pane.update(cx, |pane, cx| {
            pane.add_item(Box::new(context_editor.clone()), focus, focus, None, cx)
        });

        if prev_len != self.pane.read(cx).items_len() {
            self.subscriptions
                .push(cx.subscribe(&context_editor, Self::handle_context_editor_event));
        }

        cx.emit(AssistantPanelEvent::ContextEdited);
        cx.notify();
    }

    fn handle_context_editor_event(
        &mut self,
        _: View<ContextEditor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::TitleChanged { .. } => cx.notify(),
            EditorEvent::Edited { .. } => cx.emit(AssistantPanelEvent::ContextEdited),
            _ => {}
        }
    }

    fn deploy_history(&mut self, _: &DeployHistory, cx: &mut ViewContext<Self>) {
        let history_item_ix = self
            .pane
            .read(cx)
            .items()
            .position(|item| item.downcast::<ContextHistory>().is_some());

        if let Some(history_item_ix) = history_item_ix {
            self.pane.update(cx, |pane, cx| {
                pane.activate_item(history_item_ix, true, true, cx);
            });
        } else {
            let assistant_panel = cx.view().downgrade();
            let history = cx.new_view(|cx| {
                ContextHistory::new(
                    self.project.clone(),
                    self.context_store.clone(),
                    assistant_panel,
                    cx,
                )
            });
            self.pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(history), true, true, None, cx);
            });
        }
    }

    fn deploy_prompt_library(&mut self, _: &DeployPromptLibrary, cx: &mut ViewContext<Self>) {
        open_prompt_library(self.languages.clone(), cx).detach_and_log_err(cx);
    }

    fn reset_credentials(&mut self, _: &ResetKey, cx: &mut ViewContext<Self>) {
        CompletionProvider::global(cx)
            .reset_credentials(cx)
            .detach_and_log_err(cx);
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_selector_menu_handle.toggle(cx);
    }

    fn active_context_editor(&self, cx: &AppContext) -> Option<View<ContextEditor>> {
        self.pane
            .read(cx)
            .active_item()?
            .downcast::<ContextEditor>()
    }

    pub fn active_context(&self, cx: &AppContext) -> Option<Model<Context>> {
        Some(self.active_context_editor(cx)?.read(cx).context.clone())
    }

    fn open_saved_context(
        &mut self,
        path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let existing_context = self.pane.read(cx).items().find_map(|item| {
            item.downcast::<ContextEditor>()
                .filter(|editor| editor.read(cx).context.read(cx).path() == Some(&path))
        });
        if let Some(existing_context) = existing_context {
            return cx.spawn(|this, mut cx| async move {
                this.update(&mut cx, |this, cx| this.show_context(existing_context, cx))
            });
        }

        let context = self
            .context_store
            .update(cx, |store, cx| store.open_local_context(path.clone(), cx));
        let fs = self.fs.clone();
        let project = self.project.clone();
        let workspace = self.workspace.clone();

        let lsp_adapter_delegate = workspace
            .update(cx, |workspace, cx| {
                make_lsp_adapter_delegate(workspace.project(), cx).log_err()
            })
            .log_err()
            .flatten();

        cx.spawn(|this, mut cx| async move {
            let context = context.await?;
            let assistant_panel = this.clone();
            this.update(&mut cx, |this, cx| {
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow!("workspace dropped"))?;
                let editor = cx.new_view(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        project,
                        lsp_adapter_delegate,
                        assistant_panel,
                        cx,
                    )
                });
                this.show_context(editor, cx);
                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    fn open_remote_context(
        &mut self,
        id: ContextId,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<View<ContextEditor>>> {
        let existing_context = self.pane.read(cx).items().find_map(|item| {
            item.downcast::<ContextEditor>()
                .filter(|editor| *editor.read(cx).context.read(cx).id() == id)
        });
        if let Some(existing_context) = existing_context {
            return cx.spawn(|this, mut cx| async move {
                this.update(&mut cx, |this, cx| {
                    this.show_context(existing_context.clone(), cx)
                })?;
                Ok(existing_context)
            });
        }

        let context = self
            .context_store
            .update(cx, |store, cx| store.open_remote_context(id, cx));
        let fs = self.fs.clone();
        let workspace = self.workspace.clone();

        let lsp_adapter_delegate = workspace
            .update(cx, |workspace, cx| {
                make_lsp_adapter_delegate(workspace.project(), cx).log_err()
            })
            .log_err()
            .flatten();

        cx.spawn(|this, mut cx| async move {
            let context = context.await?;
            let assistant_panel = this.clone();
            this.update(&mut cx, |this, cx| {
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow!("workspace dropped"))?;
                let editor = cx.new_view(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        this.project.clone(),
                        lsp_adapter_delegate,
                        assistant_panel,
                        cx,
                    )
                });
                this.show_context(editor.clone(), cx);
                anyhow::Ok(editor)
            })?
        })
    }

    fn is_authenticated(&mut self, cx: &mut ViewContext<Self>) -> bool {
        CompletionProvider::global(cx).is_authenticated()
    }

    fn authenticate(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        cx.update_global::<CompletionProvider, _>(|provider, cx| provider.authenticate(cx))
    }

    fn render_signed_in(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut registrar = DivRegistrar::new(
            |panel, cx| {
                panel
                    .pane
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
            },
            cx,
        );
        BufferSearchBar::register(&mut registrar);
        let registrar = registrar.into_div();

        v_flex()
            .key_context("AssistantPanel")
            .size_full()
            .on_action(cx.listener(|this, _: &workspace::NewFile, cx| {
                this.new_context(cx);
            }))
            .on_action(cx.listener(AssistantPanel::deploy_history))
            .on_action(cx.listener(AssistantPanel::deploy_prompt_library))
            .on_action(cx.listener(AssistantPanel::reset_credentials))
            .on_action(cx.listener(AssistantPanel::toggle_model_selector))
            .child(registrar.size_full().child(self.pane.clone()))
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if let Some(authentication_prompt) = self.authentication_prompt.as_ref() {
            authentication_prompt.clone().into_any()
        } else {
            self.render_signed_in(cx).into_any_element()
        }
    }
}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match AssistantSettings::get_global(cx).dock {
            AssistantDockPosition::Left => DockPosition::Left,
            AssistantDockPosition::Bottom => DockPosition::Bottom,
            AssistantDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<AssistantSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                DockPosition::Left => AssistantDockPosition::Left,
                DockPosition::Bottom => AssistantDockPosition::Bottom,
                DockPosition::Right => AssistantDockPosition::Right,
            };
            settings.set_dock(dock);
        });
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

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            let load_credentials = self.authenticate(cx);
            cx.spawn(|this, mut cx| async move {
                load_credentials.await?;
                this.update(&mut cx, |this, cx| {
                    if this.is_authenticated(cx) && this.active_context_editor(cx).is_none() {
                        this.new_context(cx);
                    }
                })
            })
            .detach_and_log_err(cx);
        }
    }

    fn pane(&self) -> Option<View<Pane>> {
        Some(self.pane.clone())
    }

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
}

impl EventEmitter<PanelEvent> for AssistantPanel {}
impl EventEmitter<AssistantPanelEvent> for AssistantPanel {}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

pub enum ContextEditorEvent {
    Edited,
    TabContentChanged,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: gpui::Point<f32>,
    cursor: Anchor,
}

struct ActiveEditStep {
    start: language::Anchor,
    assist_ids: Vec<InlineAssistId>,
    editor: Option<WeakView<Editor>>,
    _open_editor: Task<Result<()>>,
}

pub struct ContextEditor {
    context: Model<Context>,
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
    editor: View<Editor>,
    blocks: HashSet<CustomBlockId>,
    scroll_position: Option<ScrollPosition>,
    remote_id: Option<workspace::ViewId>,
    pending_slash_command_creases: HashMap<Range<language::Anchor>, CreaseId>,
    pending_slash_command_blocks: HashMap<Range<language::Anchor>, CustomBlockId>,
    _subscriptions: Vec<Subscription>,
    active_edit_step: Option<ActiveEditStep>,
    assistant_panel: WeakView<AssistantPanel>,
}

impl ContextEditor {
    const MAX_TAB_TITLE_LEN: usize = 16;

    fn for_context(
        context: Model<Context>,
        fs: Arc<dyn Fs>,
        workspace: View<Workspace>,
        project: Model<Project>,
        lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
        assistant_panel: WeakView<AssistantPanel>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let completion_provider = SlashCommandCompletionProvider::new(
            Some(cx.view().downgrade()),
            Some(workspace.downgrade()),
        );

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(context.read(cx).buffer().clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Box::new(completion_provider));
            editor.set_collaboration_hub(Box::new(project.clone()));
            editor
        });

        let _subscriptions = vec![
            cx.observe(&context, |_, _, cx| cx.notify()),
            cx.subscribe(&context, Self::handle_context_event),
            cx.subscribe(&editor, Self::handle_editor_event),
            cx.subscribe(&editor, Self::handle_editor_search_event),
        ];

        let sections = context.read(cx).slash_command_output_sections().to_vec();
        let mut this = Self {
            context,
            editor,
            lsp_adapter_delegate,
            blocks: Default::default(),
            scroll_position: None,
            remote_id: None,
            fs,
            workspace: workspace.downgrade(),
            project,
            pending_slash_command_creases: HashMap::default(),
            pending_slash_command_blocks: HashMap::default(),
            _subscriptions,
            active_edit_step: None,
            assistant_panel,
        };
        this.update_message_headers(cx);
        this.insert_slash_command_output_sections(sections, cx);
        this
    }

    fn insert_default_prompt(&mut self, cx: &mut ViewContext<Self>) {
        let command_name = DefaultSlashCommand.name();
        self.editor.update(cx, |editor, cx| {
            editor.insert(&format!("/{command_name}"), cx)
        });
        self.split(&Split, cx);
        let command = self.context.update(cx, |context, cx| {
            let first_message_id = context.messages(cx).next().unwrap().id;
            context.update_metadata(first_message_id, cx, |metadata| {
                metadata.role = Role::System;
            });
            context.reparse_slash_commands(cx);
            context.pending_slash_commands()[0].clone()
        });

        self.run_command(
            command.source_range,
            &command.name,
            command.argument.as_deref(),
            false,
            self.workspace.clone(),
            cx,
        );
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        if !self.apply_edit_step(cx) {
            self.send_to_model(cx);
        }
    }

    fn apply_edit_step(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if let Some(step) = self.active_edit_step.as_ref() {
            InlineAssistant::update_global(cx, |assistant, cx| {
                for assist_id in &step.assist_ids {
                    assistant.start_assist(*assist_id, cx);
                }
                !step.assist_ids.is_empty()
            })
        } else {
            false
        }
    }

    fn send_to_model(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(user_message) = self.context.update(cx, |context, cx| context.assist(cx)) {
            let new_selection = {
                let cursor = user_message
                    .start
                    .to_offset(self.context.read(cx).buffer().read(cx));
                cursor..cursor
            };
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Fit)),
                    cx,
                    |selections| selections.select_ranges([new_selection]),
                );
            });
            // Avoid scrolling to the new cursor position so the assistant's output is stable.
            cx.defer(|this, _| this.scroll_position = None);
        }
    }

    fn cancel_last_assist(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        if !self
            .context
            .update(cx, |context, _| context.cancel_last_assist())
        {
            cx.propagate();
        }
    }

    fn debug_edit_steps(&mut self, _: &DebugEditSteps, cx: &mut ViewContext<Self>) {
        let mut output = String::new();
        for (i, step) in self.context.read(cx).edit_steps().iter().enumerate() {
            output.push_str(&format!("Step {}:\n", i + 1));
            output.push_str(&format!(
                "Content: {}\n",
                self.context
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .text_for_range(step.source_range.clone())
                    .collect::<String>()
            ));
            match &step.operations {
                Some(EditStepOperations::Parsed {
                    operations,
                    raw_output,
                }) => {
                    output.push_str(&format!("Raw Output:\n{raw_output}\n"));
                    output.push_str("Parsed Operations:\n");
                    for op in operations {
                        output.push_str(&format!("  {:?}\n", op));
                    }
                }
                Some(EditStepOperations::Pending(_)) => {
                    output.push_str("Operations: Pending\n");
                }
                None => {
                    output.push_str("Operations: None\n");
                }
            }
            output.push('\n');
        }

        let editor = self
            .workspace
            .update(cx, |workspace, cx| Editor::new_in_workspace(workspace, cx));

        if let Ok(editor) = editor {
            cx.spawn(|_, mut cx| async move {
                let editor = editor.await?;
                editor.update(&mut cx, |editor, cx| editor.set_text(output, cx))
            })
            .detach_and_notify_err(cx);
        }
    }

    fn cycle_message_role(&mut self, _: &CycleMessageRole, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);
        self.context.update(cx, |context, cx| {
            let messages = context
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            context.cycle_message_roles(messages, cx)
        });
    }

    fn cursors(&self, cx: &AppContext) -> Vec<usize> {
        let selections = self.editor.read(cx).selections.all::<usize>(cx);
        selections
            .into_iter()
            .map(|selection| selection.head())
            .collect()
    }

    fn insert_command(&mut self, name: &str, cx: &mut ViewContext<Self>) {
        if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
            self.editor.update(cx, |editor, cx| {
                editor.transact(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| s.try_cancel());
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let newest_cursor = editor.selections.newest::<Point>(cx).head();
                    if newest_cursor.column > 0
                        || snapshot
                            .chars_at(newest_cursor)
                            .next()
                            .map_or(false, |ch| ch != '\n')
                    {
                        editor.move_to_end_of_line(
                            &MoveToEndOfLine {
                                stop_at_soft_wraps: false,
                            },
                            cx,
                        );
                        editor.newline(&Newline, cx);
                    }

                    editor.insert(&format!("/{name}"), cx);
                    if command.requires_argument() {
                        editor.insert(" ", cx);
                        editor.show_completions(&ShowCompletions::default(), cx);
                    }
                });
            });
            if !command.requires_argument() {
                self.confirm_command(&ConfirmCommand, cx);
            }
        }
    }

    pub fn confirm_command(&mut self, _: &ConfirmCommand, cx: &mut ViewContext<Self>) {
        let selections = self.editor.read(cx).selections.disjoint_anchors();
        let mut commands_by_range = HashMap::default();
        let workspace = self.workspace.clone();
        self.context.update(cx, |context, cx| {
            context.reparse_slash_commands(cx);
            for selection in selections.iter() {
                if let Some(command) =
                    context.pending_command_for_position(selection.head().text_anchor, cx)
                {
                    commands_by_range
                        .entry(command.source_range.clone())
                        .or_insert_with(|| command.clone());
                }
            }
        });

        if commands_by_range.is_empty() {
            cx.propagate();
        } else {
            for command in commands_by_range.into_values() {
                self.run_command(
                    command.source_range,
                    &command.name,
                    command.argument.as_deref(),
                    true,
                    workspace.clone(),
                    cx,
                );
            }
            cx.stop_propagation();
        }
    }

    pub fn run_command(
        &mut self,
        command_range: Range<language::Anchor>,
        name: &str,
        argument: Option<&str>,
        insert_trailing_newline: bool,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
            if let Some(lsp_adapter_delegate) = self.lsp_adapter_delegate.clone() {
                let argument = argument.map(ToString::to_string);
                let output = command.run(argument.as_deref(), workspace, lsp_adapter_delegate, cx);
                self.context.update(cx, |context, cx| {
                    context.insert_command_output(
                        command_range,
                        output,
                        insert_trailing_newline,
                        cx,
                    )
                });
            }
        }
    }

    fn handle_context_event(
        &mut self,
        _: Model<Context>,
        event: &ContextEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let context_editor = cx.view().downgrade();

        match event {
            ContextEvent::MessagesEdited => {
                self.update_message_headers(cx);
                self.context.update(cx, |context, cx| {
                    context.save(Some(Duration::from_millis(500)), self.fs.clone(), cx);
                });
            }
            ContextEvent::EditStepsChanged => {
                cx.notify();
            }
            ContextEvent::SummaryChanged => {
                cx.emit(EditorEvent::TitleChanged);
                self.context.update(cx, |context, cx| {
                    context.save(None, self.fs.clone(), cx);
                });
            }
            ContextEvent::StreamedCompletion => {
                self.editor.update(cx, |editor, cx| {
                    if let Some(scroll_position) = self.scroll_position {
                        let snapshot = editor.snapshot(cx);
                        let cursor_point = scroll_position.cursor.to_display_point(&snapshot);
                        let scroll_top =
                            cursor_point.row().as_f32() - scroll_position.offset_before_cursor.y;
                        editor.set_scroll_position(
                            point(scroll_position.offset_before_cursor.x, scroll_top),
                            cx,
                        );
                    }
                });
            }
            ContextEvent::PendingSlashCommandsUpdated { removed, updated } => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (excerpt_id, buffer_id, _) = buffer.as_singleton().unwrap();
                    let excerpt_id = *excerpt_id;

                    editor.remove_creases(
                        removed
                            .iter()
                            .filter_map(|range| self.pending_slash_command_creases.remove(range)),
                        cx,
                    );

                    editor.remove_blocks(
                        HashSet::from_iter(
                            removed.iter().filter_map(|range| {
                                self.pending_slash_command_blocks.remove(range)
                            }),
                        ),
                        None,
                        cx,
                    );

                    let crease_ids = editor.insert_creases(
                        updated.iter().map(|command| {
                            let workspace = self.workspace.clone();
                            let confirm_command = Arc::new({
                                let context_editor = context_editor.clone();
                                let command = command.clone();
                                move |cx: &mut WindowContext| {
                                    context_editor
                                        .update(cx, |context_editor, cx| {
                                            context_editor.run_command(
                                                command.source_range.clone(),
                                                &command.name,
                                                command.argument.as_deref(),
                                                false,
                                                workspace.clone(),
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            });
                            let placeholder = FoldPlaceholder {
                                render: Arc::new(move |_, _, _| Empty.into_any()),
                                constrain_width: false,
                                merge_adjacent: false,
                            };
                            let render_toggle = {
                                let confirm_command = confirm_command.clone();
                                let command = command.clone();
                                move |row, _, _, _cx: &mut WindowContext| {
                                    render_pending_slash_command_gutter_decoration(
                                        row,
                                        &command.status,
                                        confirm_command.clone(),
                                    )
                                }
                            };
                            let render_trailer = {
                                let command = command.clone();
                                move |row, _unfold, cx: &mut WindowContext| {
                                    // TODO: In the future we should investigate how we can expose
                                    // this as a hook on the `SlashCommand` trait so that we don't
                                    // need to special-case it here.
                                    if command.name == DocsSlashCommand::NAME {
                                        return render_docs_slash_command_trailer(
                                            row,
                                            command.clone(),
                                            cx,
                                        );
                                    }

                                    Empty.into_any()
                                }
                            };

                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.end)
                                .unwrap();
                            Crease::new(start..end, placeholder, render_toggle, render_trailer)
                        }),
                        cx,
                    );

                    let block_ids = editor.insert_blocks(
                        updated
                            .iter()
                            .filter_map(|command| match &command.status {
                                PendingSlashCommandStatus::Error(error) => {
                                    Some((command, error.clone()))
                                }
                                _ => None,
                            })
                            .map(|(command, error_message)| BlockProperties {
                                style: BlockStyle::Fixed,
                                position: Anchor {
                                    buffer_id: Some(buffer_id),
                                    excerpt_id,
                                    text_anchor: command.source_range.start,
                                },
                                height: 1,
                                disposition: BlockDisposition::Below,
                                render: slash_command_error_block_renderer(error_message),
                            }),
                        None,
                        cx,
                    );

                    self.pending_slash_command_creases.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(crease_ids),
                    );

                    self.pending_slash_command_blocks.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(block_ids),
                    );
                })
            }
            ContextEvent::SlashCommandFinished {
                output_range,
                sections,
                run_commands_in_output,
            } => {
                self.insert_slash_command_output_sections(sections.iter().cloned(), cx);

                if *run_commands_in_output {
                    let commands = self.context.update(cx, |context, cx| {
                        context.reparse_slash_commands(cx);
                        context
                            .pending_commands_for_range(output_range.clone(), cx)
                            .to_vec()
                    });

                    for command in commands {
                        self.run_command(
                            command.source_range,
                            &command.name,
                            command.argument.as_deref(),
                            false,
                            self.workspace.clone(),
                            cx,
                        );
                    }
                }
            }
            ContextEvent::Operation(_) => {}
        }
    }

    fn insert_slash_command_output_sections(
        &mut self,
        sections: impl IntoIterator<Item = SlashCommandOutputSection<language::Anchor>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let mut buffer_rows_to_fold = BTreeSet::new();
            let mut creases = Vec::new();
            for section in sections {
                let start = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.start)
                    .unwrap();
                let end = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.end)
                    .unwrap();
                let buffer_row = MultiBufferRow(start.to_point(&buffer).row);
                buffer_rows_to_fold.insert(buffer_row);
                creases.push(Crease::new(
                    start..end,
                    FoldPlaceholder {
                        render: Arc::new({
                            let editor = cx.view().downgrade();
                            let icon = section.icon;
                            let label = section.label.clone();
                            move |fold_id, fold_range, _cx| {
                                let editor = editor.clone();
                                ButtonLike::new(fold_id)
                                    .style(ButtonStyle::Filled)
                                    .layer(ElevationIndex::ElevatedSurface)
                                    .child(Icon::new(icon))
                                    .child(Label::new(label.clone()).single_line())
                                    .on_click(move |_, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                let buffer_start = fold_range
                                                    .start
                                                    .to_point(&editor.buffer().read(cx).read(cx));
                                                let buffer_row = MultiBufferRow(buffer_start.row);
                                                editor.unfold_at(&UnfoldAt { buffer_row }, cx);
                                            })
                                            .ok();
                                    })
                                    .into_any_element()
                            }
                        }),
                        constrain_width: false,
                        merge_adjacent: false,
                    },
                    render_slash_command_output_toggle,
                    |_, _, _| Empty.into_any_element(),
                ));
            }

            editor.insert_creases(creases, cx);

            for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                editor.fold_at(&FoldAt { buffer_row }, cx);
            }
        });
    }

    fn handle_editor_event(
        &mut self,
        _: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::ScrollPositionChanged { autoscroll, .. } => {
                let cursor_scroll_position = self.cursor_scroll_position(cx);
                if *autoscroll {
                    self.scroll_position = cursor_scroll_position;
                } else if self.scroll_position != cursor_scroll_position {
                    self.scroll_position = None;
                }
            }
            EditorEvent::SelectionsChanged { .. } => {
                self.scroll_position = self.cursor_scroll_position(cx);
                if self
                    .edit_step_for_cursor(cx)
                    .map(|step| step.source_range.start)
                    != self.active_edit_step.as_ref().map(|step| step.start)
                {
                    if let Some(old_active_edit_step) = self.active_edit_step.take() {
                        if let Some(editor) = old_active_edit_step
                            .editor
                            .and_then(|editor| editor.upgrade())
                        {
                            self.workspace
                                .update(cx, |workspace, cx| {
                                    if let Some(pane) = workspace.pane_for(&editor) {
                                        pane.update(cx, |pane, cx| {
                                            let item_id = editor.entity_id();
                                            if pane.is_active_preview_item(item_id) {
                                                pane.close_item_by_id(
                                                    item_id,
                                                    SaveIntent::Skip,
                                                    cx,
                                                )
                                                .detach_and_log_err(cx);
                                            }
                                        });
                                    }
                                })
                                .ok();
                        }
                    }

                    if let Some(new_active_step) = self.edit_step_for_cursor(cx) {
                        let suggestions = new_active_step.edit_suggestions(&self.project, cx);
                        self.active_edit_step = Some(ActiveEditStep {
                            start: new_active_step.source_range.start,
                            assist_ids: Vec::new(),
                            editor: None,
                            _open_editor: self.open_editor_for_edit_suggestions(suggestions, cx),
                        });
                    }
                }
            }
            _ => {}
        }
        cx.emit(event.clone());
    }

    fn open_editor_for_edit_suggestions(
        &mut self,
        edit_suggestions: Task<HashMap<Model<Buffer>, Vec<EditSuggestionGroup>>>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let assistant_panel = self.assistant_panel.clone();
        cx.spawn(|this, mut cx| async move {
            let edit_suggestions = edit_suggestions.await;

            let mut assist_ids = Vec::new();
            let editor = if edit_suggestions.is_empty() {
                return Ok(());
            } else if edit_suggestions.len() == 1
                && edit_suggestions.values().next().unwrap().len() == 1
            {
                // If there's only one buffer and one suggestion group, open it directly
                let (buffer, suggestion_groups) = edit_suggestions.into_iter().next().unwrap();
                let suggestion_group = suggestion_groups.into_iter().next().unwrap();
                let editor = workspace.update(&mut cx, |workspace, cx| {
                    let active_pane = workspace.active_pane().clone();
                    workspace.open_project_item::<Editor>(active_pane, buffer, false, false, cx)
                })?;

                cx.update(|cx| {
                    for suggestion in suggestion_group.suggestions {
                        let description = suggestion.description.unwrap_or_else(|| "Delete".into());
                        let range = {
                            let buffer = editor.read(cx).buffer().read(cx).read(cx);
                            let (&excerpt_id, _, _) = buffer.as_singleton().unwrap();
                            buffer
                                .anchor_in_excerpt(excerpt_id, suggestion.range.start)
                                .unwrap()
                                ..buffer
                                    .anchor_in_excerpt(excerpt_id, suggestion.range.end)
                                    .unwrap()
                        };
                        let initial_text = suggestion.prepend_newline.then(|| "\n".into());
                        InlineAssistant::update_global(cx, |assistant, cx| {
                            assist_ids.push(assistant.suggest_assist(
                                &editor,
                                range,
                                description,
                                initial_text,
                                Some(workspace.clone()),
                                assistant_panel.upgrade().as_ref(),
                                cx,
                            ));
                        });
                    }

                    // Scroll the editor to the suggested assist
                    editor.update(cx, |editor, cx| {
                        let anchor = {
                            let buffer = editor.buffer().read(cx).read(cx);
                            let (&excerpt_id, _, _) = buffer.as_singleton().unwrap();
                            buffer
                                .anchor_in_excerpt(excerpt_id, suggestion_group.context_range.start)
                                .unwrap()
                        };

                        editor.set_scroll_anchor(
                            ScrollAnchor {
                                offset: gpui::Point::default(),
                                anchor,
                            },
                            cx,
                        );
                    });
                })?;

                editor
            } else {
                // If there are multiple buffers or suggestion groups, create a multibuffer
                let mut inline_assist_suggestions = Vec::new();
                let multibuffer = cx.new_model(|cx| {
                    let replica_id = project.read(cx).replica_id();
                    let mut multibuffer = MultiBuffer::new(replica_id, Capability::ReadWrite);
                    for (buffer, suggestion_groups) in edit_suggestions {
                        let excerpt_ids = multibuffer.push_excerpts(
                            buffer,
                            suggestion_groups
                                .iter()
                                .map(|suggestion_group| ExcerptRange {
                                    context: suggestion_group.context_range.clone(),
                                    primary: None,
                                }),
                            cx,
                        );

                        for (excerpt_id, suggestion_group) in
                            excerpt_ids.into_iter().zip(suggestion_groups)
                        {
                            for suggestion in suggestion_group.suggestions {
                                let description =
                                    suggestion.description.unwrap_or_else(|| "Delete".into());
                                let range = {
                                    let multibuffer = multibuffer.read(cx);
                                    multibuffer
                                        .anchor_in_excerpt(excerpt_id, suggestion.range.start)
                                        .unwrap()
                                        ..multibuffer
                                            .anchor_in_excerpt(excerpt_id, suggestion.range.end)
                                            .unwrap()
                                };
                                let initial_text =
                                    suggestion.prepend_newline.then(|| "\n".to_string());
                                inline_assist_suggestions.push((range, description, initial_text));
                            }
                        }
                    }
                    multibuffer
                })?;

                let editor = cx
                    .new_view(|cx| Editor::for_multibuffer(multibuffer, Some(project), true, cx))?;
                cx.update(|cx| {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        for (range, description, initial_text) in inline_assist_suggestions {
                            assist_ids.push(assistant.suggest_assist(
                                &editor,
                                range,
                                description,
                                initial_text,
                                Some(workspace.clone()),
                                assistant_panel.upgrade().as_ref(),
                                cx,
                            ));
                        }
                    })
                })?;
                workspace.update(&mut cx, |workspace, cx| {
                    workspace.add_item_to_active_pane(Box::new(editor.clone()), None, false, cx)
                })?;

                editor
            };

            this.update(&mut cx, |this, _cx| {
                if let Some(step) = this.active_edit_step.as_mut() {
                    step.assist_ids = assist_ids;
                    step.editor = Some(editor.downgrade());
                }
            })
        })
    }

    fn handle_editor_search_event(
        &mut self,
        _: View<Editor>,
        event: &SearchEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(event.clone());
    }

    fn cursor_scroll_position(&self, cx: &mut ViewContext<Self>) -> Option<ScrollPosition> {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let cursor = editor.selections.newest_anchor().head();
            let cursor_row = cursor
                .to_display_point(&snapshot.display_snapshot)
                .row()
                .as_f32();
            let scroll_position = editor
                .scroll_manager
                .anchor()
                .scroll_position(&snapshot.display_snapshot);

            let scroll_bottom = scroll_position.y + editor.visible_line_count().unwrap_or(0.);
            if (scroll_position.y..scroll_bottom).contains(&cursor_row) {
                Some(ScrollPosition {
                    cursor,
                    offset_before_cursor: point(scroll_position.x, cursor_row - scroll_position.y),
                })
            } else {
                None
            }
        })
    }

    fn update_message_headers(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let old_blocks = std::mem::take(&mut self.blocks);
            let new_blocks = self
                .context
                .read(cx)
                .messages(cx)
                .map(|message| BlockProperties {
                    position: buffer
                        .anchor_in_excerpt(excerpt_id, message.anchor)
                        .unwrap(),
                    height: 2,
                    style: BlockStyle::Sticky,
                    render: Box::new({
                        let context = self.context.clone();
                        move |cx| {
                            let message_id = message.id;
                            let sender = ButtonLike::new("role")
                                .style(ButtonStyle::Filled)
                                .child(match message.role {
                                    Role::User => Label::new("You").color(Color::Default),
                                    Role::Assistant => Label::new("Assistant").color(Color::Info),
                                    Role::System => Label::new("System").color(Color::Warning),
                                })
                                .tooltip(|cx| {
                                    Tooltip::with_meta(
                                        "Toggle message role",
                                        None,
                                        "Available roles: You (User), Assistant, System",
                                        cx,
                                    )
                                })
                                .on_click({
                                    let context = context.clone();
                                    move |_, cx| {
                                        context.update(cx, |context, cx| {
                                            context.cycle_message_roles(
                                                HashSet::from_iter(Some(message_id)),
                                                cx,
                                            )
                                        })
                                    }
                                });

                            h_flex()
                                .id(("message_header", message_id.as_u64()))
                                .pl(cx.gutter_dimensions.full_width())
                                .h_11()
                                .w_full()
                                .relative()
                                .gap_1()
                                .child(sender)
                                .children(
                                    if let MessageStatus::Error(error) = message.status.clone() {
                                        Some(
                                            div()
                                                .id("error")
                                                .tooltip(move |cx| Tooltip::text(error.clone(), cx))
                                                .child(Icon::new(IconName::XCircle)),
                                        )
                                    } else {
                                        None
                                    },
                                )
                                .into_any_element()
                        }
                    }),
                    disposition: BlockDisposition::Above,
                })
                .collect::<Vec<_>>();

            editor.remove_blocks(old_blocks, None, cx);
            let ids = editor.insert_blocks(new_blocks, None, cx);
            self.blocks = HashSet::from_iter(ids);
        });
    }

    fn insert_selection(
        workspace: &mut Workspace,
        _: &InsertIntoEditor,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(context_editor_view) = panel.read(cx).active_context_editor(cx) else {
            return;
        };
        let Some(active_editor_view) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        let context_editor = context_editor_view.read(cx).editor.read(cx);
        let anchor = context_editor.selections.newest_anchor();
        let text = context_editor
            .buffer()
            .read(cx)
            .read(cx)
            .text_for_range(anchor.range())
            .collect::<String>();

        // If nothing is selected, don't delete the current selection; instead, be a no-op.
        if !text.is_empty() {
            active_editor_view.update(cx, |editor, cx| {
                editor.insert(&text, cx);
                editor.focus(cx);
            })
        }
    }

    fn quote_selection(
        workspace: &mut Workspace,
        _: &QuoteSelection,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        let selection = editor.update(cx, |editor, cx| editor.selections.newest_adjusted(cx));
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx).snapshot(cx);
        let range = editor::ToOffset::to_offset(&selection.start, &buffer)
            ..editor::ToOffset::to_offset(&selection.end, &buffer);
        let start_language = buffer.language_at(range.start);
        let end_language = buffer.language_at(range.end);
        let language_name = if start_language == end_language {
            start_language.map(|language| language.code_fence_block_name())
        } else {
            None
        };
        let language_name = language_name.as_deref().unwrap_or("");

        let selected_text = buffer.text_for_range(range).collect::<String>();
        let text = if selected_text.is_empty() {
            None
        } else {
            Some(if language_name == "markdown" {
                selected_text
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                format!("```{language_name}\n{selected_text}\n```")
            })
        };

        // Activate the panel
        if !panel.focus_handle(cx).contains_focused(cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        }

        if let Some(text) = text {
            panel.update(cx, |_, cx| {
                // Wait to create a new context until the workspace is no longer
                // being updated.
                cx.defer(move |panel, cx| {
                    if let Some(context) = panel
                        .active_context_editor(cx)
                        .or_else(|| panel.new_context(cx))
                    {
                        context.update(cx, |context, cx| {
                            context
                                .editor
                                .update(cx, |editor, cx| editor.insert(&text, cx))
                        });
                    };
                });
            });
        }
    }

    fn copy(&mut self, _: &editor::actions::Copy, cx: &mut ViewContext<Self>) {
        let editor = self.editor.read(cx);
        let context = self.context.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let mut copied_text = String::new();
            let mut spanned_messages = 0;
            for message in context.messages(cx) {
                if message.offset_range.start >= selection.range().end {
                    break;
                } else if message.offset_range.end >= selection.range().start {
                    let range = cmp::max(message.offset_range.start, selection.range().start)
                        ..cmp::min(message.offset_range.end, selection.range().end);
                    if !range.is_empty() {
                        spanned_messages += 1;
                        write!(&mut copied_text, "## {}\n\n", message.role).unwrap();
                        for chunk in context.buffer().read(cx).text_for_range(range) {
                            copied_text.push_str(chunk);
                        }
                        copied_text.push('\n');
                    }
                }
            }

            if spanned_messages > 1 {
                cx.write_to_clipboard(ClipboardItem::new(copied_text));
                return;
            }
        }

        cx.propagate();
    }

    fn split(&mut self, _: &Split, cx: &mut ViewContext<Self>) {
        self.context.update(cx, |context, cx| {
            let selections = self.editor.read(cx).selections.disjoint_anchors();
            for selection in selections.as_ref() {
                let buffer = self.editor.read(cx).buffer().read(cx).snapshot(cx);
                let range = selection
                    .map(|endpoint| endpoint.to_offset(&buffer))
                    .range();
                context.split_message(range, cx);
            }
        });
    }

    fn save(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        self.context
            .update(cx, |context, cx| context.save(None, self.fs.clone(), cx));
    }

    fn title(&self, cx: &AppContext) -> String {
        self.context
            .read(cx)
            .summary()
            .map(|summary| summary.text.clone())
            .unwrap_or_else(|| "New Context".into())
    }

    fn render_send_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();
        let button_text = match self.edit_step_for_cursor(cx) {
            Some(edit_step) => match &edit_step.operations {
                Some(EditStepOperations::Pending(_)) => "Computing Changes...",
                Some(EditStepOperations::Parsed { .. }) => "Apply Changes",
                None => "Send",
            },
            None => "Send",
        };
        ButtonLike::new("send_button")
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ModalSurface)
            .children(
                KeyBinding::for_action_in(&Assist, &focus_handle, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .child(Label::new(button_text))
            .on_click(move |_event, cx| {
                focus_handle.dispatch_action(&Assist, cx);
            })
    }

    fn edit_step_for_cursor<'a>(&'a self, cx: &'a AppContext) -> Option<&'a EditStep> {
        let newest_cursor = self
            .editor
            .read(cx)
            .selections
            .newest_anchor()
            .head()
            .text_anchor;
        let context = self.context.read(cx);
        let buffer = context.buffer().read(cx);

        let edit_steps = context.edit_steps();
        edit_steps
            .binary_search_by(|step| {
                let step_range = step.source_range.clone();
                if newest_cursor.cmp(&step_range.start, buffer).is_lt() {
                    Ordering::Greater
                } else if newest_cursor.cmp(&step_range.end, buffer).is_gt() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .ok()
            .map(|index| &edit_steps[index])
    }
}

impl EventEmitter<EditorEvent> for ContextEditor {}
impl EventEmitter<SearchEvent> for ContextEditor {}

impl Render for ContextEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("ContextEditor")
            .capture_action(cx.listener(ContextEditor::cancel_last_assist))
            .capture_action(cx.listener(ContextEditor::save))
            .capture_action(cx.listener(ContextEditor::copy))
            .capture_action(cx.listener(ContextEditor::cycle_message_role))
            .capture_action(cx.listener(ContextEditor::confirm_command))
            .on_action(cx.listener(ContextEditor::assist))
            .on_action(cx.listener(ContextEditor::split))
            .on_action(cx.listener(ContextEditor::debug_edit_steps))
            .size_full()
            .v_flex()
            .child(
                div()
                    .flex_grow()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.editor.clone())
                    .child(
                        h_flex()
                            .w_full()
                            .absolute()
                            .bottom_0()
                            .p_4()
                            .justify_end()
                            .child(self.render_send_button(cx)),
                    ),
            )
    }
}

impl FocusableView for ContextEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for ContextEditor {
    type Event = editor::EditorEvent;

    fn tab_content_text(&self, cx: &WindowContext) -> Option<SharedString> {
        Some(util::truncate_and_trailoff(&self.title(cx), Self::MAX_TAB_TITLE_LEN).into())
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(item::ItemEvent)) {
        match event {
            EditorEvent::Edited { .. } => {
                f(item::ItemEvent::Edit);
                f(item::ItemEvent::UpdateBreadcrumbs);
            }
            EditorEvent::TitleChanged => {
                f(item::ItemEvent::UpdateTab);
            }
            _ => {}
        }
    }

    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
        Some(self.title(cx).into())
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn breadcrumbs(
        &self,
        theme: &theme::Theme,
        cx: &AppContext,
    ) -> Option<Vec<item::BreadcrumbText>> {
        let editor = self.editor.read(cx);
        let cursor = editor.selections.newest_anchor().head();
        let multibuffer = &editor.buffer().read(cx);
        let (_, symbols) = multibuffer.symbols_containing(cursor, Some(&theme.syntax()), cx)?;

        let settings = ThemeSettings::get_global(cx);

        let mut breadcrumbs = Vec::new();

        let title = self.title(cx);
        if title.chars().count() > Self::MAX_TAB_TITLE_LEN {
            breadcrumbs.push(BreadcrumbText {
                text: title,
                highlights: None,
                font: Some(settings.buffer_font.clone()),
            });
        }

        breadcrumbs.extend(symbols.into_iter().map(|symbol| BreadcrumbText {
            text: symbol.text,
            highlights: Some(symbol.highlight_ranges),
            font: Some(settings.buffer_font.clone()),
        }));
        Some(breadcrumbs)
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn set_nav_history(&mut self, nav_history: pane::ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, nav_history, cx)
        })
    }

    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| Item::navigate(editor, data, cx))
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::deactivated(editor, cx))
    }
}

impl SearchableItem for ContextEditor {
    type Match = <Editor as SearchableItem>::Match;

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.clear_matches(cx);
        });
    }

    fn update_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.update_matches(matches, cx));
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.editor
            .update(cx, |editor, cx| editor.query_suggestion(cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.activate_match(index, matches, cx);
        });
    }

    fn select_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.select_matches(matches, cx));
    }

    fn replace(
        &mut self,
        identifier: &Self::Match,
        query: &project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.replace(identifier, query, cx));
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>> {
        self.editor
            .update(cx, |editor, cx| editor.find_matches(query, cx))
    }

    fn active_match_index(
        &mut self,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        self.editor
            .update(cx, |editor, cx| editor.active_match_index(matches, cx))
    }
}

impl FollowableItem for ContextEditor {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant> {
        let context = self.context.read(cx);
        Some(proto::view::Variant::ContextEditor(
            proto::view::ContextEditor {
                context_id: context.id().to_proto(),
                editor: if let Some(proto::view::Variant::Editor(proto)) =
                    self.editor.read(cx).to_state_proto(cx)
                {
                    Some(proto)
                } else {
                    None
                },
            },
        ))
    }

    fn from_state_proto(
        workspace: View<Workspace>,
        id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut WindowContext,
    ) -> Option<Task<Result<View<Self>>>> {
        let proto::view::Variant::ContextEditor(_) = state.as_ref()? else {
            return None;
        };
        let Some(proto::view::Variant::ContextEditor(state)) = state.take() else {
            unreachable!()
        };

        let context_id = ContextId::from_proto(state.context_id);
        let editor_state = state.editor?;

        let (project, panel) = workspace.update(cx, |workspace, cx| {
            Some((
                workspace.project().clone(),
                workspace.panel::<AssistantPanel>(cx)?,
            ))
        })?;

        let context_editor =
            panel.update(cx, |panel, cx| panel.open_remote_context(context_id, cx));

        Some(cx.spawn(|mut cx| async move {
            let context_editor = context_editor.await?;
            context_editor
                .update(&mut cx, |context_editor, cx| {
                    context_editor.remote_id = Some(id);
                    context_editor.editor.update(cx, |editor, cx| {
                        editor.apply_update_proto(
                            &project,
                            proto::update_view::Variant::Editor(proto::update_view::Editor {
                                selections: editor_state.selections,
                                pending_selection: editor_state.pending_selection,
                                scroll_top_anchor: editor_state.scroll_top_anchor,
                                scroll_x: editor_state.scroll_y,
                                scroll_y: editor_state.scroll_y,
                                ..Default::default()
                            }),
                            cx,
                        )
                    })
                })?
                .await?;
            Ok(context_editor)
        }))
    }

    fn to_follow_event(event: &Self::Event) -> Option<item::FollowEvent> {
        Editor::to_follow_event(event)
    }

    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &WindowContext,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.apply_update_proto(project, message, cx)
        })
    }

    fn is_project_item(&self, _cx: &WindowContext) -> bool {
        true
    }

    fn set_leader_peer_id(
        &mut self,
        leader_peer_id: Option<proto::PeerId>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_leader_peer_id(leader_peer_id, cx)
        })
    }

    fn dedup(&self, existing: &Self, cx: &WindowContext) -> Option<item::Dedup> {
        if existing.context.read(cx).id() == self.context.read(cx).id() {
            Some(item::Dedup::KeepExisting)
        } else {
            None
        }
    }
}

pub struct ContextEditorToolbarItem {
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    active_context_editor: Option<WeakView<ContextEditor>>,
    model_selector_menu_handle: PopoverMenuHandle<ContextMenu>,
}

impl ContextEditorToolbarItem {
    pub fn new(
        workspace: &Workspace,
        model_selector_menu_handle: PopoverMenuHandle<ContextMenu>,
    ) -> Self {
        Self {
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            active_context_editor: None,
            model_selector_menu_handle,
        }
    }

    fn render_inject_context_menu(&self, cx: &mut ViewContext<Self>) -> impl Element {
        let commands = SlashCommandRegistry::global(cx);
        let active_editor_focus_handle = self.workspace.upgrade().and_then(|workspace| {
            Some(
                workspace
                    .read(cx)
                    .active_item_as::<Editor>(cx)?
                    .focus_handle(cx),
            )
        });
        let active_context_editor = self.active_context_editor.clone();

        PopoverMenu::new("inject-context-menu")
            .trigger(IconButton::new("trigger", IconName::Quote).tooltip(|cx| {
                Tooltip::with_meta("Insert Context", None, "Type / to insert via keyboard", cx)
            }))
            .menu(move |cx| {
                let active_context_editor = active_context_editor.clone()?;
                ContextMenu::build(cx, |mut menu, _cx| {
                    for command_name in commands.featured_command_names() {
                        if let Some(command) = commands.command(&command_name) {
                            let menu_text = SharedString::from(Arc::from(command.menu_text()));
                            menu = menu.custom_entry(
                                {
                                    let command_name = command_name.clone();
                                    move |_cx| {
                                        h_flex()
                                            .gap_4()
                                            .w_full()
                                            .justify_between()
                                            .child(Label::new(menu_text.clone()))
                                            .child(
                                                Label::new(format!("/{command_name}"))
                                                    .color(Color::Muted),
                                            )
                                            .into_any()
                                    }
                                },
                                {
                                    let active_context_editor = active_context_editor.clone();
                                    move |cx| {
                                        active_context_editor
                                            .update(cx, |context_editor, cx| {
                                                context_editor.insert_command(&command_name, cx)
                                            })
                                            .ok();
                                    }
                                },
                            )
                        }
                    }

                    if let Some(active_editor_focus_handle) = active_editor_focus_handle.clone() {
                        menu = menu
                            .context(active_editor_focus_handle)
                            .action("Quote Selection", Box::new(QuoteSelection));
                    }

                    menu
                })
                .into()
            })
    }

    fn render_remaining_tokens(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let model = CompletionProvider::global(cx).model();
        let context = &self
            .active_context_editor
            .as_ref()?
            .upgrade()?
            .read(cx)
            .context;
        let token_count = context.read(cx).token_count()?;
        let max_token_count = model.max_token_count();

        let remaining_tokens = max_token_count as isize - token_count as isize;
        let token_count_color = if remaining_tokens <= 0 {
            Color::Error
        } else if token_count as f32 / max_token_count as f32 >= 0.8 {
            Color::Warning
        } else {
            Color::Muted
        };

        Some(
            h_flex()
                .gap_0p5()
                .child(
                    Label::new(humanize_token_count(token_count))
                        .size(LabelSize::Small)
                        .color(token_count_color),
                )
                .child(Label::new("/").size(LabelSize::Small).color(Color::Muted))
                .child(
                    Label::new(humanize_token_count(max_token_count))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
}

impl Render for ContextEditorToolbarItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(ModelSelector::new(
                self.model_selector_menu_handle.clone(),
                self.fs.clone(),
            ))
            .children(self.render_remaining_tokens(cx))
            .child(self.render_inject_context_menu(cx))
    }
}

impl ToolbarItemView for ContextEditorToolbarItem {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        self.active_context_editor = active_pane_item
            .and_then(|item| item.act_as::<ContextEditor>(cx))
            .map(|editor| editor.downgrade());
        cx.notify();
        if self.active_context_editor.is_none() {
            ToolbarItemLocation::Hidden
        } else {
            ToolbarItemLocation::PrimaryRight
        }
    }

    fn pane_focus_update(&mut self, _pane_focused: bool, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl EventEmitter<ToolbarItemEvent> for ContextEditorToolbarItem {}

pub struct ContextHistory {
    picker: View<Picker<SavedContextPickerDelegate>>,
    _subscriptions: Vec<Subscription>,
    assistant_panel: WeakView<AssistantPanel>,
}

impl ContextHistory {
    fn new(
        project: Model<Project>,
        context_store: Model<ContextStore>,
        assistant_panel: WeakView<AssistantPanel>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(
                SavedContextPickerDelegate::new(project, context_store.clone()),
                cx,
            )
            .modal(false)
            .max_height(None)
        });

        let _subscriptions = vec![
            cx.observe(&context_store, |this, _, cx| {
                this.picker.update(cx, |picker, cx| picker.refresh(cx));
            }),
            cx.subscribe(&picker, Self::handle_picker_event),
        ];

        Self {
            picker,
            _subscriptions,
            assistant_panel,
        }
    }

    fn handle_picker_event(
        &mut self,
        _: View<Picker<SavedContextPickerDelegate>>,
        event: &SavedContextPickerEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let SavedContextPickerEvent::Confirmed(context) = event;
        self.assistant_panel
            .update(cx, |assistant_panel, cx| match context {
                ContextMetadata::Remote(metadata) => {
                    assistant_panel
                        .open_remote_context(metadata.id.clone(), cx)
                        .detach_and_log_err(cx);
                }
                ContextMetadata::Saved(metadata) => {
                    assistant_panel
                        .open_saved_context(metadata.path.clone(), cx)
                        .detach_and_log_err(cx);
                }
            })
            .ok();
    }
}

impl Render for ContextHistory {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(self.picker.clone())
    }
}

impl FocusableView for ContextHistory {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<()> for ContextHistory {}

impl Item for ContextHistory {
    type Event = ();

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("History".into())
    }
}

type ToggleFold = Arc<dyn Fn(bool, &mut WindowContext) + Send + Sync>;

fn render_slash_command_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _cx: &mut WindowContext,
) -> AnyElement {
    Disclosure::new(
        ("slash-command-output-fold-indicator", row.0 as u64),
        !is_folded,
    )
    .selected(is_folded)
    .on_click(move |_e, cx| fold(!is_folded, cx))
    .into_any_element()
}

fn render_pending_slash_command_gutter_decoration(
    row: MultiBufferRow,
    status: &PendingSlashCommandStatus,
    confirm_command: Arc<dyn Fn(&mut WindowContext)>,
) -> AnyElement {
    let mut icon = IconButton::new(
        ("slash-command-gutter-decoration", row.0),
        ui::IconName::TriangleRight,
    )
    .on_click(move |_e, cx| confirm_command(cx))
    .icon_size(ui::IconSize::Small)
    .size(ui::ButtonSize::None);

    match status {
        PendingSlashCommandStatus::Idle => {
            icon = icon.icon_color(Color::Muted);
        }
        PendingSlashCommandStatus::Running { .. } => {
            icon = icon.selected(true);
        }
        PendingSlashCommandStatus::Error(_) => icon = icon.icon_color(Color::Error),
    }

    icon.into_any_element()
}

fn render_docs_slash_command_trailer(
    row: MultiBufferRow,
    command: PendingSlashCommand,
    cx: &mut WindowContext,
) -> AnyElement {
    let Some(argument) = command.argument else {
        return Empty.into_any();
    };

    let args = DocsSlashCommandArgs::parse(&argument);

    let Some(store) = args
        .provider()
        .and_then(|provider| IndexedDocsStore::try_global(provider, cx).ok())
    else {
        return Empty.into_any();
    };

    let Some(package) = args.package() else {
        return Empty.into_any();
    };

    let mut children = Vec::new();

    if store.is_indexing(&package) {
        children.push(
            div()
                .id(("crates-being-indexed", row.0))
                .child(Icon::new(IconName::ArrowCircle).with_animation(
                    "arrow-circle",
                    Animation::new(Duration::from_secs(4)).repeat(),
                    |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                ))
                .tooltip({
                    let package = package.clone();
                    move |cx| Tooltip::text(format!("Indexing {package}…"), cx)
                })
                .into_any_element(),
        );
    }

    if let Some(latest_error) = store.latest_error_for_package(&package) {
        children.push(
            div()
                .id(("latest-error", row.0))
                .child(Icon::new(IconName::ExclamationTriangle).color(Color::Warning))
                .tooltip(move |cx| Tooltip::text(format!("failed to index: {latest_error}"), cx))
                .into_any_element(),
        )
    }

    let is_indexing = store.is_indexing(&package);
    let latest_error = store.latest_error_for_package(&package);

    if !is_indexing && latest_error.is_none() {
        return Empty.into_any();
    }

    h_flex().gap_2().children(children).into_any_element()
}

fn make_lsp_adapter_delegate(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Result<Arc<dyn LspAdapterDelegate>> {
    project.update(cx, |project, cx| {
        // TODO: Find the right worktree.
        let worktree = project
            .worktrees()
            .next()
            .ok_or_else(|| anyhow!("no worktrees when constructing ProjectLspAdapterDelegate"))?;
        Ok(ProjectLspAdapterDelegate::new(project, &worktree, cx) as Arc<dyn LspAdapterDelegate>)
    })
}

fn slash_command_error_block_renderer(message: String) -> RenderBlock {
    Box::new(move |_| {
        div()
            .pl_6()
            .child(
                Label::new(format!("error: {}", message))
                    .single_line()
                    .color(Color::Error),
            )
            .into_any()
    })
}

use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

use anyhow::Result;
use editor::{Editor, EditorSettings, SplittableEditor};
use gpui::{
    Action, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    KeyBinding, ScrollStrategy, Subscription, UniformListScrollHandle, WeakEntity, Window, actions,
    uniform_list,
};
use language::{Buffer, Capability};
use multi_buffer::MultiBuffer;
use project::{
    Project,
    vcs_store::{VcsRepositoryId, VcsRepositorySnapshot, VcsStore},
};
use settings::Settings;
use ui::{Button, Color, IconButton, IconName, Label, ListItem, prelude::*};
use util::ResultExt as _;
use vcs::{Change, ChangeStatus, ContentKind};
use workspace::{
    Panel, Workspace,
    dock::{DockPosition, PanelEvent},
};

actions!(
    vcs_panel,
    [
        /// Focuses changes provided by external version-control systems.
        ToggleFocus,
        /// Refreshes external source-control repositories.
        Refresh,
        /// Opens errors and bounded stderr output from external version-control providers.
        ShowDiagnostics,
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", menu::SelectPrevious, Some("VcsPanel")),
        KeyBinding::new("down", menu::SelectNext, Some("VcsPanel")),
        KeyBinding::new("enter", menu::Confirm, Some("VcsPanel")),
    ]);
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<VcsPanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &ShowDiagnostics, window, cx| {
            show_diagnostics(workspace, window, cx);
        });
        workspace.register_action(|workspace, _: &Refresh, _, cx| {
            if let Some(store) = workspace.project().read(cx).vcs_store().cloned() {
                store.update(cx, |store, cx| store.refresh_all(cx));
            }
        });
    })
    .detach();
}

fn show_diagnostics(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let Some(store) = workspace.project().read(cx).vcs_store() else {
        return;
    };
    let store = store.read(cx);
    let mut sections = vec!["External VCS diagnostics".to_owned()];
    for repository in store.repositories() {
        sections.push(format!(
            "{}\nScope: {}\nBaseline: {}\nLoading: {}\nError: {}",
            repository.descriptor.display_name,
            repository.descriptor.scope_uri,
            repository.selected_baseline,
            repository.loading,
            repository.error.as_deref().unwrap_or("none"),
        ));
    }
    sections.extend(store.errors().into_iter().map(|error| error.to_string()));
    sections.extend(
        store
            .diagnostics()
            .into_iter()
            .map(|message| message.to_string()),
    );
    let buffer = cx.new(|cx| {
        let mut buffer = Buffer::local(sections.join("\n\n"), cx);
        buffer.set_capability(Capability::ReadOnly, cx);
        buffer
    });
    let multibuffer = cx
        .new(|cx| MultiBuffer::singleton(buffer, cx).with_title("External VCS diagnostics".into()));
    let editor = cx.new(|cx| Editor::for_multibuffer(multibuffer, None, window, cx));
    workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
}

#[derive(Clone)]
struct Row {
    repository: VcsRepositoryId,
    path: String,
    label: String,
    depth: usize,
    change: Option<Change>,
}

pub struct VcsPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    store: Option<Entity<VcsStore>>,
    focus: FocusHandle,
    position: DockPosition,
    repositories: Vec<VcsRepositorySnapshot>,
    rows: Vec<Row>,
    selected: Option<usize>,
    scroll_handle: UniformListScrollHandle,
    error: Option<SharedString>,
    opened: HashMap<(VcsRepositoryId, String), WeakEntity<SplittableEditor>>,
    _subscription: Option<Subscription>,
}

impl VcsPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, _, cx| {
            let project = workspace.project().clone();
            let workspace = cx.entity().downgrade();
            cx.new(|cx| Self::new(workspace, project, cx))
        })
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let store = project.read(cx).vcs_store().cloned();
        let subscription = store
            .as_ref()
            .map(|store| cx.subscribe(store, |this, _, _, cx| this.update_entries(cx)));
        let mut panel = Self {
            workspace,
            project,
            store,
            focus: cx.focus_handle(),
            position: DockPosition::Left,
            repositories: Vec::new(),
            rows: Vec::new(),
            selected: None,
            scroll_handle: UniformListScrollHandle::new(),
            error: None,
            opened: HashMap::new(),
            _subscription: subscription,
        };
        panel.update_entries(cx);
        panel
    }

    fn update_entries(&mut self, cx: &mut Context<Self>) {
        let selected = self
            .selected
            .and_then(|index| self.rows.get(index))
            .map(|row| (row.repository.clone(), row.path.clone()));
        self.repositories = self
            .store
            .as_ref()
            .map(|store| store.read(cx).repositories())
            .unwrap_or_default();
        let changes = self
            .repositories
            .iter()
            .map(|repository| {
                let changes = self
                    .store
                    .as_ref()
                    .map(|store| store.read(cx).changes(&repository.key, cx))
                    .unwrap_or_default();
                (repository, changes)
            })
            .collect::<Vec<_>>();
        self.rows = rows_for_repositories(&changes);
        self.selected = selected
            .and_then(|selected| {
                self.rows.iter().position(|row| {
                    row.change.is_some() && (row.repository.clone(), row.path.clone()) == selected
                })
            })
            .or_else(|| self.rows.iter().position(|row| row.change.is_some()));
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let start = self.selected.map_or(0, |index| index + 1);
        self.selected = (start..self.rows.len())
            .find(|index| self.rows[*index].change.is_some())
            .or(self.selected);
        if let Some(index) = self.selected {
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Center);
        }
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let start = self.selected.unwrap_or(self.rows.len());
        self.selected = (0..start)
            .rev()
            .find(|index| self.rows[*index].change.is_some())
            .or(self.selected);
        if let Some(index) = self.selected {
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Center);
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected {
            self.open_diff(index, window, cx);
        }
    }

    fn open_diff(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(row) = self.rows.get(index).cloned() else {
            return;
        };
        let Some(change) = &row.change else {
            return;
        };
        self.selected = Some(index);
        if change.kind != ContentKind::File {
            self.error = Some("This change is listed with its metadata; a text diff is unavailable for binary, symlink or submodule content.".into());
            cx.notify();
            return;
        }
        let Some(store) = self.store.clone() else {
            return;
        };
        let item_key = (row.repository.clone(), row.path.clone());
        if let Some(item) = self.opened.get(&item_key).and_then(WeakEntity::upgrade) {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.activate_item(&item, true, true, window, cx)
                })
                .log_err();
            return;
        }
        self.error = None;
        let task = store.update(cx, |store, cx| {
            store.open_diff(row.repository.clone(), row.path.clone(), cx)
        });
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        cx.spawn_in(window, async move |this, cx| {
            let result: Result<()> = async {
                let opened_diff = task.await?;
                let item = workspace.update_in(cx, |workspace, window, cx| {
                    let multibuffer = cx.new(|cx| {
                        let mut multibuffer = MultiBuffer::singleton(opened_diff.buffer, cx);
                        multibuffer.add_diff(opened_diff.diff, cx);
                        multibuffer.set_title(format!("{} — Changes", row.path), cx);
                        multibuffer
                    });
                    let workspace_entity = cx.entity();
                    let editor = cx.new(|cx| {
                        SplittableEditor::new(
                            EditorSettings::get_global(cx).diff_view_style,
                            multibuffer,
                            project,
                            workspace_entity,
                            window,
                            cx,
                        )
                    });
                    workspace.add_item_to_active_pane(
                        Box::new(editor.clone()),
                        None,
                        true,
                        window,
                        cx,
                    );
                    editor
                })?;
                this.update(cx, |this, _| {
                    this.opened.insert(item_key, item.downgrade());
                })?;
                Ok(())
            }
            .await;
            if let Err(error) = result {
                this.update(cx, |this, cx| {
                    this.error = Some(format!("{error:#}").into());
                    cx.notify();
                })
                .log_err();
            }
        })
        .detach();
        cx.notify();
    }

    fn render_rows(
        &mut self,
        range: Range<usize>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        range
            .filter_map(|index| self.rows.get(index).cloned().map(|row| (index, row)))
            .map(|(index, row)| {
                let mut label = row.label;
                let (status, color) = match row.change.as_ref().map(|change| change.status) {
                    Some(ChangeStatus::Added) => ("A", Color::Success),
                    Some(ChangeStatus::Untracked) => ("?", Color::Success),
                    Some(ChangeStatus::Modified) => ("M", Color::Warning),
                    Some(ChangeStatus::Deleted) => ("D", Color::Error),
                    Some(ChangeStatus::Renamed) => ("R", Color::Muted),
                    None => ("", Color::Muted),
                };
                if let Some(change) = &row.change {
                    if let Some(old_path) = &change.old_path {
                        label = format!("{old_path} → {label}");
                    }
                    if change.old_mode.is_some()
                        && change.new_mode.is_some()
                        && change.old_mode != change.new_mode
                    {
                        label.push_str(" (mode changed)");
                    }
                }
                ListItem::new(index)
                    .inset(true)
                    .indent_level(row.depth)
                    .toggle_state(self.selected == Some(index))
                    .disabled(row.change.is_none())
                    .child(Label::new(label).color(color))
                    .end_slot(Label::new(status).color(color))
                    .on_click(
                        cx.listener(move |this, _, window, cx| this.open_diff(index, window, cx)),
                    )
                    .into_any_element()
            })
            .collect()
    }
}

fn rows_for_repositories(repositories: &[(&VcsRepositorySnapshot, Vec<Change>)]) -> Vec<Row> {
    let mut rows = Vec::new();
    for (repository, changes) in repositories {
        rows.push(Row {
            repository: repository.key.clone(),
            path: String::new(),
            label: format!(
                "{} — {}",
                repository.descriptor.display_name, repository.descriptor.scope_uri
            ),
            depth: 0,
            change: None,
        });
        let mut entries = BTreeMap::new();
        for change in changes {
            let components: Vec<_> = change.path.split('/').collect();
            for depth in 1..components.len() {
                let path = components[..depth].join("/");
                entries
                    .entry(
                        components[..depth]
                            .iter()
                            .map(|part| part.to_string())
                            .collect::<Vec<_>>(),
                    )
                    .or_insert(Row {
                        repository: repository.key.clone(),
                        path,
                        label: components[depth - 1].to_owned(),
                        depth,
                        change: None,
                    });
            }
            entries.insert(
                components
                    .iter()
                    .map(|part| part.to_string())
                    .collect::<Vec<_>>(),
                Row {
                    repository: repository.key.clone(),
                    path: change.path.clone(),
                    label: components.last().unwrap_or(&"").to_string(),
                    depth: components.len(),
                    change: Some(change.clone()),
                },
            );
        }
        rows.extend(entries.into_values());
    }
    rows
}

impl EventEmitter<PanelEvent> for VcsPanel {}
impl Focusable for VcsPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for VcsPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let errors = self
            .store
            .as_ref()
            .map(|store| store.read(cx).errors())
            .unwrap_or_default();
        let has_errors = self.error.is_some()
            || !errors.is_empty()
            || self
                .repositories
                .iter()
                .any(|repository| repository.error.is_some());
        let repository_headers = self
            .repositories
            .iter()
            .enumerate()
            .map(|(repository_index, repository)| {
                let key = repository.key.clone();
                let baselines = repository
                    .descriptor
                    .baselines
                    .iter()
                    .enumerate()
                    .map(|(index, baseline)| {
                        let key = key.clone();
                        let baseline_id = baseline.id.clone();
                        Button::new(("baseline", index), baseline.label.clone())
                            .toggle_state(repository.selected_baseline == baseline.id)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(store) = this.store.clone() {
                                    if let Err(error) = store.update(cx, |store, cx| {
                                        store.set_baseline(&key, baseline_id.clone(), cx)
                                    }) {
                                        this.error = Some(format!("{error:#}").into());
                                    }
                                }
                                cx.notify();
                            }))
                    })
                    .collect::<Vec<_>>();
                v_flex()
                    .id(("vcs-repository", repository_index))
                    .px_2()
                    .py_1()
                    .gap_1()
                    .child(Label::new(repository.descriptor.display_name.clone()))
                    .child(
                        Label::new(repository.descriptor.scope_uri.clone())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(h_flex().flex_wrap().gap_1().children(baselines))
                    .when(repository.loading, |element| {
                        element.child(Label::new("Loading changes…").color(Color::Muted))
                    })
                    .when_some(repository.error.clone(), |element, error| {
                        element.child(Label::new(error).color(Color::Error))
                    })
            })
            .collect::<Vec<_>>();
        v_flex()
            .id("vcs-panel")
            .key_context("VcsPanel")
            .track_focus(&self.focus)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .child(
                h_flex()
                    .p_2()
                    .justify_between()
                    .child(Label::new("Source Control"))
                    .child(
                        Button::new("vcs-diagnostics", "Diagnostics").on_click(cx.listener(
                            |this, _, window, cx| {
                                this.workspace
                                    .update(cx, |workspace, cx| {
                                        show_diagnostics(workspace, window, cx)
                                    })
                                    .log_err();
                            },
                        )),
                    )
                    .child(
                        IconButton::new("refresh-vcs", IconName::ArrowCircle).on_click(
                            cx.listener(|this, _, _, cx| {
                                this.error = None;
                                if let Some(store) = &this.store {
                                    store.update(cx, |store, cx| store.refresh_all(cx));
                                }
                            }),
                        ),
                    ),
            )
            .children(repository_headers)
            .children(
                errors
                    .into_iter()
                    .map(|error| Label::new(error).color(Color::Error)),
            )
            .when_some(self.error.clone(), |element, error| {
                element.child(Label::new(error).color(Color::Error))
            })
            .when(
                self.rows.iter().all(|row| row.change.is_none())
                    && !has_errors
                    && !self
                        .repositories
                        .iter()
                        .any(|repository| repository.loading),
                |element| {
                    element.child(
                        Label::new(if self.repositories.is_empty() {
                            "No external source-control repository"
                        } else {
                            "No changes for this comparison"
                        })
                        .color(Color::Muted),
                    )
                },
            )
            .child(
                uniform_list(
                    "vcs-changes",
                    self.rows.len(),
                    cx.processor(Self::render_rows),
                )
                .track_scroll(&self.scroll_handle)
                .flex_1(),
            )
    }
}

impl Panel for VcsPanel {
    fn persistent_name() -> &'static str {
        "VcsPanel"
    }
    fn panel_key() -> &'static str {
        "vcs"
    }
    fn position(&self, _: &Window, _: &App) -> DockPosition {
        self.position
    }
    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }
    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        cx.notify();
    }
    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(300.)
    }
    fn icon(&self, _: &Window, _: &App) -> Option<IconName> {
        Some(IconName::GitBranch)
    }
    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Source Control")
    }
    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
    fn activation_priority(&self) -> u32 {
        3
    }
    fn enabled(&self, cx: &App) -> bool {
        !self.repositories.is_empty()
            || self
                .store
                .as_ref()
                .is_some_and(|store| !store.read(cx).errors().is_empty())
    }
}

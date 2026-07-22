use anyhow::Result;
use chrono::Local;
use gpui::{
    Action, App, AsyncWindowContext, Context, ElementId, Entity, EventEmitter, FocusHandle,
    Focusable, KeyContext, Pixels, PromptLevel, SharedString, Subscription, WeakEntity, Window,
    actions, px,
};
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::Project;
use std::collections::HashSet;
use std::path::PathBuf;
use ui::prelude::*;
use ui::{Button, Icon, IconButton, IconSize, Label, ListHeader, ListItem, Tooltip};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::notifications::NotificationId;
use workspace::{OpenOptions, OpenVisible, Toast, Workspace};

use crate::areas::{self, AreaManifest};
use crate::notes::{EnsureNoteOutcome, TimelineEntry, ensure_note};
use crate::vault::{Vault, VaultStatus, scaffold_vault};

const TIMELINE_PANEL_KEY: &str = "BreadPaperTimelinePanel";

const TIMELINE_ENTRIES: [(&str, &str, TimelineEntry); 4] = [
    ("breadpaper-today", "Today", TimelineEntry::Today),
    ("breadpaper-yesterday", "Yesterday", TimelineEntry::Yesterday),
    ("breadpaper-this-week", "This Week", TimelineEntry::ThisWeek),
    ("breadpaper-last-week", "Last Week", TimelineEntry::LastWeek),
];

actions!(
    breadpaper,
    [
        /// Toggles focus on the BreadPaper timeline panel.
        ToggleFocus,
        /// Opens today's daily note, creating it if needed.
        OpenToday,
        /// Opens yesterday's daily note, creating it if needed.
        OpenYesterday,
        /// Opens tomorrow's daily note, creating it if needed.
        OpenTomorrow,
        /// Opens this week's weekly note, creating it if needed.
        OpenThisWeek,
        /// Opens last week's weekly note, creating it if needed.
        OpenLastWeek
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<TimelinePanel>(window, cx);
        });
        register_open_action::<OpenToday>(workspace, TimelineEntry::Today);
        register_open_action::<OpenYesterday>(workspace, TimelineEntry::Yesterday);
        register_open_action::<OpenTomorrow>(workspace, TimelineEntry::Tomorrow);
        register_open_action::<OpenThisWeek>(workspace, TimelineEntry::ThisWeek);
        register_open_action::<OpenLastWeek>(workspace, TimelineEntry::LastWeek);
    })
    .detach();
}

fn register_open_action<A: Action>(workspace: &mut Workspace, entry: TimelineEntry) {
    workspace.register_action(move |workspace, _: &A, window, cx| {
        if let Some(panel) = workspace.panel::<TimelinePanel>(cx) {
            panel.update(cx, |panel, cx| panel.open_note(entry, window, cx));
        }
    });
}

/// Shows the timeline panel (opening the left dock) when the workspace is a
/// vault. Called once at startup after all panels are registered, so the
/// timeline — not the file tree — is what a vault opens on.
pub fn show_panel_if_vault(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let is_vault = workspace
        .visible_worktrees(cx)
        .next()
        .is_some_and(|worktree| {
            matches!(
                Vault::detect(&worktree.read(cx).abs_path()),
                VaultStatus::Valid(_)
            )
        });
    if is_vault {
        workspace.open_panel::<TimelinePanel>(window, cx);
    }
}

pub struct TimelinePanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    vault_status: VaultStatus,
    /// Keyboard cursor over `TIMELINE_ENTRIES`. `None` means the highlight
    /// follows whichever entry matches the active editor item.
    selected_index: Option<usize>,
    /// Enabled Areas in registry order, loaded whenever the vault status
    /// changes.
    areas: Vec<AreaManifest>,
    /// Catalog Areas without an enabled registry entry, offered by "Add Area".
    addable_areas: Vec<AreaManifest>,
    /// Area rows start expanded; this tracks the ones the user collapsed.
    collapsed_areas: HashSet<String>,
    show_add_areas: bool,
    position: DockPosition,
    _subscriptions: Vec<Subscription>,
}

impl TimelinePanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            TimelinePanel::new(workspace, window, cx)
        })
    }

    pub fn new(
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let weak_workspace = workspace.weak_handle();
        let workspace_entity = cx.entity();
        cx.new(|cx| {
            let project_subscription =
                cx.subscribe(&project, |this: &mut Self, _, event, cx| {
                    if matches!(
                        event,
                        project::Event::WorktreeAdded(_)
                            | project::Event::WorktreeRemoved(_)
                            | project::Event::WorktreeUpdatedEntries(..)
                    ) {
                        this.refresh_vault_status(cx);
                    }
                });
            // On active-item changes, drop the keyboard cursor so the
            // highlight goes back to following the open note.
            let workspace_subscription = cx.subscribe(
                &workspace_entity,
                |this: &mut Self, _, event: &workspace::Event, cx| {
                    if matches!(event, workspace::Event::ActiveItemChanged) {
                        this.selected_index = None;
                        cx.notify();
                    }
                },
            );
            let mut this = Self {
                workspace: weak_workspace,
                project,
                focus_handle: cx.focus_handle(),
                vault_status: VaultStatus::NotAVault,
                selected_index: None,
                areas: Vec::new(),
                addable_areas: Vec::new(),
                collapsed_areas: HashSet::new(),
                show_add_areas: false,
                position: DockPosition::Left,
                _subscriptions: vec![project_subscription, workspace_subscription],
            };
            this.refresh_vault_status(cx);
            this
        })
    }

    fn workspace_root(&self, cx: &App) -> Option<PathBuf> {
        let worktree = self.project.read(cx).visible_worktrees(cx).next()?;
        Some(worktree.read(cx).abs_path().to_path_buf())
    }

    fn refresh_vault_status(&mut self, cx: &mut Context<Self>) {
        let status = match self.workspace_root(cx) {
            Some(root) => Vault::detect(&root),
            None => VaultStatus::NotAVault,
        };
        if status != self.vault_status {
            self.vault_status = status;
            self.refresh_areas();
            self.reconcile_areas(cx);
            cx.notify();
        }
    }

    /// Re-materializes enabled Areas in the background whenever a vault is
    /// (re)detected, so a vault opened after an app update self-heals any
    /// newly shipped Area files (new skills, their Claude Code bridges) that a
    /// plain open would otherwise never create. Idempotent and never clobbers
    /// user edits; writing files leaves the registry unchanged, so it does not
    /// re-trigger `refresh_vault_status`.
    fn reconcile_areas(&mut self, cx: &mut Context<Self>) {
        let VaultStatus::Valid(vault) = &self.vault_status else {
            return;
        };
        let vault = vault.clone();
        let reconcile = cx.background_spawn(async move { areas::reconcile_enabled_areas(&vault) });
        cx.spawn(async move |this, cx| {
            reconcile.await?;
            this.update(cx, |this, cx| {
                this.refresh_areas();
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
    }

    /// Reloads the Areas section state from the vault's registry and the
    /// app-shipped catalog. Registry changes flow through `Vault` equality in
    /// `refresh_vault_status`, so install/remove refresh without a restart.
    fn refresh_areas(&mut self) {
        let VaultStatus::Valid(vault) = &self.vault_status else {
            self.areas = Vec::new();
            self.addable_areas = Vec::new();
            return;
        };
        self.areas = areas::enabled_areas(vault);
        self.addable_areas = match areas::catalog() {
            Ok(catalog) => catalog
                .into_iter()
                .map(|area| area.manifest)
                .filter(|manifest| {
                    !vault
                        .config
                        .areas
                        .installed
                        .iter()
                        .any(|entry| entry.enabled && entry.id == manifest.id)
                })
                .collect(),
            Err(error) => {
                log::error!("BreadPaper: couldn't load the Areas catalog: {error:?}");
                Vec::new()
            }
        };
    }

    fn open_note(&mut self, entry: TimelineEntry, window: &mut Window, cx: &mut Context<Self>) {
        let VaultStatus::Valid(vault) = &self.vault_status else {
            // Only reachable via the `breadpaper:` commands; the panel itself
            // renders no entries outside a valid vault.
            self.workspace
                .update(cx, |workspace, cx| {
                    struct NotAVaultToast;
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<NotAVaultToast>(),
                            "This workspace isn't a BreadPaper vault.",
                        )
                        .autohide(),
                        cx,
                    );
                })
                .log_err();
            return;
        };
        let vault = vault.clone();
        let now = Local::now();
        let Some((kind, date)) = entry.resolve(now.date_naive()) else {
            return;
        };
        let time = now.time();
        let workspace = self.workspace.clone();

        let ensure_note = cx.background_spawn(async move { ensure_note(&vault, kind, date, time) });
        cx.spawn_in(window, async move |_, cx| {
            match ensure_note.await {
                Ok((path, outcome)) => {
                    if outcome == EnsureNoteOutcome::CreatedWithoutTemplate {
                        workspace
                            .update(cx, |workspace, cx| {
                                struct TemplateMissingToast;
                                workspace.show_toast(
                                    Toast::new(
                                        NotificationId::unique::<TemplateMissingToast>(),
                                        "The note template is missing, so an empty note was created.",
                                    )
                                    .autohide(),
                                    cx,
                                );
                            })
                            .log_err();
                    }
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            workspace.open_abs_path(
                                path,
                                OpenOptions {
                                    visible: Some(OpenVisible::All),
                                    ..Default::default()
                                },
                                window,
                                cx,
                            )
                        })?
                        .await?;
                    Ok(())
                }
                Err(error) => {
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.show_error(format!("Couldn't open the note: {error}"), cx);
                        })
                        .log_err();
                    Err(error)
                }
            }
        })
        .detach_and_log_err(cx);
    }

    fn create_vault_here(&mut self, cx: &mut Context<Self>) {
        let Some(root) = self.workspace_root(cx) else {
            return;
        };
        let workspace = self.workspace.clone();
        let scaffold = cx.background_spawn(async move { scaffold_vault(&root) });
        cx.spawn(async move |this, cx| {
            match scaffold.await {
                Ok(()) => this.update(cx, |this, cx| this.refresh_vault_status(cx)),
                Err(error) => {
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace
                                .show_error(format!("Couldn't create the vault: {error}"), cx);
                        })
                        .log_err();
                    Err(error)
                }
            }
        })
        .detach_and_log_err(cx);
    }

    fn vault_root(&self) -> Option<PathBuf> {
        match &self.vault_status {
            VaultStatus::Valid(vault) => Some(vault.root.clone()),
            _ => None,
        }
    }

    /// Opens an Area-shipped markdown file (explainer doc or skill) in
    /// viewing mode. A missing file gets a toast offering to re-materialize
    /// the Area.
    fn open_area_file(
        &mut self,
        relative_path: String,
        area_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(root) = self.vault_root() else {
            return;
        };
        let path = match areas::vault_file_path(&root, &relative_path) {
            Ok(path) => path,
            Err(error) => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_error(format!("Couldn't open the file: {error}"), cx);
                    })
                    .log_err();
                return;
            }
        };
        if !path.is_file() {
            self.show_missing_file_toast(relative_path, area_id, cx);
            return;
        }
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            let open_result =
                crate::open_abs_path_as_preview(workspace.clone(), path, cx).await;
            if let Err(error) = &open_result {
                workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_error(format!("Couldn't open the file: {error}"), cx);
                    })
                    .log_err();
            }
            open_result
        })
        .detach_and_log_err(cx);
    }

    /// Opens an Area surface (e.g. the weekly dashboard) with the system
    /// handler — Zed has no web view, so HTML opens in the default browser.
    fn open_surface(&mut self, relative_path: String, area_id: String, cx: &mut Context<Self>) {
        let Some(root) = self.vault_root() else {
            return;
        };
        let path = match areas::vault_file_path(&root, &relative_path) {
            Ok(path) => path,
            Err(error) => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_error(format!("Couldn't open the file: {error}"), cx);
                    })
                    .log_err();
                return;
            }
        };
        if !path.is_file() {
            self.show_missing_file_toast(relative_path, area_id, cx);
            return;
        }
        cx.open_with_system(&path);
    }

    fn show_missing_file_toast(
        &mut self,
        relative_path: String,
        area_id: String,
        cx: &mut Context<Self>,
    ) {
        let panel = cx.entity().downgrade();
        self.workspace
            .update(cx, |workspace, cx| {
                struct AreaFileMissingToast;
                workspace.show_toast(
                    Toast::new(
                        NotificationId::unique::<AreaFileMissingToast>(),
                        format!("{relative_path} is missing from the vault."),
                    )
                    .on_click("Reinstall the Area's files", move |_window, cx| {
                        panel
                            .update(cx, |panel, cx| panel.install_area(area_id.clone(), cx))
                            .log_err();
                    }),
                    cx,
                );
            })
            .log_err();
    }

    /// Materializes a catalog Area into the vault (or re-enables a disabled
    /// one) and registers it; the section refreshes without a restart.
    fn install_area(&mut self, area_id: String, cx: &mut Context<Self>) {
        let Some(root) = self.vault_root() else {
            return;
        };
        let workspace = self.workspace.clone();
        let install =
            cx.background_spawn(async move { areas::install_area(&root, &area_id) });
        cx.spawn(async move |this, cx| {
            match install.await {
                Ok(()) => this.update(cx, |this, cx| {
                    this.show_add_areas = false;
                    this.refresh_vault_status(cx);
                }),
                Err(error) => {
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.show_error(format!("Couldn't add the Area: {error}"), cx);
                        })
                        .log_err();
                    Err(error)
                }
            }
        })
        .detach_and_log_err(cx);
    }

    /// Removing always asks: deactivate (keep all files) or deactivate and
    /// delete the Area-shipped files. The prompt lists exactly what would be
    /// deleted; user notes and modified-since-install files are never deleted.
    fn remove_area(&mut self, area_id: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(root) = self.vault_root() else {
            return;
        };
        let workspace = self.workspace.clone();
        let plan = cx.background_spawn({
            let root = root.clone();
            let area_id = area_id.clone();
            async move { areas::plan_removal(&root, &area_id) }
        });
        cx.spawn_in(window, async move |this, cx| {
            let plan = match plan.await {
                Ok(plan) => plan,
                Err(error) => {
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.show_error(
                                format!("Couldn't prepare the Area removal: {error}"),
                                cx,
                            );
                        })
                        .log_err();
                    return Err(error);
                }
            };

            let mut detail = String::new();
            if plan.delete.is_empty() {
                detail.push_str("No Area-shipped files would be deleted.\n");
            } else {
                detail.push_str("Deleting the Area's files removes:\n");
                for file in &plan.delete {
                    detail.push_str("  - ");
                    detail.push_str(file);
                    detail.push('\n');
                }
            }
            if !plan.keep_modified.is_empty() {
                detail.push_str("\nModified since install, always kept:\n");
                for file in &plan.keep_modified {
                    detail.push_str("  - ");
                    detail.push_str(file);
                    detail.push('\n');
                }
            }
            detail.push_str("\nYour notes are never deleted.");

            let answer = cx.update(|window, cx| {
                window.prompt(
                    PromptLevel::Warning,
                    &format!("Remove the {} Area?", plan.area_name),
                    Some(&detail),
                    &[
                        "Deactivate, Keep All Files",
                        "Deactivate and Delete Area Files",
                        "Cancel",
                    ],
                    cx,
                )
            })?;
            let Some(answer) = answer.await.log_err() else {
                return Ok(());
            };

            let operation = match answer {
                0 => cx
                    .background_spawn({
                        let root = root.clone();
                        let area_id = area_id.clone();
                        async move { areas::deactivate_area(&root, &area_id).map(|()| None) }
                    })
                    .await,
                1 => cx
                    .background_spawn({
                        let root = root.clone();
                        let area_id = area_id.clone();
                        async move { areas::delete_area(&root, &area_id).map(Some) }
                    })
                    .await,
                _ => return Ok(()),
            };

            match operation {
                Ok(outcome) => {
                    let message = match outcome {
                        None => format!(
                            "Deactivated the {} Area. All of its files were kept.",
                            plan.area_name
                        ),
                        Some(outcome) => {
                            let mut message = format!(
                                "Removed the {} Area and deleted {} of its files.",
                                plan.area_name,
                                outcome.deleted.len()
                            );
                            if !outcome.kept_modified.is_empty() {
                                message.push_str(&format!(
                                    " Kept {} modified: {}.",
                                    outcome.kept_modified.len(),
                                    outcome.kept_modified.join(", ")
                                ));
                            }
                            message
                        }
                    };
                    this.update(cx, |this, cx| this.refresh_vault_status(cx))?;
                    workspace
                        .update(cx, |workspace, cx| {
                            struct AreaRemovedToast;
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::unique::<AreaRemovedToast>(),
                                    message,
                                )
                                .autohide(),
                                cx,
                            );
                        })
                        .log_err();
                    Ok(())
                }
                Err(error) => {
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace
                                .show_error(format!("Couldn't remove the Area: {error}"), cx);
                        })
                        .log_err();
                    Err(error)
                }
            }
        })
        .detach_and_log_err(cx);
    }

    /// The absolute path of the note open in the active editor, if any.
    fn active_item_path(&self, cx: &App) -> Option<PathBuf> {
        let workspace = self.workspace.upgrade()?;
        let item = workspace.read(cx).active_item(cx)?;
        let project_path = item.project_path(cx)?;
        self.project.read(cx).absolute_path(&project_path, cx)
    }

    /// The entry whose note is open in the active editor, if any.
    fn active_entry_index(&self, cx: &App) -> Option<usize> {
        let VaultStatus::Valid(vault) = &self.vault_status else {
            return None;
        };
        let active_path = self.active_item_path(cx)?;
        let today = Local::now().date_naive();
        TIMELINE_ENTRIES.iter().position(|(_, _, entry)| {
            entry
                .resolve(today)
                .is_some_and(|(kind, date)| vault.note_path(kind, date) == active_path)
        })
    }

    /// The highlighted entry: the keyboard cursor if one is set, otherwise the
    /// entry matching the active editor item.
    fn effective_selected_index(&self, cx: &App) -> Option<usize> {
        self.selected_index.or_else(|| self.active_entry_index(cx))
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        self.selected_index = Some(match self.effective_selected_index(cx) {
            Some(index) => (index + 1).min(TIMELINE_ENTRIES.len() - 1),
            None => 0,
        });
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected_index = Some(match self.effective_selected_index(cx) {
            Some(index) => index.saturating_sub(1),
            None => TIMELINE_ENTRIES.len() - 1,
        });
        cx.notify();
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        self.selected_index = Some(0);
        cx.notify();
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        self.selected_index = Some(TIMELINE_ENTRIES.len() - 1);
        cx.notify();
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((_, _, entry)) = self
            .effective_selected_index(cx)
            .and_then(|index| TIMELINE_ENTRIES.get(index))
        {
            self.open_note(*entry, window, cx);
        }
    }

    fn render_entries(&self, cx: &Context<Self>) -> impl IntoElement {
        let selected_index = self.effective_selected_index(cx);
        v_flex()
            .gap_px()
            .child(
                ListHeader::new("Timeline").start_slot(
                    Icon::new(IconName::Clock)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                ),
            )
            .children(TIMELINE_ENTRIES.iter().enumerate().map(
                |(index, (id, label, entry))| {
                    let entry = *entry;
                    ListItem::new(*id)
                        .toggle_state(selected_index == Some(index))
                        .child(Label::new(*label))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.open_note(entry, window, cx);
                        }))
                },
            ))
    }

    fn render_areas_section(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_px()
            .mt_2()
            .child(
                ListHeader::new("Areas").start_slot(
                    Icon::new(IconName::Blocks)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                ),
            )
            .when(self.areas.is_empty(), |this| {
                this.child(
                    div().px_2().py_1().child(
                        Label::new("No Areas are enabled in this vault.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                )
            })
            .children(
                self.areas
                    .iter()
                    .map(|manifest| self.render_area(manifest, cx)),
            )
            .child(self.render_add_area(cx))
    }

    fn render_area(&self, manifest: &AreaManifest, cx: &Context<Self>) -> AnyElement {
        let area_id = manifest.id.clone();
        let expanded = !self.collapsed_areas.contains(&manifest.id);
        let mut section = v_flex().child(
            ListItem::new(ElementId::Name(SharedString::from(format!(
                "breadpaper-area-{}",
                manifest.id
            ))))
            .toggle(expanded)
            .always_show_disclosure_icon(true)
            .on_toggle(cx.listener({
                let area_id = area_id.clone();
                move |this, _, _window, cx| {
                    if !this.collapsed_areas.remove(&area_id) {
                        this.collapsed_areas.insert(area_id.clone());
                    }
                    cx.notify();
                }
            }))
            .child(Label::new(manifest.name.clone()))
            .end_slot(
                IconButton::new(
                    ElementId::Name(SharedString::from(format!(
                        "breadpaper-remove-area-{}",
                        manifest.id
                    ))),
                    IconName::Trash,
                )
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text("Remove Area…"))
                .on_click(cx.listener({
                    let area_id = area_id.clone();
                    move |this, _, window, cx| {
                        this.remove_area(area_id.clone(), window, cx);
                    }
                })),
            )
            .on_click(cx.listener({
                let area_id = area_id.clone();
                let doc = manifest.doc.clone();
                move |this, _, window, cx| {
                    this.open_area_file(doc.clone(), area_id.clone(), window, cx);
                }
            })),
        );
        if expanded {
            section = section
                .children(manifest.skills.iter().map(|skill| {
                    ListItem::new(ElementId::Name(SharedString::from(format!(
                        "breadpaper-skill-{}-{}",
                        manifest.id, skill.id
                    ))))
                    .indent_level(1)
                    .indent_step_size(px(12.))
                    .start_slot(
                        Icon::new(IconName::Book)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(Label::new(skill.name.clone()).size(LabelSize::Small))
                    .when(!skill.summary.is_empty(), |item| {
                        item.tooltip(Tooltip::text(skill.summary.clone()))
                    })
                    .on_click(cx.listener({
                        let area_id = area_id.clone();
                        let file = skill.file.clone();
                        move |this, _, window, cx| {
                            this.open_area_file(file.clone(), area_id.clone(), window, cx);
                        }
                    }))
                }))
                .children(manifest.surfaces.iter().enumerate().map(
                    |(surface_index, surface)| {
                        ListItem::new(ElementId::Name(SharedString::from(format!(
                            "breadpaper-surface-{}-{surface_index}",
                            manifest.id
                        ))))
                        .indent_level(1)
                        .indent_step_size(px(12.))
                        .start_slot(
                            Icon::new(IconName::ArrowUpRight)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(Label::new(surface.name.clone()).size(LabelSize::Small))
                        .on_click(cx.listener({
                            let area_id = area_id.clone();
                            let open = surface.open.clone();
                            move |this, _, _window, cx| {
                                this.open_surface(open.clone(), area_id.clone(), cx);
                            }
                        }))
                    },
                ));
        }
        section.into_any_element()
    }

    fn render_add_area(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .child(
                ListItem::new("breadpaper-add-area")
                    .toggle_state(self.show_add_areas)
                    .start_slot(
                        Icon::new(IconName::Plus)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new("Add Area").color(Color::Muted))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.show_add_areas = !this.show_add_areas;
                        cx.notify();
                    })),
            )
            .when(self.show_add_areas, |this| {
                if self.addable_areas.is_empty() {
                    this.child(
                        div().px_2().py_1().child(
                            Label::new("Every catalog Area is already installed.")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                } else {
                    this.children(self.addable_areas.iter().map(|manifest| {
                        let area_id = manifest.id.clone();
                        ListItem::new(ElementId::Name(SharedString::from(format!(
                            "breadpaper-install-area-{}",
                            manifest.id
                        ))))
                        .indent_level(1)
                        .indent_step_size(px(12.))
                        .child(Label::new(manifest.name.clone()).size(LabelSize::Small))
                        .when(!manifest.summary.is_empty(), |item| {
                            item.tooltip(Tooltip::text(manifest.summary.clone()))
                        })
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.install_area(area_id.clone(), cx);
                        }))
                    }))
                }
            })
    }

    fn render_non_vault(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_2()
            .child(Label::new("This folder isn't a BreadPaper vault.").color(Color::Muted))
            .child(
                Button::new("breadpaper-create-vault", "Create vault here").on_click(cx.listener(
                    |this, _, _window, cx| {
                        this.create_vault_here(cx);
                    },
                )),
            )
    }

    fn render_invalid(&self, error: &str) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_2()
            .child(Label::new("This vault's config couldn't be loaded.").color(Color::Muted))
            .child(
                Label::new(error.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Error),
            )
            .child(
                Label::new("Fix .breadpaper/config.toml and the panel will recover.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
    }
}

impl Render for TimelinePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match &self.vault_status {
            VaultStatus::Valid(_) => v_flex()
                .child(self.render_entries(cx))
                .child(self.render_areas_section(cx))
                .into_any_element(),
            VaultStatus::NotAVault => self.render_non_vault(cx).into_any_element(),
            VaultStatus::Invalid { error } => self.render_invalid(error).into_any_element(),
        };
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BreadPaperTimelinePanel");
        key_context.add("menu");
        v_flex()
            .id("breadpaper-timeline-panel")
            .key_context(key_context)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .size_full()
            .p_1()
            .pl_2()
            .child(content)
    }
}

impl EventEmitter<PanelEvent> for TimelinePanel {}

impl Focusable for TimelinePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for TimelinePanel {
    fn persistent_name() -> &'static str {
        "BreadPaper Timeline Panel"
    }

    fn panel_key() -> &'static str {
        TIMELINE_PANEL_KEY
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _window: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(240.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Notepad)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Timeline Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        ToggleFocus.boxed_clone()
    }

    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn activation_priority(&self) -> u32 {
        // Must be unique across all panels; 0-3 and 5-7 are taken upstream.
        4
    }
}

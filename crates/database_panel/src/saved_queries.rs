//! Named queries the user saved from a query console, persisted in the
//! key-value store, and a picker to reopen (and re-run) them.

use db::kvp::KeyValueStore;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString,
    Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use ui::{
    HighlightedLabel, ListItem, ListItemSpacing, Modal, ModalFooter, ModalHeader, Section, Tooltip,
    prelude::*,
};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::{ModalView, Workspace};

use crate::{DatabasePanelSettings, QueryConsole, resolve_connection};

const SAVED_QUERIES_KEY: &str = "database_panel_saved_queries::v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedQuery {
    pub name: String,
    /// Display name of the connection the query ran on, used to find the
    /// connection back when the query is reopened.
    pub connection: String,
    pub database: Option<String>,
    pub sql: String,
}

pub(crate) fn read_saved_queries(kvp: &KeyValueStore) -> Vec<SavedQuery> {
    kvp.read_kvp(SAVED_QUERIES_KEY)
        .log_err()
        .flatten()
        .and_then(|value| serde_json::from_str(&value).log_err())
        .unwrap_or_default()
}

pub(crate) async fn write_saved_queries(
    kvp: KeyValueStore,
    queries: &[SavedQuery],
) -> anyhow::Result<()> {
    let value = serde_json::to_string(queries)?;
    kvp.write_kvp(SAVED_QUERIES_KEY.to_string(), value).await
}

/// A modal asking for a name under which to save the console's current SQL.
/// Saving under an existing name replaces that query. When the console is
/// already linked to a saved query, the modal edits it: the name is prefilled
/// and confirming renames it and refreshes its SQL.
pub struct SaveQueryModal {
    console: WeakEntity<QueryConsole>,
    name: Entity<InputField>,
    /// The linked saved query's name when editing; renaming removes this
    /// entry and writes the new one.
    original_name: Option<String>,
    connection: String,
    database: Option<String>,
    sql: String,
    error: Option<SharedString>,
}

impl SaveQueryModal {
    pub fn toggle(
        workspace: &mut Workspace,
        console: WeakEntity<QueryConsole>,
        connection: String,
        database: Option<String>,
        sql: String,
        original_name: Option<String>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| {
            let name = cx.new(|cx| InputField::new(window, cx, "Query name").label("Name"));
            if let Some(original_name) = &original_name {
                name.update(cx, |field, cx| field.set_text(original_name, window, cx));
            }
            Self {
                console,
                name,
                original_name,
                connection,
                database,
                sql,
                error: None,
            }
        });
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.save(window, cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn save(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let name = self.name.read(cx).text(cx).trim().to_string();
        if name.is_empty() {
            self.error = Some("A name is required.".into());
            cx.notify();
            return;
        }
        let query = SavedQuery {
            name: name.clone(),
            connection: self.connection.clone(),
            database: self.database.clone(),
            sql: self.sql.clone(),
        };
        let original_name = self.original_name.clone();
        let kvp = KeyValueStore::global(cx);
        cx.background_spawn(async move {
            let mut queries = read_saved_queries(&kvp);
            if let Some(original_name) = original_name
                && original_name != query.name
            {
                queries.retain(|existing| existing.name != original_name);
            }
            match queries.iter_mut().find(|existing| existing.name == query.name) {
                Some(existing) => *existing = query,
                None => queries.push(query),
            }
            write_saved_queries(kvp, &queries).await
        })
        .detach_and_log_err(cx);
        self.console
            .update(cx, |console, cx| {
                console.set_saved_query_name(Some(name), cx);
            })
            .ok();
        cx.emit(DismissEvent);
    }
}

impl ModalView for SaveQueryModal {}

impl EventEmitter<DismissEvent> for SaveQueryModal {}

impl Focusable for SaveQueryModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.name.focus_handle(cx)
    }
}

impl Render for SaveQueryModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let target = match &self.database {
            Some(database) => format!("{} · {database}", self.connection),
            None => self.connection.clone(),
        };
        let headline = if self.original_name.is_some() {
            "Edit Saved Query"
        } else {
            "Save Query"
        };
        v_flex()
            .key_context("SaveQueryModal")
            .w(rems(30.))
            .elevation_3(cx)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .child(
                Modal::new("database-save-query-modal", None)
                    .header(ModalHeader::new().headline(headline))
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_2()
                                .child(
                                    Label::new(target)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(self.name.clone())
                                .when_some(self.error.clone(), |this, error| {
                                    this.child(
                                        Label::new(error)
                                            .size(LabelSize::Small)
                                            .color(Color::Error),
                                    )
                                }),
                        ),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_1()
                                .child(Button::new("cancel", "Cancel").on_click(cx.listener(
                                    |_, _, _, cx| {
                                        cx.emit(DismissEvent);
                                    },
                                )))
                                .child(
                                    Button::new("save", "Save")
                                        .tooltip(Tooltip::text(
                                            "Saving under an existing name replaces it",
                                        ))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.save(window, cx)
                                        })),
                                ),
                        ),
                    ),
            )
    }
}

/// A fuzzy picker over the saved queries; confirming opens a query console on
/// the query's connection and runs it immediately.
pub struct SavedQueryPicker {
    picker: Entity<Picker<SavedQueryPickerDelegate>>,
}

impl SavedQueryPicker {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let weak_workspace = workspace.weak_handle();
        workspace.toggle_modal(window, cx, |window, cx| {
            Self::new(weak_workspace, window, cx)
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = SavedQueryPickerDelegate {
            picker_view: cx.entity().downgrade(),
            workspace,
            queries: Vec::new(),
            matches: Vec::new(),
            selected_index: 0,
        };
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let kvp = KeyValueStore::global(cx);
        let load = cx.background_spawn(async move { read_saved_queries(&kvp) });
        cx.spawn_in(window, {
            let picker = picker.clone();
            async move |_, cx| {
                let queries = load.await;
                picker
                    .update_in(cx, |picker, window, cx| {
                        picker.delegate.queries = queries;
                        picker.refresh(window, cx);
                    })
                    .ok();
            }
        })
        .detach();
        Self { picker }
    }
}

impl ModalView for SavedQueryPicker {}

impl EventEmitter<DismissEvent> for SavedQueryPicker {}

impl Focusable for SavedQueryPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for SavedQueryPicker {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("SavedQueryPicker")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

pub struct SavedQueryPickerDelegate {
    picker_view: WeakEntity<SavedQueryPicker>,
    workspace: WeakEntity<Workspace>,
    queries: Vec<SavedQuery>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl SavedQueryPickerDelegate {
    /// Removes the query behind match `index` and persists the list. The
    /// caller refreshes the picker to rebuild the matches.
    fn delete_query(&mut self, index: usize, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(index) else {
            return;
        };
        if mat.candidate_id >= self.queries.len() {
            return;
        }
        self.queries.remove(mat.candidate_id);
        let queries = self.queries.clone();
        let kvp = KeyValueStore::global(cx);
        cx.background_spawn(async move { write_saved_queries(kvp, &queries).await })
            .detach_and_log_err(cx);
    }
}

impl PickerDelegate for SavedQueryPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "saved query picker"
    }

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> std::sync::Arc<str> {
        "Open a saved query…".into()
    }

    fn no_matches_text(&self, _: &mut Window, _: &mut App) -> Option<SharedString> {
        Some("No saved queries".into())
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, _: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let candidates: Vec<StringMatchCandidate> = self
            .queries
            .iter()
            .enumerate()
            .map(|(index, saved)| StringMatchCandidate::new(index, &saved.name))
            .collect();
        let background = cx.background_executor().clone();
        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .map(|candidate| StringMatch {
                        candidate_id: candidate.id,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };
            this.update(cx, |this, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index)
            && let Some(saved) = self.queries.get(mat.candidate_id).cloned()
        {
            let resolved = DatabasePanelSettings::get_global(cx)
                .connections
                .iter()
                .filter_map(resolve_connection)
                .find(|(name, _)| name.as_ref() == saved.connection);
            if let Some((name, config)) = resolved {
                self.workspace
                    .update(cx, |workspace, cx| {
                        QueryConsole::open_saved(
                            workspace,
                            name,
                            config,
                            saved.database.clone(),
                            saved.sql.clone(),
                            saved.name.clone(),
                            window,
                            cx,
                        );
                    })
                    .ok();
            }
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.picker_view
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let saved = self.queries.get(mat.candidate_id)?;
        let connection_known = DatabasePanelSettings::get_global(cx)
            .connections
            .iter()
            .filter_map(resolve_connection)
            .any(|(name, _)| name.as_ref() == saved.connection);
        let mut detail = match &saved.database {
            Some(database) => format!("{} · {database}", saved.connection),
            None => saved.connection.clone(),
        };
        if !connection_known {
            detail.push_str(" · connection not configured");
        }
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(
                    Icon::new(IconName::Bookmark)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .min_w_0()
                        .child(HighlightedLabel::new(
                            saved.name.clone(),
                            mat.positions.clone(),
                        ))
                        .child(
                            Label::new(detail)
                                .size(LabelSize::XSmall)
                                .color(if connection_known {
                                    Color::Muted
                                } else {
                                    Color::Error
                                })
                                .single_line()
                                .truncate(),
                        ),
                )
                .end_slot(
                    IconButton::new(("delete-saved-query", ix), IconName::Trash)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::text("Delete Saved Query"))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            window.prevent_default();
                            this.delegate.delete_query(ix, cx);
                            this.refresh(window, cx);
                        })),
                )
                .show_end_slot_on_hover(),
        )
    }
}

use std::ops::Range;

use collections::{HashMap, HashSet};
use editor::actions::{
    FindAllReferences, GoToDeclaration, GoToDefinition, GoToImplementation, GoToTypeDefinition,
};
use editor::{Editor, EditorSettings, GotoDefinitionKind, OpenResultsIn};
use file_icons::FileIcons;
use fuzzy::StringMatchCandidate;
use gpui::{
    Action, AnyElement, App, AppContext, AsyncWindowContext, ClickEvent, Context, DismissEvent,
    Entity, EventEmitter, FocusHandle, Focusable, HighlightStyle, ListSizingBehavior, Pixels,
    ScrollStrategy, StyledText, Subscription, Task, TextStyle, UniformListScrollHandle, WeakEntity,
    actions, div, prelude::*, uniform_list,
};
use language::{Buffer, HighlightId, LanguageAwareStyling};
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use picker::{Picker, PickerDelegate};
use project::{Location, Project, ProjectPath};
use settings::{GoToDefinitionFallback, Settings as _};
use text::{Anchor, Point};
use theme_settings::ThemeSettings;
use ui::{Divider, FluentBuilder};
use ui::{ListItem, ListItemSpacing, ScrollAxes, Scrollbars, WithScrollbar, prelude::*};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::item::ItemSettings;
use workspace::notifications::NotificationId;
use workspace::{ModalView, Toast, Workspace};

actions!(lsp_locations, [ToggleReferencesPanel]);

const REFERENCES_PANEL_KEY: &str = "ReferencesPanel";

pub fn init(cx: &mut App) {
    cx.observe_new(register).detach();
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleReferencesPanel, window, cx| {
            workspace.toggle_panel_focus::<LspLocationsPanel>(window, cx);
        });
    })
    .detach();
}

/// Registers handlers for the navigation actions on each full editor. When the
/// action resolves to [`OpenResultsIn::Picker`], we open the filterable picker;
/// otherwise we `cx.propagate()` so the editor's own handler runs and builds a
/// multibuffer.
fn register(editor: &mut Editor, _window: Option<&mut Window>, cx: &mut Context<Editor>) {
    if !editor.mode().is_full() {
        return;
    }
    let handle = cx.entity().downgrade();
    editor
        .register_action({
            let handle = handle.clone();
            move |action: &GoToDefinition, window, cx| {
                handle_nav_action(
                    action.open_results_in,
                    LspPickerKind::Definition,
                    &handle,
                    window,
                    cx,
                );
            }
        })
        .detach();
    editor
        .register_action({
            let handle = handle.clone();
            move |action: &GoToDeclaration, window, cx| {
                handle_nav_action(
                    action.open_results_in,
                    LspPickerKind::Declaration,
                    &handle,
                    window,
                    cx,
                );
            }
        })
        .detach();
    editor
        .register_action({
            let handle = handle.clone();
            move |action: &GoToImplementation, window, cx| {
                handle_nav_action(
                    action.open_results_in,
                    LspPickerKind::Implementation,
                    &handle,
                    window,
                    cx,
                );
            }
        })
        .detach();
    editor
        .register_action({
            let handle = handle.clone();
            move |action: &GoToTypeDefinition, window, cx| {
                handle_nav_action(
                    action.open_results_in,
                    LspPickerKind::TypeDefinition,
                    &handle,
                    window,
                    cx,
                );
            }
        })
        .detach();
    editor
        .register_action(move |action: &FindAllReferences, window, cx| {
            handle_nav_action(
                action.open_results_in,
                LspPickerKind::References,
                &handle,
                window,
                cx,
            );
        })
        .detach();
}

/// Either opens the picker or panel for the editor, or propagates the action so
/// the editor's built-in (multibuffer) handler runs. A `None` argument falls
/// back to the `lsp_results_location` setting. Only "find all references"
/// currently has a docked panel, the other kinds fall back to the picker.
fn handle_nav_action(
    open_results_in: Option<OpenResultsIn>,
    kind: LspPickerKind,
    editor: &WeakEntity<Editor>,
    window: &mut Window,
    cx: &mut App,
) {
    let open_results_in =
        open_results_in.unwrap_or_else(|| EditorSettings::get_global(cx).lsp_results_location);
    match open_results_in {
        OpenResultsIn::Picker => {
            LspLocationsPicker::open_for_editor(kind, editor.clone(), window, cx);
        }
        OpenResultsIn::Panel if kind == LspPickerKind::References => {
            LspLocationsPanel::open_for_editor(kind, editor.clone(), window, cx);
        }
        OpenResultsIn::Panel => {
            LspLocationsPicker::open_for_editor(kind, editor.clone(), window, cx);
        }
        OpenResultsIn::MultiBuffer => {
            cx.propagate();
        }
    }
}

/// Runs the LSP query for `kind` and returns the raw locations. Returns `None`
/// (and reports any error) when there is nothing to query, so the caller stops
/// without an empty-results toast. Deduplication and dropping fileless results
/// happen later in [`build_location_matches`].
async fn run_picker_query(
    kind: LspPickerKind,
    editor: &WeakEntity<Editor>,
    workspace: &WeakEntity<Workspace>,
    project: &Entity<Project>,
    cx: &mut AsyncWindowContext,
) -> Option<Vec<Location>> {
    let query = editor
        .update(cx, |editor, cx| kind.run_query(editor, project, cx))
        .ok()
        .flatten()?;
    match query.await {
        Ok(locations) => Some(locations),
        Err(error) => {
            log::error!("LSP {kind:?} query failed: {error:#}");
            workspace
                .update(cx, |workspace, cx| workspace.show_error(error, cx))
                .log_err();
            None
        }
    }
}

/// Runs the query for `kind` and builds the displayable, deduped matches.
async fn run_picker_matches(
    kind: LspPickerKind,
    editor: &WeakEntity<Editor>,
    workspace: &WeakEntity<Workspace>,
    project: &Entity<Project>,
    cx: &mut AsyncWindowContext,
) -> Option<Vec<LocationMatch>> {
    let locations = run_picker_query(kind, editor, workspace, project, cx).await?;
    editor
        .update(cx, |_, cx| build_location_matches(&locations, cx))
        .ok()
}

fn show_no_results_toast(
    workspace: &WeakEntity<Workspace>,
    kind: LspPickerKind,
    cx: &mut AsyncWindowContext,
) {
    workspace
        .update(cx, |workspace, cx| {
            struct NoLspResults;
            workspace.show_toast(
                Toast::new(
                    NotificationId::unique::<NoLspResults>(),
                    kind.empty_message(),
                )
                .autohide(),
                cx,
            );
        })
        .log_err();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LspPickerKind {
    References,
    Definition,
    Declaration,
    Implementation,
    TypeDefinition,
}

impl LspPickerKind {
    fn placeholder(self) -> &'static str {
        match self {
            LspPickerKind::References => "Filter references…",
            LspPickerKind::Definition => "Filter definitions…",
            LspPickerKind::Declaration => "Filter declarations…",
            LspPickerKind::Implementation => "Filter implementations…",
            LspPickerKind::TypeDefinition => "Filter type definitions…",
        }
    }

    /// Message shown when the query produces no results, so the command does not
    /// appear to silently do nothing.
    fn empty_message(self) -> &'static str {
        match self {
            LspPickerKind::References => "No references found",
            LspPickerKind::Definition => "No definitions found",
            LspPickerKind::Declaration => "No declarations found",
            LspPickerKind::Implementation => "No implementations found",
            LspPickerKind::TypeDefinition => "No type definitions found",
        }
    }

    /// Runs the query for this kind against the active editor, returning the raw
    /// locations to populate the picker.
    fn run_query(
        self,
        editor: &mut Editor,
        project: &Entity<Project>,
        cx: &mut Context<Editor>,
    ) -> Option<Task<anyhow::Result<Vec<Location>>>> {
        match self {
            LspPickerKind::References => editor.find_all_references_locations(project, cx),
            LspPickerKind::Definition => {
                editor.definition_locations_of_kind(GotoDefinitionKind::Symbol, cx)
            }
            LspPickerKind::Declaration => {
                editor.definition_locations_of_kind(GotoDefinitionKind::Declaration, cx)
            }
            LspPickerKind::Implementation => {
                editor.definition_locations_of_kind(GotoDefinitionKind::Implementation, cx)
            }
            LspPickerKind::TypeDefinition => {
                editor.definition_locations_of_kind(GotoDefinitionKind::Type, cx)
            }
        }
    }
}

pub struct LspLocationsPicker {
    picker: Entity<Picker<LspLocationsDelegate>>,
    _subscription: Subscription,
}

impl LspLocationsPicker {
    fn open_for_editor(
        kind: LspPickerKind,
        editor: WeakEntity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(editor) = editor.upgrade() else {
            return;
        };
        let Some(workspace) = editor.read(cx).workspace() else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            Self::open(kind, editor, workspace, window, cx);
        });
    }

    /// Opens the picker for `kind`: runs a fresh LSP query and shows the
    /// results. An empty definitions query falls back to references when the
    /// `go_to_definition_fallback` setting calls for it, matching
    /// [`Editor::go_to_definition`].
    fn open(
        kind: LspPickerKind,
        editor: Entity<Editor>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let project = workspace.project().clone();
        let fallback = EditorSettings::get_global(cx).go_to_definition_fallback;
        let editor = editor.downgrade();
        cx.spawn_in(window, async move |workspace, cx| {
            // The kind the user invoked, kept for user-facing messages even if
            // the query below falls back to references.
            let invoked_kind = kind;
            let mut kind = kind;

            // Count on the built matches (not raw locations): they are deduped by
            // range and exclude fileless results, so a single distinct result
            // jumps directly and a fileless-only result reports "no results"
            // instead of opening a blank picker.
            let Some(mut matches) =
                run_picker_matches(kind, &editor, &workspace, &project, cx).await
            else {
                return;
            };

            if matches.is_empty()
                && kind == LspPickerKind::Definition
                && fallback == GoToDefinitionFallback::FindAllReferences
            {
                kind = LspPickerKind::References;
                let Some(references) =
                    run_picker_matches(kind, &editor, &workspace, &project, cx).await
                else {
                    return;
                };
                matches = references;
            }

            if matches.is_empty() {
                show_no_results_toast(&workspace, invoked_kind, cx);
                return;
            }

            if matches.len() == 1 {
                if let Some(location_match) = matches.into_iter().next() {
                    let location = Location {
                        buffer: location_match.buffer,
                        range: location_match.anchor_range,
                    };
                    if let Ok(task) = editor.update_in(cx, |editor, window, cx| {
                        editor.open_location(location, false, window, cx)
                    }) {
                        task.await.log_err();
                    }
                }
                return;
            }

            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        Self::new(kind, matches, project, editor, window, cx)
                    });
                })
                .log_err();
        })
        .detach();
    }

    fn new(
        kind: LspPickerKind,
        matches: Vec<LocationMatch>,
        project: Entity<Project>,
        editor: WeakEntity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let preview = picker_preview::editor_preview(project.clone(), window, cx);
        let delegate = LspLocationsDelegate::new(kind, matches, project, editor);
        let picker = cx.new(|cx| Picker::list_with_preview(delegate, preview, window, cx));
        let subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });
        Self {
            picker,
            _subscription: subscription,
        }
    }
}

impl ModalView for LspLocationsPicker {}

impl EventEmitter<DismissEvent> for LspLocationsPicker {}

impl Focusable for LspLocationsPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for LspLocationsPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

/// A docked panel showing LSP results (currently "find all references") as a
/// filterable, navigable list
pub struct LspLocationsPanel {
    delegate: LspLocationsDelegate,
    scroll_handle: UniformListScrollHandle,
    focus_handle: FocusHandle,
}

impl LspLocationsPanel {
    fn open_for_editor(
        kind: LspPickerKind,
        editor: WeakEntity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(editor) = editor.upgrade() else {
            return;
        };
        let Some(workspace) = editor.read(cx).workspace() else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            Self::open(kind, editor, workspace, window, cx);
        });
    }

    /// Opens the panel for `kind`: reveals it, runs the LSP query, then fills
    /// it with the results.
    fn open(
        kind: LspPickerKind,
        editor: Entity<Editor>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let project = workspace.project().clone();
        let editor = editor.downgrade();
        cx.spawn_in(window, async move |workspace, cx| {
            //  Reveal the panel so the user sees it while the query runs.
            let Some(panel) = workspace
                .update_in(cx, |workspace, window, cx| {
                    let panel = match workspace.panel::<LspLocationsPanel>(cx) {
                        Some(panel) => {
                            panel.update(cx, |panel, cx| {
                                panel.reset(kind, project.clone(), editor.clone(), cx);
                            });
                            panel
                        }
                        None => {
                            let panel = LspLocationsPanel::new(
                                kind,
                                project.clone(),
                                editor.clone(),
                                window,
                                cx,
                            );
                            workspace.add_panel(panel.clone(), window, cx);
                            panel
                        }
                    };
                    workspace.reveal_panel::<LspLocationsPanel>(window, cx);
                    panel.update(cx, |panel, cx| {
                        panel.focus_handle.focus(window, cx);
                    });
                    panel
                })
                .log_err()
            else {
                return;
            };

            // Run the LSP query.
            let Some(locations) = run_picker_query(kind, &editor, &workspace, &project, cx).await
            else {
                return;
            };

            if locations.is_empty() {
                return;
            }

            // Add all matches.
            panel
                .update_in(cx, |panel, _window, cx| {
                    let matches = build_location_matches(&locations, cx);
                    panel.delegate.append_matches(matches);
                    panel.scroll_to_selected();
                    cx.notify();
                })
                .log_err();

            let total = panel.update(cx, |panel, _cx| panel.delegate.all_matches.len());
            if total == 1 {
                let location = panel.update(cx, |panel, _cx| {
                    let location_match = &panel.delegate.all_matches[0];
                    Location {
                        buffer: location_match.buffer.clone(),
                        range: location_match.anchor_range.clone(),
                    }
                });
                if let Ok(task) = editor.update_in(cx, |editor, window, cx| {
                    editor.open_location(location, false, window, cx)
                }) {
                    task.await.log_err();
                }
            }
        })
        .detach();
    }

    /// Creates the panel empty, ready for the LSP results to be added.
    fn new(
        kind: LspPickerKind,
        project: Entity<Project>,
        editor: WeakEntity<Editor>,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            delegate: LspLocationsDelegate::empty_tree(kind, project, editor),
            scroll_handle: UniformListScrollHandle::new(),
            focus_handle: cx.focus_handle(),
        })
    }

    /// Resets the panel to an empty state for a fresh query.
    fn reset(
        &mut self,
        kind: LspPickerKind,
        project: Entity<Project>,
        editor: WeakEntity<Editor>,
        cx: &mut Context<Self>,
    ) {
        self.delegate = LspLocationsDelegate::empty_tree(kind, project, editor);
        self.scroll_to_selected();
        cx.notify();
    }

    fn selectable(&self, ix: usize) -> bool {
        self.delegate
            .entries
            .get(ix)
            .is_some_and(|entry| self.delegate.entry_is_selectable(entry))
    }

    fn select_next(&mut self) {
        let count = self.delegate.entries.len();
        let mut ix = self.delegate.selected_index + 1;
        while ix < count && !self.selectable(ix) {
            ix += 1;
        }
        if ix < count {
            self.delegate.selected_index = ix;
        }
    }

    fn select_previous(&mut self) {
        let mut ix = self.delegate.selected_index;
        while ix > 0 {
            ix -= 1;
            if self.selectable(ix) {
                self.delegate.selected_index = ix;
                break;
            }
        }
    }

    fn select_first(&mut self) {
        if let Some(ix) = self.delegate.first_selectable_index() {
            self.delegate.selected_index = ix;
        }
    }

    fn select_last(&mut self) {
        for ix in (0..self.delegate.entries.len()).rev() {
            if self.selectable(ix) {
                self.delegate.selected_index = ix;
                return;
            }
        }
    }

    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.delegate.entries.get(self.delegate.selected_index) {
            Some(Entry::Header(path)) => {
                self.delegate.toggle_collapsed(path.clone());
                self.scroll_to_selected();
                cx.notify();
            }
            Some(Entry::Match(_)) => self.open_selected(false, window, cx),
            _ => {}
        }
    }

    fn click_row(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.delegate.selected_index = ix;
        self.confirm(window, cx);
    }

    fn open_selected(&mut self, split: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(location_match) = self.delegate.selected_location_match() else {
            return;
        };
        let location = Location {
            buffer: location_match.buffer.clone(),
            range: location_match.anchor_range.clone(),
        };
        let Some(editor) = self.delegate.editor.upgrade() else {
            return;
        };
        editor
            .update(cx, |editor, cx| {
                editor.open_location(location, split, window, cx)
            })
            .detach_and_log_err(cx);
    }

    fn scroll_to_selected(&self) {
        self.scroll_handle
            .scroll_to_item(self.delegate.selected_index, ScrollStrategy::Nearest);
    }

    fn confirm_action(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.confirm(window, cx);
    }

    fn select_next_action(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        self.select_next();
        self.scroll_to_selected();
        cx.notify();
    }

    fn select_previous_action(
        &mut self,
        _: &SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_previous();
        self.scroll_to_selected();
        cx.notify();
    }

    fn select_first_action(
        &mut self,
        _: &SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_first();
        self.scroll_to_selected();
        cx.notify();
    }

    fn select_last_action(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        self.select_last();
        self.scroll_to_selected();
        cx.notify();
    }

    fn render_entry(&self, ix: usize, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.delegate.entries.get(ix) {
            Some(Entry::Header(path)) => {
                let selected = ix == self.delegate.selected_index;
                let collapsed = self.delegate.is_collapsed(path);
                let path_style = self.delegate.project.read(cx).path_style(cx);
                let file_name = path
                    .path
                    .file_name()
                    .map(|name| name.to_string())
                    .unwrap_or_default();
                let directory = path
                    .path
                    .parent()
                    .map(|parent| parent.display(path_style))
                    .map(SharedString::new)
                    .unwrap_or_default();
                let file_icon = ItemSettings::get_global(cx)
                    .file_icons
                    .then(|| FileIcons::get_icon(path.path.as_std_path(), cx))
                    .flatten()
                    .map(|icon| {
                        Icon::from_path(icon)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    });
                ListItem::new(ix)
                    .spacing(ListItemSpacing::Sparse)
                    .inset(true)
                    .toggle_state(selected)
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        this.click_row(ix, window, cx);
                    }))
                    .child(
                        h_flex()
                            .w_full()
                            .min_w_0()
                            .gap_1p5()
                            .child(
                                Icon::new(if collapsed {
                                    IconName::ChevronRight
                                } else {
                                    IconName::ChevronDown
                                })
                                .size(IconSize::Small)
                                .color(Color::Muted),
                            )
                            .children(file_icon)
                            .child(
                                h_flex()
                                    .w_full()
                                    .min_w_0()
                                    .gap_1()
                                    .child(Label::new(file_name).size(LabelSize::Small))
                                    .when(!directory.is_empty(), |this| {
                                        this.child(
                                            Label::new(directory)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted)
                                                .truncate_start(),
                                        )
                                    }),
                            ),
                    )
                    .into_any_element()
            }
            Some(Entry::Match(match_index)) => {
                let selected = ix == self.delegate.selected_index;
                let location_match = &self.delegate.all_matches[*match_index];
                ListItem::new(ix)
                    .spacing(ListItemSpacing::Sparse)
                    .inset(true)
                    .toggle_state(selected)
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        this.click_row(ix, window, cx);
                    }))
                    .child(
                        h_flex()
                            .w_full()
                            .min_w_0()
                            .gap_2p5()
                            .text_sm()
                            .child(
                                h_flex()
                                    .w(rems(
                                        (self.delegate.max_line_number.max(1).ilog10() + 1) as f32
                                            * 0.5,
                                    ))
                                    .justify_end()
                                    .child(
                                        Label::new(location_match.line_number.to_string()).color(
                                            Color::Custom(
                                                cx.theme().colors().text_muted.opacity(0.5),
                                            ),
                                        ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .truncate()
                                    .child(render_matched_line(location_match, cx)),
                            ),
                    )
                    .into_any_element()
            }
            _ => div().into_any_element(),
        }
    }

    fn render_header(&self, cx: &Context<Self>) -> Div {
        let (results, files) = self.delegate.summary();
        let text = if results == 0 {
            self.delegate.kind.empty_message().to_string()
        } else {
            let results_word = if results == 1 { "result" } else { "results" };
            let files_word = if files == 1 { "file" } else { "files" };
            format!("{results} {results_word} in {files} {files_word}")
        };
        h_flex()
            .w_full()
            .px(DynamicSpacing::Base06.rems(cx))
            .py_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(Label::new(text).size(LabelSize::Small).color(Color::Muted))
    }
}

impl Focusable for LspLocationsPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for LspLocationsPanel {}

impl Panel for LspLocationsPanel {
    fn persistent_name() -> &'static str {
        "References Panel"
    }

    fn panel_key() -> &'static str {
        REFERENCES_PANEL_KEY
    }

    fn activation_focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle(cx)
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(320.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::ArrowRightLeft)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("References Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleReferencesPanel)
    }

    fn activation_priority(&self) -> u32 {
        8
    }
}

impl Render for LspLocationsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = self.render_header(cx);
        let list = uniform_list(
            "references",
            self.delegate.entries.len(),
            cx.processor(move |panel: &mut Self, range: Range<usize>, window, cx| {
                range.map(|ix| panel.render_entry(ix, window, cx)).collect()
            }),
        )
        .with_sizing_behavior(ListSizingBehavior::Auto)
        .flex_grow_1()
        .min_h_0()
        .track_scroll(&self.scroll_handle)
        .into_any_element();

        v_flex()
            .size_full()
            .key_context("ReferencesPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::confirm_action))
            .on_action(cx.listener(Self::select_next_action))
            .on_action(cx.listener(Self::select_previous_action))
            .on_action(cx.listener(Self::select_first_action))
            .on_action(cx.listener(Self::select_last_action))
            .child(header)
            .child(list)
            .custom_scrollbars(
                Scrollbars::new(ScrollAxes::Vertical).tracked_scroll_handle(&self.scroll_handle),
                window,
                cx,
            )
    }
}

struct LocationMatch {
    path: ProjectPath,
    buffer: Entity<Buffer>,
    anchor_range: Range<Anchor>,
    range: Range<usize>,
    display_text: String,
    syntax_highlights: Vec<(Range<usize>, HighlightId)>,
    match_range: Range<usize>,
    line_number: u32,
}

/// A row in the grouped display list: a non-selectable file header, a match, or
/// a separator between file groups. `selected_index` indexes into this list.
enum Entry {
    Header(ProjectPath),
    Match(usize),
    Separator,
}

struct LspLocationsDelegate {
    kind: LspPickerKind,
    project: Entity<Project>,
    editor: WeakEntity<Editor>,
    all_matches: Vec<LocationMatch>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<usize>,
    entries: Vec<Entry>,
    selected_index: usize,
    max_line_number: u32,
    /// When true, files collapse/expand.
    collapse_enabled: bool,
    collapsed_paths: HashSet<ProjectPath>,
}

impl LspLocationsDelegate {
    fn new(
        kind: LspPickerKind,
        all_matches: Vec<LocationMatch>,
        project: Entity<Project>,
        editor: WeakEntity<Editor>,
    ) -> Self {
        Self::with_collapse(kind, all_matches, project, editor, false)
    }

    fn empty_tree(
        kind: LspPickerKind,
        project: Entity<Project>,
        editor: WeakEntity<Editor>,
    ) -> Self {
        Self::with_collapse(kind, Vec::new(), project, editor, true)
    }

    fn with_collapse(
        kind: LspPickerKind,
        all_matches: Vec<LocationMatch>,
        project: Entity<Project>,
        editor: WeakEntity<Editor>,
        collapse_enabled: bool,
    ) -> Self {
        // Match against the line text and the file path, mirroring the fuzzy
        // matching every other Zed picker uses.
        let candidates = all_matches
            .iter()
            .enumerate()
            .map(|(index, location_match)| {
                StringMatchCandidate::new(
                    index,
                    &format!(
                        "{} {}",
                        location_match.display_text,
                        location_match.path.path.as_unix_str()
                    ),
                )
            })
            .collect();
        let matches = (0..all_matches.len()).collect();
        let mut this = Self {
            kind,
            project,
            editor,
            all_matches,
            candidates,
            matches,
            entries: Vec::new(),
            selected_index: 0,
            max_line_number: 0,
            collapse_enabled,
            collapsed_paths: HashSet::default(),
        };
        if collapse_enabled {
            // Start collapsed so the panel reads as a list of files until the
            // user expands one.
            this.collapsed_paths = this.all_matches.iter().map(|m| m.path.clone()).collect();
        }
        this.rebuild_entries();
        this
    }

    /// Appends a batch of freshly-built matches, used for progressive loading.
    fn append_matches(&mut self, new_matches: Vec<LocationMatch>) {
        if new_matches.is_empty() {
            return;
        }
        let new_count = new_matches.len();
        let base = self.all_matches.len();
        for (index, location_match) in new_matches.iter().enumerate() {
            self.candidates.push(StringMatchCandidate::new(
                base + index,
                &format!(
                    "{} {}",
                    location_match.display_text,
                    location_match.path.path.as_unix_str()
                ),
            ));
            if self.collapse_enabled {
                self.collapsed_paths.insert(location_match.path.clone());
            }
        }
        self.all_matches.extend(new_matches);
        self.matches.extend(base..base + new_count);
        self.rebuild_entries();
    }

    /// The number of (filtered) results and the number of distinct files among
    /// them, used for the "N results in M files" panel header.
    fn summary(&self) -> (usize, usize) {
        let mut files = HashSet::default();
        for &match_index in &self.matches {
            files.insert(&self.all_matches[match_index].path);
        }
        (self.matches.len(), files.len())
    }

    /// Rebuilds the grouped [`Self::entries`] from the filtered [`Self::matches`]:
    /// one header per file, its (expanded) matches, and a separator before every
    /// group after the first (only for the flat modal picker). The collapsible
    /// panel omits separators so its uniform list rows share a height.
    fn rebuild_entries(&mut self) {
        let mut entries = Vec::with_capacity(self.matches.len());
        let mut last_path: Option<&ProjectPath> = None;
        let mut max_line_number = 0;
        for &match_index in &self.matches {
            let location_match = &self.all_matches[match_index];
            if last_path != Some(&location_match.path) {
                if last_path.is_some() && !self.collapse_enabled {
                    entries.push(Entry::Separator);
                }
                entries.push(Entry::Header(location_match.path.clone()));
                last_path = Some(&location_match.path);
            }
            max_line_number = max_line_number.max(location_match.line_number);
            if !(self.collapse_enabled && self.is_collapsed(&location_match.path)) {
                entries.push(Entry::Match(match_index));
            }
        }
        self.entries = entries;
        self.max_line_number = max_line_number;
        self.selected_index = self.first_selectable_index().unwrap_or(0);
    }

    fn is_collapsed(&self, path: &ProjectPath) -> bool {
        self.collapse_enabled && self.collapsed_paths.contains(path)
    }

    fn toggle_collapsed(&mut self, path: ProjectPath) {
        if !self.collapsed_paths.remove(&path) {
            self.collapsed_paths.insert(path.clone());
        }
        self.rebuild_entries();
        self.selected_index = self
            .entries
            .iter()
            .position(|entry| matches!(entry, Entry::Header(p) if *p == path))
            .unwrap_or_else(|| self.first_selectable_index().unwrap_or(0));
    }

    fn first_selectable_index(&self) -> Option<usize> {
        self.entries
            .iter()
            .position(|entry| self.entry_is_selectable(entry))
    }

    fn entry_is_selectable(&self, entry: &Entry) -> bool {
        match entry {
            Entry::Match(_) => true,
            Entry::Header(_) => self.collapse_enabled,
            Entry::Separator => false,
        }
    }

    fn selected_location_match(&self) -> Option<&LocationMatch> {
        match self.entries.get(self.selected_index)? {
            Entry::Match(match_index) => self.all_matches.get(*match_index),
            Entry::Header(_) | Entry::Separator => None,
        }
    }

    fn open_selected(&mut self, split: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(location_match) = self.selected_location_match() else {
            return;
        };
        let location = Location {
            buffer: location_match.buffer.clone(),
            range: location_match.anchor_range.clone(),
        };
        let Some(editor) = self.editor.upgrade() else {
            return;
        };
        editor
            .update(cx, |editor, cx| {
                editor.open_location(location, split, window, cx)
            })
            .detach_and_log_err(cx);
        cx.emit(DismissEvent);
    }
}

fn build_location_matches(locations: &[Location], cx: &App) -> Vec<LocationMatch> {
    use gpui::EntityId;
    let mut snapshots: HashMap<EntityId, language::BufferSnapshot> = HashMap::default();
    let mut matches = Vec::with_capacity(locations.len());

    for location in locations {
        let snapshot = snapshots
            .entry(location.buffer.entity_id())
            .or_insert_with(|| location.buffer.read(cx).snapshot());

        let Some(file) = snapshot.file() else {
            continue;
        };
        let path = ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        };

        let start_offset: usize = snapshot.summary_for_anchor(&location.range.start);
        let end_offset: usize = snapshot.summary_for_anchor(&location.range.end);
        let row = snapshot.offset_to_point(start_offset).row;
        let line_start = snapshot.point_to_offset(Point::new(row, 0));
        let line_end = snapshot.point_to_offset(Point::new(row, snapshot.line_len(row)));
        let full_line: String = snapshot.text_for_range(line_start..line_end).collect();

        // The row shows the line with leading indentation trimmed. Offsets below
        // are relative to that displayed text.
        let display_text = full_line.trim_start().to_string();
        let visible_start = line_end.saturating_sub(display_text.len());
        let visible_end = line_end;

        // Precompute syntax highlights for the displayed text so rendering a row
        // never re-snapshots the buffer or re-runs highlighting.
        let mut syntax_highlights = Vec::new();
        let mut offset = 0;
        for chunk in snapshot.chunks(
            visible_start..visible_end,
            LanguageAwareStyling {
                tree_sitter: true,
                diagnostics: false,
            },
        ) {
            let chunk_len = chunk.text.len();
            if let Some(id) = chunk.syntax_highlight_id {
                syntax_highlights.push((offset..offset + chunk_len, id));
            }
            offset += chunk_len;
        }

        // The match span, clamped into the displayed text. `clamp` bounds each
        // endpoint to the line; `min`/`max` then keep the range well-ordered even
        // for a malformed/inverted LSP range (clamping alone preserves bounds but
        // not `start <= end`).
        let clamped_start = start_offset.clamp(visible_start, visible_end) - visible_start;
        let clamped_end = end_offset.clamp(visible_start, visible_end) - visible_start;
        let match_range = clamped_start.min(clamped_end)..clamped_start.max(clamped_end);

        matches.push(LocationMatch {
            path,
            buffer: location.buffer.clone(),
            anchor_range: location.range.clone(),
            range: start_offset..end_offset,
            display_text,
            syntax_highlights,
            match_range,
            line_number: row + 1,
        });
    }

    // Group by file and order by position so the grouped display list is stable,
    // then drop exact-duplicate ranges a server may report more than once.
    matches.sort_by(|a, b| a.path.cmp(&b.path).then(a.range.start.cmp(&b.range.start)));
    matches.dedup_by(|a, b| a.path == b.path && a.range == b.range);
    matches
}

impl PickerDelegate for LspLocationsDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "lsp locations picker"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        self.kind.placeholder().into()
    }

    fn match_count(&self) -> usize {
        self.entries.len()
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        matches!(self.entries.get(ix), Some(Entry::Match(_)))
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn select_on_hover(&self) -> bool {
        false
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query = query.trim().to_owned();
        let candidates = self.candidates.clone();
        cx.spawn(async move |picker, cx| {
            let matches = if query.is_empty() {
                (0..candidates.len()).collect()
            } else {
                let string_matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    candidates.len(),
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await;
                let mut indices = string_matches
                    .into_iter()
                    .map(|string_match| string_match.candidate_id)
                    .collect::<Vec<_>>();
                // Restore the file-grouped, positional order (fuzzy returns by score).
                indices.sort_unstable();
                indices
            };
            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.rebuild_entries();
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.open_selected(secondary, window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn try_get_preview_data_for_match(&self, _cx: &App) -> Option<picker::PreviewUpdate> {
        let location_match = self.selected_location_match()?;
        Some(picker::PreviewUpdate::from_buffer(
            location_match.buffer.clone(),
            picker::MatchLocation {
                anchor_range: location_match.anchor_range.clone(),
                range: location_match.range.clone(),
            },
        ))
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.entries.get(ix)? {
            Entry::Separator => Some(
                div()
                    .py(DynamicSpacing::Base04.rems(cx))
                    .child(Divider::horizontal())
                    .into_any_element(),
            ),
            Entry::Header(path) => {
                let path_style = self.project.read(cx).path_style(cx);
                let file_name = path
                    .path
                    .file_name()
                    .map(|name| name.to_string())
                    .unwrap_or_default();
                let directory = path
                    .path
                    .parent()
                    .map(|parent| parent.display(path_style))
                    .map(SharedString::new)
                    .unwrap_or_default();
                let file_icon = ItemSettings::get_global(cx)
                    .file_icons
                    .then(|| FileIcons::get_icon(path.path.as_std_path(), cx))
                    .flatten()
                    .map(|icon| {
                        Icon::from_path(icon)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    });

                Some(
                    h_flex()
                        .w_full()
                        .min_w_0()
                        .px(DynamicSpacing::Base06.rems(cx))
                        .py_1()
                        .gap_1p5()
                        .children(file_icon)
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Label::new(file_name).size(LabelSize::Small))
                                .when(!directory.is_empty(), |this| {
                                    this.child(
                                        Label::new(directory)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .truncate_start(),
                                    )
                                }),
                        )
                        .into_any_element(),
                )
            }
            Entry::Match(match_index) => {
                let location_match = self.all_matches.get(*match_index)?;
                Some(
                    ListItem::new(ix)
                        .spacing(ListItemSpacing::Sparse)
                        .inset(true)
                        .toggle_state(selected)
                        .child(
                            h_flex()
                                .w_full()
                                .min_w_0()
                                .gap_2p5()
                                .text_sm()
                                .child(
                                    h_flex()
                                        .w(rems(
                                            (self.max_line_number.max(1).ilog10() + 1) as f32 * 0.5,
                                        ))
                                        .justify_end()
                                        .child(
                                            Label::new(location_match.line_number.to_string())
                                                .color(Color::Custom(
                                                    cx.theme().colors().text_muted.opacity(0.5),
                                                )),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .truncate()
                                        .child(render_matched_line(location_match, cx)),
                                ),
                        )
                        .into_any_element(),
                )
            }
        }
    }
}

/// Renders the precomputed displayed line, resolving the stored syntax highlight
/// ids against the current theme and overlaying the match with a highlighted
/// background and bold weight.
fn render_matched_line(location_match: &LocationMatch, cx: &App) -> StyledText {
    let settings = ThemeSettings::get_global(cx);
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.),
        ..Default::default()
    };

    let syntax_theme = cx.theme().syntax();
    let syntax_highlights = location_match
        .syntax_highlights
        .iter()
        .filter_map(|(range, id)| Some((range.clone(), syntax_theme.get(*id).copied()?)))
        .collect::<Vec<_>>();

    let match_style = HighlightStyle {
        background_color: Some(cx.theme().colors().search_match_background),
        font_weight: Some(gpui::FontWeight::BOLD),
        ..Default::default()
    };
    let match_highlight = (location_match.match_range.clone(), match_style);

    let highlights = gpui::combine_highlights(syntax_highlights, [match_highlight]);
    StyledText::new(location_match.display_text.clone())
        .with_default_highlights(&text_style, highlights)
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::test::editor_lsp_test_context::EditorLspTestContext;
    use gpui::TestAppContext;
    use indoc::indoc;
    use workspace::Item as _;

    async fn rust_cx(
        capabilities: lsp::ServerCapabilities,
        cx: &mut TestAppContext,
    ) -> EditorLspTestContext {
        EditorLspTestContext::new_rust(capabilities, cx).await
    }

    fn open(cx: &mut EditorLspTestContext, kind: LspPickerKind) {
        let editor = cx.editor.clone();
        let workspace = cx.workspace.clone();
        cx.update(|window, cx| {
            workspace.update(cx, |workspace, cx| {
                LspLocationsPicker::open(kind, editor, workspace, window, cx);
            });
        });
        cx.run_until_parked();
    }

    fn open_panel(cx: &mut EditorLspTestContext, kind: LspPickerKind) {
        let editor = cx.editor.clone();
        let workspace = cx.workspace.clone();
        cx.update(|window, cx| {
            workspace.update(cx, |workspace, cx| {
                LspLocationsPanel::open(kind, editor, workspace, window, cx);
            });
        });
        cx.run_until_parked();
    }

    fn active_picker(cx: &mut EditorLspTestContext) -> Option<Entity<LspLocationsPicker>> {
        let workspace = cx.workspace.clone();
        cx.update(|_window, cx| workspace.read(cx).active_modal::<LspLocationsPicker>(cx))
    }

    fn active_panel(cx: &mut EditorLspTestContext) -> Option<Entity<LspLocationsPanel>> {
        let workspace = cx.workspace.clone();
        cx.update(|_window, cx| workspace.read(cx).panel::<LspLocationsPanel>(cx))
    }

    fn references(uri: lsp::Uri, ranges: &[(u32, u32, u32)]) -> Vec<lsp::Location> {
        ranges
            .iter()
            .map(|&(row, start, end)| lsp::Location {
                uri: uri.clone(),
                range: lsp::Range::new(
                    lsp::Position::new(row, start),
                    lsp::Position::new(row, end),
                ),
            })
            .collect()
    }

    const SOURCE: &str = indoc! {r#"
        fn main() {
            let aˇbc = 123;
            let xyz = abc;
        }
    "#};

    #[gpui::test]
    async fn test_multiple_references_open_picker(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        open(&mut cx, LspPickerKind::References);

        assert!(
            active_picker(&mut cx).is_some(),
            "multiple references should open the picker"
        );
    }

    #[gpui::test]
    async fn test_single_result_jumps_without_picker(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(2, 14, 17)])))
            });

        open(&mut cx, LspPickerKind::References);

        assert!(
            active_picker(&mut cx).is_none(),
            "a single result should jump directly instead of opening the picker"
        );
        // The lone result at row 2 should be selected directly, moving the
        // cursor off its starting position on row 1.
        cx.assert_editor_state(indoc! {r#"
            fn main() {
                let abc = 123;
                let xyz = «abcˇ»;
            }
        "#});
    }

    #[gpui::test]
    async fn test_no_results_does_not_open_picker(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |_params, _| {
                Ok(Some(Vec::new()))
            });

        open(&mut cx, LspPickerKind::References);

        assert!(
            active_picker(&mut cx).is_none(),
            "an empty result should not open the picker"
        );
    }

    #[gpui::test]
    async fn test_multiple_references_open_panel(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        open_panel(&mut cx, LspPickerKind::References);

        assert!(
            active_panel(&mut cx).is_some(),
            "multiple references should open the docked panel"
        );
    }

    #[gpui::test]
    async fn test_find_all_references_honors_lsp_results_location_panel(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.update(|_window, cx| {
            cx.update_global::<settings::SettingsStore, _>(|settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.editor.lsp_results_location = Some(OpenResultsIn::Panel);
                });
            });
        });
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        cx.dispatch_action(FindAllReferences::default());
        cx.run_until_parked();

        assert!(
            active_panel(&mut cx).is_some(),
            "FindAllReferences should open the panel when lsp_results_location is panel"
        );
    }

    #[gpui::test]
    async fn test_panel_starts_with_files_collapsed(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        open_panel(&mut cx, LspPickerKind::References);
        let panel = active_panel(&mut cx).expect("multiple references should open the panel");

        cx.update(|_window, cx| {
            let delegate = &panel.read(cx).delegate;
            // Files start collapsed, so the only entry is the file header; the
            // match rows are hidden until the file is expanded.
            assert_eq!(
                delegate.entries.len(),
                1,
                "the panel should start collapsed, showing only file headers"
            );
            assert!(matches!(delegate.entries[0], Entry::Header(_)));
        });
    }

    #[gpui::test]
    async fn test_panel_expand_reveals_matches(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        open_panel(&mut cx, LspPickerKind::References);
        let panel = active_panel(&mut cx).expect("multiple references should open the panel");

        cx.update(|_window, cx| {
            panel.update(cx, |panel, _cx| {
                let path = panel.delegate.all_matches[0].path.clone();
                panel.delegate.toggle_collapsed(path);
                assert_eq!(
                    panel.delegate.entries.len(),
                    3,
                    "expanding the file should reveal its reference rows"
                );
                assert!(matches!(panel.delegate.entries[0], Entry::Header(_)));
                assert!(matches!(panel.delegate.entries[1], Entry::Match(_)));
                assert!(matches!(panel.delegate.entries[2], Entry::Match(_)));
            });
        });
    }

    #[gpui::test]
    async fn test_definition_falls_back_to_references_picker(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                definition_provider: Some(lsp::OneOf::Left(true)),
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::GotoDefinition, _, _>(async move |_params, _| {
                Ok(None)
            });
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        open(&mut cx, LspPickerKind::Definition);

        assert!(
            active_picker(&mut cx).is_some(),
            "an empty definition query should fall back to the references picker"
        );
    }

    #[gpui::test]
    async fn test_fuzzy_filter_matches_subsequence(cx: &mut TestAppContext) {
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        open(&mut cx, LspPickerKind::References);
        let modal = active_picker(&mut cx).expect("multiple references should open the picker");
        let picker = cx.update(|_window, cx| modal.read(cx).picker.clone());

        let matches = |cx: &mut EditorLspTestContext, query: &str| -> usize {
            cx.update(|window, cx| {
                picker.update(cx, |picker, cx| picker.set_query(query, window, cx));
            });
            cx.run_until_parked();
            cx.update(|_window, cx| picker.read(cx).delegate.matches.len())
        };

        // "lx" is a subsequence of "let xyz" but not a substring of either line,
        // so it only matches with fuzzy matching.
        assert_eq!(matches(&mut cx, "lx"), 1);
        assert_eq!(matches(&mut cx, "zzzz"), 0);
        assert_eq!(matches(&mut cx, ""), 2);
    }

    #[gpui::test]
    async fn test_cmd_click_fallback_honors_lsp_results_location(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                definition_provider: Some(lsp::OneOf::Left(true)),
                references_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.update(|_window, cx| {
            cx.update_global::<settings::SettingsStore, _>(|settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.editor.lsp_results_location = Some(OpenResultsIn::Picker);
                });
            });
        });
        cx.set_state(SOURCE);
        cx.lsp
            .set_request_handler::<lsp::request::GotoDefinition, _, _>(async move |_params, _| {
                Ok(None)
            });
        cx.lsp
            .set_request_handler::<lsp::request::References, _, _>(async move |params, _| {
                let uri = params.text_document_position.text_document.uri;
                Ok(Some(references(uri, &[(1, 8, 11), (2, 14, 17)])))
            });

        let screen_coord = cx
            .editor(|editor, _, cx| editor.pixel_position_of_cursor(cx))
            .unwrap();
        cx.simulate_click(screen_coord, gpui::Modifiers::secondary_key());
        cx.run_until_parked();

        assert!(
            active_picker(&mut cx).is_some(),
            "the cmd-click go-to-definition fallback should open the references picker when lsp_results_location is picker"
        );
    }

    #[gpui::test]
    async fn test_type_definition_honors_lsp_results_location(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                type_definition_provider: Some(lsp::TypeDefinitionProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.update(|_window, cx| {
            cx.update_global::<settings::SettingsStore, _>(|settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.editor.lsp_results_location = Some(OpenResultsIn::Picker);
                });
            });
        });
        cx.set_state(indoc! {r#"
            fn main() {
                struct Foo<T>(T);
                struct Bar;
                let fˇoo: Foo<Bar>;
            }
        "#});
        cx.lsp
            .set_request_handler::<lsp::request::GotoTypeDefinition, _, _>(
                async move |params, _| {
                    let uri = params.text_document_position_params.text_document.uri;
                    Ok(Some(lsp::GotoDefinitionResponse::Array(references(
                        uri,
                        &[(1, 11, 14), (2, 11, 14)],
                    ))))
                },
            );

        cx.dispatch_action(GoToTypeDefinition::default());

        assert!(
            active_picker(&mut cx).is_some(),
            "type definition should open the picker when lsp_results_location is picker"
        );
    }

    #[gpui::test]
    async fn test_declaration_honors_lsp_results_location(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let mut cx = rust_cx(
            lsp::ServerCapabilities {
                declaration_provider: Some(lsp::DeclarationCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.update(|_window, cx| {
            cx.update_global::<settings::SettingsStore, _>(|settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.editor.lsp_results_location = Some(OpenResultsIn::Picker);
                });
            });
        });
        cx.set_state(indoc! {r#"
            fn main() {
                let foo = ();
                let foo = ();
                let bar = fˇoo;
            }
        "#});
        cx.lsp
            .set_request_handler::<lsp::request::GotoDeclaration, _, _>(async move |params, _| {
                let uri = params.text_document_position_params.text_document.uri;
                Ok(Some(lsp::GotoDefinitionResponse::Array(references(
                    uri,
                    &[(1, 8, 11), (2, 8, 11)],
                ))))
            });

        cx.dispatch_action(GoToDeclaration::default());

        assert!(
            active_picker(&mut cx).is_some(),
            "declaration should open the picker when lsp_results_location is picker"
        );
    }
}

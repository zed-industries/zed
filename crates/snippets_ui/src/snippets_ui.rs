use editor::{Editor, actions::InsertSnippet};
use file_icons::FileIcons;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, Focusable, ParentElement,
    Render, SharedString, Styled, TaskExt, WeakEntity, Window, actions,
};
use language::{LanguageMatcher, LanguageName, LanguageRegistry};
use open_path_prompt::file_finder_settings::FileFinderSettings;
use paths::snippets_dir;
use picker::{Picker, PickerDelegate};
use settings::Settings;
use snippet_provider::Snippet;
use std::{
    borrow::{Borrow, Cow},
    collections::HashSet,
    fs,
    path::Path,
    sync::Arc,
};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, OpenOptions, OpenVisible, Workspace, notifications::NotifyResultExt};

#[derive(Eq, Hash, PartialEq)]
struct ScopeName(Cow<'static, str>);

struct ScopeFileName(Cow<'static, str>);

impl ScopeFileName {
    fn with_extension(self) -> String {
        format!("{}.json", self.0)
    }
}

const GLOBAL_SCOPE_NAME: &str = "global";
const GLOBAL_SCOPE_FILE_NAME: &str = "snippets";

impl From<ScopeName> for ScopeFileName {
    fn from(value: ScopeName) -> Self {
        if value.0 == GLOBAL_SCOPE_NAME {
            ScopeFileName(Cow::Borrowed(GLOBAL_SCOPE_FILE_NAME))
        } else {
            ScopeFileName(value.0)
        }
    }
}

impl From<ScopeFileName> for ScopeName {
    fn from(value: ScopeFileName) -> Self {
        if value.0 == GLOBAL_SCOPE_FILE_NAME {
            ScopeName(Cow::Borrowed(GLOBAL_SCOPE_NAME))
        } else {
            ScopeName(value.0)
        }
    }
}

actions!(
    snippets,
    [
        /// Opens the snippets configuration file.
        ConfigureSnippets,
        /// Opens the snippets folder in the file manager.
        OpenFolder
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(register).detach();
    cx.observe_new(register_insert_snippet).detach();
}

fn register(workspace: &mut Workspace, _window: Option<&mut Window>, _: &mut Context<Workspace>) {
    workspace.register_action(configure_snippets);
    workspace.register_action(open_folder);
}

// Opens a picker when `InsertSnippet` has no snippet to insert, e.g. when run from the command palette.
fn register_insert_snippet(
    editor: &mut Editor,
    _window: Option<&mut Window>,
    cx: &mut Context<Editor>,
) {
    let editor_handle = cx.entity().downgrade();
    editor
        .register_action(move |action: &InsertSnippet, window, cx| {
            if action.name.is_some() || action.snippet.is_some() {
                cx.propagate();
                return;
            }
            let Some(editor) = editor_handle.upgrade() else {
                return;
            };
            let workspace = editor
                .read(cx)
                .workspace()
                .filter(|_| editor.read(cx).mode().is_full());
            let Some(workspace) = workspace else {
                cx.propagate();
                return;
            };

            let snippets = snippets_for_editor(editor.read(cx), action.language.as_ref(), cx);
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, move |window, cx| {
                    SnippetSelector::new(editor, snippets, window, cx)
                });
            });
        })
        .detach();
}

fn snippets_for_editor(editor: &Editor, language: Option<&String>, cx: &App) -> Vec<Arc<Snippet>> {
    let Some(project) = editor.project() else {
        return Vec::new();
    };
    let provider = project.read(cx).snippets().read(cx);

    let scopes = if let Some(language) = language {
        vec![Some(language.clone())]
    } else {
        let languages = editor
            .buffer()
            .read(cx)
            .text_anchor_for_position(editor.selections.newest_anchor().head(), cx)
            .map(|(buffer, anchor)| buffer.read(cx).languages_at(anchor))
            .unwrap_or_default();
        if languages.is_empty() {
            vec![None]
        } else {
            languages
                .iter()
                .map(|language| Some(language.snippet_scope_id()))
                .collect()
        }
    };

    let mut snippets: Vec<Arc<Snippet>> = Vec::new();
    for scope in scopes {
        for snippet in provider.snippets_for(scope, cx) {
            if !snippets
                .iter()
                .any(|existing| Arc::ptr_eq(existing, &snippet))
            {
                snippets.push(snippet);
            }
        }
    }
    snippets.sort_by_cached_key(|snippet| snippet.name.to_lowercase());
    snippets
}

fn configure_snippets(
    workspace: &mut Workspace,
    _: &ConfigureSnippets,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let language_registry = workspace.app_state().languages.clone();
    let workspace_handle = workspace.weak_handle();

    workspace.toggle_modal(window, cx, move |window, cx| {
        ScopeSelector::new(language_registry, workspace_handle, window, cx)
    });
}

fn open_folder(
    workspace: &mut Workspace,
    _: &OpenFolder,
    _: &mut Window,
    cx: &mut Context<Workspace>,
) {
    fs::create_dir_all(snippets_dir()).notify_err(workspace, cx);
    cx.open_with_system(snippets_dir().borrow());
}

pub struct ScopeSelector {
    picker: Entity<Picker<ScopeSelectorDelegate>>,
}

struct SnippetSelector {
    picker: Entity<Picker<SnippetSelectorDelegate>>,
}

impl SnippetSelector {
    fn new(
        editor: Entity<Editor>,
        snippets: Vec<Arc<Snippet>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate =
            SnippetSelectorDelegate::new(editor.downgrade(), cx.entity().downgrade(), snippets);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl ModalView for SnippetSelector {}

impl EventEmitter<DismissEvent> for SnippetSelector {}

impl Focusable for SnippetSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for SnippetSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum MatchedField {
    Name,
    Prefix,
}

struct SnippetMatch {
    snippet_ix: usize,
    field: MatchedField,
    positions: Vec<usize>,
}

struct SnippetSelectorDelegate {
    editor: WeakEntity<Editor>,
    selector: WeakEntity<SnippetSelector>,
    snippets: Vec<Arc<Snippet>>,
    candidates: Vec<StringMatchCandidate>,
    // Parallel to `candidates`: the snippet and field each candidate string came from.
    candidate_sources: Vec<(usize, MatchedField)>,
    matches: Vec<SnippetMatch>,
    selected_index: usize,
}

impl SnippetSelectorDelegate {
    fn new(
        editor: WeakEntity<Editor>,
        selector: WeakEntity<SnippetSelector>,
        snippets: Vec<Arc<Snippet>>,
    ) -> Self {
        let mut candidates = Vec::new();
        let mut candidate_sources = Vec::new();
        for (snippet_ix, snippet) in snippets.iter().enumerate() {
            candidates.push(StringMatchCandidate::new(candidates.len(), &snippet.name));
            candidate_sources.push((snippet_ix, MatchedField::Name));

            let prefix = snippet.prefix.join(", ");
            if !prefix.is_empty() && prefix != snippet.name {
                candidates.push(StringMatchCandidate::new(candidates.len(), &prefix));
                candidate_sources.push((snippet_ix, MatchedField::Prefix));
            }
        }

        Self {
            editor,
            selector,
            snippets,
            candidates,
            candidate_sources,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for SnippetSelectorDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "snippet selector"
    }

    fn placeholder_text(&self, _window: &mut Window, _: &mut App) -> Arc<str> {
        "Select snippet to insert...".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _: &mut App) -> Option<SharedString> {
        Some(if self.snippets.is_empty() {
            "No snippets available for this language".into()
        } else {
            "No matches".into()
        })
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        let candidate_sources = self.candidate_sources.clone();
        let snippet_count = self.snippets.len();
        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                (0..snippet_count)
                    .map(|snippet_ix| SnippetMatch {
                        snippet_ix,
                        field: MatchedField::Name,
                        positions: Vec::new(),
                    })
                    .collect()
            } else {
                let string_matches = match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await;
                let mut seen = HashSet::new();
                string_matches
                    .into_iter()
                    .filter_map(|string_match| {
                        let (snippet_ix, field) =
                            *candidate_sources.get(string_match.candidate_id)?;
                        seen.insert(snippet_ix).then_some(SnippetMatch {
                            snippet_ix,
                            field,
                            positions: string_match.positions,
                        })
                    })
                    .collect()
            };

            this.update(cx, |this, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let body = self
            .matches
            .get(self.selected_index)
            .and_then(|mat| self.snippets.get(mat.snippet_ix))
            .map(|snippet| snippet.body.clone());
        let editor = self.editor.upgrade();
        self.dismissed(window, cx);

        if let Some((body, editor)) = body.zip(editor) {
            let action = InsertSnippet {
                language: None,
                name: None,
                snippet: Some(body),
            };
            editor.update(cx, |editor, cx| {
                window.focus(&editor.focus_handle(cx), cx);
                editor.insert_snippet_at_selections(&action, window, cx);
            });
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let snippet = self.snippets.get(mat.snippet_ix)?;
        let (name_positions, prefix_positions) = match mat.field {
            MatchedField::Name => (mat.positions.clone(), Vec::new()),
            MatchedField::Prefix => (Vec::new(), mat.positions.clone()),
        };
        let prefix = snippet.prefix.join(", ");
        let show_prefix = !prefix.is_empty() && prefix != snippet.name;
        let description = snippet
            .description
            .clone()
            .filter(|description| !description.is_empty());

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .child(
                            h_flex()
                                .gap_x_2()
                                .child(HighlightedLabel::new(snippet.name.clone(), name_positions))
                                .when(show_prefix, |row| {
                                    row.child(
                                        HighlightedLabel::new(prefix, prefix_positions)
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    )
                                }),
                        )
                        .when_some(description, |column, description| {
                            column.child(
                                Label::new(description)
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            )
                        }),
                ),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        if !self.snippets.is_empty() {
            return None;
        }

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("open-snippets-folder", "Open Snippets Folder")
                        .full_width()
                        .style(ButtonStyle::Outlined)
                        .on_click(cx.listener(|picker, _, window, cx| {
                            picker.delegate.dismissed(window, cx);
                            window.dispatch_action(OpenFolder.boxed_clone(), cx);
                        })),
                )
                .into_any_element(),
        )
    }
}

impl ScopeSelector {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate =
            ScopeSelectorDelegate::new(workspace, cx.entity().downgrade(), language_registry);

        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        Self { picker }
    }
}

impl ModalView for ScopeSelector {}

impl EventEmitter<DismissEvent> for ScopeSelector {}

impl Focusable for ScopeSelector {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ScopeSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

pub struct ScopeSelectorDelegate {
    workspace: WeakEntity<Workspace>,
    scope_selector: WeakEntity<ScopeSelector>,
    language_registry: Arc<LanguageRegistry>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    existing_scopes: HashSet<ScopeName>,
}

impl ScopeSelectorDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        scope_selector: WeakEntity<ScopeSelector>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        let languages = language_registry.language_names().into_iter();

        let candidates = std::iter::once(LanguageName::new(GLOBAL_SCOPE_NAME))
            .chain(languages)
            .enumerate()
            .map(|(candidate_id, name)| StringMatchCandidate::new(candidate_id, name.as_ref()))
            .collect::<Vec<_>>();

        let mut existing_scopes = HashSet::new();

        if let Some(read_dir) = fs::read_dir(snippets_dir()).log_err() {
            for entry in read_dir {
                if let Some(entry) = entry.log_err() {
                    let path = entry.path();
                    if let (Some(stem), Some(extension)) = (path.file_stem(), path.extension())
                        && extension.to_os_string().to_str() == Some("json")
                        && let Ok(file_name) = stem.to_os_string().into_string()
                    {
                        existing_scopes
                            .insert(ScopeName::from(ScopeFileName(Cow::Owned(file_name))));
                    }
                }
            }
        }

        Self {
            workspace,
            scope_selector,
            language_registry,
            candidates,
            matches: Vec::new(),
            selected_index: 0,
            existing_scopes,
        }
    }

    fn scope_icon(&self, matcher: &LanguageMatcher, cx: &App) -> Option<Icon> {
        matcher
            .path_suffixes
            .iter()
            .find_map(|extension| FileIcons::get_icon(Path::new(extension.as_str()), cx))
            .or(FileIcons::get(cx).get_icon_for_type("default", cx))
            .map(Icon::from_path)
            .map(|icon| icon.color(Color::Muted))
    }
}

impl PickerDelegate for ScopeSelectorDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "snippet scope selector"
    }

    fn placeholder_text(&self, _window: &mut Window, _: &mut App) -> Arc<str> {
        "Select snippet scope...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let scope_name = self.candidates[mat.candidate_id].string.clone();
            let language = self.language_registry.language_for_name(&scope_name);

            if let Some(workspace) = self.workspace.upgrade() {
                cx.spawn_in(window, async move |_, cx| {
                    let scope_file_name = ScopeFileName(match scope_name.to_lowercase().as_str() {
                        GLOBAL_SCOPE_NAME => Cow::Borrowed(GLOBAL_SCOPE_FILE_NAME),
                        _ => Cow::Owned(language.await?.snippet_scope_id()),
                    });

                    workspace.update_in(cx, |workspace, window, cx| {
                        workspace
                            .with_local_workspace(window, cx, |workspace, window, cx| {
                                workspace
                                    .open_abs_path(
                                        snippets_dir().join(scope_file_name.with_extension()),
                                        OpenOptions {
                                            visible: Some(OpenVisible::None),
                                            ..Default::default()
                                        },
                                        window,
                                        cx,
                                    )
                                    .detach();
                            })
                            .detach();
                    })
                })
                .detach_and_log_err(cx);
            };
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.scope_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
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
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches.get(ix)?;
        let name_label = mat.string.clone();

        let scope_name = ScopeName(Cow::Owned(
            LanguageName::new(&self.candidates[mat.candidate_id].string).snippet_scope_id(),
        ));
        let file_label = if self.existing_scopes.contains(&scope_name) {
            Some(ScopeFileName::from(scope_name).with_extension())
        } else {
            None
        };

        let language_icon = if FileFinderSettings::get_global(cx).file_icons {
            let language_name = LanguageName::new(mat.string.as_str());
            self.language_registry
                .available_language_for_name(language_name.as_ref())
                .and_then(|available_language| self.scope_icon(available_language.matcher(), cx))
                .or_else(|| {
                    Some(
                        Icon::from_path(IconName::ToolWeb.path())
                            .map(|icon| icon.color(Color::Muted)),
                    )
                })
        } else {
            None
        };

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot::<Icon>(language_icon)
                .child(
                    h_flex()
                        .gap_x_2()
                        .child(HighlightedLabel::new(name_label, mat.positions.clone()))
                        .when_some(file_label, |item, path_label| {
                            item.child(
                                Label::new(path_label)
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            )
                        }),
                ),
        )
    }
}

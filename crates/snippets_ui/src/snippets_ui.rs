use file_finder::file_finder_settings::FileFinderSettings;
use file_icons::FileIcons;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, ParentElement, Render, Styled,
    WeakEntity, Window, actions,
};
use language::{LanguageMatcher, LanguageName, LanguageRegistry};
use paths::snippets_dir;
use picker::{Picker, PickerDelegate};
use settings::Settings;
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
}

fn register(workspace: &mut Workspace, _window: Option<&mut Window>, _: &mut Context<Workspace>) {
    workspace.register_action(configure_snippets);
    workspace.register_action(open_folder);
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
        v_flex().w(rems(34.)).child(self.picker.clone())
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
                    if let (Some(stem), Some(extension)) = (path.file_stem(), path.extension()) {
                        if extension.to_os_string().to_str() == Some("json") {
                            if let Ok(file_name) = stem.to_os_string().into_string() {
                                existing_scopes
                                    .insert(ScopeName::from(ScopeFileName(Cow::Owned(file_name))));
                            }
                        }
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
            .find_map(|extension| FileIcons::get_icon(Path::new(extension), cx))
            .or(FileIcons::get(cx).get_icon_for_type("default", cx))
            .map(Icon::from_path)
            .map(|icon| icon.color(Color::Muted))
    }
}

impl PickerDelegate for ScopeSelectorDelegate {
    type ListItem = ListItem;

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
                        _ => Cow::Owned(language.await?.lsp_id()),
                    });

                    workspace.update_in(cx, |workspace, window, cx| {
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
                })
                .detach_and_log_err(cx);
            };
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.scope_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
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
        let mat = &self.matches[ix];
        let name_label = mat.string.clone();

        let scope_name = ScopeName(Cow::Owned(
            LanguageName::new(&self.candidates[mat.candidate_id].string).lsp_id(),
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

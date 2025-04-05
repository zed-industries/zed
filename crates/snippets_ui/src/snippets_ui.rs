use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, ParentElement, Render, Styled,
    WeakEntity, Window, actions,
};
use language::LanguageRegistry;
use paths::config_dir;
use picker::{Picker, PickerDelegate};
use std::{borrow::Borrow, fs, sync::Arc};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, OpenOptions, OpenVisible, Workspace, notifications::NotifyResultExt};

actions!(snippets, [ConfigureSnippets, OpenFolder]);

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
    fs::create_dir_all(config_dir().join("snippets")).notify_err(workspace, cx);
    cx.open_with_system(config_dir().join("snippets").borrow());
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
}

impl ScopeSelectorDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        scope_selector: WeakEntity<ScopeSelector>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        let candidates = Vec::from(["Global".to_string()]).into_iter();
        let languages = language_registry.language_names().into_iter();

        let candidates = candidates
            .chain(languages)
            .enumerate()
            .map(|(candidate_id, name)| StringMatchCandidate::new(candidate_id, &name))
            .collect::<Vec<_>>();

        Self {
            workspace,
            scope_selector,
            language_registry,
            candidates,
            matches: vec![],
            selected_index: 0,
        }
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
                    let scope = match scope_name.as_str() {
                        "Global" => "snippets".to_string(),
                        _ => language.await?.lsp_id(),
                    };

                    workspace.update_in(cx, |workspace, window, cx| {
                        workspace
                            .open_abs_path(
                                config_dir().join("snippets").join(scope + ".json"),
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
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let label = mat.string.clone();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(label, mat.positions.clone())),
        )
    }
}

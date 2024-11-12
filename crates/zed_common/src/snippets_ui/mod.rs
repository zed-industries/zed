use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, DismissEvent, EventEmitter, FocusableView, ParentElement, Render, Styled,
    View, ViewContext, VisualContext, WeakView,
};
use language::LanguageRegistry;
use paths::config_dir;
use picker::{Picker, PickerDelegate};
use std::{borrow::Borrow, fs, sync::Arc};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing, WindowContext};
use util::ResultExt;
use workspace::{notifications::NotifyResultExt, ModalView, Workspace};

actions!(snippets, [ConfigureSnippets, OpenFolder]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(register).detach();
}

fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(configure_snippets);
    workspace.register_action(open_folder);
}

fn configure_snippets(
    workspace: &mut Workspace,
    _: &ConfigureSnippets,
    cx: &mut ViewContext<Workspace>,
) {
    let language_registry = workspace.app_state().languages.clone();
    let workspace_handle = workspace.weak_handle();

    workspace.toggle_modal(cx, move |cx| {
        ScopeSelector::new(language_registry, workspace_handle, cx)
    });
}

fn open_folder(workspace: &mut Workspace, _: &OpenFolder, cx: &mut ViewContext<Workspace>) {
    fs::create_dir_all(config_dir().join("snippets")).notify_err(workspace, cx);
    cx.open_with_system(config_dir().join("snippets").borrow());
}

pub struct ScopeSelector {
    picker: View<Picker<ScopeSelectorDelegate>>,
}

impl ScopeSelector {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate =
            ScopeSelectorDelegate::new(workspace, cx.view().downgrade(), language_registry);

        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));

        Self { picker }
    }
}

impl ModalView for ScopeSelector {}

impl EventEmitter<DismissEvent> for ScopeSelector {}

impl FocusableView for ScopeSelector {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ScopeSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

pub struct ScopeSelectorDelegate {
    workspace: WeakView<Workspace>,
    scope_selector: WeakView<ScopeSelector>,
    language_registry: Arc<LanguageRegistry>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl ScopeSelectorDelegate {
    fn new(
        workspace: WeakView<Workspace>,
        scope_selector: WeakView<ScopeSelector>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        let candidates = Vec::from(["Global".to_string()]).into_iter();
        let languages = language_registry.language_names().into_iter();

        let candidates = candidates
            .chain(languages)
            .enumerate()
            .map(|(candidate_id, name)| StringMatchCandidate::new(candidate_id, name))
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

    fn placeholder_text(&self, _: &mut WindowContext) -> Arc<str> {
        "Select snippet scope...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let scope_name = self.candidates[mat.candidate_id].string.clone();
            let language = self.language_registry.language_for_name(&scope_name);

            if let Some(workspace) = self.workspace.upgrade() {
                cx.spawn(|_, mut cx| async move {
                    let scope = match scope_name.as_str() {
                        "Global" => "snippets".to_string(),
                        _ => language.await?.lsp_id(),
                    };

                    workspace.update(&mut cx, |workspace, cx| {
                        workspace
                            .open_abs_path(
                                config_dir().join("snippets").join(scope + ".json"),
                                false,
                                cx,
                            )
                            .detach();
                    })
                })
                .detach_and_log_err(cx);
            };
        }
        self.dismissed(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.scope_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn(|this, mut cx| async move {
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

            this.update(&mut cx, |this, cx| {
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
        _: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let label = mat.string.clone();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(HighlightedLabel::new(label, mat.positions.clone())),
        )
    }
}

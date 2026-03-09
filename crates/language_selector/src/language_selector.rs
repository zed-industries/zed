mod active_buffer_language;

pub use active_buffer_language::ActiveBufferLanguage;
use anyhow::Context as _;
use editor::Editor;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ParentElement,
    Render, Styled, WeakEntity, Window, actions,
};
use language::{Buffer, LanguageMatcher, LanguageName, LanguageRegistry};
use open_path_prompt::file_finder_settings::FileFinderSettings;
use picker::{Picker, PickerDelegate};
use project::Project;
use settings::Settings;
use std::{ops::Not as _, path::Path, sync::Arc};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(
    language_selector,
    [
        /// Toggles the language selector modal.
        Toggle
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(LanguageSelector::register).detach();
}

pub struct LanguageSelector {
    picker: Entity<Picker<LanguageSelectorDelegate>>,
}

impl LanguageSelector {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
        let registry = workspace.app_state().languages.clone();
        let (_, buffer, _) = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_excerpt(cx)?;
        let project = workspace.project().clone();

        workspace.toggle_modal(window, cx, move |window, cx| {
            LanguageSelector::new(buffer, project, registry, window, cx)
        });
        Some(())
    }

    fn new(
        buffer: Entity<Buffer>,
        project: Entity<Project>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let current_language_name = buffer
            .read(cx)
            .language()
            .map(|language| language.name().as_ref().to_string());
        let delegate = LanguageSelectorDelegate::new(
            cx.entity().downgrade(),
            buffer,
            project,
            language_registry,
            current_language_name,
        );

        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for LanguageSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("LanguageSelector")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

impl Focusable for LanguageSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for LanguageSelector {}
impl ModalView for LanguageSelector {}

pub struct LanguageSelectorDelegate {
    language_selector: WeakEntity<LanguageSelector>,
    buffer: Entity<Buffer>,
    project: Entity<Project>,
    language_registry: Arc<LanguageRegistry>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    current_language_candidate_index: Option<usize>,
}

impl LanguageSelectorDelegate {
    fn new(
        language_selector: WeakEntity<LanguageSelector>,
        buffer: Entity<Buffer>,
        project: Entity<Project>,
        language_registry: Arc<LanguageRegistry>,
        current_language_name: Option<String>,
    ) -> Self {
        let candidates = language_registry
            .language_names()
            .into_iter()
            .filter_map(|name| {
                language_registry
                    .available_language_for_name(name.as_ref())?
                    .hidden()
                    .not()
                    .then_some(name)
            })
            .enumerate()
            .map(|(candidate_id, name)| StringMatchCandidate::new(candidate_id, name.as_ref()))
            .collect::<Vec<_>>();

        let current_language_candidate_index = current_language_name.as_ref().and_then(|name| {
            candidates
                .iter()
                .position(|candidate| candidate.string == *name)
        });

        Self {
            language_selector,
            buffer,
            project,
            language_registry,
            candidates,
            matches: vec![],
            selected_index: current_language_candidate_index.unwrap_or(0),
            current_language_candidate_index,
        }
    }

    fn language_data_for_match(&self, mat: &StringMatch, cx: &App) -> (String, Option<Icon>) {
        let mut label = mat.string.clone();
        let buffer_language = self.buffer.read(cx).language();
        let need_icon = FileFinderSettings::get_global(cx).file_icons;

        if let Some(buffer_language) = buffer_language
            .filter(|buffer_language| buffer_language.name().as_ref() == mat.string.as_str())
        {
            label.push_str(" (current)");
            let icon = need_icon
                .then(|| self.language_icon(&buffer_language.config().matcher, cx))
                .flatten();
            (label, icon)
        } else {
            let icon = need_icon
                .then(|| {
                    let language_name = LanguageName::new(mat.string.as_str());
                    self.language_registry
                        .available_language_for_name(language_name.as_ref())
                        .and_then(|available_language| {
                            self.language_icon(available_language.matcher(), cx)
                        })
                })
                .flatten();
            (label, icon)
        }
    }

    fn language_icon(&self, matcher: &LanguageMatcher, cx: &App) -> Option<Icon> {
        matcher
            .path_suffixes
            .iter()
            .find_map(|extension| file_icons::FileIcons::get_icon(Path::new(extension), cx))
            .map(Icon::from_path)
            .map(|icon| icon.color(Color::Muted))
    }
}

impl PickerDelegate for LanguageSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a language…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let language_name = &self.candidates[mat.candidate_id].string;
            let language = self.language_registry.language_for_name(language_name);
            let project = self.project.downgrade();
            let buffer = self.buffer.downgrade();
            cx.spawn_in(window, async move |_, cx| {
                let language = language.await?;
                let project = project.upgrade().context("project was dropped")?;
                let buffer = buffer.upgrade().context("buffer was dropped")?;
                project.update(cx, |project, cx| {
                    project.set_language_for_buffer(&buffer, language, cx);
                });
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.language_selector
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
        let query_is_empty = query.is_empty();
        cx.spawn_in(window, async move |this, cx| {
            let matches = if query_is_empty {
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

            this.update_in(cx, |this, window, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));

                if query_is_empty {
                    if let Some(index) = delegate
                        .current_language_candidate_index
                        .and_then(|ci| delegate.matches.iter().position(|m| m.candidate_id == ci))
                    {
                        this.set_selected_index(index, None, false, window, cx);
                    }
                }
                cx.notify();
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches.get(ix)?;
        let (label, language_icon) = self.language_data_for_match(mat, cx);
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot::<Icon>(language_icon)
                .child(HighlightedLabel::new(label, mat.positions.clone())),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::Editor;
    use gpui::{TestAppContext, VisualTestContext};
    use language::{Language, LanguageConfig};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use std::sync::Arc;
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, MultiWorkspace, Workspace};

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            settings::init(cx);
            super::init(cx);
            editor::init(cx);
            app_state
        })
    }

    fn register_test_languages(project: &Entity<Project>, cx: &mut VisualTestContext) {
        project.read_with(cx, |project, _| {
            let language_registry = project.languages();
            language_registry.add(Arc::new(Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                None,
            )));
            language_registry.add(Arc::new(Language::new(
                LanguageConfig {
                    name: "TypeScript".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["ts".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                None,
            )));
        });
    }

    async fn open_file_editor(
        workspace: &Entity<Workspace>,
        project: &Entity<Project>,
        file_path: &str,
        cx: &mut VisualTestContext,
    ) -> Entity<Editor> {
        let worktree_id = project.update(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .expect("project should have a worktree")
                .read(cx)
                .id()
        });
        let project_path = ProjectPath {
            worktree_id,
            path: rel_path(file_path).into(),
        };
        let opened_item = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(project_path, None, true, window, cx)
            })
            .await
            .expect("file should open");

        cx.update(|_, cx| {
            opened_item
                .act_as::<Editor>(cx)
                .expect("opened item should be an editor")
        })
    }

    async fn open_empty_editor(
        workspace: &Entity<Workspace>,
        project: &Entity<Project>,
        cx: &mut VisualTestContext,
    ) -> Entity<Editor> {
        let create_buffer = project.update(cx, |project, cx| project.create_buffer(None, true, cx));
        let buffer = create_buffer.await.expect("empty buffer should be created");
        let editor = cx.new_window_entity(|window, cx| {
            Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx)
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_center(Box::new(editor.clone()), window, cx);
        });
        // Ensure the buffer has no language after the editor is created
        buffer.update(cx, |buffer, cx| {
            buffer.set_language(None, cx);
        });
        editor
    }

    async fn set_editor_language(
        project: &Entity<Project>,
        editor: &Entity<Editor>,
        language_name: &str,
        cx: &mut VisualTestContext,
    ) {
        let language = project
            .read_with(cx, |project, _| {
                project.languages().language_for_name(language_name)
            })
            .await
            .expect("language should exist in registry");
        editor.update(cx, move |editor, cx| {
            let (_, buffer, _) = editor
                .active_excerpt(cx)
                .expect("editor should have an active excerpt");
            buffer.update(cx, |buffer, cx| {
                buffer.set_language(Some(language), cx);
            });
        });
    }

    fn active_picker(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<LanguageSelectorDelegate>> {
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<LanguageSelector>(cx)
                .expect("language selector should be open")
                .read(cx)
                .picker
                .clone()
        })
    }

    fn open_selector(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<LanguageSelectorDelegate>> {
        cx.dispatch_action(Toggle);
        cx.run_until_parked();
        active_picker(workspace, cx)
    }

    fn close_selector(workspace: &Entity<Workspace>, cx: &mut VisualTestContext) {
        cx.dispatch_action(Toggle);
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert!(
                workspace.active_modal::<LanguageSelector>(cx).is_none(),
                "language selector should be closed"
            );
        });
    }

    fn assert_selected_language_for_editor(
        workspace: &Entity<Workspace>,
        editor: &Entity<Editor>,
        expected_language_name: Option<&str>,
        cx: &mut VisualTestContext,
    ) {
        workspace.update_in(cx, |workspace, window, cx| {
            let was_activated = workspace.activate_item(editor, true, true, window, cx);
            assert!(
                was_activated,
                "editor should be activated before opening the modal"
            );
        });
        cx.run_until_parked();

        let picker = open_selector(workspace, cx);
        picker.read_with(cx, |picker, _| {
            let selected_match = picker
                .delegate
                .matches
                .get(picker.delegate.selected_index)
                .expect("selected index should point to a match");
            let selected_candidate = picker
                .delegate
                .candidates
                .get(selected_match.candidate_id)
                .expect("selected match should map to a candidate");

            if let Some(expected_language_name) = expected_language_name {
                let current_language_candidate_index = picker
                    .delegate
                    .current_language_candidate_index
                    .expect("current language should map to a candidate");
                assert_eq!(
                    selected_match.candidate_id,
                    current_language_candidate_index
                );
                assert_eq!(selected_candidate.string, expected_language_name);
            } else {
                assert!(picker.delegate.current_language_candidate_index.is_none());
                assert_eq!(picker.delegate.selected_index, 0);
            }
        });
        close_selector(workspace, cx);
    }

    #[gpui::test]
    async fn test_language_selector_selects_current_language_per_active_editor(
        cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/test"),
                json!({
                    "rust_file.rs": "fn main() {}\n",
                    "typescript_file.ts": "const value = 1;\n",
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        register_test_languages(&project, cx);

        let rust_editor = open_file_editor(&workspace, &project, "rust_file.rs", cx).await;
        let typescript_editor =
            open_file_editor(&workspace, &project, "typescript_file.ts", cx).await;
        let empty_editor = open_empty_editor(&workspace, &project, cx).await;

        set_editor_language(&project, &rust_editor, "Rust", cx).await;
        set_editor_language(&project, &typescript_editor, "TypeScript", cx).await;
        cx.run_until_parked();

        assert_selected_language_for_editor(&workspace, &rust_editor, Some("Rust"), cx);
        assert_selected_language_for_editor(&workspace, &typescript_editor, Some("TypeScript"), cx);
        // Ensure the empty editor's buffer has no language before asserting
        let (_, buffer, _) = empty_editor.read_with(cx, |editor, cx| {
            editor
                .active_excerpt(cx)
                .expect("editor should have an active excerpt")
        });
        buffer.update(cx, |buffer, cx| {
            buffer.set_language(None, cx);
        });
        assert_selected_language_for_editor(&workspace, &empty_editor, None, cx);
    }
}

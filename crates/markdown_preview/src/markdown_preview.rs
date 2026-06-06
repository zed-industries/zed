use editor::Editor;
use gpui::{App, Context, Window, actions};
use markdown_preview_view::{MarkdownPreviewMode, MarkdownPreviewView};
use workspace::item::ItemHandle;
use workspace::{AutoPreviewMatch, AutoPreviewProvider, Workspace, register_auto_preview_provider};

pub mod markdown_preview_view;

pub use zed_actions::preview::markdown::{OpenPreview, OpenPreviewToTheSide};

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageUp"])]
        ScrollPageUp,
        /// Scrolls down by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageDown"])]
        ScrollPageDown,
        /// Scrolls up by approximately one visual line.
        ScrollUp,
        /// Scrolls down by approximately one visual line.
        ScrollDown,
        /// Scrolls up by one markdown element in the markdown preview
        ScrollUpByItem,
        /// Scrolls down by one markdown element in the markdown preview
        ScrollDownByItem,
        /// Scrolls to the top of the markdown preview.
        ScrollToTop,
        /// Scrolls to the bottom of the markdown preview.
        ScrollToBottom,
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

struct MarkdownAutoPreviewProvider;

impl AutoPreviewProvider for MarkdownAutoPreviewProvider {
    fn id(&self) -> &'static str {
        "markdown"
    }

    fn match_item(&self, item: &dyn ItemHandle, cx: &App) -> AutoPreviewMatch {
        if item.downcast::<MarkdownPreviewView>().is_some() {
            return AutoPreviewMatch::No;
        }
        let Some(editor) = item.act_as::<Editor>(cx) else {
            return AutoPreviewMatch::No;
        };
        let buffer = editor.read(cx).buffer().read(cx);
        let Some(buffer) = buffer.as_singleton() else {
            return AutoPreviewMatch::No;
        };
        match buffer.read(cx).language() {
            Some(language) if language.name() == "Markdown" => AutoPreviewMatch::Yes,
            None => AutoPreviewMatch::Pending,
            Some(_) => AutoPreviewMatch::No,
        }
    }

    fn create(
        &self,
        item: &dyn ItemHandle,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Box<dyn ItemHandle>> {
        let editor = item.act_as::<Editor>(cx)?;
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        let view = MarkdownPreviewView::new(
            MarkdownPreviewMode::Auto,
            editor,
            workspace_handle,
            language_registry,
            window,
            cx,
        );
        Some(Box::new(view))
    }

    fn swap(
        &self,
        preview: &dyn ItemHandle,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(preview) = preview.downcast::<MarkdownPreviewView>() else {
            return false;
        };
        let Some(editor) = item.act_as::<Editor>(cx) else {
            return false;
        };
        preview.update(cx, |preview, cx| preview.set_editor(editor, window, cx));
        true
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();

    register_auto_preview_provider(MarkdownAutoPreviewProvider, cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext as _, Entity, TestAppContext};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use serde_json::json;
    use std::path::PathBuf;
    use std::sync::Arc;
    use util::path;
    use workspace::{AppState, MultiWorkspace, open_paths};

    #[gpui::test]
    async fn test_markdown_provider_rejects_non_editor(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let provider = MarkdownAutoPreviewProvider;
            let item = cx.new(|cx| workspace::item::test::TestItem::new(cx));
            let item: Box<dyn workspace::ItemHandle> = Box::new(item);
            assert_eq!(
                provider.match_item(item.as_ref(), cx),
                workspace::AutoPreviewMatch::No
            );
        });
    }

    #[gpui::test]
    async fn test_markdown_provider_matches_markdown_editor(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "notes.md": "# Hello\n",
                    "notes.txt": "plain text\n"
                }),
            )
            .await;

        let editor = open_editor_for(&app_state, path!("/dir/notes.md"), cx).await;
        cx.update(|cx| {
            let provider = MarkdownAutoPreviewProvider;
            let item: Box<dyn ItemHandle> = Box::new(editor);
            assert_eq!(
                provider.match_item(item.as_ref(), cx),
                AutoPreviewMatch::Yes
            );
        });

        let editor = open_editor_for(&app_state, path!("/dir/notes.txt"), cx).await;
        cx.update(|cx| {
            let provider = MarkdownAutoPreviewProvider;
            let item: Box<dyn ItemHandle> = Box::new(editor);
            assert_eq!(provider.match_item(item.as_ref(), cx), AutoPreviewMatch::No);
        });
    }

    #[gpui::test]
    async fn test_markdown_provider_rejects_own_preview_view(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "notes.md": "# Hello\n"
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir/notes.md"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let preview = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                let editor: Entity<Editor> = workspace
                    .read(cx)
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                    .unwrap();
                workspace.update(cx, |workspace, cx| {
                    MarkdownPreviewView::new(
                        MarkdownPreviewMode::Auto,
                        editor,
                        workspace.weak_handle(),
                        workspace.project().read(cx).languages().clone(),
                        window,
                        cx,
                    )
                })
            })
            .unwrap();
        cx.run_until_parked();

        cx.update(|cx| {
            let provider = MarkdownAutoPreviewProvider;
            let item: Box<dyn ItemHandle> = Box::new(preview);
            assert_eq!(provider.match_item(item.as_ref(), cx), AutoPreviewMatch::No);
        });
    }

    async fn open_editor_for(
        app_state: &Arc<AppState>,
        file_path: &str,
        cx: &mut TestAppContext,
    ) -> Entity<Editor> {
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(file_path)],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let editor = multi_workspace
            .update(cx, |multi_workspace, _window, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                    .unwrap()
            })
            .unwrap();
        cx.run_until_parked();
        editor
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            editor::init(cx);
            crate::init(cx);
            state.languages.add(Arc::new(Language::new(
                LanguageConfig {
                    name: "Markdown".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["md".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                None,
            )));
            state.languages.add(Arc::new(Language::new(
                LanguageConfig {
                    name: "Plain Text".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["txt".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                None,
            )));
            state
        })
    }
}

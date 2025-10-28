use std::{
    borrow::Cow,
    ops::{Deref, DerefMut, Range},
    path::Path,
    sync::Arc,
};

use anyhow::Result;
use language::rust_lang;
use serde_json::json;

use crate::{Editor, ToPoint};
use collections::HashSet;
use futures::Future;
use gpui::{Context, Entity, Focusable as _, VisualTestContext, Window};
use indoc::indoc;
use language::{
    BlockCommentConfig, FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageQueries,
    point_to_lsp,
};
use lsp::{notification, request};
use project::Project;
use smol::stream::StreamExt;
use workspace::{AppState, Workspace, WorkspaceHandle};

use super::editor_test_context::{AssertionContextManager, EditorTestContext};

pub struct EditorLspTestContext {
    pub cx: EditorTestContext,
    pub lsp: lsp::FakeLanguageServer,
    pub workspace: Entity<Workspace>,
    pub buffer_lsp_url: lsp::Uri,
}

#[cfg(test)]
pub(crate) fn git_commit_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "Git Commit".into(),
            line_comments: vec!["#".into()],
            ..Default::default()
        },
        None,
    ))
}

impl EditorLspTestContext {
    pub async fn new(
        language: Language,
        capabilities: lsp::ServerCapabilities,
        cx: &mut gpui::TestAppContext,
    ) -> EditorLspTestContext {
        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            assets::Assets.load_test_fonts(cx);
            language::init(cx);
            crate::init(cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);
        });

        let file_name = format!(
            "file.{}",
            language
                .path_suffixes()
                .first()
                .expect("language must have a path suffix for EditorLspTestContext")
        );

        let project = Project::test(app_state.fs.clone(), [], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let mut fake_servers = language_registry.register_fake_lsp(
            language.name(),
            FakeLspAdapter {
                capabilities,
                ..Default::default()
            },
        );
        language_registry.add(Arc::new(language));

        let root = Self::root_path();

        app_state
            .fs
            .as_fake()
            .insert_tree(
                root,
                json!({
                    ".git": {},
                    "dir": {
                        file_name.clone(): ""
                    }
                }),
            )
            .await;

        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let workspace = window.root(cx).unwrap();

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);
        project
            .update(&mut cx, |project, cx| {
                project.find_or_create_worktree(root, true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let item = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path(file, None, true, window, cx)
            })
            .await
            .expect("Could not open test file");
        let editor = cx.update(|_, cx| {
            item.act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });
        editor.update_in(&mut cx, |editor, window, cx| {
            let nav_history = workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .nav_history_for_item(&cx.entity());
            editor.set_nav_history(Some(nav_history));
            window.focus(&editor.focus_handle(cx))
        });

        let lsp = fake_servers.next().await.unwrap();
        Self {
            cx: EditorTestContext {
                cx,
                window: window.into(),
                editor,
                assertion_cx: AssertionContextManager::new(),
            },
            lsp,
            workspace,
            buffer_lsp_url: lsp::Uri::from_file_path(root.join("dir").join(file_name)).unwrap(),
        }
    }

    pub async fn new_rust(
        capabilities: lsp::ServerCapabilities,
        cx: &mut gpui::TestAppContext,
    ) -> EditorLspTestContext {
        Self::new(Arc::into_inner(rust_lang()).unwrap(), capabilities, cx).await
    }

    pub async fn new_typescript(
        capabilities: lsp::ServerCapabilities,
        cx: &mut gpui::TestAppContext,
    ) -> EditorLspTestContext {
        let mut word_characters: HashSet<char> = Default::default();
        word_characters.insert('$');
        word_characters.insert('#');
        let language = Language::new(
            LanguageConfig {
                name: "Typescript".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["ts".to_string()],
                    ..Default::default()
                },
                brackets: language::BracketPairConfig {
                    pairs: vec![language::BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    }],
                    disabled_scopes_by_bracket_ix: Default::default(),
                },
                word_characters,
                ..Default::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        )
        .with_queries(LanguageQueries {
            brackets: Some(Cow::from(indoc! {r#"
                ("(" @open ")" @close)
                ("[" @open "]" @close)
                ("{" @open "}" @close)
                ("<" @open ">" @close)
                ("'" @open "'" @close)
                ("`" @open "`" @close)
                ("\"" @open "\"" @close)"#})),
            indents: Some(Cow::from(indoc! {r#"
                [
                    (call_expression)
                    (assignment_expression)
                    (member_expression)
                    (lexical_declaration)
                    (variable_declaration)
                    (assignment_expression)
                    (if_statement)
                    (for_statement)
                ] @indent

                (_ "[" "]" @end) @indent
                (_ "<" ">" @end) @indent
                (_ "{" "}" @end) @indent
                (_ "(" ")" @end) @indent
                "#})),
            ..Default::default()
        })
        .expect("Could not parse queries");

        Self::new(language, capabilities, cx).await
    }

    pub async fn new_tsx(
        capabilities: lsp::ServerCapabilities,
        cx: &mut gpui::TestAppContext,
    ) -> EditorLspTestContext {
        let mut word_characters: HashSet<char> = Default::default();
        word_characters.insert('$');
        word_characters.insert('#');
        let language = Language::new(
            LanguageConfig {
                name: "TSX".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["tsx".to_string()],
                    ..Default::default()
                },
                brackets: language::BracketPairConfig {
                    pairs: vec![language::BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    }],
                    disabled_scopes_by_bracket_ix: Default::default(),
                },
                word_characters,
                ..Default::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
        )
        .with_queries(LanguageQueries {
            brackets: Some(Cow::from(indoc! {r#"
                ("(" @open ")" @close)
                ("[" @open "]" @close)
                ("{" @open "}" @close)
                ("<" @open ">" @close)
                ("<" @open "/>" @close)
                ("</" @open ">" @close)
                ("\"" @open "\"" @close)
                ("'" @open "'" @close)
                ("`" @open "`" @close)
                ((jsx_element (jsx_opening_element) @open (jsx_closing_element) @close) (#set! newline.only))"#})),
            indents: Some(Cow::from(indoc! {r#"
                [
                    (call_expression)
                    (assignment_expression)
                    (member_expression)
                    (lexical_declaration)
                    (variable_declaration)
                    (assignment_expression)
                    (if_statement)
                    (for_statement)
                ] @indent

                (_ "[" "]" @end) @indent
                (_ "<" ">" @end) @indent
                (_ "{" "}" @end) @indent
                (_ "(" ")" @end) @indent

                (jsx_opening_element ">" @end) @indent

                (jsx_element
                  (jsx_opening_element) @start
                  (jsx_closing_element)? @end) @indent
                "#})),
            ..Default::default()
        })
        .expect("Could not parse queries");

        Self::new(language, capabilities, cx).await
    }

    pub async fn new_html(cx: &mut gpui::TestAppContext) -> Self {
        let language = Language::new(
            LanguageConfig {
                name: "HTML".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["html".into()],
                    ..Default::default()
                },
                block_comment: Some(BlockCommentConfig {
                    start: "<!--".into(),
                    prefix: "".into(),
                    end: "-->".into(),
                    tab_size: 0,
                }),
                completion_query_characters: ['-'].into_iter().collect(),
                ..Default::default()
            },
            Some(tree_sitter_html::LANGUAGE.into()),
        )
        .with_queries(LanguageQueries {
            brackets: Some(Cow::from(indoc! {r#"
                ("<" @open "/>" @close)
                ("</" @open ">" @close)
                ("<" @open ">" @close)
                ("\"" @open "\"" @close)"#})),
            ..Default::default()
        })
        .expect("Could not parse queries");
        Self::new(language, Default::default(), cx).await
    }

    /// Constructs lsp range using a marked string with '[', ']' range delimiters
    #[track_caller]
    pub fn lsp_range(&mut self, marked_text: &str) -> lsp::Range {
        let ranges = self.ranges(marked_text);
        self.to_lsp_range(ranges[0].clone())
    }

    #[expect(clippy::wrong_self_convention, reason = "This is test code")]
    pub fn to_lsp_range(&mut self, range: Range<usize>) -> lsp::Range {
        let snapshot = self.update_editor(|editor, window, cx| editor.snapshot(window, cx));
        let start_point = range.start.to_point(&snapshot.buffer_snapshot());
        let end_point = range.end.to_point(&snapshot.buffer_snapshot());

        self.editor(|editor, _, cx| {
            let buffer = editor.buffer().read(cx);
            let start = point_to_lsp(
                buffer
                    .point_to_buffer_offset(start_point, cx)
                    .unwrap()
                    .1
                    .to_point_utf16(&buffer.read(cx)),
            );
            let end = point_to_lsp(
                buffer
                    .point_to_buffer_offset(end_point, cx)
                    .unwrap()
                    .1
                    .to_point_utf16(&buffer.read(cx)),
            );

            lsp::Range { start, end }
        })
    }

    #[expect(clippy::wrong_self_convention, reason = "This is test code")]
    pub fn to_lsp(&mut self, offset: usize) -> lsp::Position {
        let snapshot = self.update_editor(|editor, window, cx| editor.snapshot(window, cx));
        let point = offset.to_point(&snapshot.buffer_snapshot());

        self.editor(|editor, _, cx| {
            let buffer = editor.buffer().read(cx);
            point_to_lsp(
                buffer
                    .point_to_buffer_offset(point, cx)
                    .unwrap()
                    .1
                    .to_point_utf16(&buffer.read(cx)),
            )
        })
    }

    pub fn update_workspace<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Workspace, &mut Window, &mut Context<Workspace>) -> T,
    {
        self.workspace.update_in(&mut self.cx.cx, update)
    }

    pub fn set_request_handler<T, F, Fut>(
        &self,
        mut handler: F,
    ) -> futures::channel::mpsc::UnboundedReceiver<()>
    where
        T: 'static + request::Request,
        T::Params: 'static + Send,
        F: 'static + Send + FnMut(lsp::Uri, T::Params, gpui::AsyncApp) -> Fut,
        Fut: 'static + Future<Output = Result<T::Result>>,
    {
        let url = self.buffer_lsp_url.clone();
        self.lsp.set_request_handler::<T, _, _>(move |params, cx| {
            let url = url.clone();
            handler(url, params, cx)
        })
    }

    pub fn notify<T: notification::Notification>(&self, params: T::Params) {
        self.lsp.notify::<T>(params);
    }

    #[cfg(target_os = "windows")]
    fn root_path() -> &'static Path {
        Path::new("C:\\root")
    }

    #[cfg(not(target_os = "windows"))]
    fn root_path() -> &'static Path {
        Path::new("/root")
    }
}

impl Deref for EditorLspTestContext {
    type Target = EditorTestContext;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl DerefMut for EditorLspTestContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

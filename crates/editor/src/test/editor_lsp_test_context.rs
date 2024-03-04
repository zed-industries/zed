use std::{
    borrow::Cow,
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};

use anyhow::Result;
use serde_json::json;

use crate::{Editor, ToPoint};
use collections::HashSet;
use futures::Future;
use gpui::{View, ViewContext, VisualTestContext};
use indoc::indoc;
use language::{
    point_to_lsp, FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageQueries,
};
use lsp::{notification, request};
use multi_buffer::ToPointUtf16;
use project::Project;
use smol::stream::StreamExt;
use workspace::{AppState, Workspace, WorkspaceHandle};

use super::editor_test_context::{AssertionContextManager, EditorTestContext};

pub struct EditorLspTestContext {
    pub cx: EditorTestContext,
    pub lsp: lsp::FakeLanguageServer,
    pub workspace: View<Workspace>,
    pub buffer_lsp_url: lsp::Url,
}

impl EditorLspTestContext {
    pub async fn new(
        language: Language,
        capabilities: lsp::ServerCapabilities,
        cx: &mut gpui::TestAppContext,
    ) -> EditorLspTestContext {
        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
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
        let mut fake_servers = language_registry.register_fake_lsp_adapter(
            language.name().as_ref(),
            FakeLspAdapter {
                capabilities,
                ..Default::default()
            },
        );
        language_registry.add(Arc::new(language));

        app_state
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "dir": { file_name.clone(): "" }}))
            .await;

        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

        let workspace = window.root_view(cx).unwrap();

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);
        project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let item = workspace
            .update(&mut cx, |workspace, cx| {
                workspace.open_path(file, None, true, cx)
            })
            .await
            .expect("Could not open test file");
        let editor = cx.update(|cx| {
            item.act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });
        editor.update(&mut cx, |editor, cx| editor.focus(cx));

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
            buffer_lsp_url: lsp::Url::from_file_path(format!("/root/dir/{file_name}")).unwrap(),
        }
    }

    pub async fn new_rust(
        capabilities: lsp::ServerCapabilities,
        cx: &mut gpui::TestAppContext,
    ) -> EditorLspTestContext {
        let language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_queries(LanguageQueries {
            indents: Some(Cow::from(indoc! {r#"
                [
                    ((where_clause) _ @end)
                    (field_expression)
                    (call_expression)
                    (assignment_expression)
                    (let_declaration)
                    (let_chain)
                    (await_expression)
                ] @indent

                (_ "[" "]" @end) @indent
                (_ "<" ">" @end) @indent
                (_ "{" "}" @end) @indent
                (_ "(" ")" @end) @indent"#})),
            brackets: Some(Cow::from(indoc! {r#"
                ("(" @open ")" @close)
                ("[" @open "]" @close)
                ("{" @open "}" @close)
                ("<" @open ">" @close)
                ("\"" @open "\"" @close)
                (closure_parameters "|" @open "|" @close)"#})),
            ..Default::default()
        })
        .expect("Could not parse queries");

        Self::new(language, capabilities, cx).await
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
                        newline: true,
                    }],
                    disabled_scopes_by_bracket_ix: Default::default(),
                },
                word_characters,
                ..Default::default()
            },
            Some(tree_sitter_typescript::language_typescript()),
        )
        .with_queries(LanguageQueries {
            brackets: Some(Cow::from(indoc! {r#"
                ("(" @open ")" @close)
                ("[" @open "]" @close)
                ("{" @open "}" @close)
                ("<" @open ">" @close)
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

    pub async fn new_html(cx: &mut gpui::TestAppContext) -> Self {
        let language = Language::new(
            LanguageConfig {
                name: "HTML".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["html".into()],
                    ..Default::default()
                },
                block_comment: Some(("<!-- ".into(), " -->".into())),
                ..Default::default()
            },
            Some(tree_sitter_html::language()),
        );
        Self::new(language, Default::default(), cx).await
    }

    // Constructs lsp range using a marked string with '[', ']' range delimiters
    pub fn lsp_range(&mut self, marked_text: &str) -> lsp::Range {
        let ranges = self.ranges(marked_text);
        self.to_lsp_range(ranges[0].clone())
    }

    pub fn to_lsp_range(&mut self, range: Range<usize>) -> lsp::Range {
        let snapshot = self.update_editor(|editor, cx| editor.snapshot(cx));
        let start_point = range.start.to_point(&snapshot.buffer_snapshot);
        let end_point = range.end.to_point(&snapshot.buffer_snapshot);

        self.editor(|editor, cx| {
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

    pub fn to_lsp(&mut self, offset: usize) -> lsp::Position {
        let snapshot = self.update_editor(|editor, cx| editor.snapshot(cx));
        let point = offset.to_point(&snapshot.buffer_snapshot);

        self.editor(|editor, cx| {
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
        F: FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> T,
    {
        self.workspace.update(&mut self.cx.cx, update)
    }

    pub fn handle_request<T, F, Fut>(
        &self,
        mut handler: F,
    ) -> futures::channel::mpsc::UnboundedReceiver<()>
    where
        T: 'static + request::Request,
        T::Params: 'static + Send,
        F: 'static + Send + FnMut(lsp::Url, T::Params, gpui::AsyncAppContext) -> Fut,
        Fut: 'static + Send + Future<Output = Result<T::Result>>,
    {
        let url = self.buffer_lsp_url.clone();
        self.lsp.handle_request::<T, _, _>(move |params, cx| {
            let url = url.clone();
            handler(url, params, cx)
        })
    }

    pub fn notify<T: notification::Notification>(&self, params: T::Params) {
        self.lsp.notify::<T>(params);
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

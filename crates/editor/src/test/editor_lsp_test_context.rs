use std::{
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};

use anyhow::Result;

use futures::Future;
use gpui::{json, ViewContext, ViewHandle};
use language::{point_to_lsp, FakeLspAdapter, Language, LanguageConfig};
use lsp::{notification, request};
use project::Project;
use smol::stream::StreamExt;
use workspace::{pane, AppState, Workspace, WorkspaceHandle};

use crate::{multi_buffer::ToPointUtf16, Editor, ToPoint};

use super::editor_test_context::EditorTestContext;

pub struct EditorLspTestContext<'a> {
    pub cx: EditorTestContext<'a>,
    pub lsp: lsp::FakeLanguageServer,
    pub workspace: ViewHandle<Workspace>,
    pub buffer_lsp_url: lsp::Url,
}

impl<'a> EditorLspTestContext<'a> {
    pub async fn new(
        mut language: Language,
        capabilities: lsp::ServerCapabilities,
        cx: &'a mut gpui::TestAppContext,
    ) -> EditorLspTestContext<'a> {
        use json::json;

        cx.update(|cx| {
            crate::init(cx);
            pane::init(cx);
        });

        let params = cx.update(AppState::test);

        let file_name = format!(
            "file.{}",
            language
                .path_suffixes()
                .first()
                .unwrap_or(&"txt".to_string())
        );

        let mut fake_servers = language
            .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
                capabilities,
                ..Default::default()
            }))
            .await;

        let project = Project::test(params.fs.clone(), [], cx).await;
        project.update(cx, |project, _| project.languages().add(Arc::new(language)));

        params
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "dir": { file_name: "" }}))
            .await;

        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(
                Default::default(),
                0,
                project.clone(),
                |_, _| unimplemented!(),
                cx,
            )
        });
        project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let item = workspace
            .update(cx, |workspace, cx| {
                workspace.open_path(file, None, true, cx)
            })
            .await
            .expect("Could not open test file");

        let editor = cx.update(|cx| {
            item.act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });
        editor.update(cx, |_, cx| cx.focus_self());

        let lsp = fake_servers.next().await.unwrap();

        Self {
            cx: EditorTestContext {
                cx,
                window_id,
                editor,
            },
            lsp,
            workspace,
            buffer_lsp_url: lsp::Url::from_file_path("/root/dir/file.rs").unwrap(),
        }
    }

    pub async fn new_rust(
        capabilities: lsp::ServerCapabilities,
        cx: &'a mut gpui::TestAppContext,
    ) -> EditorLspTestContext<'a> {
        let language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );

        Self::new(language, capabilities, cx).await
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
        self.workspace.update(self.cx.cx, update)
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

impl<'a> Deref for EditorLspTestContext<'a> {
    type Target = EditorTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a> DerefMut for EditorLspTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

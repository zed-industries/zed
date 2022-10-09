use crate::{
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
    multi_buffer::ToPointUtf16,
    AnchorRangeExt, Autoscroll, DisplayPoint, Editor, EditorMode, MultiBuffer, ToPoint,
};
use anyhow::Result;
use collections::BTreeMap;
use futures::{Future, StreamExt};
use gpui::{
    json, keymap::Keystroke, AppContext, ModelContext, ModelHandle, ViewContext, ViewHandle,
};
use indoc::indoc;
use itertools::Itertools;
use language::{point_to_lsp, Buffer, BufferSnapshot, FakeLspAdapter, Language, LanguageConfig};
use lsp::{notification, request};
use parking_lot::RwLock;
use project::Project;
use settings::Settings;
use std::{
    any::TypeId,
    ops::{Deref, DerefMut, Range},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use util::{
    assert_set_eq, set_eq,
    test::{generate_marked_text, marked_text_offsets, marked_text_ranges},
};
use workspace::{pane, AppState, Workspace, WorkspaceHandle};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

// Returns a snapshot from text containing '|' character markers with the markers removed, and DisplayPoints for each one.
pub fn marked_display_snapshot(
    text: &str,
    cx: &mut gpui::MutableAppContext,
) -> (DisplaySnapshot, Vec<DisplayPoint>) {
    let (unmarked_text, markers) = marked_text_offsets(text);

    let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
    let font_id = cx
        .font_cache()
        .select_font(family_id, &Default::default())
        .unwrap();
    let font_size = 14.0;

    let buffer = MultiBuffer::build_simple(&unmarked_text, cx);
    let display_map =
        cx.add_model(|cx| DisplayMap::new(buffer, font_id, font_size, None, 1, 1, cx));
    let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
    let markers = markers
        .into_iter()
        .map(|offset| offset.to_display_point(&snapshot))
        .collect();

    (snapshot, markers)
}

pub fn select_ranges(editor: &mut Editor, marked_text: &str, cx: &mut ViewContext<Editor>) {
    let (umarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), umarked_text);
    editor.change_selections(None, cx, |s| s.select_ranges(text_ranges));
}

pub fn assert_text_with_selections(
    editor: &mut Editor,
    marked_text: &str,
    cx: &mut ViewContext<Editor>,
) {
    let (unmarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), unmarked_text);
    assert_eq!(editor.selections.ranges(cx), text_ranges);
}

pub(crate) fn build_editor(
    buffer: ModelHandle<MultiBuffer>,
    cx: &mut ViewContext<Editor>,
) -> Editor {
    Editor::new(EditorMode::Full, buffer, None, None, cx)
}

pub struct EditorTestContext<'a> {
    pub cx: &'a mut gpui::TestAppContext,
    pub window_id: usize,
    pub editor: ViewHandle<Editor>,
    pub assertion_context: AssertionContextManager,
}

impl<'a> EditorTestContext<'a> {
    pub fn new(cx: &'a mut gpui::TestAppContext) -> EditorTestContext<'a> {
        let (window_id, editor) = cx.update(|cx| {
            cx.set_global(Settings::test(cx));
            crate::init(cx);

            let (window_id, editor) = cx.add_window(Default::default(), |cx| {
                build_editor(MultiBuffer::build_simple("", cx), cx)
            });

            editor.update(cx, |_, cx| cx.focus_self());

            (window_id, editor)
        });

        Self {
            cx,
            window_id,
            editor,
            assertion_context: AssertionContextManager::new(),
        }
    }

    pub fn add_assertion_context(&self, context: String) -> ContextHandle {
        self.assertion_context.add_context(context)
    }

    pub fn condition(
        &self,
        predicate: impl FnMut(&Editor, &AppContext) -> bool,
    ) -> impl Future<Output = ()> {
        self.editor.condition(self.cx, predicate)
    }

    pub fn editor<F, T>(&self, read: F) -> T
    where
        F: FnOnce(&Editor, &AppContext) -> T,
    {
        self.editor.read_with(self.cx, read)
    }

    pub fn update_editor<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Editor, &mut ViewContext<Editor>) -> T,
    {
        self.editor.update(self.cx, update)
    }

    pub fn multibuffer<F, T>(&self, read: F) -> T
    where
        F: FnOnce(&MultiBuffer, &AppContext) -> T,
    {
        self.editor(|editor, cx| read(editor.buffer().read(cx), cx))
    }

    pub fn update_multibuffer<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut MultiBuffer, &mut ModelContext<MultiBuffer>) -> T,
    {
        self.update_editor(|editor, cx| editor.buffer().update(cx, update))
    }

    pub fn buffer_text(&self) -> String {
        self.multibuffer(|buffer, cx| buffer.snapshot(cx).text())
    }

    pub fn buffer<F, T>(&self, read: F) -> T
    where
        F: FnOnce(&Buffer, &AppContext) -> T,
    {
        self.multibuffer(|multibuffer, cx| {
            let buffer = multibuffer.as_singleton().unwrap().read(cx);
            read(buffer, cx)
        })
    }

    pub fn update_buffer<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut Buffer, &mut ModelContext<Buffer>) -> T,
    {
        self.update_multibuffer(|multibuffer, cx| {
            let buffer = multibuffer.as_singleton().unwrap();
            buffer.update(cx, update)
        })
    }

    pub fn buffer_snapshot(&self) -> BufferSnapshot {
        self.buffer(|buffer, _| buffer.snapshot())
    }

    pub fn simulate_keystroke(&mut self, keystroke_text: &str) {
        let keystroke = Keystroke::parse(keystroke_text).unwrap();
        self.cx.dispatch_keystroke(self.window_id, keystroke, false);
    }

    pub fn simulate_keystrokes<const COUNT: usize>(&mut self, keystroke_texts: [&str; COUNT]) {
        for keystroke_text in keystroke_texts.into_iter() {
            self.simulate_keystroke(keystroke_text);
        }
    }

    pub fn ranges(&self, marked_text: &str) -> Vec<Range<usize>> {
        let (unmarked_text, ranges) = marked_text_ranges(marked_text, false);
        assert_eq!(self.buffer_text(), unmarked_text);
        ranges
    }

    pub fn display_point(&mut self, marked_text: &str) -> DisplayPoint {
        let ranges = self.ranges(marked_text);
        let snapshot = self
            .editor
            .update(self.cx, |editor, cx| editor.snapshot(cx));
        ranges[0].start.to_display_point(&snapshot)
    }

    // Returns anchors for the current buffer using `«` and `»`
    pub fn text_anchor_range(&self, marked_text: &str) -> Range<language::Anchor> {
        let ranges = self.ranges(marked_text);
        let snapshot = self.buffer_snapshot();
        snapshot.anchor_before(ranges[0].start)..snapshot.anchor_after(ranges[0].end)
    }

    /// Change the editor's text and selections using a string containing
    /// embedded range markers that represent the ranges and directions of
    /// each selection.
    ///
    /// See the `util::test::marked_text_ranges` function for more information.
    pub fn set_state(&mut self, marked_text: &str) {
        let (unmarked_text, selection_ranges) = marked_text_ranges(marked_text, true);
        self.editor.update(self.cx, |editor, cx| {
            editor.set_text(unmarked_text, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.select_ranges(selection_ranges)
            })
        })
    }

    /// Make an assertion about the editor's text and the ranges and directions
    /// of its selections using a string containing embedded range markers.
    ///
    /// See the `util::test::marked_text_ranges` function for more information.
    pub fn assert_editor_state(&mut self, marked_text: &str) {
        let (unmarked_text, expected_selections) = marked_text_ranges(marked_text, true);
        let buffer_text = self.buffer_text();
        assert_eq!(
            buffer_text, unmarked_text,
            "Unmarked text doesn't match buffer text"
        );
        self.assert_selections(expected_selections, marked_text.to_string())
    }

    pub fn assert_editor_background_highlights<Tag: 'static>(&mut self, marked_text: &str) {
        let expected_ranges = self.ranges(marked_text);
        let actual_ranges: Vec<Range<usize>> = self.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            editor
                .background_highlights
                .get(&TypeId::of::<Tag>())
                .map(|h| h.1.clone())
                .unwrap_or_default()
                .into_iter()
                .map(|range| range.to_offset(&snapshot.buffer_snapshot))
                .collect()
        });
        assert_set_eq!(actual_ranges, expected_ranges);
    }

    pub fn assert_editor_text_highlights<Tag: ?Sized + 'static>(&mut self, marked_text: &str) {
        let expected_ranges = self.ranges(marked_text);
        let snapshot = self.update_editor(|editor, cx| editor.snapshot(cx));
        let actual_ranges: Vec<Range<usize>> = snapshot
            .highlight_ranges::<Tag>()
            .map(|ranges| ranges.as_ref().clone().1)
            .unwrap_or_default()
            .into_iter()
            .map(|range| range.to_offset(&snapshot.buffer_snapshot))
            .collect();
        assert_set_eq!(actual_ranges, expected_ranges);
    }

    pub fn assert_editor_selections(&mut self, expected_selections: Vec<Range<usize>>) {
        let expected_marked_text =
            generate_marked_text(&self.buffer_text(), &expected_selections, true);
        self.assert_selections(expected_selections, expected_marked_text)
    }

    fn assert_selections(
        &mut self,
        expected_selections: Vec<Range<usize>>,
        expected_marked_text: String,
    ) {
        let actual_selections = self
            .editor
            .read_with(self.cx, |editor, cx| editor.selections.all::<usize>(cx))
            .into_iter()
            .map(|s| {
                if s.reversed {
                    s.end..s.start
                } else {
                    s.start..s.end
                }
            })
            .collect::<Vec<_>>();
        let actual_marked_text =
            generate_marked_text(&self.buffer_text(), &actual_selections, true);
        if expected_selections != actual_selections {
            panic!(
                indoc! {"
                    Editor has unexpected selections.

                    Expected selections:
                    {}

                    Actual selections:
                    {}
                "},
                expected_marked_text, actual_marked_text,
            );
        }
    }
}

impl<'a> Deref for EditorTestContext<'a> {
    type Target = gpui::TestAppContext;

    fn deref(&self) -> &Self::Target {
        self.cx
    }
}

impl<'a> DerefMut for EditorTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

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

        let (window_id, workspace) =
            cx.add_window(|cx| Workspace::new(project.clone(), |_, _| unimplemented!(), cx));
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
            .update(cx, |workspace, cx| workspace.open_path(file, true, cx))
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
                assertion_context: AssertionContextManager::new(),
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

#[derive(Clone)]
pub struct AssertionContextManager {
    id: Arc<AtomicUsize>,
    contexts: Arc<RwLock<BTreeMap<usize, String>>>,
}

impl AssertionContextManager {
    pub fn new() -> Self {
        Self {
            id: Arc::new(AtomicUsize::new(0)),
            contexts: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add_context(&self, context: String) -> ContextHandle {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let mut contexts = self.contexts.write();
        contexts.insert(id, context);
        ContextHandle {
            id,
            manager: self.clone(),
        }
    }

    pub fn context(&self) -> String {
        let contexts = self.contexts.read();
        format!("\n{}\n", contexts.values().join("\n"))
    }
}

pub struct ContextHandle {
    id: usize,
    manager: AssertionContextManager,
}

impl Drop for ContextHandle {
    fn drop(&mut self) {
        let mut contexts = self.manager.contexts.write();
        contexts.remove(&self.id);
    }
}

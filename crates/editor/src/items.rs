use crate::{Editor, Event};
use crate::{MultiBuffer, ToPoint as _};
use anyhow::Result;
use gpui::{
    elements::*, AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, RenderContext,
    Subscription, Task, View, ViewContext, ViewHandle, WeakModelHandle,
};
use language::{Buffer, Diagnostic, File as _};
use postage::watch;
use project::{File, ProjectPath, Worktree};
use std::fmt::Write;
use std::path::Path;
use text::{Point, Selection};
use workspace::{
    ItemHandle, ItemView, ItemViewHandle, PathOpener, Settings, StatusItemView, WeakItemHandle,
};

pub struct BufferOpener;

#[derive(Clone)]
pub struct BufferItemHandle(pub ModelHandle<Buffer>);

#[derive(Clone)]
struct WeakBufferItemHandle(WeakModelHandle<Buffer>);

impl PathOpener for BufferOpener {
    fn open(
        &self,
        worktree: &mut Worktree,
        project_path: ProjectPath,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<Box<dyn ItemHandle>>>> {
        let buffer = worktree.open_buffer(project_path.path, cx);
        let task = cx.spawn(|_, _| async move {
            let buffer = buffer.await?;
            Ok(Box::new(BufferItemHandle(buffer)) as Box<dyn ItemHandle>)
        });
        Some(task)
    }
}

impl ItemHandle for BufferItemHandle {
    fn add_view(
        &self,
        window_id: usize,
        settings: watch::Receiver<Settings>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(self.0.clone(), cx));
        let weak_buffer = buffer.downgrade();
        Box::new(cx.add_view(window_id, |cx| {
            Editor::for_buffer(buffer, crate::settings_builder(weak_buffer, settings), cx)
        }))
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn downgrade(&self) -> Box<dyn workspace::WeakItemHandle> {
        Box::new(WeakBufferItemHandle(self.0.downgrade()))
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        File::from_dyn(self.0.read(cx).file()).map(|f| ProjectPath {
            worktree_id: f.worktree_id(cx),
            path: f.path().clone(),
        })
    }

    fn id(&self) -> usize {
        self.0.id()
    }
}

impl WeakItemHandle for WeakBufferItemHandle {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        self.0
            .upgrade(cx)
            .map(|buffer| Box::new(BufferItemHandle(buffer)) as Box<dyn ItemHandle>)
    }

    fn id(&self) -> usize {
        self.0.id()
    }
}

impl ItemView for Editor {
    type ItemHandle = BufferItemHandle;

    fn item_handle(&self, cx: &AppContext) -> Self::ItemHandle {
        BufferItemHandle(self.buffer.read(cx).as_singleton().unwrap())
    }

    fn title(&self, cx: &AppContext) -> String {
        let filename = self
            .buffer()
            .read(cx)
            .file(cx)
            .and_then(|file| file.file_name());
        if let Some(name) = filename {
            name.to_string_lossy().into()
        } else {
            "untitled".into()
        }
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        File::from_dyn(self.buffer().read(cx).file(cx)).map(|file| ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    fn clone_on_split(&self, cx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone(cx))
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).has_conflict()
    }

    fn can_save(&self, cx: &AppContext) -> bool {
        self.project_path(cx).is_some()
    }

    fn save(&mut self, cx: &mut ViewContext<Self>) -> Result<Task<Result<()>>> {
        let save = self.buffer().update(cx, |b, cx| b.save(cx))?;
        Ok(cx.spawn(|_, _| async move {
            save.await?;
            Ok(())
        }))
    }

    fn can_save_as(&self, _: &AppContext) -> bool {
        true
    }

    fn save_as(
        &mut self,
        worktree: ModelHandle<Worktree>,
        path: &Path,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("cannot call save_as on an excerpt list")
            .clone();

        buffer.update(cx, |buffer, cx| {
            let handle = cx.handle();
            let text = buffer.as_rope().clone();
            let version = buffer.version();

            let save_as = worktree.update(cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .save_buffer_as(handle, path, text, cx)
            });

            cx.spawn(|buffer, mut cx| async move {
                save_as.await.map(|new_file| {
                    let (language, language_server) = worktree.update(&mut cx, |worktree, cx| {
                        let worktree = worktree.as_local_mut().unwrap();
                        let language = worktree
                            .language_registry()
                            .select_language(new_file.full_path())
                            .cloned();
                        let language_server = language
                            .as_ref()
                            .and_then(|language| worktree.register_language(language, cx));
                        (language, language_server.clone())
                    });

                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.did_save(version, new_file.mtime, Some(Box::new(new_file)), cx);
                        buffer.set_language(language, language_server, cx);
                    });
                })
            })
        })
    }

    fn should_activate_item_on_event(event: &Event) -> bool {
        matches!(event, Event::Activate)
    }

    fn should_close_item_on_event(event: &Event) -> bool {
        matches!(event, Event::Closed)
    }

    fn should_update_tab_on_event(event: &Event) -> bool {
        matches!(
            event,
            Event::Saved | Event::Dirtied | Event::FileHandleChanged
        )
    }
}

pub struct CursorPosition {
    position: Option<Point>,
    selected_count: usize,
    settings: watch::Receiver<Settings>,
    _observe_active_editor: Option<Subscription>,
}

impl CursorPosition {
    pub fn new(settings: watch::Receiver<Settings>) -> Self {
        Self {
            position: None,
            selected_count: 0,
            settings,
            _observe_active_editor: None,
        }
    }

    fn update_position(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx).snapshot(cx);

        self.selected_count = 0;
        let mut last_selection: Option<Selection<usize>> = None;
        for selection in editor.local_selections::<usize>(cx) {
            self.selected_count += selection.end - selection.start;
            if last_selection
                .as_ref()
                .map_or(true, |last_selection| selection.id > last_selection.id)
            {
                last_selection = Some(selection);
            }
        }
        self.position = last_selection.map(|s| s.head().to_point(&buffer));

        cx.notify();
    }
}

impl Entity for CursorPosition {
    type Event = ();
}

impl View for CursorPosition {
    fn ui_name() -> &'static str {
        "CursorPosition"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        if let Some(position) = self.position {
            let theme = &self.settings.borrow().theme.workspace.status_bar;
            let mut text = format!("{},{}", position.row + 1, position.column + 1);
            if self.selected_count > 0 {
                write!(text, " ({} selected)", self.selected_count).unwrap();
            }
            Label::new(text, theme.cursor_position.clone()).boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl StatusItemView for CursorPosition {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemViewHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.to_any().downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_position));
            self.update_position(editor, cx);
        } else {
            self.position = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}

pub struct DiagnosticMessage {
    settings: watch::Receiver<Settings>,
    diagnostic: Option<Diagnostic>,
    _observe_active_editor: Option<Subscription>,
}

impl DiagnosticMessage {
    pub fn new(settings: watch::Receiver<Settings>) -> Self {
        Self {
            diagnostic: None,
            settings,
            _observe_active_editor: None,
        }
    }

    fn update(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx);
        let cursor_position = editor.newest_selection::<usize>(&buffer.read(cx)).head();
        let new_diagnostic = buffer
            .read(cx)
            .diagnostics_in_range::<_, usize>(cursor_position..cursor_position)
            .filter(|entry| !entry.range.is_empty())
            .min_by_key(|entry| (entry.diagnostic.severity, entry.range.len()))
            .map(|entry| entry.diagnostic);
        if new_diagnostic != self.diagnostic {
            self.diagnostic = new_diagnostic;
            cx.notify();
        }
    }
}

impl Entity for DiagnosticMessage {
    type Event = ();
}

impl View for DiagnosticMessage {
    fn ui_name() -> &'static str {
        "DiagnosticMessage"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        if let Some(diagnostic) = &self.diagnostic {
            let theme = &self.settings.borrow().theme.workspace.status_bar;
            Flex::row()
                .with_child(
                    Svg::new("icons/warning.svg")
                        .with_color(theme.diagnostic_icon_color)
                        .constrained()
                        .with_height(theme.diagnostic_icon_size)
                        .contained()
                        .with_margin_right(theme.diagnostic_icon_spacing)
                        .boxed(),
                )
                .with_child(
                    Label::new(
                        diagnostic.message.lines().next().unwrap().to_string(),
                        theme.diagnostic_message.clone(),
                    )
                    .boxed(),
                )
                .boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl StatusItemView for DiagnosticMessage {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemViewHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.to_any().downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update));
            self.update(editor, cx);
        } else {
            self.diagnostic = Default::default();
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}

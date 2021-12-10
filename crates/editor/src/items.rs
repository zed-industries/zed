use crate::{Editor, EditorSettings, Event};
use anyhow::Result;
use gpui::{
    elements::*, fonts::TextStyle, AppContext, Entity, ModelContext, ModelHandle,
    MutableAppContext, RenderContext, Subscription, Task, View, ViewContext, ViewHandle,
    WeakModelHandle,
};
use language::{
    multi_buffer::{MultiBuffer, ToPoint as _},
    Diagnostic, File as _,
};
use postage::watch;
use project::{ProjectPath, Worktree};
use std::fmt::Write;
use std::path::Path;
use text::{Point, Selection};
use workspace::{
    settings, EntryOpener, ItemHandle, ItemView, ItemViewHandle, Settings, StatusItemView,
    WeakItemHandle,
};

pub struct BufferOpener;

#[derive(Clone)]
pub struct BufferItemHandle(pub ModelHandle<MultiBuffer>);

#[derive(Clone)]
struct WeakBufferItemHandle(WeakModelHandle<MultiBuffer>);

impl EntryOpener for BufferOpener {
    fn open(
        &self,
        worktree: &mut Worktree,
        project_path: ProjectPath,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<Box<dyn ItemHandle>>>> {
        let buffer = worktree.open_buffer(project_path.path, cx);
        let task = cx.spawn(|_, mut cx| async move {
            let buffer = buffer.await?;
            let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
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
        let buffer = self.0.downgrade();
        Box::new(cx.add_view(window_id, |cx| {
            Editor::for_buffer(
                self.0.clone(),
                move |cx| {
                    let settings = settings.borrow();
                    let font_cache = cx.font_cache();
                    let font_family_id = settings.buffer_font_family;
                    let font_family_name = cx.font_cache().family_name(font_family_id).unwrap();
                    let font_properties = Default::default();
                    let font_id = font_cache
                        .select_font(font_family_id, &font_properties)
                        .unwrap();
                    let font_size = settings.buffer_font_size;

                    let mut theme = settings.theme.editor.clone();
                    theme.text = TextStyle {
                        color: theme.text.color,
                        font_family_name,
                        font_family_id,
                        font_id,
                        font_size,
                        font_properties,
                        underline: None,
                    };
                    let language = buffer.upgrade(cx).and_then(|buf| buf.read(cx).language(cx));
                    let soft_wrap = match settings.soft_wrap(language) {
                        settings::SoftWrap::None => crate::SoftWrap::None,
                        settings::SoftWrap::EditorWidth => crate::SoftWrap::EditorWidth,
                        settings::SoftWrap::PreferredLineLength => crate::SoftWrap::Column(
                            settings.preferred_line_length(language).saturating_sub(1),
                        ),
                    };

                    EditorSettings {
                        tab_size: settings.tab_size,
                        soft_wrap,
                        style: theme,
                    }
                },
                cx,
            )
        }))
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn downgrade(&self) -> Box<dyn workspace::WeakItemHandle> {
        Box::new(WeakBufferItemHandle(self.0.downgrade()))
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.0.read(cx).file(cx).map(|f| ProjectPath {
            worktree_id: f.worktree_id(),
            path: f.path().clone(),
        })
    }
}

impl WeakItemHandle for WeakBufferItemHandle {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        self.0
            .upgrade(cx)
            .map(|buffer| Box::new(BufferItemHandle(buffer)) as Box<dyn ItemHandle>)
    }
}

impl ItemView for Editor {
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
        self.buffer().read(cx).file(cx).map(|file| ProjectPath {
            worktree_id: file.worktree_id(),
            path: file.path().clone(),
        })
    }

    fn clone_on_split(&self, cx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone(cx))
    }

    fn save(&mut self, cx: &mut ViewContext<Self>) -> Result<Task<Result<()>>> {
        let save = self.buffer().update(cx, |b, cx| b.save(cx))?;
        Ok(cx.spawn(|_, _| async move {
            save.await?;
            Ok(())
        }))
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
                            .languages()
                            .select_language(new_file.full_path())
                            .cloned();
                        let language_server = language
                            .as_ref()
                            .and_then(|language| worktree.ensure_language_server(language, cx));
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

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).has_conflict(cx)
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
        for selection in editor.selections::<usize>(cx) {
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
        let cursor_position = editor.newest_selection::<usize>(cx).head();
        let buffer = editor.buffer().read(cx);
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

use super::{Item, ItemView};
use crate::{status_bar::StatusItemView, Settings};
use anyhow::Result;
use buffer::{Point, Selection, ToPoint};
use editor::{Editor, EditorSettings, Event};
use gpui::{
    elements::*, fonts::TextStyle, AppContext, Entity, ModelHandle, RenderContext, Subscription,
    Task, View, ViewContext, ViewHandle,
};
use language::{Buffer, Diagnostic, File as _};
use postage::watch;
use project::{ProjectPath, Worktree};
use std::fmt::Write;
use std::path::Path;

impl Item for Buffer {
    type View = Editor;

    fn build_view(
        handle: ModelHandle<Self>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self::View>,
    ) -> Self::View {
        Editor::for_buffer(
            handle,
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
                EditorSettings {
                    tab_size: settings.tab_size,
                    style: theme,
                }
            },
            cx,
        )
    }

    fn project_path(&self) -> Option<ProjectPath> {
        self.file().map(|f| ProjectPath {
            worktree_id: f.worktree_id(),
            path: f.path().clone(),
        })
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
            .file()
            .and_then(|file| file.file_name());
        if let Some(name) = filename {
            name.to_string_lossy().into()
        } else {
            "untitled".into()
        }
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.buffer().read(cx).file().map(|file| ProjectPath {
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
        self.buffer().update(cx, |buffer, cx| {
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
        self.buffer().read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).has_conflict()
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
        let buffer = editor.buffer().read(cx);

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
        self.position = last_selection.map(|s| s.head().to_point(buffer));

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
        active_pane_item: Option<&dyn crate::ItemViewHandle>,
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
        let cursor_position = editor.newest_selection(cx).head();
        let new_diagnostic = editor
            .buffer()
            .read(cx)
            .diagnostics_in_range::<usize, usize>(cursor_position..cursor_position)
            .min_by_key(|(range, diagnostic)| (diagnostic.severity, range.len()))
            .map(|(_, diagnostic)| diagnostic.clone());
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
        active_pane_item: Option<&dyn crate::ItemViewHandle>,
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

use crate::{Autoscroll, Editor, Event, MultiBuffer, NavigationData, ToOffset, ToPoint as _};
use anyhow::Result;
use gpui::{
    elements::*, AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, RenderContext,
    Subscription, Task, View, ViewContext, ViewHandle, WeakModelHandle,
};
use language::{Bias, Buffer, Diagnostic, File as _};
use postage::watch;
use project::{File, Project, ProjectPath};
use std::path::PathBuf;
use std::rc::Rc;
use std::{cell::RefCell, fmt::Write};
use text::{Point, Selection};
use util::ResultExt;
use workspace::{
    ItemHandle, ItemNavHistory, ItemView, ItemViewHandle, NavHistory, PathOpener, Settings,
    StatusItemView, WeakItemHandle, Workspace,
};

pub struct BufferOpener;

#[derive(Clone)]
pub struct BufferItemHandle(pub ModelHandle<Buffer>);

#[derive(Clone)]
struct WeakBufferItemHandle(WeakModelHandle<Buffer>);

#[derive(Clone)]
pub struct MultiBufferItemHandle(pub ModelHandle<MultiBuffer>);

#[derive(Clone)]
struct WeakMultiBufferItemHandle(WeakModelHandle<MultiBuffer>);

impl PathOpener for BufferOpener {
    fn open(
        &self,
        project: &mut Project,
        project_path: ProjectPath,
        cx: &mut ModelContext<Project>,
    ) -> Option<Task<Result<Box<dyn ItemHandle>>>> {
        let buffer = project.open_buffer(project_path, cx);
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
        workspace: &Workspace,
        nav_history: Rc<RefCell<NavHistory>>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(self.0.clone(), cx));
        let weak_buffer = buffer.downgrade();
        Box::new(cx.add_view(window_id, |cx| {
            let mut editor = Editor::for_buffer(
                buffer,
                crate::settings_builder(weak_buffer, workspace.settings()),
                Some(workspace.project().clone()),
                cx,
            );
            editor.nav_history = Some(ItemNavHistory::new(nav_history, &cx.handle()));
            editor
        }))
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn to_any(&self) -> gpui::AnyModelHandle {
        self.0.clone().into()
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

impl ItemHandle for MultiBufferItemHandle {
    fn add_view(
        &self,
        window_id: usize,
        workspace: &Workspace,
        nav_history: Rc<RefCell<NavHistory>>,
        cx: &mut MutableAppContext,
    ) -> Box<dyn ItemViewHandle> {
        let weak_buffer = self.0.downgrade();
        Box::new(cx.add_view(window_id, |cx| {
            let mut editor = Editor::for_buffer(
                self.0.clone(),
                crate::settings_builder(weak_buffer, workspace.settings()),
                Some(workspace.project().clone()),
                cx,
            );
            editor.nav_history = Some(ItemNavHistory::new(nav_history, &cx.handle()));
            editor
        }))
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn to_any(&self) -> gpui::AnyModelHandle {
        self.0.clone().into()
    }

    fn downgrade(&self) -> Box<dyn WeakItemHandle> {
        Box::new(WeakMultiBufferItemHandle(self.0.downgrade()))
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        None
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

impl WeakItemHandle for WeakMultiBufferItemHandle {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        self.0
            .upgrade(cx)
            .map(|buffer| Box::new(MultiBufferItemHandle(buffer)) as Box<dyn ItemHandle>)
    }

    fn id(&self) -> usize {
        self.0.id()
    }
}

impl ItemView for Editor {
    fn item_id(&self, cx: &AppContext) -> usize {
        if let Some(buffer) = self.buffer.read(cx).as_singleton() {
            buffer.id()
        } else {
            self.buffer.id()
        }
    }

    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) {
        if let Some(data) = data.downcast_ref::<NavigationData>() {
            let buffer = self.buffer.read(cx).read(cx);
            let offset = if buffer.can_resolve(&data.anchor) {
                data.anchor.to_offset(&buffer)
            } else {
                buffer.clip_offset(data.offset, Bias::Left)
            };

            drop(buffer);
            let nav_history = self.nav_history.take();
            self.select_ranges([offset..offset], Some(Autoscroll::Fit), cx);
            self.nav_history = nav_history;
        }
    }

    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox {
        let title = self.title(cx);
        Label::new(title, style.label.clone()).boxed()
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

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        let selection = self.newest_anchor_selection();
        self.push_to_nav_history(selection.head(), None, cx);
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).has_conflict()
    }

    fn can_save(&self, cx: &AppContext) -> bool {
        !self.buffer().read(cx).is_singleton() || self.project_path(cx).is_some()
    }

    fn save(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer().clone();
        let buffers = buffer.read(cx).all_buffers();
        let transaction = project.update(cx, |project, cx| project.format(buffers, true, cx));
        cx.spawn(|this, mut cx| async move {
            let transaction = transaction.await.log_err();
            this.update(&mut cx, |editor, cx| {
                editor.request_autoscroll(Autoscroll::Fit, cx)
            });
            buffer
                .update(&mut cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !buffer.is_singleton() {
                            buffer.push_transaction(&transaction.0);
                        }
                    }

                    buffer.save(cx)
                })
                .await?;
            Ok(())
        })
    }

    fn can_save_as(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).is_singleton()
    }

    fn save_as(
        &mut self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("cannot call save_as on an excerpt list")
            .clone();

        project.update(cx, |project, cx| {
            project.save_buffer_as(buffer, abs_path, cx)
        })
    }

    fn should_activate_item_on_event(event: &Event) -> bool {
        matches!(event, Event::Activate)
    }

    fn should_close_item_on_event(event: &Event) -> bool {
        matches!(event, Event::Closed)
    }

    fn should_update_tab_on_event(event: &Event) -> bool {
        matches!(event, Event::Saved | Event::Dirtied | Event::TitleChanged)
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
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
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
            Label::new(
                diagnostic.message.split('\n').next().unwrap().to_string(),
                theme.diagnostic_message.clone(),
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
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update));
            self.update(editor, cx);
        } else {
            self.diagnostic = Default::default();
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}

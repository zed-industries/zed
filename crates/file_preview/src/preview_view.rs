use std::mem;
use std::sync::Arc;

use file_icons::FileIcons;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    RenderImage, Styled, Subscription, Task, WeakEntity, Window, div, img,
};
use language::{Buffer, BufferEvent};
use multi_buffer::MultiBuffer;
use ui::prelude::*;
use workspace::item::Item;
use workspace::{Pane, Workspace};

use crate::formats::FilePreviewFormat;

pub struct FilePreviewView {
    pub(crate) format: Arc<dyn FilePreviewFormat>,
    focus_handle: FocusHandle,
    buffer: Option<Entity<Buffer>>,
    current_image: Option<Result<Arc<RenderImage>, SharedString>>,
    _refresh: Task<()>,
    _buffer_subscription: Option<Subscription>,
    _workspace_subscription: Option<Subscription>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PreviewMode {
    /// The preview always shows the buffer it was opened with.
    Default,
    /// The preview follows the last active editor of a matching file type.
    Follow,
}

impl FilePreviewView {
    pub fn new(
        mode: PreviewMode,
        format: Arc<dyn FilePreviewFormat>,
        active_buffer: Entity<MultiBuffer>,
        workspace_handle: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let workspace_subscription = if mode == PreviewMode::Follow
                && let Some(workspace) = workspace_handle.upgrade()
            {
                Some(Self::subscribe_to_workspace(format.clone(), workspace, window, cx))
            } else {
                None
            };

            let buffer = active_buffer.read_with(cx, |buffer, _cx| buffer.as_singleton());

            let subscription = buffer
                .as_ref()
                .map(|buffer| Self::create_buffer_subscription(buffer, window, cx));

            let mut this = Self {
                format,
                focus_handle: cx.focus_handle(),
                buffer,
                current_image: None,
                _buffer_subscription: subscription,
                _workspace_subscription: workspace_subscription,
                _refresh: Task::ready(()),
            };
            this.render_image(window, cx);

            this
        })
    }

    fn subscribe_to_workspace(
        format: Arc<dyn FilePreviewFormat>,
        workspace: Entity<Workspace>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe_in(
            &workspace,
            window,
            move |this: &mut FilePreviewView, workspace, event: &workspace::Event, window, cx| {
                if let workspace::Event::ActiveItemChanged = event {
                    let workspace = workspace.read(cx);
                    if let Some(active_item) = workspace.active_item(cx)
                        && let Some(buffer) = active_item.downcast::<MultiBuffer>()
                        && Self::is_supported_file(format.as_ref(), &buffer, cx)
                    {
                        let Some(buffer) = buffer.read(cx).as_singleton() else {
                            return;
                        };
                        if this.buffer.as_ref() != Some(&buffer) {
                            this._buffer_subscription =
                                Some(Self::create_buffer_subscription(&buffer, window, cx));
                            this.buffer = Some(buffer);
                            this.render_image(window, cx);
                            cx.notify();
                        }
                    } else {
                        this.set_current(None, window, cx);
                    }
                }
            },
        )
    }

    fn render_image(&mut self, window: &Window, cx: &mut Context<Self>) {
        let Some(buffer) = self.buffer.as_ref() else {
            return;
        };

        let renderer = cx.svg_renderer();
        let content = buffer.read(cx).snapshot().text();
        let format = self.format.clone();

        let background_task = cx.background_spawn(async move { format.render(content, renderer) });

        self._refresh = cx.spawn_in(window, async move |this, cx| {
            let result = background_task.await;

            this.update_in(cx, |view, window, cx| {
                let current = result.map_err(|e| e.to_string().into());
                view.set_current(Some(current), window, cx);
            })
            .ok();
        });
    }

    fn set_current(
        &mut self,
        image: Option<Result<Arc<RenderImage>, SharedString>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(Ok(image)) = mem::replace(&mut self.current_image, image) {
            window.drop_image(image).ok();
        }
        cx.notify();
    }

    fn create_buffer_subscription(
        buffer: &Entity<Buffer>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe_in(
            buffer,
            window,
            move |this, _buffer, event: &BufferEvent, window, cx| match event {
                BufferEvent::Edited | BufferEvent::Saved => {
                    this.render_image(window, cx);
                }
                _ => {}
            },
        )
    }

    /// Returns true if the buffer's file extension is handled by this format.
    pub fn is_supported_file(
        format: &dyn FilePreviewFormat,
        buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> bool {
        buffer
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .is_some_and(|file| {
                std::path::Path::new(file.file_name(cx))
                    .extension()
                    .is_some_and(|ext| {
                        format
                            .extensions()
                            .iter()
                            .any(|e| ext.eq_ignore_ascii_case(e))
                    })
            })
    }

    pub fn resolve_active_buffer(
        format: &dyn FilePreviewFormat,
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<MultiBuffer>> {
        workspace
            .active_item(cx)?
            .act_as::<MultiBuffer>(cx)
            .filter(|buffer| Self::is_supported_file(format, buffer, cx))
    }

    pub fn find_existing_in_pane(
        pane: &Pane,
        buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> Option<usize> {
        let buffer_id = buffer.entity_id();
        pane.items_of_type::<FilePreviewView>()
            .find(|view| {
                view.read(cx)
                    .buffer
                    .as_ref()
                    .is_some_and(|b| b.entity_id() == buffer_id)
            })
            .and_then(|view| pane.index_for_item(&view))
    }

    /// Open (or focus an existing) preview in the active pane.
    pub fn open_in_place(
        format: Arc<dyn FilePreviewFormat>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(buffer) = Self::resolve_active_buffer(format.as_ref(), workspace, cx) else {
            return;
        };
        let workspace_handle = workspace.weak_handle();
        let view = Self::new(
            PreviewMode::Default,
            format,
            buffer.clone(),
            workspace_handle,
            window,
            cx,
        );
        workspace.active_pane().update(cx, |pane, cx| {
            if let Some(idx) = Self::find_existing_in_pane(pane, &buffer, cx) {
                pane.activate_item(idx, true, true, window, cx);
            } else {
                pane.add_item(Box::new(view), true, true, None, window, cx);
            }
        });
        cx.notify();
    }

    /// Open (or focus an existing) preview in a right-side split.
    pub fn open_to_side(
        format: Arc<dyn FilePreviewFormat>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(buffer) = Self::resolve_active_buffer(format.as_ref(), workspace, cx) else {
            return;
        };
        let workspace_handle = workspace.weak_handle();
        let view = Self::new(
            PreviewMode::Default,
            format,
            buffer.clone(),
            workspace_handle,
            window,
            cx,
        );
        let pane = workspace
            .find_pane_in_direction(workspace::SplitDirection::Right, cx)
            .unwrap_or_else(|| {
                workspace.split_pane(
                    workspace.active_pane().clone(),
                    workspace::SplitDirection::Right,
                    window,
                    cx,
                )
            });
        pane.update(cx, |pane, cx| {
            if let Some(idx) = Self::find_existing_in_pane(pane, &buffer, cx) {
                pane.activate_item(idx, true, true, window, cx);
            } else {
                pane.add_item(Box::new(view), false, false, None, window, cx);
            }
        });
        cx.notify();
    }

    /// Open a "following" preview that tracks the active editor.
    pub fn open_following(
        format: Arc<dyn FilePreviewFormat>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(buffer) = Self::resolve_active_buffer(format.as_ref(), workspace, cx) else {
            return;
        };
        let workspace_handle = workspace.weak_handle();
        let view = Self::new(
            PreviewMode::Follow,
            format,
            buffer,
            workspace_handle,
            window,
            cx,
        );
        workspace.active_pane().update(cx, |pane, cx| {
            pane.add_item(Box::new(view), true, true, None, window, cx);
        });
        cx.notify();
    }
}

impl Render for FilePreviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display_name = self.format.display_name().to_string();
        let display_name_clone = display_name.clone();
        v_flex()
            .id("FilePreview")
            .key_context("FilePreview")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .flex()
            .justify_center()
            .items_center()
            .map(|this| match self.current_image.clone() {
                Some(Ok(image)) => {
                    this.child(img(image).max_w_full().max_h_full().with_fallback(move || {
                        h_flex()
                            .p_4()
                            .gap_2()
                            .child(Icon::new(IconName::Warning))
                            .child(format!("Failed to render {display_name}"))
                            .into_any_element()
                    }))
                }
                Some(Err(e)) => this.child(div().p_4().child(e).into_any_element()),
                None => this.child(
                    div()
                        .p_4()
                        .child(format!("No {display_name_clone} file selected")),
                ),
            })
    }
}

impl Focusable for FilePreviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for FilePreviewView {}

impl Item for FilePreviewView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, cx: &App) -> Option<Icon> {
        self.buffer
            .as_ref()
            .and_then(|buffer| buffer.read(cx).file())
            .and_then(|file| FileIcons::get_icon(file.path().as_std_path(), cx))
            .map(Icon::from_path)
            .or_else(|| Some(Icon::new(IconName::Image)))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.buffer
            .as_ref()
            .and_then(|buffer| buffer.read(cx).file())
            .map(|name| format!("Preview {}", name.file_name(cx)).into())
            .unwrap_or_else(|| format!("{} Preview", self.format.display_name()).into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some(self.format.telemetry_event())
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(workspace::item::ItemEvent)) {}
}

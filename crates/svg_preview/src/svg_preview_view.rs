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
use workspace::item::{Item, ItemHandle};
use workspace::{AutoPreviewMatch, AutoPreviewProvider, Pane, Workspace};

use crate::{OpenFollowingPreview, OpenPreview, OpenPreviewToTheSide};

pub struct SvgPreviewView {
    focus_handle: FocusHandle,
    buffer: Option<Entity<Buffer>>,
    current_svg: Option<Result<Arc<RenderImage>, SharedString>>,
    _refresh: Task<()>,
    _buffer_subscription: Option<Subscription>,
    _workspace_subscription: Option<Subscription>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SvgPreviewMode {
    /// The preview will always show the contents of the provided editor.
    Default,
    /// The preview will "follow" the last active editor of an SVG file.
    Follow,
    /// A single reusable auto-preview; re-pointed in place when another SVG opens.
    Auto,
}

impl SvgPreviewView {
    pub fn new(
        mode: SvgPreviewMode,
        active_buffer: Entity<MultiBuffer>,
        workspace_handle: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let workspace_subscription = if mode == SvgPreviewMode::Follow
                && let Some(workspace) = workspace_handle.upgrade()
            {
                Some(Self::subscribe_to_workspace(workspace, window, cx))
            } else {
                None
            };

            let buffer = active_buffer.read_with(cx, |buffer, _cx| buffer.as_singleton());

            let subscription = buffer
                .as_ref()
                .map(|buffer| Self::create_buffer_subscription(buffer, window, cx));

            let mut this = Self {
                focus_handle: cx.focus_handle(),
                buffer,
                current_svg: None,
                _buffer_subscription: subscription,
                _workspace_subscription: workspace_subscription,
                _refresh: Task::ready(()),
            };
            this.render_image(window, cx);

            this
        })
    }

    fn subscribe_to_workspace(
        workspace: Entity<Workspace>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe_in(
            &workspace,
            window,
            move |this: &mut SvgPreviewView, workspace, event: &workspace::Event, window, cx| {
                if let workspace::Event::ActiveItemChanged = event {
                    let workspace = workspace.read(cx);
                    if let Some(active_item) = workspace.active_item(cx)
                        && let Some(buffer) = active_item.downcast::<MultiBuffer>()
                        && Self::is_svg_file(&buffer, cx)
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
        const SCALE_FACTOR: f32 = 1.0;

        let renderer = cx.svg_renderer();
        let content = buffer.read(cx).snapshot();
        let background_task = cx.background_spawn(async move {
            renderer.render_single_frame(content.text().as_bytes(), SCALE_FACTOR)
        });

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
        if let Some(Ok(image)) = mem::replace(&mut self.current_svg, image) {
            window.drop_image(image).ok();
        }
        cx.notify();
    }

    pub fn set_buffer(
        &mut self,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(buffer) = buffer.read(cx).as_singleton() else {
            return;
        };
        if self.buffer.as_ref() == Some(&buffer) {
            return;
        }
        self._buffer_subscription = Some(Self::create_buffer_subscription(&buffer, window, cx));
        self.buffer = Some(buffer);
        self.render_image(window, cx);
        cx.notify();
    }

    fn find_existing_preview_item_idx(
        pane: &Pane,
        buffer: &Entity<MultiBuffer>,
        cx: &App,
    ) -> Option<usize> {
        let buffer_id = buffer.read(cx).as_singleton()?.entity_id();
        pane.items_of_type::<SvgPreviewView>()
            .find(|view| {
                view.read(cx)
                    .buffer
                    .as_ref()
                    .is_some_and(|buffer| buffer.entity_id() == buffer_id)
            })
            .and_then(|view| pane.index_for_item(&view))
    }

    pub fn resolve_active_item_as_svg_buffer(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<MultiBuffer>> {
        workspace
            .active_item(cx)?
            .act_as::<MultiBuffer>(cx)
            .filter(|buffer| Self::is_svg_file(&buffer, cx))
    }

    fn create_svg_view(
        mode: SvgPreviewMode,
        workspace: &mut Workspace,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<SvgPreviewView> {
        let workspace_handle = workspace.weak_handle();
        SvgPreviewView::new(mode, buffer, workspace_handle, window, cx)
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
                BufferEvent::Edited { .. } | BufferEvent::Saved => {
                    this.render_image(window, cx);
                }
                _ => {}
            },
        )
    }

    pub fn is_svg_file(buffer: &Entity<MultiBuffer>, cx: &App) -> bool {
        buffer
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .is_some_and(|file| {
                std::path::Path::new(file.file_name(cx))
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("svg"))
            })
    }

    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(buffer) = Self::resolve_active_item_as_svg_buffer(workspace, cx)
                && Self::is_svg_file(&buffer, cx)
            {
                let view = Self::create_svg_view(
                    SvgPreviewMode::Default,
                    workspace,
                    buffer.clone(),
                    window,
                    cx,
                );
                workspace.active_pane().update(cx, |pane, cx| {
                    if let Some(existing_view_idx) =
                        Self::find_existing_preview_item_idx(pane, &buffer, cx)
                    {
                        pane.activate_item(existing_view_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view), true, true, None, window, cx)
                    }
                });
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_svg_buffer(workspace, cx)
                && Self::is_svg_file(&editor, cx)
            {
                let editor_clone = editor.clone();
                let view = Self::create_svg_view(
                    SvgPreviewMode::Default,
                    workspace,
                    editor_clone,
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
                    if let Some(existing_view_idx) =
                        Self::find_existing_preview_item_idx(pane, &editor, cx)
                    {
                        pane.activate_item(existing_view_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view), false, false, None, window, cx)
                    }
                });
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenFollowingPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_svg_buffer(workspace, cx)
                && Self::is_svg_file(&editor, cx)
            {
                let view =
                    Self::create_svg_view(SvgPreviewMode::Follow, workspace, editor, window, cx);
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.add_item(Box::new(view), true, true, None, window, cx)
                });
                cx.notify();
            }
        });
    }
}

impl Render for SvgPreviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("SvgPreview")
            .key_context("SvgPreview")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .flex()
            .justify_center()
            .items_center()
            .map(|this| match self.current_svg.clone() {
                Some(Ok(image)) => {
                    this.child(img(image).max_w_full().max_h_full().with_fallback(|| {
                        h_flex()
                            .p_4()
                            .gap_2()
                            .child(Icon::new(IconName::Warning))
                            .child("Failed to load SVG image")
                            .into_any_element()
                    }))
                }
                Some(Err(e)) => this.child(div().p_4().child(e).into_any_element()),
                None => this.child(div().p_4().child("No SVG file selected")),
            })
    }
}

impl Focusable for SvgPreviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for SvgPreviewView {}

impl Item for SvgPreviewView {
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
            .and_then(|svg_path| svg_path.read(cx).file())
            .map(|name| format!("Preview {}", name.file_name(cx)).into())
            .unwrap_or_else(|| "SVG Preview".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("svg preview: open")
    }

    fn to_item_events(_event: &Self::Event, _f: &mut dyn FnMut(workspace::item::ItemEvent)) {}
}

pub struct SvgAutoPreviewProvider;

impl AutoPreviewProvider for SvgAutoPreviewProvider {
    fn id(&self) -> &'static str {
        "svg"
    }

    fn match_item(&self, item: &dyn ItemHandle, cx: &App) -> AutoPreviewMatch {
        if item.downcast::<SvgPreviewView>().is_some() {
            return AutoPreviewMatch::No;
        }
        match item.act_as::<MultiBuffer>(cx) {
            Some(buffer) if SvgPreviewView::is_svg_file(&buffer, cx) => AutoPreviewMatch::Yes,
            _ => AutoPreviewMatch::No,
        }
    }

    fn create(
        &self,
        item: &dyn ItemHandle,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Box<dyn ItemHandle>> {
        let buffer = item.act_as::<MultiBuffer>(cx)?;
        let workspace_handle = workspace.weak_handle();
        let view = SvgPreviewView::new(SvgPreviewMode::Auto, buffer, workspace_handle, window, cx);
        Some(Box::new(view))
    }

    fn swap(
        &self,
        preview: &dyn ItemHandle,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(preview) = preview.downcast::<SvgPreviewView>() else {
            return false;
        };
        let Some(buffer) = item.act_as::<MultiBuffer>(cx) else {
            return false;
        };
        if buffer.read(cx).as_singleton().is_none() {
            return false;
        }
        preview.update(cx, |preview, cx| preview.set_buffer(buffer, window, cx));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use serde_json::json;
    use std::path::PathBuf;
    use util::path;
    use workspace::{AppState, MultiWorkspace, open_paths};

    #[gpui::test]
    async fn test_svg_provider_matches_extension(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let provider = SvgAutoPreviewProvider;
            let item = cx.new(|cx| workspace::item::test::TestItem::new(cx));
            let item: Box<dyn workspace::ItemHandle> = Box::new(item);
            assert_eq!(
                provider.match_item(item.as_ref(), cx),
                workspace::AutoPreviewMatch::No
            );
        });
    }

    #[gpui::test]
    async fn test_svg_provider_matches_real_svg_item(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "image.svg": "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
                    "notes.txt": "hello",
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[
                    PathBuf::from(path!("/dir/image.svg")),
                    PathBuf::from(path!("/dir/notes.txt")),
                ],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        multi_workspace
            .update(cx, |multi_workspace, _window, cx| {
                let provider = SvgAutoPreviewProvider;
                let workspace = multi_workspace.workspace().read(cx);
                let pane = workspace.active_pane().read(cx);
                let svg_item = pane
                    .items()
                    .find(|item| {
                        item.act_as::<MultiBuffer>(cx)
                            .is_some_and(|buffer| SvgPreviewView::is_svg_file(&buffer, cx))
                    })
                    .expect("svg item should be open");
                assert_eq!(
                    provider.match_item(svg_item.as_ref(), cx),
                    AutoPreviewMatch::Yes
                );

                let txt_item = pane
                    .items()
                    .find(|item| {
                        item.act_as::<MultiBuffer>(cx)
                            .is_some_and(|buffer| !SvgPreviewView::is_svg_file(&buffer, cx))
                    })
                    .expect("non-svg item should be open");
                assert_eq!(
                    provider.match_item(txt_item.as_ref(), cx),
                    AutoPreviewMatch::No
                );
            })
            .unwrap();
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            editor::init(cx);
            crate::init(cx);
            state
        })
    }
}

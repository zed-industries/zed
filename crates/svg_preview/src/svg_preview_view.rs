use std::{io::Cursor, mem, sync::Arc};

use file_icons::FileIcons;
use gpui::{
    App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable, Image, ImageFormat,
    IntoElement, ParentElement, Render, RenderImage, Styled, Subscription, Task, WeakEntity,
    Window, div, img,
};
use language::{Buffer, BufferEvent};
use multi_buffer::MultiBuffer;
use ui::{ContextMenu, prelude::*, right_click_menu};
use workspace::item::Item;
use workspace::{Pane, Workspace};

use crate::{CopyAsImage, OpenFollowingPreview, OpenPreview, OpenPreviewToTheSide};

#[derive(Clone)]
struct RenderedSvg {
    preview_image: Arc<RenderImage>,
    clipboard_image: Arc<Image>,
}

pub struct SvgPreviewView {
    focus_handle: FocusHandle,
    buffer: Option<Entity<Buffer>>,
    current_svg: Option<Result<RenderedSvg, SharedString>>,
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
            let preview_image = renderer
                .render_single_frame(content.text().as_bytes(), SCALE_FACTOR)
                .map_err(|error| SharedString::from(error.to_string()))?;
            let clipboard_image = Arc::new(Self::clipboard_image_for_render_image(&preview_image)?);

            Ok(RenderedSvg {
                preview_image,
                clipboard_image,
            })
        });

        self._refresh = cx.spawn_in(window, async move |this, cx| {
            let result = background_task.await;

            this.update_in(cx, |view, window, cx| {
                view.set_current(Some(result), window, cx);
            })
            .ok();
        });
    }

    fn set_current(
        &mut self,
        image: Option<Result<RenderedSvg, SharedString>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(Ok(rendered_svg)) = mem::replace(&mut self.current_svg, image) {
            window.drop_image(rendered_svg.preview_image).ok();
        }
        cx.notify();
    }

    fn clipboard_image_for_render_image(render_image: &RenderImage) -> Result<Image, SharedString> {
        let size = render_image.size(0);
        let width = u32::try_from(size.width.0)
            .map_err(|_| SharedString::from("Failed to render SVG image"))?;
        let height = u32::try_from(size.height.0)
            .map_err(|_| SharedString::from("Failed to render SVG image"))?;

        if width == 0 || height == 0 {
            return Err("Failed to render SVG image".into());
        }

        let mut rgba_bytes = render_image
            .as_bytes(0)
            .ok_or_else(|| SharedString::from("Failed to render SVG image"))?
            .to_vec();

        for pixel in rgba_bytes.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        let rgba_image = image::RgbaImage::from_raw(width, height, rgba_bytes)
            .ok_or_else(|| SharedString::from("Failed to encode SVG image"))?;
        let dynamic_image = image::DynamicImage::ImageRgba8(rgba_image);
        let mut png_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut png_bytes);
        dynamic_image
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|error| SharedString::from(format!("Failed to encode SVG image: {error}")))?;

        Ok(Image::from_bytes(ImageFormat::Png, png_bytes))
    }

    fn copy_as_image(&mut self, _: &CopyAsImage, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(Ok(rendered_svg)) = self.current_svg.as_ref() {
            cx.write_to_clipboard(ClipboardItem::new_image(
                rendered_svg.clipboard_image.as_ref(),
            ));
        }
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

    pub fn open_preview_in_pane(
        workspace: &mut Workspace,
        buffer: Entity<MultiBuffer>,
        pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::activate_or_add_preview(workspace, buffer, pane, true, window, cx);
    }

    pub fn open_preview_to_the_side_of_pane(
        workspace: &mut Workspace,
        buffer: Entity<MultiBuffer>,
        origin_pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let target_pane = workspace.adjacent_pane_of(&origin_pane, window, cx);
        Self::activate_or_add_preview(workspace, buffer, target_pane, false, window, cx);
    }

    fn activate_or_add_preview(
        workspace: &mut Workspace,
        buffer: Entity<MultiBuffer>,
        pane: Entity<Pane>,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let existing_view_idx = Self::find_existing_preview_item_idx(pane.read(cx), &buffer, cx);
        if let Some(existing_view_idx) = existing_view_idx {
            pane.update(cx, |pane, cx| {
                pane.activate_item(existing_view_idx, focus, focus, window, cx);
            });
        } else {
            let view =
                Self::create_svg_view(SvgPreviewMode::Default, workspace, buffer, window, cx);
            pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(view), focus, focus, None, window, cx)
            });
        }
        cx.notify();
    }

    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(buffer) = Self::resolve_active_item_as_svg_buffer(workspace, cx) {
                let pane = workspace.active_pane().clone();
                Self::open_preview_in_pane(workspace, buffer, pane, window, cx);
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, window, cx| {
            if let Some(buffer) = Self::resolve_active_item_as_svg_buffer(workspace, cx) {
                let pane = workspace.active_pane().clone();
                Self::open_preview_to_the_side_of_pane(workspace, buffer, pane, window, cx);
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
        let focus_handle = self.focus_handle(cx);

        v_flex()
            .id("SvgPreview")
            .key_context("SvgPreview")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::copy_as_image))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .flex()
            .justify_center()
            .items_center()
            .map(|this| match self.current_svg.clone() {
                Some(Ok(rendered_svg)) => {
                    let menu_focus_handle = focus_handle.clone();

                    this.child(
                        right_click_menu("svg-preview-context-menu")
                            .trigger(move |_, _, _| {
                                img(rendered_svg.preview_image)
                                    .max_w_full()
                                    .max_h_full()
                                    .with_fallback(|| {
                                        h_flex()
                                            .p_4()
                                            .gap_2()
                                            .child(Icon::new(IconName::Warning))
                                            .child("Failed to load SVG image")
                                            .into_any_element()
                                    })
                            })
                            .menu(move |window, cx| {
                                let menu_focus_handle = menu_focus_handle.clone();

                                ContextMenu::build(window, cx, move |menu, _, _| {
                                    menu.context(menu_focus_handle)
                                        .action("Copy Image", Box::new(CopyAsImage))
                                })
                            }),
                    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_image_for_render_image_encodes_png() {
        let render_image = RenderImage::new(vec![image::Frame::new(
            image::RgbaImage::from_raw(1, 1, vec![3, 2, 1, 4]).unwrap(),
        )]);

        let clipboard_image =
            SvgPreviewView::clipboard_image_for_render_image(&render_image).unwrap();

        assert_eq!(clipboard_image.format, ImageFormat::Png);

        let decoded =
            image::load_from_memory_with_format(&clipboard_image.bytes, image::ImageFormat::Png)
                .unwrap()
                .into_rgba8();

        assert_eq!(decoded.get_pixel(0, 0).0, [1, 2, 3, 4]);
    }
}

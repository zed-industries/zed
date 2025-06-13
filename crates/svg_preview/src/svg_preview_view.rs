use std::path::PathBuf;

use editor::{Editor, EditorEvent};
use file_icons::FileIcons;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, ImageSource, IntoElement,
    ParentElement, Render, Resource, RetainAllImageCache, Styled, Subscription, WeakEntity, Window,
    div, img,
};
use ui::prelude::*;
use workspace::item::Item;
use workspace::{Pane, Workspace};

use crate::{OpenPreview, OpenPreviewToTheSide};

pub struct SvgPreviewView {
    focus_handle: FocusHandle,
    svg_path: Option<PathBuf>,
    image_cache: Entity<RetainAllImageCache>,
    _editor_subscription: Subscription,
}

impl SvgPreviewView {
    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_svg_editor(workspace, cx) {
                if Self::is_svg_file(&editor, cx) {
                    let view = Self::create_svg_view(workspace, editor, window, cx);
                    workspace.active_pane().update(cx, |pane, cx| {
                        if let Some(existing_view_idx) = Self::find_existing_preview_item_idx(pane)
                        {
                            pane.activate_item(existing_view_idx, true, true, window, cx);
                        } else {
                            pane.add_item(Box::new(view), true, true, None, window, cx)
                        }
                    });
                    cx.notify();
                }
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_svg_editor(workspace, cx) {
                if Self::is_svg_file(&editor, cx) {
                    let view = Self::create_svg_view(workspace, editor.clone(), window, cx);
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
                        if let Some(existing_view_idx) = Self::find_existing_preview_item_idx(pane)
                        {
                            pane.activate_item(existing_view_idx, true, true, window, cx);
                        } else {
                            pane.add_item(Box::new(view), false, false, None, window, cx)
                        }
                    });
                    editor.focus_handle(cx).focus(window);
                    cx.notify();
                }
            }
        });
    }

    fn find_existing_preview_item_idx(pane: &Pane) -> Option<usize> {
        pane.items_of_type::<SvgPreviewView>()
            .nth(0)
            .and_then(|view| pane.index_for_item(&view))
    }

    pub fn resolve_active_item_as_svg_editor(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<Editor>> {
        let editor = workspace.active_item(cx)?.act_as::<Editor>(cx)?;

        if Self::is_svg_file(&editor, cx) {
            Some(editor)
        } else {
            None
        }
    }

    fn create_svg_view(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<SvgPreviewView> {
        let workspace_handle = workspace.weak_handle();
        SvgPreviewView::new(editor, workspace_handle, window, cx)
    }

    pub fn new(
        active_editor: Entity<Editor>,
        _workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let svg_path = Self::get_svg_path(&active_editor, cx);
            let image_cache = RetainAllImageCache::new(cx);

            let subscription = cx.subscribe_in(
                &active_editor,
                window,
                |this: &mut SvgPreviewView, _editor, event: &EditorEvent, window, cx| {
                    match event {
                        EditorEvent::Saved => {
                            // Remove cached image to force reload
                            if let Some(svg_path) = &this.svg_path {
                                let resource = Resource::Path(svg_path.clone().into());
                                this.image_cache.update(cx, |cache, cx| {
                                    cache.remove(&resource, window, cx);
                                });
                            }
                            cx.notify();
                        }
                        _ => {}
                    }
                },
            );

            Self {
                focus_handle: cx.focus_handle(),
                svg_path,
                image_cache,
                _editor_subscription: subscription,
            }
        })
    }

    pub fn is_svg_file<V>(editor: &Entity<Editor>, cx: &mut Context<V>) -> bool {
        let buffer = editor.read(cx).buffer().read(cx);
        if let Some(buffer) = buffer.as_singleton() {
            if let Some(file) = buffer.read(cx).file() {
                return file
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("svg"))
                    .unwrap_or(false);
            }
        }
        false
    }

    fn get_svg_path<V>(editor: &Entity<Editor>, cx: &mut Context<V>) -> Option<PathBuf> {
        let buffer = editor.read(cx).buffer().read(cx);
        if let Some(buffer) = buffer.as_singleton() {
            if let Some(file) = buffer.read(cx).file() {
                return Some(file.path().to_path_buf());
            }
        }
        None
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
            .child(if let Some(svg_path) = &self.svg_path {
                img(ImageSource::from(svg_path.clone()))
                    .image_cache(&self.image_cache)
                    .max_w_full()
                    .max_h_full()
                    .with_fallback(|| {
                        div()
                            .p_4()
                            .child("Failed to load SVG file")
                            .into_any_element()
                    })
                    .into_any_element()
            } else {
                div().p_4().child("No SVG file selected").into_any_element()
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
        // Use the same icon as SVG files in the file tree
        self.svg_path
            .as_ref()
            .and_then(|svg_path| FileIcons::get_icon(svg_path, cx))
            .map(Icon::from_path)
            .or_else(|| Some(Icon::new(IconName::Image)))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.svg_path
            .as_ref()
            .and_then(|svg_path| svg_path.file_name())
            .map(|name| name.to_string_lossy())
            .map(|name| format!("Preview {}", name).into())
            .unwrap_or_else(|| "SVG Preview".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("svg preview: open")
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(workspace::item::ItemEvent)) {}
}

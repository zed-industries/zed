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

use crate::{OpenFollowingPreview, OpenPreview, OpenPreviewToTheSide};

pub struct SvgPreviewView {
    focus_handle: FocusHandle,
    svg_path: Option<PathBuf>,
    image_cache: Entity<RetainAllImageCache>,
    _editor_subscription: Subscription,
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
    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_svg_editor(workspace, cx)
                && Self::is_svg_file(&editor, cx)
            {
                let view = Self::create_svg_view(
                    SvgPreviewMode::Default,
                    workspace,
                    editor.clone(),
                    window,
                    cx,
                );
                workspace.active_pane().update(cx, |pane, cx| {
                    if let Some(existing_view_idx) =
                        Self::find_existing_preview_item_idx(pane, &editor, cx)
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
            if let Some(editor) = Self::resolve_active_item_as_svg_editor(workspace, cx)
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
            if let Some(editor) = Self::resolve_active_item_as_svg_editor(workspace, cx)
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

    fn find_existing_preview_item_idx(
        pane: &Pane,
        editor: &Entity<Editor>,
        cx: &App,
    ) -> Option<usize> {
        let editor_path = Self::get_svg_path(editor, cx);
        pane.items_of_type::<SvgPreviewView>()
            .find(|view| {
                let view_read = view.read(cx);
                view_read.svg_path.is_some() && view_read.svg_path == editor_path
            })
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
        mode: SvgPreviewMode,
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<SvgPreviewView> {
        let workspace_handle = workspace.weak_handle();
        SvgPreviewView::new(mode, editor, workspace_handle, window, cx)
    }

    pub fn new(
        mode: SvgPreviewMode,
        active_editor: Entity<Editor>,
        workspace_handle: WeakEntity<Workspace>,
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
                    if event == &EditorEvent::Saved {
                        // Remove cached image to force reload
                        if let Some(svg_path) = &this.svg_path {
                            let resource = Resource::Path(svg_path.clone().into());
                            this.image_cache.update(cx, |cache, cx| {
                                cache.remove(&resource, window, cx);
                            });
                        }
                        cx.notify();
                    }
                },
            );

            // Subscribe to workspace active item changes to follow SVG files
            let workspace_subscription = if mode == SvgPreviewMode::Follow {
                workspace_handle.upgrade().map(|workspace_handle| {
                    cx.subscribe_in(
                        &workspace_handle,
                        window,
                        |this: &mut SvgPreviewView,
                         workspace,
                         event: &workspace::Event,
                         _window,
                         cx| {
                            if let workspace::Event::ActiveItemChanged = event {
                                let workspace_read = workspace.read(cx);
                                if let Some(active_item) = workspace_read.active_item(cx)
                                    && let Some(editor_entity) = active_item.downcast::<Editor>()
                                    && Self::is_svg_file(&editor_entity, cx)
                                {
                                    let new_path = Self::get_svg_path(&editor_entity, cx);
                                    if this.svg_path != new_path {
                                        this.svg_path = new_path;
                                        cx.notify();
                                    }
                                }
                            }
                        },
                    )
                })
            } else {
                None
            };

            Self {
                focus_handle: cx.focus_handle(),
                svg_path,
                image_cache,
                _editor_subscription: subscription,
                _workspace_subscription: workspace_subscription,
            }
        })
    }

    pub fn is_svg_file<C>(editor: &Entity<Editor>, cx: &C) -> bool
    where
        C: std::borrow::Borrow<App>,
    {
        let app = cx.borrow();
        let buffer = editor.read(app).buffer().read(app);
        if let Some(buffer) = buffer.as_singleton()
            && let Some(file) = buffer.read(app).file()
        {
            return file
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("svg"))
                .unwrap_or(false);
        }
        false
    }

    fn get_svg_path<C>(editor: &Entity<Editor>, cx: &C) -> Option<PathBuf>
    where
        C: std::borrow::Borrow<App>,
    {
        let app = cx.borrow();
        let buffer = editor.read(app).buffer().read(app).as_singleton()?;
        let file = buffer.read(app).file()?;
        let local_file = file.as_local()?;
        Some(local_file.abs_path(app))
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

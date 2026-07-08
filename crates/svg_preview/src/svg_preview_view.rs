use std::mem;
use std::sync::Arc;

use anyhow::Result;
use editor::Editor;
use file_icons::FileIcons;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    RenderImage, Styled, Subscription, Task, WeakEntity, Window, div, img,
};
use language::{Buffer, BufferEvent};
use multi_buffer::MultiBuffer;
use project::{Project, ProjectEntryId, ProjectPath};
use settings::Settings as _;
use ui::prelude::*;
use workspace::item::{Item, ItemBufferKind, ProjectItem};
use workspace::{AutoPreview, Pane, Workspace, WorkspaceSettings};
use zed_actions::preview::{OpenSource, Toggle, TogglePlacement};

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
                    // When the active item is not an SVG buffer, keep showing the last
                    // previewed file instead of blanking the view.
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

    pub fn is_svg_path(path: impl AsRef<std::path::Path>) -> bool {
        path.as_ref()
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"))
    }

    fn is_following(&self) -> bool {
        self._workspace_subscription.is_some()
    }

    pub(crate) fn auto_preview_provider() -> workspace::AutoPreviewProvider {
        workspace::AutoPreviewProvider {
            applies_to: |item, cx| {
                item.downcast::<Editor>()
                    .is_some_and(|editor| Self::is_svg_file(editor.read(cx).buffer(), cx))
            },
            has_open_sources: |workspace, cx| {
                workspace
                    .items_of_type::<Editor>(cx)
                    .any(|editor| Self::is_svg_file(editor.read(cx).buffer(), cx))
            },
            is_follow_view: |item, cx| {
                item.downcast::<SvgPreviewView>()
                    .is_some_and(|view| view.read(cx).is_following())
            },
            is_preview_view: |item, cx| {
                item.downcast::<SvgPreviewView>()
                    .is_some_and(|view| !view.read(cx).is_following())
            },
            build_follow_view: |workspace, window, cx| {
                let buffer = Self::resolve_active_item_as_svg_buffer(workspace, cx)?;
                Some(Box::new(Self::create_svg_view(
                    SvgPreviewMode::Follow,
                    workspace,
                    buffer,
                    window,
                    cx,
                )))
            },
            build_preview_view: |workspace, item, window, cx| {
                let editor = item.downcast::<Editor>()?;
                let buffer = editor.read(cx).buffer().clone();
                Some(Box::new(Self::create_svg_view(
                    SvgPreviewMode::Default,
                    workspace,
                    buffer,
                    window,
                    cx,
                )))
            },
            source_view: |workspace, item, window, cx| {
                let preview = item.downcast::<SvgPreviewView>()?;
                let buffer = preview.read(cx).buffer.clone()?;
                let existing_editor = workspace.items_of_type::<Editor>(cx).find(|editor| {
                    editor.read(cx).buffer().read(cx).as_singleton().as_ref() == Some(&buffer)
                });
                let editor = existing_editor.unwrap_or_else(|| {
                    let project = workspace.project().clone();
                    cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx))
                });
                Some(Box::new(editor))
            },
        }
    }

    /// Opens (or reveals) a preview for the active SVG editor.
    /// Returns false when the active item is not an SVG editor.
    fn open_preview_for_active_editor(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        let Some(buffer) = Self::resolve_active_item_as_svg_buffer(workspace, cx) else {
            return false;
        };
        let view = Self::create_svg_view(
            SvgPreviewMode::Default,
            workspace,
            buffer.clone(),
            window,
            cx,
        );
        workspace.active_pane().update(cx, |pane, cx| {
            if let Some(existing_view_idx) = Self::find_existing_preview_item_idx(pane, &buffer, cx)
            {
                pane.activate_item(existing_view_idx, true, true, window, cx);
            } else {
                pane.add_item(Box::new(view), true, true, None, window, cx)
            }
        });
        cx.notify();
        true
    }

    /// Activates (or opens) a text editor for the active SVG preview.
    /// Returns false when the active item is not an SVG preview.
    fn open_source_for_active_preview(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        let Some(preview) = workspace
            .active_item(cx)
            .and_then(|item| item.downcast::<SvgPreviewView>())
        else {
            return false;
        };
        let Some(buffer) = preview.read(cx).buffer.clone() else {
            return true;
        };
        let existing_editor = workspace.items_of_type::<Editor>(cx).find(|editor| {
            editor.read(cx).buffer().read(cx).as_singleton().as_ref() == Some(&buffer)
        });
        if let Some(editor) = existing_editor {
            workspace.activate_item(&editor, true, true, window, cx);
        } else {
            let project = workspace.project().clone();
            let editor = cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(Box::new(editor), true, true, None, window, cx);
            });
        }
        true
    }

    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            Self::open_preview_for_active_editor(workspace, window, cx);
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

        workspace.register_action(move |workspace, _: &OpenSource, window, cx| {
            if !Self::open_source_for_active_preview(workspace, window, cx) {
                cx.propagate();
            }
        });

        workspace.register_action(move |workspace, action: &Toggle, window, cx| {
            let handled = match action.placement {
                TogglePlacement::InPlace => {
                    Self::open_source_for_active_preview(workspace, window, cx)
                        || Self::open_preview_for_active_editor(workspace, window, cx)
                }
                TogglePlacement::ToTheSide => {
                    workspace::show_side_preview_for_active_item(workspace, window, cx)
                }
            };
            if !handled {
                cx.propagate();
            }
        });
    }
}

/// A [`project::ProjectItem`] that claims SVG files when the `auto_preview` setting
/// is set to `in_place`, so that opening such files shows their rendered preview instead of an editor.
pub struct SvgPreviewItem {
    buffer: Entity<Buffer>,
}

impl project::ProjectItem for SvgPreviewItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        if WorkspaceSettings::get_global(cx).auto_preview != AutoPreview::InPlace
            || !project
                .read(cx)
                .absolute_path(path, cx)
                .is_some_and(SvgPreviewView::is_svg_path)
        {
            return None;
        }
        let buffer = project.update(cx, |project, cx| project.open_buffer(path.clone(), cx));
        Some(cx.spawn(async move |cx| {
            let buffer = buffer.await?;
            Ok(cx.new(|_| SvgPreviewItem { buffer }))
        }))
    }

    fn entry_id(&self, cx: &App) -> Option<ProjectEntryId> {
        project::ProjectItem::entry_id(self.buffer.read(cx), cx)
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        project::ProjectItem::project_path(self.buffer.read(cx), cx)
    }

    fn is_dirty(&self) -> bool {
        // This item is only a carrier between `try_open` and `for_project_item`: the
        // preview reports its dirty state through the buffer it renders.
        false
    }
}

impl ProjectItem for SvgPreviewView {
    type Item = SvgPreviewItem;

    fn for_project_item(
        _project: Entity<Project>,
        _pane: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = item.read(cx).buffer.clone();
        let subscription = Self::create_buffer_subscription(&buffer, window, cx);
        let mut this = Self {
            focus_handle: cx.focus_handle(),
            buffer: Some(buffer),
            current_svg: None,
            _buffer_subscription: Some(subscription),
            _workspace_subscription: None,
            _refresh: Task::ready(()),
        };
        this.render_image(window, cx);
        this
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

    fn buffer_kind(&self, _cx: &App) -> ItemBufferKind {
        ItemBufferKind::Singleton
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.buffer
            .as_ref()
            .is_some_and(|buffer| buffer.read(cx).is_dirty())
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        // Previews that follow the active editor are not bound to a single file.
        if self.is_following() {
            return;
        }
        if let Some(buffer) = &self.buffer {
            f(buffer.entity_id(), buffer.read(cx))
        }
    }

    fn to_item_events(_event: &Self::Event, _f: &mut dyn FnMut(workspace::item::ItemEvent)) {}
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;

    use editor::Editor;
    use gpui::{BorrowAppContext as _, Focusable as _, TestAppContext, WindowHandle};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use util::rel_path::rel_path;
    use workspace::{AppState, AutoPreview, MultiWorkspace, open_paths};
    use zed_actions::preview::OpenSource;

    use super::SvgPreviewView;

    const SVG_CONTENTS: &str = r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#;

    #[gpui::test]
    async fn auto_preview_opens_svg_files_as_preview(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        set_auto_preview(cx, AutoPreview::InPlace);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "a.svg": SVG_CONTENTS,
                    "b.txt": "plain text",
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        cx.run_until_parked();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        open_workspace_path(&multi_workspace, "a.svg", cx).await;
        let preview = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(workspace.active_pane().read(cx).items_len(), 1);
                    let preview = workspace
                        .active_item(cx)
                        .and_then(|item| item.downcast::<SvgPreviewView>())
                        .expect("SVG file should have been opened as a preview");
                    assert!(
                        preview.read(cx).focus_handle.contains_focused(window, cx),
                        "the opened preview should be focused"
                    );
                    preview
                })
            })
            .unwrap();
        assert_eq!(
            preview
                .read_with(cx, |preview, cx| preview
                    .buffer
                    .as_ref()
                    .unwrap()
                    .read(cx)
                    .file()
                    .unwrap()
                    .path()
                    .clone())
                .as_ref(),
            rel_path("a.svg")
        );

        // Reopening the file should reuse the existing preview.
        open_workspace_path(&multi_workspace, "a.svg", cx).await;
        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(workspace.active_pane().read(cx).items_len(), 1);
                    assert_eq!(
                        workspace
                            .active_item(cx)
                            .and_then(|item| item.downcast::<SvgPreviewView>()),
                        Some(preview.clone())
                    );
                })
            })
            .unwrap();

        open_workspace_path(&multi_workspace, "b.txt", cx).await;
        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(workspace.active_pane().read(cx).items_len(), 2);
                    assert!(
                        workspace
                            .active_item(cx)
                            .and_then(|item| item.downcast::<Editor>())
                            .is_some(),
                        "non-previewable files should still open in an editor"
                    );
                })
            })
            .unwrap();
    }

    #[gpui::test]
    async fn open_source_opens_an_editor_for_the_previewed_file(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        set_auto_preview(cx, AutoPreview::InPlace);
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/dir"), json!({ "a.svg": SVG_CONTENTS }))
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        cx.run_until_parked();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        open_workspace_path(&multi_workspace, "a.svg", cx).await;
        multi_workspace
            .update(cx, |_, window, cx| {
                window.dispatch_action(Box::new(OpenSource), cx);
            })
            .unwrap();
        cx.run_until_parked();

        let editor = multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(
                        workspace.active_pane().read(cx).items_len(),
                        2,
                        "an editor should have been added next to the preview"
                    );
                    workspace
                        .active_item(cx)
                        .and_then(|item| item.downcast::<Editor>())
                        .expect("the editor for the previewed file should be active")
                })
            })
            .unwrap();
        let editor_path = editor.read_with(cx, |editor, cx| {
            editor
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap()
                .read(cx)
                .file()
                .unwrap()
                .path()
                .clone()
        });
        assert_eq!(editor_path.as_ref(), rel_path("a.svg"));
    }

    #[gpui::test]
    async fn svg_files_open_in_an_editor_by_default(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/dir"), json!({ "a.svg": SVG_CONTENTS }))
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        cx.run_until_parked();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        open_workspace_path(&multi_workspace, "a.svg", cx).await;
        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert!(
                        workspace
                            .active_item(cx)
                            .and_then(|item| item.downcast::<Editor>())
                            .is_some(),
                        "with auto_preview off, SVG files should open in an editor"
                    );
                })
            })
            .unwrap();
    }

    #[gpui::test]
    async fn to_the_side_auto_preview_follows_editors_and_closes_with_them(
        cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        set_auto_preview(cx, AutoPreview::ToTheSide);
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/dir"), json!({ "a.svg": SVG_CONTENTS }))
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        cx.run_until_parked();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        open_workspace_path(&multi_workspace, "a.svg", cx).await;
        multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let editor = workspace
                        .active_item(cx)
                        .and_then(|item| item.downcast::<Editor>())
                        .expect("SVG files should still open in an editor");
                    assert!(
                        editor
                            .read(cx)
                            .focus_handle(cx)
                            .contains_focused(window, cx),
                        "the editor should keep the focus"
                    );
                    assert_eq!(
                        workspace.panes().len(),
                        2,
                        "a pane should have been split for the preview"
                    );
                    let preview = workspace
                        .items_of_type::<SvgPreviewView>(cx)
                        .next()
                        .expect("a preview should have been opened to the side");
                    assert!(preview.read(cx).is_following());
                    assert_ne!(
                        workspace.pane_for(&preview),
                        Some(workspace.active_pane().clone()),
                        "the preview should live in the other pane"
                    );
                })
            })
            .unwrap();

        multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let editors = workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>();
                    for editor in editors {
                        let pane = workspace.pane_for(&editor).unwrap();
                        pane.update(cx, |pane, cx| {
                            pane.remove_item(editor.entity_id(), false, true, window, cx)
                        });
                    }
                })
            })
            .unwrap();
        cx.run_until_parked();

        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(
                        workspace.items_of_type::<SvgPreviewView>(cx).count(),
                        0,
                        "the preview should close when no SVG editors remain"
                    );
                    assert_eq!(
                        workspace.panes().len(),
                        1,
                        "the preview pane should be removed with the preview"
                    );
                })
            })
            .unwrap();
    }

    #[gpui::test]
    async fn single_side_preview_is_shared_between_preview_kinds(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        cx.update(markdown_preview::init);
        set_auto_preview(cx, AutoPreview::ToTheSide);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "a.md": "# a",
                    "b.svg": SVG_CONTENTS,
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        cx.run_until_parked();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        open_workspace_path(&multi_workspace, "a.md", cx).await;
        let markdown_preview_pane = multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    let preview = workspace
                        .items_of_type::<markdown_preview::markdown_preview_view::MarkdownPreviewView>(cx)
                        .next()
                        .expect("a markdown preview should have been opened to the side");
                    let pane = workspace.pane_for(&preview).unwrap();
                    assert_eq!(pane.read(cx).items_len(), 1);
                    pane
                })
            })
            .unwrap();

        open_workspace_path(&multi_workspace, "b.svg", cx).await;
        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(
                        workspace.panes().len(),
                        2,
                        "the preview pane should be reused for the other preview kind"
                    );
                    assert_eq!(
                        workspace
                            .items_of_type::<markdown_preview::markdown_preview_view::MarkdownPreviewView>(cx)
                            .count(),
                        0,
                        "the markdown preview should have been replaced"
                    );
                    let preview = workspace
                        .items_of_type::<SvgPreviewView>(cx)
                        .next()
                        .expect("an SVG preview should have taken the preview tab slot");
                    assert!(preview.read(cx).is_following());
                    let pane = workspace.pane_for(&preview).unwrap();
                    assert_eq!(pane, markdown_preview_pane);
                    assert_eq!(
                        pane.read(cx).items_len(),
                        1,
                        "a single dynamic preview tab should be kept to the side"
                    );
                })
            })
            .unwrap();

        open_workspace_path(&multi_workspace, "a.md", cx).await;
        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    assert_eq!(workspace.items_of_type::<SvgPreviewView>(cx).count(), 0);
                    let preview = workspace
                        .items_of_type::<markdown_preview::markdown_preview_view::MarkdownPreviewView>(cx)
                        .next()
                        .expect("the markdown preview should be back in the preview tab slot");
                    let pane = workspace.pane_for(&preview).unwrap();
                    assert_eq!(pane.read(cx).items_len(), 1);
                })
            })
            .unwrap();
    }

    fn set_auto_preview(cx: &mut TestAppContext, auto_preview: AutoPreview) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.workspace.auto_preview = Some(auto_preview);
                });
            });
        });
    }

    async fn open_workspace_path(
        multi_workspace: &WindowHandle<MultiWorkspace>,
        file: &str,
        cx: &mut TestAppContext,
    ) {
        let open_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let worktree_id = workspace
                        .project()
                        .read(cx)
                        .worktrees(cx)
                        .next()
                        .unwrap()
                        .read(cx)
                        .id();
                    workspace.open_path((worktree_id, rel_path(file)), None, true, window, cx)
                })
            })
            .unwrap();
        open_task.await.unwrap();
        cx.run_until_parked();
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

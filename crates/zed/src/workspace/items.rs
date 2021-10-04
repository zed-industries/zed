use super::{Item, ItemView};
use crate::{project::ProjectPath, Settings};
use anyhow::Result;
use buffer::{Buffer, File as _};
use editor::{Editor, EditorSettings, Event};
use gpui::{fonts::TextStyle, AppContext, ModelHandle, Task, ViewContext};
use postage::watch;
use std::path::Path;
use worktree::Worktree;

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
                    underline: false,
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
            .and_then(|file| file.file_name(cx));
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
                    let language = worktree.read_with(&cx, |worktree, cx| {
                        worktree
                            .languages()
                            .select_language(new_file.full_path(cx))
                            .cloned()
                    });

                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.did_save(version, new_file.mtime, Some(Box::new(new_file)), cx);
                        buffer.set_language(language, cx);
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

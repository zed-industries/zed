use acp_thread::{MentionUri, selection_name};
use agent::{ThreadStore, outline};
use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::{Context as _, Result, anyhow};
use assistant_slash_commands::{codeblock_fence_for_path, collect_diagnostics_output};
use collections::{HashMap, HashSet};
use editor::{
    Anchor, Editor, EditorSnapshot, ExcerptId, FoldPlaceholder, ToOffset,
    display_map::{Crease, CreaseId, CreaseMetadata, FoldId},
    scroll::Autoscroll,
};
use futures::{AsyncReadExt as _, FutureExt as _, future::Shared};
use gpui::{
    AppContext, ClipboardEntry, Context, Empty, Entity, EntityId, Image, ImageFormat, Img,
    SharedString, Task, WeakEntity,
};
use http_client::{AsyncBody, HttpClientWithUrl};
use itertools::Either;
use language::Buffer;
use language_model::LanguageModelImage;
use multi_buffer::MultiBufferRow;
use postage::stream::Stream as _;
use project::{Project, ProjectItem, ProjectPath, Worktree};
use prompt_store::{PromptId, PromptStore};
use rope::Point;
use std::{
    cell::RefCell,
    ffi::OsStr,
    fmt::Write,
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use text::OffsetRangeExt;
use ui::{Disclosure, Toggleable, prelude::*};
use util::{ResultExt, debug_panic, rel_path::RelPath};
use workspace::{Workspace, notifications::NotifyResultExt as _};

use crate::ui::MentionCrease;

pub type MentionTask = Shared<Task<Result<Mention, String>>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mention {
    Text {
        content: String,
        tracked_buffers: Vec<Entity<Buffer>>,
    },
    Image(MentionImage),
    Link,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MentionImage {
    pub data: SharedString,
    pub format: ImageFormat,
}

pub struct MentionSet {
    project: WeakEntity<Project>,
    thread_store: Option<Entity<ThreadStore>>,
    prompt_store: Option<Entity<PromptStore>>,
    mentions: HashMap<CreaseId, (MentionUri, MentionTask)>,
}

impl MentionSet {
    pub fn new(
        project: WeakEntity<Project>,
        thread_store: Option<Entity<ThreadStore>>,
        prompt_store: Option<Entity<PromptStore>>,
    ) -> Self {
        Self {
            project,
            thread_store,
            prompt_store,
            mentions: HashMap::default(),
        }
    }

    pub fn contents(
        &self,
        full_mention_content: bool,
        cx: &mut App,
    ) -> Task<Result<HashMap<CreaseId, (MentionUri, Mention)>>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("Project not found")));
        };
        let mentions = self.mentions.clone();
        cx.spawn(async move |cx| {
            let mut contents = HashMap::default();
            for (crease_id, (mention_uri, task)) in mentions {
                let content = if full_mention_content
                    && let MentionUri::Directory { abs_path } = &mention_uri
                {
                    cx.update(|cx| full_mention_for_directory(&project, abs_path, cx))
                        .await?
                } else {
                    task.await.map_err(|e| anyhow!("{e}"))?
                };

                contents.insert(crease_id, (mention_uri, content));
            }
            Ok(contents)
        })
    }

    pub fn remove_invalid(&mut self, snapshot: &EditorSnapshot) {
        for (crease_id, crease) in snapshot.crease_snapshot.creases() {
            if !crease.range().start.is_valid(snapshot.buffer_snapshot()) {
                self.mentions.remove(&crease_id);
            }
        }
    }

    pub fn insert_mention(&mut self, crease_id: CreaseId, uri: MentionUri, task: MentionTask) {
        self.mentions.insert(crease_id, (uri, task));
    }

    pub fn remove_mention(&mut self, crease_id: &CreaseId) {
        self.mentions.remove(crease_id);
    }

    pub fn creases(&self) -> HashSet<CreaseId> {
        self.mentions.keys().cloned().collect()
    }

    pub fn mentions(&self) -> HashSet<MentionUri> {
        self.mentions.values().map(|(uri, _)| uri.clone()).collect()
    }

    pub fn set_mentions(&mut self, mentions: HashMap<CreaseId, (MentionUri, MentionTask)>) {
        self.mentions = mentions;
    }

    pub fn clear(&mut self) -> impl Iterator<Item = (CreaseId, (MentionUri, MentionTask))> {
        self.mentions.drain()
    }

    #[cfg(test)]
    pub fn has_thread_store(&self) -> bool {
        self.thread_store.is_some()
    }

    pub fn confirm_mention_completion(
        &mut self,
        crease_text: SharedString,
        start: text::Anchor,
        content_len: usize,
        mention_uri: MentionUri,
        supports_images: bool,
        editor: Entity<Editor>,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(());
        };

        let snapshot = editor.update(cx, |editor, cx| editor.snapshot(window, cx));
        let Some(start_anchor) = snapshot.buffer_snapshot().as_singleton_anchor(start) else {
            return Task::ready(());
        };
        let excerpt_id = start_anchor.excerpt_id;
        let end_anchor = snapshot.buffer_snapshot().anchor_before(
            start_anchor.to_offset(&snapshot.buffer_snapshot()) + content_len + 1usize,
        );

        let crease = if let MentionUri::File { abs_path } = &mention_uri
            && let Some(extension) = abs_path.extension()
            && let Some(extension) = extension.to_str()
            && Img::extensions().contains(&extension)
            && !extension.contains("svg")
        {
            let Some(project_path) = project
                .read(cx)
                .project_path_for_absolute_path(&abs_path, cx)
            else {
                log::error!("project path not found");
                return Task::ready(());
            };
            let image_task = project.update(cx, |project, cx| project.open_image(project_path, cx));
            let image = cx
                .spawn(async move |_, cx| {
                    let image = image_task.await.map_err(|e| e.to_string())?;
                    let image = image.update(cx, |image, _| image.image.clone());
                    Ok(image)
                })
                .shared();
            insert_crease_for_mention(
                excerpt_id,
                start,
                content_len,
                mention_uri.name().into(),
                IconName::Image.path().into(),
                Some(image),
                editor.clone(),
                window,
                cx,
            )
        } else {
            insert_crease_for_mention(
                excerpt_id,
                start,
                content_len,
                crease_text,
                mention_uri.icon_path(cx),
                None,
                editor.clone(),
                window,
                cx,
            )
        };
        let Some((crease_id, tx)) = crease else {
            return Task::ready(());
        };

        let task = match mention_uri.clone() {
            MentionUri::Fetch { url } => {
                self.confirm_mention_for_fetch(url, workspace.read(cx).client().http_client(), cx)
            }
            MentionUri::Directory { .. } => Task::ready(Ok(Mention::Link)),
            MentionUri::Thread { id, .. } => self.confirm_mention_for_thread(id, cx),
            MentionUri::TextThread { .. } => {
                Task::ready(Err(anyhow!("Text thread mentions are no longer supported")))
            }
            MentionUri::File { abs_path } => {
                self.confirm_mention_for_file(abs_path, supports_images, cx)
            }
            MentionUri::Symbol {
                abs_path,
                line_range,
                ..
            } => self.confirm_mention_for_symbol(abs_path, line_range, cx),
            MentionUri::Rule { id, .. } => self.confirm_mention_for_rule(id, cx),
            MentionUri::Diagnostics {
                include_errors,
                include_warnings,
            } => self.confirm_mention_for_diagnostics(include_errors, include_warnings, cx),
            MentionUri::PastedImage => {
                debug_panic!("pasted image URI should not be included in completions");
                Task::ready(Err(anyhow!(
                    "pasted imaged URI should not be included in completions"
                )))
            }
            MentionUri::Selection { .. } => {
                debug_panic!("unexpected selection URI");
                Task::ready(Err(anyhow!("unexpected selection URI")))
            }
        };
        let task = cx
            .spawn(async move |_, _| task.await.map_err(|e| e.to_string()))
            .shared();
        self.mentions.insert(crease_id, (mention_uri, task.clone()));

        // Notify the user if we failed to load the mentioned context
        cx.spawn_in(window, async move |this, cx| {
            let result = task.await.notify_async_err(cx);
            drop(tx);
            if result.is_none() {
                this.update(cx, |this, cx| {
                    editor.update(cx, |editor, cx| {
                        // Remove mention
                        editor.edit([(start_anchor..end_anchor, "")], cx);
                    });
                    this.mentions.remove(&crease_id);
                })
                .ok();
            }
        })
    }

    pub fn confirm_mention_for_file(
        &self,
        abs_path: PathBuf,
        supports_images: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("project not found")));
        };

        let Some(project_path) = project
            .read(cx)
            .project_path_for_absolute_path(&abs_path, cx)
        else {
            return Task::ready(Err(anyhow!("project path not found")));
        };
        let extension = abs_path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        if Img::extensions().contains(&extension) && !extension.contains("svg") {
            if !supports_images {
                return Task::ready(Err(anyhow!("This model does not support images yet")));
            }
            let task = project.update(cx, |project, cx| project.open_image(project_path, cx));
            return cx.spawn(async move |_, cx| {
                let image = task.await?;
                let image = image.update(cx, |image, _| image.image.clone());
                let image = cx
                    .update(|cx| LanguageModelImage::from_image(image, cx))
                    .await;
                if let Some(image) = image {
                    Ok(Mention::Image(MentionImage {
                        data: image.source,
                        format: LanguageModelImage::FORMAT,
                    }))
                } else {
                    Err(anyhow!("Failed to convert image"))
                }
            });
        }

        let buffer = project.update(cx, |project, cx| project.open_buffer(project_path, cx));
        cx.spawn(async move |_, cx| {
            let buffer = buffer.await?;
            let buffer_content = outline::get_buffer_content_or_outline(
                buffer.clone(),
                Some(&abs_path.to_string_lossy()),
                &cx,
            )
            .await?;

            Ok(Mention::Text {
                content: buffer_content.text,
                tracked_buffers: vec![buffer],
            })
        })
    }

    fn confirm_mention_for_fetch(
        &self,
        url: url::Url,
        http_client: Arc<HttpClientWithUrl>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        cx.background_executor().spawn(async move {
            let content = fetch_url_content(http_client, url.to_string()).await?;
            Ok(Mention::Text {
                content,
                tracked_buffers: Vec::new(),
            })
        })
    }

    fn confirm_mention_for_symbol(
        &self,
        abs_path: PathBuf,
        line_range: RangeInclusive<u32>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("project not found")));
        };
        let Some(project_path) = project
            .read(cx)
            .project_path_for_absolute_path(&abs_path, cx)
        else {
            return Task::ready(Err(anyhow!("project path not found")));
        };
        let buffer = project.update(cx, |project, cx| project.open_buffer(project_path, cx));
        cx.spawn(async move |_, cx| {
            let buffer = buffer.await?;
            let mention = buffer.update(cx, |buffer, cx| {
                let start = Point::new(*line_range.start(), 0).min(buffer.max_point());
                let end = Point::new(*line_range.end() + 1, 0).min(buffer.max_point());
                let content = buffer.text_for_range(start..end).collect();
                Mention::Text {
                    content,
                    tracked_buffers: vec![cx.entity()],
                }
            });
            Ok(mention)
        })
    }

    fn confirm_mention_for_rule(
        &mut self,
        id: PromptId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(prompt_store) = self.prompt_store.as_ref() else {
            return Task::ready(Err(anyhow!("Missing prompt store")));
        };
        let prompt = prompt_store.read(cx).load(id, cx);
        cx.spawn(async move |_, _| {
            let prompt = prompt.await?;
            Ok(Mention::Text {
                content: prompt,
                tracked_buffers: Vec::new(),
            })
        })
    }

    pub fn confirm_mention_for_selection(
        &mut self,
        source_range: Range<text::Anchor>,
        selections: Vec<(Entity<Buffer>, Range<text::Anchor>, Range<usize>)>,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(project) = self.project.upgrade() else {
            return;
        };

        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        let Some(start) = snapshot.as_singleton_anchor(source_range.start) else {
            return;
        };

        let offset = start.to_offset(&snapshot);

        for (buffer, selection_range, range_to_fold) in selections {
            let range = snapshot.anchor_after(offset + range_to_fold.start)
                ..snapshot.anchor_after(offset + range_to_fold.end);

            let abs_path = buffer
                .read(cx)
                .project_path(cx)
                .and_then(|project_path| project.read(cx).absolute_path(&project_path, cx));
            let snapshot = buffer.read(cx).snapshot();

            let text = snapshot
                .text_for_range(selection_range.clone())
                .collect::<String>();
            let point_range = selection_range.to_point(&snapshot);
            let line_range = point_range.start.row..=point_range.end.row;

            let uri = MentionUri::Selection {
                abs_path: abs_path.clone(),
                line_range: line_range.clone(),
            };
            let crease = crease_for_mention(
                selection_name(abs_path.as_deref(), &line_range).into(),
                uri.icon_path(cx),
                range,
                editor.downgrade(),
            );

            let crease_id = editor.update(cx, |editor, cx| {
                let crease_ids = editor.insert_creases(vec![crease.clone()], cx);
                editor.fold_creases(vec![crease], false, window, cx);
                crease_ids.first().copied().unwrap()
            });

            self.mentions.insert(
                crease_id,
                (
                    uri,
                    Task::ready(Ok(Mention::Text {
                        content: text,
                        tracked_buffers: vec![buffer],
                    }))
                    .shared(),
                ),
            );
        }

        // Take this explanation with a grain of salt but, with creases being
        // inserted, GPUI's recomputes the editor layout in the next frames, so
        // directly calling `editor.request_autoscroll` wouldn't work as
        // expected. We're leveraging `cx.on_next_frame` to wait 2 frames and
        // ensure that the layout has been recalculated so that the autoscroll
        // request actually shows the cursor's new position.
        cx.on_next_frame(window, move |_, window, cx| {
            cx.on_next_frame(window, move |_, _, cx| {
                editor.update(cx, |editor, cx| {
                    editor.request_autoscroll(Autoscroll::fit(), cx)
                });
            });
        });
    }

    fn confirm_mention_for_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(thread_store) = self.thread_store.clone() else {
            return Task::ready(Err(anyhow!(
                "Thread mentions are only supported for the native agent"
            )));
        };
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("project not found")));
        };

        let server = Rc::new(agent::NativeAgentServer::new(
            project.read(cx).fs().clone(),
            thread_store,
        ));
        let delegate = AgentServerDelegate::new(
            project.read(cx).agent_server_store().clone(),
            project.clone(),
            None,
            None,
        );
        let connection = server.connect(None, delegate, cx);
        cx.spawn(async move |_, cx| {
            let (agent, _) = connection.await?;
            let agent = agent.downcast::<agent::NativeAgentConnection>().unwrap();
            let summary = agent
                .0
                .update(cx, |agent, cx| agent.thread_summary(id, cx))
                .await?;
            Ok(Mention::Text {
                content: summary.to_string(),
                tracked_buffers: Vec::new(),
            })
        })
    }

    fn confirm_mention_for_diagnostics(
        &self,
        include_errors: bool,
        include_warnings: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("project not found")));
        };

        let diagnostics_task = collect_diagnostics_output(
            project,
            assistant_slash_commands::Options {
                include_errors,
                include_warnings,
                path_matcher: None,
            },
            cx,
        );
        cx.spawn(async move |_, _| {
            let output = diagnostics_task.await?;
            let content = output
                .map(|output| output.text)
                .unwrap_or_else(|| "No diagnostics found.".into());
            Ok(Mention::Text {
                content,
                tracked_buffers: Vec::new(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use prompt_store;
    use release_channel;
    use semver::Version;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use theme;
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.update(|cx| {
            theme::init(theme::LoadThemes::JustBase, cx);
            release_channel::init(Version::new(0, 0, 0), cx);
            prompt_store::init(cx);
        });
    }

    #[gpui::test]
    async fn test_thread_mentions_disabled(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let thread_store = None;
        let mention_set = cx.new(|_cx| MentionSet::new(project.downgrade(), thread_store, None));

        let task = mention_set.update(cx, |mention_set, cx| {
            mention_set.confirm_mention_for_thread(acp::SessionId::new("thread-1"), cx)
        });

        let error = task.await.unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Thread mentions are only supported for the native agent"),
            "Unexpected error: {error:#}"
        );
    }
}

/// Inserts a list of images into the editor as context mentions.
/// This is the shared implementation used by both paste and file picker operations.
pub(crate) async fn insert_images_as_context(
    images: Vec<gpui::Image>,
    editor: Entity<Editor>,
    mention_set: Entity<MentionSet>,
    cx: &mut gpui::AsyncWindowContext,
) {
    if images.is_empty() {
        return;
    }

    let replacement_text = MentionUri::PastedImage.as_link().to_string();

    for image in images {
        let Some((excerpt_id, text_anchor, multibuffer_anchor)) = editor
            .update_in(cx, |editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                let (excerpt_id, _, buffer_snapshot) =
                    snapshot.buffer_snapshot().as_singleton().unwrap();

                let text_anchor = buffer_snapshot.anchor_before(buffer_snapshot.len());
                let multibuffer_anchor = snapshot
                    .buffer_snapshot()
                    .anchor_in_excerpt(*excerpt_id, text_anchor);
                editor.edit(
                    [(
                        multi_buffer::Anchor::max()..multi_buffer::Anchor::max(),
                        format!("{replacement_text} "),
                    )],
                    cx,
                );
                (*excerpt_id, text_anchor, multibuffer_anchor)
            })
            .ok()
        else {
            break;
        };

        let content_len = replacement_text.len();
        let Some(start_anchor) = multibuffer_anchor else {
            continue;
        };
        let end_anchor = editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            snapshot.anchor_before(start_anchor.to_offset(&snapshot) + content_len)
        });
        let image = Arc::new(image);
        let Ok(Some((crease_id, tx))) = cx.update(|window, cx| {
            insert_crease_for_mention(
                excerpt_id,
                text_anchor,
                content_len,
                MentionUri::PastedImage.name().into(),
                IconName::Image.path().into(),
                Some(Task::ready(Ok(image.clone())).shared()),
                editor.clone(),
                window,
                cx,
            )
        }) else {
            continue;
        };
        let task = cx
            .spawn(async move |cx| {
                let image = cx
                    .update(|_, cx| LanguageModelImage::from_image(image, cx))
                    .map_err(|e| e.to_string())?
                    .await;
                drop(tx);
                if let Some(image) = image {
                    Ok(Mention::Image(MentionImage {
                        data: image.source,
                        format: LanguageModelImage::FORMAT,
                    }))
                } else {
                    Err("Failed to convert image".into())
                }
            })
            .shared();

        mention_set.update(cx, |mention_set, _cx| {
            mention_set.insert_mention(crease_id, MentionUri::PastedImage, task.clone())
        });

        if task.await.notify_async_err(cx).is_none() {
            editor.update(cx, |editor, cx| {
                editor.edit([(start_anchor..end_anchor, "")], cx);
            });
            mention_set.update(cx, |mention_set, _cx| {
                mention_set.remove_mention(&crease_id)
            });
        }
    }
}

pub(crate) fn paste_images_as_context(
    editor: Entity<Editor>,
    mention_set: Entity<MentionSet>,
    window: &mut Window,
    cx: &mut App,
) -> Option<Task<()>> {
    let clipboard = cx.read_from_clipboard()?;
    Some(window.spawn(cx, async move |cx| {
        use itertools::Itertools;
        let (mut images, paths) = clipboard
            .into_entries()
            .filter_map(|entry| match entry {
                ClipboardEntry::Image(image) => Some(Either::Left(image)),
                ClipboardEntry::ExternalPaths(paths) => Some(Either::Right(paths)),
                _ => None,
            })
            .partition_map::<Vec<_>, Vec<_>, _, _, _>(std::convert::identity);

        if !paths.is_empty() {
            images.extend(
                cx.background_spawn(async move {
                    let mut images = vec![];
                    for path in paths.into_iter().flat_map(|paths| paths.paths().to_owned()) {
                        let Ok(content) = async_fs::read(path).await else {
                            continue;
                        };
                        let Ok(format) = image::guess_format(&content) else {
                            continue;
                        };
                        images.push(gpui::Image::from_bytes(
                            match format {
                                image::ImageFormat::Png => gpui::ImageFormat::Png,
                                image::ImageFormat::Jpeg => gpui::ImageFormat::Jpeg,
                                image::ImageFormat::WebP => gpui::ImageFormat::Webp,
                                image::ImageFormat::Gif => gpui::ImageFormat::Gif,
                                image::ImageFormat::Bmp => gpui::ImageFormat::Bmp,
                                image::ImageFormat::Tiff => gpui::ImageFormat::Tiff,
                                image::ImageFormat::Ico => gpui::ImageFormat::Ico,
                                _ => continue,
                            },
                            content,
                        ));
                    }
                    images
                })
                .await,
            );
        }

        cx.update(|_window, cx| {
            cx.stop_propagation();
        })
        .ok();

        insert_images_as_context(images, editor, mention_set, cx).await;
    }))
}

pub(crate) fn insert_crease_for_mention(
    excerpt_id: ExcerptId,
    anchor: text::Anchor,
    content_len: usize,
    crease_label: SharedString,
    crease_icon: SharedString,
    // abs_path: Option<Arc<Path>>,
    image: Option<Shared<Task<Result<Arc<Image>, String>>>>,
    editor: Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Option<(CreaseId, postage::barrier::Sender)> {
    let (tx, rx) = postage::barrier::channel();

    let crease_id = editor.update(cx, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);

        let start = snapshot.anchor_in_excerpt(excerpt_id, anchor)?;

        let start = start.bias_right(&snapshot);
        let end = snapshot.anchor_before(start.to_offset(&snapshot) + content_len);

        let placeholder = FoldPlaceholder {
            render: render_mention_fold_button(
                crease_label.clone(),
                crease_icon.clone(),
                start..end,
                rx,
                image,
                cx.weak_entity(),
                cx,
            ),
            merge_adjacent: false,
            ..Default::default()
        };

        let crease = Crease::Inline {
            range: start..end,
            placeholder,
            render_toggle: None,
            render_trailer: None,
            metadata: Some(CreaseMetadata {
                label: crease_label,
                icon_path: crease_icon,
            }),
        };

        let ids = editor.insert_creases(vec![crease.clone()], cx);
        editor.fold_creases(vec![crease], false, window, cx);

        Some(ids[0])
    })?;

    Some((crease_id, tx))
}

pub(crate) fn crease_for_mention(
    label: SharedString,
    icon_path: SharedString,
    range: Range<Anchor>,
    editor_entity: WeakEntity<Editor>,
) -> Crease<Anchor> {
    let placeholder = FoldPlaceholder {
        render: render_fold_icon_button(icon_path.clone(), label.clone(), editor_entity),
        merge_adjacent: false,
        ..Default::default()
    };

    let render_trailer = move |_row, _unfold, _window: &mut Window, _cx: &mut App| Empty.into_any();

    Crease::inline(range, placeholder, fold_toggle("mention"), render_trailer)
        .with_metadata(CreaseMetadata { icon_path, label })
}

fn render_fold_icon_button(
    icon_path: SharedString,
    label: SharedString,
    editor: WeakEntity<Editor>,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    Arc::new({
        move |fold_id, fold_range, cx| {
            let is_in_text_selection = editor
                .update(cx, |editor, cx| editor.is_range_selected(&fold_range, cx))
                .unwrap_or_default();

            MentionCrease::new(fold_id, icon_path.clone(), label.clone())
                .is_toggled(is_in_text_selection)
                .into_any_element()
        }
    })
}

fn fold_toggle(
    name: &'static str,
) -> impl Fn(
    MultiBufferRow,
    bool,
    Arc<dyn Fn(bool, &mut Window, &mut App) + Send + Sync>,
    &mut Window,
    &mut App,
) -> AnyElement {
    move |row, is_folded, fold, _window, _cx| {
        Disclosure::new((name, row.0 as u64), !is_folded)
            .toggle_state(is_folded)
            .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
            .into_any_element()
    }
}

fn full_mention_for_directory(
    project: &Entity<Project>,
    abs_path: &Path,
    cx: &mut App,
) -> Task<Result<Mention>> {
    fn collect_files_in_path(worktree: &Worktree, path: &RelPath) -> Vec<(Arc<RelPath>, String)> {
        let mut files = Vec::new();

        for entry in worktree.child_entries(path) {
            if entry.is_dir() {
                files.extend(collect_files_in_path(worktree, &entry.path));
            } else if entry.is_file() {
                files.push((
                    entry.path.clone(),
                    worktree
                        .full_path(&entry.path)
                        .to_string_lossy()
                        .to_string(),
                ));
            }
        }

        files
    }

    let Some(project_path) = project
        .read(cx)
        .project_path_for_absolute_path(&abs_path, cx)
    else {
        return Task::ready(Err(anyhow!("project path not found")));
    };
    let Some(entry) = project.read(cx).entry_for_path(&project_path, cx) else {
        return Task::ready(Err(anyhow!("project entry not found")));
    };
    let directory_path = entry.path.clone();
    let worktree_id = project_path.worktree_id;
    let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) else {
        return Task::ready(Err(anyhow!("worktree not found")));
    };
    let project = project.clone();
    cx.spawn(async move |cx| {
        let file_paths = worktree.read_with(cx, |worktree, _cx| {
            collect_files_in_path(worktree, &directory_path)
        });
        let descendants_future = cx.update(|cx| {
            futures::future::join_all(file_paths.into_iter().map(
                |(worktree_path, full_path): (Arc<RelPath>, String)| {
                    let rel_path = worktree_path
                        .strip_prefix(&directory_path)
                        .log_err()
                        .map_or_else(|| worktree_path.clone(), |rel_path| rel_path.into());

                    let open_task = project.update(cx, |project, cx| {
                        project.buffer_store().update(cx, |buffer_store, cx| {
                            let project_path = ProjectPath {
                                worktree_id,
                                path: worktree_path,
                            };
                            buffer_store.open_buffer(project_path, cx)
                        })
                    });

                    cx.spawn(async move |cx| {
                        let buffer = open_task.await.log_err()?;
                        let buffer_content = outline::get_buffer_content_or_outline(
                            buffer.clone(),
                            Some(&full_path),
                            &cx,
                        )
                        .await
                        .ok()?;

                        Some((rel_path, full_path, buffer_content.text, buffer))
                    })
                },
            ))
        });

        let contents = cx
            .background_spawn(async move {
                let (contents, tracked_buffers): (Vec<_>, Vec<_>) = descendants_future
                    .await
                    .into_iter()
                    .flatten()
                    .map(|(rel_path, full_path, rope, buffer)| {
                        ((rel_path, full_path, rope), buffer)
                    })
                    .unzip();
                Mention::Text {
                    content: render_directory_contents(contents),
                    tracked_buffers,
                }
            })
            .await;
        anyhow::Ok(contents)
    })
}

fn render_directory_contents(entries: Vec<(Arc<RelPath>, String, String)>) -> String {
    let mut output = String::new();
    for (_relative_path, full_path, content) in entries {
        let fence = codeblock_fence_for_path(Some(&full_path), None);
        write!(output, "\n{fence}\n{content}\n```").unwrap();
    }
    output
}

fn render_mention_fold_button(
    label: SharedString,
    icon: SharedString,
    range: Range<Anchor>,
    mut loading_finished: postage::barrier::Receiver,
    image_task: Option<Shared<Task<Result<Arc<Image>, String>>>>,
    editor: WeakEntity<Editor>,
    cx: &mut App,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    let loading = cx.new(|cx| {
        let loading = cx.spawn(async move |this, cx| {
            loading_finished.recv().await;
            this.update(cx, |this: &mut LoadingContext, cx| {
                this.loading = None;
                cx.notify();
            })
            .ok();
        });
        LoadingContext {
            id: cx.entity_id(),
            label,
            icon,
            range,
            editor,
            loading: Some(loading),
            image: image_task.clone(),
        }
    });
    Arc::new(move |_fold_id, _fold_range, _cx| loading.clone().into_any_element())
}

struct LoadingContext {
    id: EntityId,
    label: SharedString,
    icon: SharedString,
    range: Range<Anchor>,
    editor: WeakEntity<Editor>,
    loading: Option<Task<()>>,
    image: Option<Shared<Task<Result<Arc<Image>, String>>>>,
}

impl Render for LoadingContext {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_in_text_selection = self
            .editor
            .update(cx, |editor, cx| editor.is_range_selected(&self.range, cx))
            .unwrap_or_default();

        let id = ElementId::from(("loading_context", self.id));

        MentionCrease::new(id, self.icon.clone(), self.label.clone())
            .is_toggled(is_in_text_selection)
            .is_loading(self.loading.is_some())
            .when_some(self.image.clone(), |this, image_task| {
                this.image_preview(move |_, cx| {
                    let image = image_task.peek().cloned().transpose().ok().flatten();
                    let image_task = image_task.clone();
                    cx.new::<ImageHover>(|cx| ImageHover {
                        image,
                        _task: cx.spawn(async move |this, cx| {
                            if let Ok(image) = image_task.clone().await {
                                this.update(cx, |this, cx| {
                                    if this.image.replace(image).is_none() {
                                        cx.notify();
                                    }
                                })
                                .ok();
                            }
                        }),
                    })
                    .into()
                })
            })
    }
}

struct ImageHover {
    image: Option<Arc<Image>>,
    _task: Task<()>,
}

impl Render for ImageHover {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(image) = self.image.clone() {
            div()
                .p_1p5()
                .elevation_2(cx)
                .child(gpui::img(image).h_auto().max_w_96().rounded_sm())
                .into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}

async fn fetch_url_content(http_client: Arc<HttpClientWithUrl>, url: String) -> Result<String> {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum ContentType {
        Html,
        Plaintext,
        Json,
    }
    use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown};

    let url = if !url.starts_with("https://") && !url.starts_with("http://") {
        format!("https://{url}")
    } else {
        url
    };

    let mut response = http_client.get(&url, AsyncBody::default(), true).await?;
    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("error reading response body")?;

    if response.status().is_client_error() {
        let text = String::from_utf8_lossy(body.as_slice());
        anyhow::bail!(
            "status error {}, response: {text:?}",
            response.status().as_u16()
        );
    }

    let Some(content_type) = response.headers().get("content-type") else {
        anyhow::bail!("missing Content-Type header");
    };
    let content_type = content_type
        .to_str()
        .context("invalid Content-Type header")?;
    let content_type = match content_type {
        "text/html" => ContentType::Html,
        "text/plain" => ContentType::Plaintext,
        "application/json" => ContentType::Json,
        _ => ContentType::Html,
    };

    match content_type {
        ContentType::Html => {
            let mut handlers: Vec<TagHandler> = vec![
                Rc::new(RefCell::new(markdown::WebpageChromeRemover)),
                Rc::new(RefCell::new(markdown::ParagraphHandler)),
                Rc::new(RefCell::new(markdown::HeadingHandler)),
                Rc::new(RefCell::new(markdown::ListHandler)),
                Rc::new(RefCell::new(markdown::TableHandler::new())),
                Rc::new(RefCell::new(markdown::StyledTextHandler)),
            ];
            if url.contains("wikipedia.org") {
                use html_to_markdown::structure::wikipedia;

                handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaChromeRemover)));
                handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaInfoboxHandler)));
                handlers.push(Rc::new(
                    RefCell::new(wikipedia::WikipediaCodeHandler::new()),
                ));
            } else {
                handlers.push(Rc::new(RefCell::new(markdown::CodeHandler)));
            }
            convert_html_to_markdown(&body[..], &mut handlers)
        }
        ContentType::Plaintext => Ok(std::str::from_utf8(&body)?.to_owned()),
        ContentType::Json => {
            let json: serde_json::Value = serde_json::from_slice(&body)?;

            Ok(format!(
                "```json\n{}\n```",
                serde_json::to_string_pretty(&json)?
            ))
        }
    }
}

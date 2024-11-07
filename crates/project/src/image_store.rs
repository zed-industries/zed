use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use crate::{Project, ProjectEntryId, ProjectPath, ProjectTransaction};
use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use fs::Fs;
use gpui::{hash, prelude::*, AppContext, Img, Model, ModelContext, Subscription, Task, WeakModel};
use rpc::AnyProtoClient;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use worktree::{PathChange, Worktree};

struct ImageId();

pub struct ImageItem {
    pub entry_id: ProjectEntryId,
    pub project_path: ProjectPath,
    pub abs_path: PathBuf,
    pub image: Arc<gpui::Image>,
}

impl crate::Item for ImageItem {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let path = path.clone();
        let project = project.clone();

        let ext = path
            .path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase)
            .unwrap_or_default();
        let ext = ext.as_str();

        // Only open the item if it's a binary image (no SVGs, etc.)
        // Since we do not have a way to toggle to an editor
        if Img::extensions().contains(&ext) && !ext.contains("svg") {
            Some(cx.spawn(|mut cx| async move {
                project
                    .update(&mut cx, |project, cx| project.open_image(path, cx))?
                    .await
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        Some(self.entry_id)
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }
}

trait ImageStoreImpl {
    fn open_image(
        &self,
        path: Arc<Path>,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<Model<ImageItem>>>;

    fn reload_images(
        &self,
        images: HashSet<Model<ImageItem>>,
        push_to_history: bool,
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<ProjectTransaction>>;

    fn as_remote(&self) -> Option<Model<RemoteImageStore>>;
    fn as_local(&self) -> Option<Model<LocalImageStore>>;
}

struct RemoteImageStore {}

struct LocalImageStore {
    fs: Arc<dyn Fs>,
    local_image_ids_by_path: HashMap<ProjectPath, ImageId>,
    local_image_ids_by_entry_id: HashMap<ProjectEntryId, ImageId>,
    image_store: WeakModel<ImageStore>,
    worktree_store: Model<WorktreeStore>,
    _subscription: Subscription,
}

pub struct ImageStore {
    state: Box<dyn ImageStoreImpl>,
    opened_images: HashMap<ImageId, WeakModel<ImageItem>>,
    worktree_store: Model<WorktreeStore>,
}

impl ImageStore {
    pub fn local(
        worktree_store: Model<WorktreeStore>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let this = cx.weak_model();
        Self {
            state: Box::new(cx.new_model(|cx| {
                let subscription = cx.subscribe(
                    &worktree_store,
                    |this: &mut LocalImageStore, _, event, cx| {
                        if let WorktreeStoreEvent::WorktreeAdded(worktree) = event {
                            this.subscribe_to_worktree(worktree, cx);
                        }
                    },
                );

                LocalImageStore {
                    fs,
                    local_image_ids_by_path: Default::default(),
                    local_image_ids_by_entry_id: Default::default(),
                    image_store: this,
                    worktree_store: worktree_store.clone(),
                    _subscription: subscription,
                }
            })),
            opened_images: Default::default(),
            worktree_store,
        }
    }

    pub fn remote(
        worktree_store: Model<WorktreeStore>,
        upstream_client: AnyProtoClient,
        remote_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        todo!()
    }

    pub fn images(&self) -> impl '_ + Iterator<Item = Model<ImageItem>> {
        self.opened_images
            .values()
            .filter_map(|image| image.upgrade())
    }

    pub fn get_by_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<Model<ImageItem>> {
        self.images()
            .find(|image| &image.read(cx).project_path == path)
    }

    pub fn open_image(
        &mut self,
        project_path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<ImageItem>>> {
        let existing_image = self.get_by_path(&project_path, cx);
        if let Some(existing_image) = existing_image {
            return Task::ready(Ok(existing_image));
        }

        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow::anyhow!("no such worktree")));
        };

        self.state
            .open_image(project_path.path.clone(), worktree, cx)
    }
}

impl ImageStoreImpl for Model<LocalImageStore> {
    fn open_image(
        &self,
        path: Arc<Path>,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<Model<ImageItem>>> {
        let fs = self.read(cx).fs.clone();
        let worktree = worktree.read(cx);

        let Ok(abs_path) = worktree.absolutize(&path) else {
            return Task::ready(Err(anyhow::anyhow!(
                "failed to find absolute path for image {path:?}"
            )));
        };
        let Some(entry) = worktree.entry_for_path(&path) else {
            return Task::ready(Err(anyhow::anyhow!(
                "failed to find entry for path {path:?}"
            )));
        };
        let entry_id = entry.id;
        let worktree_id = worktree.id();

        cx.spawn(move |_, mut cx| async move {
            let data = cx
                .background_executor()
                .spawn({
                    let abs_path = abs_path.clone();
                    async move { load_image(fs, &abs_path).await }
                })
                .await?;
            cx.new_model(|_| ImageItem {
                entry_id,
                abs_path,
                project_path: ProjectPath { worktree_id, path },
                image: data,
            })
        })
    }

    fn reload_images(
        &self,
        images: HashSet<Model<ImageItem>>,
        push_to_history: bool,
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<ProjectTransaction>> {
        Task::ready(Err(anyhow::anyhow!("not implemented")))
    }

    fn as_remote(&self) -> Option<Model<RemoteImageStore>> {
        None
    }

    fn as_local(&self) -> Option<Model<LocalImageStore>> {
        Some(self.clone())
    }
}

impl LocalImageStore {
    fn subscribe_to_worktree(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        cx.subscribe(worktree, |this, worktree, event, cx| {
            if worktree.read(cx).is_local() {
                match event {
                    worktree::Event::UpdatedEntries(changes) => {
                        this.local_worktree_entries_changed(&worktree, changes, cx);
                    }
                    _ => {}
                }
            }
        })
        .detach();
    }

    fn local_worktree_entries_changed(
        &mut self,
        worktree_handle: &Model<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();
        for (path, entry_id, _) in changes {
            self.local_worktree_entry_changed(*entry_id, path, worktree_handle, &snapshot, cx);
        }
    }

    fn local_worktree_entry_changed(
        &mut self,
        entry_id: ProjectEntryId,
        path: &Arc<Path>,
        worktree: &Model<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        // TODO: reload the image
        None
    }
}

async fn load_image(fs: Arc<dyn Fs>, abs_path: &Path) -> anyhow::Result<Arc<gpui::Image>> {
    let new_contents = fs
        .load_bytes(&abs_path)
        .await
        .with_context(|| format!("failed to load bytes for path {abs_path:?}"))?;

    let format = image::guess_format(&new_contents)?;

    Ok(Arc::new(gpui::Image {
        id: hash(&new_contents),
        format: match format {
            image::ImageFormat::Png => gpui::ImageFormat::Png,
            image::ImageFormat::Jpeg => gpui::ImageFormat::Jpeg,
            image::ImageFormat::Gif => gpui::ImageFormat::Gif,
            image::ImageFormat::WebP => gpui::ImageFormat::Webp,
            image::ImageFormat::Tiff => gpui::ImageFormat::Tiff,
            image::ImageFormat::Bmp => gpui::ImageFormat::Bmp,
            _ => {
                log::error!("Image format not supported");
                Err(anyhow::anyhow!("Image format not supported"))?
            }
        },
        bytes: new_contents,
    }))
}

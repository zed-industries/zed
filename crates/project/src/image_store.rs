use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use crate::{Project, ProjectEntryId, ProjectPath};
use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use futures::channel::oneshot;
use gpui::{
    hash, prelude::*, AppContext, EventEmitter, Img, Model, ModelContext, Subscription, Task,
    WeakModel,
};
use language::File;
use rpc::AnyProtoClient;
use std::ffi::OsStr;
use std::num::NonZeroU64;
use std::path::Path;
use std::sync::Arc;
use util::ResultExt;
use worktree::{LoadedBinaryFile, PathChange, Worktree};

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct ImageId(NonZeroU64);

impl std::fmt::Display for ImageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<NonZeroU64> for ImageId {
    fn from(id: NonZeroU64) -> Self {
        ImageId(id)
    }
}

pub enum ImageItemEvent {
    ReloadNeeded,
    Reloaded,
    FileHandleChanged,
}

impl EventEmitter<ImageItemEvent> for ImageItem {}

pub enum ImageStoreEvent {
    ImageAdded(Model<ImageItem>),
}

impl EventEmitter<ImageStoreEvent> for ImageStore {}

pub struct ImageItem {
    pub id: ImageId,
    pub file: Arc<dyn File>,
    pub image: Arc<gpui::Image>,
    reload_task: Option<Task<()>>,
}

impl ImageItem {
    pub fn project_path(&self, cx: &AppContext) -> ProjectPath {
        ProjectPath {
            worktree_id: self.file.worktree_id(cx),
            path: self.file.path().clone(),
        }
    }

    pub fn path(&self) -> &Arc<Path> {
        self.file.path()
    }

    fn file_updated(&mut self, new_file: Arc<dyn File>, cx: &mut ModelContext<Self>) {
        let mut file_changed = false;

        let old_file = self.file.as_ref();
        if new_file.path() != old_file.path() {
            file_changed = true;
        }

        if !new_file.is_deleted() {
            let new_mtime = new_file.mtime();
            if new_mtime != old_file.mtime() {
                file_changed = true;
                cx.emit(ImageItemEvent::ReloadNeeded);
            }
        }

        self.file = new_file;
        if file_changed {
            cx.emit(ImageItemEvent::FileHandleChanged);
            cx.notify();
        }
    }

    fn reload(&mut self, cx: &mut ModelContext<Self>) -> Option<oneshot::Receiver<()>> {
        let local_file = self.file.as_local()?;
        let (tx, rx) = futures::channel::oneshot::channel();

        let content = local_file.load_bytes(cx);
        self.reload_task = Some(cx.spawn(|this, mut cx| async move {
            if let Some(image) = content
                .await
                .context("Failed to load image content")
                .and_then(create_gpui_image)
                .log_err()
            {
                this.update(&mut cx, |this, cx| {
                    this.image = image;
                    cx.emit(ImageItemEvent::Reloaded);
                })
                .log_err();
            }
            _ = tx.send(());
        }));
        Some(rx)
    }
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
        worktree::File::from_dyn(Some(&self.file))?.entry_id
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        Some(self.project_path(cx).clone())
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
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<()>>;

    fn as_local(&self) -> Option<Model<LocalImageStore>>;
}

struct RemoteImageStore {}

struct LocalImageStore {
    local_image_ids_by_path: HashMap<ProjectPath, ImageId>,
    local_image_ids_by_entry_id: HashMap<ProjectEntryId, ImageId>,
    image_store: WeakModel<ImageStore>,
    _subscription: Subscription,
}

pub struct ImageStore {
    state: Box<dyn ImageStoreImpl>,
    opened_images: HashMap<ImageId, WeakModel<ImageItem>>,
    worktree_store: Model<WorktreeStore>,
}

impl ImageStore {
    pub fn local(worktree_store: Model<WorktreeStore>, cx: &mut ModelContext<Self>) -> Self {
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
                    local_image_ids_by_path: Default::default(),
                    local_image_ids_by_entry_id: Default::default(),
                    image_store: this,
                    _subscription: subscription,
                }
            })),
            opened_images: Default::default(),
            worktree_store,
        }
    }

    pub fn remote(
        worktree_store: Model<WorktreeStore>,
        _upstream_client: AnyProtoClient,
        _remote_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            state: Box::new(cx.new_model(|_| RemoteImageStore {})),
            opened_images: Default::default(),
            worktree_store,
        }
    }

    pub fn images(&self) -> impl '_ + Iterator<Item = Model<ImageItem>> {
        self.opened_images
            .values()
            .filter_map(|image| image.upgrade())
    }

    pub fn get(&self, image_id: ImageId) -> Option<Model<ImageItem>> {
        self.opened_images
            .get(&image_id)
            .and_then(|image| image.upgrade())
    }

    pub fn get_by_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<Model<ImageItem>> {
        self.images()
            .find(|image| &image.read(cx).project_path(cx) == path)
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

    pub fn reload_images(
        &self,
        images: HashSet<Model<ImageItem>>,
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<()>> {
        if images.is_empty() {
            return Task::ready(Ok(()));
        }

        self.state.reload_images(images, cx)
    }

    fn add_image(
        &mut self,
        image: Model<ImageItem>,
        cx: &mut ModelContext<ImageStore>,
    ) -> Result<()> {
        let image_id = image.read(cx).id;

        self.opened_images.insert(image_id, image.downgrade());

        cx.subscribe(&image, Self::on_image_event).detach();
        cx.emit(ImageStoreEvent::ImageAdded(image));
        Ok(())
    }

    fn on_image_event(
        &mut self,
        image: Model<ImageItem>,
        event: &ImageItemEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            ImageItemEvent::FileHandleChanged => {
                if let Some(local) = self.state.as_local() {
                    local.update(cx, |local, cx| {
                        local.image_changed_file(image, cx);
                    })
                }
            }
            _ => {}
        }
    }
}

impl ImageStoreImpl for Model<LocalImageStore> {
    fn open_image(
        &self,
        path: Arc<Path>,
        worktree: Model<Worktree>,
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<Model<ImageItem>>> {
        let this = self.clone();

        let load_file = worktree.update(cx, |worktree, cx| {
            worktree.load_binary_file(path.as_ref(), cx)
        });
        cx.spawn(move |image_store, mut cx| async move {
            let LoadedBinaryFile { file, content } = load_file.await?;
            let image = create_gpui_image(content)?;

            let model = cx.new_model(|cx| ImageItem {
                id: cx.entity_id().as_non_zero_u64().into(),
                file: file.clone(),
                image,
                reload_task: None,
            })?;

            let image_id = cx.read_model(&model, |model, _| model.id)?;

            this.update(&mut cx, |this, cx| {
                image_store.update(cx, |image_store, cx| {
                    image_store.add_image(model.clone(), cx)
                })??;
                this.local_image_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    },
                    image_id,
                );

                if let Some(entry_id) = file.entry_id {
                    this.local_image_ids_by_entry_id.insert(entry_id, image_id);
                }

                anyhow::Ok(())
            })??;

            Ok(model)
        })
    }

    fn reload_images(
        &self,
        images: HashSet<Model<ImageItem>>,
        cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<()>> {
        cx.spawn(move |_, mut cx| async move {
            for image in images {
                if let Some(rec) = image.update(&mut cx, |image, cx| image.reload(cx))? {
                    rec.await?
                }
            }
            Ok(())
        })
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
        let project_path = ProjectPath {
            worktree_id: snapshot.id(),
            path: path.clone(),
        };
        let image_id = match self.local_image_ids_by_entry_id.get(&entry_id) {
            Some(&image_id) => image_id,
            None => self.local_image_ids_by_path.get(&project_path).copied()?,
        };

        let image = self
            .image_store
            .update(cx, |image_store, _| {
                if let Some(image) = image_store.get(image_id) {
                    Some(image)
                } else {
                    image_store.opened_images.remove(&image_id);
                    None
                }
            })
            .ok()
            .flatten();
        let image = if let Some(image) = image {
            image
        } else {
            self.local_image_ids_by_path.remove(&project_path);
            self.local_image_ids_by_entry_id.remove(&entry_id);
            return None;
        };

        image.update(cx, |image, cx| {
            let Some(old_file) = worktree::File::from_dyn(Some(&image.file)) else {
                return;
            };
            if old_file.worktree != *worktree {
                return;
            }

            let new_file = if let Some(entry) = old_file
                .entry_id
                .and_then(|entry_id| snapshot.entry_for_id(entry_id))
            {
                worktree::File {
                    is_local: true,
                    entry_id: Some(entry.id),
                    mtime: entry.mtime,
                    path: entry.path.clone(),
                    worktree: worktree.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else if let Some(entry) = snapshot.entry_for_path(old_file.path.as_ref()) {
                worktree::File {
                    is_local: true,
                    entry_id: Some(entry.id),
                    mtime: entry.mtime,
                    path: entry.path.clone(),
                    worktree: worktree.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else {
                worktree::File {
                    is_local: true,
                    entry_id: old_file.entry_id,
                    path: old_file.path.clone(),
                    mtime: old_file.mtime,
                    worktree: worktree.clone(),
                    is_deleted: true,
                    is_private: old_file.is_private,
                }
            };

            if new_file == *old_file {
                return;
            }

            if new_file.path != old_file.path {
                self.local_image_ids_by_path.remove(&ProjectPath {
                    path: old_file.path.clone(),
                    worktree_id: old_file.worktree_id(cx),
                });
                self.local_image_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: new_file.worktree_id(cx),
                        path: new_file.path.clone(),
                    },
                    image_id,
                );
            }

            if new_file.entry_id != old_file.entry_id {
                if let Some(entry_id) = old_file.entry_id {
                    self.local_image_ids_by_entry_id.remove(&entry_id);
                }
                if let Some(entry_id) = new_file.entry_id {
                    self.local_image_ids_by_entry_id.insert(entry_id, image_id);
                }
            }

            image.file_updated(Arc::new(new_file), cx);
        });
        None
    }

    fn image_changed_file(&mut self, image: Model<ImageItem>, cx: &mut AppContext) -> Option<()> {
        let file = worktree::File::from_dyn(Some(&image.read(cx).file))?;

        let image_id = image.read(cx).id;
        if let Some(entry_id) = file.entry_id {
            match self.local_image_ids_by_entry_id.get(&entry_id) {
                Some(_) => {
                    return None;
                }
                None => {
                    self.local_image_ids_by_entry_id.insert(entry_id, image_id);
                }
            }
        };
        self.local_image_ids_by_path.insert(
            ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path.clone(),
            },
            image_id,
        );

        Some(())
    }
}

fn create_gpui_image(content: Vec<u8>) -> anyhow::Result<Arc<gpui::Image>> {
    let format = image::guess_format(&content)?;

    Ok(Arc::new(gpui::Image {
        id: hash(&content),
        format: match format {
            image::ImageFormat::Png => gpui::ImageFormat::Png,
            image::ImageFormat::Jpeg => gpui::ImageFormat::Jpeg,
            image::ImageFormat::WebP => gpui::ImageFormat::Webp,
            image::ImageFormat::Gif => gpui::ImageFormat::Gif,
            image::ImageFormat::Bmp => gpui::ImageFormat::Bmp,
            image::ImageFormat::Tiff => gpui::ImageFormat::Tiff,
            _ => Err(anyhow::anyhow!("Image format not supported"))?,
        },
        bytes: content,
    }))
}

impl ImageStoreImpl for Model<RemoteImageStore> {
    fn open_image(
        &self,
        _path: Arc<Path>,
        _worktree: Model<Worktree>,
        _cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<Model<ImageItem>>> {
        Task::ready(Err(anyhow::anyhow!(
            "Opening images from remote is not supported"
        )))
    }

    fn reload_images(
        &self,
        _images: HashSet<Model<ImageItem>>,
        _cx: &mut ModelContext<ImageStore>,
    ) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "Reloading images from remote is not supported"
        )))
    }

    fn as_local(&self) -> Option<Model<LocalImageStore>> {
        None
    }
}

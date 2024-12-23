use crate::{
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    Project, ProjectEntryId, ProjectItem, ProjectPath,
};
use anyhow::{Context as _, Result};
use collections::{hash_map, HashMap, HashSet};
use futures::{channel::oneshot, StreamExt};
use gpui::{
    hash, prelude::*, AppContext, EventEmitter, Img, Model, ModelContext, Subscription, Task,
    WeakModel,
};
use language::{DiskState, File};
use rpc::{AnyProtoClient, ErrorExt as _};
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

        let old_state = old_file.disk_state();
        let new_state = new_file.disk_state();
        if old_state != new_state {
            file_changed = true;
            if matches!(new_state, DiskState::Present { .. }) {
                cx.emit(ImageItemEvent::ReloadNeeded)
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

impl ProjectItem for ImageItem {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let path = path.clone();
        let project = project.clone();

        let worktree_abs_path = project
            .read(cx)
            .worktree_for_id(path.worktree_id, cx)?
            .read(cx)
            .abs_path();

        // Resolve the file extension from either the worktree path (if it's a single file)
        // or from the project path's subpath.
        let ext = worktree_abs_path
            .extension()
            .or_else(|| path.path.extension())
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

    fn is_dirty(&self) -> bool {
        false
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
    #[allow(clippy::type_complexity)]
    loading_images_by_path: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<Model<ImageItem>, Arc<anyhow::Error>>>>,
    >,
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
            loading_images_by_path: Default::default(),
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
            loading_images_by_path: Default::default(),
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

        let loading_watch = match self.loading_images_by_path.entry(project_path.clone()) {
            // If the given path is already being loaded, then wait for that existing
            // task to complete and return the same image.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being loaded.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let project_path = project_path.clone();
                let load_image = self
                    .state
                    .open_image(project_path.path.clone(), worktree, cx);

                cx.spawn(move |this, mut cx| async move {
                    let load_result = load_image.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _cx| {
                        // Record the fact that the image is no longer loading.
                        this.loading_images_by_path.remove(&project_path);
                        let image = load_result.map_err(Arc::new)?;
                        Ok(image)
                    })?);
                    anyhow::Ok(())
                })
                .detach();
                rx
            }
        };

        cx.background_executor().spawn(async move {
            Self::wait_for_loading_image(loading_watch)
                .await
                .map_err(|e| e.cloned())
        })
    }

    pub async fn wait_for_loading_image(
        mut receiver: postage::watch::Receiver<
            Option<Result<Model<ImageItem>, Arc<anyhow::Error>>>,
        >,
    ) -> Result<Model<ImageItem>, Arc<anyhow::Error>> {
        loop {
            if let Some(result) = receiver.borrow().as_ref() {
                match result {
                    Ok(image) => return Ok(image.to_owned()),
                    Err(e) => return Err(e.to_owned()),
                }
            }
            receiver.next().await;
        }
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

            let snapshot_entry = old_file
                .entry_id
                .and_then(|entry_id| snapshot.entry_for_id(entry_id))
                .or_else(|| snapshot.entry_for_path(old_file.path.as_ref()));

            let new_file = if let Some(entry) = snapshot_entry {
                worktree::File {
                    disk_state: match entry.mtime {
                        Some(mtime) => DiskState::Present { mtime },
                        None => old_file.disk_state,
                    },
                    is_local: true,
                    entry_id: Some(entry.id),
                    path: entry.path.clone(),
                    worktree: worktree.clone(),
                    is_private: entry.is_private,
                }
            } else {
                worktree::File {
                    disk_state: DiskState::Deleted,
                    is_local: true,
                    entry_id: old_file.entry_id,
                    path: old_file.path.clone(),
                    worktree: worktree.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::PathBuf;

    pub fn init_test(cx: &mut TestAppContext) {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::try_init().ok();
        }

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }

    #[gpui::test]
    async fn test_image_not_loaded_twice(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree("/root", json!({})).await;
        // Create a png file that consists of a single white pixel
        fs.insert_file(
            "/root/image_1.png",
            vec![
                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
                0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78,
                0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
                0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
            ],
        )
        .await;

        let project = Project::test(fs, ["/root".as_ref()], cx).await;

        let worktree_id =
            cx.update(|cx| project.read(cx).worktrees(cx).next().unwrap().read(cx).id());

        let project_path = ProjectPath {
            worktree_id,
            path: PathBuf::from("image_1.png").into(),
        };

        let (task1, task2) = project.update(cx, |project, cx| {
            (
                project.open_image(project_path.clone(), cx),
                project.open_image(project_path.clone(), cx),
            )
        });

        let image1 = task1.await.unwrap();
        let image2 = task2.await.unwrap();

        assert_eq!(image1, image2);
    }
}

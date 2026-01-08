use crate::{
    Project, ProjectEntryId, ProjectItem, ProjectPath,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};
use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet, hash_map};
use futures::{StreamExt, channel::oneshot};
use gpui::{
    App, AsyncApp, Context, Entity, EventEmitter, Img, Subscription, Task, WeakEntity, prelude::*,
};
pub use image::ImageFormat;
use image::{ExtendedColorType, GenericImageView, ImageReader};
use language::{DiskState, File};
use rpc::{AnyProtoClient, ErrorExt as _, TypedEnvelope, proto};
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::sync::Arc;
use util::{ResultExt, rel_path::RelPath};
use worktree::{LoadedBinaryFile, PathChange, Worktree, WorktreeId};

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct ImageId(NonZeroU64);

impl ImageId {
    pub fn to_proto(&self) -> u64 {
        self.0.get()
    }
}

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

#[derive(Debug)]
pub enum ImageItemEvent {
    ReloadNeeded,
    Reloaded,
    FileHandleChanged,
    MetadataUpdated,
}

impl EventEmitter<ImageItemEvent> for ImageItem {}

pub enum ImageStoreEvent {
    ImageAdded(Entity<ImageItem>),
}

impl EventEmitter<ImageStoreEvent> for ImageStore {}

#[derive(Debug, Clone, Copy)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub colors: Option<ImageColorInfo>,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Copy)]
pub struct ImageColorInfo {
    pub channels: u8,
    pub bits_per_channel: u8,
}

impl ImageColorInfo {
    pub fn from_color_type(color_type: impl Into<ExtendedColorType>) -> Option<Self> {
        let (channels, bits_per_channel) = match color_type.into() {
            ExtendedColorType::L8 => (1, 8),
            ExtendedColorType::L16 => (1, 16),
            ExtendedColorType::La8 => (2, 8),
            ExtendedColorType::La16 => (2, 16),
            ExtendedColorType::Rgb8 => (3, 8),
            ExtendedColorType::Rgb16 => (3, 16),
            ExtendedColorType::Rgba8 => (4, 8),
            ExtendedColorType::Rgba16 => (4, 16),
            ExtendedColorType::A8 => (1, 8),
            ExtendedColorType::Bgr8 => (3, 8),
            ExtendedColorType::Bgra8 => (4, 8),
            ExtendedColorType::Cmyk8 => (4, 8),
            _ => return None,
        };

        Some(Self {
            channels,
            bits_per_channel,
        })
    }

    pub const fn bits_per_pixel(&self) -> u8 {
        self.channels * self.bits_per_channel
    }
}

pub struct ImageItem {
    pub id: ImageId,
    pub file: Arc<worktree::File>,
    pub image: Arc<gpui::Image>,
    reload_task: Option<Task<()>>,
    pub image_metadata: Option<ImageMetadata>,
}

impl ImageItem {
    fn compute_metadata_from_bytes(image_bytes: &[u8]) -> Result<ImageMetadata> {
        let image_format = image::guess_format(image_bytes)?;

        let mut image_reader = ImageReader::new(std::io::Cursor::new(image_bytes));
        image_reader.set_format(image_format);
        let image = image_reader.decode()?;

        let (width, height) = image.dimensions();

        Ok(ImageMetadata {
            width,
            height,
            file_size: image_bytes.len() as u64,
            format: image_format,
            colors: ImageColorInfo::from_color_type(image.color()),
        })
    }

    pub async fn load_image_metadata(
        image: Entity<ImageItem>,
        project: Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<ImageMetadata> {
        let (fs, image_path) = cx.update(|cx| {
            let fs = project.read(cx).fs().clone();
            let image_path = image
                .read(cx)
                .abs_path(cx)
                .context("absolutizing image file path")?;
            anyhow::Ok((fs, image_path))
        })?;

        let image_bytes = fs.load_bytes(&image_path).await?;
        Self::compute_metadata_from_bytes(&image_bytes)
    }

    pub fn project_path(&self, cx: &App) -> ProjectPath {
        ProjectPath {
            worktree_id: self.file.worktree_id(cx),
            path: self.file.path().clone(),
        }
    }

    pub fn abs_path(&self, cx: &App) -> Option<PathBuf> {
        Some(self.file.as_local()?.abs_path(cx))
    }

    fn file_updated(&mut self, new_file: Arc<worktree::File>, cx: &mut Context<Self>) {
        let mut file_changed = false;

        let old_file = &self.file;
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

    fn reload(&mut self, cx: &mut Context<Self>) -> Option<oneshot::Receiver<()>> {
        let local_file = self.file.as_local()?;
        let (tx, rx) = futures::channel::oneshot::channel();

        let content = local_file.load_bytes(cx);
        self.reload_task = Some(cx.spawn(async move |this, cx| {
            if let Some(image) = content
                .await
                .context("Failed to load image content")
                .and_then(create_gpui_image)
                .log_err()
            {
                this.update(cx, |this, cx| {
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

pub fn is_image_file(project: &Entity<Project>, path: &ProjectPath, cx: &App) -> bool {
    let ext = util::maybe!({
        let worktree_abs_path = project
            .read(cx)
            .worktree_for_id(path.worktree_id, cx)?
            .read(cx)
            .abs_path();
        path.path
            .extension()
            .or_else(|| worktree_abs_path.extension()?.to_str())
            .map(str::to_lowercase)
    });

    match ext {
        Some(ext) => Img::extensions().contains(&ext.as_str()) && !ext.contains("svg"),
        None => false,
    }
}

impl ProjectItem for ImageItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
        if is_image_file(project, path, cx) {
            Some(cx.spawn({
                let path = path.clone();
                let project = project.clone();
                async move |cx| {
                    project
                        .update(cx, |project, cx| project.open_image(path, cx))
                        .await
                }
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        self.file.entry_id
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        Some(self.project_path(cx))
    }

    fn is_dirty(&self) -> bool {
        false
    }
}

trait ImageStoreImpl {
    fn open_image(
        &self,
        path: Arc<RelPath>,
        worktree: Entity<Worktree>,
        cx: &mut Context<ImageStore>,
    ) -> Task<Result<Entity<ImageItem>>>;

    fn reload_images(
        &self,
        images: HashSet<Entity<ImageItem>>,
        cx: &mut Context<ImageStore>,
    ) -> Task<Result<()>>;

    fn as_local(&self) -> Option<Entity<LocalImageStore>>;
    fn as_remote(&self) -> Option<Entity<RemoteImageStore>>;
}

struct RemoteImageStore {
    upstream_client: AnyProtoClient,
    project_id: u64,
    loading_remote_images_by_id: HashMap<ImageId, LoadingRemoteImage>,
    remote_image_listeners:
        HashMap<ImageId, Vec<oneshot::Sender<anyhow::Result<Entity<ImageItem>>>>>,
    loaded_images: HashMap<ImageId, Entity<ImageItem>>,
}

struct LoadingRemoteImage {
    state: proto::ImageState,
    chunks: Vec<Vec<u8>>,
    received_size: u64,
}

struct LocalImageStore {
    local_image_ids_by_path: HashMap<ProjectPath, ImageId>,
    local_image_ids_by_entry_id: HashMap<ProjectEntryId, ImageId>,
    image_store: WeakEntity<ImageStore>,
    _subscription: Subscription,
}

pub struct ImageStore {
    state: Box<dyn ImageStoreImpl>,
    opened_images: HashMap<ImageId, WeakEntity<ImageItem>>,
    worktree_store: Entity<WorktreeStore>,
    #[allow(clippy::type_complexity)]
    loading_images_by_path: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<Entity<ImageItem>, Arc<anyhow::Error>>>>,
    >,
}

impl ImageStore {
    pub fn local(worktree_store: Entity<WorktreeStore>, cx: &mut Context<Self>) -> Self {
        let this = cx.weak_entity();
        Self {
            state: Box::new(cx.new(|cx| {
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
        worktree_store: Entity<WorktreeStore>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            state: Box::new(cx.new(|_| RemoteImageStore {
                upstream_client,
                project_id,
                loading_remote_images_by_id: Default::default(),
                remote_image_listeners: Default::default(),
                loaded_images: Default::default(),
            })),
            opened_images: Default::default(),
            loading_images_by_path: Default::default(),
            worktree_store,
        }
    }

    pub fn images(&self) -> impl '_ + Iterator<Item = Entity<ImageItem>> {
        self.opened_images
            .values()
            .filter_map(|image| image.upgrade())
    }

    pub fn get(&self, image_id: ImageId) -> Option<Entity<ImageItem>> {
        self.opened_images
            .get(&image_id)
            .and_then(|image| image.upgrade())
    }

    pub fn get_by_path(&self, path: &ProjectPath, cx: &App) -> Option<Entity<ImageItem>> {
        self.images()
            .find(|image| &image.read(cx).project_path(cx) == path)
    }

    pub fn open_image(
        &mut self,
        project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<ImageItem>>> {
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

                let load_image = self
                    .state
                    .open_image(project_path.path.clone(), worktree, cx);

                cx.spawn(async move |this, cx| {
                    let load_result = load_image.await;
                    *tx.borrow_mut() = Some(this.update(cx, |this, _cx| {
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

        cx.background_spawn(async move {
            Self::wait_for_loading_image(loading_watch)
                .await
                .map_err(|e| e.cloned())
        })
    }

    pub async fn wait_for_loading_image(
        mut receiver: postage::watch::Receiver<
            Option<Result<Entity<ImageItem>, Arc<anyhow::Error>>>,
        >,
    ) -> Result<Entity<ImageItem>, Arc<anyhow::Error>> {
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
        images: HashSet<Entity<ImageItem>>,
        cx: &mut Context<ImageStore>,
    ) -> Task<Result<()>> {
        if images.is_empty() {
            return Task::ready(Ok(()));
        }

        self.state.reload_images(images, cx)
    }

    fn add_image(&mut self, image: Entity<ImageItem>, cx: &mut Context<ImageStore>) -> Result<()> {
        let image_id = image.read(cx).id;
        self.opened_images.insert(image_id, image.downgrade());
        cx.subscribe(&image, Self::on_image_event).detach();
        cx.emit(ImageStoreEvent::ImageAdded(image));
        Ok(())
    }

    fn on_image_event(
        &mut self,
        image: Entity<ImageItem>,
        event: &ImageItemEvent,
        cx: &mut Context<Self>,
    ) {
        if let ImageItemEvent::FileHandleChanged = event
            && let Some(local) = self.state.as_local()
        {
            local.update(cx, |local, cx| {
                local.image_changed_file(image, cx);
            })
        }
    }

    pub fn handle_create_image_for_peer(
        &mut self,
        envelope: TypedEnvelope<proto::CreateImageForPeer>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Some(remote) = self.state.as_remote() {
            let worktree_store = self.worktree_store.clone();
            let image = remote.update(cx, |remote, cx| {
                remote.handle_create_image_for_peer(envelope, &worktree_store, cx)
            })?;
            if let Some(image) = image {
                remote.update(cx, |this, cx| {
                    let image = image.clone();
                    let image_id = image.read(cx).id;
                    this.loaded_images.insert(image_id, image)
                });

                self.add_image(image, cx)?;
            }
        }

        Ok(())
    }
}

impl RemoteImageStore {
    pub fn wait_for_remote_image(
        &mut self,
        id: ImageId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<ImageItem>>> {
        if let Some(image) = self.loaded_images.remove(&id) {
            return Task::ready(Ok(image));
        }

        let (tx, rx) = oneshot::channel();
        self.remote_image_listeners.entry(id).or_default().push(tx);

        cx.spawn(async move |_this, cx| {
            let result = cx.background_spawn(async move { rx.await? }).await;
            result
        })
    }

    pub fn handle_create_image_for_peer(
        &mut self,
        envelope: TypedEnvelope<proto::CreateImageForPeer>,
        worktree_store: &Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Result<Option<Entity<ImageItem>>> {
        use proto::create_image_for_peer::Variant;
        match envelope.payload.variant {
            Some(Variant::State(state)) => {
                let image_id =
                    ImageId::from(NonZeroU64::new(state.id).context("invalid image id")?);

                self.loading_remote_images_by_id.insert(
                    image_id,
                    LoadingRemoteImage {
                        state,
                        chunks: Vec::new(),
                        received_size: 0,
                    },
                );
                Ok(None)
            }
            Some(Variant::Chunk(chunk)) => {
                let image_id =
                    ImageId::from(NonZeroU64::new(chunk.image_id).context("invalid image id")?);

                let loading = self
                    .loading_remote_images_by_id
                    .get_mut(&image_id)
                    .context("received chunk for unknown image")?;

                loading.received_size += chunk.data.len() as u64;
                loading.chunks.push(chunk.data);

                if loading.received_size == loading.state.content_size {
                    let loading = self.loading_remote_images_by_id.remove(&image_id).unwrap();

                    let mut content = Vec::with_capacity(loading.received_size as usize);
                    for chunk_data in loading.chunks {
                        content.extend_from_slice(&chunk_data);
                    }

                    let image_metadata = ImageItem::compute_metadata_from_bytes(&content).log_err();
                    let image = create_gpui_image(content)?;

                    let proto_file = loading.state.file.context("missing file in image state")?;
                    let worktree_id = WorktreeId::from_proto(proto_file.worktree_id);
                    let worktree = worktree_store
                        .read(cx)
                        .worktree_for_id(worktree_id, cx)
                        .context("worktree not found")?;

                    let file = Arc::new(
                        worktree::File::from_proto(proto_file, worktree, cx)
                            .context("invalid file in image state")?,
                    );

                    let entity = cx.new(|_cx| ImageItem {
                        id: image_id,
                        file,
                        image,
                        image_metadata,
                        reload_task: None,
                    });

                    if let Some(listeners) = self.remote_image_listeners.remove(&image_id) {
                        for listener in listeners {
                            listener.send(Ok(entity.clone())).ok();
                        }
                    }

                    Ok(Some(entity))
                } else {
                    Ok(None)
                }
            }
            None => {
                log::warn!("Received CreateImageForPeer with no variant");
                Ok(None)
            }
        }
    }

    // TODO: subscribe to worktree and update image contents or at least mark as dirty on file changes
}

impl ImageStoreImpl for Entity<LocalImageStore> {
    fn open_image(
        &self,
        path: Arc<RelPath>,
        worktree: Entity<Worktree>,
        cx: &mut Context<ImageStore>,
    ) -> Task<Result<Entity<ImageItem>>> {
        let this = self.clone();

        let load_file = worktree.update(cx, |worktree, cx| {
            worktree.load_binary_file(path.as_ref(), cx)
        });
        cx.spawn(async move |image_store, cx| {
            let LoadedBinaryFile { file, content } = load_file.await?;
            let image = create_gpui_image(content)?;

            let entity = cx.new(|cx| ImageItem {
                id: cx.entity_id().as_non_zero_u64().into(),
                file: file.clone(),
                image,
                image_metadata: None,
                reload_task: None,
            });

            let image_id = cx.read_entity(&entity, |model, _| model.id);

            this.update(cx, |this, cx| {
                image_store.update(cx, |image_store, cx| {
                    image_store.add_image(entity.clone(), cx)
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
            })?;

            Ok(entity)
        })
    }

    fn reload_images(
        &self,
        images: HashSet<Entity<ImageItem>>,
        cx: &mut Context<ImageStore>,
    ) -> Task<Result<()>> {
        cx.spawn(async move |_, cx| {
            for image in images {
                if let Some(rec) = image.update(cx, |image, cx| image.reload(cx)) {
                    rec.await?
                }
            }
            Ok(())
        })
    }

    fn as_local(&self) -> Option<Entity<LocalImageStore>> {
        Some(self.clone())
    }

    fn as_remote(&self) -> Option<Entity<RemoteImageStore>> {
        None
    }
}

impl ImageStoreImpl for Entity<RemoteImageStore> {
    fn open_image(
        &self,
        path: Arc<RelPath>,
        worktree: Entity<Worktree>,
        cx: &mut Context<ImageStore>,
    ) -> Task<Result<Entity<ImageItem>>> {
        let worktree_id = worktree.read(cx).id().to_proto();
        let (project_id, client) = {
            let store = self.read(cx);
            (store.project_id, store.upstream_client.clone())
        };
        let remote_store = self.clone();

        cx.spawn(async move |_image_store, cx| {
            let response = client
                .request(rpc::proto::OpenImageByPath {
                    project_id,
                    worktree_id,
                    path: path.to_proto(),
                })
                .await?;

            let image_id = ImageId::from(
                NonZeroU64::new(response.image_id).context("invalid image_id in response")?,
            );

            remote_store
                .update(cx, |remote_store, cx| {
                    remote_store.wait_for_remote_image(image_id, cx)
                })
                .await
        })
    }

    fn reload_images(
        &self,
        _images: HashSet<Entity<ImageItem>>,
        _cx: &mut Context<ImageStore>,
    ) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "Reloading images from remote is not supported"
        )))
    }

    fn as_local(&self) -> Option<Entity<LocalImageStore>> {
        None
    }

    fn as_remote(&self) -> Option<Entity<RemoteImageStore>> {
        Some(self.clone())
    }
}

impl LocalImageStore {
    fn subscribe_to_worktree(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        cx.subscribe(worktree, |this, worktree, event, cx| {
            if worktree.read(cx).is_local()
                && let worktree::Event::UpdatedEntries(changes) = event
            {
                this.local_worktree_entries_changed(&worktree, changes, cx);
            }
        })
        .detach();
    }

    fn local_worktree_entries_changed(
        &mut self,
        worktree_handle: &Entity<Worktree>,
        changes: &[(Arc<RelPath>, ProjectEntryId, PathChange)],
        cx: &mut Context<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();
        for (path, entry_id, _) in changes {
            self.local_worktree_entry_changed(*entry_id, path, worktree_handle, &snapshot, cx);
        }
    }

    fn local_worktree_entry_changed(
        &mut self,
        entry_id: ProjectEntryId,
        path: &Arc<RelPath>,
        worktree: &Entity<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut Context<Self>,
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
            let old_file = &image.file;
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

            if new_file == **old_file {
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

    fn image_changed_file(&mut self, image: Entity<ImageItem>, cx: &mut App) -> Option<()> {
        let image = image.read(cx);
        let file = &image.file;

        let image_id = image.id;
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

    Ok(Arc::new(gpui::Image::from_bytes(
        match format {
            image::ImageFormat::Png => gpui::ImageFormat::Png,
            image::ImageFormat::Jpeg => gpui::ImageFormat::Jpeg,
            image::ImageFormat::WebP => gpui::ImageFormat::Webp,
            image::ImageFormat::Gif => gpui::ImageFormat::Gif,
            image::ImageFormat::Bmp => gpui::ImageFormat::Bmp,
            image::ImageFormat::Tiff => gpui::ImageFormat::Tiff,
            image::ImageFormat::Ico => gpui::ImageFormat::Ico,
            format => anyhow::bail!("Image format {format:?} not supported"),
        },
        content,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::rel_path::rel_path;

    pub fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
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
            path: rel_path("image_1.png").into(),
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

    #[gpui::test]
    fn test_compute_metadata_from_bytes() {
        // Single white pixel PNG
        let png_bytes = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];

        let metadata = ImageItem::compute_metadata_from_bytes(&png_bytes).unwrap();

        assert_eq!(metadata.width, 1);
        assert_eq!(metadata.height, 1);
        assert_eq!(metadata.file_size, png_bytes.len() as u64);
        assert_eq!(metadata.format, image::ImageFormat::Png);
        assert!(metadata.colors.is_some());
    }
}

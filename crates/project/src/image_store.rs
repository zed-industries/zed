use crate::{
    Project, ProjectEntryId, ProjectItem, ProjectPath,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};
use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet, hash_map};
use futures::{StreamExt, channel::oneshot};
use gpui::{
    App, Asset, AssetLogger, AsyncApp, Context, Entity, EventEmitter, ImageCacheError, ImageSource,
    Img, RenderImage, Subscription, Task, WeakEntity, prelude::*,
};
pub use image::ImageFormat;
use image::{ExtendedColorType, GenericImageView, ImageReader};
use language::{DiskState, File};
use rpc::{AnyProtoClient, ErrorExt as _, TypedEnvelope, proto};
use std::future::Future;
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
    pub pdf_info: Option<PdfPageInfo>,
}

#[derive(Clone)]
pub struct PdfPageEntry {
    pub page_index: usize,
    pub width: u32,
    pub height: u32,
    pub image: Arc<gpui::Image>,
    pub links: Vec<kkpdf_zed::PdfLinkAnnotation>,
    pub text_segments: Vec<kkpdf_zed::PdfTextSegment>,
    pub page_text: String,
}

#[derive(Clone)]
pub struct PdfPageInfo {
    pub current_page: usize,
    pub total_pages: usize,
    pub pdf_bytes: Arc<Vec<u8>>,
    pub pages: Vec<PdfPageEntry>,
    pub full_text: Option<String>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct ProjectImageSource {
    project: WeakEntity<Project>,
    path: ProjectPath,
}

enum ProjectImageAsset {}

impl Asset for ProjectImageAsset {
    type Source = ProjectImageSource;
    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        source: Self::Source,
        cx: &mut App,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let svg_renderer = cx.svg_renderer();
        let load_image = cx.spawn(async move |cx| {
            let open_image = source
                .project
                .update(cx, |project, cx| project.open_image(source.path, cx))?;
            let image = open_image.await?;
            Ok::<_, anyhow::Error>(image.read_with(cx, |image, _cx| image.image.clone()))
        });

        async move {
            let image = load_image.await?;
            image.to_image_data(svg_renderer).map_err(Into::into)
        }
    }
}

pub fn project_image_source(project: WeakEntity<Project>, path: ProjectPath) -> ImageSource {
    let source = ProjectImageSource { project, path };
    ImageSource::from(move |window: &mut gpui::Window, cx: &mut App| {
        window.use_asset::<AssetLogger<ProjectImageAsset>>(&source, cx)
    })
}

impl ImageItem {
    pub fn compute_metadata_from_bytes(image_bytes: &[u8]) -> Result<ImageMetadata> {
        if image_bytes.starts_with(b"%PDF-") {
            let engine = kkpdf_zed::pdfium::PdfiumEngine::new();
            let doc = engine.load_document_from_bytes(image_bytes, None)?;
            let dim = doc.page_size(0).unwrap_or(kkpdf_zed::PageDimensions::new(612.0, 792.0));
            let (width, height) = dim.to_pixel_size(1.0, 144.0);
            return Ok(ImageMetadata {
                width,
                height,
                file_size: image_bytes.len() as u64,
                format: image::ImageFormat::Png,
                colors: ImageColorInfo::from_color_type(image::ColorType::Rgba8),
            });
        }

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

    pub fn is_pdf(&self) -> bool {
        self.pdf_info.is_some()
    }

    pub fn current_page(&self) -> usize {
        self.pdf_info.as_ref().map(|p| p.current_page).unwrap_or(0)
    }

    pub fn total_pages(&self) -> usize {
        self.pdf_info.as_ref().map(|p| p.total_pages).unwrap_or(1)
    }

    pub fn extract_text(&self) -> Option<String> {
        let info = self.pdf_info.as_ref()?;
        if let Some(ref text) = info.full_text {
            Some(text.clone())
        } else {
            kkpdf_zed::PdfiumEngine::new().extract_text_from_bytes(&info.pdf_bytes, None).ok()
        }
    }

    pub fn set_page(&mut self, page_index: usize, cx: &mut Context<Self>) -> bool {
        let Some(ref mut info) = self.pdf_info else {
            return false;
        };
        if page_index >= info.total_pages || page_index == info.current_page {
            return false;
        }

        info.current_page = page_index;
        if let Some(page) = info.pages.get(page_index) {
            self.image = page.image.clone();
            self.image_metadata = Some(ImageMetadata {
                width: page.width,
                height: page.height,
                file_size: info.pdf_bytes.len() as u64,
                format: image::ImageFormat::Png,
                colors: ImageColorInfo::from_color_type(image::ColorType::Rgba8),
            });
            cx.emit(ImageItemEvent::Reloaded);
            cx.emit(ImageItemEvent::MetadataUpdated);
            cx.notify();
            true
        } else {
            false
        }
    }

    pub fn next_page(&mut self, cx: &mut Context<Self>) -> bool {
        let next = self.current_page() + 1;
        self.set_page(next, cx)
    }

    pub fn previous_page(&mut self, cx: &mut Context<Self>) -> bool {
        let cur = self.current_page();
        if cur > 0 {
            self.set_page(cur - 1, cx)
        } else {
            false
        }
    }

    pub fn first_page(&mut self, cx: &mut Context<Self>) -> bool {
        self.set_page(0, cx)
    }

    pub fn last_page(&mut self, cx: &mut Context<Self>) -> bool {
        let last = self.total_pages().saturating_sub(1);
        self.set_page(last, cx)
    }

    fn reload(&mut self, cx: &mut Context<Self>) -> Option<oneshot::Receiver<()>> {
        let local_file = self.file.as_local()?;
        let (tx, rx) = futures::channel::oneshot::channel();

        let content = local_file.load_bytes(cx);
        let background = cx.background_executor().clone();
        self.reload_task = Some(cx.spawn(async move |this, cx| {
            if let Ok(bytes) = content.await.context("Failed to load image content") {
                let is_pdf = bytes.starts_with(b"%PDF-");
                let cur_page = this.read_with(cx, |this, _| this.current_page()).unwrap_or(0);

                if is_pdf {
                    let bytes_clone = bytes.clone();
                    let pdf_res = background.spawn(async move {
                        create_gpui_images_from_pdf(&bytes_clone)
                    }).await;
                    if let Ok((pages, full_text)) = pdf_res {
                        let total_pages = pages.len();
                        let target_page = cur_page.min(total_pages.saturating_sub(1));
                        let first_image = pages
                            .get(target_page)
                            .map(|p| p.image.clone())
                            .unwrap_or_else(|| create_gpui_image(bytes.clone()).unwrap());
                        let (width, height) = pages
                            .get(target_page)
                            .map(|p| (p.width, p.height))
                            .unwrap_or((0, 0));
                        this.update(cx, |this, cx| {
                            this.image = first_image;
                            this.pdf_info = Some(PdfPageInfo {
                                current_page: target_page,
                                total_pages,
                                pdf_bytes: Arc::new(bytes.clone()),
                                pages,
                                full_text,
                            });
                            this.image_metadata = Some(ImageMetadata {
                                width,
                                height,
                                file_size: bytes.len() as u64,
                                format: image::ImageFormat::Png,
                                colors: ImageColorInfo::from_color_type(image::ColorType::Rgba8),
                            });
                            cx.emit(ImageItemEvent::Reloaded);
                            cx.emit(ImageItemEvent::MetadataUpdated);
                        })
                        .log_err();
                    }
                } else if let Some(image) = create_gpui_image(bytes).log_err() {
                    this.update(cx, |this, cx| {
                        this.image = image;
                        this.pdf_info = None;
                        cx.emit(ImageItemEvent::Reloaded);
                    })
                    .log_err();
                }
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
        Some(ext) => (Img::extensions().contains(&ext.as_str()) || ext == "pdf") && !ext.contains("svg"),
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

                    let is_pdf = content.starts_with(b"%PDF-");
                    let (image, pdf_info) = if is_pdf {
                        if let Ok((pages, full_text)) = create_gpui_images_from_pdf(&content) {
                            let first_image = pages
                                .first()
                                .map(|p| p.image.clone())
                                .unwrap_or_else(|| create_gpui_image(content.clone()).unwrap());
                            let total_pages = pages.len();
                            let pdf_info = PdfPageInfo {
                                current_page: 0,
                                total_pages,
                                pdf_bytes: Arc::new(content.clone()),
                                pages,
                                full_text,
                            };
                            (first_image, Some(pdf_info))
                        } else {
                            (create_gpui_image(content.clone())?, None)
                        }
                    } else {
                        (create_gpui_image(content.clone())?, None)
                    };
                    let image_metadata = ImageItem::compute_metadata_from_bytes(&content).log_err();

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
                        pdf_info,
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
        let background = cx.background_executor().clone();
        cx.spawn(async move |image_store, cx| {
            let LoadedBinaryFile { file, content } = load_file.await?;
            let is_pdf = content.starts_with(b"%PDF-");
            let (image, pdf_info) = if is_pdf {
                let content_clone = content.clone();
                let pdf_res = background.spawn(async move {
                    create_gpui_images_from_pdf(&content_clone)
                }).await;
                if let Ok((pages, full_text)) = pdf_res {
                    let first_image = pages
                        .first()
                        .map(|p| p.image.clone())
                        .unwrap_or_else(|| create_gpui_image(content.clone()).unwrap());
                    let total_pages = pages.len();
                    let pdf_info = PdfPageInfo {
                        current_page: 0,
                        total_pages,
                        pdf_bytes: Arc::new(content.clone()),
                        pages,
                        full_text,
                    };
                    (first_image, Some(pdf_info))
                } else {
                    (create_gpui_image(content.clone())?, None)
                }
            } else {
                (create_gpui_image(content.clone())?, None)
            };

            let entity = cx.new(|cx| ImageItem {
                id: cx.entity_id().as_non_zero_u64().into(),
                file: file.clone(),
                image,
                image_metadata: None,
                pdf_info,
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
                    path: path.as_unix_str().to_owned(),
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
                        Some(mtime) => DiskState::Present {
                            mtime,
                            size: entry.size,
                        },
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

pub fn create_gpui_images_from_pdf(
    pdf_bytes: &[u8],
) -> anyhow::Result<(Vec<PdfPageEntry>, Option<String>)> {
    let engine = kkpdf_zed::PdfiumEngine::new();
    let doc_details = engine.extract_document_details(pdf_bytes).unwrap_or_default();
    let doc = engine.load_document_from_bytes(pdf_bytes, None)?;
    let total_pages = doc.total_pages();
    let options = kkpdf_zed::rasterizer::RasterizerOptions {
        target_dpi: 144.0,
        zoom_factor: 1.0,
        dark_mode: false,
        saturation_threshold: 0.18,
    };

    let mut pages = Vec::with_capacity(total_pages);
    for page_idx in 0..total_pages {
        let page = engine.render_page_from_bytes(pdf_bytes, page_idx, options)?;
        let mut png_bytes: Vec<u8> = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
            page.width,
            page.height,
            page.rgba_buffer.as_ref().clone(),
        )
        .ok_or_else(|| anyhow::anyhow!("Failed to convert RGBA to ImageBuffer"))?;

        img_buf.write_to(&mut cursor, image::ImageFormat::Png)?;
        let image = Arc::new(gpui::Image::from_bytes(
            gpui::ImageFormat::Png,
            png_bytes,
        ));

        let page_detail = doc_details.pages.get(page_idx);
        let links = page_detail.map(|d| d.links.clone()).unwrap_or_default();
        let text_segments = page_detail.map(|d| d.text_segments.clone()).unwrap_or_default();
        let page_text = page_detail.map(|d| d.text.clone()).unwrap_or_default();

        pages.push(PdfPageEntry {
            page_index: page_idx,
            width: page.width,
            height: page.height,
            image,
            links,
            text_segments,
            page_text,
        });
    }

    let full_text = if !doc_details.full_text.is_empty() {
        Some(doc_details.full_text)
    } else {
        engine.extract_text_from_bytes(pdf_bytes, None).ok()
    };

    Ok((pages, full_text))
}

pub fn create_gpui_image_from_pdf_page(
    pdf_bytes: &[u8],
    page_index: usize,
) -> anyhow::Result<(Arc<gpui::Image>, u32, u32)> {
    let engine = kkpdf_zed::pdfium::PdfiumEngine::new();
    let options = kkpdf_zed::rasterizer::RasterizerOptions {
        target_dpi: 144.0,
        zoom_factor: 1.0,
        dark_mode: false,
        saturation_threshold: 0.18,
    };
    let page = engine.render_page_from_bytes(pdf_bytes, page_index, options)?;

    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_bytes);
    let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
        page.width,
        page.height,
        page.rgba_buffer.as_ref().clone(),
    )
    .ok_or_else(|| anyhow::anyhow!("Failed to convert RGBA to ImageBuffer"))?;

    img_buf.write_to(&mut cursor, image::ImageFormat::Png)?;
    Ok((
        Arc::new(gpui::Image::from_bytes(
            gpui::ImageFormat::Png,
            png_bytes,
        )),
        page.width,
        page.height,
    ))
}

fn create_gpui_image(content: Vec<u8>) -> anyhow::Result<Arc<gpui::Image>> {
    if content.starts_with(b"%PDF-") {
        let (img, _, _) = create_gpui_image_from_pdf_page(&content, 0)?;
        return Ok(img);
    }

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
            image::ImageFormat::Pnm => gpui::ImageFormat::Pnm,
            format => anyhow::bail!("Image format {format:?} not supported"),
        },
        content,
    )))
}

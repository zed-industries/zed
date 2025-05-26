use anyhow::Result;
use futures::AsyncReadExt;
use gpui::{Context, Entity, Window};
use http_client::HttpClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use ui::{IconName, List, ListItem, Label, LabelSize, Button, ButtonStyle, IconButton, prelude::*, Color, Icon};
use ui_input::SingleLineInput;

/// Information about a model available in a repository (e.g., Hugging Face)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RepositoryModel {
    /// Model ID (e.g., "microsoft/DialoGPT-medium")
    pub id: String,
    /// Display name for the model
    pub name: String,
    /// Model description
    pub description: Option<String>,
    /// Model author/organization
    pub author: String,
    /// Model tags (e.g., ["conversational", "text-generation"])
    pub tags: Vec<String>,
    /// Number of downloads
    pub downloads: u64,
    /// Number of likes
    pub likes: u32,
    /// Model size in bytes (if available)
    pub size: Option<u64>,
    /// Available files for download
    pub files: Vec<ModelFile>,
    /// Repository URL
    pub repository_url: String,
    /// Last modified date
    pub last_modified: Option<String>,
}

/// Information about a specific model file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ModelFile {
    /// File name
    pub name: String,
    /// File size in bytes
    pub size: u64,
    /// File type/format (e.g., "gguf", "safetensors")
    pub file_type: String,
    /// Download URL
    pub download_url: String,
    /// File hash for verification (if available)
    pub hash: Option<String>,
}

/// Download status for a model
#[derive(Clone, Debug, PartialEq)]
pub enum DownloadStatus {
    /// Not started
    Pending,
    /// Currently downloading
    Downloading { progress: f32, speed: String },
    /// Download completed successfully
    Completed,
    /// Download failed
    Failed { error: String },
    /// Download was cancelled
    Cancelled,
}

/// Information about a model download
#[derive(Debug)]
pub struct ModelDownload {
    /// Model being downloaded
    pub model: RepositoryModel,
    /// Selected file to download
    pub file: ModelFile,
    /// Current download status
    pub status: DownloadStatus,
    /// Local file path where the model will be saved
    pub local_path: PathBuf,
}

/// Configuration for repository providers
pub trait RepositoryProvider {
    /// Search for models in the repository
    fn search_models(
        http_client: Arc<dyn HttpClient>,
        query: String,
    ) -> impl std::future::Future<Output = Result<Vec<RepositoryModel>>> + Send;

    /// Download a file from the repository
    fn download_file(
        http_client: Arc<dyn HttpClient>,
        url: String,
        local_path: PathBuf,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Hugging Face repository provider
pub struct HuggingFaceProvider;

impl RepositoryProvider for HuggingFaceProvider {
    async fn search_models(
        http_client: Arc<dyn HttpClient>,
        query: String,
    ) -> Result<Vec<RepositoryModel>> {
        use http_client::{Method, Request};
        
        // Search for GGUF models on Hugging Face
        let search_url = format!(
            "https://huggingface.co/api/models?search={}&filter=gguf&sort=downloads&direction=-1&limit=20",
            urlencoding::encode(&query)
        );

        let request = Request::builder()
            .method(Method::GET)
            .uri(search_url)
            .header("User-Agent", "Zed-Editor/1.0")
            .body(Default::default())?;

        let response = http_client.send(request).await?;
        let mut body = response.into_body();
        let mut body_bytes = Vec::new();
        body.read_to_end(&mut body_bytes).await?;
        
        #[derive(Deserialize)]
        struct HuggingFaceModel {
            id: String,
            #[serde(rename = "modelId")]
            model_id: Option<String>,
            author: Option<String>,
            #[serde(rename = "lastModified")]
            last_modified: Option<String>,
            downloads: Option<u64>,
            likes: Option<u32>,
            tags: Option<Vec<String>>,
            #[serde(rename = "cardData")]
            card_data: Option<serde_json::Value>,
        }

        let hf_models: Vec<HuggingFaceModel> = serde_json::from_slice(&body_bytes)?;
        
        let mut models = Vec::new();
        for hf_model in hf_models {
            // Fetch model files to find GGUF files
            let files = Self::fetch_model_files(http_client.clone(), &hf_model.id).await?;
            let gguf_files: Vec<_> = files.into_iter()
                .filter(|f| f.file_type == "gguf")
                .collect();

            if !gguf_files.is_empty() {
                let description = hf_model.card_data
                    .as_ref()
                    .and_then(|data| data.get("description"))
                    .and_then(|desc| desc.as_str())
                    .map(|s| s.to_string());
                
                let model = RepositoryModel {
                    id: hf_model.id.clone(),
                    name: hf_model.model_id.unwrap_or(hf_model.id.clone()),
                    description,
                    author: hf_model.author.unwrap_or_else(|| {
                        hf_model.id.split('/').next().unwrap_or("Unknown").to_string()
                    }),
                    tags: hf_model.tags.unwrap_or_default(),
                    downloads: hf_model.downloads.unwrap_or(0),
                    likes: hf_model.likes.unwrap_or(0),
                    size: gguf_files.iter().map(|f| f.size).sum::<u64>().into(),
                    files: gguf_files,
                    repository_url: format!("https://huggingface.co/{}", hf_model.id),
                    last_modified: hf_model.last_modified,
                };
                models.push(model);
            }
        }

        Ok(models)
    }

    async fn download_file(
        http_client: Arc<dyn HttpClient>,
        url: String,
        local_path: PathBuf,
    ) -> Result<()> {
        use http_client::{Method, Request};
        use std::io::Write;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = local_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("User-Agent", "Zed-Editor/1.0")
            .body(Default::default())?;

        let response = http_client.send(request).await?;
        let mut body = response.into_body();
        let mut body_bytes = Vec::new();
        body.read_to_end(&mut body_bytes).await?;
        
        // Write to file
        let mut file = std::fs::File::create(local_path)?;
        file.write_all(&body_bytes)?;
        file.sync_all()?;

        Ok(())
    }
}

impl HuggingFaceProvider {
    async fn fetch_model_files(
        http_client: Arc<dyn HttpClient>,
        model_id: &str,
    ) -> Result<Vec<ModelFile>> {
        use http_client::{Method, Request};
        
        let files_url = format!("https://huggingface.co/api/models/{}/tree/main", model_id);

        let request = Request::builder()
            .method(Method::GET)
            .uri(files_url)
            .header("User-Agent", "Zed-Editor/1.0")
            .body(Default::default())?;

        let response = http_client.send(request).await?;
        let mut body = response.into_body();
        let mut body_bytes = Vec::new();
        body.read_to_end(&mut body_bytes).await?;
        
        #[derive(Deserialize)]
        struct HuggingFaceFile {
            path: String,
            size: Option<u64>,
            #[serde(rename = "type")]
            file_type: String,
            oid: Option<String>,
        }

        let hf_files: Vec<HuggingFaceFile> = serde_json::from_slice(&body_bytes)?;
        
        let mut files = Vec::new();
        for hf_file in hf_files {
            if hf_file.file_type == "file" && hf_file.path.ends_with(".gguf") {
                let file = ModelFile {
                    name: hf_file.path.clone(),
                    size: hf_file.size.unwrap_or(0),
                    file_type: "gguf".to_string(),
                    download_url: format!(
                        "https://huggingface.co/{}/resolve/main/{}",
                        model_id, hf_file.path
                    ),
                    hash: hf_file.oid,
                };
                files.push(file);
            }
        }

        Ok(files)
    }
}

/// Generic model repository component
pub struct ModelRepository<P: RepositoryProvider> {
    http_client: Arc<dyn HttpClient>,
    models_directory: PathBuf,
    search_query: String,
    search_input: Entity<SingleLineInput>,
    available_models: Vec<RepositoryModel>,
    active_downloads: Vec<ModelDownload>,
    is_searching: bool,
    selected_model: Option<RepositoryModel>,
    model_detail_view: Option<Entity<ModelDetailView<P>>>,
    _provider: std::marker::PhantomData<P>,
}

impl<P: RepositoryProvider + 'static> ModelRepository<P> {
    pub fn new(http_client: Arc<dyn HttpClient>, models_directory: PathBuf, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| SingleLineInput::new(window, cx, "Search models..."));
        
        Self {
            http_client,
            models_directory,
            search_query: String::new(),
            search_input,
            available_models: Vec::new(),
            active_downloads: Vec::new(),
            is_searching: false,
            selected_model: None,
            model_detail_view: None,
            _provider: std::marker::PhantomData,
        }
    }

    pub fn search_models(&mut self, query: String, cx: &mut Context<Self>) {
        if query.trim().is_empty() {
            return;
        }

        self.is_searching = true;
        self.search_query = query.clone();
        
        let http_client = self.http_client.clone();
        cx.spawn(async move |this, cx| {
            let result = P::search_models(http_client, query).await;
            this.update(cx, |this, cx| {
                this.is_searching = false;
                match result {
                    Ok(models) => {
                        this.available_models = models;
                    }
                    Err(e) => {
                        log::error!("Failed to search models: {}", e);
                        this.available_models.clear();
                    }
                }
                cx.notify();
            }).ok();
        }).detach();
    }

    pub fn download_model(&mut self, model: RepositoryModel, file: ModelFile, cx: &mut Context<Self>) {
        let local_path = self.models_directory.join(&file.name);
        
        // Create download entry
        let download = ModelDownload {
            model: model.clone(),
            file: file.clone(),
            status: DownloadStatus::Pending,
            local_path: local_path.clone(),
        };

        let download_index = self.active_downloads.len();
        self.active_downloads.push(download);

        // Start download task
        let http_client = self.http_client.clone();
        let download_url = file.download_url.clone();
        
        cx.spawn(async move |this, cx| {
            let result = P::download_file(http_client, download_url, local_path.clone()).await;
            
            this.update(cx, |this, cx| {
                if let Some(download) = this.active_downloads.get_mut(download_index) {
                    match result {
                        Ok(_) => {
                            download.status = DownloadStatus::Completed;
                            log::info!("Successfully downloaded model: {}", download.file.name);
                        }
                        Err(e) => {
                            download.status = DownloadStatus::Failed { 
                                error: e.to_string() 
                            };
                            log::error!("Failed to download model: {}", e);
                        }
                    }
                }
                cx.notify();
            }).ok();
        }).detach();

        if let Some(download) = self.active_downloads.get_mut(download_index) {
            download.status = DownloadStatus::Downloading { 
                progress: 0.0, 
                speed: "Starting...".to_string() 
            };
        }
    }

    pub fn show_model_details(&mut self, model: RepositoryModel, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_model = Some(model.clone());
        let repository = cx.entity().clone();
        self.model_detail_view = Some(cx.new(|cx| ModelDetailView::new(model, repository, window, cx)));
    }

    pub fn close_model_details(&mut self, cx: &mut Context<Self>) {
        self.selected_model = None;
        self.model_detail_view = None;
        cx.notify();
    }

    pub fn cancel_download(&mut self, download_index: usize, cx: &mut Context<Self>) {
        if let Some(download) = self.active_downloads.get_mut(download_index) {
            download.status = DownloadStatus::Cancelled;
        }
        cx.notify();
    }

    pub fn format_file_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}

impl<P: RepositoryProvider + 'static> Render for ModelRepository<P> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .size_full()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb_4()
                    .child(
                        Label::new("Model Repository")
                            .size(LabelSize::Large)
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .w_64()
                                    .child(self.search_input.clone())
                            )
                            .child(
                                Button::new("search", "Search")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        let query = this.search_input.read(cx).editor.read(cx).text(cx).to_string();
                                        this.search_models(query, cx);
                                    }))
                            )
                    )
            )
            .child(
                if let Some(detail_view) = &self.model_detail_view {
                    detail_view.clone().into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(
                            if self.is_searching {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(Label::new("Searching models..."))
                                    .into_any_element()
                            } else if self.available_models.is_empty() {
                                div()
                                    .child(
                                        Label::new("Search for models to download from the repository")
                                            .color(Color::Muted)
                                    )
                                    .into_any_element()
                            } else {
                                div()
                                    .child(
                                        List::new()
                                            .children(
                                                self.available_models
                                                    .iter()
                                                    .map(|model| {
                                                        ListItem::new(gpui::SharedString::from(model.id.clone()))
                                                            .start_slot(Icon::new(IconName::Download))
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .justify_between()
                                                                    .items_center()
                                                                    .child(
                                                                        div()
                                                                            .flex()
                                                                            .flex_col()
                                                                            .gap_1()
                                                                            .child(Label::new(model.name.clone()))
                                                                            .child(
                                                                                Label::new(format!("by {}", model.author))
                                                                                    .size(LabelSize::Small)
                                                                                    .color(Color::Muted)
                                                                            )
                                                                            .child(
                                                                                Label::new(format!(
                                                                                    "{} downloads • {} files • {}",
                                                                                    model.downloads,
                                                                                    model.files.len(),
                                                                                    model.size.map(Self::format_file_size)
                                                                                        .unwrap_or_else(|| "Unknown size".to_string())
                                                                                ))
                                                                                .size(LabelSize::Small)
                                                                                .color(Color::Muted)
                                                                            )
                                                                    )
                                                                    .child(
                                                                        Button::new("view_details", "View Details")
                                                                            .style(ButtonStyle::Subtle)
                                                                            .on_click({
                                                                                let model = model.clone();
                                                                                cx.listener(move |this, _, window, cx| {
                                                                                    this.show_model_details(model.clone(), window, cx);
                                                                                })
                                                                            })
                                                                    )
                                                            )
                                                    })
                                            )
                                    )
                                    .into_any_element()
                            }
                        )
                        .when(!self.active_downloads.is_empty(), |this| {
                            this.child(
                                div()
                                    .mt_6()
                                    .child(
                                        Label::new("Active Downloads")
                                            .size(LabelSize::Large)
                                    )
                                    .child(
                                        List::new()
                                            .children(
                                                self.active_downloads
                                                    .iter()
                                                    .enumerate()
                                                    .map(|(index, download)| {
                                                        ListItem::new(gpui::SharedString::from(format!("download_{}", index)))
                                                            .start_slot(Icon::new(IconName::Download))
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .justify_between()
                                                                    .items_center()
                                                                    .child(
                                                                        div()
                                                                            .flex()
                                                                            .flex_col()
                                                                            .gap_1()
                                                                            .child(Label::new(download.file.name.clone()))
                                                                            .child(
                                                                                Label::new(format!("from {}", download.model.name))
                                                                                    .size(LabelSize::Small)
                                                                                    .color(Color::Muted)
                                                                            )
                                                                            .child(
                                                                                match &download.status {
                                                                                    DownloadStatus::Pending => {
                                                                                        Label::new("Pending...")
                                                                                            .size(LabelSize::Small)
                                                                                            .color(Color::Muted)
                                                                                    }
                                                                                    DownloadStatus::Downloading { progress, speed } => {
                                                                                        Label::new(format!("Downloading... {:.1}% ({})", progress * 100.0, speed))
                                                                                            .size(LabelSize::Small)
                                                                                            .color(Color::Accent)
                                                                                    }
                                                                                    DownloadStatus::Completed => {
                                                                                        Label::new("Completed")
                                                                                            .size(LabelSize::Small)
                                                                                            .color(Color::Success)
                                                                                    }
                                                                                    DownloadStatus::Failed { error } => {
                                                                                        Label::new(format!("Failed: {}", error))
                                                                                            .size(LabelSize::Small)
                                                                                            .color(Color::Error)
                                                                                    }
                                                                                    DownloadStatus::Cancelled => {
                                                                                        Label::new("Cancelled")
                                                                                            .size(LabelSize::Small)
                                                                                            .color(Color::Muted)
                                                                                    }
                                                                                }
                                                                            )
                                                                    )
                                                                    .child(
                                                                        match download.status {
                                                                            DownloadStatus::Downloading { .. } => {
                                                                                IconButton::new("cancel", IconName::X)
                                                                                    .on_click({
                                                                                        let download_index = index;
                                                                                        cx.listener(move |this, _, _window, cx| {
                                                                                            this.cancel_download(download_index, cx);
                                                                                        })
                                                                                    })
                                                                                    .into_any_element()
                                                                            }
                                                                            _ => div().into_any_element()
                                                                        }
                                                                    )
                                                            )
                                                    })
                                            )
                                    )
                            )
                        })
                        .into_any_element()
                }
            )
    }
}

pub struct ModelDetailView<P: RepositoryProvider> {
    model: RepositoryModel,
    parent_repository: Entity<ModelRepository<P>>,
    selected_file: Option<ModelFile>,
}

impl<P: RepositoryProvider + 'static> ModelDetailView<P> {
    pub fn new(
        model: RepositoryModel, 
        parent_repository: Entity<ModelRepository<P>>, 
        _window: &mut Window, 
        _cx: &mut Context<Self>
    ) -> Self {
        Self {
            model,
            parent_repository,
            selected_file: None,
        }
    }

    fn download_selected_file(&mut self, _cx: &mut Context<Self>) {
        if let Some(file) = &self.selected_file {
            let model = self.model.clone();
            let file = file.clone();
            self.parent_repository.update(_cx, |repo, cx| {
                repo.download_model(model, file, cx);
            });
        }
    }

    fn close_details(&mut self, _cx: &mut Context<Self>) {
        self.parent_repository.update(_cx, |repo, cx| {
            repo.close_model_details(cx);
        });
    }
}

impl<P: RepositoryProvider + 'static> Render for ModelDetailView<P> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb_4()
                    .child(
                        Label::new("Model Details")
                            .size(LabelSize::Large)
                    )
                    .child(
                        IconButton::new("close", IconName::X)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.close_details(cx);
                            }))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                Label::new(self.model.name.clone())
                                    .size(LabelSize::Large)
                            )
                            .child(
                                Label::new(format!("by {}", self.model.author))
                                    .size(LabelSize::Default)
                                    .color(Color::Muted)
                            )
                            .when_some(self.model.description.as_ref(), |this, desc| {
                                this.child(
                                    Label::new(desc.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                )
                            })
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                Label::new(format!("Downloads: {}", self.model.downloads))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .child(
                                Label::new(format!("Likes: {}", self.model.likes))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                            .when_some(self.model.size, |this, size| {
                                this.child(
                                    Label::new(format!("Size: {}", ModelRepository::<HuggingFaceProvider>::format_file_size(size)))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                )
                            })
                    )
                    .when(!self.model.tags.is_empty(), |this| {
                        this.child(
                            div()
                                .flex()
                                .gap_2()
                                .flex_wrap()
                                .children(
                                    self.model.tags.iter().map(|tag| {
                                        div()
                                            .px_2()
                                            .py_1()
                                            .bg(cx.theme().colors().surface_background)
                                            .rounded_md()
                                            .child(
                                                Label::new(tag.clone())
                                                    .size(LabelSize::Small)
                                            )
                                    })
                                )
                        )
                    })
                    .child(
                        div()
                            .mt_4()
                            .child(
                                Label::new("Available Files")
                                    .size(LabelSize::Large)
                            )
                            .child(
                                List::new()
                                    .children(
                                        self.model.files.iter().map(|file| {
                                            let _is_selected = self.selected_file.as_ref()
                                                .map(|f| f.name == file.name)
                                                .unwrap_or(false);
                                            
                                            ListItem::new(gpui::SharedString::from(file.name.clone()))
                                                .start_slot(Icon::new(IconName::File))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .justify_between()
                                                        .items_center()
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .gap_1()
                                                                .child(Label::new(file.name.clone()))
                                                                .child(
                                                                    Label::new(format!(
                                                                        "{} • {}",
                                                                        file.file_type.to_uppercase(),
                                                                        ModelRepository::<HuggingFaceProvider>::format_file_size(file.size)
                                                                    ))
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted)
                                                                )
                                                        )
                                                )
                                                .on_click({
                                                    let file = file.clone();
                                                    cx.listener(move |this, _, _window, cx| {
                                                        this.selected_file = Some(file.clone());
                                                        cx.notify();
                                                    })
                                                })
                                        })
                                    )
                            )
                    )
                    .child(
                        div()
                            .mt_4()
                            .flex()
                            .gap_2()
                            .justify_end()
                            .child(
                                Button::new("visit_repo", "Visit Repository")
                                    .style(ButtonStyle::Subtle)
                                    .on_click({
                                        let url = self.model.repository_url.clone();
                                        move |_, _window, cx| {
                                            cx.open_url(&url);
                                        }
                                    })
                            )
                            .child(
                                Button::new("download", "Download Selected")
                                    .style(ButtonStyle::Filled)
                                    .disabled(self.selected_file.is_none())
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.download_selected_file(cx);
                                        this.close_details(cx);
                                    }))
                            )
                    )
            )
    }
}

/// Type alias for Hugging Face model repository
pub type HuggingFaceModelRepository = ModelRepository<HuggingFaceProvider>; 
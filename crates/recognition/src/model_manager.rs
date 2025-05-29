use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelSize {
    Tiny,
    TinyEn,
    Base,
    BaseEn,
    Small,
    SmallEn,
    Medium,
    MediumEn,
    Large,
    LargeV1,
    LargeV2,
    LargeV3,
}

impl ModelSize {
    pub fn model_name(&self) -> &'static str {
        match self {
            ModelSize::Tiny => "tiny",
            ModelSize::TinyEn => "tiny.en",
            ModelSize::Base => "base",
            ModelSize::BaseEn => "base.en",
            ModelSize::Small => "small",
            ModelSize::SmallEn => "small.en",
            ModelSize::Medium => "medium",
            ModelSize::MediumEn => "medium.en",
            ModelSize::Large => "large",
            ModelSize::LargeV1 => "large-v1",
            ModelSize::LargeV2 => "large-v2",
            ModelSize::LargeV3 => "large-v3",
        }
    }

    pub fn expected_size_mb(&self) -> u64 {
        match self {
            ModelSize::Tiny => 39,
            ModelSize::TinyEn => 39,
            ModelSize::Base => 74,
            ModelSize::BaseEn => 74,
            ModelSize::Small => 244,
            ModelSize::SmallEn => 244,
            ModelSize::Medium => 769,
            ModelSize::MediumEn => 769,
            ModelSize::Large => 1550,
            ModelSize::LargeV1 => 1550,
            ModelSize::LargeV2 => 1550,
            ModelSize::LargeV3 => 1550,
        }
    }

    pub fn is_english_only(&self) -> bool {
        matches!(self, 
            ModelSize::TinyEn | ModelSize::BaseEn | 
            ModelSize::SmallEn | ModelSize::MediumEn
        )
    }
}

pub struct ModelManager {
    client: Client,
    cache_dir: PathBuf,
    model_urls: HashMap<ModelSize, String>,
}

impl ModelManager {
    pub fn new() -> Result<Self> {
        let cache_dir = Self::get_cache_dir()?;
        fs::create_dir_all(&cache_dir)?;

        let mut model_urls = HashMap::new();

        // Official OpenAI Whisper model URLs
        let base_url = "https://openaipublic.azureedge.net/main/whisper/models";
        
        // Add all model URLs
        model_urls.insert(ModelSize::Tiny, format!("{}/65147644a518d12f04e32d6f3b26facc3f8dd46e/tiny.pt", base_url));
        model_urls.insert(ModelSize::TinyEn, format!("{}/d3dd57d32accea0b295c96e26691aa14d8822fac/tiny.en.pt", base_url));
        model_urls.insert(ModelSize::Base, format!("{}/ed3a0b6b1c0edf879ad9b11b1af5a0e6ab5db9205f891f668f8b0e6c6326e34e/base.pt", base_url));
        model_urls.insert(ModelSize::BaseEn, format!("{}/60ed3bcb2e9d9b5bd9a80b3f17c89e2fc1c7b5e3/base.en.pt", base_url));
        model_urls.insert(ModelSize::Small, format!("{}/9ecf779972d90ba49c06d968637d720dd632c55bbf19a8717b5f295d4f83a47d/small.pt", base_url));
        model_urls.insert(ModelSize::SmallEn, format!("{}/f953ad0fd29cacd07d5a9eda5624af0f6bcf2258be67c92b79389873d91e0872/small.en.pt", base_url));
        model_urls.insert(ModelSize::Medium, format!("{}/345ae4da62f9b3d59415adc60127b97c714f32e89e936602e85993674d08dcb1/medium.pt", base_url));
        model_urls.insert(ModelSize::MediumEn, format!("{}/d7440d1dc186f76616787038b6f2daa50b7b8a29/medium.en.pt", base_url));
        model_urls.insert(ModelSize::Large, format!("{}/e4b87e7e0bf463eb8e6956e646f1e277e901512310def2c24bf0e11bd3c28e9a/large.pt", base_url));
        model_urls.insert(ModelSize::LargeV1, format!("{}/e4b87e7e0bf463eb8e6956e646f1e277e901512310def2c24bf0e11bd3c28e9a/large-v1.pt", base_url));
        model_urls.insert(ModelSize::LargeV2, format!("{}/81f7c96c852ee8fc832187b0132e569d6c3065a3252ed18e56effd0b6a73e524/large-v2.pt", base_url));
        model_urls.insert(ModelSize::LargeV3, format!("{}/e5b1a55b89c1367dacf97e3e19bfd829a01529dbfdeefa8caeb59b3f1b81dadb/large-v3.pt", base_url));

        Ok(Self {
            client: Client::new(),
            cache_dir,
            model_urls,
        })
    }

    pub async fn ensure_model(&self, size: ModelSize) -> Result<PathBuf> {
        let model_path = self.get_model_path(size);
        
        if model_path.exists() && self.verify_model(&model_path, size).await? {
            log::info!("Model {} already cached at {:?}", size.model_name(), model_path);
            return Ok(model_path);
        }

        log::info!("Downloading Whisper model: {}", size.model_name());
        self.download_model(size).await
    }

    async fn download_model(&self, size: ModelSize) -> Result<PathBuf> {
        let url = self.model_urls.get(&size)
            .ok_or_else(|| anyhow!("No URL configured for model size: {:?}", size))?;
        
        let model_path = self.get_model_path(size);
        let temp_path = model_path.with_extension("tmp");

        log::info!("Downloading model from: {}", url);
        
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!("Failed to download model: HTTP {}", response.status()));
        }

        let total_size = response.content_length().unwrap_or(0);
        log::info!("Model size: {} MB", total_size / 1024 / 1024);

        let mut file = tokio::fs::File::create(&temp_path).await?;
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        use futures::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            
            if total_size > 0 {
                let progress = (downloaded as f64 / total_size as f64) * 100.0;
                if downloaded % (1024 * 1024) == 0 { // Log every MB
                    log::info!("Download progress: {:.1}%", progress);
                }
            }
        }

        file.sync_all().await?;
        drop(file);

        // Move temp file to final location
        tokio::fs::rename(&temp_path, &model_path).await?;
        
        log::info!("Model downloaded successfully: {:?}", model_path);
        Ok(model_path)
    }

    async fn verify_model(&self, path: &Path, size: ModelSize) -> Result<bool> {
        if !path.exists() {
            return Ok(false);
        }

        // Check file size as a basic verification
        let metadata = tokio::fs::metadata(path).await?;
        let expected_size = size.expected_size_mb() * 1024 * 1024;
        let actual_size = metadata.len();
        
        // Allow 20% variance in file size (PyTorch models can vary)
        let size_diff = if actual_size > expected_size {
            actual_size - expected_size
        } else {
            expected_size - actual_size
        };
        
        let variance = (size_diff as f64 / expected_size as f64) * 100.0;
        
        if variance > 20.0 {
            log::warn!("Model file size variance too high: {:.1}%", variance);
            return Ok(false);
        }

        Ok(true)
    }

    fn get_model_path(&self, size: ModelSize) -> PathBuf {
        self.cache_dir.join(format!("ggml-{}.bin", size.model_name()))
    }

    fn get_cache_dir() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?;
        
        Ok(home.join(".cache").join("zed").join("whisper"))
    }

    pub fn list_cached_models(&self) -> Result<Vec<ModelSize>> {
        let mut cached = Vec::new();
        
        let all_sizes = [
            ModelSize::Tiny, ModelSize::TinyEn, ModelSize::Base, ModelSize::BaseEn,
            ModelSize::Small, ModelSize::SmallEn, ModelSize::Medium, ModelSize::MediumEn,
            ModelSize::Large, ModelSize::LargeV1, ModelSize::LargeV2, ModelSize::LargeV3,
        ];
        
        for &size in &all_sizes {
            let path = self.get_model_path(size);
            if path.exists() {
                cached.push(size);
            }
        }
        
        Ok(cached)
    }

    pub fn clear_cache(&self) -> Result<()> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)?;
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    pub fn get_recommended_model() -> ModelSize {
        // Base model is a good balance between speed and accuracy
        ModelSize::Base
    }

    pub fn get_fastest_model() -> ModelSize {
        ModelSize::Tiny
    }

    pub fn get_most_accurate_model() -> ModelSize {
        ModelSize::LargeV3
    }
} 
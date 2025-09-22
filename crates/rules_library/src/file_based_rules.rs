use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use collections::HashMap;
use gpui::{App, AppContext, EventEmitter, Global, SharedString, Task};
use parking_lot::RwLock;
use paths;
use prompt_store::{PromptId, PromptMetadata, UserPromptId};
use rope::Rope;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
};
use text::LineEnding;
use util::ResultExt;
use uuid::Uuid;
use fuzzy::StringMatchCandidate;

/// File-based storage for rules library using plaintext markdown files
#[derive(Clone)]
pub struct FileBasedRulesStore {
    rules_dir: PathBuf,
    metadata_cache: Arc<RwLock<MetadataCache>>,
}

pub struct PromptsUpdatedEvent;

impl EventEmitter<PromptsUpdatedEvent> for FileBasedRulesStore {}

#[derive(Default)]
struct MetadataCache {
    metadata: Vec<PromptMetadata>,
    metadata_by_id: HashMap<PromptId, PromptMetadata>,
}

impl MetadataCache {
    fn insert(&mut self, metadata: PromptMetadata) {
        self.metadata_by_id.insert(metadata.id, metadata.clone());
        if let Some(old_metadata) = self.metadata.iter_mut().find(|m| m.id == metadata.id) {
            *old_metadata = metadata;
        } else {
            self.metadata.push(metadata);
        }
        self.sort();
    }

    fn remove(&mut self, id: PromptId) {
        self.metadata.retain(|metadata| metadata.id != id);
        self.metadata_by_id.remove(&id);
    }

    fn sort(&mut self) {
        self.metadata.sort_unstable_by(|a, b| {
            a.title
                .cmp(&b.title)
                .then_with(|| b.saved_at.cmp(&a.saved_at))
        });
    }
}

/// Frontmatter metadata for rule files
#[derive(Debug, Serialize, Deserialize)]
struct RuleFrontmatter {
    /// Unique identifier for the rule
    id: String,
    /// Human-readable title
    title: Option<String>,
    /// Whether this rule should be used by default
    default: bool,
    /// When this rule was last saved
    saved_at: DateTime<Utc>,
}

impl FileBasedRulesStore {
    pub fn new(rules_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&rules_dir)?;
        
        let store = Self {
            rules_dir,
            metadata_cache: Arc::new(RwLock::new(MetadataCache::default())),
        };
        
        store.reload_metadata()?;
        Ok(store)
    }

    /// Reload metadata from all rule files in the directory
    fn reload_metadata(&self) -> Result<()> {
        let mut cache = MetadataCache::default();
        
        if !self.rules_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(&self.rules_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(metadata) = self.load_metadata_from_file(&path) {
                    cache.insert(metadata);
                }
            }
        }
        
        *self.metadata_cache.write() = cache;
        Ok(())
    }

    /// Load metadata from a rule file's frontmatter
    fn load_metadata_from_file(&self, path: &Path) -> Result<PromptMetadata> {
        let content = std::fs::read_to_string(path)?;
        let (frontmatter, _) = self.parse_frontmatter(&content)?;
        
        let uuid = Uuid::parse_str(&frontmatter.id)
            .with_context(|| format!("Invalid UUID in rule file: {}", path.display()))?;
        
        Ok(PromptMetadata {
            id: PromptId::User {
                uuid: UserPromptId(uuid),
            },
            title: frontmatter.title.map(SharedString::from),
            default: frontmatter.default,
            saved_at: frontmatter.saved_at,
        })
    }

    /// Parse frontmatter from markdown content
    fn parse_frontmatter(&self, content: &str) -> Result<(RuleFrontmatter, String)> {
        if !content.starts_with("---\n") {
            return Err(anyhow::anyhow!("Rule file must start with frontmatter"));
        }
        
        let end_marker = content.find("\n---\n")
            .ok_or_else(|| anyhow::anyhow!("Rule file must have closing frontmatter marker"))?;
        
        let frontmatter_json = &content[4..end_marker];
        let body = &content[end_marker + 5..];
        
        let frontmatter: RuleFrontmatter = serde_json::from_str(frontmatter_json)
            .with_context(|| "Failed to parse frontmatter")?;
        
        Ok((frontmatter, body.to_string()))
    }

    /// Generate frontmatter JSON from metadata
    fn generate_frontmatter(&self, metadata: &PromptMetadata) -> String {
        let frontmatter = RuleFrontmatter {
            id: match metadata.id {
                PromptId::User { uuid } => uuid.0.to_string(),
                PromptId::EditWorkflow => "edit-workflow".to_string(),
            },
            title: metadata.title.as_ref().map(|s| s.to_string()),
            default: metadata.default,
            saved_at: metadata.saved_at,
        };
        
        format!("---\n{}\n---\n", serde_json::to_string_pretty(&frontmatter).unwrap_or_default())
    }

    /// Get the file path for a rule ID
    fn get_rule_file_path(&self, id: PromptId) -> PathBuf {
        let filename = match id {
            PromptId::User { uuid } => format!("{}.md", uuid.0),
            PromptId::EditWorkflow => "edit-workflow.md".to_string(),
        };
        self.rules_dir.join(filename)
    }

    /// Load a rule's content from file
    pub fn load(&self, id: PromptId) -> Result<String> {
        let path = self.get_rule_file_path(id);
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read rule file: {}", path.display()))?;
        
        let (_, body) = self.parse_frontmatter(&content)?;
        Ok(body)
    }

    /// Save a rule to file
    pub fn save(&self, id: PromptId, title: Option<SharedString>, default: bool, body: &Rope) -> Result<()> {
        if id.is_built_in() {
            return Err(anyhow::anyhow!("Built-in rules cannot be saved"));
        }

        let metadata = PromptMetadata {
            id,
            title,
            default,
            saved_at: Utc::now(),
        };

        let frontmatter = self.generate_frontmatter(&metadata);
        let content = format!("{}{}", frontmatter, body.to_string());
        
        let path = self.get_rule_file_path(id);
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write rule file: {}", path.display()))?;

        // Update cache
        self.metadata_cache.write().insert(metadata);
        Ok(())
    }

    /// Delete a rule file
    pub fn delete(&self, id: PromptId) -> Result<()> {
        let path = self.get_rule_file_path(id);
        std::fs::remove_file(&path)
            .with_context(|| format!("Failed to delete rule file: {}", path.display()))?;

        // Update cache
        self.metadata_cache.write().remove(id);
        Ok(())
    }

    /// Get all rule metadata
    pub fn all_prompt_metadata(&self) -> Vec<PromptMetadata> {
        self.metadata_cache.read().metadata.clone()
    }

    /// Get default rule metadata
    pub fn default_prompt_metadata(&self) -> Vec<PromptMetadata> {
        self.metadata_cache
            .read()
            .metadata
            .iter()
            .filter(|metadata| metadata.default)
            .cloned()
            .collect()
    }

    /// Get metadata for a specific rule
    pub fn metadata(&self, id: PromptId) -> Option<PromptMetadata> {
        self.metadata_cache.read().metadata_by_id.get(&id).cloned()
    }

    /// Get the first rule
    pub fn first(&self) -> Option<PromptMetadata> {
        self.metadata_cache.read().metadata.first().cloned()
    }

    /// Find rule ID by title
    pub fn id_for_title(&self, title: &str) -> Option<PromptId> {
        let metadata_cache = self.metadata_cache.read();
        let metadata = metadata_cache
            .metadata
            .iter()
            .find(|metadata| metadata.title.as_ref().map(|title| &***title) == Some(title))?;
        Some(metadata.id)
    }

    /// Get the number of rules
    pub fn prompt_count(&self) -> usize {
        self.metadata_cache.read().metadata.len()
    }

    /// Reload all rules from disk (useful for filesystem watching)
    pub fn reload(&self) -> Result<()> {
        self.reload_metadata()
    }

    /// Migrate rules from the old LMDB format to the new plaintext format
    pub fn migrate_from_lmdb(&self) -> Result<()> {
        let lmdb_path = paths::prompts_dir().join("prompts-library-db.0.mdb");
        
        // Check if LMDB database exists
        if !lmdb_path.exists() {
            log::debug!("No LMDB database found at {:?}, skipping migration", lmdb_path);
            return Ok(());
        }

        log::info!("Starting migration from LMDB to plaintext format");

        // Try to open the LMDB database
        let db_env = match unsafe {
            heed::EnvOpenOptions::new()
                .map_size(1024 * 1024 * 1024) // 1GB
                .max_dbs(4)
                .open(&lmdb_path)
        } {
            Ok(env) => env,
            Err(e) => {
                log::warn!("Failed to open LMDB database for migration: {}", e);
                return Ok(());
            }
        };

        // Try to open the metadata and bodies databases
        let txn = match db_env.read_txn() {
            Ok(txn) => txn,
            Err(e) => {
                log::warn!("Failed to create read transaction for migration: {}", e);
                return Ok(());
            }
        };

        let mut migrated_count = 0;
        let mut skipped_count = 0;

        // Try to migrate from v2 databases first
        if let (Ok(Some(metadata_db)), Ok(Some(bodies_db))) = (
            db_env.open_database::<heed::types::SerdeJson<PromptId>, heed::types::SerdeJson<PromptMetadata>>(&txn, Some("metadata.v2")),
            db_env.open_database::<heed::types::SerdeJson<PromptId>, heed::types::Str>(&txn, Some("bodies.v2"))
        ) {
            log::info!("Found v2 LMDB database, migrating to plaintext format");
            
            for result in metadata_db.iter(&txn)? {
                let (prompt_id, metadata) = result?;
                
                // Skip built-in prompts
                if prompt_id.is_built_in() {
                    skipped_count += 1;
                    continue;
                }

                // Get the body content
                let body_content = match bodies_db.get(&txn, &prompt_id)? {
                    Some(content) => content.to_string(),
                    None => {
                        log::warn!("No body found for prompt {:?}, skipping", prompt_id);
                        skipped_count += 1;
                        continue;
                    }
                };

                // Check if this rule already exists in the new format
                let file_path = self.get_rule_file_path(prompt_id);
                if file_path.exists() {
                    log::debug!("Rule file already exists for {:?}, skipping", prompt_id);
                    skipped_count += 1;
                    continue;
                }

                // Create the new rule file
                let frontmatter = self.generate_frontmatter(&metadata);
                let content = format!("{}{}", frontmatter, body_content);
                
                if let Err(e) = std::fs::write(&file_path, content) {
                    log::error!("Failed to write rule file {:?}: {}", file_path, e);
                    skipped_count += 1;
                    continue;
                }

                log::debug!("Migrated rule {:?} to {:?}", prompt_id, file_path);
                migrated_count += 1;
            }
        }
        // Try to migrate from v1 databases
        else if let (Ok(Some(metadata_db)), Ok(Some(bodies_db))) = (
            db_env.open_database::<heed::types::SerdeBincode<Uuid>, heed::types::SerdeBincode<PromptMetadata>>(&txn, Some("metadata")),
            db_env.open_database::<heed::types::SerdeBincode<Uuid>, heed::types::SerdeBincode<String>>(&txn, Some("bodies"))
        ) {
            log::info!("Found v1 LMDB database, migrating to plaintext format");
            
            for result in metadata_db.iter(&txn)? {
                let (uuid, metadata) = result?;
                let prompt_id = PromptId::User { uuid: UserPromptId(uuid) };
                
                // Skip built-in prompts
                if prompt_id.is_built_in() {
                    skipped_count += 1;
                    continue;
                }

                // Get the body content
                let body_content = match bodies_db.get(&txn, &uuid)? {
                    Some(content) => content,
                    None => {
                        log::warn!("No body found for prompt {:?}, skipping", prompt_id);
                        skipped_count += 1;
                        continue;
                    }
                };

                // Check if this rule already exists in the new format
                let file_path = self.get_rule_file_path(prompt_id);
                if file_path.exists() {
                    log::debug!("Rule file already exists for {:?}, skipping", prompt_id);
                    skipped_count += 1;
                    continue;
                }

                // Create the new rule file
                let frontmatter = self.generate_frontmatter(&metadata);
                let content = format!("{}{}", frontmatter, body_content);
                
                if let Err(e) = std::fs::write(&file_path, content) {
                    log::error!("Failed to write rule file {:?}: {}", file_path, e);
                    skipped_count += 1;
                    continue;
                }

                log::debug!("Migrated rule {:?} to {:?}", prompt_id, file_path);
                migrated_count += 1;
            }
        } else {
            log::warn!("No compatible LMDB databases found for migration");
            return Ok(());
        }

        log::info!("Migration completed: {} rules migrated, {} skipped", migrated_count, skipped_count);

        // Reload metadata after migration
        self.reload_metadata()?;

        Ok(())
    }

    /// Search for rules matching a query
    pub fn search(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &App,
    ) -> Task<Vec<PromptMetadata>> {
        let cached_metadata = self.metadata_cache.read().metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
            let mut matches = if query.is_empty() {
                cached_metadata
            } else {
                let candidates = cached_metadata
                    .iter()
                    .enumerate()
                    .filter_map(|(ix, metadata)| {
                        Some(StringMatchCandidate::new(ix, metadata.title.as_ref()?))
                    })
                    .collect::<Vec<_>>();
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &cancellation_flag,
                    executor,
                )
                .await;
                matches
                    .into_iter()
                    .map(|mat| cached_metadata[mat.candidate_id].clone())
                    .collect()
            };
            matches.sort_by_key(|metadata| std::cmp::Reverse(metadata.default));
            matches
        })
    }

    /// Load a rule's content from file (async version for compatibility)
    pub fn load_async(&self, id: PromptId, cx: &App) -> Task<Result<String>> {
        let store = self.clone();
        cx.background_spawn(async move {
            let mut content = store.load(id)?;
            LineEnding::normalize(&mut content);
            Ok(content)
        })
    }

    /// Save a rule to file (async version for compatibility)
    pub fn save_async(
        &self,
        id: PromptId,
        title: Option<SharedString>,
        default: bool,
        body: Rope,
        executor: &gpui::BackgroundExecutor,
    ) -> Task<Result<()>> {
        let store = self.clone();
        executor.spawn(async move {
            store.save(id, title, default, &body)?;
            Ok(())
        })
    }

    /// Save only metadata for a rule (async version for compatibility)
    pub fn save_metadata_async(
        &self,
        id: PromptId,
        title: Option<SharedString>,
        default: bool,
        executor: &gpui::BackgroundExecutor,
    ) -> Task<Result<()>> {
        let store = self.clone();
        executor.spawn(async move {
            // Load the existing file content
            let path = store.get_rule_file_path(id);
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read rule file: {}", path.display()))?;
            
            // Parse the existing frontmatter and body
            let (_, body) = store.parse_frontmatter(&content)?;
            
            // Create new metadata with updated values
            let metadata = PromptMetadata {
                id,
                title,
                default,
                saved_at: Utc::now(),
            };
            
            // Generate new frontmatter and combine with existing body
            let frontmatter = store.generate_frontmatter(&metadata);
            let new_content = format!("{}{}", frontmatter, body);
            
            // Write the updated content
            std::fs::write(&path, new_content)
                .with_context(|| format!("Failed to write rule file: {}", path.display()))?;
            
            // Update cache
            store.metadata_cache.write().insert(metadata);
            Ok(())
        })
    }

    /// Delete a rule file (async version for compatibility)
    pub fn delete_async(&self, id: PromptId, executor: &gpui::BackgroundExecutor) -> Task<Result<()>> {
        let store = self.clone();
        executor.spawn(async move {
            store.delete(id)?;
            Ok(())
        })
    }
}


/// Global file-based rules store
pub struct GlobalFileBasedRulesStore(Arc<FileBasedRulesStore>);

impl Global for GlobalFileBasedRulesStore {}

impl GlobalFileBasedRulesStore {
    pub fn global(cx: &App) -> Arc<FileBasedRulesStore> {
        cx.global::<Self>().0.clone()
    }
}

/// Initialize the file-based rules store
pub fn init(cx: &mut App) {
    let rules_dir = paths::prompts_dir().join("rules");
    let store = FileBasedRulesStore::new(rules_dir).log_err();
    
    if let Some(store) = store {
        // Run migration from LMDB if needed
        if let Err(e) = store.migrate_from_lmdb() {
            log::error!("Failed to migrate rules from LMDB: {}", e);
        }
        
        cx.set_global(GlobalFileBasedRulesStore(Arc::new(store)));
    }
}
 
use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext, Entity};
use project::Project;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a snapshot of a file's content at a specific point in time
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileSnapshot {
    /// The file path
    pub path: PathBuf,
    /// The full content of the file at this checkpoint
    pub content: String,
    /// Language ID for the file
    pub language: Option<String>,
}

/// Type of checkpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointType {
    /// User manually created checkpoint or edited the file
    UserEdit,
    /// AI agent made edits through tools
    AgentEdit,
    /// Automatic checkpoint before a major operation
    Automatic,
}

/// A checkpoint capturing the state of multiple files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Unique ID for this checkpoint
    pub id: String,
    /// Type of checkpoint
    pub checkpoint_type: CheckpointType,
    /// Timestamp when checkpoint was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Message index in thread where this checkpoint was created
    pub message_index: usize,
    /// Snapshots of all files at this checkpoint
    pub file_snapshots: HashMap<PathBuf, FileSnapshot>,
    /// Optional description of what changed
    pub description: Option<String>,
}

impl Checkpoint {
    pub fn new(
        checkpoint_type: CheckpointType,
        message_index: usize,
        description: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            checkpoint_type,
            timestamp: chrono::Utc::now(),
            message_index,
            file_snapshots: HashMap::default(),
            description,
        }
    }

    /// Add a file snapshot to this checkpoint
    pub fn add_file(&mut self, snapshot: FileSnapshot) {
        self.file_snapshots.insert(snapshot.path.clone(), snapshot);
    }

    /// Get a file snapshot from this checkpoint
    pub fn get_file(&self, path: &PathBuf) -> Option<&FileSnapshot> {
        self.file_snapshots.get(path)
    }

    /// Get all file paths in this checkpoint
    pub fn file_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.file_snapshots.keys()
    }
}

/// Manages checkpoints for a conversation thread
pub struct CheckpointManager {
    /// All checkpoints in chronological order
    checkpoints: Vec<Checkpoint>,
    /// Current checkpoint index (for rollback/forward navigation)
    current_index: Option<usize>,
    /// Files that have been modified since the last checkpoint
    modified_files: HashMap<PathBuf, String>,
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            current_index: None,
            modified_files: HashMap::default(),
        }
    }

    /// Create a new checkpoint
    pub fn create_checkpoint(
        &mut self,
        checkpoint_type: CheckpointType,
        message_index: usize,
        description: Option<String>,
        _project: Entity<Project>,
        _cx: &mut impl AppContext,
    ) -> Result<usize> {
        let mut checkpoint = Checkpoint::new(checkpoint_type, message_index, description);

        // Capture current state of all modified files
        for (path, content) in &self.modified_files {
            checkpoint.add_file(FileSnapshot {
                path: path.clone(),
                content: content.clone(),
                language: None, // TODO: Get language from project
            });
        }

        // Add checkpoint to history
        self.checkpoints.push(checkpoint);
        let new_index = self.checkpoints.len() - 1;
        self.current_index = Some(new_index);

        // Clear modified files after checkpoint
        self.modified_files.clear();

        Ok(new_index)
    }

    /// Track that a file has been modified
    pub fn mark_file_modified(&mut self, path: PathBuf, content: String) {
        self.modified_files.insert(path, content);
    }

    /// Get all checkpoints
    pub fn checkpoints(&self) -> &[Checkpoint] {
        &self.checkpoints
    }

    /// Get current checkpoint index
    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    /// Get a specific checkpoint
    pub fn get_checkpoint(&self, index: usize) -> Option<&Checkpoint> {
        self.checkpoints.get(index)
    }

    /// Get the most recent checkpoint
    pub fn latest_checkpoint(&self) -> Option<&Checkpoint> {
        self.checkpoints.last()
    }

    /// Get checkpoint at or before a specific message index
    pub fn get_checkpoint_before_message(
        &self,
        message_index: usize,
    ) -> Option<(usize, &Checkpoint)> {
        self.checkpoints
            .iter()
            .enumerate()
            .rev()
            .find(|(_, cp)| cp.message_index <= message_index)
    }

    /// Rollback to a specific checkpoint
    pub fn rollback_to_checkpoint(
        &mut self,
        checkpoint_index: usize,
        _project: Entity<Project>,
        _cx: &mut impl AppContext,
    ) -> Result<Vec<PathBuf>> {
        let checkpoint = self
            .checkpoints
            .get(checkpoint_index)
            .ok_or_else(|| anyhow::anyhow!("Checkpoint not found"))?;

        let mut restored_files = Vec::new();

        // Restore all files from this checkpoint
        for (path, _snapshot) in &checkpoint.file_snapshots {
            // TODO: Actually restore the file content through project/worktree
            // This would require integration with Zed's buffer system
            restored_files.push(path.clone());
        }

        self.current_index = Some(checkpoint_index);

        Ok(restored_files)
    }

    /// Go forward to a later checkpoint
    pub fn forward_to_checkpoint(
        &mut self,
        checkpoint_index: usize,
        project: Entity<Project>,
        cx: &mut impl AppContext,
    ) -> Result<Vec<PathBuf>> {
        if checkpoint_index >= self.checkpoints.len() {
            return Err(anyhow::anyhow!("Checkpoint index out of range"));
        }

        self.rollback_to_checkpoint(checkpoint_index, project, cx)
    }

    /// Get files changed between two checkpoints
    pub fn get_changed_files(&self, from_index: usize, to_index: usize) -> Vec<PathBuf> {
        let mut changed_files = std::collections::HashSet::new();

        let start = from_index.min(to_index);
        let end = from_index.max(to_index);

        for checkpoint in &self.checkpoints[start..=end.min(self.checkpoints.len() - 1)] {
            for path in checkpoint.file_paths() {
                changed_files.insert(path.clone());
            }
        }

        changed_files.into_iter().collect()
    }

    /// Clear all checkpoints
    pub fn clear(&mut self) {
        self.checkpoints.clear();
        self.current_index = None;
        self.modified_files.clear();
    }

    /// Remove checkpoints after a specific index (when branching conversation)
    pub fn truncate_after(&mut self, message_index: usize) {
        self.checkpoints
            .retain(|cp| cp.message_index <= message_index);
        if let Some(current) = self.current_index {
            if current >= self.checkpoints.len() {
                self.current_index = if self.checkpoints.is_empty() {
                    None
                } else {
                    Some(self.checkpoints.len() - 1)
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_creation() {
        let checkpoint = Checkpoint::new(
            CheckpointType::UserEdit,
            0,
            Some("Initial state".to_string()),
        );

        assert_eq!(checkpoint.checkpoint_type, CheckpointType::UserEdit);
        assert_eq!(checkpoint.message_index, 0);
        assert_eq!(checkpoint.description, Some("Initial state".to_string()));
        assert!(checkpoint.file_snapshots.is_empty());
    }

    #[test]
    fn test_checkpoint_add_file() {
        let mut checkpoint = Checkpoint::new(CheckpointType::UserEdit, 0, None);

        let snapshot = FileSnapshot {
            path: PathBuf::from("/test/file.rs"),
            content: "fn main() {}".to_string(),
            language: Some("rust".to_string()),
        };

        checkpoint.add_file(snapshot.clone());

        assert_eq!(checkpoint.file_snapshots.len(), 1);
        assert_eq!(
            checkpoint.get_file(&PathBuf::from("/test/file.rs")),
            Some(&snapshot)
        );
    }

    #[test]
    fn test_checkpoint_manager() {
        let mut manager = CheckpointManager::new();

        manager.mark_file_modified(PathBuf::from("/test/file.rs"), "content1".to_string());

        assert_eq!(manager.checkpoints().len(), 0);
        assert!(manager.current_index().is_none());
    }

    #[test]
    fn test_get_checkpoint_before_message() {
        let mut manager = CheckpointManager::new();
        let mut cp1 = Checkpoint::new(CheckpointType::UserEdit, 0, None);
        cp1.file_snapshots.insert(
            PathBuf::from("/file1.rs"),
            FileSnapshot {
                path: PathBuf::from("/file1.rs"),
                content: "v1".to_string(),
                language: None,
            },
        );

        let mut cp2 = Checkpoint::new(CheckpointType::AgentEdit, 5, None);
        cp2.file_snapshots.insert(
            PathBuf::from("/file1.rs"),
            FileSnapshot {
                path: PathBuf::from("/file1.rs"),
                content: "v2".to_string(),
                language: None,
            },
        );

        manager.checkpoints.push(cp1);
        manager.checkpoints.push(cp2);

        let result = manager.get_checkpoint_before_message(3);
        assert!(result.is_some());
        let (index, checkpoint) = result.unwrap();
        assert_eq!(index, 0);
        assert_eq!(checkpoint.message_index, 0);

        let result = manager.get_checkpoint_before_message(7);
        assert!(result.is_some());
        let (index, checkpoint) = result.unwrap();
        assert_eq!(index, 1);
        assert_eq!(checkpoint.message_index, 5);
    }

    #[test]
    fn test_truncate_after() {
        let mut manager = CheckpointManager::new();
        manager
            .checkpoints
            .push(Checkpoint::new(CheckpointType::UserEdit, 0, None));
        manager
            .checkpoints
            .push(Checkpoint::new(CheckpointType::AgentEdit, 5, None));
        manager
            .checkpoints
            .push(Checkpoint::new(CheckpointType::UserEdit, 10, None));

        manager.current_index = Some(2);

        manager.truncate_after(5);

        assert_eq!(manager.checkpoints.len(), 2);
        assert_eq!(manager.current_index, Some(1));
    }
}

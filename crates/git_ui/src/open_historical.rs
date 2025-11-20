//! Functionality to open historical versions of files from git commits.
use anyhow::{Context as _, Result};
use git::repository::RepoPath;
use gpui::{App, AppContext as _, AsyncWindowContext, Entity};
use language::{
    Buffer, Capability, DiskState, File, LanguageRegistry, LineEnding, ReplicaId, Rope, TextBuffer,
};
use project::WorktreeId;
use std::{path::PathBuf, sync::Arc};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath};

/// A virtual file representing a historical version of a file at a specific commit.
pub struct GitHistoricalBlob {
    path: RepoPath,
    worktree_id: WorktreeId,
    #[allow(dead_code)]
    commit_sha: String,
    #[allow(dead_code)]
    version_label: VersionLabel,
}

/// Label to distinguish between different versions of historical files.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionLabel {
    /// The file state after the commit was applied (includes the commit's changes)
    Changed,
    /// The file state before the commit was applied (parent commit)
    Unchanged,
}

impl GitHistoricalBlob {
    pub fn new(
        path: RepoPath,
        worktree_id: WorktreeId,
        commit_sha: String,
        version_label: VersionLabel,
    ) -> Self {
        Self {
            path,
            worktree_id,
            commit_sha,
            version_label,
        }
    }

    #[allow(dead_code)]
    fn display_name(&self) -> String {
        let short_sha = &self.commit_sha[..7.min(self.commit_sha.len())];
        let version = match self.version_label {
            VersionLabel::Changed => "after",
            VersionLabel::Unchanged => "before",
        };
        format!(
            "{} @ {} ({})",
            self.path.file_name().unwrap_or("file"),
            short_sha,
            version
        )
    }
}

impl File for GitHistoricalBlob {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        // Historical files are always "new" (not on disk)
        DiskState::New
    }

    fn path_style(&self, _: &App) -> PathStyle {
        PathStyle::Posix
    }

    fn path(&self) -> &Arc<RelPath> {
        self.path.as_ref()
    }

    fn full_path(&self, _: &App) -> PathBuf {
        self.path.as_std_path().to_path_buf()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
        // Return a name that includes commit info
        self.path.file_name().unwrap_or("file")
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        self.worktree_id
    }

    fn to_proto(&self, _cx: &App) -> language::proto::File {
        unimplemented!("Historical blobs cannot be serialized")
    }

    fn is_private(&self) -> bool {
        false
    }
}

/// Prepares a historical file buffer for opening.
///
/// Returns the buffer entity that can then be opened by the caller in the appropriate
/// window context. This allows the caller to handle the actual UI work (pane splitting,
/// editor creation) in a window.spawn context where they have access to Window.
///
/// # Arguments
/// * `text` - The content of the file at the historical point
/// * `path` - The repository path of the file
/// * `worktree_id` - The worktree ID for the file
/// * `commit_sha` - The commit hash
/// * `version_label` - Whether this is the "changed" or "unchanged" version
/// * `language_registry` - Registry for language detection
/// * `cx` - The async app context
///
/// # Returns
/// A read-only buffer entity containing the historical file content.
pub async fn prepare_historical_buffer(
    text: String,
    path: RepoPath,
    worktree_id: WorktreeId,
    commit_sha: String,
    version_label: VersionLabel,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AsyncWindowContext,
) -> Result<Entity<Buffer>> {
    log::info!(
        "prepare_historical_buffer: Starting for path={:?}, commit={}, version={:?}, text_len={}",
        path.as_std_path(),
        &commit_sha[..7.min(commit_sha.len())],
        version_label,
        text.len()
    );

    // Create the virtual historical file
    let historical_file = Arc::new(GitHistoricalBlob::new(
        path.clone(),
        worktree_id,
        commit_sha.clone(),
        version_label,
    )) as Arc<dyn File>;

    log::debug!("prepare_historical_buffer: Created GitHistoricalBlob");

    // Build the buffer with the historical content
    let buffer = build_historical_buffer(text, historical_file, &language_registry, cx)
        .await
        .context("Failed to build historical buffer")?;

    log::debug!("prepare_historical_buffer: Buffer built successfully");

    // Set the buffer to read-only-unless-saved (allows editing after save)
    buffer
        .update(cx, |buffer, cx| {
            buffer.set_capability(Capability::ReadOnlyUnlessSaved, cx);
            log::debug!("prepare_historical_buffer: Set buffer capability to ReadOnlyUnlessSaved");
        })
        .context("Failed to set buffer capability")?;

    log::info!(
        "prepare_historical_buffer: Successfully created buffer for {:?}",
        path.file_name()
    );

    Ok(buffer)
}

/// Builds a buffer from historical file content.
async fn build_historical_buffer(
    mut text: String,
    file: Arc<dyn File>,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut AsyncWindowContext,
) -> Result<Entity<Buffer>> {
    log::debug!("build_historical_buffer: Starting, text_len={}", text.len());

    let line_ending = LineEnding::detect(&text);
    log::debug!(
        "build_historical_buffer: Detected line ending: {:?}",
        line_ending
    );

    LineEnding::normalize(&mut text);
    let text = Rope::from(text);

    let language = cx
        .update(|_window, cx| language_registry.language_for_file(&file, Some(&text), cx))
        .context("Failed to detect language for file")?;
    log::debug!(
        "build_historical_buffer: Language lookup result: {:?}",
        language.as_ref().map(|l| l.name())
    );

    let language = if let Some(language) = language {
        let lang_result = language_registry
            .load_language(&language)
            .await
            .ok()
            .and_then(|e| e.log_err());
        log::debug!(
            "build_historical_buffer: Language loaded: {}",
            lang_result.is_some()
        );
        lang_result
    } else {
        log::debug!("build_historical_buffer: No language detected for file");
        None
    };

    let buffer = cx
        .new(|cx| {
            log::debug!("build_historical_buffer: Creating new buffer entity");
            let buffer = TextBuffer::new_normalized(
                ReplicaId::LOCAL,
                cx.entity_id().as_non_zero_u64().into(),
                line_ending,
                text,
            );
            let mut buffer = Buffer::build(buffer, Some(file), Capability::ReadOnlyUnlessSaved);
            buffer.set_language(language, cx);
            log::debug!("build_historical_buffer: Buffer entity created");
            buffer
        })
        .context("Failed to create buffer entity")?;

    log::debug!("build_historical_buffer: Successfully built buffer");
    Ok(buffer)
}

/// Prepares the "changed" version of a file from a commit (file state after commit).
///
/// This extracts the text from a buffer that's already in memory in the commit view
/// and returns a new read-only buffer. The caller should open this buffer in an editor.
pub async fn prepare_changed_buffer_from_commit(
    buffer: Entity<Buffer>,
    path: RepoPath,
    worktree_id: WorktreeId,
    commit_sha: String,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AsyncWindowContext,
) -> Result<Entity<Buffer>> {
    log::info!(
        "prepare_changed_buffer_from_commit: Extracting changed version for path={:?}, commit={}",
        path.as_std_path(),
        &commit_sha[..7.min(commit_sha.len())]
    );

    let text = buffer
        .update(cx, |buffer, _| {
            let text = buffer.text();
            log::debug!(
                "prepare_changed_buffer_from_commit: Extracted text, len={}",
                text.len()
            );
            text
        })
        .context("Failed to extract text from buffer")?;

    prepare_historical_buffer(
        text,
        path,
        worktree_id,
        commit_sha,
        VersionLabel::Changed,
        language_registry,
        cx,
    )
    .await
}

/// Prepares the "unchanged" version of a file from a commit (file state before commit).
///
/// This extracts the base text from a BufferDiff that's already in memory in the commit view
/// and returns a new read-only buffer. The caller should open this buffer in an editor.
pub async fn prepare_unchanged_buffer_from_commit(
    buffer_diff: Entity<buffer_diff::BufferDiff>,
    path: RepoPath,
    worktree_id: WorktreeId,
    commit_sha: String,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut AsyncWindowContext,
) -> Result<Entity<Buffer>> {
    log::info!(
        "prepare_unchanged_buffer_from_commit: Extracting unchanged version for path={:?}, commit={}",
        path.as_std_path(),
        &commit_sha[..7.min(commit_sha.len())]
    );

    let text = buffer_diff
        .update(cx, |diff, _| {
            let text = diff.base_text().text();
            log::debug!(
                "prepare_unchanged_buffer_from_commit: Extracted base text, len={}",
                text.len()
            );
            text
        })
        .context("Failed to extract base text from buffer diff")?;

    prepare_historical_buffer(
        text,
        path,
        worktree_id,
        commit_sha,
        VersionLabel::Unchanged,
        language_registry,
        cx,
    )
    .await
}

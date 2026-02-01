use acp_thread::{AgentSessionInfo, AgentSessionList, AgentSessionListRequest, AgentSessionListResponse};
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use collections::HashMap;
use gpui::{App, Task};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Information about a Claude Code session from the CLI storage
#[derive(Debug, Clone)]
pub struct ClaudeCodeSessionInfo {
    pub session_id: String,
    pub title: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub git_branch: Option<String>,
    pub message_count: usize,
    pub full_path: PathBuf,
}

/// The sessions-index.json format
#[derive(Debug, Deserialize)]
struct SessionsIndex {
    version: u32,
    #[serde(rename = "originalPath")]
    original_path: String,
    entries: Vec<SessionEntry>,
}

#[derive(Debug, Deserialize)]
struct SessionEntry {
    #[serde(rename = "sessionId")]
    session_id: String,
    #[serde(rename = "fullPath")]
    full_path: String,
    #[serde(rename = "firstPrompt")]
    first_prompt: Option<String>,
    #[serde(rename = "messageCount")]
    message_count: Option<usize>,
    created: Option<String>,
    modified: Option<String>,
    #[serde(rename = "gitBranch")]
    git_branch: Option<String>,
}

/// Custom session names stored in session-names.json
#[derive(Debug, Default, Serialize, Deserialize)]
struct SessionNames {
    names: HashMap<String, String>,
}

/// Reads Claude Code session data from ~/.claude/projects/<project>/
pub struct ClaudeCodeSessionIndex {
    project_path: PathBuf,
    sessions_dir: PathBuf,
}

impl ClaudeCodeSessionIndex {
    /// Create a session index for the given project path.
    /// Returns None if no Claude Code sessions exist for this project.
    pub fn for_project(project_path: &Path) -> Option<Self> {
        let folder_name = Self::project_path_to_folder_name(project_path);
        let sessions_dir = dirs::home_dir()?.join(".claude/projects").join(&folder_name);

        if sessions_dir.exists() && sessions_dir.join("sessions-index.json").exists() {
            Some(Self {
                project_path: project_path.to_path_buf(),
                sessions_dir,
            })
        } else {
            None
        }
    }

    /// Convert a project path to Claude Code's folder naming convention.
    /// `/Volumes/Code/GitHub/zed` -> `-Volumes-Code-GitHub-zed`
    fn project_path_to_folder_name(path: &Path) -> String {
        path.to_string_lossy().replace('/', "-")
    }

    pub fn sessions_dir(&self) -> &Path {
        &self.sessions_dir
    }

    pub fn project_path(&self) -> &Path {
        &self.project_path
    }

    /// List all sessions from the sessions-index.json file
    pub fn list_sessions(&self) -> Result<Vec<ClaudeCodeSessionInfo>> {
        let index_path = self.sessions_dir.join("sessions-index.json");
        let file = File::open(&index_path)
            .with_context(|| format!("Failed to open {}", index_path.display()))?;
        let reader = BufReader::new(file);
        let index: SessionsIndex = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse {}", index_path.display()))?;

        let mut sessions = Vec::new();
        for entry in index.entries {
            let created = entry
                .created
                .as_ref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);

            let modified = entry
                .modified
                .as_ref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(created);

            // Use first 120 chars of first prompt as title, or session ID if not available
            let title = entry
                .first_prompt
                .as_ref()
                .map(|p| {
                    // Clean up the prompt: remove leading/trailing whitespace and normalize newlines
                    let cleaned = p.trim().replace('\n', " ");
                    let trimmed: String = cleaned.chars().take(120).collect();
                    if cleaned.len() > 120 {
                        format!("{}...", trimmed)
                    } else {
                        trimmed
                    }
                })
                .unwrap_or_else(|| entry.session_id.clone());

            sessions.push(ClaudeCodeSessionInfo {
                session_id: entry.session_id,
                title,
                created,
                modified,
                git_branch: entry.git_branch,
                message_count: entry.message_count.unwrap_or(0),
                full_path: PathBuf::from(entry.full_path),
            });
        }

        // Sort by modified time, most recent first
        sessions.sort_by(|a, b| b.modified.cmp(&a.modified));

        Ok(sessions)
    }

    /// Delete a session by removing its JSONL file and updating the index
    pub fn delete_session(&self, session_id: &str) -> Result<()> {
        // Remove the session file
        let session_file = self.sessions_dir.join(format!("{}.jsonl", session_id));
        if session_file.exists() {
            std::fs::remove_file(&session_file)
                .with_context(|| format!("Failed to delete {}", session_file.display()))?;
        }

        // Also remove any session directory (for artifacts like screenshots)
        let session_dir = self.sessions_dir.join(session_id);
        if session_dir.exists() && session_dir.is_dir() {
            std::fs::remove_dir_all(&session_dir)
                .with_context(|| format!("Failed to delete {}", session_dir.display()))?;
        }

        // Update the sessions-index.json
        self.update_index_after_delete(session_id)?;

        Ok(())
    }

    fn update_index_after_delete(&self, deleted_session_id: &str) -> Result<()> {
        let index_path = self.sessions_dir.join("sessions-index.json");
        let file = File::open(&index_path)?;
        let reader = BufReader::new(file);
        let mut index: serde_json::Value = serde_json::from_reader(reader)?;

        if let Some(entries) = index.get_mut("entries").and_then(|e| e.as_array_mut()) {
            entries.retain(|entry| {
                entry
                    .get("sessionId")
                    .and_then(|id| id.as_str())
                    .map(|id| id != deleted_session_id)
                    .unwrap_or(true)
            });
        }

        let file = File::create(&index_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &index)?;

        Ok(())
    }
}

/// Custom session names manager
pub struct SessionCustomNames {
    names: HashMap<String, String>,
    path: PathBuf,
}

impl SessionCustomNames {
    pub fn load(sessions_dir: &Path) -> Self {
        let path = sessions_dir.join("session-names.json");
        let names = if path.exists() {
            File::open(&path)
                .ok()
                .and_then(|file| serde_json::from_reader(BufReader::new(file)).ok())
                .map(|sn: SessionNames| sn.names)
                .unwrap_or_default()
        } else {
            HashMap::default()
        };

        Self { names, path }
    }

    pub fn get(&self, session_id: &str) -> Option<&String> {
        self.names.get(session_id)
    }

    pub fn set(&mut self, session_id: String, name: String) {
        self.names.insert(session_id, name);
    }

    pub fn remove(&mut self, session_id: &str) {
        self.names.remove(session_id);
    }

    pub fn save(&self) -> Result<()> {
        let session_names = SessionNames {
            names: self.names.clone(),
        };
        let file = File::create(&self.path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &session_names)?;
        Ok(())
    }
}

/// Implements AgentSessionList for Claude Code CLI sessions
pub struct ClaudeCodeSessionList {
    index: ClaudeCodeSessionIndex,
    custom_names: RefCell<SessionCustomNames>,
}

impl ClaudeCodeSessionList {
    pub fn new(index: ClaudeCodeSessionIndex) -> Self {
        let custom_names = SessionCustomNames::load(index.sessions_dir());
        Self {
            index,
            custom_names: RefCell::new(custom_names),
        }
    }

    pub fn project_path(&self) -> &Path {
        self.index.project_path()
    }

    pub fn sessions_dir(&self) -> &Path {
        self.index.sessions_dir()
    }

    /// Rename a session (stores custom name in session-names.json)
    pub fn rename_session(&self, session_id: &str, name: Option<String>) -> Result<()> {
        let mut custom_names = self.custom_names.borrow_mut();
        if let Some(name) = name {
            if !name.is_empty() {
                custom_names.set(session_id.to_string(), name);
            } else {
                custom_names.remove(session_id);
            }
        } else {
            custom_names.remove(session_id);
        }
        custom_names.save()
    }

    /// Get the session file path for loading messages
    pub fn session_file_path(&self, session_id: &str) -> PathBuf {
        self.index.sessions_dir.join(format!("{}.jsonl", session_id))
    }

    /// Synchronously list sessions (for use in pickers/modals)
    pub fn list_sessions_sync(&self) -> Vec<AgentSessionInfo> {
        let sessions = match self.index.list_sessions() {
            Ok(sessions) => sessions,
            Err(e) => {
                log::error!("Failed to list Claude Code sessions: {:?}", e);
                return Vec::new();
            }
        };

        let custom_names = self.custom_names.borrow();
        sessions
            .into_iter()
            .map(|s| {
                let title = custom_names
                    .get(&s.session_id)
                    .cloned()
                    .unwrap_or(s.title);

                AgentSessionInfo {
                    session_id: acp::SessionId::new(s.session_id.clone()),
                    cwd: Some(self.index.project_path.clone()),
                    title: Some(title.into()),
                    updated_at: Some(s.modified),
                    meta: None,
                }
            })
            .collect()
    }
}

impl AgentSessionList for ClaudeCodeSessionList {
    fn list_sessions(
        &self,
        _request: AgentSessionListRequest,
        _cx: &mut App,
    ) -> Task<Result<AgentSessionListResponse>> {
        log::info!("ClaudeCodeSessionList::list_sessions called");
        let sessions = match self.index.list_sessions() {
            Ok(sessions) => {
                log::info!("Found {} Claude Code sessions", sessions.len());
                sessions
            }
            Err(e) => {
                log::error!("Error loading Claude Code sessions: {:?}", e);
                return Task::ready(Err(e));
            }
        };

        let custom_names = self.custom_names.borrow();
        let infos: Vec<AgentSessionInfo> = sessions
            .into_iter()
            .map(|s| {
                let title = custom_names
                    .get(&s.session_id)
                    .cloned()
                    .unwrap_or(s.title);

                AgentSessionInfo {
                    session_id: acp::SessionId::new(s.session_id.clone()),
                    cwd: Some(self.index.project_path.clone()),
                    title: Some(title.into()),
                    updated_at: Some(s.modified),
                    meta: None,
                }
            })
            .collect();

        log::info!("Returning {} session infos", infos.len());
        Task::ready(Ok(AgentSessionListResponse::new(infos)))
    }

    fn delete_session(&self, session_id: &acp::SessionId, _cx: &mut App) -> Task<Result<()>> {
        let session_id_str = session_id.to_string();

        // Remove custom name if any
        if let Err(e) = self.rename_session(&session_id_str, None) {
            log::warn!("Failed to remove custom name for session {}: {}", session_id_str, e);
        }

        // Delete the session files
        match self.index.delete_session(&session_id_str) {
            Ok(()) => Task::ready(Ok(())),
            Err(e) => Task::ready(Err(e)),
        }
    }

    fn delete_sessions(&self, _cx: &mut App) -> Task<Result<()>> {
        // Delete all sessions - iterate through and delete each
        let sessions = match self.index.list_sessions() {
            Ok(s) => s,
            Err(e) => return Task::ready(Err(e)),
        };

        for session in sessions {
            if let Err(e) = self.index.delete_session(&session.session_id) {
                log::warn!("Failed to delete session {}: {}", session.session_id, e);
            }
        }

        // Clear all custom names
        {
            let mut custom_names = self.custom_names.borrow_mut();
            custom_names.names.clear();
            if let Err(e) = custom_names.save() {
                log::warn!("Failed to save custom names after clearing: {}", e);
            }
        }

        Task::ready(Ok(()))
    }

    fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
        // We don't have a watch mechanism for file system changes yet
        // Could implement using notify crate in the future
        None
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

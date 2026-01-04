use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use fs::Fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Record describing an approval for a command signature.
///
/// - `allowed_always`: if true this command is permanently approved (no prompt)
/// - `allowed_once_count`: number of one-time approvals remaining
/// - `created_at` / `last_used_at`: timestamps for auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub signature: String,
    pub allowed_always: bool,
    pub allowed_once_count: u32,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    /// Optional human-readable note (e.g., "allowed on project X")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Approval {
    fn new(
        signature: String,
        allowed_always: bool,
        allowed_once_count: u32,
        note: Option<String>,
    ) -> Self {
        Self {
            signature,
            allowed_always,
            allowed_once_count,
            created_at: Utc::now(),
            last_used_at: None,
            note,
        }
    }
}

/// Persisted approvals store.
///
/// This module manages approvals for executing commands from command rules.
/// Approvals are stored in a JSON file inside the rules directory so they can
/// be tracked with Git or synced by the user if desired.
///
/// API notes:
/// - `is_approved` will consume one-time approvals (decrement `allowed_once_count`)
///   and persist the change.
/// - `approve_once` increases the `allowed_once_count` by one and persists.
/// - `approve_always` marks the signature as permanently approved.
/// - Signatures are plain canonical strings (see `signature_from_parts`).
pub struct CommandApprovals {
    approvals: RwLock<HashMap<String, Approval>>,
    file_path: PathBuf,
    fs: Arc<dyn Fs>,
}

impl CommandApprovals {
    /// Create a new approvals manager that stores data under `rules_dir`.
    ///
    /// The on-disk path will be `rules_dir/_command_approvals.json`.
    pub fn new(rules_dir: PathBuf, fs: Arc<dyn Fs>) -> Self {
        let file_path = rules_dir.join("_command_approvals.json");
        Self {
            approvals: RwLock::new(HashMap::new()),
            file_path,
            fs,
        }
    }

    /// Load approvals from disk into memory. If the file does not exist this will be a no-op.
    pub async fn init(&self) -> Result<()> {
        if self
            .fs
            .metadata(&self.file_path)
            .await
            .ok()
            .flatten()
            .is_some()
        {
            let data = self
                .fs
                .load(&self.file_path)
                .await
                .context("loading approvals file")?;
            let map: HashMap<String, Approval> =
                serde_json::from_str(&data).context("parsing approvals JSON")?;
            let mut guard = self.approvals.write().unwrap();
            *guard = map;
        }
        Ok(())
    }

    /// Persist current approvals to disk atomically.
    async fn persist(&self) -> Result<()> {
        let json = {
            let guard = self.approvals.read().unwrap();
            serde_json::to_string_pretty(&*guard).context("serializing approvals")?
        };
        // Ensure parent dir exists (best-effort)
        if let Some(parent) = self.file_path.parent() {
            let _ = self.fs.create_dir(parent).await;
        }
        self.fs
            .atomic_write(self.file_path.clone(), json)
            .await
            .context("writing approvals file")?;
        Ok(())
    }

    /// Canonical signature for a command + args.
    ///
    /// This is intentionally simple and stable: we join command and args
    /// with the ASCII unit separator so that keys are reversible for debugging.
    pub fn signature_from_parts(cmd: &str, args: &[String]) -> String {
        // Use a separator that is unlikely to appear in normal shells.
        // We deliberately avoid hashing to keep the store human-readable.
        let sep = '\x1f';
        let mut s =
            String::with_capacity(cmd.len() + 1 + args.iter().map(|a| a.len() + 1).sum::<usize>());
        s.push_str(cmd);
        for arg in args {
            s.push(sep);
            s.push_str(arg);
        }
        s
    }

    /// Check whether the command signature is approved.
    ///
    /// If a one-time approval exists (allowed_once_count > 0) it will be consumed
    /// (decremented) and persisted.
    pub async fn is_approved(&self, signature: &str) -> Result<bool> {
        let should_persist = {
            let mut guard = self.approvals.write().unwrap();
            if let Some(record) = guard.get_mut(signature) {
                // If permanently allowed, return true
                if record.allowed_always {
                    record.last_used_at = Some(Utc::now());
                    true
                } else if record.allowed_once_count > 0 {
                    // If one-time allowances exist, consume one and persist
                    record.allowed_once_count = record.allowed_once_count.saturating_sub(1);
                    record.last_used_at = Some(Utc::now());
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };

        if should_persist {
            self.persist().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Approve the signature for a single one-time run.
    pub async fn approve_once(&self, signature: &str, note: Option<String>) -> Result<()> {
        {
            let mut guard = self.approvals.write().unwrap();
            let record = guard
                .entry(signature.to_string())
                .or_insert_with(|| Approval::new(signature.to_string(), false, 0, note.clone()));
            record.allowed_once_count = record.allowed_once_count.saturating_add(1);
            if note.is_some() {
                record.note = note;
            }
            record.last_used_at = Some(Utc::now());
        }
        self.persist().await?;
        Ok(())
    }

    /// Approve the signature permanently.
    pub async fn approve_always(&self, signature: &str, note: Option<String>) -> Result<()> {
        {
            let mut guard = self.approvals.write().unwrap();
            let record = guard
                .entry(signature.to_string())
                .or_insert_with(|| Approval::new(signature.to_string(), true, 0, note.clone()));
            record.allowed_always = true;
            if note.is_some() {
                record.note = note;
            }
            record.last_used_at = Some(Utc::now());
        }
        self.persist().await?;
        Ok(())
    }

    /// Revoke approval (remove entry).
    pub async fn revoke(&self, signature: &str) -> Result<()> {
        {
            let mut guard = self.approvals.write().unwrap();
            guard.remove(signature);
        }
        self.persist().await?;
        Ok(())
    }

    /// List all approvals (snapshot).
    pub fn list(&self) -> Vec<Approval> {
        let guard = self.approvals.read().unwrap();
        guard.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_is_stable() {
        let sig1 = CommandApprovals::signature_from_parts(
            "git",
            &["status".to_string(), "--short".to_string()],
        );
        let sig2 = CommandApprovals::signature_from_parts(
            "git",
            &["status".to_string(), "--short".to_string()],
        );
        assert_eq!(sig1, sig2);
        assert!(sig1.starts_with("git"));
    }
}

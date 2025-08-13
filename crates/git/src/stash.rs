use crate::Oid;
use anyhow::Result;
use std::{str::FromStr, sync::Arc};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StashEntry {
    pub index: usize,
    pub oid: Oid,
    pub message: String,
    pub branch: Option<String>,
    pub timestamp: i64,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct GitStash {
    pub entries: Arc<[StashEntry]>,
}

impl GitStash {
    pub fn apply(&mut self, other: GitStash) {
        self.entries = other.entries;
    }
}

impl FromStr for GitStash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // git stash list --pretty=%gd:%H:%s
        let entries = s
            .split('\n')
            .filter_map(|entry| {
                let mut parts = entry.splitn(4, '\0');
                let raw_idx = parts.next().and_then(|i| {
                    let trimmed = i.trim();
                    if trimmed.starts_with("stash@{") && trimmed.ends_with('}') {
                        trimmed
                            .strip_prefix("stash@{")
                            .and_then(|s| s.strip_suffix('}'))
                    } else {
                        None
                    }
                });
                let raw_oid = parts.next();
                let raw_date = parts.next().and_then(|d| d.parse().ok());
                let message = parts.next();

                if let (Some(raw_idx), Some(raw_oid), Some(raw_date), Some(message)) =
                    (raw_idx, raw_oid, raw_date, message)
                {
                    let (branch, message) = parse_stash_entry(message);
                    let index = raw_idx.parse::<usize>().ok()?;
                    let oid = Oid::from_str(raw_oid).ok()?;
                    let entry = StashEntry {
                        index,
                        oid,
                        message: message.to_string(),
                        branch: branch.map(Into::into),
                        timestamp: raw_date,
                    };
                    return Some(entry);
                }
                None
            })
            .collect::<Arc<[StashEntry]>>();
        Ok(Self {
            entries: entries.clone(),
        })
    }
}

fn parse_stash_entry(input: &str) -> (Option<&str>, &str) {
    // Try to match "WIP on <branch>: <message>" pattern
    if let Some(stripped) = input.strip_prefix("WIP on ") {
        if let Some(colon_pos) = stripped.find(": ") {
            let branch = &stripped[..colon_pos];
            let message = &stripped[colon_pos + 2..];
            return (Some(branch), message);
        }
    }

    // Try to match "On <branch>: <message>" pattern
    if let Some(stripped) = input.strip_prefix("On ") {
        if let Some(colon_pos) = stripped.find(": ") {
            let branch = &stripped[..colon_pos];
            let message = &stripped[colon_pos + 2..];
            return (Some(branch), message);
        }
    }

    // Edge case: format doesn't match, return None for branch and full string as message
    (None, input)
}

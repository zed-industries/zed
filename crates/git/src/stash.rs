use crate::Oid;
use anyhow::Result;
use std::{str::FromStr, sync::Arc};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StashEntry {
    pub index: usize,
    pub oid: Oid,
    pub message: String,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct GitStash {
    pub entries: Arc<[StashEntry]>,
}

impl FromStr for GitStash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // git stash list --pretty=%gd:%H:%s
        let entries = s
            .split('\n')
            .filter_map(|entry| {
                let mut parts = entry.splitn(3, ':');
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
                let message = parts.next();

                if let (Some(raw_idx), Some(raw_oid), Some(message)) = (raw_idx, raw_oid, message) {
                    let index = raw_idx.parse::<usize>().ok()?;
                    let oid = Oid::from_str(raw_oid).ok()?;
                    let entry = StashEntry {
                        index,
                        oid,
                        message: message.to_string(),
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

use anyhow::Result;
use serde::Deserialize;

/// A single blocking function pattern from the blocklist.
#[derive(Debug, Clone, Deserialize)]
pub struct BlockingEntry {
    /// Path pattern like `std::fs::*` or `std::thread::sleep`.
    pub path: String,
    /// Human-readable category (e.g., "filesystem", "thread").
    pub category: String,
    /// Help text suggesting an alternative.
    pub help: String,
    /// Optional tier: if "pedantic", only active with --pedantic.
    #[serde(default)]
    pub tier: Option<String>,
}

/// An escape hatch entry — closures passed to these functions are not flagged.
#[derive(Debug, Clone, Deserialize)]
pub struct SafeWrapperEntry {
    /// Path pattern like `smol::unblock`.
    pub path: String,
    /// Explanation of why this is safe.
    pub note: String,
}

#[derive(Debug, Deserialize)]
struct BlocklistFile {
    #[serde(default)]
    blocking: Vec<BlockingEntry>,
    #[serde(default)]
    safe_wrapper: Vec<SafeWrapperEntry>,
}

/// The loaded blocklist database.
#[derive(Debug)]
pub struct Blocklist {
    pub entries: Vec<BlockingEntry>,
    pub safe_wrappers: Vec<SafeWrapperEntry>,
}

const BLOCKLIST_TOML: &str = include_str!("../blocklist.toml");

impl Blocklist {
    /// Load the compiled-in blocklist, optionally including pedantic entries.
    pub fn load(pedantic: bool) -> Result<Self> {
        let file: BlocklistFile = toml::from_str(BLOCKLIST_TOML)?;
        let entries = if pedantic {
            file.blocking
        } else {
            file.blocking
                .into_iter()
                .filter(|entry| entry.tier.as_deref() != Some("pedantic"))
                .collect()
        };
        Ok(Blocklist {
            entries,
            safe_wrappers: file.safe_wrapper,
        })
    }

    /// Check if a call path matches any entry in the blocklist.
    /// Returns the matching entry if found.
    pub fn matches(&self, call_path: &str) -> Option<&BlockingEntry> {
        self.entries.iter().find(|entry| {
            if entry.path.ends_with("::*") {
                let prefix = &entry.path[..entry.path.len() - 1];
                call_path.starts_with(prefix)
            } else {
                call_path == entry.path
            }
        })
    }

    /// Check if a call path matches a safe wrapper (escape hatch).
    pub fn is_safe_wrapper(&self, call_path: &str) -> bool {
        self.safe_wrappers
            .iter()
            .any(|wrapper| call_path == wrapper.path)
    }
}

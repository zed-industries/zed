//! The workspace's OMP sessions, read from OMP's own store. OMP's RPC has
//! no command that lists sessions, so the session manager reads the header
//! OMP writes at the top of each session file: a `title` record (rewritten
//! in place on a rename), then the `session` record.

use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// One session OMP keeps in the directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEntry {
    pub id: String,
    pub path: PathBuf,
    pub title: Option<String>,
    /// When OMP started it (its header's ISO 8601 timestamp).
    pub started: String,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Header {
    Title {
        title: Option<String>,
    },
    Session {
        id: String,
        title: Option<String>,
        #[serde(default)]
        timestamp: String,
    },
}

/// Every session file in `dir`, newest first. A file whose header does not
/// parse is not a session and is skipped.
pub fn list(dir: &Path) -> std::io::Result<Vec<SessionEntry>> {
    let mut entries: Vec<SessionEntry> = std::fs::read_dir(dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
        .filter_map(|p| read(&p))
        .collect();
    entries.sort_by(|a, b| b.started.cmp(&a.started).then(b.id.cmp(&a.id)));
    Ok(entries)
}

fn read(path: &Path) -> Option<SessionEntry> {
    let file = std::fs::File::open(path).ok()?;
    let mut renamed = None;
    for line in BufReader::new(file).lines().take(2) {
        match serde_json::from_str::<Header>(&line.ok()?).ok()? {
            Header::Title { title } => renamed = title,
            Header::Session {
                id,
                title,
                timestamp,
            } => {
                return Some(SessionEntry {
                    id,
                    path: path.to_path_buf(),
                    title: renamed.or(title).filter(|t| !t.is_empty()),
                    started: timestamp,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const OLDER: &str = concat!(
        r#"{"type":"title","v":1,"title":"release checklist v2","source":"user","updatedAt":"2026-10-14T10:16:28.390Z","pad":"  "}"#,
        "\n",
        r#"{"type":"session","version":3,"id":"01a139e5-ba79-729d-95f7-e2f592ebbd2e","title":"release checklist","titleSource":"user","timestamp":"2026-10-14T10:11:59.225Z","cwd":"/w"}"#,
        "\n",
    );
    const NEWER: &str = concat!(
        r#"{"type":"session","version":3,"id":"01a139e5-e3fa-7590-a571-57102af41fd6","timestamp":"2026-10-14T10:12:09.850Z","cwd":"/w"}"#,
        "\n",
    );

    #[test]
    fn lists_sessions_newest_first_with_the_renamed_title() {
        let dir = std::env::temp_dir().join(format!("cedian-sessions-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.jsonl"), OLDER).unwrap();
        std::fs::write(dir.join("b.jsonl"), NEWER).unwrap();
        std::fs::write(dir.join("notes.jsonl"), "{\"type\":\"other\"}\n").unwrap();
        std::fs::write(dir.join(".a.jsonl.lock.os"), "").unwrap();

        let listed = list(&dir).unwrap();
        let shown: Vec<_> = listed
            .iter()
            .map(|e| (e.id.as_str(), e.title.as_deref()))
            .collect();
        assert_eq!(
            shown,
            vec![
                ("01a139e5-e3fa-7590-a571-57102af41fd6", None),
                (
                    "01a139e5-ba79-729d-95f7-e2f592ebbd2e",
                    Some("release checklist v2")
                ),
            ]
        );
        assert_eq!(listed[1].path, dir.join("a.jsonl"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

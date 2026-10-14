//! Fixture format: one JSON object per line, `{"dir":"in"|"out"|"fs","frame":{…}}`.
//! `out` = server → host (OMP stdout), `in` = host → server (OMP stdin),
//! `fs` = a workspace file OMP's own tool wrote (`{"type":"fs_write"|"fs_delete",
//! "path": <cwd-relative>, "content"?}`), replayed onto disk at the same point
//! so OMP-native edits replay too (row G).
//! An `in` record may carry `"checkReason": true`: replay then also checks a
//! host tool refusal's reason, not only that it was refused.
//! Machine-specific paths are stored as placeholders so a fixture recorded in
//! one temp dir replays in another, and no home path is committed.

use serde_json::{Value, json};
use std::path::Path;

/// Frame direction relative to OMP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    /// Host → OMP (stdin).
    In,
    /// OMP → host (stdout).
    Out,
    /// File effect of an OMP tool call.
    Fs,
}

/// One recorded frame.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub dir: Dir,
    pub frame: Value,
    /// The host must refuse this host tool call for the recorded reason.
    pub check_reason: bool,
    /// The host's host tool result must carry the recorded text, whole.
    pub check_result: bool,
}

impl Record {
    /// Parse one fixture line.
    pub fn parse(line: &str) -> Result<Self, String> {
        let v: Value = serde_json::from_str(line).map_err(|e| format!("bad fixture line: {e}"))?;
        let dir = match v.get("dir").and_then(Value::as_str) {
            Some("in") => Dir::In,
            Some("out") => Dir::Out,
            Some("fs") => Dir::Fs,
            other => return Err(format!("bad fixture dir: {other:?}")),
        };
        let frame = v.get("frame").cloned().ok_or("fixture line has no frame")?;
        let check_reason = v.get("checkReason") == Some(&Value::Bool(true));
        let check_result = v.get("checkResult") == Some(&Value::Bool(true));
        Ok(Self {
            dir,
            frame,
            check_reason,
            check_result,
        })
    }

    /// Serialize as one fixture line (no trailing newline).
    pub fn to_line(&self) -> String {
        let dir = match self.dir {
            Dir::In => "in",
            Dir::Out => "out",
            Dir::Fs => "fs",
        };
        let mut line = json!({"dir": dir, "frame": self.frame});
        if self.check_reason {
            line["checkReason"] = json!(true);
        }
        if self.check_result {
            line["checkResult"] = json!(true);
        }
        line.to_string()
    }

    /// The frame's `type`, if any.
    pub fn frame_type(&self) -> Option<&str> {
        self.frame.get("type").and_then(Value::as_str)
    }
}

/// Real path ↔ placeholder token pairs for one run.
#[derive(Debug, Clone)]
pub struct Placeholders {
    /// `(real, token)`, longest `real` first so nested paths redact correctly.
    pairs: Vec<(String, &'static str)>,
    /// Redact-only spellings, never expanded: OMP may report a
    /// `/private/var/…` path cedian passed as `/var/…`.
    aliases: Vec<(String, &'static str)>,
}

impl Placeholders {
    pub fn new(session_dir: &Path, cwd: &Path, home: Option<String>) -> Self {
        let mut pairs = Vec::new();
        let mut aliases = Vec::new();
        let mut add = |path: &Path, token: &'static str| {
            let s = path.to_string_lossy().trim_end_matches('/').to_string();
            if s.len() > 1 {
                // macOS `/var` ↔ `/private/var`: redact both spellings.
                if let Ok(canon) = path.canonicalize() {
                    let canon = canon.to_string_lossy().trim_end_matches('/').to_string();
                    if canon != s {
                        pairs.push((canon, token));
                    }
                }
                if let Some(short) = s.strip_prefix("/private") {
                    aliases.push((short.to_string(), token));
                }
                pairs.push((s, token));
            }
        };
        add(session_dir, "${CEDIAN_SESSION_DIR}");
        add(cwd, "${CEDIAN_CWD}");
        if let Some(home) = home {
            add(Path::new(&home), "${HOME}");
        }
        pairs.sort_by_key(|p| std::cmp::Reverse(p.0.len()));
        aliases.sort_by_key(|p| std::cmp::Reverse(p.0.len()));
        Self { pairs, aliases }
    }

    /// Real paths → tokens (recording).
    pub fn redact(&self, text: &str) -> String {
        // Full spellings first: an alias is a suffix of its `/private` form.
        self.pairs
            .iter()
            .chain(&self.aliases)
            .fold(text.to_string(), |acc, (real, token)| {
                acc.replace(real.as_str(), token)
            })
    }

    /// Tokens → this run's real paths (replay). Uses the spelling cedian
    /// passed on argv (the last pair pushed per token is the argv form).
    pub fn expand(&self, text: &str) -> String {
        let mut out = text.to_string();
        for token in ["${CEDIAN_SESSION_DIR}", "${CEDIAN_CWD}", "${HOME}"] {
            if let Some(real) = self.argv_form(token) {
                out = out.replace(token, real);
            }
        }
        out
    }

    fn argv_form(&self, token: &str) -> Option<&str> {
        // Prefer a non-canonical (argv) spelling when both exist: it is the
        // one cedian itself compares against.
        let mut forms: Vec<&str> = self
            .pairs
            .iter()
            .filter(|(_, t)| *t == token)
            .map(|(r, _)| r.as_str())
            .collect();
        forms.sort_by_key(|r| r.starts_with("/private/"));
        forms.first().copied()
    }

    /// Redact a JSON value: paths → tokens, and opaque provider blobs
    /// ([`OPAQUE_KEYS`]) blanked — cedian never reads them, and they are
    /// account-bound ciphertext that does not belong in a committed fixture.
    pub fn redact_value(&self, v: &Value) -> Value {
        let mut v =
            serde_json::from_str(&self.redact(&v.to_string())).unwrap_or_else(|_| v.clone());
        blank_opaque(&mut v);
        v
    }

    /// Expand a JSON value via its serialized form.
    pub fn expand_value(&self, v: &Value) -> Value {
        serde_json::from_str(&self.expand(&v.to_string())).unwrap_or_else(|_| v.clone())
    }
}

/// Provider-encrypted reasoning fields, replaced by [`OPAQUE`] when recording.
pub const OPAQUE_KEYS: &[&str] = &["thinkingSignature", "encrypted_content"];
/// Stand-in value for a blanked opaque field.
pub const OPAQUE: &str = "${OPAQUE}";

fn blank_opaque(v: &mut Value) {
    match v {
        Value::Object(map) => {
            for (k, x) in map.iter_mut() {
                if OPAQUE_KEYS.contains(&k.as_str()) && x.is_string() {
                    *x = Value::String(OPAQUE.to_string());
                } else {
                    blank_opaque(x);
                }
            }
        }
        Value::Array(items) => items.iter_mut().for_each(blank_opaque),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_line_roundtrip() {
        let r = Record {
            dir: Dir::Out,
            frame: json!({"type": "ready"}),
            check_reason: false,
            check_result: false,
        };
        assert_eq!(Record::parse(&r.to_line()).unwrap(), r);
        let checked = Record {
            dir: Dir::In,
            frame: json!({"type": "host_tool_result"}),
            check_reason: true,
            check_result: true,
        };
        assert_eq!(Record::parse(&checked.to_line()).unwrap(), checked);
        assert!(Record::parse(r#"{"dir":"sideways","frame":{}}"#).is_err());
    }

    #[test]
    fn opaque_provider_blobs_blanked() {
        let p = Placeholders::new(Path::new("/tmp/a"), Path::new("/tmp/b"), None);
        let v = p.redact_value(
            &json!({"m": {"content": [{"thinkingSignature": "xyz", "text": "hi"}],
            "providerPayload": {"items": [{"encrypted_content": "abc"}]}}}),
        );
        assert_eq!(v["m"]["content"][0]["thinkingSignature"], OPAQUE);
        assert_eq!(v["m"]["content"][0]["text"], "hi");
        assert_eq!(
            v["m"]["providerPayload"]["items"][0]["encrypted_content"],
            OPAQUE
        );
    }

    #[test]
    fn private_var_short_spelling_is_redacted_not_expanded() {
        let p = Placeholders::new(
            Path::new("/private/var/t/s"),
            Path::new("/private/var/t/ws"),
            None,
        );
        assert_eq!(p.redact("/var/t/ws/notes.txt"), "${CEDIAN_CWD}/notes.txt");
        assert_eq!(p.redact("/private/var/t/ws/a"), "${CEDIAN_CWD}/a");
        assert_eq!(p.expand("${CEDIAN_CWD}/a"), "/private/var/t/ws/a");
    }

    #[test]
    fn redact_then_expand_moves_between_dirs() {
        let rec = Placeholders::new(
            Path::new("/tmp/rec/sessions"),
            Path::new("/tmp/rec"),
            Some("/Users/u".to_string()),
        );
        let text = "/tmp/rec/sessions/a /tmp/rec/b.txt /Users/u/.omp";
        let redacted = rec.redact(text);
        assert_eq!(
            redacted,
            "${CEDIAN_SESSION_DIR}/a ${CEDIAN_CWD}/b.txt ${HOME}/.omp"
        );
        let play = Placeholders::new(Path::new("/tmp/p/s"), Path::new("/tmp/p"), None);
        assert_eq!(
            play.expand(&redacted),
            "/tmp/p/s/a /tmp/p/b.txt ${HOME}/.omp"
        );
    }
}

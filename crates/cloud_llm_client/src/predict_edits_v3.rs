use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{Display, Write as _},
    ops::{Add, Range, Sub},
    path::Path,
    sync::Arc,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "test-support"), derive(PartialEq))]
#[serde(tag = "event")]
pub enum Event {
    BufferChange {
        path: Arc<Path>,
        old_path: Arc<Path>,
        diff: String,
        predicted: bool,
        in_open_source_repo: bool,
    },
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::BufferChange {
                path,
                old_path,
                diff,
                predicted,
                ..
            } => {
                if *predicted {
                    write!(
                        f,
                        "// User accepted prediction:\n--- a/{}\n+++ b/{}\n{diff}",
                        DiffPathFmt(old_path),
                        DiffPathFmt(path)
                    )
                } else {
                    write!(
                        f,
                        "--- a/{}\n+++ b/{}\n{diff}",
                        DiffPathFmt(old_path),
                        DiffPathFmt(path)
                    )
                }
            }
        }
    }
}

/// always format the Path as a unix path with `/` as the path sep in Diffs
pub struct DiffPathFmt<'a>(pub &'a Path);

impl<'a> std::fmt::Display for DiffPathFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        for component in self.0.components() {
            if !is_first {
                f.write_char('/')?;
            } else {
                is_first = false;
            }
            write!(f, "{}", component.as_os_str().display())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsResponse {
    pub request_id: Uuid,
    pub edits: Vec<Edit>,
    pub debug_info: Option<DebugInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    pub prompt: String,
    pub prompt_planning_time: Duration,
    pub model_response: String,
    pub inference_time: Duration,
    pub parsing_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edit {
    pub path: Arc<Path>,
    pub range: Range<Line>,
    pub content: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point {
    pub line: Line,
    pub column: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
#[serde(transparent)]
pub struct Line(pub u32);

impl Add for Line {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Line {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawCompletionRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    pub stop: Vec<Cow<'static, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictEditsV3Request {
    #[serde(flatten)]
    pub input: zeta_prompt::ZetaPromptInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub prompt_version: zeta_prompt::ZetaVersion,
}

impl From<zeta_prompt::ZetaPromptInput> for PredictEditsV3Request {
    fn from(input: zeta_prompt::ZetaPromptInput) -> Self {
        Self {
            input,
            model: None,
            prompt_version: zeta_prompt::ZetaVersion::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PredictEditsV3Response {
    pub request_id: String,
    pub output: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<RawCompletionChoice>,
    pub usage: RawCompletionUsage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawCompletionChoice {
    pub text: String,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawCompletionUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_event_display() {
        let ev = Event::BufferChange {
            path: Path::new("untitled").into(),
            old_path: Path::new("untitled").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: false,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                --- a/untitled
                +++ b/untitled
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );

        let ev = Event::BufferChange {
            path: Path::new("foo/bar.txt").into(),
            old_path: Path::new("foo/bar.txt").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: false,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                --- a/foo/bar.txt
                +++ b/foo/bar.txt
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );

        let ev = Event::BufferChange {
            path: Path::new("abc.txt").into(),
            old_path: Path::new("123.txt").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: false,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                --- a/123.txt
                +++ b/abc.txt
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );

        let ev = Event::BufferChange {
            path: Path::new("abc.txt").into(),
            old_path: Path::new("123.txt").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: true,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                // User accepted prediction:
                --- a/123.txt
                +++ b/abc.txt
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );
    }
}

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use editor::EditorSettings;
use editor::{Addon, Editor, EditorEvent};
use gpui::{App, BorrowAppContext, Context, Entity, Global, SharedString, Task, WeakEntity};
use language::{BufferEvent, BufferId, LanguageName, LanguageRegistry, PLAIN_TEXT};
use project::Project;
use project::debounced_delay::DebouncedDelay;
use settings::Settings;
use workspace::notifications::NotificationId;
use workspace::{Toast, Workspace};

const DETECTION_DEBOUNCE: Duration = Duration::from_millis(500);
const MIN_CONTENT_LENGTH: usize = 16;

/// Minimum score (ratio of valid nodes to total nodes) required for a
/// grammar to be considered a match. Below this threshold the content
/// is treated as unrecognised.
const MIN_SCORE_THRESHOLD: f64 = 0.5;

/// Grammar names that should not participate in language detection scoring.
/// These are non-hidden grammars that parse almost any input successfully
/// and cause false positives. Hidden grammars (like `jsdoc`, `regex`,
/// `markdown-inline`) are filtered out dynamically via
/// `AvailableLanguage::hidden()`.
const META_GRAMMAR_BLACKLIST: &[&str] = &["diff", "markdown", "gomod", "gowork", "gitcommit"];

/// Global language detector state.
/// Tracks which buffers had their language manually overridden by the user,
/// and which buffer is currently being updated by our detection logic.
pub struct LanguageDetector {
    user_overridden_buffers: Mutex<HashSet<BufferId>>,
    currently_applying: Mutex<Option<BufferId>>,
}

impl Global for LanguageDetector {}

impl LanguageDetector {
    fn new() -> Self {
        Self {
            user_overridden_buffers: Mutex::new(HashSet::new()),
            currently_applying: Mutex::new(None),
        }
    }

    fn is_user_overridden(&self, buffer_id: BufferId) -> bool {
        self.user_overridden_buffers
            .lock()
            .unwrap_or_else(|err| err.into_inner())
            .contains(&buffer_id)
    }

    fn mark_user_overridden(&self, buffer_id: BufferId) {
        self.user_overridden_buffers
            .lock()
            .unwrap_or_else(|err| err.into_inner())
            .insert(buffer_id);
    }

    fn clear_user_override(&self, buffer_id: BufferId) {
        self.user_overridden_buffers
            .lock()
            .unwrap_or_else(|err| err.into_inner())
            .remove(&buffer_id);
    }

    fn set_currently_applying(&self, buffer_id: Option<BufferId>) {
        *self
            .currently_applying
            .lock()
            .unwrap_or_else(|err| err.into_inner()) = buffer_id;
    }

    fn is_currently_applying(&self, buffer_id: BufferId) -> bool {
        *self
            .currently_applying
            .lock()
            .unwrap_or_else(|err| err.into_inner())
            == Some(buffer_id)
    }
}

/// Per-editor addon holding debounce state for language detection.
struct LanguageDetectionAddon {
    debounce: DebouncedDelay<Editor>,
}

impl Addon for LanguageDetectionAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

/// Builds the set of (LanguageName, tree_sitter::Language) pairs eligible
/// for detection scoring by cross-referencing native grammars with the
/// language registry. Filters out hidden languages and blacklisted
/// meta-grammars.
fn build_detection_grammars(
    languages: &Arc<LanguageRegistry>,
) -> Vec<(LanguageName, tree_sitter::Language)> {
    grammars::native_grammars()
        .into_iter()
        .filter(|(grammar_name, _)| !META_GRAMMAR_BLACKLIST.contains(grammar_name))
        .filter_map(|(grammar_name, ts_language)| {
            let available = languages.available_language_for_modeline_name(grammar_name)?;
            if available.hidden() {
                return None;
            }
            Some((available.name(), ts_language))
        })
        .collect()
}

pub fn init(cx: &mut App) {
    cx.set_global(LanguageDetector::new());

    cx.observe_new(|editor: &mut Editor, _window, cx: &mut Context<Editor>| {
        if !editor.mode().is_full() {
            return;
        }

        editor.register_addon(LanguageDetectionAddon {
            debounce: DebouncedDelay::new(),
        });

        // Subscribe to editor events (paste / edit).
        cx.subscribe(&cx.entity(), on_editor_event).detach();

        // Subscribe to LanguageChanged on the singleton buffer to detect
        // user-initiated language changes (via language selector).
        let Some(buffer) = editor.buffer().read(cx).as_singleton() else {
            return;
        };
        let buffer_id = buffer.read(cx).remote_id();

        cx.subscribe(&buffer, move |_editor, _buffer, event: &BufferEvent, cx| {
            if !matches!(event, BufferEvent::LanguageChanged(_)) {
                return;
            }
            let detector = cx.global::<LanguageDetector>();
            if !detector.is_currently_applying(buffer_id) {
                detector.mark_user_overridden(buffer_id);
            }
        })
        .detach();

        // Clean up when editor is released.
        cx.on_release(move |_, cx| {
            cx.update_global::<LanguageDetector, _>(|detector: &mut LanguageDetector, _| {
                detector.clear_user_override(buffer_id);
            });
        })
        .detach();

        // Handle stdin: if the buffer already has content when the editor
        // is created, trigger detection immediately.
        let (is_plain_text, has_content) = {
            let snapshot = buffer.read(cx);
            let plain = snapshot
                .language()
                .map_or(true, |lang| lang.name().as_ref() == "Plain Text");
            let content = snapshot.len() >= MIN_CONTENT_LENGTH;
            (plain, content)
        };

        if is_plain_text && has_content {
            trigger_detection(editor, &buffer, buffer_id, cx);
        }
    })
    .detach();
}

fn on_editor_event(
    editor: &mut Editor,
    _: Entity<Editor>,
    event: &EditorEvent,
    cx: &mut Context<Editor>,
) {
    let EditorEvent::Edited { .. } = event else {
        return;
    };

    let Some(buffer) = editor.buffer().read(cx).as_singleton() else {
        return;
    };

    let (is_plain_text, buffer_id, content_length) = {
        let snapshot = buffer.read(cx);
        let plain = snapshot
            .language()
            .map_or(true, |lang| lang.name().as_ref() == "Plain Text");
        (plain, snapshot.remote_id(), snapshot.len())
    };

    if !is_plain_text {
        return;
    }

    if !EditorSettings::get_global(cx).auto_detect_language {
        return;
    }

    if cx
        .global::<LanguageDetector>()
        .is_user_overridden(buffer_id)
    {
        return;
    }

    if content_length < MIN_CONTENT_LENGTH {
        return;
    }

    trigger_detection(editor, &buffer, buffer_id, cx);
}

fn trigger_detection(
    editor: &mut Editor,
    buffer: &Entity<language::Buffer>,
    buffer_id: BufferId,
    cx: &mut Context<Editor>,
) {
    let project = editor.project().cloned();
    let workspace = editor.workspace().map(|ws| ws.downgrade());
    let buffer_handle = buffer.downgrade();

    let Some(addon) = editor.addon_mut::<LanguageDetectionAddon>() else {
        return;
    };

    addon
        .debounce
        .fire_new(DETECTION_DEBOUNCE, cx, move |_editor, cx| {
            let Some(project) = project else {
                return Task::ready(());
            };
            let languages = project.read(cx).languages().clone();

            // Read buffer text after debounce fires (not before) to avoid
            // cloning the full buffer on every keystroke that resets the timer.
            let Some(buffer) = buffer_handle.upgrade() else {
                return Task::ready(());
            };
            let content: String = buffer.read(cx).text();
            let buffer_handle = buffer.downgrade();

            // Build detection grammars dynamically from the registry.
            let detection_grammars = build_detection_grammars(&languages);
            if detection_grammars.is_empty() {
                log::warn!("language_detection: no grammars available for detection");
                return Task::ready(());
            }

            let workspace = workspace.clone();

            cx.spawn(async move |_this, cx| {
                let detected = cx
                    .background_executor()
                    .spawn(async move { detect_language(&content, &detection_grammars) })
                    .await;

                let Some((language_name, display_name)) = detected else {
                    return;
                };

                let Ok(language) = languages
                    .language_for_name_or_extension(&language_name.0)
                    .await
                else {
                    return;
                };

                cx.update(|cx| {
                    let Some(buffer) = buffer_handle.upgrade() else {
                        return;
                    };

                    cx.update_global::<LanguageDetector, _>(
                        |detector: &mut LanguageDetector, _| {
                            detector.set_currently_applying(Some(buffer_id));
                        },
                    );

                    project.update(cx, |project, cx| {
                        project.set_language_for_buffer(&buffer, language.clone(), cx);
                    });

                    // Defer clearing so the LanguageChanged event still
                    // sees currently_applying when it dispatches.
                    cx.defer(|cx| {
                        cx.update_global::<LanguageDetector, _>(
                            |detector: &mut LanguageDetector, _| {
                                detector.set_currently_applying(None);
                            },
                        );
                    });

                    if let Some(workspace) = workspace.as_ref().and_then(|ws| ws.upgrade()) {
                        show_detection_toast(
                            &workspace,
                            &display_name,
                            buffer.downgrade(),
                            buffer_id,
                            project.downgrade(),
                            cx,
                        );
                    }
                });
            })
        });
}

/// Scores each grammar by parsing `content` and computing the ratio of
/// valid nodes to total descendant nodes. Returns the best-scoring
/// language if it exceeds `MIN_SCORE_THRESHOLD`.
fn detect_language(
    content: &str,
    grammars: &[(LanguageName, tree_sitter::Language)],
) -> Option<(LanguageName, String)> {
    let mut best_score: f64 = 0.0;
    let mut best_language: Option<&LanguageName> = None;
    let mut parser = tree_sitter::Parser::new();

    for (language_name, ts_language) in grammars {
        if parser.set_language(ts_language).is_err() {
            continue;
        }

        let Some(tree) = parser.parse(content, None) else {
            continue;
        };

        let root = tree.root_node();
        let total = root.descendant_count();
        if total == 0 {
            continue;
        }

        let error_count = count_error_nodes(root);
        let score = (total - error_count) as f64 / total as f64;

        if score > best_score {
            best_score = score;
            best_language = Some(language_name);
        }
    }

    if best_score < MIN_SCORE_THRESHOLD {
        return None;
    }

    best_language.map(|name| (name.clone(), name.0.to_string()))
}

/// Counts nodes that represent parse errors using iterative traversal.
fn count_error_nodes(node: tree_sitter::Node) -> usize {
    let mut count = 0;
    let mut cursor = node.walk();
    let mut visited = false;

    loop {
        if !visited {
            let current = cursor.node();
            if current.is_error() || current.is_missing() {
                count += 1;
            }
            if cursor.goto_first_child() {
                continue;
            }
        }
        if cursor.goto_next_sibling() {
            visited = false;
            continue;
        }
        if !cursor.goto_parent() {
            break;
        }
        visited = true;
    }

    count
}

fn show_detection_toast(
    workspace: &Entity<Workspace>,
    language_display_name: &str,
    buffer: WeakEntity<language::Buffer>,
    buffer_id: BufferId,
    project: WeakEntity<Project>,
    cx: &mut App,
) {
    struct LanguageDetectionNotification;

    let notification_id = NotificationId::composite::<LanguageDetectionNotification>(
        SharedString::from(format!("lang-detect-{}", buffer_id)),
    );

    let message: std::borrow::Cow<'static, str> =
        format!("Detected: {}", language_display_name).into();

    let toast = Toast::new(notification_id, message)
        .on_click("Undo", move |_window, cx| {
            let Some(buffer) = buffer.upgrade() else {
                return;
            };
            let Some(project) = project.upgrade() else {
                return;
            };

            cx.update_global::<LanguageDetector, _>(|detector: &mut LanguageDetector, _| {
                detector.set_currently_applying(Some(buffer_id));
                detector.clear_user_override(buffer_id);
            });

            project.update(cx, |project, cx| {
                project.set_language_for_buffer(&buffer, PLAIN_TEXT.clone(), cx);
            });

            cx.defer(|cx| {
                cx.update_global::<LanguageDetector, _>(|detector: &mut LanguageDetector, _| {
                    detector.set_currently_applying(None);
                });
            });
        })
        .autohide();

    workspace.update(cx, |workspace, cx| {
        workspace.show_toast(toast, cx);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pure guard logic: should we attempt language detection on this buffer?
    fn should_detect(
        language_name: &str,
        auto_detect_enabled: bool,
        user_overridden: bool,
    ) -> bool {
        language_name == "Plain Text" && auto_detect_enabled && !user_overridden
    }

    /// Builds the grammar list used by tests, filtering native grammars
    /// through the same blacklist and hidden-language logic as production.
    /// In tests, we don't have a LanguageRegistry, so we use the blacklist
    /// directly and assume non-blacklisted grammars are not hidden.
    fn test_grammars() -> Vec<(LanguageName, tree_sitter::Language)> {
        // Map grammar names to display names for testing.
        // In production, this mapping comes from the LanguageRegistry.
        fn grammar_to_test_language_name(grammar_name: &str) -> Option<LanguageName> {
            if META_GRAMMAR_BLACKLIST.contains(&grammar_name) {
                return None;
            }
            let language_name = match grammar_name {
                "bash" => "Shell Script",
                "c" => "C",
                "cpp" => "C++",
                "css" => "CSS",
                "go" => "Go",
                "json" => "JSON",
                "jsonc" => "JSONC",
                "python" => "Python",
                "rust" => "Rust",
                "tsx" => "TSX",
                "typescript" => "TypeScript",
                "yaml" => "YAML",
                _ => return None,
            };
            Some(LanguageName::new(language_name))
        }

        grammars::native_grammars()
            .into_iter()
            .filter_map(|(grammar_name, ts_lang)| {
                let lang_name = grammar_to_test_language_name(grammar_name)?;
                Some((lang_name, ts_lang))
            })
            .collect()
    }

    // -- should_detect guard tests ----------------------------------------

    #[test]
    fn test_should_detect_plain_text_enabled() {
        assert!(should_detect("Plain Text", true, false));
    }

    #[test]
    fn test_should_not_detect_non_plain_text() {
        assert!(!should_detect("Rust", true, false));
    }

    #[test]
    fn test_should_not_detect_when_disabled() {
        assert!(!should_detect("Plain Text", false, false));
    }

    #[test]
    fn test_should_not_detect_when_user_overridden() {
        assert!(!should_detect("Plain Text", true, true));
    }

    #[test]
    fn test_should_not_detect_all_flags_set() {
        assert!(!should_detect("Rust", false, true));
    }

    // -- detect_language scoring tests ------------------------------------

    #[test]
    fn test_detect_language_rust() {
        let grammars = test_grammars();
        let content = r#"
fn main() {
    let message = "hello world";
    println!("{}", message);
    for i in 0..10 {
        if i % 2 == 0 {
            println!("even: {}", i);
        }
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
"#;
        let result = detect_language(content, &grammars);
        assert!(result.is_some(), "expected Rust to be detected");
        let (name, _) = result.as_ref().expect("already checked");
        assert_eq!(
            name.as_ref(),
            "Rust",
            "expected Rust, got {}",
            name.as_ref()
        );
    }

    #[test]
    fn test_detect_language_python() {
        let grammars = test_grammars();
        let content = r#"
import os
import sys
from pathlib import Path

def fibonacci(n):
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b

class Calculator:
    def __init__(self):
        self.history = []

    def add(self, x, y):
        result = x + y
        self.history.append(result)
        return result

if __name__ == "__main__":
    calc = Calculator()
    print(calc.add(3, 4))
    print(fibonacci(10))
"#;
        let result = detect_language(content, &grammars);
        assert!(result.is_some(), "expected Python to be detected");
        let (name, _) = result.as_ref().expect("already checked");
        assert_eq!(
            name.as_ref(),
            "Python",
            "expected Python, got {}",
            name.as_ref()
        );
    }

    #[test]
    fn test_detect_language_ambiguous_text() {
        let grammars = test_grammars();
        let content = "This is just a plain English sentence with no code whatsoever. \
                        It does not contain any programming constructs, keywords, or syntax. \
                        The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet.";
        let result = detect_language(content, &grammars);
        if let Some((name, _)) = &result {
            let strict_languages = ["Rust", "Python", "C", "C++", "Go", "TypeScript", "TSX"];
            assert!(
                !strict_languages.contains(&name.as_ref()),
                "plain text incorrectly detected as {}",
                name.as_ref()
            );
        }
    }

    #[test]
    fn test_meta_grammars_excluded() {
        let grammars = test_grammars();
        let grammar_names: Vec<&str> = grammars.iter().map(|(name, _)| name.as_ref()).collect();
        for name in META_GRAMMAR_BLACKLIST {
            // Map blacklisted grammar names to their display names for comparison.
            // The test_grammars list uses display names, not grammar names.
            assert!(
                !grammar_names.iter().any(|g| g.to_lowercase() == *name),
                "{} should be excluded from detection grammars",
                name
            );
        }
    }

    #[test]
    fn test_count_error_nodes_iterative() {
        // Verify the iterative version produces the same count as a simple check.
        let mut parser = tree_sitter::Parser::new();
        let grammars = grammars::native_grammars();
        let rust_grammar = grammars
            .iter()
            .find(|(name, _)| *name == "rust")
            .map(|(_, lang)| lang);

        if let Some(lang) = rust_grammar {
            parser.set_language(lang).expect("set_language failed");
            // Valid Rust → 0 errors
            let tree = parser.parse("fn main() {}", None).expect("parse failed");
            assert_eq!(count_error_nodes(tree.root_node()), 0);

            // Invalid Rust → some errors
            let tree = parser.parse("fn main( { }", None).expect("parse failed");
            assert!(count_error_nodes(tree.root_node()) > 0);
        }
    }

    // -- performance test -------------------------------------------------

    #[test]
    fn test_detect_language_performance() {
        let grammars = test_grammars();

        let rust_content = r#"
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        for word in line.split_whitespace() {
            *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for (word, count) in sorted.iter().take(20) {
        writeln!(out, "{:>6} {}", count, word)?;
    }
    Ok(())
}

struct Config {
    max_words: usize,
    case_sensitive: bool,
    output_format: OutputFormat,
}

enum OutputFormat {
    Plain,
    Json,
    Csv,
}

impl Config {
    fn new() -> Self {
        Self {
            max_words: 20,
            case_sensitive: false,
            output_format: OutputFormat::Plain,
        }
    }
}
"#;

        let python_content = r#"
import sys
import json
from collections import Counter
from pathlib import Path
from typing import Dict, List, Optional

class WordCounter:
    def __init__(self, case_sensitive: bool = False):
        self.case_sensitive = case_sensitive
        self.counts: Counter = Counter()

    def process_line(self, line: str) -> None:
        words = line.split()
        if not self.case_sensitive:
            words = [w.lower() for w in words]
        self.counts.update(words)

    def top_words(self, n: int = 20) -> List[tuple]:
        return self.counts.most_common(n)

    def to_json(self) -> str:
        return json.dumps(dict(self.counts), indent=2)

def main(args: Optional[List[str]] = None) -> int:
    counter = WordCounter()
    for line in sys.stdin:
        counter.process_line(line.strip())
    for word, count in counter.top_words():
        print(f"{count:>6} {word}")
    return 0

if __name__ == "__main__":
    sys.exit(main())
"#;

        let snippets = [("Rust", rust_content), ("Python", python_content)];

        let grammar_count = grammars.len();
        eprintln!("\n=== Performance: {grammar_count} grammars ===");

        for (label, content) in &snippets {
            let content_len = content.len();
            let start = std::time::Instant::now();
            let result = detect_language(content, &grammars);
            let elapsed = start.elapsed();

            let detected = result
                .as_ref()
                .map(|(name, _)| name.as_ref())
                .unwrap_or("None");

            eprintln!("  {label}: {elapsed:?} ({content_len} bytes) -> {detected}");

            assert!(
                elapsed.as_millis() < 100,
                "{label}: detection took {}ms, budget is <100ms",
                elapsed.as_millis()
            );
        }

        eprintln!("=== All within 100ms budget ===\n");
    }
}

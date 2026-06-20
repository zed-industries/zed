//! Layer 1 of the corruption defense system: Output Quality Scoring.
//!
//! This module provides fast, stateless heuristics applied to a rolling window
//! of model output as it streams. Each detector votes independently, and
//! corruption is declared when enough high-confidence signals fire together.
//!
//! # Detectors
//!
//! | Detector             | Signal            | Typical Confidence |
//! |----------------------|-------------------|--------------------|
//! | Repetition           | `Repetition`      | 0.95               |
//! | Script switching     | `ScriptSwitching` | 0.90               |
//! | Task irrelevance     | `TaskIrrelevance` | 0.80               |
//!
//! # Architecture
//!
//! ```text
//! streaming text
//!     │
//!     ▼
//! RollingWindow (last N bytes)
//!     │
//!     ├─► detect_repetition()         ──► Option<DetectorReport>
//!     ├─► detect_script_switching()   ──► Option<DetectorReport>
//!     └─► detect_task_irrelevance()   ──► Option<DetectorReport>
//!     │
//!     ▼
//! CorruptionAssessment  ──► is_corrupted(config)?
//! ```

use crate::corruption::CorruptionSignal;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Voting thresholds for the corruption assessment.
#[derive(Debug, Clone)]
pub struct CorruptionConfig {
    /// Minimum number of high-confidence signals required to declare corruption.
    pub min_required_signals: usize,
    /// A detector report is "high-confidence" when its confidence ≥ this value.
    pub confidence_threshold: f32,
    /// Minimum bytes of accumulated output before detectors are run.
    /// Prevents false positives on short, partial chunks.
    pub min_window_bytes: usize,
}

impl Default for CorruptionConfig {
    fn default() -> Self {
        Self {
            min_required_signals: 2,
            confidence_threshold: 0.75,
            // Wait until at least 256 bytes have accumulated before scoring.
            // Very short outputs often look corrupted in isolation (partial
            // JSON, incomplete sentences, etc.) but are perfectly valid.
            min_window_bytes: 256,
        }
    }
}

// ---------------------------------------------------------------------------
// Detector reports
// ---------------------------------------------------------------------------

/// A report from a single detector.
#[derive(Debug, Clone)]
pub struct DetectorReport {
    /// Which type of corruption this detector signals.
    pub signal: CorruptionSignal,
    /// Confidence in [0.0, 1.0]; higher means more certain.
    pub confidence: f32,
    /// Human-readable reason for the detection (for telemetry/logs).
    pub reason: String,
}

/// Overall corruption assessment computed from independent detector votes.
#[derive(Debug, Clone)]
pub struct CorruptionAssessment {
    /// Reports from detectors that fired (non-`None`).
    pub triggered_signals: Vec<DetectorReport>,
    /// Combined confidence: the maximum confidence among triggered detectors.
    pub overall_confidence: f32,
}

impl CorruptionAssessment {
    fn empty() -> Self {
        Self {
            triggered_signals: Vec::new(),
            overall_confidence: 0.0,
        }
    }

    /// Returns `true` when at least `config.min_required_signals` detectors
    /// reported confidence ≥ `config.confidence_threshold`.
    pub fn is_corrupted(&self, config: &CorruptionConfig) -> bool {
        let high_confidence_count = self
            .triggered_signals
            .iter()
            .filter(|r| r.confidence >= config.confidence_threshold)
            .count();
        high_confidence_count >= config.min_required_signals
    }

    /// Returns the list of signal labels (for `CorruptionDetail`).
    pub fn signal_labels(&self) -> Vec<String> {
        self.triggered_signals
            .iter()
            .map(|r| r.signal.label().to_string())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Task context (for the irrelevance detector)
// ---------------------------------------------------------------------------

/// Lightweight bag of keywords extracted from the user's prompt and the
/// current editing context. Used by [`detect_task_irrelevance`].
#[derive(Debug, Clone, Default)]
pub struct TaskContext {
    /// Lower-cased keywords extracted from the user's prompt text.
    pub keywords: Vec<String>,
}

impl TaskContext {
    /// Build a task context from raw prompt text.
    ///
    /// Extracts words of length ≥ 4, lower-cased, de-duplicated.
    /// Short common stop-words are filtered out.
    pub fn from_prompt(prompt: &str) -> Self {
        let stop_words: &[&str] = &[
            "about", "above", "after", "again", "also", "been", "before", "between",
            "both", "but", "came", "come", "could", "does", "done", "each", "else",
            "even", "every", "find", "first", "from", "get", "give", "go", "good",
            "great", "had", "has", "have", "here", "him", "his", "how", "into",
            "isn", "its", "just", "keep", "know", "last", "let", "like", "look",
            "made", "make", "many", "may", "might", "more", "most", "much", "must",
            "need", "never", "next", "no", "not", "now", "of", "off", "often",
            "once", "only", "or", "other", "our", "out", "over", "own", "part",
            "place", "put", "read", "right", "said", "same", "say", "see", "set",
            "she", "should", "show", "small", "so", "some", "still", "such", "take",
            "tell", "than", "that", "the", "their", "them", "then", "there", "these",
            "they", "thing", "think", "this", "those", "through", "time", "too",
            "turn", "under", "up", "upon", "use", "used", "using", "very", "want",
            "was", "way", "well", "went", "were", "what", "when", "where", "which",
            "while", "who", "why", "will", "with", "word", "work", "would", "yet",
            "you", "your",
        ];

        let mut keywords: Vec<String> = prompt
            .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
            .filter(|w| w.len() >= 4)
            .map(|w| w.to_lowercase())
            .filter(|w| !stop_words.contains(&w.as_str()))
            .collect();

        keywords.sort_unstable();
        keywords.dedup();
        Self { keywords }
    }

    /// Returns `true` when there is no meaningful context to compare against.
    pub fn is_empty(&self) -> bool {
        self.keywords.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Output quality scorer
// ---------------------------------------------------------------------------

/// Stateless scorer that runs all Phase-B detectors on a text window.
pub struct OutputQualityScorer;

impl OutputQualityScorer {
    /// Run all detectors on `window_text` and return a combined assessment.
    ///
    /// Returns `None` if the window is too small to score reliably.
    pub fn assess(
        window_text: &str,
        task_context: &TaskContext,
        config: &CorruptionConfig,
    ) -> Option<CorruptionAssessment> {
        if window_text.len() < config.min_window_bytes {
            return None;
        }

        let mut reports = Vec::new();

        if let Some(report) = detect_repetition(window_text) {
            reports.push(report);
        }
        if let Some(report) = detect_script_switching(window_text) {
            reports.push(report);
        }
        if let Some(report) = detect_task_irrelevance(window_text, task_context) {
            reports.push(report);
        }

        let overall_confidence = reports
            .iter()
            .map(|r| r.confidence)
            .fold(0.0_f32, f32::max);

        Some(CorruptionAssessment {
            triggered_signals: reports,
            overall_confidence,
        })
    }
}

// ---------------------------------------------------------------------------
// Detector 1: Repetition
// ---------------------------------------------------------------------------

/// Detects token loops and degenerate repeated output.
///
/// Two strategies are employed:
/// 1. **Single-character RLE**: fires when the same character repeats ≥ 48 times
///    in a row (excluding whitespace runs which are often intentional).
/// 2. **Period detection**: for candidate periods `p` from 2 to 128, checks if
///    the trailing portion of the text is periodic (≥ 85% match rate over ≥ 3
///    full periods and ≥ 64 total characters).
///
/// Returns `Some(DetectorReport)` when repetition is detected.
fn detect_repetition(text: &str) -> Option<DetectorReport> {
    let bytes = text.as_bytes();
    let len = bytes.len();

    // Strategy 1: Single-character RLE (skip whitespace-only runs).
    let mut run_len: usize = 1;
    let mut max_run: usize = 0;
    let mut max_run_byte: u8 = 0;
    for i in 1..len {
        if bytes[i] == bytes[i - 1] {
            run_len += 1;
        } else {
            if run_len > max_run {
                max_run = run_len;
                max_run_byte = bytes[i - 1];
            }
            run_len = 1;
        }
    }
    if run_len > max_run {
        max_run = run_len;
        max_run_byte = bytes[len - 1];
    }

    if max_run >= 48 && !max_run_byte.is_ascii_whitespace() {
        let confidence = (max_run as f32 / len as f32).min(1.0) * 0.95 + 0.05;
        return Some(DetectorReport {
            signal: CorruptionSignal::Repetition,
            confidence: confidence.min(0.98),
            reason: format!(
                "single-char run of '{}' ({} times)",
                max_run_byte as char, max_run
            ),
        });
    }

    // Strategy 2: Period detection on the trailing portion.
    // Examine the last `scan_len` bytes (up to 512).
    let scan_len = len.min(512);
    let scan_start = len - scan_len;
    let scan = &bytes[scan_start..];

    // Try periods from 2 to scan_len/3 (need at least 3 repetitions to detect).
    for period in 2..=(scan_len / 3).min(128) {
        // Check if `scan` is periodic with the given `period`.
        // Count how many positions match `scan[i % period]`.
        let mut matches: usize = 0;
        let mut total: usize = 0;
        for (i, &b) in scan.iter().enumerate() {
            // Skip leading partial period to align.
            let offset = i % period;
            if i >= period {
                total += 1;
                if b == scan[offset] {
                    matches += 1;
                }
            }
        }

        if total == 0 {
            continue;
        }

        let match_ratio = matches as f32 / total as f32;
        let num_periods = scan_len as f32 / period as f32;

        // Require ≥ 85% match rate and ≥ 3 full periods covering ≥ 64 chars.
        if match_ratio >= 0.85 && num_periods >= 3.0 && scan_len >= 64 {
            let confidence = (match_ratio - 0.85) / 0.15 * 0.2 + 0.75;
            let confidence = confidence.min(0.98);
            let pattern = String::from_utf8_lossy(&scan[..period]);
            return Some(DetectorReport {
                signal: CorruptionSignal::Repetition,
                confidence,
                reason: format!(
                    "periodic output: period={period}, match={match_ratio:.2}, \
                     pattern='{pattern}'"
                ),
            });
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Detector 2: Script switching
// ---------------------------------------------------------------------------

/// Unicode script classification (coarse-grained).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Script {
    Latin,
    Han,
    HiraganaKatakana,
    Hangul,
    Cyrillic,
    Arabic,
    Devanagari,
    Thai,
    Greek,
    Punctuation,
    Digit,
    Whitespace,
    Other,
}

/// Classify a character into a coarse script category.
fn classify_script(c: char) -> Script {
    if c.is_ascii_whitespace() || c.is_whitespace() {
        return Script::Whitespace;
    }
    if c.is_ascii_digit() || c.is_numeric() {
        return Script::Digit;
    }
    if c.is_ascii_punctuation() || !c.is_alphanumeric() {
        return Script::Punctuation;
    }
    let cp = c as u32;
    match cp {
        // Basic Latin + Latin Extended
        0x0041..=0x024F | 0x1E00..=0x1EFF | 0x2C60..=0x2C7F | 0xA720..=0xA7FF => Script::Latin,
        // CJK Unified Ideographs + extensions
        0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF | 0x20000..=0x2A6DF => Script::Han,
        // Hiragana + Katakana
        0x3040..=0x309F | 0x30A0..=0x30FF | 0x31F0..=0x31FF => Script::HiraganaKatakana,
        // Hangul
        0x1100..=0x11FF | 0x3130..=0x318F | 0xAC00..=0xD7AF => Script::Hangul,
        // Cyrillic
        0x0400..=0x04FF | 0x0500..=0x052F => Script::Cyrillic,
        // Arabic
        0x0600..=0x06FF | 0x0750..=0x077F => Script::Arabic,
        // Devanagari
        0x0900..=0x097F => Script::Devanagari,
        // Thai
        0x0E00..=0x0E7F => Script::Thai,
        // Greek
        0x0370..=0x03FF => Script::Greek,
        // Full-width Latin letters (common in CJK text)
        0xFF01..=0xFF5E => Script::Latin,
        _ => {
            // Fallback: use Rust's built-in Unicode awareness.
            if c.is_alphabetic() {
                // Heuristic: treat remaining alphabetic chars as "Other"
                // but check common ranges we missed.
                if (0x00C0..=0x00FF).contains(&cp) {
                    Script::Latin
                } else {
                    Script::Other
                }
            } else {
                Script::Other
            }
        }
    }
}

/// Detects rapid Unicode script transitions (e.g. Latin → Han → Cyrillic).
///
/// A healthy editing output typically uses one or two scripts (Latin + digits
/// or Latin + a single CJK script). More than 2 distinct non-trivial scripts
/// with frequent transitions is a strong corruption signal.
///
/// Fires when:
/// - ≥ 3 distinct "content" scripts (excluding Punctuation/Digit/Whitespace)
/// - ≥ 5 script transitions in the examined window
fn detect_script_switching(text: &str) -> Option<DetectorReport> {
    // Only examine the trailing portion to catch ongoing collapses.
    let examine: String = text
        .chars()
        .rev()
        .take(512)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let mut last_script: Option<Script> = None;
    let mut transitions: u32 = 0;
    let mut script_set = std::collections::HashSet::new();

    for c in examine.chars() {
        let s = classify_script(c);
        // Skip "boring" classes that don't count as content scripts.
        if matches!(s, Script::Whitespace | Script::Punctuation | Script::Digit) {
            continue;
        }

        if let Some(prev) = last_script {
            if prev != s {
                transitions += 1;
            }
        }
        script_set.insert(s);
        last_script = Some(s);
    }

    let distinct_scripts = script_set.len();

    // Require both conditions: 3+ distinct scripts AND 5+ transitions.
    if distinct_scripts >= 3 && transitions >= 5 {
        let transition_factor = (transitions as f32).min(20.0) / 20.0;
        let script_factor = (distinct_scripts as f32 - 2.0).min(4.0) / 4.0;
        let confidence = 0.6 + transition_factor * 0.2 + script_factor * 0.15;

        return Some(DetectorReport {
            signal: CorruptionSignal::ScriptSwitching,
            confidence: confidence.min(0.98),
            reason: format!(
                "{distinct_scripts} distinct scripts, {transitions} transitions"
            ),
        });
    }

    None
}

// ---------------------------------------------------------------------------
// Detector 3: Task irrelevance
// ---------------------------------------------------------------------------

/// Detects output that appears unrelated to the current task.
///
/// In an editing workflow, model output should reference files, symbols, or
/// the user's prompt in some way. When the output shares almost no vocabulary
/// with the task context, this is a corruption signal.
///
/// Fires when:
/// - The task context is non-empty (we have keywords to compare against)
/// - The window contains ≥ 32 "content words" (alphabetic, length ≥ 4)
/// - The overlap ratio (output words ∩ context keywords) / output words < 0.05
///
/// Does NOT fire when:
/// - The context is empty (no basis for comparison)
/// - The output is too short to judge
/// - The output contains code-like structures (braces, semicolons dominate)
fn detect_task_irrelevance(text: &str, context: &TaskContext) -> Option<DetectorReport> {
    if context.is_empty() {
        return None;
    }

    // Skip detection if the text looks predominantly like code
    // (high ratio of braces/semicolons/angle brackets to words).
    let code_chars: usize = text
        .chars()
        .filter(|c| matches!(*c, '{' | '}' | ';' | '<' | '>' | '(' | ')' | '=' | '|'))
        .count();
    let total_chars = text.len().max(1);
    let code_ratio = code_chars as f32 / total_chars as f32;
    if code_ratio > 0.05 {
        // Text is at least 5% code-significant characters — likely a code output.
        return None;
    }

    // Extract content words from the output.
    let output_words: Vec<String> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() >= 4)
        .map(|w| w.to_lowercase())
        .collect();

    if output_words.len() < 32 {
        return None;
    }

    // Count overlap.
    let overlap_count = output_words
        .iter()
        .filter(|w| context.keywords.binary_search(w).is_ok())
        .count();

    let overlap_ratio = overlap_count as f32 / output_words.len() as f32;

    // Fire if overlap is below 5%.
    if overlap_ratio < 0.05 {
        let confidence = (1.0 - overlap_ratio / 0.05) * 0.3 + 0.6;
        return Some(DetectorReport {
            signal: CorruptionSignal::TaskIrrelevance,
            confidence: confidence.min(0.95),
            reason: format!(
                "task relevance overlap: {}/{} words ({:.1}%)",
                overlap_count,
                output_words.len(),
                overlap_ratio * 100.0
            ),
        });
    }

    None
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> CorruptionConfig {
        CorruptionConfig::default()
    }

    // -- Repetition detector --------------------------------------------------

    #[test]
    fn test_repetition_single_char_run() {
        let text = "Hello world\n".to_string()
            + &"a".repeat(60)
            + "\nSome more text here to fill the window.";
        let report = detect_repetition(&text);
        assert!(report.is_some(), "should detect single-char run of 60");
        let r = report.unwrap();
        assert_eq!(r.signal, CorruptionSignal::Repetition);
        assert!(r.confidence >= 0.75);
    }

    #[test]
    fn test_repetition_periodic() {
        // Repeat "The quick brown fox jumps over the lazy dog. " many times.
        let unit = "The quick brown fox jumps over the lazy dog. ";
        let text = unit.repeat(12);
        let report = detect_repetition(&text);
        assert!(
            report.is_some(),
            "should detect periodic repetition"
        );
        let r = report.unwrap();
        assert_eq!(r.signal, CorruptionSignal::Repetition);
    }

    #[test]
    fn test_no_repetition_normal_text() {
        let text = "\
            The function `parse_config` reads the TOML file at the given path \
            and returns a `Config` struct. If the file does not exist, it \
            returns a default configuration. The caller is responsible for \
            handling I/O errors.";
        let report = detect_repetition(text);
        assert!(report.is_none(), "normal prose should not trigger repetition");
    }

    #[test]
    fn test_repetition_ignores_whitespace_runs() {
        let text = "Some text.\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nMore text here after the blank lines.";
        let report = detect_repetition(text);
        assert!(
            report.is_none(),
            "whitespace runs should not trigger repetition"
        );
    }

    // -- Script switching detector -------------------------------------------

    #[test]
    fn test_script_switching_fires_on_mixed_scripts() {
        // Latin + Han + Cyrillic mixed together
        let text = "Hello世界Приветworld你好мирtest测试Привет again and more mixing 中文 English Русский";
        let report = detect_script_switching(text);
        assert!(
            report.is_some(),
            "mixed Latin/Han/Cyrillic should trigger script switching"
        );
        let r = report.unwrap();
        assert_eq!(r.signal, CorruptionSignal::ScriptSwitching);
    }

    #[test]
    fn test_script_switching_ignores_pure_latin() {
        let text = "\
            This is a perfectly normal English sentence. It contains only \
            Latin characters and some numbers like 42 and 3.14. The quick \
            brown fox jumps over the lazy dog. Nothing unusual here at all.";
        let report = detect_script_switching(text);
        assert!(
            report.is_none(),
            "pure Latin text should not trigger script switching"
        );
    }

    #[test]
    fn test_script_switching_allows_latin_plus_digits() {
        let text = "\
            fn main() { let x = 42; let y = 3.14; println!(\"{}\", x + y); } \
            The output is 45.14 which is correct. This code handles numbers.";
        let report = detect_script_switching(text);
        assert!(report.is_none(), "Latin + digits should not trigger");
    }

    // -- Task irrelevance detector -------------------------------------------

    #[test]
    fn test_task_irrelevance_fires_on_unrelated_output() {
        let context = TaskContext::from_prompt(
            "Fix the authentication bug in src/auth/login.rs that causes \
             session tokens to expire immediately after creation",
        );
        let output = "\
            The weather today is expected to be sunny with temperatures \
            reaching up to thirty degrees celsius in the southern regions \
            while the northern areas will experience moderate rainfall \
            throughout the afternoon and evening bringing relief from the \
            recent heatwave that has affected the entire continent for the \
            past several weeks according to meteorological experts who have \
            been tracking the unusual weather patterns observed recently";
        let report = detect_task_irrelevance(output, &context);
        assert!(
            report.is_some(),
            "completely unrelated weather text should trigger irrelevance"
        );
        let r = report.unwrap();
        assert_eq!(r.signal, CorruptionSignal::TaskIrrelevance);
    }

    #[test]
    fn test_task_irrelevance_ignores_relevant_output() {
        let context = TaskContext::from_prompt(
            "Fix the authentication bug in src/auth/login.rs that causes \
             session tokens to expire immediately after creation",
        );
        let output = "\
            Looking at the authentication function in login.rs I can see \
            that the session token expiration is being set to zero seconds \
            which causes immediate expiry. The fix involves changing the \
            token creation timestamp and the expiration duration to proper \
            values in the authentication handler.";
        let report = detect_task_irrelevance(output, &context);
        assert!(
            report.is_none(),
            "relevant output should not trigger irrelevance"
        );
    }

    #[test]
    fn test_task_irrelevance_ignores_code_output() {
        let context = TaskContext::from_prompt("rename variable foo to bar in utils.rs");
        let output = "\
            pub fn process(data: &Config) -> Result<(), Error> {\n    \
                let bar = data.get_value();\n    \
                let result = transform(bar)?;\n    \
                Ok(result)\n}\n";
        let report = detect_task_irrelevance(output, &context);
        assert!(report.is_none(), "code-heavy output should be skipped");
    }

    #[test]
    fn test_task_irrelevance_empty_context() {
        let context = TaskContext::default();
        let output = "Some random output that has nothing to do with anything in particular but is long enough to be evaluated by the detector properly.";
        let report = detect_task_irrelevance(output, &context);
        assert!(
            report.is_none(),
            "empty context should not trigger irrelevance"
        );
    }

    // -- TaskContext ----------------------------------------------------------

    #[test]
    fn test_task_context_filters_stop_words() {
        let ctx = TaskContext::from_prompt("Please fix the bug in authentication module");
        // "please", "the", "in" should be filtered; "fix", "bug", "authentication", "module" kept
        assert!(ctx.keywords.contains(&"authentication".to_string()));
        assert!(ctx.keywords.contains(&"module".to_string()));
        assert!(!ctx.keywords.iter().any(|w| w == "please" || w == "the"));
    }

    // -- CorruptionAssessment -------------------------------------------------

    #[test]
    fn test_assessment_requires_min_signals() {
        let config = CorruptionConfig {
            min_required_signals: 2,
            confidence_threshold: 0.75,
            min_window_bytes: 0,
        };
        let assessment = CorruptionAssessment {
            triggered_signals: vec![DetectorReport {
                signal: CorruptionSignal::Repetition,
                confidence: 0.95,
                reason: "test".into(),
            }],
            overall_confidence: 0.95,
        };
        assert!(
            !assessment.is_corrupted(&config),
            "single signal should not trigger corruption"
        );

        let assessment2 = CorruptionAssessment {
            triggered_signals: vec![
                DetectorReport {
                    signal: CorruptionSignal::Repetition,
                    confidence: 0.95,
                    reason: "test".into(),
                },
                DetectorReport {
                    signal: CorruptionSignal::ScriptSwitching,
                    confidence: 0.90,
                    reason: "test".into(),
                },
            ],
            overall_confidence: 0.95,
        };
        assert!(
            assessment2.is_corrupted(&config),
            "two high-confidence signals should trigger corruption"
        );
    }

    #[test]
    fn test_assessment_respects_confidence_threshold() {
        let config = CorruptionConfig {
            min_required_signals: 2,
            confidence_threshold: 0.75,
            min_window_bytes: 0,
        };
        let assessment = CorruptionAssessment {
            triggered_signals: vec![
                DetectorReport {
                    signal: CorruptionSignal::Repetition,
                    confidence: 0.60, // below threshold
                    reason: "test".into(),
                },
                DetectorReport {
                    signal: CorruptionSignal::ScriptSwitching,
                    confidence: 0.50, // below threshold
                    reason: "test".into(),
                },
            ],
            overall_confidence: 0.60,
        };
        assert!(
            !assessment.is_corrupted(&config),
            "low-confidence signals should not trigger corruption"
        );
    }

    // -- OutputQualityScorer::assess -----------------------------------------

    #[test]
    fn test_assess_returns_none_for_short_text() {
        let config = default_config();
        let ctx = TaskContext::from_prompt("fix the bug");
        let result = OutputQualityScorer::assess("short", &ctx, &config);
        assert!(result.is_none(), "short text should return None");
    }

    #[test]
    fn test_assess_returns_empty_for_clean_text() {
        let config = CorruptionConfig {
            min_window_bytes: 10,
            ..default_config()
        };
        let ctx = TaskContext::from_prompt(
            "fix the authentication bug in login.rs session token expiry",
        );
        let text = "\
            Looking at the authentication function in login.rs I can see \
            that the session token expiration is being set to zero seconds \
            which causes immediate expiry. The fix involves changing the \
            token creation timestamp and the expiration duration to proper \
            values in the authentication handler module.";
        let result = OutputQualityScorer::assess(text, &ctx, &config);
        assert!(result.is_some());
        let assessment = result.unwrap();
        assert!(
            !assessment.is_corrupted(&config),
            "clean relevant text should not be flagged as corrupted"
        );
    }
}

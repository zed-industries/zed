use regex::Regex;
use std::{num::NonZeroU32, sync::LazyLock};

/// The settings extracted from an emacs/vim modelines.
///
/// The parsing tries to best match the modeline directives and
/// variables to Zed, matching LanguageSettings fields.
///
/// It is not exhaustive, but covers the most common settings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModelineSettings {
    /// The zed language name from the modeline.
    pub language: Option<String>,
    /// How many columns a tab should occupy.
    pub tab_size: Option<NonZeroU32>,
    /// Whether to indent lines using tab characters, as opposed to multiple
    /// spaces.
    pub hard_tabs: Option<bool>,
    /// The number of bytes that comprise the indentation.
    pub indent_size: Option<NonZeroU32>,
    /// Whether to auto-indent lines
    pub auto_indent: Option<bool>,
    /// The column at which to soft-wrap lines, for buffers where soft-wrap
    /// is enabled.
    pub preferred_line_length: Option<NonZeroU32>,
    /// Coding/encoding specification.
    pub encoding: Option<String>,

    /// Extra and unknown variables.
    pub emacs_extra_variables: Vec<(String, String)>,
    pub vim_extra_variables: Vec<(String, String)>,
}

impl ModelineSettings {
    fn has_settings(&self) -> bool {
        self != &Self::default()
    }
}

/// Parse modelines from file content.
///
/// Supports:
/// - Emacs modelines: -*- mode: rust; tab-width: 4; indent-tabs-mode: nil; -*- and "Local Variables"
/// - Vim modelines: vim: set ft=rust ts=4 sw=4 et:
pub fn parse_modeline(first_lines: &[&str], last_lines: &[&str]) -> Option<ModelineSettings> {
    let mut settings = ModelineSettings::default();

    parse_modelines(first_lines, &mut settings);

    // Parse Emacs Local Variables in last lines
    parse_emacs_local_variables(last_lines, &mut settings);

    // Also check for vim modelines in last lines if we don't have settings yet
    if !settings.has_settings() {
        parse_vim_modelines(last_lines, &mut settings);
    }

    Some(settings).filter(|s| s.has_settings())
}

fn parse_modelines(modelines: &[&str], settings: &mut ModelineSettings) {
    for line in modelines {
        parse_emacs_modeline(line, settings);
        // if emacs is set, do not check for vim modelines
        if settings.has_settings() {
            return;
        }
    }

    parse_vim_modelines(modelines, settings);
}

static EMACS_MODELINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-\*-\s*(.+?)\s*-\*-").expect("valid regex"));

/// Parse Emacs-style modelines
/// Format: -*- mode: rust; tab-width: 4; indent-tabs-mode: nil; -*-
/// See Emacs (set-auto-mode)
fn parse_emacs_modeline(line: &str, settings: &mut ModelineSettings) {
    let Some(captures) = EMACS_MODELINE_RE.captures(line) else {
        return;
    };
    let Some(modeline_content) = captures.get(1).map(|m| m.as_str()) else {
        return;
    };
    for part in modeline_content.split(';') {
        parse_emacs_key_value(part, settings, true);
    }
}

/// Parse Emacs-style Local Variables block
///
/// Emacs supports a "Local Variables" block at the end of files:
/// ```text
/// /* Local Variables: */
/// /* mode: c */
/// /* tab-width: 4 */
/// /* End: */
/// ```
///
/// Emacs related code is hack-local-variables--find-variables in
/// https://cgit.git.savannah.gnu.org/cgit/emacs.git/tree/lisp/files.el#n4346
fn parse_emacs_local_variables(lines: &[&str], settings: &mut ModelineSettings) {
    let Some((i, prefix, suffix)) = lines.iter().enumerate().find_map(|(i, line)| {
        line.find("Local Variables:")
            .map(|prefix_len| (i, &line[..prefix_len], &line[prefix_len + 16..]))
    }) else {
        return;
    };

    let mut concat = None;
    let mut i = i + 1;
    loop {
        let Some(line) = lines.get(i) else {
            return;
        };
        let Some(line) = line.strip_prefix(prefix) else {
            return;
        };
        let Some(line) = line.strip_suffix(suffix) else {
            return;
        };
        let mut line = line.trim();
        let mut line_bs = None;
        if let Some(line) = line.strip_suffix('\\') {
            if concat.is_none() {
                concat = Some(String::new());
            }
            line_bs = Some(line);
        }
        if let Some(c) = &mut concat {
            if let Some(line) = line_bs {
                c.push_str(line);
                i += 1;
                continue;
            } else {
                c.push_str(line);
                line = concat.as_ref().unwrap();
            }
        }
        if line == "End:" {
            return;
        }
        parse_emacs_key_value(line, settings, false);
        concat = None;
        i += 1;
    }
}

fn parse_emacs_key_value(part: &str, settings: &mut ModelineSettings, bare: bool) {
    let part = part.trim();
    if part.is_empty() {
        return;
    }

    if let Some((key, value)) = part.split_once(':') {
        let key = key.trim();
        let value = value.trim();

        match key.to_lowercase().as_str() {
            "mode" => {
                settings.language = Some(modeline_language_to_zed(value));
            }
            "c-basic-offset" | "python-indent-offset" => {
                if let Ok(size) = value.parse::<NonZeroU32>() {
                    settings.indent_size = Some(size);
                }
            }
            "fill-column" => {
                if let Ok(size) = value.parse::<NonZeroU32>() {
                    settings.preferred_line_length = Some(size);
                }
            }
            "tab-width" => {
                if let Ok(size) = value.parse::<NonZeroU32>() {
                    settings.tab_size = Some(size);
                }
            }
            "indent-tabs-mode" => {
                settings.hard_tabs = Some(value != "nil");
            }
            "coding" => {
                settings.encoding = Some(value.to_string());
            }
            "electric-indent-mode" => {
                settings.auto_indent = Some(value != "nil");
            }
            key => settings
                .emacs_extra_variables
                .push((key.to_string(), value.to_string())),
        }
    } else if bare {
        // Handle bare mode specification (e.g., -*- rust -*-)
        settings.language = Some(modeline_language_to_zed(part));
    }
}

fn parse_vim_modelines(modelines: &[&str], settings: &mut ModelineSettings) {
    for line in modelines {
        parse_vim_modeline(line, settings);
    }
}

static VIM_MODELINE_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        // Second form: [text{white}]{vi:vim:Vim:}[white]se[t] {options}:[text]
        // Allow escaped colons in options: match non-colon chars or backslash followed by any char
        // Only include ex: here if it's not at start of line
        r"(?:(?:^|\s)(vi|vim|Vim):(?:\s*)se(?:t)?\s+((?:[^\\:]|\\.)*):|(?:\s)(ex):(?:\s*)se(?:t)?\s+((?:[^\\:]|\\.)*))",
        // First form: [text{white}]{vi:vim:}[white]{options}
        // Note: ex: at start of line is ignored (spec says it could be short for "example:")
        r"(?:(?:^|\s+)(vi|vim):(?:\s*(.+))|(?:\s+)(ex):(?:\s*(.+)))",
    ]
        .iter()
        .map(|pattern| Regex::new(pattern).expect("valid regex"))
        .collect()
});

/// Parse Vim-style modelines
/// Supports both forms:
/// 1. First form: vi:noai:sw=3 ts=6
/// 2. Second form: vim: set ft=rust ts=4 sw=4 et:
fn parse_vim_modeline(line: &str, settings: &mut ModelineSettings) {
    for re in VIM_MODELINE_PATTERNS.iter() {
        if let Some(captures) = re.captures(line) {
            if let Some(options) = captures.get(2).or_else(|| captures.get(4)) {
                parse_vim_settings(options.as_str().trim(), settings);
                break;
            }
        }
    }
}

fn parse_vim_settings(content: &str, settings: &mut ModelineSettings) {
    fn split_colon_unescape(input: &str) -> Vec<String> {
        let mut split = Vec::new();
        let mut str = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some(escaped_char) => str.push(escaped_char),
                    None => str.push('\\'),
                }
            } else if c == ':' {
                split.push(std::mem::take(&mut str));
            } else {
                str.push(c);
            }
        }
        split.push(str);
        split
    }

    let parts = split_colon_unescape(content);
    for colon_part in parts {
        let colon_part = colon_part.trim();
        if colon_part.is_empty() {
            continue;
        }

        // Each colon part might contain space-separated options
        let space_parts: Vec<&str> = colon_part.split_whitespace().collect();
        for part in space_parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if let Some((key, value)) = part.split_once('=') {
                match key {
                    "ft" | "filetype" => {
                        settings.language = Some(modeline_language_to_zed(value));
                    }
                    "ts" | "tabstop" => {
                        if let Ok(size) = value.parse::<NonZeroU32>() {
                            settings.tab_size = Some(size);
                        } else {
                            // Store invalid values as extra variables
                            settings
                                .vim_extra_variables
                                .push((key.to_string(), value.to_string()));
                        }
                    }
                    "sw" | "shiftwidth" => {
                        if let Ok(size) = value.parse::<NonZeroU32>() {
                            settings.indent_size = Some(size);
                        } else {
                            // Store invalid values as extra variables
                            settings
                                .vim_extra_variables
                                .push((key.to_string(), value.to_string()));
                        }
                    }
                    "tw" | "textwidth" => {
                        if let Ok(size) = value.parse::<NonZeroU32>() {
                            settings.preferred_line_length = Some(size);
                        } else {
                            // Store invalid values as extra variables
                            settings
                                .vim_extra_variables
                                .push((key.to_string(), value.to_string()));
                        }
                    }
                    "fileencoding" | "encoding" | "enc" => {
                        settings.encoding = Some(value.to_string());
                    }
                    _ => {
                        settings
                            .vim_extra_variables
                            .push((key.to_string(), value.to_string()));
                    }
                }
            } else {
                match part {
                    "ai" | "autoindent" => {
                        settings.auto_indent = Some(true);
                    }
                    "noai" | "noautoindent" => {
                        settings.auto_indent = Some(false);
                    }
                    "et" | "expandtab" => {
                        settings.hard_tabs = Some(false);
                    }
                    "noet" | "noexpandtab" => {
                        settings.hard_tabs = Some(true);
                    }
                    "set" => {
                        // Ignore the "set" keyword itself
                    }
                    _ => {
                        settings
                            .vim_extra_variables
                            .push((part.to_string(), "true".to_string()));
                    }
                }
            }
        }
    }
}

/// Normalize language names from modelines to Zed language names
///
/// It may make sense to move the mapping somewhere else and allow further customization.
fn modeline_language_to_zed(name: &str) -> String {
    let name = name.trim().to_lowercase();

    match name.as_str() {
        "c" => "C".to_string(),
        "c++" | "cxx" => "C++".to_string(),
        "css" => "CSS".to_string(),
        "bash" | "fish" | "sh" | "shell" | "zsh" => "Shell Script".to_string(),
        "go" | "golang" => "Go".to_string(),
        "html" | "htm" => "HTML".to_string(),
        "javascript" | "js" => "JavaScript".to_string(),
        "json" => "JSON".to_string(),
        "jsonc" => "JSONC".to_string(),
        "makefile" => "Make".to_string(),
        "markdown" | "md" => "Markdown".to_string(),
        "meson" => "Meson".to_string(),
        "perl" | "pl" => "Perl".to_string(),
        "python" | "py" => "Python".to_string(),
        "ruby" | "rb" => "Ruby".to_string(),
        "rust" | "rs" => "Rust".to_string(),
        "rst" => "reST".to_string(),
        "scheme" => "Scheme".to_string(),
        "latex" | "tex" => "LaTeX".to_string(),
        "text" | "txt" => "Plain Text".to_string(),
        "typescript" | "ts" => "TypeScript".to_string(),
        "xml" => "XML".to_string(),
        "yaml" => "YAML".to_string(),
        _ => name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_no_modeline() {
        let content = "This is just regular content\nwith no modeline";
        assert!(parse_modeline(&[content], &[content]).is_none());
    }

    #[test]
    fn test_emacs_bare_mode() {
        let content = "/* -*- rust -*- */";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("Rust".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_emacs_modeline_parsing() {
        let content = "/* -*- mode: rust; tab-width: 4; indent-tabs-mode: nil; -*- */";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("Rust".to_string()),
                tab_size: Some(NonZeroU32::new(4).unwrap()),
                hard_tabs: Some(false),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_emacs_last_line_parsing() {
        let content = indoc! {r#"
        # Local Variables:
        # compile-command: "cc foo.c -Dfoo=bar -Dhack=whatever \
        #   -Dmumble=blaah"
        # End:
        "#}
        .lines()
        .collect::<Vec<_>>();
        let settings = parse_modeline(&[], &content).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                emacs_extra_variables: vec![(
                    "compile-command".to_string(),
                    "\"cc foo.c -Dfoo=bar -Dhack=whatever -Dmumble=blaah\"".to_string()
                ),],
                ..Default::default()
            }
        );

        let content = indoc! {"
            foo
            /* Local Variables: */
            /* eval: (font-lock-mode -1) */
            /* mode: old-c */
            /* mode: c */
            /* End: */
            /* mode: ignored */
        "}
        .lines()
        .collect::<Vec<_>>();
        let settings = parse_modeline(&[], &content).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("C".to_string()),
                emacs_extra_variables: vec![(
                    "eval".to_string(),
                    "(font-lock-mode -1)".to_string()
                ),],
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_vim_modeline_parsing() {
        // Test second form (set format)
        let content = "// vim: set ft=rust ts=4 sw=4 et:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("Rust".to_string()),
                tab_size: Some(NonZeroU32::new(4).unwrap()),
                hard_tabs: Some(false),
                indent_size: Some(NonZeroU32::new(4).unwrap()),
                ..Default::default()
            }
        );

        // Test first form (colon-separated)
        let content = "vi:noai:sw=3:ts=6";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                tab_size: Some(NonZeroU32::new(6).unwrap()),
                auto_indent: Some(false),
                indent_size: Some(NonZeroU32::new(3).unwrap()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_vim_modeline_first_form() {
        // Examples from vim specification: vi:noai:sw=3 ts=6
        let content = "   vi:noai:sw=3 ts=6 ";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                tab_size: Some(NonZeroU32::new(6).unwrap()),
                auto_indent: Some(false),
                indent_size: Some(NonZeroU32::new(3).unwrap()),
                ..Default::default()
            }
        );

        // Test with filetype
        let content = "vim:ft=python:ts=8:noet";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("Python".to_string()),
                tab_size: Some(NonZeroU32::new(8).unwrap()),
                hard_tabs: Some(true),
                ..Default::default()
            }
        );

        // Test 'ex:' format
        let content = "   ex:ft=javascript:et:sw=2";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("JavaScript".to_string()),
                hard_tabs: Some(false),
                indent_size: Some(NonZeroU32::new(2).unwrap()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_vim_modeline_second_form() {
        // Examples from vim specification: /* vim: set ai tw=75: */
        let content = "/* vim: set ai tw=75: */";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                auto_indent: Some(true),
                preferred_line_length: Some(NonZeroU32::new(75).unwrap()),
                ..Default::default()
            }
        );

        // Test with 'Vim:' (capital V)
        let content = "/* Vim: set ai tw=75: */";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                auto_indent: Some(true),
                preferred_line_length: Some(NonZeroU32::new(75).unwrap()),
                ..Default::default()
            }
        );

        // Test 'se' shorthand
        let content = "// vi: se ft=c ts=4:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("C".to_string()),
                tab_size: Some(NonZeroU32::new(4).unwrap()),
                ..Default::default()
            }
        );

        // Test complex modeline with encoding
        let content = "# vim: set ft=python ts=4 sw=4 et encoding=utf-8:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("Python".to_string()),
                tab_size: Some(NonZeroU32::new(4).unwrap()),
                hard_tabs: Some(false),
                indent_size: Some(NonZeroU32::new(4).unwrap()),
                encoding: Some("utf-8".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_vim_modeline_edge_cases() {
        // Test modeline at start of line (compatibility with version 3.0)
        let content = "vi:ts=2:et";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                tab_size: Some(NonZeroU32::new(2).unwrap()),
                hard_tabs: Some(false),
                ..Default::default()
            }
        );

        // Test vim at start of line
        let content = "vim:ft=rust:noet";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("Rust".to_string()),
                hard_tabs: Some(true),
                ..Default::default()
            }
        );

        // Test that 'ex:' at start of line is ignored (as per spec)
        let content = "ex:ft=ignored";
        assert!(parse_modeline(&[content], &[]).is_none());

        // Test mixed boolean flags
        let content = "vim: set wrap noet ts=8:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                tab_size: Some(NonZeroU32::new(8).unwrap()),
                hard_tabs: Some(true),
                vim_extra_variables: vec![("wrap".to_string(), "true".to_string())],
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_vim_modeline_invalid_cases() {
        // Test malformed options are ignored gracefully
        let content = "vim: set ts=invalid ft=rust:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                language: Some("Rust".to_string()),
                vim_extra_variables: vec![("ts".to_string(), "invalid".to_string())],
                ..Default::default()
            }
        );

        // Test empty modeline content - this should still work as there might be options
        let content = "vim: set :";
        // This should return None because there are no actual options
        let result = parse_modeline(&[content], &[]);
        assert!(result.is_none(), "Expected None but got: {:?}", result);

        // Test modeline without proper format
        let content = "not a modeline";
        assert!(parse_modeline(&[content], &[]).is_none());

        // Test word that looks like modeline but isn't
        let content = "example: this could be confused with ex:";
        assert!(parse_modeline(&[content], &[]).is_none());
    }

    #[test]
    fn test_vim_language_mapping() {
        // Test vim-specific language mappings
        let content = "vim: set ft=sh:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.language, Some("Shell Script".to_string()));

        let content = "vim: set ft=golang:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.language, Some("Go".to_string()));

        let content = "vim: set filetype=js:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.language, Some("JavaScript".to_string()));
    }

    #[test]
    fn test_vim_extra_variables() {
        // Test that unknown vim options are stored as extra variables
        let content = "vim: set foldmethod=marker conceallevel=2 custom=value:";
        let settings = parse_modeline(&[content], &[]).unwrap();

        assert!(
            settings
                .vim_extra_variables
                .contains(&("foldmethod".to_string(), "marker".to_string()))
        );
        assert!(
            settings
                .vim_extra_variables
                .contains(&("conceallevel".to_string(), "2".to_string()))
        );
        assert!(
            settings
                .vim_extra_variables
                .contains(&("custom".to_string(), "value".to_string()))
        );
    }

    #[test]
    fn test_modeline_position() {
        // Test modeline in first lines
        let first_lines = ["#!/bin/bash", "# vim: set ft=bash ts=4:"];
        let settings = parse_modeline(&first_lines, &[]).unwrap();
        assert_eq!(settings.language, Some("Shell Script".to_string()));

        // Test modeline in last lines
        let last_lines = ["", "/* vim: set ft=c: */"];
        let settings = parse_modeline(&[], &last_lines).unwrap();
        assert_eq!(settings.language, Some("C".to_string()));

        // Test no modeline found
        let content = ["regular content", "no modeline here"];
        assert!(parse_modeline(&content, &content).is_none());
    }

    #[test]
    fn test_vim_modeline_version_checks() {
        // Note: Current implementation doesn't support version checks yet
        // These are tests for future implementation based on vim spec

        // Test version-specific modelines (currently ignored in our implementation)
        let content = "/* vim700: set foldmethod=marker */";
        // Should be ignored for now since we don't support version checks
        assert!(parse_modeline(&[content], &[]).is_none());

        let content = "/* vim>702: set cole=2: */";
        // Should be ignored for now since we don't support version checks
        assert!(parse_modeline(&[content], &[]).is_none());
    }

    #[test]
    fn test_vim_modeline_colon_escaping() {
        // Test colon escaping as mentioned in vim spec

        // According to vim spec: "if you want to include a ':' in a set command precede it with a '\'"
        let content = r#"/* vim: set fdm=expr fde=getline(v\:lnum)=~'{'?'>1'\:'1': */"#;

        let result = parse_modeline(&[content], &[]).unwrap();

        // The modeline should parse fdm=expr and fde=getline(v:lnum)=~'{'?'>1':'1'
        // as extra variables since they're not recognized settings
        assert_eq!(result.vim_extra_variables.len(), 2);
        assert_eq!(result.vim_extra_variables[0], ("fdm".to_string(), "expr".to_string()));
        assert_eq!(result.vim_extra_variables[1], ("fde".to_string(), "getline(v:lnum)=~'{'?'>1':'1'".to_string()));
    }

    #[test]
    fn test_vim_modeline_whitespace_requirements() {
        // Test whitespace requirements from vim spec

        // Valid: whitespace before vi/vim/ex
        let content = "  vim: set ft=rust:";
        assert!(parse_modeline(&[content], &[]).is_some());

        // Valid: tab before vi/vim/ex
        let content = "\tvim: set ft=rust:";
        assert!(parse_modeline(&[content], &[]).is_some());

        // Valid: vi/vim at start of line (compatibility)
        let content = "vim: set ft=rust:";
        assert!(parse_modeline(&[content], &[]).is_some());

        // Invalid: ex at start of line should be ignored
        let content = "ex: set ft=rust:";
        assert!(parse_modeline(&[content], &[]).is_none());

        // Valid: ex with whitespace before
        let content = " ex: set ft=rust:";
        assert!(parse_modeline(&[content], &[]).is_some());
    }

    #[test]
    fn test_vim_modeline_comprehensive_examples() {
        // Real-world examples from vim documentation and common usage

        // Python example
        let content = "# vim: set expandtab tabstop=4 shiftwidth=4 softtabstop=4:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.hard_tabs, Some(false));
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(4).unwrap()));

        // C example with multiple options
        let content = "/* vim: set ts=8 sw=8 noet ai cindent: */";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(8).unwrap()));
        assert_eq!(settings.hard_tabs, Some(true));
        assert!(
            settings
                .vim_extra_variables
                .contains(&("cindent".to_string(), "true".to_string()))
        );

        // Shell script example
        let content = "# vi: set ft=sh ts=2 sw=2 et:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.language, Some("Shell Script".to_string()));
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(2).unwrap()));
        assert_eq!(settings.hard_tabs, Some(false));

        // First form colon-separated
        let content = "vim:ft=xml:ts=2:sw=2:et";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.language, Some("XML".to_string()));
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(2).unwrap()));
        assert_eq!(settings.hard_tabs, Some(false));
    }

    #[test]
    fn test_combined_emacs_vim_detection() {
        // Test that both emacs and vim modelines can be detected in the same file

        let first_lines = [
            "#!/usr/bin/env python3",
            "# -*- coding: utf-8 -*-",
            "# vim: set ft=python ts=4 sw=4 et:",
        ];

        // Should find the emacs modeline first (with coding)
        let settings = parse_modeline(&first_lines, &[]).unwrap();
        assert_eq!(settings.encoding, Some("utf-8".to_string()));

        // Test vim-only content
        let vim_only = ["# vim: set ft=python ts=4 sw=4 et:"];
        let settings = parse_modeline(&vim_only, &[]).unwrap();
        assert_eq!(settings.language, Some("Python".to_string()));
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(4).unwrap()));
        assert_eq!(settings.hard_tabs, Some(false));
    }

    #[test]
    fn test_language_normalization() {
        assert_eq!(modeline_language_to_zed("c++"), "C++");
        assert_eq!(modeline_language_to_zed("js"), "JavaScript");
        assert_eq!(modeline_language_to_zed("unknown"), "unknown");
    }
}

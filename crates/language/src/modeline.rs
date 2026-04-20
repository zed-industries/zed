use regex::Regex;
use std::{num::NonZeroU32, sync::LazyLock};

/// The settings extracted from an emacs/vim modelines.
///
/// The parsing tries to best match the modeline directives and
/// variables to Zed, matching LanguageSettings fields.
/// The mode mapping is done later thanks to the LanguageRegistry.
///
/// It is not exhaustive, but covers the most common settings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModelineSettings {
    /// The emacs mode or vim filetype.
    pub mode: Option<String>,
    /// How many columns a tab should occupy.
    pub tab_size: Option<NonZeroU32>,
    /// Whether to indent lines using tab characters, as opposed to multiple
    /// spaces.
    pub hard_tabs: Option<bool>,
    /// The number of bytes that comprise the indentation.
    pub indent_size: Option<NonZeroU32>,
    /// Whether to auto-indent lines.
    pub auto_indent: Option<bool>,
    /// The column at which to soft-wrap lines.
    pub preferred_line_length: Option<NonZeroU32>,
    /// Whether to ensure a final newline at the end of the file.
    pub ensure_final_newline: Option<bool>,
    /// Whether to show trailing whitespace on the editor.
    pub show_trailing_whitespace: Option<bool>,

    /// Emacs modeline variables that were parsed but not mapped to Zed settings.
    /// Stored as (variable-name, value) pairs.
    pub emacs_extra_variables: Vec<(String, String)>,
    /// Vim modeline options that were parsed but not mapped to Zed settings.
    /// Stored as (option-name, value) pairs.
    pub vim_extra_variables: Vec<(String, Option<String>)>,
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
    const LOCAL_VARIABLES: &str = "Local Variables:";

    let Some((start_idx, prefix, suffix)) = lines.iter().enumerate().find_map(|(i, line)| {
        let prefix_len = line.find(LOCAL_VARIABLES)?;
        let suffix_start = prefix_len + LOCAL_VARIABLES.len();
        Some((i, line.get(..prefix_len)?, line.get(suffix_start..)?))
    }) else {
        return;
    };

    let mut continuation = String::new();

    for line in &lines[start_idx + 1..] {
        let Some(content) = line
            .strip_prefix(prefix)
            .and_then(|l| l.strip_suffix(suffix))
            .map(str::trim)
        else {
            return;
        };

        if let Some(continued) = content.strip_suffix('\\') {
            continuation.push_str(continued);
            continue;
        }

        let to_parse = if continuation.is_empty() {
            content
        } else {
            continuation.push_str(content);
            &continuation
        };

        if to_parse == "End:" {
            return;
        }

        parse_emacs_key_value(to_parse, settings, false);
        continuation.clear();
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
                settings.mode = Some(value.to_string());
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
            "electric-indent-mode" => {
                settings.auto_indent = Some(value != "nil");
            }
            "require-final-newline" => {
                settings.ensure_final_newline = Some(value != "nil");
            }
            "show-trailing-whitespace" => {
                settings.show_trailing_whitespace = Some(value != "nil");
            }
            key => settings
                .emacs_extra_variables
                .push((key.to_string(), value.to_string())),
        }
    } else if bare {
        // Handle bare mode specification (e.g., -*- rust -*-)
        settings.mode = Some(part.to_string());
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
        r"(?:^|\s)(vi|vim|Vim):(?:\s*)se(?:t)?\s+((?:[^\\:]|\\.)*):",
        // First form: [text{white}]{vi:vim:}[white]{options}
        r"(?:^|\s+)(vi|vim):(?:\s*(.+))",
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
            if let Some(options) = captures.get(2) {
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
        for part in colon_part.split_whitespace() {
            if let Some((key, value)) = part.split_once('=') {
                match key {
                    "ft" | "filetype" => {
                        settings.mode = Some(value.to_string());
                    }
                    "ts" | "tabstop" => {
                        if let Ok(size) = value.parse::<NonZeroU32>() {
                            settings.tab_size = Some(size);
                        }
                    }
                    "sw" | "shiftwidth" => {
                        if let Ok(size) = value.parse::<NonZeroU32>() {
                            settings.indent_size = Some(size);
                        }
                    }
                    "tw" | "textwidth" => {
                        if let Ok(size) = value.parse::<NonZeroU32>() {
                            settings.preferred_line_length = Some(size);
                        }
                    }
                    _ => {
                        settings
                            .vim_extra_variables
                            .push((key.to_string(), Some(value.to_string())));
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
                    "eol" | "endofline" => {
                        settings.ensure_final_newline = Some(true);
                    }
                    "noeol" | "noendofline" => {
                        settings.ensure_final_newline = Some(false);
                    }
                    "set" => {
                        // Ignore the "set" keyword itself
                    }
                    _ => {
                        settings.vim_extra_variables.push((part.to_string(), None));
                    }
                }
            }
        }
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
                mode: Some("rust".to_string()),
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
                mode: Some("rust".to_string()),
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
                mode: Some("c".to_string()),
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
                mode: Some("rust".to_string()),
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
                mode: Some("python".to_string()),
                tab_size: Some(NonZeroU32::new(8).unwrap()),
                hard_tabs: Some(true),
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
                mode: Some("c".to_string()),
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
                mode: Some("python".to_string()),
                tab_size: Some(NonZeroU32::new(4).unwrap()),
                hard_tabs: Some(false),
                indent_size: Some(NonZeroU32::new(4).unwrap()),
                vim_extra_variables: vec![("encoding".to_string(), Some("utf-8".to_string()))],
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
                mode: Some("rust".to_string()),
                hard_tabs: Some(true),
                ..Default::default()
            }
        );

        // Test mixed boolean flags
        let content = "vim: set wrap noet ts=8:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(
            settings,
            ModelineSettings {
                tab_size: Some(NonZeroU32::new(8).unwrap()),
                hard_tabs: Some(true),
                vim_extra_variables: vec![("wrap".to_string(), None)],
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
                mode: Some("rust".to_string()),
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
        assert_eq!(settings.mode, Some("sh".to_string()));

        let content = "vim: set ft=golang:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.mode, Some("golang".to_string()));

        let content = "vim: set filetype=js:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.mode, Some("js".to_string()));
    }

    #[test]
    fn test_vim_extra_variables() {
        // Test that unknown vim options are stored as extra variables
        let content = "vim: set foldmethod=marker conceallevel=2 custom=value:";
        let settings = parse_modeline(&[content], &[]).unwrap();

        assert!(
            settings
                .vim_extra_variables
                .contains(&("foldmethod".to_string(), Some("marker".to_string())))
        );
        assert!(
            settings
                .vim_extra_variables
                .contains(&("conceallevel".to_string(), Some("2".to_string())))
        );
        assert!(
            settings
                .vim_extra_variables
                .contains(&("custom".to_string(), Some("value".to_string())))
        );
    }

    #[test]
    fn test_modeline_position() {
        // Test modeline in first lines
        let first_lines = ["#!/bin/bash", "# vim: set ft=bash ts=4:"];
        let settings = parse_modeline(&first_lines, &[]).unwrap();
        assert_eq!(settings.mode, Some("bash".to_string()));

        // Test modeline in last lines
        let last_lines = ["", "/* vim: set ft=c: */"];
        let settings = parse_modeline(&[], &last_lines).unwrap();
        assert_eq!(settings.mode, Some("c".to_string()));

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
        assert_eq!(
            result.vim_extra_variables[0],
            ("fdm".to_string(), Some("expr".to_string()))
        );
        assert_eq!(
            result.vim_extra_variables[1],
            (
                "fde".to_string(),
                Some("getline(v:lnum)=~'{'?'>1':'1'".to_string())
            )
        );
    }

    #[test]
    fn test_vim_modeline_whitespace_requirements() {
        // Test whitespace requirements from vim spec

        // Valid: whitespace before vi/vim
        let content = "  vim: set ft=rust:";
        assert!(parse_modeline(&[content], &[]).is_some());

        // Valid: tab before vi/vim
        let content = "\tvim: set ft=rust:";
        assert!(parse_modeline(&[content], &[]).is_some());

        // Valid: vi/vim at start of line (compatibility)
        let content = "vim: set ft=rust:";
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
                .contains(&("cindent".to_string(), None))
        );

        // Shell script example
        let content = "# vi: set ft=sh ts=2 sw=2 et:";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.mode, Some("sh".to_string()));
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(2).unwrap()));
        assert_eq!(settings.hard_tabs, Some(false));

        // First form colon-separated
        let content = "vim:ft=xml:ts=2:sw=2:et";
        let settings = parse_modeline(&[content], &[]).unwrap();
        assert_eq!(settings.mode, Some("xml".to_string()));
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(2).unwrap()));
        assert_eq!(settings.hard_tabs, Some(false));
    }

    #[test]
    fn test_combined_emacs_vim_detection() {
        // Test that both emacs and vim modelines can be detected in the same file

        let first_lines = [
            "#!/usr/bin/env python3",
            "# -*- require-final-newline: t; -*-",
            "# vim: set ft=python ts=4 sw=4 et:",
        ];

        // Should find the emacs modeline first (with coding)
        let settings = parse_modeline(&first_lines, &[]).unwrap();
        assert_eq!(settings.ensure_final_newline, Some(true));
        assert_eq!(settings.tab_size, None);

        // Test vim-only content
        let vim_only = ["# vim: set ft=python ts=4 sw=4 et:"];
        let settings = parse_modeline(&vim_only, &[]).unwrap();
        assert_eq!(settings.mode, Some("python".to_string()));
        assert_eq!(settings.tab_size, Some(NonZeroU32::new(4).unwrap()));
        assert_eq!(settings.hard_tabs, Some(false));
    }
}

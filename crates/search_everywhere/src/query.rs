use crate::SearchTab;

#[derive(Debug, PartialEq, Eq)]
pub struct ScopePrefix<'a> {
    pub tab: SearchTab,
    pub rest: &'a str,
}

const COMMANDS: &[(&str, &str, SearchTab)] = &[
    ("all", "", SearchTab::All),
    ("classes", "c", SearchTab::Classes),
    ("files", "f", SearchTab::Files),
    ("symbols", "s", SearchTab::Symbols),
    ("actions", "a", SearchTab::Actions),
    ("text", "t", SearchTab::Text),
];

// A command only counts once it's followed by a space, so typing `/f` on the way
// to `/files` doesn't switch tabs early.
pub fn parse_scope_prefix(raw: &str) -> Option<ScopePrefix<'_>> {
    if let Some(rest) = raw.strip_prefix('#') {
        return Some(ScopePrefix {
            tab: SearchTab::Symbols,
            rest,
        });
    }
    let command_line = raw.strip_prefix('/')?;
    let (command, rest) = command_line.split_once(' ')?;
    let command = command.to_ascii_lowercase();
    COMMANDS
        .iter()
        .find(|(long, short, _)| command == *long || (!short.is_empty() && command == *short))
        .map(|(_, _, tab)| ScopePrefix { tab: *tab, rest })
}

// 1-based, as typed by the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinePosition {
    pub row: u32,
    pub column: Option<u32>,
}

pub fn split_line_suffix(query: &str) -> (&str, Option<LinePosition>) {
    let parse = |s: &str| -> Option<u32> {
        (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
            .then(|| s.parse().ok())
            .flatten()
    };

    let Some((head, last)) = query.rsplit_once(':') else {
        return (query, None);
    };
    let Some(last) = parse(last) else {
        return (query, None);
    };
    if let Some((path, row)) = head.rsplit_once(':')
        && let Some(row) = parse(row)
        && !path.is_empty()
    {
        return (
            path,
            Some(LinePosition {
                row,
                column: Some(last),
            }),
        );
    }
    if head.is_empty() {
        return (query, None);
    }
    (
        head,
        Some(LinePosition {
            row: last,
            column: None,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_prefixes() {
        assert_eq!(
            parse_scope_prefix("/files foo"),
            Some(ScopePrefix {
                tab: SearchTab::Files,
                rest: "foo"
            })
        );
        assert_eq!(
            parse_scope_prefix("/c Foo"),
            Some(ScopePrefix {
                tab: SearchTab::Classes,
                rest: "Foo"
            })
        );
        assert_eq!(
            parse_scope_prefix("/Text "),
            Some(ScopePrefix {
                tab: SearchTab::Text,
                rest: ""
            })
        );
        assert_eq!(
            parse_scope_prefix("#bar"),
            Some(ScopePrefix {
                tab: SearchTab::Symbols,
                rest: "bar"
            })
        );
        // Still typing the command.
        assert_eq!(parse_scope_prefix("/fil"), None);
        // Unknown command, or a path that happens to start with a slash.
        assert_eq!(parse_scope_prefix("/usr/lib foo"), None);
        assert_eq!(parse_scope_prefix("plain"), None);
    }

    #[test]
    fn line_suffixes() {
        assert_eq!(split_line_suffix("main.rs"), ("main.rs", None));
        assert_eq!(
            split_line_suffix("main.rs:12"),
            (
                "main.rs",
                Some(LinePosition {
                    row: 12,
                    column: None
                })
            )
        );
        assert_eq!(
            split_line_suffix("main.rs:12:4"),
            (
                "main.rs",
                Some(LinePosition {
                    row: 12,
                    column: Some(4)
                })
            )
        );
        // Rust paths and incomplete suffixes are not line numbers.
        assert_eq!(split_line_suffix("std::fmt"), ("std::fmt", None));
        assert_eq!(split_line_suffix("main.rs:"), ("main.rs:", None));
        assert_eq!(split_line_suffix(":12"), (":12", None));
    }
}

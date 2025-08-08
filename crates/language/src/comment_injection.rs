//! Support for tagged comments.
//!
//! Contains functionality common for both languages built into Zed and provided by Zed extensions.

use super::{LanguageQueries, QUERY_FILENAME_PREFIXES};

/// Support injecting the "tagged comment language" into other languages.
pub trait CommentInjection {
    // Entry format: (language name, injection query variant)
    const LANGUAGE_COMMENT_INJECTIONS: &[(&'static str, CommentInjectionQuery)];

    // Repeatedly used injection query clauses.
    const C_COMMENT: &str = "(comment)";
    // Repeatedly used injection queries.
    const Q_COMMENT: CommentInjectionQuery = CommentInjectionQuery::Clause {
        clause: Self::C_COMMENT,
    };

    /// Adds the tagged comment language injection query to the injections of supported languages.
    fn update_injections(language: &str, queries: &mut LanguageQueries) {
        use std::fmt::Write as _;

        if let Some((comment_injection, query)) = Self::LANGUAGE_COMMENT_INJECTIONS
            .iter()
            .find(|(defined, _)| *defined == language)
            .and_then(|(_, comment_injection)| {
                QUERY_FILENAME_PREFIXES.iter().find_map(|(name, query)| {
                    if *name == "injections" {
                        Some((comment_injection, query))
                    } else {
                        None
                    }
                })
            })
        {
            match query(queries) {
                None => *query(queries) = Some(comment_injection.to_string().into()),
                Some(injections) => write!(injections.to_mut(), "{comment_injection}").unwrap(),
            }
        };
    }
}

#[derive(Debug)]
pub enum CommentInjectionQuery {
    /// Match clause only.
    Clause { clause: &'static str },
    /// Match clause and a match expression.
    ClauseAndExp {
        clause: &'static str,
        exp: &'static str,
    },
}

impl std::fmt::Display for CommentInjectionQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        match self {
            Self::Clause { clause } => {
                write!(f, "{clause} @injection.content")?;
            }
            Self::ClauseAndExp { clause, exp } => {
                write!(
                    f,
                    "{clause} @injection.content (#match? @injection.content {exp})"
                )?;
            }
        }
        write!(f, r#" (#set! injection.language "comment"))"#)
    }
}

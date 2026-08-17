use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

use crate::Key;

#[derive(Debug, Clone)]
pub(crate) enum CustomError {
    DuplicateKey {
        key: String,
        table: Option<Vec<Key>>,
    },
    DottedKeyExtendWrongType {
        key: Vec<Key>,
        actual: &'static str,
    },
    OutOfRange,
    #[cfg_attr(feature = "unbounded", allow(dead_code))]
    RecursionLimitExceeded,
}

impl CustomError {
    pub(crate) fn duplicate_key(path: &[Key], i: usize) -> Self {
        assert!(i < path.len());
        let key = &path[i];
        let repr = key
            .as_repr()
            .and_then(|key| key.as_raw().as_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| {
                #[cfg(feature = "display")]
                {
                    key.default_repr().as_raw().as_str().unwrap().to_owned()
                }
                #[cfg(not(feature = "display"))]
                {
                    format!("{:?}", key.get())
                }
            });
        Self::DuplicateKey {
            key: repr,
            table: Some(path[..i].to_vec()),
        }
    }

    pub(crate) fn extend_wrong_type(path: &[Key], i: usize, actual: &'static str) -> Self {
        assert!(i < path.len());
        Self::DottedKeyExtendWrongType {
            key: path[..=i].to_vec(),
            actual,
        }
    }
}

impl StdError for CustomError {
    fn description(&self) -> &'static str {
        "TOML parse error"
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CustomError::DuplicateKey { key, table } => {
                if let Some(table) = table {
                    if table.is_empty() {
                        write!(f, "duplicate key `{key}` in document root")
                    } else {
                        let path = table.iter().map(|k| k.get()).collect::<Vec<_>>().join(".");
                        write!(f, "duplicate key `{key}` in table `{path}`")
                    }
                } else {
                    write!(f, "duplicate key `{key}`")
                }
            }
            CustomError::DottedKeyExtendWrongType { key, actual } => {
                let path = key.iter().map(|k| k.get()).collect::<Vec<_>>().join(".");
                write!(
                    f,
                    "dotted key `{path}` attempted to extend non-table type ({actual})"
                )
            }
            CustomError::OutOfRange => write!(f, "value is out of range"),
            CustomError::RecursionLimitExceeded => write!(f, "recursion limit exceeded"),
        }
    }
}

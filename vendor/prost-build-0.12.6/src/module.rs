use std::fmt;

use crate::ident::to_snake;

/// A Rust module path for a Protobuf package.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Module {
    components: Vec<String>,
}

impl Module {
    /// Construct a module path from an iterator of parts.
    pub fn from_parts<I>(parts: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        Self {
            components: parts.into_iter().map(|s| s.into()).collect(),
        }
    }

    /// Construct a module path from a Protobuf package name.
    ///
    /// Constituent parts are automatically converted to snake case in order to follow
    /// Rust module naming conventions.
    pub fn from_protobuf_package_name(name: &str) -> Self {
        Self {
            components: name
                .split('.')
                .filter(|s| !s.is_empty())
                .map(to_snake)
                .collect(),
        }
    }

    /// An iterator over the parts of the path.
    pub fn parts(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(|s| s.as_str())
    }

    #[must_use]
    #[inline(always)]
    pub(crate) fn starts_with(&self, needle: &[String]) -> bool
    where
        String: PartialEq,
    {
        self.components.starts_with(needle)
    }

    /// Format the module path into a filename for generated Rust code.
    ///
    /// If the module path is empty, `default` is used to provide the root of the filename.
    pub fn to_file_name_or(&self, default: &str) -> String {
        let mut root = if self.components.is_empty() {
            default.to_owned()
        } else {
            self.components.join(".")
        };

        root.push_str(".rs");

        root
    }

    /// The number of parts in the module's path.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Whether the module's path contains any components.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub(crate) fn part(&self, idx: usize) -> &str {
        self.components[idx].as_str()
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = self.parts();
        if let Some(first) = parts.next() {
            f.write_str(first)?;
        }
        for part in parts {
            f.write_str("::")?;
            f.write_str(part)?;
        }
        Ok(())
    }
}

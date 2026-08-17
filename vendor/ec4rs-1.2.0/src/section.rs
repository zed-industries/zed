use crate::glob::Glob;
use crate::{rawvalue::RawValue, Properties};

use std::path::Path;

// Glob internals aren't stable enough to safely implement PartialEq here.

/// One section of an EditorConfig file.
#[derive(Clone)]
pub struct Section {
    pattern: Glob,
    props: crate::Properties,
}

impl Section {
    /// Constrcts a new [`Section`] that applies to files matching the specified pattern.
    pub fn new(pattern: &str) -> Section {
        Section {
            pattern: Glob::new(pattern),
            props: crate::Properties::new(),
        }
    }
    /// Returns a shared reference to the internal [`Properties`] map.
    pub fn props(&self) -> &Properties {
        &self.props
    }
    /// Returns a mutable reference to the internal [`Properties`] map.
    pub fn props_mut(&mut self) -> &mut Properties {
        &mut self.props
    }
    /// Extracts the [`Properties`] map from `self`.
    pub fn into_props(self) -> Properties {
        self.props
    }
    /// Adds a property with the specified key, lowercasing the key.
    pub fn insert(&mut self, key: impl AsRef<str>, val: impl Into<RawValue>) {
        self.props
            .insert_raw_for_key(key.as_ref().to_lowercase(), val);
    }
    /// Returns true if and only if this section applies to a file at the specified path.
    pub fn applies_to(&self, path: impl AsRef<Path>) -> bool {
        self.pattern.matches(path.as_ref())
    }
}

impl crate::PropertiesSource for &Section {
    /// Adds this section's properties to a [`Properties`].
    ///
    /// This implementation is infallible.
    fn apply_to(
        self,
        props: &mut Properties,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::Error> {
        let path_ref = path.as_ref();
        if self.applies_to(path_ref) {
            let _ = self.props.apply_to(props, path_ref);
        }
        Ok(())
    }
}

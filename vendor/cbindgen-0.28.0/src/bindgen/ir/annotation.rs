/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

use crate::bindgen::config::{Config, Language};
use crate::bindgen::utilities::SynAttributeHelpers;

// A system for specifying properties on items. Annotations are
// given through document comments and parsed by this code.
//
// An annotation is in the form cbindgen:PROPERTY=VALUE
// Where PROPERTY depends on the item
// Where VALUE can be
//  * list - [Item1, Item2, Item3, ...]
//  * atom - Foo
//  * bool - true,false
// Examples:
//  * cbindgen:field-names=[mHandle, mNamespace]
//  * cbindgen:function-postfix=WR_DESTRUCTOR_SAFE

/// A value specified by an annotation.
#[derive(Debug, Clone)]
pub enum AnnotationValue {
    List(Vec<String>),
    Atom(Option<String>),
    Bool(bool),
}

/// A set of annotations specified by a document comment.
#[derive(Debug, Default, Clone)]
pub struct AnnotationSet {
    annotations: HashMap<String, AnnotationValue>,
    pub must_use: bool,
    pub deprecated: Option<String>,
}

pub enum DeprecatedNoteKind {
    Function,
    Struct,
    Enum,
    EnumVariant,
}

impl AnnotationSet {
    pub fn new() -> AnnotationSet {
        AnnotationSet {
            annotations: HashMap::new(),
            must_use: false,
            deprecated: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty() && !self.must_use
    }

    pub(crate) fn must_use(&self, config: &Config) -> bool {
        self.must_use && config.language != Language::Cython
    }

    pub(crate) fn deprecated_note<'c>(
        &self,
        config: &'c Config,
        kind: DeprecatedNoteKind,
    ) -> Option<Cow<'c, str>> {
        let note = self.deprecated.as_deref()?;

        if config.language == Language::Cython {
            return None;
        }

        if note.is_empty() {
            return Some(Cow::Borrowed(match kind {
                DeprecatedNoteKind::Enum => config.enumeration.deprecated.as_deref()?,
                DeprecatedNoteKind::EnumVariant => {
                    config.enumeration.deprecated_variant.as_deref()?
                }
                DeprecatedNoteKind::Function => config.function.deprecated.as_deref()?,
                DeprecatedNoteKind::Struct => config.structure.deprecated.as_deref()?,
            }));
        }

        let format = match kind {
            DeprecatedNoteKind::Enum => &config.enumeration.deprecated_with_note,
            DeprecatedNoteKind::EnumVariant => &config.enumeration.deprecated_variant_with_note,
            DeprecatedNoteKind::Function => &config.function.deprecated_with_note,
            DeprecatedNoteKind::Struct => &config.structure.deprecated_with_note,
        }
        .as_ref()?;
        Some(Cow::Owned(format.replace("{}", &format!("{:?}", note))))
    }

    pub fn load(attrs: &[syn::Attribute]) -> Result<AnnotationSet, String> {
        let lines = attrs.get_comment_lines();
        let lines: Vec<&str> = lines
            .iter()
            .filter_map(|line| {
                let line = line.trim_start();
                if !line.starts_with("cbindgen:") {
                    return None;
                }

                Some(line)
            })
            .collect();

        let must_use = attrs.has_attr_word("must_use");
        let deprecated = attrs.find_deprecated_note();
        let mut annotations = HashMap::new();

        // Look at each line for an annotation
        for line in lines {
            debug_assert!(line.starts_with("cbindgen:"));

            // Remove the "cbindgen:" prefix
            let annotation = &line[9..];

            // Split the annotation in two
            let parts: Vec<&str> = annotation.split('=').map(|x| x.trim()).collect();

            if parts.len() > 2 {
                return Err(format!("Couldn't parse {}.", line));
            }

            // Grab the name that this annotation is modifying
            let name = parts[0];

            // If the annotation only has a name, assume it's setting a bool flag
            if parts.len() == 1 {
                annotations.insert(name.to_string(), AnnotationValue::Bool(true));
                continue;
            }

            // Parse the value we're setting the name to
            let value = parts[1];

            if let Some(x) = parse_list(value) {
                annotations.insert(name.to_string(), AnnotationValue::List(x));
                continue;
            }
            if let Ok(x) = value.parse::<bool>() {
                annotations.insert(name.to_string(), AnnotationValue::Bool(x));
                continue;
            }
            annotations.insert(
                name.to_string(),
                if value.is_empty() {
                    AnnotationValue::Atom(None)
                } else {
                    AnnotationValue::Atom(Some(value.to_string()))
                },
            );
        }

        Ok(AnnotationSet {
            annotations,
            must_use,
            deprecated,
        })
    }

    /// Adds an annotation value if none is specified.
    pub fn add_default(&mut self, name: &str, value: AnnotationValue) {
        if let Entry::Vacant(e) = self.annotations.entry(name.to_string()) {
            e.insert(value);
        }
    }

    pub fn list(&self, name: &str) -> Option<Vec<String>> {
        match self.annotations.get(name) {
            Some(AnnotationValue::List(x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn atom(&self, name: &str) -> Option<Option<String>> {
        match self.annotations.get(name) {
            Some(AnnotationValue::Atom(x)) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn bool(&self, name: &str) -> Option<bool> {
        match self.annotations.get(name) {
            Some(AnnotationValue::Bool(x)) => Some(*x),
            _ => None,
        }
    }

    pub fn parse_atom<T>(&self, name: &str) -> Option<T>
    where
        T: Default + FromStr,
    {
        match self.annotations.get(name) {
            Some(AnnotationValue::Atom(x)) => Some(
                x.as_ref()
                    .map_or(T::default(), |y| y.parse::<T>().ok().unwrap()),
            ),
            _ => None,
        }
    }
}

/// Parse lists like "[x, y, z]". This is not implemented efficiently or well.
fn parse_list(list: &str) -> Option<Vec<String>> {
    if list.len() < 2 {
        return None;
    }

    match (list.chars().next(), list.chars().last()) {
        (Some('['), Some(']')) => Some(
            list[1..list.len() - 1]
                .split(',')
                .map(|x| x.trim().to_string())
                .collect(),
        ),
        _ => None,
    }
}

use crate::base::{Bytes, BytesCow, eq_case_insensitive};
use crate::errors::RewritingError;
use crate::html::escape_double_quotes_only;
use crate::parser::AttributeBuffer;
use crate::rewritable_units::Serialize;
use encoding_rs::Encoding;
use std::cell::OnceCell;
use std::fmt::{self, Debug};
use thiserror::Error;

/// An error that occurs when invalid value is provided for the attribute name.
#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum AttributeNameError {
    /// The provided value is empty.
    #[error("Attribute name can't be empty.")]
    Empty,

    /// The provided value contains a character that is forbidden by the HTML grammar in attribute
    /// names (e.g. `'='`).
    #[error("`{0}` character is forbidden in the attribute name")]
    ForbiddenCharacter(char),

    /// The provided value contains a character that can't be represented in the document's
    /// [`encoding`].
    ///
    /// [`encoding`]: ../struct.Settings.html#structfield.encoding
    #[error(
        "The attribute name contains a character that can't be represented in the document's character encoding."
    )]
    UnencodableCharacter,
}

/// An attribute of an [`Element`].
///
/// This is an immutable representation of an attribute. To modify element's attributes use
/// approriate [`Element`]'s methods.
///
/// [`Element`]: struct.Element.html
pub struct Attribute<'i> {
    name: BytesCow<'i>,
    value: BytesCow<'i>,
    raw: Option<Bytes<'i>>,
    encoding: &'static Encoding,
}

impl<'i> Attribute<'i> {
    #[inline]
    #[must_use]
    const fn new(
        name: BytesCow<'i>,
        value: BytesCow<'i>,
        raw: Bytes<'i>,
        encoding: &'static Encoding,
    ) -> Self {
        Attribute {
            name,
            value,
            raw: Some(raw),
            encoding,
        }
    }

    fn name_from_string(
        name: String,
        encoding: &'static Encoding,
    ) -> Result<BytesCow<'static>, AttributeNameError> {
        if name.is_empty() {
            Err(AttributeNameError::Empty)
        } else if let Some(ch) = name.as_bytes().iter().copied().find(|&ch| {
            matches!(
                ch,
                b' ' | b'\n' | b'\r' | b'\t' | b'\x0C' | b'/' | b'>' | b'='
            )
        }) {
            Err(AttributeNameError::ForbiddenCharacter(ch as char))
        } else {
            // NOTE: if character can't be represented in the given
            // encoding then encoding_rs replaces it with a numeric
            // character reference. Character references are not
            // supported in attribute names, so we need to bail.
            BytesCow::owned_from_str_without_replacements(name, encoding)
                .map_err(|_| AttributeNameError::UnencodableCharacter)
        }
    }

    /// Returns the name of the attribute, always ASCII lowercased.
    #[inline]
    #[must_use]
    pub fn name(&self) -> String {
        self.name.as_lowercase_string(self.encoding)
    }

    /// Returns the name of the attribute, preserving its case.
    #[inline]
    #[must_use]
    pub fn name_preserve_case(&self) -> String {
        self.name.as_string(self.encoding)
    }

    /// Returns the value of the attribute. The value may have HTML/XML entities.
    #[inline]
    #[must_use]
    pub fn value(&self) -> String {
        self.value.as_string(self.encoding)
    }

    #[inline]
    fn set_value(&mut self, value: &str) {
        self.value = BytesCow::owned_from_str(value, self.encoding);
        self.raw = None;
    }
}

impl Serialize for &Attribute<'_> {
    #[inline]
    fn into_bytes(self, output_handler: &mut dyn FnMut(&[u8])) -> Result<(), RewritingError> {
        if let Some(raw) = self.raw.as_ref() {
            output_handler(raw);
        } else {
            output_handler(&self.name);
            output_handler(b"=\"");
            escape_double_quotes_only(self.value.as_ref(), output_handler);
            output_handler(b"\"");
        }
        Ok(())
    }
}

impl Debug for Attribute<'_> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

pub(crate) struct Attributes<'i> {
    input: &'i Bytes<'i>,
    attribute_buffer: &'i AttributeBuffer,
    items: OnceCell<Vec<Attribute<'i>>>,
    pub(crate) encoding: &'static Encoding,
}

impl<'i> Attributes<'i> {
    #[inline]
    #[must_use]
    pub(super) fn new(
        input: &'i Bytes<'i>,
        attribute_buffer: &'i AttributeBuffer,
        encoding: &'static Encoding,
    ) -> Self {
        Attributes {
            input,
            attribute_buffer,
            items: OnceCell::default(),
            encoding,
        }
    }

    pub(crate) fn map_attribute<R>(
        &self,
        name: &str,
        map: impl Fn(&Attribute<'_>) -> R,
    ) -> Option<R> {
        let name = Attribute::name_from_string(name.to_ascii_lowercase(), self.encoding).ok()?;
        let check = move |attr: &Attribute<'_>| {
            if eq_case_insensitive(&attr.name.as_ref(), &name.as_ref()) {
                Some(map(attr))
            } else {
                None
            }
        };
        match self.items.get() {
            Some(items) => items.iter().find_map(check),
            None => self.iter_attrs().find_map(|a| check(&a)),
        }
    }

    #[inline(never)]
    pub(crate) fn get_attribute(&self, name: &str) -> Option<String> {
        self.map_attribute(name, |attr| attr.value())
    }

    #[inline(never)]
    pub(crate) fn has_attribute(&self, name: &str) -> bool {
        self.map_attribute(name, |_| true).unwrap_or(false)
    }

    /// Adds or replaces the attribute. The value may have HTML/XML entities.
    ///
    /// Quotes will be escaped if needed. Other entities won't be changed.
    pub fn set_attribute(
        &mut self,
        name: &str,
        value: &str,
        encoding: &'static Encoding,
    ) -> Result<(), AttributeNameError> {
        let name = Attribute::name_from_string(name.to_ascii_lowercase(), encoding)?;
        let items = self.as_mut_vec();
        match items
            .iter_mut()
            .find(|attr| eq_case_insensitive(&attr.name.as_ref(), &name.as_ref()))
        {
            Some(attr) => attr.set_value(value),
            None => {
                items.push(Attribute {
                    name,
                    value: BytesCow::owned_from_str(value, encoding),
                    raw: None,
                    encoding,
                });
            }
        }

        Ok(())
    }

    pub fn remove_attribute(&mut self, name: &str) -> bool {
        let Ok(name) = Attribute::name_from_string(name.to_ascii_lowercase(), self.encoding) else {
            return false;
        };
        let items = self.as_mut_vec();
        let len_before = items.len();
        items.retain(|attr| !eq_case_insensitive(&attr.name.as_ref(), &name.as_ref()));
        len_before != items.len()
    }

    pub fn is_empty(&self) -> bool {
        // check without materializing items
        self.items
            .get()
            .map(|items| items.is_empty())
            .unwrap_or(self.attribute_buffer.is_empty())
    }

    fn iter_attrs(&self) -> impl Iterator<Item = Attribute<'i>> {
        let cant_fail = || {
            debug_assert!(false);
            Bytes::default()
        };
        self.attribute_buffer.iter().map(move |a| {
            Attribute::new(
                self.input
                    .opt_slice(Some(a.name))
                    .unwrap_or_else(cant_fail)
                    .into(),
                self.input
                    .opt_slice(Some(a.value))
                    .unwrap_or_else(cant_fail)
                    .into(),
                self.input
                    .opt_slice(Some(a.raw_range))
                    .unwrap_or_else(cant_fail),
                self.encoding,
            )
        })
    }

    #[inline(never)]
    fn init_items(&self) -> Vec<Attribute<'i>> {
        self.iter_attrs().collect()
    }

    pub(crate) fn to_slice(&self) -> &[Attribute<'i>] {
        self.items.get_or_init(|| self.init_items())
    }

    #[inline]
    fn as_mut_vec(&mut self) -> &mut Vec<Attribute<'i>> {
        if self.items.get().is_none() {
            // `get_mut_or_init` is unstable and has a pointless re-entrancy check
            let cell = OnceCell::new();
            let _ = cell.set(self.init_items());
            self.items = cell; // optimizes out get_mut()
        }
        self.items.get_mut().unwrap_or_else(|| unreachable!())
    }

    #[cfg(test)]
    pub(crate) const fn raw_attributes(&self) -> (&'i Bytes<'i>, &'i AttributeBuffer) {
        (self.input, self.attribute_buffer)
    }
}

impl Serialize for &mut Attributes<'_> {
    #[inline]
    fn into_bytes(self, output_handler: &mut dyn FnMut(&[u8])) -> Result<(), RewritingError> {
        for attr in self.as_mut_vec() {
            output_handler(b" ");
            attr.into_bytes(output_handler)?;
        }
        Ok(())
    }
}

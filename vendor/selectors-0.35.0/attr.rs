/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::parser::SelectorImpl;
use cssparser::ToCss;
use std::fmt;

#[cfg(feature = "to_shmem")]
use to_shmem_derive::ToShmem;

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "to_shmem", derive(ToShmem))]
#[cfg_attr(feature = "to_shmem", shmem(no_bounds))]
pub struct AttrSelectorWithOptionalNamespace<Impl: SelectorImpl> {
    #[cfg_attr(feature = "to_shmem", shmem(field_bound))]
    pub namespace: Option<NamespaceConstraint<(Impl::NamespacePrefix, Impl::NamespaceUrl)>>,
    #[cfg_attr(feature = "to_shmem", shmem(field_bound))]
    pub local_name: Impl::LocalName,
    pub local_name_lower: Impl::LocalName,
    #[cfg_attr(feature = "to_shmem", shmem(field_bound))]
    pub operation: ParsedAttrSelectorOperation<Impl::AttrValue>,
}

impl<Impl: SelectorImpl> AttrSelectorWithOptionalNamespace<Impl> {
    pub fn namespace(&self) -> Option<NamespaceConstraint<&Impl::NamespaceUrl>> {
        self.namespace.as_ref().map(|ns| match ns {
            NamespaceConstraint::Any => NamespaceConstraint::Any,
            NamespaceConstraint::Specific((_, ref url)) => NamespaceConstraint::Specific(url),
        })
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "to_shmem", derive(ToShmem))]
pub enum NamespaceConstraint<NamespaceUrl> {
    Any,

    /// Empty string for no namespace
    Specific(NamespaceUrl),
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "to_shmem", derive(ToShmem))]
pub enum ParsedAttrSelectorOperation<AttrValue> {
    Exists,
    WithValue {
        operator: AttrSelectorOperator,
        case_sensitivity: ParsedCaseSensitivity,
        value: AttrValue,
    },
}

#[derive(Clone, Eq, PartialEq)]
pub enum AttrSelectorOperation<AttrValue> {
    Exists,
    WithValue {
        operator: AttrSelectorOperator,
        case_sensitivity: CaseSensitivity,
        value: AttrValue,
    },
}

impl<AttrValue> AttrSelectorOperation<AttrValue> {
    pub fn eval_str(&self, element_attr_value: &str) -> bool
    where
        AttrValue: AsRef<str>,
    {
        match *self {
            AttrSelectorOperation::Exists => true,
            AttrSelectorOperation::WithValue {
                operator,
                case_sensitivity,
                ref value,
            } => operator.eval_str(element_attr_value, value.as_ref(), case_sensitivity),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "to_shmem", derive(ToShmem))]
pub enum AttrSelectorOperator {
    Equal,
    Includes,
    DashMatch,
    Prefix,
    Substring,
    Suffix,
}

impl ToCss for AttrSelectorOperator {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        // https://drafts.csswg.org/cssom/#serializing-selectors
        // See "attribute selector".
        dest.write_str(match *self {
            AttrSelectorOperator::Equal => "=",
            AttrSelectorOperator::Includes => "~=",
            AttrSelectorOperator::DashMatch => "|=",
            AttrSelectorOperator::Prefix => "^=",
            AttrSelectorOperator::Substring => "*=",
            AttrSelectorOperator::Suffix => "$=",
        })
    }
}

impl AttrSelectorOperator {
    pub fn eval_str(
        self,
        element_attr_value: &str,
        attr_selector_value: &str,
        case_sensitivity: CaseSensitivity,
    ) -> bool {
        let e = element_attr_value.as_bytes();
        let s = attr_selector_value.as_bytes();
        let case = case_sensitivity;
        match self {
            AttrSelectorOperator::Equal => case.eq(e, s),
            AttrSelectorOperator::Prefix => {
                !s.is_empty() && e.len() >= s.len() && case.eq(&e[..s.len()], s)
            },
            AttrSelectorOperator::Suffix => {
                !s.is_empty() && e.len() >= s.len() && case.eq(&e[(e.len() - s.len())..], s)
            },
            AttrSelectorOperator::Substring => {
                !s.is_empty() && case.contains(element_attr_value, attr_selector_value)
            },
            AttrSelectorOperator::Includes => {
                !s.is_empty()
                    && element_attr_value
                        .split(SELECTOR_WHITESPACE)
                        .any(|part| case.eq(part.as_bytes(), s))
            },
            AttrSelectorOperator::DashMatch => {
                case.eq(e, s) || (e.get(s.len()) == Some(&b'-') && case.eq(&e[..s.len()], s))
            },
        }
    }
}

/// The definition of whitespace per CSS Selectors Level 3 § 4.
pub static SELECTOR_WHITESPACE: &[char] = &[' ', '\t', '\n', '\r', '\x0C'];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "to_shmem", derive(ToShmem))]
pub enum ParsedCaseSensitivity {
    /// 's' was specified.
    ExplicitCaseSensitive,
    /// 'i' was specified.
    AsciiCaseInsensitive,
    /// No flags were specified and HTML says this is a case-sensitive attribute.
    CaseSensitive,
    /// No flags were specified and HTML says this is a case-insensitive attribute.
    AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaseSensitivity {
    CaseSensitive,
    AsciiCaseInsensitive,
}

impl CaseSensitivity {
    pub fn eq(self, a: &[u8], b: &[u8]) -> bool {
        match self {
            CaseSensitivity::CaseSensitive => a == b,
            CaseSensitivity::AsciiCaseInsensitive => a.eq_ignore_ascii_case(b),
        }
    }

    pub fn contains(self, haystack: &str, needle: &str) -> bool {
        match self {
            CaseSensitivity::CaseSensitive => haystack.contains(needle),
            CaseSensitivity::AsciiCaseInsensitive => {
                if let Some((&n_first_byte, n_rest)) = needle.as_bytes().split_first() {
                    haystack.bytes().enumerate().any(|(i, byte)| {
                        if !byte.eq_ignore_ascii_case(&n_first_byte) {
                            return false;
                        }
                        let after_this_byte = &haystack.as_bytes()[i + 1..];
                        match after_this_byte.get(..n_rest.len()) {
                            None => false,
                            Some(haystack_slice) => haystack_slice.eq_ignore_ascii_case(n_rest),
                        }
                    })
                } else {
                    // any_str.contains("") == true,
                    // though these cases should be handled with *NeverMatches and never go here.
                    true
                }
            },
        }
    }
}

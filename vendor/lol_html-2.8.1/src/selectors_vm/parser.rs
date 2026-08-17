use super::SelectorError;
use crate::html::Namespace;
use cssparser::{Parser as CssParser, ParserInput, ToCss};
use selectors::parser::{
    Combinator, Component, NonTSPseudoClass, Parser, PseudoElement, SelectorImpl, SelectorList,
    SelectorParseErrorKind,
};
use selectors::parser::{NthSelectorData, ParseRelative};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct SelectorImplDescriptor;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct CssString(Box<str>);

impl<'a> From<&'a str> for CssString {
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

impl CssString {
    pub fn to_boxed_slice(&self) -> Box<str> {
        self.0.clone()
    }
}

impl std::ops::Deref for CssString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl ToCss for CssString {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str(&self.0)
    }
}

impl precomputed_hash::PrecomputedHash for CssString {
    fn precomputed_hash(&self) -> u32 {
        if let Some(v) = self.0.as_bytes().get(..4) {
            let mut tmp = [0u8; 4];
            tmp.copy_from_slice(v);
            u32::from_ne_bytes(tmp)
        } else {
            0
        }
    }
}

impl precomputed_hash::PrecomputedHash for Namespace {
    fn precomputed_hash(&self) -> u32 {
        *self as u32
    }
}

impl SelectorImpl for SelectorImplDescriptor {
    type AttrValue = CssString;
    type Identifier = CssString;
    type LocalName = CssString;
    type NamespacePrefix = CssString;
    type NamespaceUrl = Namespace;
    type BorrowedNamespaceUrl = Namespace;
    type BorrowedLocalName = CssString;

    type NonTSPseudoClass = NonTSPseudoClassStub;
    type PseudoElement = PseudoElementStub;

    type ExtraMatchingData<'unused> = ();
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub(crate) enum PseudoElementStub {}

impl ToCss for PseudoElementStub {
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        #[allow(clippy::uninhabited_references)]
        match *self {}
    }
}

impl PseudoElement for PseudoElementStub {
    type Impl = SelectorImplDescriptor;
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub(crate) enum NonTSPseudoClassStub {}

impl NonTSPseudoClass for NonTSPseudoClassStub {
    type Impl = SelectorImplDescriptor;

    fn is_active_or_hover(&self) -> bool {
        #[allow(clippy::uninhabited_references)]
        match *self {}
    }

    fn is_user_action_state(&self) -> bool {
        #[allow(clippy::uninhabited_references)]
        match *self {}
    }
}

impl ToCss for NonTSPseudoClassStub {
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        #[allow(clippy::uninhabited_references)]
        match *self {}
    }
}

#[allow(dead_code)]
struct SelectorsParser;

impl SelectorsParser {
    fn validate_component(
        component: &Component<SelectorImplDescriptor>,
        inside_any_negation: bool,
    ) -> Result<(), SelectorError> {
        // NOTE: always use explicit variants in this match, so we
        // get compile-time error if new component types were added to
        // the parser.
        #[deny(clippy::wildcard_enum_match_arm)]
        match component {
            Component::Combinator(combinator) if inside_any_negation => {
                // the stack vm can only support positive matches
                Err(SelectorError::UnsupportedPseudoClassOrElement)
            }
            Component::Combinator(combinator) => match combinator {
                // Supported
                Combinator::Child | Combinator::Descendant => Ok(()),

                // Unsupported
                Combinator::Part => Err(SelectorError::UnsupportedPseudoClassOrElement),
                Combinator::NextSibling => Err(SelectorError::UnsupportedCombinator('+')),
                Combinator::LaterSibling => Err(SelectorError::UnsupportedCombinator('~')),
                Combinator::PseudoElement | Combinator::SlotAssignment => {
                    Err(SelectorError::UnsupportedPseudoClassOrElement)
                }
            },

            // Supported
            Component::LocalName(_)
            | Component::ExplicitUniversalType
            | Component::ExplicitAnyNamespace
            | Component::ExplicitNoNamespace
            | Component::ID(_)
            | Component::Class(_)
            | Component::AttributeInNoNamespaceExists { .. }
            | Component::AttributeInNoNamespace { .. } => Ok(()),

            Component::Nth(data) => Self::validate_nth(data),

            Component::Negation(selectors) => Self::validate_selectors(selectors.slice(), true),

            // Unsupported
            Component::Empty
            | Component::Part(_)
            | Component::Host(_)
            | Component::Is(_)
            | Component::Root
            | Component::Scope
            | Component::Where(_)
            | Component::PseudoElement(_)
            | Component::NonTSPseudoClass(_)
            | Component::Slotted(_) => Err(SelectorError::UnsupportedPseudoClassOrElement),

            // This is for `:nth-child(n of .selector)`,
            // which is subtly different from `.selector:nth-of-type(n)`
            Component::NthOf(_) => Err(SelectorError::UnsupportedPseudoClassOrElement),

            Component::DefaultNamespace(_)
            | Component::Namespace(_, _)
            | Component::AttributeOther(_) => Err(SelectorError::NamespacedSelector),

            Component::ImplicitScope
            | Component::ParentSelector
            | Component::RelativeSelectorAnchor
            | Component::Has(_) => Err(SelectorError::UnsupportedSyntax),
            Component::Invalid(_) => Err(SelectorError::UnexpectedToken),
        }
    }

    fn validate(
        selector_list: SelectorList<SelectorImplDescriptor>,
    ) -> Result<SelectorList<SelectorImplDescriptor>, SelectorError> {
        Self::validate_selectors(selector_list.slice(), false)?;
        Ok(selector_list)
    }

    fn validate_selectors(
        selector_list: &[selectors::parser::Selector<SelectorImplDescriptor>],
        inside_any_negation: bool,
    ) -> Result<(), SelectorError> {
        for selector in selector_list {
            for component in selector.iter_raw_match_order() {
                Self::validate_component(component, inside_any_negation)?;
            }
        }
        Ok(())
    }

    fn validate_nth(nth: &NthSelectorData) -> Result<(), SelectorError> {
        match nth.ty {
            selectors::parser::NthType::Child | selectors::parser::NthType::OfType => Ok(()),
            selectors::parser::NthType::LastChild
            | selectors::parser::NthType::OnlyChild
            | selectors::parser::NthType::LastOfType
            | selectors::parser::NthType::OnlyOfType => {
                Err(SelectorError::UnsupportedPseudoClassOrElement)
            }
        }
    }

    #[inline]
    pub fn parse(selector: &str) -> Result<SelectorList<SelectorImplDescriptor>, SelectorError> {
        let mut input = ParserInput::new(selector);
        let mut css_parser = CssParser::new(&mut input);

        SelectorList::parse(&Self, &mut css_parser, ParseRelative::No)
            .map_err(SelectorError::from)
            .and_then(Self::validate)
    }
}

impl<'i> Parser<'i> for SelectorsParser {
    type Impl = SelectorImplDescriptor;
    type Error = SelectorParseErrorKind<'i>;

    fn parse_nth_child_of(&self) -> bool {
        false
    }
}

/// Parsed CSS selector.
///
/// Parsed selector can be used for different [element content handlers] without a necessity
/// to re-parse CSS selector string for each of them.
///
/// # Example
///
/// The structure implements the [`FromStr`] trait, so it can be constructed through
/// [`str`]'s [`parse`] method.
///
/// ```
/// use lol_html::Selector;
///
/// let selector: Selector = "#foo".parse().unwrap();
/// ```
///
/// # Supported selector
///
/// Currently the rewriter supports the following CSS selectors:
///
/// Pattern                        | Represents                                                                                                                  |
/// ------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
/// `*`                            | any element                                                                                                                 |
/// `E`                            | any element of type `E`                                                                                                     |
/// `E:nth-child(n)`               | an `E` element, the n-th child of its parent                                                                                |
/// `E:first-child`                | an `E` element, first child of its parent                                                                                   |
/// `E:nth-of-type(n)`             | an `E` element, the n-th sibling of its type                                                                                |
/// `E:first-of-type`              | an `E` element, first sibling of its type                                                                                   |
/// `E:not(s)`                     | an `E` element that does not match either compound selector `s`                                                             |
/// `E.warning`                    | an `E` element belonging to the class `warning`                                                                             |
/// `E#myid`                       | an `E` element with `ID` equal to `"myid"`.                                                                                 |
/// `E[foo]`                       | an `E` element with a `foo` attribute                                                                                       |
/// `E[foo="bar"]`                 | an `E` element whose foo attribute value is exactly equal to `"bar"`                                                        |
/// `E[foo="bar" i]`               | an `E` element whose foo attribute value is exactly equal to any (ASCII-range) case-permutation of `"bar"`                  |
/// `E[foo="bar" s]`               | an `E` element whose foo attribute value is exactly and case-sensitively equal to `"bar"`                                   |
/// `E[foo~="bar"]`                | an `E` element whose foo attribute value is a list of whitespace-separated values, one of which is exactly equal to `"bar"` |
/// `E[foo^="bar"]`                | an `E` element whose foo attribute value begins exactly with the string `"bar"`                                             |
/// `E[foo$="bar"]`                | an `E` element whose foo attribute value ends exactly with the string `"bar"`                                               |
/// `E[foo*="bar"]`                | an `E` element whose foo attribute value contains the substring `"bar"`                                                     |
/// <code>E\[foo&#124;="en"\]</code> | an `E` element whose foo attribute value is a hyphen-separated list of values beginning with `"en"`                         |
/// `E F`                          | an `F` element descendant of an `E` element                                                                                 |
/// `E > F`                        | an `F` element child of an `E` element                                                                                      |
///
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
/// [`parse`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
/// [element content handlers]: struct.Settings.html#structfield.element_content_handlers
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
#[derive(Clone, Debug)]
pub struct Selector(pub(crate) SelectorList<SelectorImplDescriptor>);

impl FromStr for Selector {
    type Err = SelectorError;

    #[inline]
    fn from_str(selector: &str) -> Result<Self, Self::Err> {
        Ok(Self(SelectorsParser::parse(selector)?))
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Visitor traits for selectors.

#![deny(missing_docs)]

use crate::attr::NamespaceConstraint;
use crate::parser::{Combinator, Component, RelativeSelector, Selector, SelectorImpl};
use bitflags::bitflags;

/// A trait to visit selector properties.
///
/// All the `visit_foo` methods return a boolean indicating whether the
/// traversal should continue or not.
pub trait SelectorVisitor: Sized {
    /// The selector implementation this visitor wants to visit.
    type Impl: SelectorImpl;

    /// Visit an attribute selector that may match (there are other selectors
    /// that may never match, like those containing whitespace or the empty
    /// string).
    fn visit_attribute_selector(
        &mut self,
        _namespace: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        _local_name: &<Self::Impl as SelectorImpl>::LocalName,
        _local_name_lower: &<Self::Impl as SelectorImpl>::LocalName,
    ) -> bool {
        true
    }

    /// Visit a simple selector.
    fn visit_simple_selector(&mut self, _: &Component<Self::Impl>) -> bool {
        true
    }

    /// Visit a nested relative selector list. The caller is responsible to call visit
    /// into the internal selectors if / as needed.
    ///
    /// The default implementation skips it altogether.
    fn visit_relative_selector_list(&mut self, _list: &[RelativeSelector<Self::Impl>]) -> bool {
        true
    }

    /// Visit a nested selector list. The caller is responsible to call visit
    /// into the internal selectors if / as needed.
    ///
    /// The default implementation does this.
    fn visit_selector_list(
        &mut self,
        _list_kind: SelectorListKind,
        list: &[Selector<Self::Impl>],
    ) -> bool {
        for nested in list {
            if !nested.visit(self) {
                return false;
            }
        }
        true
    }

    /// Visits a complex selector.
    ///
    /// Gets the combinator to the right of the selector, or `None` if the
    /// selector is the rightmost one.
    fn visit_complex_selector(&mut self, _combinator_to_right: Option<Combinator>) -> bool {
        true
    }
}

bitflags! {
    /// The kinds of components the visitor is visiting the selector list of, if any
    #[derive(Clone, Copy, Default)]
    pub struct SelectorListKind: u8 {
        /// The visitor is inside :not(..)
        const NEGATION = 1 << 0;
        /// The visitor is inside :is(..)
        const IS = 1 << 1;
        /// The visitor is inside :where(..)
        const WHERE = 1 << 2;
        /// The visitor is inside :nth-child(.. of <selector list>) or
        /// :nth-last-child(.. of <selector list>)
        const NTH_OF = 1 << 3;
        /// The visitor is inside :has(..)
        const HAS = 1 << 4;
    }
}

impl SelectorListKind {
    /// Construct a SelectorListKind for the corresponding component.
    pub fn from_component<Impl: SelectorImpl>(component: &Component<Impl>) -> Self {
        match component {
            Component::Negation(_) => SelectorListKind::NEGATION,
            Component::Is(_) => SelectorListKind::IS,
            Component::Where(_) => SelectorListKind::WHERE,
            Component::NthOf(_) => SelectorListKind::NTH_OF,
            _ => SelectorListKind::empty(),
        }
    }

    /// Whether the visitor is inside :not(..)
    pub fn in_negation(&self) -> bool {
        self.intersects(SelectorListKind::NEGATION)
    }

    /// Whether the visitor is inside :is(..)
    pub fn in_is(&self) -> bool {
        self.intersects(SelectorListKind::IS)
    }

    /// Whether the visitor is inside :where(..)
    pub fn in_where(&self) -> bool {
        self.intersects(SelectorListKind::WHERE)
    }

    /// Whether the visitor is inside :nth-child(.. of <selector list>) or
    /// :nth-last-child(.. of <selector list>)
    pub fn in_nth_of(&self) -> bool {
        self.intersects(SelectorListKind::NTH_OF)
    }

    /// Whether the visitor is inside :has(..)
    pub fn in_has(&self) -> bool {
        self.intersects(SelectorListKind::HAS)
    }

    /// Whether this nested selector is relevant for nth-of dependencies.
    pub fn relevant_to_nth_of_dependencies(&self) -> bool {
        // Order of nesting for `:has` and `:nth-child(.. of ..)` doesn't matter, because:
        // * `:has(:nth-child(.. of ..))`: The location of the anchoring element is
        //   independent from where `:nth-child(.. of ..)` is applied.
        // * `:nth-child(.. of :has(..))`: Invalidations inside `:has` must first use the
        //   `:has` machinary to find the anchor, then carry out the remaining invalidation.
        self.in_nth_of() && !self.in_has()
    }
}

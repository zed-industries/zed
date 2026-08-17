/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Traits that nodes must implement. Breaks the otherwise-cyclic dependency
//! between layout and style.

use crate::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use crate::bloom::BloomFilter;
use crate::matching::{ElementSelectorFlags, MatchingContext};
use crate::parser::SelectorImpl;
use std::fmt::Debug;
use std::ptr::NonNull;

/// Opaque representation of an Element, for identity comparisons.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpaqueElement(NonNull<()>);

unsafe impl Send for OpaqueElement {}
// This should be safe given that we do not provide a way to recover
// the original reference.
unsafe impl Sync for OpaqueElement {}

impl OpaqueElement {
    /// Creates a new OpaqueElement from an arbitrarily-typed pointer.
    pub fn new<T>(ptr: &T) -> Self {
        unsafe {
            OpaqueElement(NonNull::new_unchecked(
                ptr as *const T as *const () as *mut (),
            ))
        }
    }

    /// Creates a new OpaqueElement from a type-erased non-null pointer
    pub fn from_non_null_ptr(ptr: NonNull<()>) -> Self {
        Self(ptr)
    }

    /// Returns a const ptr to the contained reference. Unsafe especially
    /// since Element can be recovered and potentially-mutated.
    pub unsafe fn as_const_ptr<T>(&self) -> *const T {
        self.0.as_ptr() as *const T
    }
}

pub trait Element: Sized + Clone + Debug {
    type Impl: SelectorImpl;

    /// Converts self into an opaque representation.
    fn opaque(&self) -> OpaqueElement;

    fn parent_element(&self) -> Option<Self>;

    /// Whether the parent node of this element is a shadow root.
    fn parent_node_is_shadow_root(&self) -> bool;

    /// The host of the containing shadow root, if any.
    fn containing_shadow_host(&self) -> Option<Self>;

    /// The parent of a given pseudo-element, after matching a pseudo-element
    /// selector.
    ///
    /// This is guaranteed to be called in a pseudo-element.
    fn pseudo_element_originating_element(&self) -> Option<Self> {
        debug_assert!(self.is_pseudo_element());
        self.parent_element()
    }

    /// Whether we're matching on a pseudo-element.
    fn is_pseudo_element(&self) -> bool;

    /// Skips non-element nodes
    fn prev_sibling_element(&self) -> Option<Self>;

    /// Skips non-element nodes
    fn next_sibling_element(&self) -> Option<Self>;

    /// Skips non-element nodes
    fn first_element_child(&self) -> Option<Self>;

    fn is_html_element_in_html_document(&self) -> bool;

    fn has_local_name(&self, local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName) -> bool;

    /// Empty string for no namespace
    fn has_namespace(&self, ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl) -> bool;

    /// Whether this element and the `other` element have the same local name and namespace.
    fn is_same_type(&self, other: &Self) -> bool;

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>,
    ) -> bool;

    fn has_attr_in_no_namespace(
        &self,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
    ) -> bool {
        self.attr_matches(
            &NamespaceConstraint::Specific(&crate::parser::namespace_empty_string::<Self::Impl>()),
            local_name,
            &AttrSelectorOperation::Exists,
        )
    }

    fn match_non_ts_pseudo_class(
        &self,
        pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        context: &mut MatchingContext<Self::Impl>,
    ) -> bool;

    fn match_pseudo_element(
        &self,
        pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        context: &mut MatchingContext<Self::Impl>,
    ) -> bool;

    /// Sets selector flags on the elemnt itself or the parent, depending on the
    /// flags, which indicate what kind of work may need to be performed when
    /// DOM state changes.
    fn apply_selector_flags(&self, flags: ElementSelectorFlags);

    /// Whether this element is a `link`.
    fn is_link(&self) -> bool;

    /// Returns whether the element is an HTML <slot> element.
    fn is_html_slot_element(&self) -> bool;

    /// Returns the assigned <slot> element this element is assigned to.
    ///
    /// Necessary for the `::slotted` pseudo-class.
    fn assigned_slot(&self) -> Option<Self> {
        None
    }

    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool;

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool;

    fn has_custom_state(&self, name: &<Self::Impl as SelectorImpl>::Identifier) -> bool;

    /// Returns the mapping from the `exportparts` attribute in the reverse
    /// direction, that is, in an outer-tree -> inner-tree direction.
    fn imported_part(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl>::Identifier>;

    fn is_part(&self, name: &<Self::Impl as SelectorImpl>::Identifier) -> bool;

    /// Returns whether this element matches `:empty`.
    ///
    /// That is, whether it does not contain any child element or any non-zero-length text node.
    /// See http://dev.w3.org/csswg/selectors-3/#empty-pseudo
    fn is_empty(&self) -> bool;

    /// Returns whether this element matches `:root`,
    /// i.e. whether it is the root element of a document.
    ///
    /// Note: this can be false even if `.parent_element()` is `None`
    /// if the parent node is a `DocumentFragment`.
    fn is_root(&self) -> bool;

    /// Returns whether this element should ignore matching nth child
    /// selector.
    fn ignores_nth_child_selectors(&self) -> bool {
        false
    }

    /// Add hashes unique to this element to the given filter, returning true
    /// if any got added.
    fn add_element_unique_hashes(&self, filter: &mut BloomFilter) -> bool;
}

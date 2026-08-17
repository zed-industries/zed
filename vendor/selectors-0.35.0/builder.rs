/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Helper module to build up a selector safely and efficiently.
//!
//! Our selector representation is designed to optimize matching, and has
//! several requirements:
//! * All simple selectors and combinators are stored inline in the same buffer as Component
//!   instances.
//! * We store the top-level compound selectors from right to left, i.e. in matching order.
//! * We store the simple selectors for each combinator from left to right, so that we match the
//!   cheaper simple selectors first.
//!
//! For example, the selector:
//!
//!   .bar:hover > .baz:nth-child(2) + .qux
//!
//! Gets stored as:
//!
//!   [.qux,  + , .baz, :nth-child(2),  > , .bar, :hover]
//!
//! Meeting all these constraints without extra memmove traffic during parsing is non-trivial. This
//! module encapsulates those details and presents an easy-to-use API for the parser.

use crate::parser::{
    Combinator, Component, ParseRelative, RelativeSelector, Selector, SelectorData, SelectorImpl,
};
use crate::sink::Push;
use bitflags::bitflags;
use derive_more::{Add, AddAssign};
use servo_arc::Arc;
use smallvec::SmallVec;
use std::cmp;
use std::slice;

#[cfg(feature = "to_shmem")]
use to_shmem_derive::ToShmem;

/// Top-level SelectorBuilder struct. This should be stack-allocated by the consumer and never
/// moved (because it contains a lot of inline data that would be slow to memmove).
///
/// After instantiation, callers may call the push_simple_selector() and push_combinator() methods
/// to append selector data as it is encountered (from left to right). Once the process is
/// complete, callers should invoke build(), which transforms the contents of the SelectorBuilder
/// into a heap- allocated Selector and leaves the builder in a drained state.
#[derive(Debug)]
pub struct SelectorBuilder<Impl: SelectorImpl> {
    /// The entire sequence of components. We make this large because the result of parsing a
    /// selector is fed into a new Arc-ed allocation, so any spilled vec would be a wasted
    /// allocation. Also, Components are large enough that we don't have much cache locality
    /// benefit from reserving stack space for fewer of them.
    components: SmallVec<[Component<Impl>; 32]>,
    last_compound_start: Option<usize>,
}

impl<Impl: SelectorImpl> Push<Component<Impl>> for SelectorBuilder<Impl> {
    fn push(&mut self, value: Component<Impl>) {
        self.push_simple_selector(value);
    }
}

impl<Impl: SelectorImpl> SelectorBuilder<Impl> {
    /// Pushes a simple selector onto the current compound selector.
    #[inline(always)]
    pub fn push_simple_selector(&mut self, ss: Component<Impl>) {
        debug_assert!(!ss.is_combinator());
        self.components.push(ss);
    }

    /// Completes the current compound selector and starts a new one, delimited by the given
    /// combinator.
    #[inline(always)]
    pub fn push_combinator(&mut self, c: Combinator) {
        self.reverse_last_compound();
        self.components.push(Component::Combinator(c));
        self.last_compound_start = Some(self.components.len());
    }

    fn reverse_last_compound(&mut self) {
        let start = self.last_compound_start.unwrap_or(0);
        self.components[start..].reverse();
    }

    /// Returns true if combinators have ever been pushed to this builder.
    #[inline(always)]
    pub fn has_combinators(&self) -> bool {
        self.last_compound_start.is_some()
    }

    /// Consumes the builder, producing a Selector.
    #[inline(always)]
    pub fn build(&mut self, parse_relative: ParseRelative) -> SelectorData<Impl> {
        // Compute the specificity and flags.
        let sf = specificity_and_flags(
            self.components.iter(),
            /* for_nesting_parent = */ false,
        );
        self.build_with_specificity_and_flags(sf, parse_relative)
    }

    /// Builds with an explicit SpecificityAndFlags. This is separated from build() so that unit
    /// tests can pass an explicit specificity.
    #[inline(always)]
    pub(crate) fn build_with_specificity_and_flags(
        &mut self,
        mut spec: SpecificityAndFlags,
        parse_relative: ParseRelative,
    ) -> SelectorData<Impl> {
        let implicit_addition = match parse_relative {
            ParseRelative::ForNesting if !spec.flags.intersects(SelectorFlags::HAS_PARENT) => {
                Some((Component::ParentSelector, SelectorFlags::HAS_PARENT))
            },
            ParseRelative::ForScope
                if !spec
                    .flags
                    .intersects(SelectorFlags::HAS_SCOPE | SelectorFlags::HAS_PARENT) =>
            {
                Some((Component::ImplicitScope, SelectorFlags::HAS_SCOPE))
            },
            _ => None,
        };
        let implicit_selector_and_combinator;
        let implicit_selector = if let Some((component, flag)) = implicit_addition {
            spec.flags.insert(flag);
            implicit_selector_and_combinator =
                [Component::Combinator(Combinator::Descendant), component];
            &implicit_selector_and_combinator[..]
        } else {
            &[]
        };

        // As an optimization, for a selector without combinators, we can just keep the order
        // as-is.
        if self.last_compound_start.is_none() {
            return Arc::from_header_and_iter(
                spec,
                ExactChain(self.components.drain(..), implicit_selector.iter().cloned()),
            );
        }

        self.reverse_last_compound();
        Arc::from_header_and_iter(
            spec,
            ExactChain(
                self.components.drain(..).rev(),
                implicit_selector.iter().cloned(),
            ),
        )
    }
}

impl<Impl: SelectorImpl> Default for SelectorBuilder<Impl> {
    #[inline(always)]
    fn default() -> Self {
        SelectorBuilder {
            components: SmallVec::new(),
            last_compound_start: None,
        }
    }
}

// This is effectively a Chain<>, but Chain isn't an ExactSizeIterator, see
// https://github.com/rust-lang/rust/issues/34433
struct ExactChain<A, B>(A, B);

impl<A, B, Item> ExactSizeIterator for ExactChain<A, B>
where
    A: ExactSizeIterator<Item = Item>,
    B: ExactSizeIterator<Item = Item>,
{
    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}

impl<A, B, Item> Iterator for ExactChain<A, B>
where
    A: ExactSizeIterator<Item = Item>,
    B: ExactSizeIterator<Item = Item>,
{
    type Item = Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().or_else(|| self.1.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

/// Flags that indicate at which point of parsing a selector are we.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
#[cfg_attr(feature = "to_shmem", derive(ToShmem))]
pub(crate) struct SelectorFlags(u8);

bitflags! {
    impl SelectorFlags: u8 {
        const HAS_PSEUDO = 1 << 0;
        const HAS_SLOTTED = 1 << 1;
        const HAS_PART = 1 << 2;
        const HAS_PARENT = 1 << 3;
        const HAS_HOST = 1 << 4;
        const HAS_SCOPE = 1 << 5;
    }
}

impl core::fmt::Debug for SelectorFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.is_empty() {
            write!(f, "{:#x}", Self::empty().bits())
        } else {
            bitflags::parser::to_writer(self, f)
        }
    }
}

impl SelectorFlags {
    /// When you nest a pseudo-element with something like:
    ///
    ///   ::before { & { .. } }
    ///
    /// It is not supposed to work, because :is(::before) is invalid. We can't propagate the
    /// pseudo-flags from inner to outer selectors, to avoid breaking our invariants.
    pub(crate) fn forbidden_for_nesting() -> Self {
        Self::HAS_PSEUDO | Self::HAS_SLOTTED | Self::HAS_PART
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "to_shmem", derive(ToShmem))]
pub struct SpecificityAndFlags {
    /// There are two free bits here, since we use ten bits for each specificity
    /// kind (id, class, element).
    pub(crate) specificity: u32,
    /// There's padding after this field due to the size of the flags.
    pub(crate) flags: SelectorFlags,
}

const MAX_10BIT: u32 = (1u32 << 10) - 1;

#[derive(Add, AddAssign, Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Specificity {
    id_selectors: u32,
    class_like_selectors: u32,
    element_selectors: u32,
}

impl Specificity {
    // Return the specficity of a single class-like selector.
    #[inline]
    pub fn single_class_like() -> Self {
        Specificity {
            id_selectors: 0,
            class_like_selectors: 1,
            element_selectors: 0,
        }
    }
}

impl From<u32> for Specificity {
    #[inline]
    fn from(value: u32) -> Specificity {
        assert!(value <= MAX_10BIT << 20 | MAX_10BIT << 10 | MAX_10BIT);
        Specificity {
            id_selectors: value >> 20,
            class_like_selectors: (value >> 10) & MAX_10BIT,
            element_selectors: value & MAX_10BIT,
        }
    }
}

impl From<Specificity> for u32 {
    #[inline]
    fn from(specificity: Specificity) -> u32 {
        cmp::min(specificity.id_selectors, MAX_10BIT) << 20
            | cmp::min(specificity.class_like_selectors, MAX_10BIT) << 10
            | cmp::min(specificity.element_selectors, MAX_10BIT)
    }
}

fn specificity_and_flags<Impl>(
    iter: slice::Iter<Component<Impl>>,
    for_nesting_parent: bool,
) -> SpecificityAndFlags
where
    Impl: SelectorImpl,
{
    fn component_specificity<Impl>(
        simple_selector: &Component<Impl>,
        specificity: &mut Specificity,
        flags: &mut SelectorFlags,
        for_nesting_parent: bool,
    ) where
        Impl: SelectorImpl,
    {
        match *simple_selector {
            Component::Combinator(..) => {},
            Component::ParentSelector => flags.insert(SelectorFlags::HAS_PARENT),
            Component::Part(..) => {
                flags.insert(SelectorFlags::HAS_PART);
                if !for_nesting_parent {
                    specificity.element_selectors += 1
                }
            },
            Component::PseudoElement(ref pseudo) => {
                use crate::parser::PseudoElement;
                flags.insert(SelectorFlags::HAS_PSEUDO);
                if !for_nesting_parent {
                    specificity.element_selectors += pseudo.specificity_count();
                }
            },
            Component::LocalName(..) => specificity.element_selectors += 1,
            Component::Slotted(ref selector) => {
                flags.insert(SelectorFlags::HAS_SLOTTED);
                if !for_nesting_parent {
                    specificity.element_selectors += 1;
                    // Note that due to the way ::slotted works we only compete with
                    // other ::slotted rules, so the above rule doesn't really
                    // matter, but we do it still for consistency with other
                    // pseudo-elements.
                    //
                    // See: https://github.com/w3c/csswg-drafts/issues/1915
                    *specificity += Specificity::from(selector.specificity());
                }
                flags.insert(selector.flags());
            },
            Component::Host(ref selector) => {
                flags.insert(SelectorFlags::HAS_HOST);
                specificity.class_like_selectors += 1;
                if let Some(ref selector) = *selector {
                    // See: https://github.com/w3c/csswg-drafts/issues/1915
                    *specificity += Specificity::from(selector.specificity());
                    flags.insert(selector.flags());
                }
            },
            Component::ID(..) => {
                specificity.id_selectors += 1;
            },
            Component::Class(..)
            | Component::AttributeInNoNamespace { .. }
            | Component::AttributeInNoNamespaceExists { .. }
            | Component::AttributeOther(..)
            | Component::Root
            | Component::Empty
            | Component::Nth(..)
            | Component::NonTSPseudoClass(..) => {
                specificity.class_like_selectors += 1;
            },
            Component::Scope | Component::ImplicitScope => {
                flags.insert(SelectorFlags::HAS_SCOPE);
                if matches!(*simple_selector, Component::Scope) {
                    specificity.class_like_selectors += 1;
                }
            },
            Component::NthOf(ref nth_of_data) => {
                // https://drafts.csswg.org/selectors/#specificity-rules:
                //
                //     The specificity of the :nth-last-child() pseudo-class,
                //     like the :nth-child() pseudo-class, combines the
                //     specificity of a regular pseudo-class with that of its
                //     selector argument S.
                specificity.class_like_selectors += 1;
                let sf = selector_list_specificity_and_flags(
                    nth_of_data.selectors().iter(),
                    for_nesting_parent,
                );
                *specificity += Specificity::from(sf.specificity);
                flags.insert(sf.flags);
            },
            // https://drafts.csswg.org/selectors/#specificity-rules:
            //
            //     The specificity of an :is(), :not(), or :has() pseudo-class
            //     is replaced by the specificity of the most specific complex
            //     selector in its selector list argument.
            Component::Where(ref list)
            | Component::Negation(ref list)
            | Component::Is(ref list) => {
                let sf = selector_list_specificity_and_flags(
                    list.slice().iter(),
                    /* nested = */ true,
                );
                if !matches!(*simple_selector, Component::Where(..)) {
                    *specificity += Specificity::from(sf.specificity);
                }
                flags.insert(sf.flags);
            },
            Component::Has(ref relative_selectors) => {
                let sf = relative_selector_list_specificity_and_flags(
                    relative_selectors,
                    for_nesting_parent,
                );
                *specificity += Specificity::from(sf.specificity);
                flags.insert(sf.flags);
            },
            Component::ExplicitUniversalType
            | Component::ExplicitAnyNamespace
            | Component::ExplicitNoNamespace
            | Component::DefaultNamespace(..)
            | Component::Namespace(..)
            | Component::RelativeSelectorAnchor
            | Component::Invalid(..) => {
                // Does not affect specificity
            },
        }
    }

    let mut specificity = Default::default();
    let mut flags = Default::default();
    for simple_selector in iter {
        component_specificity(
            &simple_selector,
            &mut specificity,
            &mut flags,
            for_nesting_parent,
        );
    }
    SpecificityAndFlags {
        specificity: specificity.into(),
        flags,
    }
}

/// Finds the maximum specificity of elements in the list and returns it.
pub(crate) fn selector_list_specificity_and_flags<'a, Impl: SelectorImpl>(
    itr: impl Iterator<Item = &'a Selector<Impl>>,
    for_nesting_parent: bool,
) -> SpecificityAndFlags {
    let mut specificity = 0;
    let mut flags = SelectorFlags::empty();
    for selector in itr {
        let selector_flags = selector.flags();
        let selector_specificity = if for_nesting_parent
            && selector_flags.intersects(SelectorFlags::forbidden_for_nesting())
        {
            // In this case we need to re-compute the specificity.
            specificity_and_flags(selector.iter_raw_match_order(), for_nesting_parent).specificity
        } else {
            selector.specificity()
        };
        specificity = std::cmp::max(specificity, selector_specificity);
        flags.insert(selector.flags());
    }
    SpecificityAndFlags { specificity, flags }
}

pub(crate) fn relative_selector_list_specificity_and_flags<Impl: SelectorImpl>(
    list: &[RelativeSelector<Impl>],
    for_nesting_parent: bool,
) -> SpecificityAndFlags {
    selector_list_specificity_and_flags(list.iter().map(|rel| &rel.selector), for_nesting_parent)
}

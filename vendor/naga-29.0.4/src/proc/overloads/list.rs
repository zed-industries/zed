//! An [`OverloadSet`] represented as a vector of rules.
//!
//! [`OverloadSet`]: crate::proc::overloads::OverloadSet

use crate::common::{DiagnosticDebug, ForDebug, ForDebugWithTypes};
use crate::ir;
use crate::proc::overloads::one_bits_iter::OneBitsIter;
use crate::proc::overloads::Rule;
use crate::proc::{GlobalCtx, TypeResolution};

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::fmt;

/// A simple list of overloads.
///
/// Note that this type is not quite as general as it looks, in that
/// the implementation of `most_preferred` doesn't work for arbitrary
/// lists of overloads. See the documentation for [`List::rules`] for
/// details.
#[derive(Clone)]
pub(in crate::proc::overloads) struct List {
    /// A bitmask of which elements of `rules` are included in the set.
    members: u64,

    /// A list of type rules that are members of the set.
    ///
    /// These must be listed in order such that every rule in the list
    /// is always more preferred than all subsequent rules in the
    /// list. If there is no such arrangement of rules, then you
    /// cannot use `List` to represent the overload set.
    rules: Rc<Vec<Rule>>,
}

impl List {
    pub(in crate::proc::overloads) fn from_rules(rules: Vec<Rule>) -> List {
        List {
            members: len_to_full_mask(rules.len()),
            rules: Rc::new(rules),
        }
    }

    fn members(&self) -> impl Iterator<Item = (u64, &Rule)> {
        OneBitsIter::new(self.members).map(|mask| {
            let index = mask.trailing_zeros() as usize;
            (mask, &self.rules[index])
        })
    }

    fn filter<F>(&self, mut pred: F) -> List
    where
        F: FnMut(&Rule) -> bool,
    {
        let mut filtered_members = 0;
        for (mask, rule) in self.members() {
            if pred(rule) {
                filtered_members |= mask;
            }
        }

        List {
            members: filtered_members,
            rules: self.rules.clone(),
        }
    }
}

impl crate::proc::overloads::OverloadSet for List {
    fn is_empty(&self) -> bool {
        self.members == 0
    }

    fn min_arguments(&self) -> usize {
        self.members()
            .fold(None, |best, (_, rule)| {
                // This is different from `max_arguments` because
                // `<Option as PartialOrd>` doesn't work the way we'd like.
                let len = rule.arguments.len();
                Some(match best {
                    Some(best) => core::cmp::max(best, len),
                    None => len,
                })
            })
            .unwrap()
    }

    fn max_arguments(&self) -> usize {
        self.members()
            .fold(None, |n, (_, rule)| {
                core::cmp::max(n, Some(rule.arguments.len()))
            })
            .unwrap()
    }

    fn arg(&self, i: usize, arg_ty: &ir::TypeInner, types: &crate::UniqueArena<ir::Type>) -> Self {
        log::debug!("arg {i} of type {:?}", arg_ty.for_debug(types));
        self.filter(|rule| {
            if log::log_enabled!(log::Level::Debug) {
                log::debug!("    considering rule {:?}", rule.for_debug(types));
                match rule.arguments.get(i) {
                    Some(rule_ty) => {
                        let rule_ty = rule_ty.inner_with(types);
                        if arg_ty.non_struct_equivalent(rule_ty, types) {
                            log::debug!("    types are equivalent");
                        } else {
                            match arg_ty.automatically_converts_to(rule_ty, types) {
                                Some((from, to)) => {
                                    log::debug!(
                                        "    converts automatically from {:?} to {:?}",
                                        from.for_debug(),
                                        to.for_debug()
                                    );
                                }
                                None => {
                                    log::debug!("    no conversion possible");
                                }
                            }
                        }
                    }
                    None => {
                        log::debug!("    argument index {i} out of bounds");
                    }
                }
            }
            rule.arguments.get(i).is_some_and(|rule_ty| {
                let rule_ty = rule_ty.inner_with(types);
                arg_ty.non_struct_equivalent(rule_ty, types)
                    || arg_ty.automatically_converts_to(rule_ty, types).is_some()
            })
        })
    }

    fn concrete_only(self, types: &crate::UniqueArena<ir::Type>) -> Self {
        self.filter(|rule| {
            rule.arguments
                .iter()
                .all(|arg_ty| !arg_ty.inner_with(types).is_abstract(types))
        })
    }

    fn most_preferred(&self) -> Rule {
        // As documented for `Self::rules`, whatever rule is first is
        // the most preferred. `OverloadSet` documents this method to
        // panic if the set is empty.
        let (_, rule) = self.members().next().unwrap();
        rule.clone()
    }

    fn overload_list(&self, _gctx: &GlobalCtx<'_>) -> Vec<Rule> {
        self.members().map(|(_, rule)| rule.clone()).collect()
    }

    fn allowed_args(&self, i: usize, _gctx: &GlobalCtx<'_>) -> Vec<TypeResolution> {
        self.members()
            .map(|(_, rule)| rule.arguments[i].clone())
            .collect()
    }

    fn for_debug(&self, types: &crate::UniqueArena<ir::Type>) -> impl fmt::Debug {
        DiagnosticDebug((self, types))
    }
}

const fn len_to_full_mask(n: usize) -> u64 {
    // This is a const function, which _sometimes_ gets called,
    // so this lint is _sometimes_ triggered, depending on feature set.
    #[expect(clippy::allow_attributes)]
    #[allow(clippy::panic)]
    if n >= 64 {
        panic!("List::rules can only hold up to 63 rules");
    }

    (1_u64 << n) - 1
}

impl ForDebugWithTypes for &List {}

impl fmt::Debug for DiagnosticDebug<(&List, &crate::UniqueArena<ir::Type>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (list, types) = self.0;

        f.debug_list()
            .entries(list.members().map(|(_mask, rule)| rule.for_debug(types)))
            .finish()
    }
}

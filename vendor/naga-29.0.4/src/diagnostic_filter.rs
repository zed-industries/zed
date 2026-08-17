//! [`DiagnosticFilter`]s and supporting functionality.

use alloc::boxed::Box;

use crate::{Arena, Handle};

#[cfg(feature = "wgsl-in")]
use crate::FastIndexMap;
#[cfg(feature = "wgsl-in")]
use crate::Span;
#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

/// A severity set on a [`DiagnosticFilter`].
///
/// <https://www.w3.org/TR/WGSL/#diagnostic-severity>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub enum Severity {
    Off,
    Info,
    Warning,
    Error,
}

impl Severity {
    /// Checks whether this severity is [`Self::Error`].
    ///
    /// Naga does not yet support diagnostic items at lesser severities than
    /// [`Severity::Error`]. When this is implemented, this method should be deleted, and the
    /// severity should be used directly for reporting diagnostics.
    pub(crate) fn report_diag<E>(
        self,
        err: E,
        log_handler: impl FnOnce(E, log::Level),
    ) -> Result<(), E> {
        let log_level = match self {
            Severity::Off => return Ok(()),

            // NOTE: These severities are not yet reported.
            Severity::Info => log::Level::Info,
            Severity::Warning => log::Level::Warn,

            Severity::Error => return Err(err),
        };
        log_handler(err, log_level);
        Ok(())
    }
}

/// A filterable triggering rule in a [`DiagnosticFilter`].
///
/// <https://www.w3.org/TR/WGSL/#filterable-triggering-rules>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub enum FilterableTriggeringRule {
    Standard(StandardFilterableTriggeringRule),
    Unknown(Box<str>),
    User(Box<[Box<str>; 2]>),
}

/// A filterable triggering rule in a [`DiagnosticFilter`].
///
/// <https://www.w3.org/TR/WGSL/#filterable-triggering-rules>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub enum StandardFilterableTriggeringRule {
    DerivativeUniformity,
}

impl StandardFilterableTriggeringRule {
    /// The default severity associated with this triggering rule.
    ///
    /// See <https://www.w3.org/TR/WGSL/#filterable-triggering-rules> for a table of default
    /// severities.
    pub(crate) const fn default_severity(self) -> Severity {
        match self {
            Self::DerivativeUniformity => Severity::Error,
        }
    }
}

/// A filtering rule that modifies how diagnostics are emitted for shaders.
///
/// <https://www.w3.org/TR/WGSL/#diagnostic-filter>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct DiagnosticFilter {
    pub new_severity: Severity,
    pub triggering_rule: FilterableTriggeringRule,
}

/// Determines whether [`DiagnosticFilterMap::add`] should consider full duplicates a conflict.
///
/// In WGSL, directive position does not consider this case a conflict, while attribute position
/// does.
#[cfg(feature = "wgsl-in")]
pub(crate) enum ShouldConflictOnFullDuplicate {
    /// Use this for attributes in WGSL.
    Yes,
    /// Use this for directives in WGSL.
    No,
}

/// A map from diagnostic filters to their severity and span.
///
/// Front ends can use this to collect the set of filters applied to a
/// particular language construct, and detect duplicate/conflicting filters.
///
/// For example, WGSL has global diagnostic filters that apply to the entire
/// module, and diagnostic range filter attributes that apply to a specific
/// function, statement, or other smaller construct. The set of filters applied
/// to any given construct must not conflict, but they can be overridden by
/// filters on other constructs nested within it. A front end can use a
/// `DiagnosticFilterMap` to collect the filters applied to a single construct,
/// using the [`add`] method's error checking to forbid conflicts.
///
/// For each filter it contains, a `DiagnosticFilterMap` records the requested
/// severity, and the source span of the filter itself.
///
/// [`add`]: DiagnosticFilterMap::add
#[derive(Clone, Debug, Default)]
#[cfg(feature = "wgsl-in")]
pub(crate) struct DiagnosticFilterMap(FastIndexMap<FilterableTriggeringRule, (Severity, Span)>);

#[cfg(feature = "wgsl-in")]
impl DiagnosticFilterMap {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Add the given `diagnostic_filter` parsed at the given `span` to this map.
    pub(crate) fn add(
        &mut self,
        diagnostic_filter: DiagnosticFilter,
        span: Span,
        should_conflict_on_full_duplicate: ShouldConflictOnFullDuplicate,
    ) -> Result<(), ConflictingDiagnosticRuleError> {
        use indexmap::map::Entry;

        let &mut Self(ref mut diagnostic_filters) = self;
        let DiagnosticFilter {
            new_severity,
            triggering_rule,
        } = diagnostic_filter;

        match diagnostic_filters.entry(triggering_rule.clone()) {
            Entry::Vacant(entry) => {
                entry.insert((new_severity, span));
            }
            Entry::Occupied(entry) => {
                let &(first_severity, first_span) = entry.get();
                let should_conflict_on_full_duplicate = match should_conflict_on_full_duplicate {
                    ShouldConflictOnFullDuplicate::Yes => true,
                    ShouldConflictOnFullDuplicate::No => false,
                };
                if first_severity != new_severity || should_conflict_on_full_duplicate {
                    return Err(ConflictingDiagnosticRuleError {
                        triggering_rule_spans: [first_span, span],
                    });
                }
            }
        }
        Ok(())
    }

    /// Were any rules specified?
    pub(crate) fn is_empty(&self) -> bool {
        let &Self(ref map) = self;
        map.is_empty()
    }

    /// Returns the spans of all contained rules.
    pub(crate) fn spans(&self) -> impl Iterator<Item = Span> + '_ {
        let &Self(ref map) = self;
        map.iter().map(|(_, &(_, span))| span)
    }
}

#[cfg(feature = "wgsl-in")]
impl IntoIterator for DiagnosticFilterMap {
    type Item = (FilterableTriggeringRule, (Severity, Span));

    type IntoIter = indexmap::map::IntoIter<FilterableTriggeringRule, (Severity, Span)>;

    fn into_iter(self) -> Self::IntoIter {
        let Self(this) = self;
        this.into_iter()
    }
}

/// An error returned by [`DiagnosticFilterMap::add`] when it encounters conflicting rules.
#[cfg(feature = "wgsl-in")]
#[derive(Clone, Debug)]
pub(crate) struct ConflictingDiagnosticRuleError {
    pub triggering_rule_spans: [Span; 2],
}

/// Represents a single parent-linking node in a tree of [`DiagnosticFilter`]s backed by a
/// [`crate::Arena`].
///
/// A single element of a _tree_ of diagnostic filter rules stored in
/// [`crate::Module::diagnostic_filters`]. When nodes are built by a front-end, module-applicable
/// filter rules are chained together in runs based on parse site.  For instance, given the
/// following:
///
/// - Module-applicable rules `a` and `b`.
/// - Rules `c` and `d`, applicable to an entry point called `c_and_d_func`.
/// - Rule `e`, applicable to an entry point called `e_func`.
///
/// The tree would be represented as follows:
///
/// ```text
/// a <- b
///      ^
///      |- c <- d
///      |
///      \- e
/// ```
///
/// ...where:
///
/// - `d` is the first leaf consulted by validation in `c_and_d_func`.
/// - `e` is the first leaf consulted by validation in `e_func`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct DiagnosticFilterNode {
    pub inner: DiagnosticFilter,
    pub parent: Option<Handle<DiagnosticFilterNode>>,
}

impl DiagnosticFilterNode {
    /// Finds the most specific filter rule applicable to `triggering_rule` from the chain of
    /// diagnostic filter rules in `arena`, starting with `node`, and returns its severity. If none
    /// is found, return the value of [`StandardFilterableTriggeringRule::default_severity`].
    ///
    /// When `triggering_rule` is not applicable to this node, its parent is consulted recursively.
    pub(crate) fn search(
        node: Option<Handle<Self>>,
        arena: &Arena<Self>,
        triggering_rule: StandardFilterableTriggeringRule,
    ) -> Severity {
        let mut next = node;
        while let Some(handle) = next {
            let node = &arena[handle];
            let &Self { ref inner, parent } = node;
            let &DiagnosticFilter {
                triggering_rule: ref rule,
                new_severity,
            } = inner;

            if rule == &FilterableTriggeringRule::Standard(triggering_rule) {
                return new_severity;
            }

            next = parent;
        }
        triggering_rule.default_severity()
    }
}

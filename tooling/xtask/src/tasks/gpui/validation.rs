//! Rules that the crates selected for a GPUI release have to satisfy.
//!
//! To add a rule, implement [`Rule`] in a submodule and list it in [`RULES`].

mod apache_licensed;
mod unique_publish_names;

use std::fmt;

use guppy::graph::{DependencyDirection, ExternalSource, PackageMetadata, PackageSet};
use thiserror::Error;

use super::crate_graph::is_zed_fork;
use crate::tasks::gpui::publish_plan::PublishPlanEntry;

const RULES: &[&dyn Rule] = &[
    &apache_licensed::ApacheLicensed,
    &unique_publish_names::UniquePublishNames,
    &ZedOwnedRepositories,
];

/// A check over the crates selected for a GPUI release.
pub(crate) trait Rule {
    /// What the rule guarantees, used to label the violations it reports.
    fn name(&self) -> &'static str;

    /// Returns one violation per crate that breaks the rule.
    fn check(
        &self,
        plan: &[PublishPlanEntry<'_>],
        dependencies: &[PackageMetadata<'_>],
    ) -> Vec<Violation>;
}

/// Why a crate does not satisfy a [`Rule`].
pub(crate) type Violation = Box<dyn std::error::Error + Send + Sync>;

/// Runs every rule and reports all violations at once, so that a single run
/// surfaces everything that needs fixing.
pub fn validate(
    plan: &[PublishPlanEntry<'_>],
    dependencies: &PackageSet<'_>,
) -> Result<(), ValidationError> {
    let dependencies: Vec<_> = dependencies
        .packages(DependencyDirection::Forward)
        .collect();
    validate_with(RULES, plan, &dependencies)
}

struct ZedOwnedRepositories;

impl Rule for ZedOwnedRepositories {
    fn name(&self) -> &'static str {
        "Zed-owned repositories"
    }

    fn check(
        &self,
        _: &[PublishPlanEntry<'_>],
        dependencies: &[PackageMetadata<'_>],
    ) -> Vec<Violation> {
        dependencies
            .iter()
            .filter_map(|package| {
                let source = package.source();
                if let Some(ExternalSource::Git { repository, .. }) = source.parse_external()
                    && !is_zed_fork(source)
                {
                    return Some(
                        format!("{} requires a fork of {repository}", package.name()).into(),
                    );
                }
                None
            })
            .collect()
    }
}

fn validate_with(
    rules: &[&dyn Rule],
    plan: &[PublishPlanEntry<'_>],
    dependencies: &[PackageMetadata<'_>],
) -> Result<(), ValidationError> {
    let violations = rules
        .iter()
        .flat_map(|rule| {
            rule.check(plan, dependencies)
                .into_iter()
                .map(|violation| (rule.name(), violation))
        })
        .collect::<Vec<_>>();

    if violations.is_empty() {
        return Ok(());
    }
    Err(ValidationError { violations })
}

#[derive(Debug, Error)]
pub struct ValidationError {
    violations: Vec<(&'static str, Violation)>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "found {count} violation(s) in the GPUI crate graph:",
            count = self.violations.len()
        )?;
        for (rule, violation) in &self.violations {
            write!(formatter, "\n  [{rule}] {violation}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Error)]
    #[error("{0} is held together with tape")]
    struct Ramshackle(&'static str);

    struct Failing;

    impl Rule for Failing {
        fn name(&self) -> &'static str {
            "crates are structurally sound"
        }

        fn check(&self, _: &[PublishPlanEntry<'_>], _: &[PackageMetadata<'_>]) -> Vec<Violation> {
            vec![
                Violation::from(Ramshackle("util")),
                Violation::from(Ramshackle("gpui")),
            ]
        }
    }

    #[test]
    fn labels_each_violation_with_the_rule_that_found_it() {
        assert!(validate_with(&[], &[], &[]).is_ok());

        let error = validate_with(&[&Failing], &[], &[])
            .expect_err("expected the failing rule to be reported");
        assert_eq!(
            error.to_string(),
            indoc::indoc! {"
                found 2 violation(s) in the GPUI crate graph:
                  [crates are structurally sound] util is held together with tape
                  [crates are structurally sound] gpui is held together with tape"}
        );
    }
}

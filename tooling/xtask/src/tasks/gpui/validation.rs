//! Rules that the crates selected for a GPUI release have to satisfy.
//!
//! To add a rule, implement [`Rule`] for a unit struct and list it in [`RULES`].

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;

use thiserror::Error;

use super::publish_plan::PublishPlanEntry;

const APACHE_LICENSE: &str = "Apache-2.0";
const APACHE_LICENSE_FILE: &str = "LICENSE-APACHE";

/// License files that contradict [`APACHE_LICENSE_FILE`] when present alongside it.
const CONFLICTING_LICENSE_FILES: &[&str] = &["LICENSE-GPL", "LICENSE-AGPL"];

const RULES: &[&dyn Rule] = &[&ApacheLicensed, &UniquePublishNames];

/// A check over the crates selected for a GPUI release.
trait Rule {
    /// What the rule guarantees, used to label the violations it reports.
    fn name(&self) -> &'static str;

    fn check(&self, plan: &[PublishPlanEntry<'_>]) -> Vec<Violation>;
}

/// A problem found by a [`Rule`].
#[derive(Debug, Error)]
pub enum Violation {
    #[error("{crate_name}: expected `license = \"{APACHE_LICENSE}\"`, found `{license}`")]
    WrongLicense { crate_name: String, license: String },
    #[error("{crate_name}: missing a `license` field, expected `{APACHE_LICENSE}`")]
    MissingLicense { crate_name: String },
    #[error("{crate_name}: `{APACHE_LICENSE_FILE}` must be a symlink to the repository license")]
    UnlinkedLicenseFile { crate_name: String },
    #[error("{crate_name}: `{conflicting}` contradicts `license = \"{APACHE_LICENSE}\"`")]
    ConflictingLicenseFile {
        crate_name: String,
        conflicting: &'static str,
    },
    #[error("`{crate_name}` and `{other}` would both be published as `{published_name}`")]
    DuplicatePublishName {
        crate_name: String,
        other: String,
        published_name: String,
    },
}

/// Runs every rule and reports all violations at once, so that a single run
/// surfaces everything that needs fixing.
pub fn validate(plan: &[PublishPlanEntry<'_>]) -> Result<(), ValidationError> {
    validate_with(RULES, plan)
}

fn validate_with(
    rules: &[&dyn Rule],
    plan: &[PublishPlanEntry<'_>],
) -> Result<(), ValidationError> {
    let violations = rules
        .iter()
        .flat_map(|rule| {
            rule.check(plan)
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

struct ApacheLicensed;

impl Rule for ApacheLicensed {
    fn name(&self) -> &'static str {
        "workspace crates are Apache-2.0 licensed"
    }

    fn check(&self, plan: &[PublishPlanEntry<'_>]) -> Vec<Violation> {
        plan.iter()
            .filter(|entry| entry.package.in_workspace())
            .filter_map(|entry| {
                let directory = entry.package.manifest_path().as_std_path().parent()?;
                license_violation(entry.package.name(), entry.package.license(), directory)
            })
            .collect()
    }
}

fn license_violation(
    crate_name: &str,
    license: Option<&str>,
    directory: &Path,
) -> Option<Violation> {
    let crate_name = crate_name.to_owned();
    match license {
        Some(APACHE_LICENSE) => {}
        Some(license) => {
            return Some(Violation::WrongLicense {
                crate_name,
                license: license.to_owned(),
            });
        }
        None => return Some(Violation::MissingLicense { crate_name }),
    }

    if !is_symlinked_license(&directory.join(APACHE_LICENSE_FILE)) {
        return Some(Violation::UnlinkedLicenseFile { crate_name });
    }

    let conflicting = CONFLICTING_LICENSE_FILES
        .iter()
        .copied()
        .find(|license_file| directory.join(license_file).exists())?;
    Some(Violation::ConflictingLicenseFile {
        crate_name,
        conflicting,
    })
}

fn is_symlinked_license(license_file: &Path) -> bool {
    fs::symlink_metadata(license_file).is_ok_and(|metadata| metadata.is_symlink())
        && fs::canonicalize(license_file)
            .is_ok_and(|target| target.file_name() == Some(APACHE_LICENSE_FILE.as_ref()))
}

struct UniquePublishNames;

impl Rule for UniquePublishNames {
    fn name(&self) -> &'static str {
        "published crate names are unique"
    }

    fn check(&self, plan: &[PublishPlanEntry<'_>]) -> Vec<Violation> {
        duplicate_publish_names(plan.iter().map(|entry| {
            (
                entry.repository.target_name(entry.package.name()),
                entry.package.name(),
            )
        }))
    }
}

fn duplicate_publish_names<'a>(
    published: impl IntoIterator<Item = (String, &'a str)>,
) -> Vec<Violation> {
    let mut crates_by_published_name = BTreeMap::<String, Vec<&str>>::new();
    for (published_name, crate_name) in published {
        crates_by_published_name
            .entry(published_name)
            .or_default()
            .push(crate_name);
    }

    let mut violations = Vec::new();
    for (published_name, mut crates) in crates_by_published_name {
        crates.sort_unstable();
        for pair in crates.windows(2) {
            let [crate_name, other] = pair else { continue };
            violations.push(Violation::DuplicatePublishName {
                crate_name: (*crate_name).to_owned(),
                other: (*other).to_owned(),
                published_name: published_name.clone(),
            });
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    struct Failing;

    impl Rule for Failing {
        fn name(&self) -> &'static str {
            "failing"
        }

        fn check(&self, _: &[PublishPlanEntry<'_>]) -> Vec<Violation> {
            vec![Violation::MissingLicense {
                crate_name: "util".to_owned(),
            }]
        }
    }

    #[test]
    fn labels_each_violation_with_the_rule_that_found_it() {
        assert!(validate_with(&[], &[]).is_ok());

        let error = validate_with(&[&Failing, &Failing], &[])
            .expect_err("expected the failing rule to be reported");
        assert_eq!(
            error.to_string(),
            indoc::indoc! {"
                found 2 violation(s) in the GPUI crate graph:
                  [failing] util: missing a `license` field, expected `Apache-2.0`
                  [failing] util: missing a `license` field, expected `Apache-2.0`"}
        );
    }

    #[cfg(unix)]
    #[test]
    fn requires_an_apache_license_field_and_symlink() -> Result<()> {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir()?;
        let root = root.path();
        fs::write(root.join(APACHE_LICENSE_FILE), "Apache License")?;
        fs::write(root.join("LICENSE-GPL"), "GPL")?;

        let crate_directory = |name: &str, license_link: Option<&str>| -> Result<_> {
            let directory = root.join(name);
            fs::create_dir(&directory)?;
            if let Some(license_link) = license_link {
                symlink(license_link, directory.join(APACHE_LICENSE_FILE))?;
            }
            Ok(directory)
        };

        let linked = crate_directory("linked", Some("../LICENSE-APACHE"))?;
        assert!(license_violation("linked", Some(APACHE_LICENSE), &linked).is_none());
        assert!(matches!(
            license_violation("linked", Some("MIT"), &linked),
            Some(Violation::WrongLicense { license, .. }) if license == "MIT"
        ));
        assert!(matches!(
            license_violation("linked", None, &linked),
            Some(Violation::MissingLicense { .. })
        ));

        for (name, license_link) in [
            ("missing", None),
            ("broken", Some("../nowhere")),
            ("mismatched", Some("../LICENSE-GPL")),
        ] {
            let directory = crate_directory(name, license_link)?;
            assert!(
                matches!(
                    license_violation(name, Some(APACHE_LICENSE), &directory),
                    Some(Violation::UnlinkedLicenseFile { .. })
                ),
                "{name}"
            );
        }

        let copied = crate_directory("copied", None)?;
        fs::write(copied.join(APACHE_LICENSE_FILE), "Apache License")?;
        assert!(matches!(
            license_violation("copied", Some(APACHE_LICENSE), &copied),
            Some(Violation::UnlinkedLicenseFile { .. })
        ));

        fs::write(linked.join("LICENSE-GPL"), "GPL")?;
        assert!(matches!(
            license_violation("linked", Some(APACHE_LICENSE), &linked),
            Some(Violation::ConflictingLicenseFile {
                conflicting: "LICENSE-GPL",
                ..
            })
        ));
        Ok(())
    }

    #[test]
    fn reports_crates_that_would_be_published_under_the_same_name() {
        let violations = duplicate_publish_names([
            ("gpui_util".to_owned(), "util"),
            ("gpui_collections".to_owned(), "collections"),
            ("gpui_util".to_owned(), "gpui_util"),
            ("zed-fork".to_owned(), "fork"),
        ]);
        assert_eq!(
            violations
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["`gpui_util` and `util` would both be published as `gpui_util`"]
        );

        assert!(
            duplicate_publish_names([
                ("gpui_util".to_owned(), "util"),
                ("gpui_collections".to_owned(), "collections"),
            ])
            .is_empty()
        );
    }
}

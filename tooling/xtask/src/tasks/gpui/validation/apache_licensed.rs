use std::fs;
use std::path::Path;

use thiserror::Error;

use super::{Rule, Violation};
use crate::tasks::gpui::publish_plan::PublishPlanEntry;

const APACHE_LICENSE: &str = "Apache-2.0";
const APACHE_LICENSE_FILE: &str = "LICENSE-APACHE";

/// License files that contradict [`APACHE_LICENSE_FILE`] when present alongside it.
const CONFLICTING_LICENSE_FILES: &[&str] = &["LICENSE-GPL", "LICENSE-AGPL"];

pub struct ApacheLicensed;

impl Rule for ApacheLicensed {
    fn name(&self) -> &'static str {
        "workspace crates are Apache-2.0 licensed"
    }

    fn check(&self, plan: &[PublishPlanEntry<'_>]) -> Vec<Violation> {
        plan.iter()
            .filter(|entry| entry.package.in_workspace())
            .filter_map(|entry| {
                let directory = entry.package.manifest_path().as_std_path().parent()?;
                let violation =
                    license_violation(entry.package.name(), entry.package.license(), directory)?;
                Some(Violation::from(violation))
            })
            .collect()
    }
}

#[derive(Debug, Error)]
enum LicenseViolation {
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
}

fn license_violation(
    crate_name: &str,
    license: Option<&str>,
    directory: &Path,
) -> Option<LicenseViolation> {
    let crate_name = crate_name.to_owned();
    match license {
        Some(APACHE_LICENSE) => {}
        Some(license) => {
            return Some(LicenseViolation::WrongLicense {
                crate_name,
                license: license.to_owned(),
            });
        }
        None => return Some(LicenseViolation::MissingLicense { crate_name }),
    }

    if !is_symlinked_license(&directory.join(APACHE_LICENSE_FILE)) {
        return Some(LicenseViolation::UnlinkedLicenseFile { crate_name });
    }

    let conflicting = CONFLICTING_LICENSE_FILES
        .iter()
        .copied()
        .find(|license_file| directory.join(license_file).exists())?;
    Some(LicenseViolation::ConflictingLicenseFile {
        crate_name,
        conflicting,
    })
}

fn is_symlinked_license(license_file: &Path) -> bool {
    fs::symlink_metadata(license_file).is_ok_and(|metadata| metadata.is_symlink())
        && fs::canonicalize(license_file)
            .is_ok_and(|target| target.file_name() == Some(APACHE_LICENSE_FILE.as_ref()))
}

#[cfg(all(test, unix))]
mod tests {
    use anyhow::Result;

    use super::*;

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
            Some(LicenseViolation::WrongLicense { license, .. }) if license == "MIT"
        ));
        assert!(matches!(
            license_violation("linked", None, &linked),
            Some(LicenseViolation::MissingLicense { .. })
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
                    Some(LicenseViolation::UnlinkedLicenseFile { .. })
                ),
                "{name}"
            );
        }

        let copied = crate_directory("copied", None)?;
        fs::write(copied.join(APACHE_LICENSE_FILE), "Apache License")?;
        assert!(matches!(
            license_violation("copied", Some(APACHE_LICENSE), &copied),
            Some(LicenseViolation::UnlinkedLicenseFile { .. })
        ));

        fs::write(linked.join("LICENSE-GPL"), "GPL")?;
        assert!(matches!(
            license_violation("linked", Some(APACHE_LICENSE), &linked),
            Some(LicenseViolation::ConflictingLicenseFile {
                conflicting: "LICENSE-GPL",
                ..
            })
        ));
        Ok(())
    }
}

use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;

use crate::workspace::load_workspace;

#[derive(Parser)]
pub struct LicensesArgs {}

pub fn run_licenses(_args: LicensesArgs) -> Result<()> {
    let workspace = load_workspace()?;

    for member in workspace.members {
        let crate_dir = PathBuf::from(&member);

        if has_any_license_file(
            &crate_dir,
            &["LICENSE-APACHE", "LICENSE-GPL", "LICENSE-AGPL"],
        ) {
            continue;
        }

        println!("Missing license: {member}");
    }

    Ok(())
}

fn has_any_license_file(path: &Path, license_files: &[&str]) -> bool {
    for license_file in license_files {
        let path_to_license = path.join(license_file);
        if path_to_license.exists() {
            return true;
        }
    }

    false
}

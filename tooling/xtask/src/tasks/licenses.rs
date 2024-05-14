use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;

use crate::workspace::load_workspace;

#[derive(Parser)]
pub struct LicensesArgs {}

pub fn run_licenses(_args: LicensesArgs) -> Result<()> {
    const LICENSE_FILES: &[&'static str] = &["LICENSE-APACHE", "LICENSE-GPL", "LICENSE-AGPL"];

    let workspace = load_workspace()?;

    for member in workspace.members {
        let crate_dir = PathBuf::from(&member);

        if let Some(license_file) = first_license_file(&crate_dir, &LICENSE_FILES) {
            if !license_file.is_symlink() {
                println!("{} is not a symlink", license_file.display());
            }

            continue;
        }

        println!("Missing license: {member}");
    }

    Ok(())
}

fn first_license_file(path: &Path, license_files: &[&str]) -> Option<PathBuf> {
    for license_file in license_files {
        let path_to_license = path.join(license_file);
        if path_to_license.exists() {
            return Some(path_to_license);
        }
    }

    None
}

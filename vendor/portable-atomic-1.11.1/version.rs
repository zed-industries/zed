// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{env, iter, process::Command, str};

pub(crate) fn rustc_version() -> Option<Version> {
    let rustc = env::var_os("RUSTC")?;
    let rustc_wrapper = if env::var_os("CARGO_ENCODED_RUSTFLAGS").is_some() {
        env::var_os("RUSTC_WRAPPER").filter(|v| !v.is_empty())
    } else {
        // Cargo sets environment variables for wrappers correctly only since https://github.com/rust-lang/cargo/pull/9601.
        None
    };
    // Do not apply RUSTC_WORKSPACE_WRAPPER: https://github.com/cuviper/autocfg/issues/58#issuecomment-2067625980
    let mut rustc = rustc_wrapper.into_iter().chain(iter::once(rustc));
    let mut cmd = Command::new(rustc.next().unwrap());
    cmd.args(rustc);
    // Use verbose version output because the packagers add extra strings to the normal version output.
    // Do not use long flags (--version --verbose) because clippy-deriver doesn't handle them properly.
    // -vV is also matched with that cargo internally uses: https://github.com/rust-lang/cargo/blob/0.80.0/src/cargo/util/rustc.rs#L65
    let output = cmd.arg("-vV").output().ok()?;
    let verbose_version = str::from_utf8(&output.stdout).ok()?;
    Version::parse(verbose_version)
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Version {
    pub(crate) minor: u32,
    pub(crate) nightly: bool,
    commit_date: Date,
    pub(crate) llvm: u32,
}

impl Version {
    // The known latest stable version. If we unable to determine
    // the rustc version, we assume this is the current version.
    // It is no problem if this is older than the actual latest stable.
    // LLVM version is assumed to be the minimum external LLVM version:
    // https://github.com/rust-lang/rust/blob/1.87.0/src/bootstrap/src/core/build_steps/llvm.rs#L617
    pub(crate) const LATEST: Self = Self::stable(87, 18);

    pub(crate) const fn stable(rustc_minor: u32, llvm_major: u32) -> Self {
        Self { minor: rustc_minor, nightly: false, commit_date: Date::UNKNOWN, llvm: llvm_major }
    }

    pub(crate) fn probe(&self, minor: u32, year: u16, month: u8, day: u8) -> bool {
        if self.nightly {
            self.minor > minor
                || self.minor == minor && self.commit_date >= Date::new(year, month, day)
        } else {
            self.minor >= minor
        }
    }

    #[cfg(test)]
    pub(crate) fn commit_date(&self) -> &Date {
        &self.commit_date
    }

    pub(crate) fn parse(verbose_version: &str) -> Option<Self> {
        let mut release = verbose_version
            .lines()
            .find(|line| line.starts_with("release: "))
            .map(|line| &line["release: ".len()..])?
            .splitn(2, '-');
        let version = release.next().unwrap();
        let channel = release.next().unwrap_or_default();
        let mut digits = version.splitn(3, '.');
        let major = digits.next()?;
        if major != "1" {
            return None;
        }
        let minor = digits.next()?.parse::<u32>().ok()?;
        let _patch = digits.next().unwrap_or("0").parse::<u32>().ok()?;
        let nightly = match env::var_os("RUSTC_BOOTSTRAP") {
            // When -1 is passed rustc works like stable, e.g., cfg(target_feature = "unstable_target_feature") will never be set. https://github.com/rust-lang/rust/pull/132993
            Some(ref v) if v == "-1" => false,
            _ => channel == "nightly" || channel == "dev",
        };

        // Note that rustc 1.49-1.50 (and 1.13 or older) don't print LLVM version.
        let llvm_major = (|| {
            let version = verbose_version
                .lines()
                .find(|line| line.starts_with("LLVM version: "))
                .map(|line| &line["LLVM version: ".len()..])?;
            let mut digits = version.splitn(3, '.');
            let major = digits.next()?.parse::<u32>().ok()?;
            let _minor = digits.next()?.parse::<u32>().ok()?;
            let _patch = digits.next().unwrap_or("0").parse::<u32>().ok()?;
            Some(major)
        })()
        .unwrap_or(0);

        // we don't refer commit date on stable/beta.
        if nightly {
            let commit_date = (|| {
                let mut commit_date = verbose_version
                    .lines()
                    .find(|line| line.starts_with("commit-date: "))
                    .map(|line| &line["commit-date: ".len()..])?
                    .splitn(3, '-');
                let year = commit_date.next()?.parse::<u16>().ok()?;
                let month = commit_date.next()?.parse::<u8>().ok()?;
                let day = commit_date.next()?.parse::<u8>().ok()?;
                if month > 12 || day > 31 {
                    return None;
                }
                Some(Date::new(year, month, day))
            })();
            Some(Self {
                minor,
                nightly,
                commit_date: commit_date.unwrap_or(Date::UNKNOWN),
                llvm: llvm_major,
            })
        } else {
            Some(Self::stable(minor, llvm_major))
        }
    }
}

#[derive(PartialEq, PartialOrd)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Date {
    pub(crate) year: u16,
    pub(crate) month: u8,
    pub(crate) day: u8,
}

impl Date {
    const UNKNOWN: Self = Self::new(0, 0, 0);

    const fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

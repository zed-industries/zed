use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Entity, Subscription, Task};
use postage::watch;
use project::Worktree;
use regex::Regex;
use strum::VariantArray;
use util::ResultExt as _;
use worktree::ChildEntriesOptions;

/// Matches the most common license locations, with US and UK English spelling.
static LICENSE_FILE_NAME_REGEX: LazyLock<regex::bytes::Regex> = LazyLock::new(|| {
    regex::bytes::RegexBuilder::new(
        "^ \
        (?: license | licence)? \
        (?: [\\-._]? \
            (?: apache (?: [\\-._] (?: 2.0 | 2 ))? | \
                0? bsd (?: [\\-._] [0123])? (?: [\\-._] clause)? | \
                isc | \
                mit | \
                upl))? \
        (?: [\\-._]? (?: license | licence))? \
        (?: \\.txt | \\.md)? \
        $",
    )
    .ignore_whitespace(true)
    .case_insensitive(true)
    .build()
    .unwrap()
});

#[derive(Debug, Clone, Copy, Eq, PartialEq, VariantArray)]
pub enum OpenSourceLicense {
    Apache2_0,
    BSD0Clause,
    BSD1Clause,
    BSD2Clause,
    BSD3Clause,
    ISC,
    MIT,
    UPL1_0,
}

impl Display for OpenSourceLicense {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spdx_identifier())
    }
}

impl OpenSourceLicense {
    pub fn spdx_identifier(&self) -> &'static str {
        match self {
            OpenSourceLicense::Apache2_0 => "apache-2.0",
            OpenSourceLicense::BSD0Clause => "0bsd",
            OpenSourceLicense::BSD1Clause => "bsd-1-clause",
            OpenSourceLicense::BSD2Clause => "bsd-2-clause",
            OpenSourceLicense::BSD3Clause => "bsd-3-clause",
            OpenSourceLicense::ISC => "isc",
            OpenSourceLicense::MIT => "mit",
            OpenSourceLicense::UPL1_0 => "upl-1.0",
        }
    }

    pub fn regex(&self) -> &'static str {
        match self {
            OpenSourceLicense::Apache2_0 => include_str!("license_detection/apache-2.0.regex"),
            OpenSourceLicense::BSD0Clause => include_str!("license_detection/0bsd.regex"),
            OpenSourceLicense::BSD1Clause => include_str!("license_detection/bsd-1-clause.regex"),
            OpenSourceLicense::BSD2Clause => include_str!("license_detection/bsd-2-clause.regex"),
            OpenSourceLicense::BSD3Clause => include_str!("license_detection/bsd-3-clause.regex"),
            OpenSourceLicense::ISC => include_str!("license_detection/isc.regex"),
            OpenSourceLicense::MIT => include_str!("license_detection/mit.regex"),
            OpenSourceLicense::UPL1_0 => include_str!("license_detection/upl-1.0.regex"),
        }
    }
}

fn detect_license(license: &str) -> Option<OpenSourceLicense> {
    static LICENSE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        let mut regex_string = String::new();
        let mut is_first = true;
        for license in OpenSourceLicense::VARIANTS {
            if is_first {
                regex_string.push_str("^(?:(");
                is_first = false;
            } else {
                regex_string.push_str(")|(");
            }
            regex_string.push_str(&canonicalize_license_text(license.regex()));
        }
        regex_string.push_str("))$");
        let regex = Regex::new(&regex_string).unwrap();
        assert_eq!(regex.captures_len(), OpenSourceLicense::VARIANTS.len() + 1);
        regex
    });

    LICENSE_REGEX
        .captures(&canonicalize_license_text(license))
        .and_then(|captures| {
            let license = OpenSourceLicense::VARIANTS
                .iter()
                .enumerate()
                .find(|(index, _)| captures.get(index + 1).is_some())
                .map(|(_, license)| *license);
            if license.is_none() {
                log::error!("bug: open source license regex matched without any capture groups");
            }
            license
        })
}

/// Canonicalizes the whitespace of license text and license regexes.
fn canonicalize_license_text(license: &str) -> String {
    license
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

pub enum LicenseDetectionWatcher {
    Local {
        is_open_source_rx: watch::Receiver<bool>,
        _is_open_source_task: Task<()>,
        _worktree_subscription: Subscription,
    },
    SingleFile,
    Remote,
}

impl LicenseDetectionWatcher {
    pub fn new(worktree: &Entity<Worktree>, cx: &mut App) -> Self {
        let worktree_ref = worktree.read(cx);
        if worktree_ref.is_single_file() {
            return Self::SingleFile;
        }

        let (files_to_check_tx, mut files_to_check_rx) = futures::channel::mpsc::unbounded();

        let Worktree::Local(local_worktree) = worktree_ref else {
            return Self::Remote;
        };
        let fs = local_worktree.fs().clone();
        let worktree_abs_path = local_worktree.abs_path().clone();

        let options = ChildEntriesOptions {
            include_files: true,
            include_dirs: false,
            include_ignored: true,
        };
        for top_file in local_worktree.child_entries_with_options(Path::new(""), options) {
            let path_bytes = top_file.path.as_os_str().as_encoded_bytes();
            if top_file.is_created() && LICENSE_FILE_NAME_REGEX.is_match(path_bytes) {
                let rel_path = top_file.path.clone();
                files_to_check_tx.unbounded_send(rel_path).ok();
            }
        }

        let _worktree_subscription =
            cx.subscribe(worktree, move |_worktree, event, _cx| match event {
                worktree::Event::UpdatedEntries(updated_entries) => {
                    for updated_entry in updated_entries.iter() {
                        let rel_path = &updated_entry.0;
                        let path_bytes = rel_path.as_os_str().as_encoded_bytes();
                        if LICENSE_FILE_NAME_REGEX.is_match(path_bytes) {
                            files_to_check_tx.unbounded_send(rel_path.clone()).ok();
                        }
                    }
                }
                worktree::Event::DeletedEntry(_) | worktree::Event::UpdatedGitRepositories(_) => {}
            });

        let (mut is_open_source_tx, is_open_source_rx) = watch::channel_with::<bool>(false);

        let _is_open_source_task = cx.background_spawn(async move {
            let mut eligible_licenses = BTreeSet::new();
            while let Some(rel_path) = files_to_check_rx.next().await {
                let abs_path = worktree_abs_path.join(&rel_path);
                let was_open_source = !eligible_licenses.is_empty();
                if Self::is_path_eligible(&fs, abs_path).await.unwrap_or(false) {
                    eligible_licenses.insert(rel_path);
                } else {
                    eligible_licenses.remove(&rel_path);
                }
                let is_open_source = !eligible_licenses.is_empty();
                if is_open_source != was_open_source {
                    *is_open_source_tx.borrow_mut() = is_open_source;
                }
            }
        });

        Self::Local {
            is_open_source_rx,
            _is_open_source_task,
            _worktree_subscription,
        }
    }

    async fn is_path_eligible(fs: &Arc<dyn Fs>, abs_path: PathBuf) -> Option<bool> {
        log::debug!("checking if `{abs_path:?}` is an open source license");
        // Resolve symlinks so that the file size from metadata is correct.
        let Some(abs_path) = fs.canonicalize(&abs_path).await.ok() else {
            log::debug!(
                "`{abs_path:?}` license file probably deleted (error canonicalizing the path)"
            );
            return None;
        };
        let metadata = fs.metadata(&abs_path).await.log_err()??;
        // If the license file is >32kb it's unlikely to legitimately match any eligible license.
        if metadata.len > 32768 {
            return None;
        }
        let text = fs.load(&abs_path).await.log_err()?;
        let is_eligible = detect_license(&text).is_some();
        if is_eligible {
            log::debug!(
                "`{abs_path:?}` matches a license that is eligible for data collection (if enabled)"
            );
        } else {
            log::debug!(
                "`{abs_path:?}` does not match a license that is eligible for data collection"
            );
        }
        Some(is_eligible)
    }

    /// Answers false until we find out it's open source
    pub fn is_project_open_source(&self) -> bool {
        match self {
            Self::Local {
                is_open_source_rx, ..
            } => *is_open_source_rx.borrow(),
            Self::SingleFile | Self::Remote => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::{Settings as _, SettingsStore};
    use unindent::unindent;
    use worktree::WorktreeSettings;

    use super::*;

    const APACHE_2_0_TXT: &str = include_str!("license_detection/apache-2.0.txt");
    const ISC_TXT: &str = include_str!("license_detection/isc.txt");
    const MIT_TXT: &str = include_str!("license_detection/mit.txt");
    const UPL_1_0_TXT: &str = include_str!("license_detection/upl-1.0.txt");
    const BSD_0_CLAUSE_TXT: &str = include_str!("license_detection/0bsd.txt");
    const BSD_1_CLAUSE_TXT: &str = include_str!("license_detection/bsd-1-clause.txt");
    const BSD_2_CLAUSE_TXT: &str = include_str!("license_detection/bsd-2-clause.txt");
    const BSD_3_CLAUSE_TXT: &str = include_str!("license_detection/bsd-3-clause.txt");

    #[track_caller]
    fn assert_matches_license(text: &str, license: OpenSourceLicense) {
        let license_regex =
            Regex::new(&format!("^{}$", canonicalize_license_text(license.regex()))).unwrap();
        assert!(license_regex.is_match(&canonicalize_license_text(text)));
        assert_eq!(detect_license(text), Some(license));
    }

    #[test]
    fn test_0bsd_positive_detection() {
        assert_matches_license(BSD_0_CLAUSE_TXT, OpenSourceLicense::BSD0Clause);
    }

    #[test]
    fn test_apache_positive_detection() {
        assert_matches_license(APACHE_2_0_TXT, OpenSourceLicense::Apache2_0);

        let license_with_appendix = format!(
            r#"{APACHE_2_0_TXT}

            END OF TERMS AND CONDITIONS

            APPENDIX: How to apply the Apache License to your work.

                To apply the Apache License to your work, attach the following
                boilerplate notice, with the fields enclosed by brackets "[]"
                replaced with your own identifying information. (Don't include
                the brackets!)  The text should be enclosed in the appropriate
                comment syntax for the file format. We also recommend that a
                file or class name and description of purpose be included on the
                same "printed page" as the copyright notice for easier
                identification within third-party archives.

            Copyright [yyyy] [name of copyright owner]

            Licensed under the Apache License, Version 2.0 (the "License");
            you may not use this file except in compliance with the License.
            You may obtain a copy of the License at

                http://www.apache.org/licenses/LICENSE-2.0

            Unless required by applicable law or agreed to in writing, software
            distributed under the License is distributed on an "AS IS" BASIS,
            WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
            See the License for the specific language governing permissions and
            limitations under the License."#
        );
        assert_matches_license(&license_with_appendix, OpenSourceLicense::Apache2_0);

        // Sometimes people fill in the appendix with copyright info.
        let license_with_copyright = license_with_appendix.replace(
            "Copyright [yyyy] [name of copyright owner]",
            "Copyright 2025 John Doe",
        );
        assert!(license_with_copyright != license_with_appendix);
        assert_matches_license(&license_with_copyright, OpenSourceLicense::Apache2_0);
    }

    #[test]
    fn test_apache_negative_detection() {
        assert!(
            detect_license(&format!(
                "{APACHE_2_0_TXT}\n\nThe terms in this license are void if P=NP."
            ))
            .is_none()
        );
    }

    #[test]
    fn test_bsd_1_clause_positive_detection() {
        assert_matches_license(BSD_1_CLAUSE_TXT, OpenSourceLicense::BSD1Clause);
    }

    #[test]
    fn test_bsd_2_clause_positive_detection() {
        assert_matches_license(BSD_2_CLAUSE_TXT, OpenSourceLicense::BSD2Clause);
    }

    #[test]
    fn test_bsd_3_clause_positive_detection() {
        assert_matches_license(BSD_3_CLAUSE_TXT, OpenSourceLicense::BSD3Clause);
    }

    #[test]
    fn test_isc_positive_detection() {
        assert_matches_license(ISC_TXT, OpenSourceLicense::ISC);
    }

    #[test]
    fn test_isc_negative_detection() {
        let license_text = format!(
            r#"{ISC_TXT}

            This project is dual licensed under the ISC License and the MIT License."#
        );

        assert!(detect_license(&license_text).is_none());
    }

    #[test]
    fn test_mit_positive_detection() {
        assert_matches_license(MIT_TXT, OpenSourceLicense::MIT);
    }

    #[test]
    fn test_mit_negative_detection() {
        let license_text = format!(
            r#"{MIT_TXT}

            This project is dual licensed under the MIT License and the Apache License, Version 2.0."#
        );
        assert!(detect_license(&license_text).is_none());
    }

    #[test]
    fn test_upl_positive_detection() {
        assert_matches_license(UPL_1_0_TXT, OpenSourceLicense::UPL1_0);
    }

    #[test]
    fn test_upl_negative_detection() {
        let license_text = format!(
            r#"{UPL_1_0_TXT}

            This project is dual licensed under the UPL License and the MIT License."#
        );

        assert!(detect_license(&license_text).is_none());
    }

    #[test]
    fn test_license_file_name_regex() {
        // Test basic license file names
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"licence"));

        // Test with extensions
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.txt"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.md"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE.txt"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE.md"));

        // Test with specific license types
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-APACHE"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE_MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-ISC"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-UPL"));

        // Test with "license" coming after
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"APACHE-LICENSE"));

        // Test version numbers
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"APACHE-2"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"APACHE-2.0"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-1"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-2"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-3"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-3-CLAUSE"));

        // Test combinations
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-MIT.txt"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE.ISC.md"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license_upl"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.APACHE.2.0"));

        // Test case insensitive
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"License"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license-mit.TXT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE_isc.MD"));

        // Test edge cases that should match
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license.mit"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"licence-upl.txt"));

        // Test non-matching patterns
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"COPYING"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.html"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"MYLICENSE"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"src/LICENSE"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.old"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-GPL"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSEABC"));
    }

    #[test]
    fn test_canonicalize_license_text() {
        let input = "  Paragraph 1\nwith multiple lines\n\n\n\nParagraph 2\nwith more lines\n  ";
        let expected = "paragraph 1 with multiple lines paragraph 2 with more lines";
        assert_eq!(canonicalize_license_text(input), expected);

        // Test tabs and mixed whitespace
        let input = "Word1\t\tWord2\n\n   Word3\r\n\r\n\r\nWord4   ";
        let expected = "word1 word2 word3 word4";
        assert_eq!(canonicalize_license_text(input), expected);
    }

    #[test]
    fn test_license_detection_canonicalizes_whitespace() {
        let mit_with_weird_spacing = unindent(
            r#"
                MIT License


                Copyright (c) 2024 John Doe


                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software   and   associated   documentation files (the "Software"), to deal
                in the Software without restriction, including without limitation the rights
                to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
                copies of the Software, and to permit persons to whom the Software is
                furnished to do so, subject to the following conditions:



                The above copyright notice and this permission notice shall be included in all
                copies or substantial portions of the Software.



                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
                IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
                FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
                AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
                LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
                OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
                SOFTWARE.
            "#
            .trim(),
        );

        assert_matches_license(&mit_with_weird_spacing, OpenSourceLicense::MIT);
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            WorktreeSettings::register(cx);
        });
    }

    #[gpui::test]
    async fn test_watcher_single_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "main.rs": "fn main() {}" }))
            .await;

        let worktree = Worktree::local(
            Path::new("/root/main.rs"),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let watcher = cx.update(|cx| LicenseDetectionWatcher::new(&worktree, cx));
        assert!(matches!(watcher, LicenseDetectionWatcher::SingleFile));
        assert!(!watcher.is_project_open_source());
    }

    #[gpui::test]
    async fn test_watcher_updates_on_changes(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "main.rs": "fn main() {}" }))
            .await;

        let worktree = Worktree::local(
            Path::new("/root"),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let watcher = cx.update(|cx| LicenseDetectionWatcher::new(&worktree, cx));
        assert!(matches!(watcher, LicenseDetectionWatcher::Local { .. }));
        assert!(!watcher.is_project_open_source());

        fs.write(Path::new("/root/LICENSE-MIT"), MIT_TXT.as_bytes())
            .await
            .unwrap();

        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.write(Path::new("/root/LICENSE-APACHE"), APACHE_2_0_TXT.as_bytes())
            .await
            .unwrap();

        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.write(Path::new("/root/LICENSE-MIT"), "Nevermind".as_bytes())
            .await
            .unwrap();

        // Still considered open source as LICENSE-APACHE is present
        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.write(
            Path::new("/root/LICENSE-APACHE"),
            "Also nevermind".as_bytes(),
        )
        .await
        .unwrap();

        cx.background_executor.run_until_parked();
        assert!(!watcher.is_project_open_source());
    }

    #[gpui::test]
    async fn test_watcher_initially_opensource_and_then_deleted(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({ "main.rs": "fn main() {}", "LICENSE-MIT": MIT_TXT }),
        )
        .await;

        let worktree = Worktree::local(
            Path::new("/root"),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let watcher = cx.update(|cx| LicenseDetectionWatcher::new(&worktree, cx));
        assert!(matches!(watcher, LicenseDetectionWatcher::Local { .. }));

        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.remove_file(
            Path::new("/root/LICENSE-MIT"),
            fs::RemoveOptions {
                recursive: false,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();

        cx.background_executor.run_until_parked();
        assert!(!watcher.is_project_open_source());
    }
}

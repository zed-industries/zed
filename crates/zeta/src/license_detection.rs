use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Entity, Subscription, Task};
use postage::watch;
use project::Worktree;
use regex::Regex;
use util::ResultExt as _;
use worktree::ChildEntriesOptions;

/// Matches the most common license locations, with US and UK English spelling.
const LICENSE_FILE_NAME_REGEX: LazyLock<regex::bytes::Regex> = LazyLock::new(|| {
    regex::bytes::RegexBuilder::new(
        "^ (?: license | licence) (?: [\\-._] (?: isc | mit | upl))? (?: \\.txt | \\.md)? $",
    )
    .ignore_whitespace(true)
    .case_insensitive(true)
    .build()
    .unwrap()
});

fn is_license_eligible_for_data_collection(license: &str) -> bool {
    // TODO: Include more licenses later (namely, Apache)
    const LICENSE_REGEXES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
        [
            include_str!("license_detection/isc.regex"),
            include_str!("license_detection/mit.regex"),
            include_str!("license_detection/upl.regex"),
        ]
        .into_iter()
        .map(|pattern| Regex::new(&canonicalize_license_text(pattern)).unwrap())
        .collect()
    });

    let license = canonicalize_license_text(license);
    LICENSE_REGEXES.iter().any(|regex| regex.is_match(&license))
}

/// Canonicalizes the whitespace of license text and license regexes.
fn canonicalize_license_text(license: &str) -> String {
    const PARAGRAPH_SEPARATOR_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\s*\n\s*\n\s*").unwrap());

    PARAGRAPH_SEPARATOR_REGEX
        .split(license)
        .filter(|paragraph| !paragraph.trim().is_empty())
        .map(|paragraph| paragraph.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n\n")
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
        log::info!("checking if `{abs_path:?}` is an open source license");
        // Resolve symlinks so that the file size from metadata is correct.
        let Some(abs_path) = fs.canonicalize(&abs_path).await.ok() else {
            log::info!(
                "`{abs_path:?}` license file probably deleted (error canonicalizing the path)"
            );
            return None;
        };
        let metadata = fs.metadata(&abs_path).await.log_err()??;
        // If the license file is >5kb it's unlikely to match any eligible license. This avoids the
        // potential for reading very large files into memory.
        if metadata.len > 5120 {
            return None;
        }
        let text = fs.load(&abs_path).await.log_err()?;
        let is_eligible = is_license_eligible_for_data_collection(&text);
        if is_eligible {
            log::info!(
                "`{abs_path:?}` matches a license that is eligible for data collection (if enabled)"
            );
        } else {
            log::info!(
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
    use unindent::unindent;

    use super::*;

    #[test]
    fn test_mit_positive_detection() {
        let example_license = unindent(
            r#"
                MIT License

                Copyright (c) 2024 John Doe

                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software and associated documentation files (the "Software"), to deal
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

        assert!(is_license_eligible_for_data_collection(&example_license));

        let example_license = unindent(
            r#"
                The MIT License (MIT)

                Copyright (c) 2019 John Doe

                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software and associated documentation files (the "Software"), to deal
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

        assert!(is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_mit_negative_detection() {
        let example_license = unindent(
            r#"
                MIT License

                Copyright (c) 2024 John Doe

                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software and associated documentation files (the "Software"), to deal
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

                This project is dual licensed under the MIT License and the Apache License, Version 2.0.
            "#
            .trim(),
        );

        assert!(!is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_isc_positive_detection() {
        let example_license = unindent(
            r#"
                ISC License

                Copyright (c) 2024, John Doe

                Permission to use, copy, modify, and/or distribute this software for any
                purpose with or without fee is hereby granted, provided that the above
                copyright notice and this permission notice appear in all copies.

                THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
                WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
                MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
                ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
                WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
                ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
                OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
            "#
            .trim(),
        );

        assert!(is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_isc_negative_detection() {
        let example_license = unindent(
            r#"
                ISC License

                Copyright (c) 2024, John Doe

                Permission to use, copy, modify, and/or distribute this software for any
                purpose with or without fee is hereby granted, provided that the above
                copyright notice and this permission notice appear in all copies.

                THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
                WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
                MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
                ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
                WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
                ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
                OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

                This project is dual licensed under the ISC License and the MIT License.
            "#
            .trim(),
        );

        assert!(!is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_upl_positive_detection() {
        let example_license = unindent(
            r#"
                Copyright (c) 2025, John Doe

                The Universal Permissive License (UPL), Version 1.0

                Subject to the condition set forth below, permission is hereby granted to any person
                obtaining a copy of this software, associated documentation and/or data (collectively
                the "Software"), free of charge and under any and all copyright rights in the
                Software, and any and all patent rights owned or freely licensable by each licensor
                hereunder covering either (i) the unmodified Software as contributed to or provided
                by such licensor, or (ii) the Larger Works (as defined below), to deal in both

                (a) the Software, and

                (b) any piece of software and/or hardware listed in the lrgrwrks.txt file if one is
                    included with the Software (each a "Larger Work" to which the Software is
                    contributed by such licensors),

                without restriction, including without limitation the rights to copy, create
                derivative works of, display, perform, and distribute the Software and make, use,
                sell, offer for sale, import, export, have made, and have sold the Software and the
                Larger Work(s), and to sublicense the foregoing rights on either these or other
                terms.

                This license is subject to the following condition:

                The above copyright notice and either this complete permission notice or at a minimum
                a reference to the UPL must be included in all copies or substantial portions of the
                Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
                INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
                PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
                HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
                CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
                OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
            "#
            .trim(),
        );

        assert!(is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_upl_negative_detection() {
        let example_license = unindent(
            r#"
                UPL License

                Copyright (c) 2024, John Doe

                The Universal Permissive License (UPL), Version 1.0

                Subject to the condition set forth below, permission is hereby granted to any person
                obtaining a copy of this software, associated documentation and/or data (collectively
                the "Software"), free of charge and under any and all copyright rights in the
                Software, and any and all patent rights owned or freely licensable by each licensor
                hereunder covering either (i) the unmodified Software as contributed to or provided
                by such licensor, or (ii) the Larger Works (as defined below), to deal in both

                (a) the Software, and

                (b) any piece of software and/or hardware listed in the lrgrwrks.txt file if one is
                    included with the Software (each a "Larger Work" to which the Software is
                    contributed by such licensors),

                without restriction, including without limitation the rights to copy, create
                derivative works of, display, perform, and distribute the Software and make, use,
                sell, offer for sale, import, export, have made, and have sold the Software and the
                Larger Work(s), and to sublicense the foregoing rights on either these or other
                terms.

                This license is subject to the following condition:

                The above copyright notice and either this complete permission notice or at a minimum
                a reference to the UPL must be included in all copies or substantial portions of the
                Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
                INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
                PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
                HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
                CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
                OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

                This project is dual licensed under the ISC License and the MIT License.
            "#
            .trim(),
        );

        assert!(!is_license_eligible_for_data_collection(&example_license));
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
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE_MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-ISC"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-UPL"));

        // Test combinations
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-MIT.txt"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE.ISC.md"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license_upl"));

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
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-APACHE"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"MYLICENSE"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"src/LICENSE"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.old"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-GPL"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSEABC"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b""));
    }

    #[test]
    fn test_canonicalize_license_text() {
        // Test basic whitespace normalization
        let input = "Line 1\n   Line 2   \n\n\n  Line 3  ";
        let expected = "Line 1 Line 2\n\nLine 3";
        assert_eq!(canonicalize_license_text(input), expected);

        // Test paragraph separation
        let input = "Paragraph 1\nwith multiple lines\n\n\n\nParagraph 2\nwith more lines";
        let expected = "Paragraph 1 with multiple lines\n\nParagraph 2 with more lines";
        assert_eq!(canonicalize_license_text(input), expected);

        // Test empty paragraphs are filtered out
        let input = "\n\n\nParagraph 1\n\n\n   \n\n\nParagraph 2\n\n\n";
        let expected = "Paragraph 1\n\nParagraph 2";
        assert_eq!(canonicalize_license_text(input), expected);

        // Test single line
        let input = "   Single line with spaces   ";
        let expected = "Single line with spaces";
        assert_eq!(canonicalize_license_text(input), expected);

        // Test multiple consecutive spaces within lines
        let input = "Word1    Word2\n\nWord3     Word4";
        let expected = "Word1 Word2\n\nWord3 Word4";
        assert_eq!(canonicalize_license_text(input), expected);

        // Test tabs and mixed whitespace
        let input = "Word1\t\tWord2\n\n   Word3\r\n\r\n\r\nWord4   ";
        let expected = "Word1 Word2\n\nWord3\n\nWord4";
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

        assert!(is_license_eligible_for_data_collection(
            &mit_with_weird_spacing
        ));
    }
}

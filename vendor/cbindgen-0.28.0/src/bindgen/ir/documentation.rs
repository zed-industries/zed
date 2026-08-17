/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::utilities::SynAttributeHelpers;

#[derive(Debug, Clone)]
pub struct Documentation {
    pub doc_comment: Vec<String>,
}

impl Documentation {
    pub fn load(attrs: &[syn::Attribute]) -> Self {
        let doc = attrs
            .get_comment_lines()
            .into_iter()
            .filter(|x| !x.trim_start().starts_with("cbindgen:"))
            .collect();

        Documentation { doc_comment: doc }
    }

    pub fn simple(line: &str) -> Self {
        Documentation {
            doc_comment: vec![line.to_owned()],
        }
    }

    pub fn none() -> Self {
        Documentation {
            doc_comment: Vec::new(),
        }
    }
}

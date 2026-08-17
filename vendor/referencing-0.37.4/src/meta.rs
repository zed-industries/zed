//! Built-in JSON Schema meta-schemas.
//!
//! This module provides access to the official JSON Schema meta-schemas for different draft versions.
use serde_json::Value;
use std::sync::{Arc, LazyLock};

use crate::Draft;

macro_rules! schema {
    ($vis:vis $name:ident, $path:expr) => {
        $vis static $name: LazyLock<Arc<serde_json::Value>> = LazyLock::new(|| {
            Arc::new(serde_json::from_slice(include_bytes!($path)).expect("Invalid schema"))
        });
    };
    ($name:ident, $path:expr) => {
        schema!(pub(crate) $name, $path);
    };
}

schema!(pub DRAFT4, "../metaschemas/draft4.json");
schema!(pub DRAFT6, "../metaschemas/draft6.json");
schema!(pub DRAFT7, "../metaschemas/draft7.json");
schema!(pub DRAFT201909, "../metaschemas/draft2019-09/schema.json");
schema!(
    pub DRAFT201909_APPLICATOR,
    "../metaschemas/draft2019-09/meta/applicator.json"
);
schema!(
    pub DRAFT201909_CONTENT,
    "../metaschemas/draft2019-09/meta/content.json"
);
schema!(
    pub DRAFT201909_CORE,
    "../metaschemas/draft2019-09/meta/core.json"
);
schema!(
    pub DRAFT201909_FORMAT,
    "../metaschemas/draft2019-09/meta/format.json"
);
schema!(
    pub DRAFT201909_META_DATA,
    "../metaschemas/draft2019-09/meta/meta-data.json"
);
schema!(
    pub DRAFT201909_VALIDATION,
    "../metaschemas/draft2019-09/meta/validation.json"
);
schema!(pub DRAFT202012, "../metaschemas/draft2020-12/schema.json");
schema!(
    pub DRAFT202012_CORE,
    "../metaschemas/draft2020-12/meta/core.json"
);
schema!(
    pub DRAFT202012_APPLICATOR,
    "../metaschemas/draft2020-12/meta/applicator.json"
);
schema!(
    pub DRAFT202012_UNEVALUATED,
    "../metaschemas/draft2020-12/meta/unevaluated.json"
);
schema!(
    pub DRAFT202012_VALIDATION,
    "../metaschemas/draft2020-12/meta/validation.json"
);
schema!(
    pub DRAFT202012_META_DATA,
    "../metaschemas/draft2020-12/meta/meta-data.json"
);
schema!(
    pub DRAFT202012_FORMAT_ANNOTATION,
    "../metaschemas/draft2020-12/meta/format-annotation.json"
);
schema!(
    pub DRAFT202012_FORMAT_ASSERTION,
    "../metaschemas/draft2020-12/meta/format-assertion.json"
);
schema!(
    pub DRAFT202012_CONTENT,
    "../metaschemas/draft2020-12/meta/content.json"
);
pub(crate) static META_SCHEMAS_ALL: LazyLock<[(&'static str, &'static Value); 18]> =
    LazyLock::new(|| {
        [
            ("http://json-schema.org/draft-04/schema#", &*DRAFT4),
            ("http://json-schema.org/draft-06/schema#", &*DRAFT6),
            ("http://json-schema.org/draft-07/schema#", &*DRAFT7),
            (
                "https://json-schema.org/draft/2019-09/schema",
                &*DRAFT201909,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/applicator",
                &*DRAFT201909_APPLICATOR,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/content",
                &*DRAFT201909_CONTENT,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/core",
                &*DRAFT201909_CORE,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/format",
                &*DRAFT201909_FORMAT,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/meta-data",
                &*DRAFT201909_META_DATA,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/validation",
                &*DRAFT201909_VALIDATION,
            ),
            (
                "https://json-schema.org/draft/2020-12/schema",
                &*DRAFT202012,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/core",
                &*DRAFT202012_CORE,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/applicator",
                &*DRAFT202012_APPLICATOR,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/unevaluated",
                &*DRAFT202012_UNEVALUATED,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/validation",
                &*DRAFT202012_VALIDATION,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/meta-data",
                &*DRAFT202012_META_DATA,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/format-annotation",
                &*DRAFT202012_FORMAT_ANNOTATION,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/content",
                &*DRAFT202012_CONTENT,
            ),
        ]
    });

pub(crate) static META_SCHEMAS_DRAFT4: LazyLock<[(&'static str, &'static Value); 1]> =
    LazyLock::new(|| [("http://json-schema.org/draft-04/schema#", &*DRAFT4)]);

pub(crate) static META_SCHEMAS_DRAFT6: LazyLock<[(&'static str, &'static Value); 1]> =
    LazyLock::new(|| [("http://json-schema.org/draft-06/schema#", &*DRAFT6)]);

pub(crate) static META_SCHEMAS_DRAFT7: LazyLock<[(&'static str, &'static Value); 1]> =
    LazyLock::new(|| [("http://json-schema.org/draft-07/schema#", &*DRAFT7)]);

pub(crate) static META_SCHEMAS_DRAFT2019: LazyLock<[(&'static str, &'static Value); 7]> =
    LazyLock::new(|| {
        [
            (
                "https://json-schema.org/draft/2019-09/schema",
                &*DRAFT201909,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/applicator",
                &*DRAFT201909_APPLICATOR,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/content",
                &*DRAFT201909_CONTENT,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/core",
                &*DRAFT201909_CORE,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/format",
                &*DRAFT201909_FORMAT,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/meta-data",
                &*DRAFT201909_META_DATA,
            ),
            (
                "https://json-schema.org/draft/2019-09/meta/validation",
                &*DRAFT201909_VALIDATION,
            ),
        ]
    });

pub(crate) static META_SCHEMAS_DRAFT2020: LazyLock<[(&'static str, &'static Value); 8]> =
    LazyLock::new(|| {
        [
            (
                "https://json-schema.org/draft/2020-12/schema",
                &*DRAFT202012,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/core",
                &*DRAFT202012_CORE,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/applicator",
                &*DRAFT202012_APPLICATOR,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/unevaluated",
                &*DRAFT202012_UNEVALUATED,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/validation",
                &*DRAFT202012_VALIDATION,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/meta-data",
                &*DRAFT202012_META_DATA,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/format-annotation",
                &*DRAFT202012_FORMAT_ANNOTATION,
            ),
            (
                "https://json-schema.org/draft/2020-12/meta/content",
                &*DRAFT202012_CONTENT,
            ),
        ]
    });

/// Return all the meta-schemas which are part of a given draft.
pub(crate) fn metas_for_draft(draft: Draft) -> &'static [(&'static str, &'static Value)] {
    match draft {
        Draft::Draft4 => &*META_SCHEMAS_DRAFT4,
        Draft::Draft6 => &*META_SCHEMAS_DRAFT6,
        Draft::Draft7 => &*META_SCHEMAS_DRAFT7,
        Draft::Draft201909 => &*META_SCHEMAS_DRAFT2019,
        // Unknown drafts default to 2020-12 vocabularies.
        // Custom meta-schemas should explicitly declare vocabularies in their $vocabulary field.
        Draft::Draft202012 | Draft::Unknown => &*META_SCHEMAS_DRAFT2020,
    }
}

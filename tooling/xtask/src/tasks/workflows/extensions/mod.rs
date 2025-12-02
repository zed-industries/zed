use gh_workflow::{Job, UsesJob};
use indexmap::IndexMap;

use crate::tasks::workflows::vars;

pub(crate) mod bump_version;
pub(crate) mod release_version;
pub(crate) mod run_tests;

pub(crate) trait WithAppSecrets: Sized {
    fn with_app_secrets(self) -> Self;
}

impl WithAppSecrets for Job<UsesJob> {
    fn with_app_secrets(self) -> Self {
        self.secrets(IndexMap::from([
            ("app-id".to_owned(), vars::ZED_ZIPPY_APP_ID.to_owned()),
            (
                "app-secret".to_owned(),
                vars::ZED_ZIPPY_APP_PRIVATE_KEY.to_owned(),
            ),
        ]))
    }
}

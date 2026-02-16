use gh_workflow::{ctx::Context, *};

use crate::tasks::workflows::steps::named;

pub(crate) fn deploy_collab() -> Workflow {
    named::workflow()
}

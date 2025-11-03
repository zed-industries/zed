use std::cell::RefCell;

use gh_workflow::{Concurrency, Env, Expression};

use crate::tasks::workflows::steps::NamedJob;

macro_rules! secret {
    ($secret_name:ident) => {
        pub const $secret_name: &str = concat!("${{ secrets.", stringify!($secret_name), " }}");
    };
}

macro_rules! var {
    ($secret_name:ident) => {
        pub const $secret_name: &str = concat!("${{ vars.", stringify!($secret_name), " }}");
    };
}

secret!(APPLE_NOTARIZATION_ISSUER_ID);
secret!(APPLE_NOTARIZATION_KEY);
secret!(APPLE_NOTARIZATION_KEY_ID);
secret!(AZURE_SIGNING_CLIENT_ID);
secret!(AZURE_SIGNING_CLIENT_SECRET);
secret!(AZURE_SIGNING_TENANT_ID);
secret!(CACHIX_AUTH_TOKEN);
secret!(DIGITALOCEAN_SPACES_ACCESS_KEY);
secret!(DIGITALOCEAN_SPACES_SECRET_KEY);
secret!(GITHUB_TOKEN);
secret!(MACOS_CERTIFICATE);
secret!(MACOS_CERTIFICATE_PASSWORD);
secret!(SENTRY_AUTH_TOKEN);
secret!(ZED_CLIENT_CHECKSUM_SEED);
secret!(ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON);
secret!(ZED_SENTRY_MINIDUMP_ENDPOINT);

// todo(ci) make these secrets too...
var!(AZURE_SIGNING_ACCOUNT_NAME);
var!(AZURE_SIGNING_CERT_PROFILE_NAME);
var!(AZURE_SIGNING_ENDPOINT);

pub const GITHUB_SHA: &str = "${{ github.event.pull_request.head.sha || github.sha }}";

pub fn mac_bundle_envs() -> Env {
    Env::default()
        .add("MACOS_CERTIFICATE", MACOS_CERTIFICATE)
        .add("MACOS_CERTIFICATE_PASSWORD", MACOS_CERTIFICATE_PASSWORD)
        .add("APPLE_NOTARIZATION_KEY", APPLE_NOTARIZATION_KEY)
        .add("APPLE_NOTARIZATION_KEY_ID", APPLE_NOTARIZATION_KEY_ID)
        .add("APPLE_NOTARIZATION_ISSUER_ID", APPLE_NOTARIZATION_ISSUER_ID)
}

pub fn windows_bundle_envs() -> Env {
    Env::default()
        .add("AZURE_TENANT_ID", AZURE_SIGNING_TENANT_ID)
        .add("AZURE_CLIENT_ID", AZURE_SIGNING_CLIENT_ID)
        .add("AZURE_CLIENT_SECRET", AZURE_SIGNING_CLIENT_SECRET)
        .add("ACCOUNT_NAME", AZURE_SIGNING_ACCOUNT_NAME)
        .add("CERT_PROFILE_NAME", AZURE_SIGNING_CERT_PROFILE_NAME)
        .add("ENDPOINT", AZURE_SIGNING_ENDPOINT)
        .add("FILE_DIGEST", "SHA256")
        .add("TIMESTAMP_DIGEST", "SHA256")
        .add("TIMESTAMP_SERVER", "http://timestamp.acs.microsoft.com")
}

pub(crate) fn one_workflow_per_non_main_branch() -> Concurrency {
    Concurrency::default()
        .group("${{ github.workflow }}-${{ github.ref_name }}-${{ github.ref_name == 'main' && github.sha || 'anysha' }}")
        .cancel_in_progress(true)
}

// Represents a pattern to check for changed files and corresponding output variable
pub(crate) struct PathCondition {
    pub name: &'static str,
    pub pattern: &'static str,
    pub invert: bool,
    pub set_by_step: RefCell<Option<String>>,
}
impl PathCondition {
    pub fn new(name: &'static str, pattern: &'static str) -> Self {
        Self {
            name,
            pattern,
            invert: false,
            set_by_step: Default::default(),
        }
    }
    pub fn inverted(name: &'static str, pattern: &'static str) -> Self {
        Self {
            name,
            pattern,
            invert: true,
            set_by_step: Default::default(),
        }
    }
    pub fn guard(&self, job: NamedJob) -> NamedJob {
        let set_by_step = self
            .set_by_step
            .borrow()
            .clone()
            .unwrap_or_else(|| panic!("condition {},is never set", self.name));
        NamedJob {
            name: job.name,
            job: job
                .job
                .add_needs(set_by_step.clone())
                .cond(Expression::new(format!(
                    "needs.{}.outputs.{} == 'true'",
                    &set_by_step, self.name
                ))),
        }
    }
}

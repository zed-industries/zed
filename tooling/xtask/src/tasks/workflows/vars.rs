use std::cell::RefCell;

use gh_workflow::{
    Concurrency, Env, Expression, Step, WorkflowCallInput, WorkflowCallSecret,
    WorkflowDispatchInput,
};

use crate::tasks::workflows::{runners::Platform, steps::NamedJob};

macro_rules! secret {
    ($secret_name:ident) => {
        pub const $secret_name: &str = concat!("${{ secrets.", stringify!($secret_name), " }}");
    };
}

macro_rules! var {
    ($var_name:ident) => {
        pub const $var_name: &str = concat!("${{ vars.", stringify!($var_name), " }}");
    };
}

secret!(ANTHROPIC_API_KEY);
secret!(OPENAI_API_KEY);
secret!(GOOGLE_AI_API_KEY);
secret!(GOOGLE_CLOUD_PROJECT);
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
secret!(SLACK_APP_ZED_UNIT_EVALS_BOT_TOKEN);
secret!(ZED_ZIPPY_APP_ID);
secret!(ZED_ZIPPY_APP_PRIVATE_KEY);
secret!(DISCORD_WEBHOOK_RELEASE_NOTES);
secret!(WINGET_TOKEN);
secret!(VERCEL_TOKEN);
secret!(SLACK_WEBHOOK_WORKFLOW_FAILURES);

// todo(ci) make these secrets too...
var!(AZURE_SIGNING_ACCOUNT_NAME);
var!(AZURE_SIGNING_CERT_PROFILE_NAME);
var!(AZURE_SIGNING_ENDPOINT);

pub fn bundle_envs(platform: Platform) -> Env {
    let env = Env::default()
        .add("CARGO_INCREMENTAL", 0)
        .add("ZED_CLIENT_CHECKSUM_SEED", ZED_CLIENT_CHECKSUM_SEED)
        .add("ZED_MINIDUMP_ENDPOINT", ZED_SENTRY_MINIDUMP_ENDPOINT);

    match platform {
        Platform::Linux => env,
        Platform::Mac => env
            .add("MACOS_CERTIFICATE", MACOS_CERTIFICATE)
            .add("MACOS_CERTIFICATE_PASSWORD", MACOS_CERTIFICATE_PASSWORD)
            .add("APPLE_NOTARIZATION_KEY", APPLE_NOTARIZATION_KEY)
            .add("APPLE_NOTARIZATION_KEY_ID", APPLE_NOTARIZATION_KEY_ID)
            .add("APPLE_NOTARIZATION_ISSUER_ID", APPLE_NOTARIZATION_ISSUER_ID),
        Platform::Windows => env
            .add("AZURE_TENANT_ID", AZURE_SIGNING_TENANT_ID)
            .add("AZURE_CLIENT_ID", AZURE_SIGNING_CLIENT_ID)
            .add("AZURE_CLIENT_SECRET", AZURE_SIGNING_CLIENT_SECRET)
            .add("ACCOUNT_NAME", AZURE_SIGNING_ACCOUNT_NAME)
            .add("CERT_PROFILE_NAME", AZURE_SIGNING_CERT_PROFILE_NAME)
            .add("ENDPOINT", AZURE_SIGNING_ENDPOINT)
            .add("FILE_DIGEST", "SHA256")
            .add("TIMESTAMP_DIGEST", "SHA256")
            .add("TIMESTAMP_SERVER", "http://timestamp.acs.microsoft.com"),
    }
}

pub fn one_workflow_per_non_main_branch() -> Concurrency {
    one_workflow_per_non_main_branch_and_token("")
}

pub fn one_workflow_per_non_main_branch_and_token<T: AsRef<str>>(token: T) -> Concurrency {
    Concurrency::default()
        .group(format!(
            concat!(
                "${{{{ github.workflow }}}}-${{{{ github.ref_name }}}}-",
                "${{{{ github.ref_name == 'main' && github.sha || 'anysha' }}}}{}"
            ),
            token.as_ref()
        ))
        .cancel_in_progress(true)
}

pub(crate) fn allow_concurrent_runs() -> Concurrency {
    Concurrency::default()
        .group("${{ github.workflow }}-${{ github.ref_name }}-${{ github.run_id }}")
        .cancel_in_progress(true)
}

// Represents a pattern to check for changed files and corresponding output variable
pub struct PathCondition {
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
                .add_need(set_by_step.clone())
                .cond(Expression::new(format!(
                    "needs.{}.outputs.{} == 'true'",
                    &set_by_step, self.name
                ))),
        }
    }
}

pub(crate) struct StepOutput {
    pub name: &'static str,
    step_id: String,
}

impl StepOutput {
    pub fn new<T>(step: &Step<T>, name: &'static str) -> Self {
        Self {
            name,
            step_id: step
                .value
                .id
                .clone()
                .expect("Steps that produce outputs must have an ID"),
        }
    }

    pub fn expr(&self) -> String {
        format!("steps.{}.outputs.{}", self.step_id, self.name)
    }

    pub fn as_job_output(self, job: &NamedJob) -> JobOutput {
        JobOutput {
            job_name: job.name.clone(),
            name: self.name,
        }
    }
}

impl serde::Serialize for StepOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for StepOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{{ {} }}}}", self.expr())
    }
}

pub(crate) struct JobOutput {
    job_name: String,
    name: &'static str,
}

impl JobOutput {
    pub fn expr(&self) -> String {
        format!("needs.{}.outputs.{}", self.job_name, self.name)
    }
}

impl serde::Serialize for JobOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for JobOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{{ {} }}}}", self.expr())
    }
}

pub struct WorkflowInput {
    pub input_type: &'static str,
    pub name: &'static str,
    pub default: Option<String>,
    pub description: Option<String>,
}

impl WorkflowInput {
    pub fn string(name: &'static str, default: Option<String>) -> Self {
        Self {
            input_type: "string",
            name,
            default,
            description: None,
        }
    }

    pub fn bool(name: &'static str, default: Option<bool>) -> Self {
        Self {
            input_type: "boolean",
            name,
            default: default.as_ref().map(ToString::to_string),
            description: None,
        }
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn input(&self) -> WorkflowDispatchInput {
        WorkflowDispatchInput {
            description: self
                .description
                .clone()
                .unwrap_or_else(|| self.name.to_owned()),
            required: self.default.is_none(),
            input_type: self.input_type.to_owned(),
            default: self.default.clone(),
        }
    }

    pub fn call_input(&self) -> WorkflowCallInput {
        WorkflowCallInput {
            description: self.name.to_owned(),
            required: self.default.is_none(),
            input_type: self.input_type.to_owned(),
            default: self.default.clone(),
        }
    }

    pub(crate) fn expr(&self) -> String {
        format!("inputs.{}", self.name)
    }
}

impl std::fmt::Display for WorkflowInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{{ {} }}}}", self.expr())
    }
}

impl serde::Serialize for WorkflowInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub(crate) struct WorkflowSecret {
    pub name: &'static str,
    description: String,
    required: bool,
}

impl WorkflowSecret {
    pub fn new(name: &'static str, description: impl ToString) -> Self {
        Self {
            name,
            description: description.to_string(),
            required: true,
        }
    }

    pub fn secret_configuration(&self) -> WorkflowCallSecret {
        WorkflowCallSecret {
            description: self.description.clone(),
            required: self.required,
        }
    }
}

impl std::fmt::Display for WorkflowSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{{ secrets.{} }}}}", self.name)
    }
}

impl serde::Serialize for WorkflowSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub mod assets {
    // NOTE: these asset names also exist in the zed.dev codebase.
    pub const MAC_AARCH64: &str = "Zed-aarch64.dmg";
    pub const MAC_X86_64: &str = "Zed-x86_64.dmg";
    pub const LINUX_AARCH64: &str = "zed-linux-aarch64.tar.gz";
    pub const LINUX_X86_64: &str = "zed-linux-x86_64.tar.gz";
    pub const WINDOWS_X86_64: &str = "Zed-x86_64.exe";
    pub const WINDOWS_AARCH64: &str = "Zed-aarch64.exe";

    pub const REMOTE_SERVER_MAC_AARCH64: &str = "zed-remote-server-macos-aarch64.gz";
    pub const REMOTE_SERVER_MAC_X86_64: &str = "zed-remote-server-macos-x86_64.gz";
    pub const REMOTE_SERVER_LINUX_AARCH64: &str = "zed-remote-server-linux-aarch64.gz";
    pub const REMOTE_SERVER_LINUX_X86_64: &str = "zed-remote-server-linux-x86_64.gz";

    pub fn all() -> Vec<&'static str> {
        vec![
            MAC_AARCH64,
            MAC_X86_64,
            LINUX_AARCH64,
            LINUX_X86_64,
            WINDOWS_X86_64,
            WINDOWS_AARCH64,
            REMOTE_SERVER_MAC_AARCH64,
            REMOTE_SERVER_MAC_X86_64,
            REMOTE_SERVER_LINUX_AARCH64,
            REMOTE_SERVER_LINUX_X86_64,
        ]
    }
}

use gh_workflow::WorkflowCallInput;

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

pub fn input(name: &str, input: WorkflowCallInput) -> (String, (&str, WorkflowCallInput)) {
    return (format!("${{{{ inputs.{name} }}}}"), (name, input));
}

secret!(GITHUB_TOKEN);
secret!(CACHIX_AUTH_TOKEN);
secret!(ZED_CLIENT_CHECKSUM_SEED);
secret!(ZED_SENTRY_MINIDUMP_ENDPOINT);
secret!(ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON);
secret!(MACOS_CERTIFICATE);
secret!(MACOS_CERTIFICATE_PASSWORD);
secret!(APPLE_NOTARIZATION_KEY);
secret!(APPLE_NOTARIZATION_KEY_ID);
secret!(APPLE_NOTARIZATION_ISSUER_ID);
secret!(SENTRY_AUTH_TOKEN);
secret!(AZURE_SIGNING_TENANT_ID);
secret!(AZURE_SIGNING_CLIENT_ID);
secret!(AZURE_SIGNING_CLIENT_SECRET);

// todo(ci) make these secrets too...
var!(AZURE_SIGNING_ACCOUNT_NAME);
var!(AZURE_SIGNING_CERT_PROFILE_NAME);
var!(AZURE_SIGNING_ENDPOINT);

pub const GITHUB_SHA: &str = "${{ github.event.pull_request.head.sha || github.sha }}";

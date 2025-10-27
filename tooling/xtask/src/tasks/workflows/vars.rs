use gh_workflow::WorkflowCallInput;

macro_rules! secret {
    ($secret_name:ident) => {
        pub const $secret_name: &str = concat!("${{ secrets.", stringify!($secret_name), " }}");
    };
}

secret!(GITHUB_TOKEN);
secret!(CACHIX_AUTH_TOKEN);
secret!(ZED_CLIENT_CHECKSUM_SEED);
secret!(ZED_MINIDUMP_ENDPOINT);
secret!(ZED_CLOUD_PROVIDER_ADDITIONAL_MODELS_JSON);
secret!(MACOS_CERTIFICATE);
secret!(MACOS_CERTIFICATE_PASSWORD);
secret!(APPLE_NOTARIZATION_KEY);
secret!(APPLE_NOTARIZATION_KEY_ID);
secret!(APPLE_NOTARIZATION_ISSUER_ID);

pub fn input(name: &str, input: WorkflowCallInput) -> (String, (&str, WorkflowCallInput)) {
    return (format!("${{{{ inputs.{name} }}}}"), (name, input));
}

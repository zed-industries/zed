use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context as _, Result, anyhow, bail};
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{AsyncReadExt, StreamExt as _, channel::oneshot, stream};
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, Context, Entity, FutureExt as _, Global,
    SharedString, Subscription, Task, TaskExt as _,
};
use http_client::{AsyncBody, HttpClient, StatusCode};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use settings::{Settings as _, SettingsFile, SettingsStore};
use sha2::{Digest as _, Sha256};
use url::Url;
use util::ResultExt as _;

use crate::project_settings::{ContextServerSettings, ProjectSettings};

const REGISTRY_API_BASE_URL: &str = "https://registry.modelcontextprotocol.io/v0.1";
const REGISTRY_PAGE_LIMIT: usize = 100;
const REGISTRY_FETCH_TIMEOUT: Duration = Duration::from_secs(30);
const REGISTRY_SEARCH_FETCH_TIMEOUT: Duration = Duration::from_secs(60);
const REGISTRY_INSTALLED_REFRESH_CONCURRENCY: usize = 2;
const REGISTRY_INPUTS_CREDENTIAL_VERSION: u32 = 1;
const REGISTRY_INPUTS_CREDENTIAL_USERNAME: &str = "mcp-registry-inputs";

#[derive(Serialize, Deserialize)]
struct StoredRegistryInputs {
    pub version: u32,
    pub inputs: HashMap<String, Vec<String>>,
}

pub async fn read_server_secrets(
    credential_id: &str,
    cx: &AsyncApp,
) -> Result<HashMap<String, Vec<String>>> {
    let key = registry_inputs_credential_key(credential_id)?;
    let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
    let Some((username, payload)) = credentials_provider.read_credentials(&key, cx).await? else {
        return Ok(HashMap::default());
    };

    if username != REGISTRY_INPUTS_CREDENTIAL_USERNAME {
        bail!("invalid username for stored MCP Registry inputs");
    }

    let stored: StoredRegistryInputs =
        serde_json::from_slice(&payload).context("parsing stored MCP Registry inputs")?;
    if stored.version != REGISTRY_INPUTS_CREDENTIAL_VERSION {
        bail!(
            "unsupported stored MCP Registry inputs version {}",
            stored.version
        );
    }

    Ok(stored.inputs)
}

pub async fn write_server_secrets(
    credential_id: &str,
    inputs: &HashMap<String, Vec<String>>,
    cx: &AsyncApp,
) -> Result<()> {
    let key = registry_inputs_credential_key(credential_id)?;
    let payload = serde_json::to_vec(&StoredRegistryInputs {
        version: REGISTRY_INPUTS_CREDENTIAL_VERSION,
        inputs: inputs.clone(),
    })
    .context("serializing MCP Registry inputs")?;
    let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
    credentials_provider
        .write_credentials(&key, REGISTRY_INPUTS_CREDENTIAL_USERNAME, &payload, cx)
        .await
}

pub async fn delete_server_secrets(credential_id: &str, cx: &AsyncApp) -> Result<()> {
    let key = registry_inputs_credential_key(credential_id)?;
    let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
    credentials_provider.delete_credentials(&key, cx).await
}

pub fn registry_credential_is_referenced(credential_id: &str, cx: &App) -> bool {
    let settings_store = cx.global::<SettingsStore>();
    if settings_store
        .get_all_files()
        .into_iter()
        .filter_map(|file| settings_store.get_content_for_file(file))
        .any(|settings| settings_reference_registry_credential(settings, credential_id))
        || settings_store
            .get_content_for_file(SettingsFile::Global)
            .is_some_and(|settings| settings_reference_registry_credential(settings, credential_id))
    {
        return true;
    }

    settings_store.raw_user_settings().is_some_and(|settings| {
        settings
            .profiles
            .values()
            .any(|profile| settings_reference_registry_credential(&profile.settings, credential_id))
            || settings::ReleaseChannelOverrides::OVERRIDE_KEYS
                .iter()
                .filter_map(|key| settings.release_channel_overrides.get_by_key(key))
                .any(|settings| settings_reference_registry_credential(settings, credential_id))
            || settings::PlatformOverrides::OVERRIDE_KEYS
                .iter()
                .filter_map(|key| settings.platform_overrides.get_by_key(key))
                .any(|settings| settings_reference_registry_credential(settings, credential_id))
    })
}

fn settings_reference_registry_credential(
    settings: &settings::SettingsContent,
    credential_id: &str,
) -> bool {
    settings.project.context_servers.values().any(|settings| {
        matches!(
            settings,
            settings::ContextServerSettingsContent::Registry { registry, .. }
                if registry.credential_id.as_deref() == Some(credential_id)
        )
    })
}

fn registry_inputs_credential_key(credential_id: &str) -> Result<String> {
    if credential_id.is_empty()
        || credential_id.len() > 128
        || !credential_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        bail!("MCP Registry credential ID is invalid");
    }
    Ok(format!(
        "mcp-registry-inputs:v{REGISTRY_INPUTS_CREDENTIAL_VERSION}:{credential_id}"
    ))
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerListResponse {
    #[serde(default)]
    pub servers: Vec<ServerResponse>,
    #[serde(default)]
    pub metadata: ResponseMetadata,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    #[serde(default, alias = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub count: Option<usize>,
    #[serde(default)]
    pub total: Option<usize>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerResponse {
    #[serde(default)]
    pub server: Server,
    #[serde(rename = "_meta", default)]
    pub metadata: ServerMetadata,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum McpRegistryInstallationSource {
    Package {
        registry_type: String,
        identifier: String,
    },
    Remote {
        url: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpRegistryInstallationOption {
    pub label: String,
    pub source: McpRegistryInstallationSource,
    pub inputs: Vec<McpRegistryInputDescriptor>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpRegistryInputDescriptor {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    pub repeated: bool,
    pub format: String,
    pub default: Option<String>,
    pub placeholder: Option<String>,
    pub choices: Vec<String>,
}

impl ServerResponse {
    pub fn name(&self) -> &str {
        &self.server.name
    }

    pub fn title(&self) -> Option<&str> {
        self.server.title.as_deref()
    }

    pub fn description(&self) -> &str {
        &self.server.description
    }

    pub fn version(&self) -> &str {
        &self.server.version
    }

    pub fn repository(&self) -> Option<&Repository> {
        self.server.repository.as_ref()
    }

    pub fn website(&self) -> Option<&str> {
        self.server.website_url.as_deref()
    }

    pub fn installation_options(&self) -> Vec<McpRegistryInstallationOption> {
        if self.is_deleted() {
            return Vec::new();
        }

        let mut options = Vec::with_capacity(
            self.server
                .packages
                .len()
                .saturating_add(self.server.remotes.len()),
        );
        let package_source_counts = self
            .server
            .packages
            .iter()
            .filter(|package| package.is_supported_installation())
            .fold(HashMap::default(), |mut counts, package| {
                *counts
                    .entry((&package.registry_type, &package.identifier))
                    .or_insert(0) += 1;
                counts
            });
        let remote_source_counts = self
            .server
            .remotes
            .iter()
            .filter_map(|remote| match remote {
                Transport::StreamableHttp(transport) if transport.is_supported_installation() => {
                    Some(&transport.url)
                }
                _ => None,
            })
            .fold(HashMap::default(), |mut counts, url| {
                *counts.entry(url).or_insert(0) += 1;
                counts
            });

        for package in &self.server.packages {
            if !package.is_supported_installation()
                || package_source_counts
                    .get(&(&package.registry_type, &package.identifier))
                    .copied()
                    != Some(1)
            {
                continue;
            }

            let label_prefix = package
                .runtime_hint
                .as_deref()
                .filter(|runtime_hint| !runtime_hint.is_empty())
                .unwrap_or(&package.registry_type);
            options.push(McpRegistryInstallationOption {
                label: format!("{label_prefix} · {}", package.identifier),
                source: McpRegistryInstallationSource::Package {
                    registry_type: package.registry_type.clone(),
                    identifier: package.identifier.clone(),
                },
                inputs: package.input_descriptors(),
            });
        }

        for remote in &self.server.remotes {
            let Transport::StreamableHttp(transport) = remote else {
                continue;
            };
            if !transport.is_supported_installation()
                || remote_source_counts.get(&transport.url).copied() != Some(1)
            {
                continue;
            }

            options.push(McpRegistryInstallationOption {
                label: transport.url.clone(),
                source: McpRegistryInstallationSource::Remote {
                    url: transport.url.clone(),
                },
                inputs: transport.input_descriptors(),
            });
        }

        options
    }

    pub fn is_deleted(&self) -> bool {
        self.metadata.official.status.as_deref() == Some("deleted")
    }
}

fn is_supported_npm_registry(registry_base_url: Option<&str>) -> bool {
    registry_base_url.is_none_or(|registry_base_url| {
        registry_base_url.is_empty()
            || registry_base_url.trim_end_matches('/') == "https://registry.npmjs.org"
    })
}

fn is_exact_package_version(version: &str) -> bool {
    semver::Version::parse(version).is_ok()
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerMetadata {
    #[serde(rename = "io.modelcontextprotocol.registry/official", default)]
    pub official: OfficialServerMetadata,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficialServerMetadata {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub status_changed_at: Option<String>,
    #[serde(default)]
    pub published_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub is_latest: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "$schema", default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub repository: Option<Repository>,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub website_url: Option<String>,
    #[serde(default)]
    pub icons: Vec<Icon>,
    #[serde(default)]
    pub packages: Vec<Package>,
    #[serde(default)]
    pub remotes: Vec<Transport>,
    #[serde(rename = "_meta", default)]
    pub metadata: HashMap<String, Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub subfolder: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    #[serde(default)]
    pub src: String,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub sizes: Vec<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    #[serde(default)]
    pub registry_type: String,
    #[serde(default)]
    pub registry_base_url: Option<String>,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub file_sha256: Option<String>,
    #[serde(default)]
    pub runtime_hint: Option<String>,
    #[serde(default)]
    pub transport: Transport,
    #[serde(default)]
    pub runtime_arguments: Vec<Argument>,
    #[serde(default)]
    pub package_arguments: Vec<Argument>,
    #[serde(default)]
    pub environment_variables: Vec<KeyValueInput>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Package {
    pub fn is_supported_installation(&self) -> bool {
        self.registry_type == "npm"
            && !self.identifier.is_empty()
            && self.file_sha256.is_none()
            && self
                .version
                .as_deref()
                .is_some_and(is_exact_package_version)
            && is_supported_npm_registry(self.registry_base_url.as_deref())
            && self
                .runtime_hint
                .as_deref()
                .is_none_or(|runtime_hint| runtime_hint.is_empty() || runtime_hint == "npx")
            && matches!(self.transport, Transport::Stdio(_))
            && !self
                .runtime_arguments
                .iter()
                .chain(&self.package_arguments)
                .any(argument_contains_secret_input)
            && !self
                .runtime_arguments
                .iter()
                .any(argument_overrides_npm_policy)
            && !self
                .environment_variables
                .iter()
                .any(|input| is_npm_bootstrap_environment_variable(&input.name))
    }

    pub fn input_descriptors(&self) -> Vec<McpRegistryInputDescriptor> {
        let mut inputs = Vec::new();

        append_argument_input_descriptors(&mut inputs, "runtime_argument", &self.runtime_arguments);
        append_argument_input_descriptors(&mut inputs, "package_argument", &self.package_arguments);
        append_key_value_input_descriptors(&mut inputs, "environment", &self.environment_variables);

        if let Transport::StreamableHttp(transport) | Transport::Sse(transport) = &self.transport {
            transport.append_input_descriptors(&mut inputs);
        }

        inputs
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Transport {
    Stdio(StdioTransport),
    StreamableHttp(HttpTransport),
    Sse(HttpTransport),
    Unknown(Value),
}

impl Default for Transport {
    fn default() -> Self {
        Self::Unknown(Value::Null)
    }
}

impl<'de> Deserialize<'de> for Transport {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let transport_type = value.get("type").and_then(Value::as_str);

        match transport_type {
            Some("stdio") => serde_json::from_value(value.clone())
                .map(Self::Stdio)
                .or_else(|_| Ok(Self::Unknown(value))),
            Some("streamable-http") => serde_json::from_value(value.clone())
                .map(Self::StreamableHttp)
                .or_else(|_| Ok(Self::Unknown(value))),
            Some("sse") => serde_json::from_value(value.clone())
                .map(Self::Sse)
                .or_else(|_| Ok(Self::Unknown(value))),
            _ => Ok(Self::Unknown(value)),
        }
    }
}

impl Transport {
    pub fn transport_type(&self) -> Option<&str> {
        match self {
            Self::Stdio(transport) => Some(&transport.transport_type),
            Self::StreamableHttp(transport) | Self::Sse(transport) => {
                Some(&transport.transport_type)
            }
            Self::Unknown(value) => value.get("type").and_then(Value::as_str),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StdioTransport {
    #[serde(rename = "type", default)]
    pub transport_type: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HttpTransport {
    #[serde(rename = "type", default)]
    pub transport_type: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: Vec<KeyValueInput>,
    #[serde(default)]
    pub variables: HashMap<String, Input>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl HttpTransport {
    pub fn is_supported_installation(&self) -> bool {
        self.transport_type == "streamable-http"
            && !self.url.is_empty()
            && !self.variables.iter().any(|(name, input)| {
                template_references_variable(&self.url, name) && input.is_secret
            })
    }

    pub fn input_descriptors(&self) -> Vec<McpRegistryInputDescriptor> {
        let mut inputs = Vec::new();
        self.append_input_descriptors(&mut inputs);
        inputs
    }

    fn append_input_descriptors(&self, inputs: &mut Vec<McpRegistryInputDescriptor>) {
        append_key_value_input_descriptors(inputs, "header", &self.headers);

        let mut variables = self.variables.iter().collect::<Vec<_>>();
        variables.sort_unstable_by_key(|(name, _)| *name);
        for (name, input) in variables {
            if input.value.is_some() || !template_references_variable(&self.url, name) {
                continue;
            }
            inputs.push(input_descriptor(
                format!("variable:{name}"),
                name.clone(),
                input,
                false,
                None,
            ));
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Argument {
    Named(NamedArgument),
    Positional(PositionalArgument),
    Unknown(Value),
}

impl<'de> Deserialize<'de> for Argument {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let argument_type = value.get("type").and_then(Value::as_str);

        match argument_type {
            Some("named") => serde_json::from_value(value.clone())
                .map(Self::Named)
                .or_else(|_| Ok(Self::Unknown(value))),
            Some("positional") => serde_json::from_value(value.clone())
                .map(Self::Positional)
                .or_else(|_| Ok(Self::Unknown(value))),
            _ => Ok(Self::Unknown(value)),
        }
    }
}

impl Argument {
    pub fn input(&self) -> Option<&InputWithVariables> {
        match self {
            Self::Named(argument) => Some(&argument.input),
            Self::Positional(argument) => Some(&argument.input),
            Self::Unknown(_) => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedArgument {
    #[serde(rename = "type", default)]
    pub argument_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(flatten)]
    pub input: InputWithVariables,
    #[serde(default)]
    pub is_repeated: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionalArgument {
    #[serde(rename = "type", default)]
    pub argument_type: String,
    #[serde(flatten)]
    pub input: InputWithVariables,
    #[serde(default)]
    pub value_hint: Option<String>,
    #[serde(default)]
    pub is_repeated: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValueInput {
    #[serde(default)]
    pub name: String,
    #[serde(flatten)]
    pub input: InputWithVariables,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InputWithVariables {
    #[serde(flatten)]
    pub input: Input,
    #[serde(default)]
    pub variables: HashMap<String, Input>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_required: bool,
    #[serde(default = "default_input_format")]
    pub format: String,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub is_secret: bool,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub choices: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            description: None,
            is_required: false,
            format: default_input_format(),
            value: None,
            is_secret: false,
            default: None,
            placeholder: None,
            choices: Vec::new(),
            extra: HashMap::default(),
        }
    }
}

fn default_input_format() -> String {
    "string".to_owned()
}

fn append_argument_input_descriptors(
    descriptors: &mut Vec<McpRegistryInputDescriptor>,
    id_prefix: &str,
    arguments: &[Argument],
) {
    for (index, argument) in arguments.iter().enumerate() {
        let id = argument_input_id(id_prefix, argument);
        match argument {
            Argument::Named(argument) => {
                if !is_standalone_flag(&argument.input) {
                    append_input_with_variables_descriptors(
                        descriptors,
                        id,
                        argument.name.clone(),
                        &argument.input,
                        argument.is_repeated,
                    );
                }
            }
            Argument::Positional(argument) => append_input_with_variables_descriptors(
                descriptors,
                id,
                argument
                    .value_hint
                    .clone()
                    .unwrap_or_else(|| format!("Argument {}", index.saturating_add(1))),
                &argument.input,
                argument.is_repeated,
            ),
            Argument::Unknown(_) => {}
        }
    }
}

fn append_key_value_input_descriptors(
    descriptors: &mut Vec<McpRegistryInputDescriptor>,
    id_prefix: &str,
    inputs: &[KeyValueInput],
) {
    for input in inputs {
        if input.name.is_empty() {
            continue;
        }
        append_input_with_variables_descriptors(
            descriptors,
            format!("{id_prefix}:{}", input.name),
            input.name.clone(),
            &input.input,
            false,
        );
    }
}

fn append_input_with_variables_descriptors(
    descriptors: &mut Vec<McpRegistryInputDescriptor>,
    id: String,
    label: String,
    input: &InputWithVariables,
    repeated: bool,
) {
    if input.input.value.is_none() {
        descriptors.push(input_descriptor(id, label, &input.input, repeated, None));
        return;
    }

    let template = input.input.value.as_deref().unwrap_or_default();
    let mut variables = input.variables.iter().collect::<Vec<_>>();
    variables.sort_unstable_by_key(|(name, _)| *name);
    for (name, variable) in variables {
        if variable.value.is_some() || !template_references_variable(template, name) {
            continue;
        }
        descriptors.push(input_descriptor(
            format!("{id}.variable:{name}"),
            name.clone(),
            variable,
            repeated,
            Some(&input.input),
        ));
    }
}

fn argument_contains_secret_input(argument: &Argument) -> bool {
    argument.input().is_some_and(|input| {
        input.input.is_secret
            || input.input.value.as_deref().is_some_and(|template| {
                input.variables.iter().any(|(name, variable)| {
                    template_references_variable(template, name) && variable.is_secret
                })
            })
    })
}

fn argument_overrides_npm_policy(argument: &Argument) -> bool {
    match argument {
        Argument::Named(argument) => is_npm_policy_argument(&argument.name),
        Argument::Positional(argument) => argument
            .input
            .input
            .value
            .as_deref()
            .or(argument.input.input.default.as_deref())
            .is_some_and(is_npm_policy_argument),
        Argument::Unknown(_) => false,
    }
}

fn is_npm_policy_argument(argument: &str) -> bool {
    let key = argument
        .trim_start_matches('-')
        .split('=')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .replace('_', "-");
    let key = key.strip_prefix("no-").unwrap_or(&key);
    const PROTECTED_KEYS: &[&str] = &[
        "registry",
        "userconfig",
        "globalconfig",
        "before",
        "min-release-age",
        "min-release-age-exclude",
        "min-release-age-exclude-scopes",
    ];

    key.is_empty()
        || PROTECTED_KEYS
            .iter()
            .any(|protected_key| protected_key.starts_with(&key))
        || key.rsplit_once(':').is_some_and(|(_, scoped_key)| {
            !scoped_key.is_empty() && "registry".starts_with(scoped_key)
        })
}

fn is_npm_bootstrap_environment_variable(name: &str) -> bool {
    let name = name.to_ascii_uppercase();
    name.starts_with("NPM_CONFIG_")
        || matches!(
            name.as_str(),
            "HOME"
                | "USERPROFILE"
                | "APPDATA"
                | "PATH"
                | "NODE_OPTIONS"
                | "NODE_EXTRA_CA_CERTS"
                | "NODE_TLS_REJECT_UNAUTHORIZED"
                | "NPM_TOKEN"
                | "NPM_AUTH_TOKEN"
                | "NODE_AUTH_TOKEN"
                | "HTTP_PROXY"
                | "HTTPS_PROXY"
                | "ALL_PROXY"
                | "NO_PROXY"
                | "SSL_CERT_FILE"
                | "SSL_CERT_DIR"
        )
}

fn input_descriptor(
    id: String,
    label: String,
    input: &Input,
    repeated: bool,
    inherited_input: Option<&Input>,
) -> McpRegistryInputDescriptor {
    McpRegistryInputDescriptor {
        id,
        label,
        description: input.description.clone(),
        required: input.is_required || inherited_input.is_some_and(|input| input.is_required),
        secret: input.is_secret || inherited_input.is_some_and(|input| input.is_secret),
        repeated,
        format: input.format.clone(),
        default: input.default.clone(),
        placeholder: input.placeholder.clone(),
        choices: input.choices.clone(),
    }
}

fn template_references_variable(template: &str, name: &str) -> bool {
    template.contains(&format!("{{{name}}}"))
}

fn argument_input_id(id_prefix: &str, argument: &Argument) -> String {
    let mut hasher = Sha256::new();
    match argument {
        Argument::Named(argument) => {
            update_fingerprint(&mut hasher, b"named");
            update_fingerprint(&mut hasher, argument.name.as_bytes());
            update_fingerprint(&mut hasher, &[u8::from(argument.is_repeated)]);
            update_input_with_variables_fingerprint(&mut hasher, &argument.input);
        }
        Argument::Positional(argument) => {
            update_fingerprint(&mut hasher, b"positional");
            update_optional_string_fingerprint(&mut hasher, argument.value_hint.as_deref());
            update_fingerprint(&mut hasher, &[u8::from(argument.is_repeated)]);
            update_input_with_variables_fingerprint(&mut hasher, &argument.input);
        }
        Argument::Unknown(value) => {
            update_fingerprint(&mut hasher, b"unknown");
            update_json_fingerprint(&mut hasher, value);
        }
    }
    format!("{id_prefix}:{:x}", hasher.finalize())
}

fn update_input_with_variables_fingerprint(hasher: &mut Sha256, input: &InputWithVariables) {
    update_input_fingerprint(hasher, &input.input);
    let Some(template) = input.input.value.as_deref() else {
        return;
    };
    let mut variables = input.variables.iter().collect::<Vec<_>>();
    variables.sort_unstable_by_key(|(name, _)| *name);
    for (name, variable) in variables {
        if !template_references_variable(template, name) {
            continue;
        }
        update_fingerprint(hasher, name.as_bytes());
        update_input_fingerprint(hasher, variable);
    }
}

fn update_input_fingerprint(hasher: &mut Sha256, input: &Input) {
    update_fingerprint(hasher, &[u8::from(input.is_required)]);
    update_fingerprint(hasher, input.format.as_bytes());
    update_optional_string_fingerprint(hasher, input.value.as_deref());
    update_fingerprint(hasher, &[u8::from(input.is_secret)]);
    update_optional_string_fingerprint(hasher, input.default.as_deref());

    let mut choices = input.choices.iter().collect::<Vec<_>>();
    choices.sort_unstable();
    for choice in choices {
        update_fingerprint(hasher, choice.as_bytes());
    }
}

fn update_optional_string_fingerprint(hasher: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            update_fingerprint(hasher, &[1]);
            update_fingerprint(hasher, value.as_bytes());
        }
        None => update_fingerprint(hasher, &[0]),
    }
}

fn update_json_fingerprint(hasher: &mut Sha256, value: &Value) {
    match value {
        Value::Null => update_fingerprint(hasher, b"null"),
        Value::Bool(value) => update_fingerprint(hasher, &[u8::from(*value)]),
        Value::Number(value) => update_fingerprint(hasher, value.to_string().as_bytes()),
        Value::String(value) => update_fingerprint(hasher, value.as_bytes()),
        Value::Array(values) => {
            update_fingerprint(hasher, b"array");
            for value in values {
                update_json_fingerprint(hasher, value);
            }
        }
        Value::Object(values) => {
            update_fingerprint(hasher, b"object");
            let mut values = values.iter().collect::<Vec<_>>();
            values.sort_unstable_by_key(|(key, _)| *key);
            for (key, value) in values {
                update_fingerprint(hasher, key.as_bytes());
                update_json_fingerprint(hasher, value);
            }
        }
    }
}

fn update_fingerprint(hasher: &mut Sha256, value: &[u8]) {
    hasher.update(value.len().to_le_bytes());
    hasher.update(value);
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResolvedMcpRegistryServer {
    Npm {
        package_spec: String,
        runtime_arguments: Vec<String>,
        package_arguments: Vec<String>,
        environment: HashMap<String, String>,
    },
    Http {
        url: Url,
        headers: HashMap<String, String>,
    },
}

pub fn resolve_server_configuration(
    server: &ServerResponse,
    source: &McpRegistryInstallationSource,
    settings_inputs: &HashMap<String, Vec<String>>,
    secret_inputs: &HashMap<String, Vec<String>>,
) -> Result<ResolvedMcpRegistryServer> {
    if server.is_deleted() {
        if let Some(status_message) = server
            .metadata
            .official
            .status_message
            .as_deref()
            .filter(|message| !message.trim().is_empty())
        {
            bail!(
                "MCP Registry server `{}` was removed: {status_message}",
                server.name()
            );
        }
        bail!("MCP Registry server `{}` was removed", server.name());
    }

    match source {
        McpRegistryInstallationSource::Package {
            registry_type,
            identifier,
        } => {
            let mut packages = server.server.packages.iter().filter(|package| {
                package.registry_type == *registry_type
                    && package.identifier == *identifier
                    && package.is_supported_installation()
            });
            let package = packages.next().with_context(|| {
                format!(
                    "MCP Registry server `{}` no longer offers the selected package",
                    server.name()
                )
            })?;
            if packages.next().is_some() {
                bail!(
                    "MCP Registry server `{}` offers multiple matching packages for `{identifier}`",
                    server.name()
                );
            }
            let version = package
                .version
                .as_deref()
                .context("supported npm package is missing its version")?;
            let runtime_arguments = resolve_arguments(
                "runtime_argument",
                &package.runtime_arguments,
                settings_inputs,
                secret_inputs,
            )?;
            let package_arguments = resolve_arguments(
                "package_argument",
                &package.package_arguments,
                settings_inputs,
                secret_inputs,
            )?;
            let environment = resolve_key_value_inputs(
                "environment",
                &package.environment_variables,
                settings_inputs,
                secret_inputs,
            )?;
            if let Some(argument) = runtime_arguments
                .iter()
                .find(|argument| is_npm_policy_argument(argument))
            {
                bail!("MCP Registry npm runtime argument `{argument}` overrides npm policy");
            }
            if let Some(name) = environment
                .keys()
                .find(|name| is_npm_bootstrap_environment_variable(name))
            {
                bail!("MCP Registry environment variable `{name}` affects npm bootstrap policy");
            }

            Ok(ResolvedMcpRegistryServer::Npm {
                package_spec: node_runtime::npm_package_spec_with_version_ceiling(&format!(
                    "{}@{version}",
                    package.identifier
                )),
                runtime_arguments,
                package_arguments,
                environment,
            })
        }
        McpRegistryInstallationSource::Remote { url } => {
            let mut transports =
                server
                    .server
                    .remotes
                    .iter()
                    .filter_map(|transport| match transport {
                        Transport::StreamableHttp(transport)
                            if transport.url == *url && transport.is_supported_installation() =>
                        {
                            Some(transport)
                        }
                        _ => None,
                    });
            let transport = transports.next().with_context(|| {
                format!(
                    "MCP Registry server `{}` no longer offers the selected remote endpoint",
                    server.name()
                )
            })?;
            if transports.next().is_some() {
                bail!(
                    "MCP Registry server `{}` offers multiple matching remote endpoints for `{url}`",
                    server.name()
                );
            }

            let url = resolve_http_url(transport, settings_inputs, secret_inputs)?;
            let headers = resolve_key_value_inputs(
                "header",
                &transport.headers,
                settings_inputs,
                secret_inputs,
            )?;
            Ok(ResolvedMcpRegistryServer::Http { url, headers })
        }
    }
}

fn resolve_http_url(
    transport: &HttpTransport,
    settings_inputs: &HashMap<String, Vec<String>>,
    secret_inputs: &HashMap<String, Vec<String>>,
) -> Result<Url> {
    let template = &transport.url;
    let mut url = template.clone();
    let mut variables = transport.variables.iter().collect::<Vec<_>>();
    variables.sort_unstable_by_key(|(name, _)| *name);
    for (name, input) in variables {
        if !template_references_variable(template, name) {
            continue;
        }
        let id = format!("variable:{name}");
        let values = resolve_input(input, &id, false, settings_inputs, secret_inputs)?;
        let placeholder = format!("{{{name}}}");
        let Some(value) = values.first() else {
            if url.contains(&placeholder) {
                bail!("MCP Registry input `{id}` is required by the remote URL");
            }
            continue;
        };
        url = url.replace(&placeholder, value);
    }

    if url.contains(['{', '}']) {
        bail!("MCP Registry remote URL contains an unresolved variable");
    }
    let url = Url::parse(&url).context("parsing resolved MCP Registry remote URL")?;
    if !matches!(url.scheme(), "http" | "https") {
        bail!("MCP Registry remote URL must use HTTP or HTTPS");
    }
    Ok(url)
}

fn resolve_arguments(
    id_prefix: &str,
    arguments: &[Argument],
    settings_inputs: &HashMap<String, Vec<String>>,
    secret_inputs: &HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    let mut resolved = Vec::new();
    for argument in arguments {
        let id = argument_input_id(id_prefix, argument);
        match argument {
            Argument::Named(argument) => {
                if argument.name.is_empty() {
                    bail!("MCP Registry named argument `{id}` has no name");
                }
                if is_standalone_flag(&argument.input) {
                    resolved.push(argument.name.clone());
                    continue;
                }
                let values = resolve_input_with_variables(
                    &argument.input,
                    &id,
                    argument.is_repeated,
                    settings_inputs,
                    secret_inputs,
                )?;
                for value in values {
                    resolved.push(argument.name.clone());
                    resolved.push(value);
                }
            }
            Argument::Positional(argument) => {
                resolved.extend(resolve_input_with_variables(
                    &argument.input,
                    &id,
                    argument.is_repeated,
                    settings_inputs,
                    secret_inputs,
                )?);
            }
            Argument::Unknown(_) => {
                bail!("MCP Registry argument `{id}` uses an unsupported type");
            }
        }
    }
    Ok(resolved)
}

fn resolve_key_value_inputs(
    id_prefix: &str,
    inputs: &[KeyValueInput],
    settings_inputs: &HashMap<String, Vec<String>>,
    secret_inputs: &HashMap<String, Vec<String>>,
) -> Result<HashMap<String, String>> {
    let mut resolved = HashMap::default();
    for input in inputs {
        if input.name.is_empty() {
            bail!("MCP Registry {id_prefix} input has no name");
        }
        let id = format!("{id_prefix}:{}", input.name);
        let values =
            resolve_input_with_variables(&input.input, &id, false, settings_inputs, secret_inputs)?;
        let Some(value) = values.into_iter().next() else {
            continue;
        };
        if resolved.insert(input.name.clone(), value).is_some() {
            bail!(
                "MCP Registry contains duplicate {id_prefix} input `{}`",
                input.name
            );
        }
    }
    Ok(resolved)
}

fn resolve_input_with_variables(
    input: &InputWithVariables,
    id: &str,
    repeated: bool,
    settings_inputs: &HashMap<String, Vec<String>>,
    secret_inputs: &HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    let Some(template) = input.input.value.as_ref() else {
        return resolve_input(&input.input, id, repeated, settings_inputs, secret_inputs);
    };

    let mut variables = input.variables.iter().collect::<Vec<_>>();
    variables.sort_unstable_by_key(|(name, _)| *name);
    let mut resolved_variables = Vec::with_capacity(variables.len());
    let mut value_count = 1;
    for (name, variable) in variables {
        if !template_references_variable(template, name) {
            continue;
        }
        let variable_id = format!("{id}.variable:{name}");
        let variable = input_with_inherited_template_flags(variable, &input.input);
        let values = resolve_input(
            &variable,
            &variable_id,
            repeated,
            settings_inputs,
            secret_inputs,
        )?;
        if values.is_empty() {
            return Ok(Vec::new());
        }
        value_count = value_count.max(values.len());
        resolved_variables.push((name, values));
    }

    for (name, values) in &resolved_variables {
        if values.len() != 1 && values.len() != value_count {
            bail!(
                "repeated MCP Registry input `{id}.variable:{name}` has {} values, expected 1 or {value_count}",
                values.len()
            );
        }
    }

    let mut resolved = Vec::with_capacity(value_count);
    for index in 0..value_count {
        let mut value = template.clone();
        for (name, values) in &resolved_variables {
            let replacement = values
                .get(index)
                .or_else(|| values.first())
                .context("resolved MCP Registry template variable has no value")?;
            value = value.replace(&format!("{{{name}}}"), replacement);
        }
        validate_resolved_input(&input.input, id, &value)?;
        resolved.push(value);
    }
    Ok(resolved)
}

fn input_with_inherited_template_flags(input: &Input, outer_input: &Input) -> Input {
    let mut input = input.clone();
    input.is_required |= outer_input.is_required;
    input.is_secret |= outer_input.is_secret;
    input
}

fn resolve_input(
    input: &Input,
    id: &str,
    repeated: bool,
    settings_inputs: &HashMap<String, Vec<String>>,
    secret_inputs: &HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    if let Some(value) = input.value.as_ref() {
        validate_resolved_input(input, id, value)?;
        return Ok(vec![value.clone()]);
    }

    let configured_inputs = if input.is_secret {
        secret_inputs
    } else {
        settings_inputs
    };
    let values = configured_inputs
        .get(id)
        .cloned()
        .or_else(|| input.default.as_ref().map(|default| vec![default.clone()]))
        .unwrap_or_default();

    if input.is_required && (values.is_empty() || values.iter().any(|value| value.is_empty())) {
        bail!("MCP Registry input `{id}` is required");
    }
    if !repeated && values.len() > 1 {
        bail!("MCP Registry input `{id}` does not accept repeated values");
    }
    for value in &values {
        validate_resolved_input(input, id, value)?;
    }
    Ok(values)
}

fn validate_resolved_input(input: &Input, id: &str, value: &str) -> Result<()> {
    if input.is_required && value.is_empty() {
        bail!("MCP Registry input `{id}` is required");
    }
    if !input.choices.is_empty() && !input.choices.iter().any(|choice| choice == value) {
        bail!("MCP Registry input `{id}` is not one of its allowed choices");
    }
    match input.format.as_str() {
        "string" | "filepath" => {}
        "boolean" => {
            value
                .parse::<bool>()
                .with_context(|| format!("MCP Registry input `{id}` must be a boolean"))?;
        }
        "number" => {
            let number = value
                .parse::<f64>()
                .with_context(|| format!("MCP Registry input `{id}` must be a number"))?;
            if !number.is_finite() {
                bail!("MCP Registry input `{id}` must be a finite number");
            }
        }
        format => bail!("MCP Registry input `{id}` uses unsupported format `{format}`"),
    }
    Ok(())
}

fn is_standalone_flag(input: &InputWithVariables) -> bool {
    input.input.value.is_none()
        && !input.input.is_required
        && !input.input.is_secret
        && input.input.default.is_none()
        && input.input.placeholder.is_none()
        && input.input.choices.is_empty()
        && input.input.format == "string"
        && input.variables.is_empty()
}

struct GlobalMcpRegistryStore(Entity<McpRegistryStore>);

impl Global for GlobalMcpRegistryStore {}

pub struct McpRegistryStore {
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    registry_api_base_url: String,
    servers: Vec<ServerResponse>,
    cached_servers: HashMap<String, ServerResponse>,
    installation_source_hints: HashMap<String, McpRegistryInstallationSource>,
    pending_server_operations: HashSet<String>,
    installed_server_names: HashSet<String>,
    refreshed_server_names: HashSet<String>,
    server_generations: HashMap<String, u64>,
    query: Option<String>,
    next_cursor: Option<String>,
    is_fetching: bool,
    fetch_error: Option<SharedString>,
    list_generation: u64,
    installed_refresh_generation: u64,
    cache_loaded: bool,
    cache_load_waiters: Vec<oneshot::Sender<()>>,
    pending_list_fetch: Option<Task<()>>,
    pending_installed_refresh: Option<Task<()>>,
    pending_installed_refresh_names: HashSet<String>,
    installed_refresh_waiters:
        HashMap<String, Vec<oneshot::Sender<std::result::Result<ServerResponse, SharedString>>>>,
    pending_server_refreshes: HashMap<String, Task<()>>,
    pending_cache_write: Option<Task<()>>,
    pending_cache_snapshot: Option<CachedServersFile>,
    _settings_subscription: Option<Subscription>,
}

impl McpRegistryStore {
    pub fn init_global(
        cx: &mut App,
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
    ) -> Entity<Self> {
        if let Some(store) = Self::try_global(cx) {
            return store;
        }

        let store =
            cx.new(|cx| Self::new(fs, http_client, REGISTRY_API_BASE_URL.to_owned(), true, cx));
        cx.set_global(GlobalMcpRegistryStore(store.clone()));
        store
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalMcpRegistryStore>().0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalMcpRegistryStore>()
            .map(|store| store.0.clone())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn init_test_global(cx: &mut App, servers: Vec<ServerResponse>) -> Entity<Self> {
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.background_executor().clone());
        let store = cx.new(|_cx| Self {
            fs,
            http_client: http_client::FakeHttpClient::with_404_response(),
            registry_api_base_url: REGISTRY_API_BASE_URL.to_owned(),
            servers,
            cached_servers: HashMap::default(),
            installation_source_hints: HashMap::default(),
            pending_server_operations: HashSet::default(),
            installed_server_names: HashSet::default(),
            refreshed_server_names: HashSet::default(),
            server_generations: HashMap::default(),
            query: None,
            next_cursor: None,
            is_fetching: false,
            fetch_error: None,
            list_generation: 0,
            installed_refresh_generation: 0,
            cache_loaded: true,
            cache_load_waiters: Vec::new(),
            pending_list_fetch: None,
            pending_installed_refresh: None,
            pending_installed_refresh_names: HashSet::default(),
            installed_refresh_waiters: HashMap::default(),
            pending_server_refreshes: HashMap::default(),
            pending_cache_write: None,
            pending_cache_snapshot: None,
            _settings_subscription: None,
        });
        cx.set_global(GlobalMcpRegistryStore(store.clone()));
        store
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_servers(&mut self, servers: Vec<ServerResponse>, cx: &mut Context<Self>) {
        self.servers = servers;
        cx.notify();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_cached_servers(&mut self, servers: Vec<ServerResponse>, cx: &mut Context<Self>) {
        self.cached_servers = servers
            .into_iter()
            .filter(|server| !server.name().is_empty())
            .map(|server| (server.name().to_owned(), server))
            .collect();
        cx.notify();
    }

    pub fn servers(&self) -> &[ServerResponse] {
        &self.servers
    }

    pub fn cached_server(&self, name: &str) -> Option<&ServerResponse> {
        self.cached_servers.get(name)
    }

    pub fn begin_server_operation(&mut self, name: &str, cx: &mut Context<Self>) -> bool {
        let inserted = self.pending_server_operations.insert(name.to_owned());
        if inserted {
            cx.notify();
        }
        inserted
    }

    pub fn finish_server_operation(&mut self, name: &str, cx: &mut Context<Self>) {
        if self.pending_server_operations.remove(name) {
            cx.notify();
        }
    }

    pub fn is_server_operation_pending(&self, name: &str) -> bool {
        self.pending_server_operations.contains(name)
    }

    pub fn is_fetching(&self) -> bool {
        self.is_fetching
    }

    pub fn fetch_error(&self) -> Option<SharedString> {
        self.fetch_error.clone()
    }

    pub fn has_more(&self) -> bool {
        self.next_cursor.is_some()
    }

    pub fn search(&mut self, query: Option<String>, cx: &mut Context<Self>) {
        self.list_generation = self.list_generation.wrapping_add(1);
        self.pending_list_fetch.take();
        self.query = normalize_query(query);
        self.servers.clear();
        self.next_cursor = None;
        self.is_fetching = false;
        self.fetch_error = None;
        self.fetch_page(None, cx);
    }

    pub fn load_more(&mut self, cx: &mut Context<Self>) {
        if self.is_fetching {
            return;
        }

        let Some(cursor) = self.next_cursor.clone() else {
            return;
        };
        self.fetch_page(Some(cursor), cx);
    }

    pub fn fetch_server_list_page(
        &self,
        query: Option<String>,
        cursor: Option<String>,
        cx: &App,
    ) -> Task<Result<ServerListResponse>> {
        let http_client = self.http_client.clone();
        let registry_api_base_url = self.registry_api_base_url.clone();
        let executor = cx.background_executor().clone();
        let query = normalize_query(query);
        cx.background_spawn(async move {
            fetch_server_list(
                http_client,
                &registry_api_base_url,
                query.as_deref(),
                cursor.as_deref(),
                REGISTRY_PAGE_LIMIT,
                &executor,
            )
            .await
        })
    }

    pub fn server_details(
        &mut self,
        name: &str,
        cx: &mut Context<Self>,
    ) -> Task<Result<ServerResponse>> {
        if let Some(server) = self.cached_server(name).cloned() {
            self.refresh_server_in_background(name, cx);
            return Task::ready(Ok(server));
        }

        if !self.cache_loaded {
            let (sender, receiver) = oneshot::channel();
            self.cache_load_waiters.push(sender);
            let name = name.to_owned();
            return cx.spawn(async move |this, cx| {
                receiver
                    .await
                    .context("waiting for the MCP Registry cache to load")?;
                let details_task = this.update(cx, |this, cx| this.server_details(&name, cx))?;
                details_task.await
            });
        }

        if self.pending_installed_refresh_names.contains(name) {
            let (sender, receiver) = oneshot::channel();
            self.installed_refresh_waiters
                .entry(name.to_owned())
                .or_default()
                .push(sender);
            let name = name.to_owned();
            return cx.spawn(async move |_this, _cx| {
                receiver
                    .await
                    .with_context(|| format!("waiting for the MCP Registry refresh for `{name}`"))?
                    .map_err(|error| anyhow!(error))
            });
        }

        let generation = self.next_server_generation(name);

        let http_client = self.http_client.clone();
        let registry_api_base_url = self.registry_api_base_url.clone();
        let executor = cx.background_executor().clone();
        let name = name.to_owned();
        cx.spawn(async move |this, cx| {
            let result =
                fetch_server_response(http_client, &registry_api_base_url, &name, true, &executor)
                    .await;
            this.update(cx, |this, cx| {
                if !this.is_current_server_generation(&name, generation) {
                    return this.cached_server(&name).cloned().map_or(result, Ok);
                }

                let server = result?;
                this.installed_server_names = installed_registry_server_names(cx);
                this.refreshed_server_names.insert(name.clone());
                this.cached_servers.insert(name, server.clone());
                this.persist_cached_servers(cx);
                cx.notify();
                Ok(server)
            })?
        })
    }

    pub fn server_installation(
        &mut self,
        name: &str,
        cx: &mut Context<Self>,
    ) -> Task<Result<(ServerResponse, McpRegistryInstallationSource)>> {
        let details_task = self.server_details(name, cx);
        let name = name.to_owned();
        cx.spawn(async move |this, cx| {
            let server = details_task.await?;
            if server.name() != name {
                bail!("MCP Registry returned details for an unexpected server");
            }

            let source =
                this.update(cx, |this, cx| this.select_installation_source(&server, cx))??;
            Ok((server, source))
        })
    }

    pub fn remember_server(&mut self, server: ServerResponse, cx: &mut Context<Self>) {
        self.remember_server_inner(server, None, cx);
    }

    pub fn remember_server_installation(
        &mut self,
        server: ServerResponse,
        source: McpRegistryInstallationSource,
        cx: &mut Context<Self>,
    ) {
        self.remember_server_inner(server, Some(source), cx);
    }

    fn remember_server_inner(
        &mut self,
        server: ServerResponse,
        source: Option<McpRegistryInstallationSource>,
        cx: &mut Context<Self>,
    ) {
        let name = server.name().to_owned();
        if name.is_empty() {
            log::warn!("not caching an MCP Registry server with an empty name");
            return;
        }

        self.installed_server_names = installed_registry_server_names(cx);
        self.next_server_generation(&name);
        self.refreshed_server_names.insert(name.clone());
        self.cached_servers.insert(name.clone(), server.clone());
        if let Some(source) = source {
            self.installation_source_hints.insert(name.clone(), source);
        }
        self.complete_installed_refresh_waiters(&name, Ok(server));
        self.persist_cached_servers(cx);
        cx.notify();
    }

    fn select_installation_source(
        &mut self,
        server: &ServerResponse,
        cx: &mut Context<Self>,
    ) -> Result<McpRegistryInstallationSource> {
        let name = server.name();
        let options = server.installation_options();
        let source = self
            .installation_source_hints
            .get(name)
            .filter(|source| options.iter().any(|option| &option.source == *source))
            .cloned()
            .or_else(|| {
                options
                    .iter()
                    .min_by_key(|option| &option.source)
                    .map(|option| option.source.clone())
            });

        let Some(source) = source else {
            if self.installation_source_hints.remove(name).is_some() {
                self.persist_cached_servers(cx);
            }
            bail!("MCP Registry server `{name}` does not offer a supported installation source");
        };

        if self.installation_source_hints.get(name) != Some(&source) {
            self.installation_source_hints
                .insert(name.to_owned(), source.clone());
            self.persist_cached_servers(cx);
        }
        Ok(source)
    }

    pub fn refresh_installed(&mut self, cx: &mut Context<Self>) {
        let installed_server_names = installed_registry_server_names(cx);
        if !self.cache_loaded {
            self.installed_server_names = installed_server_names;
            return;
        }
        self.refresh_installed_names(installed_server_names, true, cx);
    }

    fn new(
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        registry_api_base_url: String,
        load_cache: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        let installed_server_names = installed_registry_server_names(cx);
        let settings_subscription = cx.observe_global::<SettingsStore>(|this, cx| {
            let installed_server_names = installed_registry_server_names(cx);
            if installed_server_names == this.installed_server_names {
                return;
            }

            if this.cache_loaded {
                this.refresh_installed_names(installed_server_names, false, cx);
            } else {
                this.installed_server_names = installed_server_names;
            }
        });

        let mut store = Self {
            fs: fs.clone(),
            http_client,
            registry_api_base_url,
            servers: Vec::new(),
            cached_servers: HashMap::default(),
            installation_source_hints: HashMap::default(),
            pending_server_operations: HashSet::default(),
            installed_server_names,
            refreshed_server_names: HashSet::default(),
            server_generations: HashMap::default(),
            query: None,
            next_cursor: None,
            is_fetching: false,
            fetch_error: None,
            list_generation: 0,
            installed_refresh_generation: 0,
            cache_loaded: !load_cache,
            cache_load_waiters: Vec::new(),
            pending_list_fetch: None,
            pending_installed_refresh: None,
            pending_installed_refresh_names: HashSet::default(),
            installed_refresh_waiters: HashMap::default(),
            pending_server_refreshes: HashMap::default(),
            pending_cache_write: None,
            pending_cache_snapshot: None,
            _settings_subscription: Some(settings_subscription),
        };

        if load_cache {
            store.load_cached_servers(fs, cx);
        }
        store
    }

    fn fetch_page(&mut self, cursor: Option<String>, cx: &mut Context<Self>) {
        let generation = self.list_generation;
        let query = self.query.clone();
        let http_client = self.http_client.clone();
        let registry_api_base_url = self.registry_api_base_url.clone();
        let executor = cx.background_executor().clone();

        self.is_fetching = true;
        self.fetch_error = None;
        cx.notify();

        self.pending_list_fetch = Some(cx.spawn(async move |this, cx| {
            let result = fetch_server_list(
                http_client,
                &registry_api_base_url,
                query.as_deref(),
                cursor.as_deref(),
                REGISTRY_PAGE_LIMIT,
                &executor,
            )
            .await;

            this.update(cx, |this, cx| {
                if this.list_generation != generation {
                    return;
                }

                this.pending_list_fetch = None;
                this.is_fetching = false;
                match result {
                    Ok(response) => {
                        this.append_servers(response.servers);
                        this.next_cursor = response
                            .metadata
                            .next_cursor
                            .filter(|cursor| !cursor.is_empty());
                        this.fetch_error = None;
                    }
                    Err(error) => {
                        this.fetch_error = Some(SharedString::from(format!("{error:#}")));
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    fn append_servers(&mut self, servers: Vec<ServerResponse>) {
        let mut known_versions: HashSet<(String, String)> = self
            .servers
            .iter()
            .map(|server| (server.name().to_owned(), server.version().to_owned()))
            .collect();

        for server in servers {
            let key = (server.name().to_owned(), server.version().to_owned());
            if known_versions.insert(key) {
                self.servers.push(server);
            }
        }
    }

    fn load_cached_servers(&mut self, fs: Arc<dyn Fs>, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let cache_path = registry_cache_path();
            let cached_file = if fs.is_file(&cache_path).await {
                fs.load_bytes(&cache_path)
                    .await
                    .context("reading cached MCP Registry servers")
                    .and_then(|bytes| {
                        serde_json::from_slice(&bytes)
                            .context("parsing cached MCP Registry servers")
                    })
            } else {
                Ok(CachedServersFile::default())
            };

            this.update(cx, |this, cx| {
                match cached_file {
                    Ok(cached_file) => {
                        let mut published_cache = false;
                        for server in cached_file.servers {
                            let name = server.name().to_owned();
                            if !name.is_empty() && !this.cached_servers.contains_key(&name) {
                                this.cached_servers.insert(name, server);
                                published_cache = true;
                            }
                        }
                        for (name, source) in cached_file.source_hints {
                            if this.cached_servers.contains_key(&name) {
                                this.installation_source_hints.entry(name).or_insert(source);
                            }
                        }
                        if published_cache {
                            cx.notify();
                        }
                    }
                    Err(error) => {
                        log::warn!("failed to load MCP Registry server cache: {error:#}");
                    }
                }

                this.cache_loaded = true;
                let installed_server_names = installed_registry_server_names(cx);
                this.refresh_installed_names(installed_server_names, false, cx);
                for waiter in this.cache_load_waiters.drain(..) {
                    if waiter.send(()).is_err() {
                        log::debug!("MCP Registry cache waiter was canceled");
                    }
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn refresh_installed_names(
        &mut self,
        installed_server_names: HashSet<String>,
        force: bool,
        cx: &mut Context<Self>,
    ) {
        if self.pending_installed_refresh.is_some()
            && installed_server_names == self.installed_server_names
        {
            return;
        }

        let removed_server_names = self
            .installed_server_names
            .difference(&installed_server_names)
            .cloned()
            .collect::<Vec<_>>();
        for name in removed_server_names {
            self.complete_installed_refresh_waiters(
                &name,
                Err(format!("MCP Registry server `{name}` is no longer installed").into()),
            );
        }

        self.installed_refresh_generation = self.installed_refresh_generation.wrapping_add(1);
        let generation = self.installed_refresh_generation;
        self.pending_installed_refresh.take();
        self.pending_installed_refresh_names.clear();
        self.installed_server_names = installed_server_names;

        if self.installed_server_names.is_empty() {
            let waiting_server_names = self
                .installed_refresh_waiters
                .keys()
                .cloned()
                .collect::<Vec<_>>();
            for name in waiting_server_names {
                self.complete_installed_refresh_waiters(
                    &name,
                    Err(format!("MCP Registry server `{name}` is no longer installed").into()),
                );
            }
            self.persist_cached_servers(cx);
            return;
        }

        let http_client = self.http_client.clone();
        let registry_api_base_url = self.registry_api_base_url.clone();
        let executor = cx.background_executor().clone();
        let names = self
            .installed_server_names
            .iter()
            .filter(|name| force || !self.refreshed_server_names.contains(*name))
            .cloned()
            .collect::<Vec<_>>();
        if names.is_empty() {
            self.persist_cached_servers(cx);
            return;
        }
        let names = names
            .into_iter()
            .map(|name| {
                let generation = self.next_server_generation(&name);
                (name, generation)
            })
            .collect::<Vec<_>>();
        self.pending_installed_refresh_names = names
            .iter()
            .map(|(name, _generation)| name.clone())
            .collect();

        self.pending_installed_refresh = Some(cx.spawn(async move |this, cx| {
            let mut responses = stream::iter(names.into_iter().map(|(name, generation)| {
                let http_client = http_client.clone();
                let registry_api_base_url = registry_api_base_url.clone();
                let executor = executor.clone();
                async move {
                    let result = fetch_server_response(
                        http_client,
                        &registry_api_base_url,
                        &name,
                        true,
                        &executor,
                    )
                    .await;
                    (name, generation, result)
                }
            }))
            .buffer_unordered(REGISTRY_INSTALLED_REFRESH_CONCURRENCY);

            while let Some((name, server_generation, result)) = responses.next().await {
                let Some(is_current) = this
                    .update(cx, |this, cx| {
                        if this.installed_refresh_generation != generation {
                            return false;
                        }

                        this.apply_installed_refresh_result(name, server_generation, result, cx);
                        true
                    })
                    .log_err()
                else {
                    return;
                };
                if !is_current {
                    return;
                }
            }

            this.update(cx, |this, cx| {
                if this.installed_refresh_generation != generation {
                    return;
                }
                this.pending_installed_refresh = None;
                this.pending_installed_refresh_names.clear();
                let waiting_server_names = this
                    .installed_refresh_waiters
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>();
                for name in waiting_server_names {
                    let result = this.cached_server(&name).cloned().map_or_else(
                        || {
                            Err(
                                format!("MCP Registry refresh for `{name}` did not complete")
                                    .into(),
                            )
                        },
                        Ok,
                    );
                    this.complete_installed_refresh_waiters(&name, result);
                }
                this.persist_cached_servers(cx);
                cx.notify();
            })
            .log_err();
        }));
    }

    fn apply_installed_refresh_result(
        &mut self,
        name: String,
        server_generation: u64,
        result: Result<ServerResponse>,
        cx: &mut Context<Self>,
    ) {
        self.pending_installed_refresh_names.remove(&name);
        if !self.installed_server_names.contains(&name) {
            self.complete_installed_refresh_waiters(
                &name,
                Err(format!("MCP Registry server `{name}` is no longer installed").into()),
            );
            return;
        }

        if !self.is_current_server_generation(&name, server_generation) {
            let result = self.cached_server(&name).cloned().map_or_else(
                || Err(format!("MCP Registry refresh for `{name}` was superseded").into()),
                Ok,
            );
            self.complete_installed_refresh_waiters(&name, result);
            return;
        }

        self.refreshed_server_names.insert(name.clone());
        match result {
            Ok(server) => {
                self.cached_servers.insert(name.clone(), server.clone());
                self.complete_installed_refresh_waiters(&name, Ok(server));
                cx.notify();
            }
            Err(error) => {
                let error_message = format!("{error:#}");
                log::warn!(
                    "failed to refresh installed MCP Registry server {name}: {error_message}"
                );
                self.complete_installed_refresh_waiters(&name, Err(error_message.into()));
            }
        }
    }

    fn complete_installed_refresh_waiters(
        &mut self,
        name: &str,
        result: std::result::Result<ServerResponse, SharedString>,
    ) {
        let Some(waiters) = self.installed_refresh_waiters.remove(name) else {
            return;
        };
        for waiter in waiters {
            if waiter.send(result.clone()).is_err() {
                log::debug!("MCP Registry refresh waiter for `{name}` was canceled");
            }
        }
    }

    fn refresh_server_in_background(&mut self, name: &str, cx: &mut Context<Self>) {
        if self.refreshed_server_names.contains(name)
            || self.pending_server_refreshes.contains_key(name)
            || (self.installed_server_names.contains(name)
                && self.pending_installed_refresh.is_some())
        {
            return;
        }
        self.refreshed_server_names.insert(name.to_owned());
        let generation = self.next_server_generation(name);

        let http_client = self.http_client.clone();
        let registry_api_base_url = self.registry_api_base_url.clone();
        let executor = cx.background_executor().clone();
        let name = name.to_owned();
        let task_name = name.clone();
        let task = cx.spawn(async move |this, cx| {
            let result = fetch_server_response(
                http_client,
                &registry_api_base_url,
                &task_name,
                true,
                &executor,
            )
            .await;
            this.update(cx, |this, cx| {
                this.pending_server_refreshes.remove(&task_name);
                if !this.is_current_server_generation(&task_name, generation) {
                    return;
                }
                match result {
                    Ok(server) => {
                        this.cached_servers.insert(task_name.clone(), server);
                        this.persist_cached_servers(cx);
                        cx.notify();
                    }
                    Err(error) => {
                        log::warn!(
                            "failed to refresh installed MCP Registry server {task_name}: {error:#}"
                        );
                    }
                }
            })
            .log_err();
        });
        self.pending_server_refreshes.insert(name, task);
    }

    fn next_server_generation(&mut self, name: &str) -> u64 {
        let generation = self.server_generations.entry(name.to_owned()).or_default();
        *generation = generation.wrapping_add(1);
        *generation
    }

    fn is_current_server_generation(&self, name: &str, generation: u64) -> bool {
        self.server_generations.get(name).copied() == Some(generation)
    }

    fn persist_cached_servers(&mut self, cx: &mut Context<Self>) {
        let mut servers = self.cached_servers.values().cloned().collect::<Vec<_>>();
        servers.sort_unstable_by(|left, right| left.name().cmp(right.name()));
        let source_hints = self
            .installation_source_hints
            .iter()
            .filter(|(name, _source)| self.cached_servers.contains_key(*name))
            .map(|(name, source)| (name.clone(), source.clone()))
            .collect();
        let snapshot = CachedServersFile {
            servers,
            source_hints,
        };

        if self.pending_cache_write.is_some() {
            self.pending_cache_snapshot = Some(snapshot);
            return;
        }
        self.start_cache_write(snapshot, cx);
    }

    fn start_cache_write(&mut self, snapshot: CachedServersFile, cx: &mut Context<Self>) {
        let fs = self.fs.clone();

        self.pending_cache_write = Some(cx.spawn(async move |this, cx| {
            let result = write_cached_servers(fs, snapshot).await;
            if let Err(error) = result {
                log::warn!("failed to write MCP Registry server cache: {error:#}");
            }

            this.update(cx, |this, cx| {
                this.pending_cache_write = None;
                if let Some(snapshot) = this.pending_cache_snapshot.take() {
                    this.start_cache_write(snapshot, cx);
                }
            })
            .log_err();
        }));
    }
}

#[derive(Default, Serialize, Deserialize)]
struct CachedServersFile {
    #[serde(default)]
    servers: Vec<ServerResponse>,
    #[serde(default)]
    source_hints: HashMap<String, McpRegistryInstallationSource>,
}

fn installed_registry_server_names(cx: &App) -> HashSet<String> {
    ProjectSettings::get_global(cx)
        .context_servers
        .iter()
        .filter_map(|(name, settings)| {
            matches!(settings, ContextServerSettings::Registry { .. }).then(|| name.to_string())
        })
        .collect()
}

fn registry_cache_path() -> PathBuf {
    paths::data_dir().join("mcp_registry").join("servers.json")
}

async fn write_cached_servers(fs: Arc<dyn Fs>, cache: CachedServersFile) -> Result<()> {
    let cache_path = registry_cache_path();
    let cache_dir = cache_path
        .parent()
        .context("MCP Registry cache path has no parent")?;
    fs.create_dir(cache_dir)
        .await
        .context("creating MCP Registry cache directory")?;
    let json = serde_json::to_string(&cache).context("serializing MCP Registry server cache")?;
    fs.atomic_write(cache_path, json)
        .await
        .context("writing MCP Registry server cache")
}

fn normalize_query(query: Option<String>) -> Option<String> {
    query.and_then(|query| {
        let query = query.trim();
        (!query.is_empty()).then(|| query.to_owned())
    })
}

async fn fetch_server_list(
    http_client: Arc<dyn HttpClient>,
    registry_api_base_url: &str,
    query: Option<&str>,
    cursor: Option<&str>,
    limit: usize,
    executor: &BackgroundExecutor,
) -> Result<ServerListResponse> {
    let url = server_list_url(registry_api_base_url, query, cursor, limit)?;
    let timeout = if query.is_some_and(|query| !query.is_empty()) {
        REGISTRY_SEARCH_FETCH_TIMEOUT
    } else {
        REGISTRY_FETCH_TIMEOUT
    };
    let body = fetch_url_body(http_client, url.as_str(), timeout, executor).await?;
    serde_json::from_slice(&body).context("parsing MCP Registry server list")
}

async fn fetch_server_response(
    http_client: Arc<dyn HttpClient>,
    registry_api_base_url: &str,
    name: &str,
    include_deleted: bool,
    executor: &BackgroundExecutor,
) -> Result<ServerResponse> {
    if name.is_empty() {
        bail!("cannot fetch an MCP Registry server with an empty name");
    }

    let url = server_detail_url(registry_api_base_url, name, include_deleted)?;
    let body = fetch_url_body(http_client, url.as_str(), REGISTRY_FETCH_TIMEOUT, executor).await?;
    let mut response: ServerResponse =
        serde_json::from_slice(&body).context("parsing MCP Registry server detail")?;
    if response.server.name.is_empty() {
        response.server.name = name.to_owned();
    } else if response.server.name != name {
        bail!("MCP Registry returned details for an unexpected server");
    }
    Ok(response)
}

fn server_list_url(
    registry_api_base_url: &str,
    query: Option<&str>,
    cursor: Option<&str>,
    limit: usize,
) -> Result<Url> {
    let mut url = registry_endpoint_url(registry_api_base_url, &["servers"])?;
    {
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.append_pair("version", "latest");
        query_pairs.append_pair("limit", &limit.clamp(1, REGISTRY_PAGE_LIMIT).to_string());
        if let Some(query) = query.filter(|query| !query.is_empty()) {
            query_pairs.append_pair("search", query);
        }
        if let Some(cursor) = cursor.filter(|cursor| !cursor.is_empty()) {
            query_pairs.append_pair("cursor", cursor);
        }
    }
    Ok(url)
}

fn server_detail_url(
    registry_api_base_url: &str,
    name: &str,
    include_deleted: bool,
) -> Result<Url> {
    let mut url = registry_endpoint_url(
        registry_api_base_url,
        &["servers", name, "versions", "latest"],
    )?;
    if include_deleted {
        url.query_pairs_mut().append_pair("include_deleted", "true");
    }
    Ok(url)
}

fn registry_endpoint_url(registry_api_base_url: &str, path_segments: &[&str]) -> Result<Url> {
    let mut url = Url::parse(registry_api_base_url).context("parsing MCP Registry API URL")?;
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| anyhow!("MCP Registry API URL cannot be a base URL"))?;
        segments.pop_if_empty();
        for segment in path_segments {
            segments.push(segment);
        }
    }
    Ok(url)
}

async fn fetch_url_body(
    http_client: Arc<dyn HttpClient>,
    url: &str,
    timeout: Duration,
    executor: &BackgroundExecutor,
) -> Result<Vec<u8>> {
    let (status, body) = async {
        let mut response = http_client
            .get(url, AsyncBody::default(), true)
            .await
            .with_context(|| format!("requesting {url}"))?;
        let status = response.status();
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .with_context(|| format!("reading response from {url}"))?;
        anyhow::Ok((status, body))
    }
    .with_timeout(timeout, executor)
    .await
    .map_err(|_| {
        anyhow!(
            "timed out after {}s while fetching {url}",
            timeout.as_secs()
        )
    })??;

    ensure_success_status(status, &body)?;
    Ok(body)
}

fn ensure_success_status(status: StatusCode, body: &[u8]) -> Result<()> {
    if status.is_success() {
        return Ok(());
    }

    let preview_length = body.len().min(4 * 1024);
    let body_preview = String::from_utf8_lossy(body.get(..preview_length).unwrap_or_default());
    bail!(
        "MCP Registry returned status {}: {body_preview}",
        status.as_u16()
    )
}

#[cfg(test)]
mod tests {
    use std::{
        collections::VecDeque,
        future,
        sync::{
            Arc,
            atomic::{AtomicBool, AtomicUsize, Ordering},
        },
        time::Duration,
    };

    use super::*;
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, Response};
    use parking_lot::Mutex;

    fn test_server(name: &str, version: &str, description: &str) -> ServerResponse {
        serde_json::from_value(serde_json::json!({
            "server": {
                "name": name,
                "version": version,
                "description": description
            }
        }))
        .expect("test server should parse")
    }

    fn test_server_http_response(server: &ServerResponse) -> Result<Response<AsyncBody>> {
        Ok(Response::builder()
            .status(200)
            .body(AsyncBody::from(serde_json::to_vec(server)?))?)
    }

    fn controlled_http_client(
        response_count: usize,
    ) -> (
        Arc<dyn HttpClient>,
        VecDeque<oneshot::Sender<Result<Response<AsyncBody>>>>,
        Arc<AtomicUsize>,
    ) {
        let mut senders = VecDeque::with_capacity(response_count);
        let mut receivers = VecDeque::with_capacity(response_count);
        for _ in 0..response_count {
            let (sender, receiver) = oneshot::channel();
            senders.push_back(sender);
            receivers.push_back(receiver);
        }

        let receivers = Arc::new(Mutex::new(receivers));
        let request_count = Arc::new(AtomicUsize::new(0));
        let http_client = FakeHttpClient::create({
            let request_count = request_count.clone();
            move |_| {
                let receiver = receivers.lock().pop_front();
                request_count.fetch_add(1, Ordering::SeqCst);
                async move {
                    receiver
                        .context("unexpected MCP Registry request")?
                        .await
                        .context("MCP Registry test response was canceled")?
                }
            }
        }) as Arc<dyn HttpClient>;
        (http_client, senders, request_count)
    }

    fn initialize_project_settings(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            ProjectSettings::register(cx);
        });
    }

    fn set_installed_registry_servers(cx: &mut TestAppContext, server_names: &[&str]) {
        cx.update(|cx| {
            let mut project_settings = ProjectSettings::get_global(cx).clone();
            for server_name in server_names {
                project_settings.context_servers.insert(
                    (*server_name).into(),
                    ContextServerSettings::Registry {
                        enabled: true,
                        remote: false,
                        registry: settings::McpRegistryServerSettings {
                            credential_id: None,
                            inputs: HashMap::default(),
                        },
                    },
                );
            }
            ProjectSettings::override_global(project_settings, cx);
        });
    }

    #[gpui::test]
    async fn search_requests_use_an_extended_timeout(cx: &mut TestAppContext) {
        let http_client =
            FakeHttpClient::create(|_| future::pending::<Result<Response<AsyncBody>>>())
                as Arc<dyn HttpClient>;
        let executor = cx.executor();

        let list_finished = Arc::new(AtomicBool::new(false));
        let list_task = executor.spawn({
            let http_client = http_client.clone();
            let request_executor = executor.clone();
            let list_finished = list_finished.clone();
            async move {
                let result = fetch_server_list(
                    http_client,
                    "https://registry.example.test/v0.1",
                    None,
                    None,
                    REGISTRY_PAGE_LIMIT,
                    &request_executor,
                )
                .await;
                list_finished.store(true, Ordering::SeqCst);
                result
            }
        });
        let detail_finished = Arc::new(AtomicBool::new(false));
        let detail_task = executor.spawn({
            let http_client = http_client.clone();
            let request_executor = executor.clone();
            let detail_finished = detail_finished.clone();
            async move {
                let result = fetch_server_response(
                    http_client,
                    "https://registry.example.test/v0.1",
                    "io.example/server",
                    false,
                    &request_executor,
                )
                .await;
                detail_finished.store(true, Ordering::SeqCst);
                result
            }
        });
        let search_finished = Arc::new(AtomicBool::new(false));
        let search_task = executor.spawn({
            let request_executor = executor.clone();
            let search_finished = search_finished.clone();
            async move {
                let result = fetch_server_list(
                    http_client,
                    "https://registry.example.test/v0.1",
                    Some("filesystem"),
                    None,
                    REGISTRY_PAGE_LIMIT,
                    &request_executor,
                )
                .await;
                search_finished.store(true, Ordering::SeqCst);
                result
            }
        });

        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(31));
        cx.run_until_parked();

        assert!(list_finished.load(Ordering::SeqCst));
        assert!(detail_finished.load(Ordering::SeqCst));
        assert!(!search_finished.load(Ordering::SeqCst));
        assert!(
            list_task
                .await
                .expect_err("unfiltered list request should time out")
                .to_string()
                .contains("timed out after 30s")
        );
        assert!(
            detail_task
                .await
                .expect_err("detail request should time out")
                .to_string()
                .contains("timed out after 30s")
        );

        cx.executor().advance_clock(Duration::from_secs(30));
        cx.run_until_parked();
        assert!(search_finished.load(Ordering::SeqCst));
        assert!(
            search_task
                .await
                .expect_err("search request should time out")
                .to_string()
                .contains("timed out after 60s")
        );
    }

    #[gpui::test]
    fn finds_registry_credentials_across_settings_layers(cx: &mut App) {
        fn registry_settings(credential_id: &str) -> Value {
            serde_json::json!({
                "registry": {
                    "credential_id": credential_id
                }
            })
        }

        let user_settings = serde_json::json!({
            "context_servers": {
                "base": registry_settings("base-credential")
            },
            "profiles": {
                "Profile": {
                    "settings": {
                        "context_servers": {
                            "profile": registry_settings("profile-credential")
                        }
                    }
                }
            },
            "macos": {
                "context_servers": {
                    "platform": registry_settings("platform-credential")
                }
            },
            "stable": {
                "context_servers": {
                    "release": registry_settings("release-credential")
                }
            }
        });
        let server_settings = serde_json::json!({
            "context_servers": {
                "server": registry_settings("server-credential")
            }
        });
        let global_settings = serde_json::json!({
            "context_servers": {
                "global": registry_settings("global-credential")
            }
        });
        let project_settings = serde_json::json!({
            "context_servers": {
                "project": registry_settings("project-credential")
            }
        });

        let mut settings_store = SettingsStore::test(cx);
        settings_store
            .set_user_settings(&user_settings.to_string(), cx)
            .expect("user settings should parse");
        settings_store
            .set_server_settings(&server_settings.to_string(), cx)
            .expect("server settings should parse");
        settings_store
            .set_global_settings(&global_settings.to_string(), cx)
            .expect("global settings should parse");
        settings_store
            .set_local_settings(
                settings::WorktreeId::from_usize(1),
                settings::LocalSettingsPath::InWorktree(util::rel_path::rel_path("project").into()),
                settings::LocalSettingsKind::Settings,
                Some(&project_settings.to_string()),
                cx,
            )
            .expect("project settings should parse");
        cx.set_global(settings_store);

        for credential_id in [
            "base-credential",
            "profile-credential",
            "platform-credential",
            "release-credential",
            "server-credential",
            "global-credential",
            "project-credential",
        ] {
            assert!(registry_credential_is_referenced(credential_id, cx));
        }
        assert!(!registry_credential_is_referenced(
            "unreferenced-credential",
            cx
        ));
    }

    #[test]
    fn parses_live_registry_shapes_and_preserves_unknown_types() {
        let response: ServerListResponse = serde_json::from_value(serde_json::json!({
            "servers": [{
                "server": {
                    "$schema": "https://static.modelcontextprotocol.io/schemas/2025-10-17/server.schema.json",
                    "name": "io.example/server",
                    "description": "An example server",
                    "title": "Example",
                    "repository": {
                        "url": "https://github.com/example/server",
                        "source": "github"
                    },
                    "version": "1.2.3",
                    "websiteUrl": "https://example.com",
                    "icons": [{"src": "https://example.com/icon.png", "mimeType": "image/png"}],
                    "packages": [{
                        "registryType": "future-registry",
                        "identifier": "example-server",
                        "transport": {"type": "stdio"},
                        "runtimeArguments": [{
                            "type": "named",
                            "name": "--port",
                            "value": "{port}",
                            "variables": {
                                "port": {"format": "number", "default": "3000"}
                            }
                        }],
                        "packageArguments": [{"type": "future-argument", "payload": 1}],
                        "environmentVariables": [{
                            "name": "TOKEN",
                            "isRequired": true,
                            "isSecret": true
                        }]
                    }],
                    "remotes": [
                        {"type": "streamable-http", "url": "https://example.com/mcp"},
                        {"type": "future-transport", "endpoint": "wss://example.com"}
                    ]
                },
                "_meta": {
                    "io.modelcontextprotocol.registry/official": {
                        "status": "active",
                        "publishedAt": "2026-01-01T00:00:00Z",
                        "isLatest": true
                    }
                }
            }],
            "metadata": {"nextCursor": "opaque:value", "count": 1}
        }))
        .expect("live-like registry response should parse");

        let server = &response.servers[0];
        assert_eq!(server.name(), "io.example/server");
        assert_eq!(server.title(), Some("Example"));
        assert_eq!(
            response.metadata.next_cursor.as_deref(),
            Some("opaque:value")
        );
        assert!(matches!(
            server.server.packages[0].transport,
            Transport::Stdio(_)
        ));
        assert!(matches!(
            server.server.packages[0].package_arguments[0],
            Argument::Unknown(_)
        ));
        assert!(matches!(server.server.remotes[1], Transport::Unknown(_)));
    }

    #[test]
    fn parses_incomplete_older_records() {
        let response: ServerListResponse = serde_json::from_value(serde_json::json!({
            "servers": [{"server": {"name": "io.example/old"}}]
        }))
        .expect("incomplete registry records should parse");

        let server = &response.servers[0];
        assert_eq!(server.name(), "io.example/old");
        assert_eq!(server.description(), "");
        assert_eq!(server.version(), "");
        assert!(server.server.packages.is_empty());
        assert!(!server.metadata.official.is_latest.unwrap_or(false));
    }

    #[test]
    fn builds_list_and_encoded_detail_urls() {
        let list_url = server_list_url(
            REGISTRY_API_BASE_URL,
            Some("file system"),
            Some("opaque:value/next"),
            1_000,
        )
        .expect("list URL should build");
        let query_pairs = list_url.query_pairs().collect::<HashMap<_, _>>();
        assert_eq!(
            query_pairs.get("version").map(|value| value.as_ref()),
            Some("latest")
        );
        assert_eq!(
            query_pairs.get("limit").map(|value| value.as_ref()),
            Some("100")
        );
        assert_eq!(
            query_pairs.get("search").map(|value| value.as_ref()),
            Some("file system")
        );
        assert_eq!(
            query_pairs.get("cursor").map(|value| value.as_ref()),
            Some("opaque:value/next")
        );

        let detail_url = server_detail_url(REGISTRY_API_BASE_URL, "io.example/server name", false)
            .expect("detail URL should build");
        assert_eq!(
            detail_url.as_str(),
            "https://registry.modelcontextprotocol.io/v0.1/servers/io.example%2Fserver%20name/versions/latest"
        );

        let installed_detail_url =
            server_detail_url(REGISTRY_API_BASE_URL, "io.example/server name", true)
                .expect("installed detail URL should build");
        assert_eq!(
            installed_detail_url.as_str(),
            "https://registry.modelcontextprotocol.io/v0.1/servers/io.example%2Fserver%20name/versions/latest?include_deleted=true"
        );
    }

    #[test]
    fn validates_credential_ids_and_exact_package_versions() {
        assert_eq!(
            registry_inputs_credential_key("0f8fad5b-d9cb-469f-a165-70867728950e")
                .expect("UUID credential ID should be valid"),
            "mcp-registry-inputs:v1:0f8fad5b-d9cb-469f-a165-70867728950e"
        );
        for credential_id in ["", "contains_underscore", "contains.dot", "contains:colon"] {
            assert!(registry_inputs_credential_key(credential_id).is_err());
        }
        assert!(registry_inputs_credential_key(&"a".repeat(129)).is_err());

        for version in ["1.2.3", "1.2.3-beta.1+build.7"] {
            assert!(is_exact_package_version(version));
        }
        for version in [
            "",
            "latest",
            "next",
            "beta",
            "release-2026.07",
            " 1.2.3",
            "1.2.3 ",
            "1.2 3",
            "^1.2.3",
            "1.x",
            "a || b",
        ] {
            assert!(!is_exact_package_version(version));
        }
    }

    #[test]
    fn installation_options_only_include_supported_sources() {
        let server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": "io.example/server",
                "packages": [
                    {
                        "registryType": "npm",
                        "registryBaseUrl": "https://registry.npmjs.org/",
                        "identifier": "@example/server",
                        "version": "1.2.3",
                        "runtimeHint": "npx",
                        "transport": {"type": "stdio"},
                        "runtimeArguments": [{
                            "type": "named",
                            "name": "--mount",
                            "value": "{source}:{target}",
                            "isRepeated": true,
                            "variables": {
                                "target": {"description": "Target path"},
                                "source": {"format": "filepath"}
                            }
                        }]
                    },
                    {
                        "registryType": "npm",
                        "identifier": "@example/secondary",
                        "version": "1.2.3",
                        "transport": {"type": "stdio"}
                    },
                    {
                        "registryType": "npm",
                        "registryBaseUrl": "https://npm.example.com",
                        "identifier": "custom-registry",
                        "version": "1.0.0",
                        "transport": {"type": "stdio"}
                    },
                    {
                        "registryType": "npm",
                        "identifier": "bun-only",
                        "version": "1.0.0",
                        "runtimeHint": "bunx",
                        "transport": {"type": "stdio"}
                    },
                    {
                        "registryType": "npm",
                        "identifier": "missing-version",
                        "transport": {"type": "stdio"}
                    }
                ],
                "remotes": [
                    {"type": "streamable-http", "url": "https://example.com/mcp"},
                    {"type": "sse", "url": "https://example.com/sse"}
                ]
            }
        }))
        .expect("server should parse");

        let options = server.installation_options();
        assert_eq!(options.len(), 3);
        assert!(matches!(
            &options[0].source,
            McpRegistryInstallationSource::Package { identifier, .. }
                if identifier == "@example/server"
        ));
        let argument_id = argument_input_id(
            "runtime_argument",
            &server.server.packages[0].runtime_arguments[0],
        );
        assert_eq!(
            options[0]
                .inputs
                .iter()
                .map(|input| (input.id.clone(), input.repeated))
                .collect::<Vec<_>>(),
            vec![
                (format!("{argument_id}.variable:source"), true),
                (format!("{argument_id}.variable:target"), true),
            ]
        );
        assert!(matches!(
            &options[2].source,
            McpRegistryInstallationSource::Remote { url }
                if url == "https://example.com/mcp"
        ));
    }

    #[gpui::test]
    async fn server_installation_replaces_a_stale_source_hint(cx: &mut TestAppContext) {
        initialize_project_settings(cx);
        let server_name = "io.example/source-change";
        let original_server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": server_name,
                "version": "1.0.0",
                "packages": [{
                    "registryType": "npm",
                    "identifier": "@example/source-change",
                    "version": "1.0.0",
                    "transport": {"type": "stdio"}
                }],
                "remotes": [{
                    "type": "streamable-http",
                    "url": "https://example.com/old-mcp"
                }]
            }
        }))
        .expect("original server should parse");
        let selected_remote = McpRegistryInstallationSource::Remote {
            url: "https://example.com/old-mcp".to_owned(),
        };
        let registry_store = cx.update(|cx| McpRegistryStore::init_test_global(cx, Vec::new()));
        registry_store.update(cx, |store, cx| {
            store.remember_server_installation(original_server, selected_remote, cx);
        });

        let updated_server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": server_name,
                "version": "2.0.0",
                "packages": [{
                    "registryType": "npm",
                    "identifier": "@example/replacement",
                    "version": "2.0.0",
                    "transport": {"type": "stdio"}
                }]
            }
        }))
        .expect("updated server should parse");
        registry_store.update(cx, |store, cx| store.remember_server(updated_server, cx));

        let (server, source) = registry_store
            .update(cx, |store, cx| store.server_installation(server_name, cx))
            .await
            .expect("replacement installation should resolve");
        assert_eq!(server.version(), "2.0.0");
        assert_eq!(
            source,
            McpRegistryInstallationSource::Package {
                registry_type: "npm".to_owned(),
                identifier: "@example/replacement".to_owned(),
            }
        );
        registry_store.read_with(cx, |store, _cx| {
            assert_eq!(
                store.installation_source_hints.get(server_name),
                Some(&source)
            );
        });
    }

    #[test]
    fn rejects_unverified_integrity_and_npm_policy_overrides() {
        let server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": "io.example/policy",
                "packages": [
                    {
                        "registryType": "npm",
                        "identifier": "with-integrity",
                        "version": "1.0.0",
                        "fileSha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "transport": {"type": "stdio"}
                    },
                    {
                        "registryType": "npm",
                        "identifier": "runtime-policy",
                        "version": "1.0.0",
                        "transport": {"type": "stdio"},
                        "runtimeArguments": [{"type": "named", "name": "--registry"}]
                    },
                    {
                        "registryType": "npm",
                        "identifier": "environment-policy",
                        "version": "1.0.0",
                        "transport": {"type": "stdio"},
                        "environmentVariables": [{"name": "npm_config_min_release_age"}]
                    },
                    {
                        "registryType": "npm",
                        "identifier": "dynamic-policy",
                        "version": "1.0.0",
                        "transport": {"type": "stdio"},
                        "runtimeArguments": [{
                            "type": "positional",
                            "valueHint": "runtime option",
                            "isRequired": true
                        }]
                    }
                ]
            }
        }))
        .expect("server should parse");

        assert!(!server.server.packages[0].is_supported_installation());
        assert!(!server.server.packages[1].is_supported_installation());
        assert!(!server.server.packages[2].is_supported_installation());
        assert!(server.server.packages[3].is_supported_installation());

        let dynamic_package = &server.server.packages[3];
        let argument_id =
            argument_input_id("runtime_argument", &dynamic_package.runtime_arguments[0]);
        let source = McpRegistryInstallationSource::Package {
            registry_type: "npm".to_owned(),
            identifier: dynamic_package.identifier.clone(),
        };
        let error = resolve_server_configuration(
            &server,
            &source,
            &HashMap::from_iter([(
                argument_id,
                vec!["--registry=https://example.com".to_owned()],
            )]),
            &HashMap::default(),
        )
        .expect_err("resolved npm policy overrides should be rejected");
        assert!(format!("{error:#}").contains("overrides npm policy"));
    }

    #[test]
    fn rejects_abbreviated_and_negated_npm_policy_arguments() {
        for argument in [
            "--reg=https://example.com",
            "--userc=/tmp/npmrc",
            "--globalc=/tmp/npmrc",
            "--before=2026-01-01",
            "--min=0",
            "--no-before",
            "--no-min-release-age",
            "--@scope:reg=https://example.com",
        ] {
            assert!(
                is_npm_policy_argument(argument),
                "expected `{argument}` to be treated as an npm policy override"
            );
        }

        assert!(!is_npm_policy_argument("--loglevel=warn"));
        assert!(!is_npm_policy_argument("--package-lock=false"));
    }

    #[test]
    fn rejects_npm_bootstrap_environment_overrides() {
        for name in [
            "HOME",
            "userprofile",
            "PATH",
            "npm_config_registry",
            "NODE_OPTIONS",
            "NODE_EXTRA_CA_CERTS",
            "NPM_TOKEN",
            "NODE_AUTH_TOKEN",
            "HTTPS_PROXY",
            "NO_PROXY",
            "SSL_CERT_FILE",
        ] {
            assert!(
                is_npm_bootstrap_environment_variable(name),
                "expected `{name}` to affect npm bootstrap behavior"
            );
        }

        assert!(!is_npm_bootstrap_environment_variable("SERVER_API_KEY"));
    }

    #[test]
    fn rejects_ambiguous_installation_sources_and_deleted_servers() {
        let server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": "io.example/ambiguous",
                "packages": [
                    {
                        "registryType": "npm",
                        "identifier": "@example/server",
                        "version": "1.0.0",
                        "transport": {"type": "stdio"}
                    },
                    {
                        "registryType": "npm",
                        "identifier": "@example/server",
                        "version": "2.0.0",
                        "transport": {"type": "stdio"}
                    }
                ],
                "remotes": [
                    {"type": "streamable-http", "url": "https://example.com/mcp"},
                    {"type": "streamable-http", "url": "https://example.com/mcp"}
                ]
            }
        }))
        .expect("server should parse");
        assert!(server.installation_options().is_empty());

        let package_source = McpRegistryInstallationSource::Package {
            registry_type: "npm".to_owned(),
            identifier: "@example/server".to_owned(),
        };
        let error = resolve_server_configuration(
            &server,
            &package_source,
            &HashMap::default(),
            &HashMap::default(),
        )
        .expect_err("duplicate package sources should be ambiguous");
        assert!(format!("{error:#}").contains("multiple matching packages"));

        let remote_source = McpRegistryInstallationSource::Remote {
            url: "https://example.com/mcp".to_owned(),
        };
        let error = resolve_server_configuration(
            &server,
            &remote_source,
            &HashMap::default(),
            &HashMap::default(),
        )
        .expect_err("duplicate remote sources should be ambiguous");
        assert!(format!("{error:#}").contains("multiple matching remote endpoints"));

        let mut deleted_server = server;
        deleted_server.metadata.official.status = Some("deleted".to_owned());
        assert!(deleted_server.installation_options().is_empty());
        let error = resolve_server_configuration(
            &deleted_server,
            &package_source,
            &HashMap::default(),
            &HashMap::default(),
        )
        .expect_err("deleted servers should not resolve");
        assert!(format!("{error:#}").contains("was removed"));
    }

    #[test]
    fn argument_input_ids_are_stable_and_template_flags_are_inherited() {
        let templated_argument: Argument = serde_json::from_value(serde_json::json!({
            "type": "named",
            "name": "--token",
            "value": "Bearer {token}",
            "isRequired": true,
            "isSecret": true,
            "variables": {
                "unused": {
                    "isRequired": true,
                    "format": "unsupported"
                },
                "token": {
                    "description": "Access token"
                }
            }
        }))
        .expect("templated argument should parse");
        let positional_argument: Argument = serde_json::from_value(serde_json::json!({
            "type": "positional",
            "valueHint": "workspace",
            "format": "filepath"
        }))
        .expect("positional argument should parse");

        let mut original_descriptors = Vec::new();
        append_argument_input_descriptors(
            &mut original_descriptors,
            "runtime_argument",
            &[templated_argument.clone(), positional_argument.clone()],
        );
        let mut reordered_descriptors = Vec::new();
        append_argument_input_descriptors(
            &mut reordered_descriptors,
            "runtime_argument",
            &[positional_argument, templated_argument.clone()],
        );

        let descriptor_ids = |descriptors: Vec<McpRegistryInputDescriptor>| {
            descriptors
                .into_iter()
                .map(|descriptor| (descriptor.label, descriptor.id))
                .collect::<HashMap<_, _>>()
        };
        assert_eq!(
            descriptor_ids(original_descriptors.clone()),
            descriptor_ids(reordered_descriptors)
        );
        assert_eq!(
            original_descriptors
                .iter()
                .map(|descriptor| descriptor.label.as_str())
                .collect::<Vec<_>>(),
            ["token", "workspace"]
        );
        let token_descriptor = &original_descriptors[0];
        assert!(token_descriptor.required);
        assert!(token_descriptor.secret);
        assert!(
            !original_descriptors
                .iter()
                .any(|descriptor| descriptor.label == "unused")
        );

        let resolved = resolve_arguments(
            "runtime_argument",
            std::slice::from_ref(&templated_argument),
            &HashMap::from_iter([(token_descriptor.id.clone(), vec!["plaintext".to_owned()])]),
            &HashMap::from_iter([(token_descriptor.id.clone(), vec!["keychain".to_owned()])]),
        )
        .expect("the inherited secret input should resolve from secret storage");
        assert_eq!(resolved, ["--token", "Bearer keychain"]);
    }

    #[test]
    fn resolves_npm_inputs_without_reading_secrets_from_settings() {
        let server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": "io.example/server",
                "packages": [{
                    "registryType": "npm",
                    "identifier": "@example/server",
                    "version": "1.2.3",
                    "transport": {"type": "stdio"},
                    "runtimeArguments": [
                        {"type": "named", "name": "--quiet"},
                        {
                            "type": "named",
                            "name": "--mount",
                            "value": "{source}:{target}",
                            "isRepeated": true,
                            "variables": {
                                "source": {"format": "filepath", "isRequired": true},
                                "target": {"format": "filepath", "default": "/app"}
                            }
                        }
                    ],
                    "packageArguments": [{
                        "type": "positional",
                        "valueHint": "root",
                        "isRequired": true,
                        "isRepeated": true
                    }],
                    "environmentVariables": [
                        {"name": "LOG_LEVEL", "default": "info"},
                        {"name": "TOKEN", "isRequired": true, "isSecret": true}
                    ]
                }]
            }
        }))
        .expect("server should parse");
        let source = McpRegistryInstallationSource::Package {
            registry_type: "npm".to_owned(),
            identifier: "@example/server".to_owned(),
        };
        let package = &server.server.packages[0];
        let mount_argument_id =
            argument_input_id("runtime_argument", &package.runtime_arguments[1]);
        let root_argument_id = argument_input_id("package_argument", &package.package_arguments[0]);
        let settings_inputs = HashMap::from_iter([
            (
                format!("{mount_argument_id}.variable:source"),
                vec!["/one".to_owned(), "/two".to_owned()],
            ),
            (
                root_argument_id,
                vec!["first".to_owned(), "second".to_owned()],
            ),
            ("environment:TOKEN".to_owned(), vec!["plaintext".to_owned()]),
        ]);
        let secret_inputs =
            HashMap::from_iter([("environment:TOKEN".to_owned(), vec!["keychain".to_owned()])]);

        let resolved =
            resolve_server_configuration(&server, &source, &settings_inputs, &secret_inputs)
                .expect("configuration should resolve");
        let ResolvedMcpRegistryServer::Npm {
            package_spec,
            runtime_arguments,
            package_arguments,
            environment,
        } = resolved
        else {
            panic!("expected npm configuration");
        };
        assert_eq!(package_spec, "@example/server@0.0.0 - 1.2.3");
        assert_eq!(
            runtime_arguments,
            ["--quiet", "--mount", "/one:/app", "--mount", "/two:/app"]
        );
        assert_eq!(package_arguments, ["first", "second"]);
        assert_eq!(
            environment.get("LOG_LEVEL").map(String::as_str),
            Some("info")
        );
        assert_eq!(
            environment.get("TOKEN").map(String::as_str),
            Some("keychain")
        );

        let error =
            resolve_server_configuration(&server, &source, &settings_inputs, &HashMap::default())
                .expect_err("plaintext settings must not satisfy a secret input");
        let error = format!("{error:#}");
        assert!(error.contains("environment:TOKEN"));
        assert!(!error.contains("plaintext"));
    }

    #[test]
    fn resolves_remote_url_and_secret_header_templates() {
        let server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": "io.example/remote",
                "remotes": [{
                    "type": "streamable-http",
                    "url": "https://example.com/{tenant}/mcp",
                    "variables": {
                        "tenant": {"isRequired": true}
                    },
                    "headers": [{
                        "name": "Authorization",
                        "value": "Bearer {token}",
                        "isRequired": true,
                        "variables": {
                            "token": {"isRequired": true, "isSecret": true}
                        }
                    }]
                }]
            }
        }))
        .expect("server should parse");
        let source = McpRegistryInstallationSource::Remote {
            url: "https://example.com/{tenant}/mcp".to_owned(),
        };
        let settings_inputs = HashMap::from_iter([
            ("variable:tenant".to_owned(), vec!["team".to_owned()]),
            (
                "header:Authorization.variable:token".to_owned(),
                vec!["plaintext".to_owned()],
            ),
        ]);
        let secret_inputs = HashMap::from_iter([(
            "header:Authorization.variable:token".to_owned(),
            vec!["keychain".to_owned()],
        )]);

        let resolved =
            resolve_server_configuration(&server, &source, &settings_inputs, &secret_inputs)
                .expect("remote configuration should resolve");
        let ResolvedMcpRegistryServer::Http { url, headers } = resolved else {
            panic!("expected HTTP configuration");
        };
        assert_eq!(url.as_str(), "https://example.com/team/mcp");
        assert_eq!(
            headers.get("Authorization").map(String::as_str),
            Some("Bearer keychain")
        );
    }

    #[gpui::test]
    async fn server_details_joins_installed_refresh(cx: &mut TestAppContext) {
        let server_name = "io.example/server";
        initialize_project_settings(cx);
        cx.update(|cx| {
            let mut project_settings = ProjectSettings::get_global(cx).clone();
            project_settings.context_servers.insert(
                server_name.into(),
                ContextServerSettings::Registry {
                    enabled: true,
                    remote: false,
                    registry: settings::McpRegistryServerSettings {
                        credential_id: None,
                        inputs: HashMap::default(),
                    },
                },
            );
            ProjectSettings::override_global(project_settings, cx);
        });

        let (http_client, mut response_senders, request_count) = controlled_http_client(1);
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client,
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });
        cx.executor().allow_parking();

        registry_store.update(cx, |store, cx| store.refresh_installed(cx));
        cx.run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);

        let details_task =
            registry_store.update(cx, |store, cx| store.server_details(server_name, cx));
        cx.run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);

        let refreshed_server = test_server(server_name, "2.0.0", "Installed refresh response");
        let response_sender = response_senders
            .pop_front()
            .expect("installed refresh response sender should exist");
        assert!(
            response_sender
                .send(test_server_http_response(&refreshed_server))
                .is_ok()
        );
        let details = details_task
            .await
            .expect("joined detail request should succeed");
        assert_eq!(details.version(), "2.0.0");
        cx.run_until_parked();

        registry_store.read_with(cx, |store, _cx| {
            assert_eq!(
                store
                    .cached_server(server_name)
                    .map(ServerResponse::version),
                Some("2.0.0")
            );
        });
    }

    #[gpui::test]
    async fn server_details_only_joins_refresh_for_the_same_name(cx: &mut TestAppContext) {
        let active_server_name = "io.example/active";
        let excluded_server_name = "io.example/excluded";
        initialize_project_settings(cx);
        set_installed_registry_servers(cx, &[active_server_name, excluded_server_name]);
        let (http_client, response_senders, request_count) = controlled_http_client(2);
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client,
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });
        cx.executor().allow_parking();

        registry_store.update(cx, |store, cx| {
            store
                .refreshed_server_names
                .insert(excluded_server_name.to_owned());
            store.refresh_installed_names(installed_registry_server_names(cx), false, cx);
        });
        let details_task = registry_store.update(cx, |store, cx| {
            store.server_details(excluded_server_name, cx)
        });
        cx.run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 2);

        for response_sender in response_senders {
            assert!(
                response_sender
                    .send(test_server_http_response(&test_server(
                        "",
                        "1.0.0",
                        "Refreshed server",
                    )))
                    .is_ok()
            );
        }
        assert_eq!(
            details_task
                .await
                .expect("independent detail request should succeed")
                .name(),
            excluded_server_name
        );
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn repeated_installed_refresh_is_idempotent_while_in_flight(cx: &mut TestAppContext) {
        let server_name = "io.example/server";
        initialize_project_settings(cx);
        set_installed_registry_servers(cx, &[server_name]);
        let (http_client, mut response_senders, request_count) = controlled_http_client(1);
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client,
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });
        cx.executor().allow_parking();

        registry_store.update(cx, |store, cx| store.refresh_installed(cx));
        registry_store.update(cx, |store, cx| store.refresh_installed(cx));
        cx.run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);

        let response_sender = response_senders
            .pop_front()
            .expect("installed refresh response sender should exist");
        assert!(
            response_sender
                .send(test_server_http_response(&test_server(
                    server_name,
                    "1.0.0",
                    "Installed server",
                )))
                .is_ok()
        );
        cx.run_until_parked();
        registry_store.read_with(cx, |store, _cx| {
            assert!(store.pending_installed_refresh.is_none());
        });
    }

    #[gpui::test]
    async fn installed_refresh_limits_concurrent_requests(cx: &mut TestAppContext) {
        let server_names = ["io.example/one", "io.example/two", "io.example/three"];
        initialize_project_settings(cx);
        set_installed_registry_servers(cx, &server_names);
        let (http_client, mut response_senders, request_count) = controlled_http_client(3);
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client,
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });
        cx.executor().allow_parking();

        registry_store.update(cx, |store, cx| store.refresh_installed(cx));
        cx.run_until_parked();
        assert_eq!(
            request_count.load(Ordering::SeqCst),
            REGISTRY_INSTALLED_REFRESH_CONCURRENCY
        );

        let response = test_server_http_response(&test_server("", "1.0.0", "Installed server"));
        let first_response_sender = response_senders
            .pop_front()
            .expect("first installed refresh response sender should exist");
        assert!(first_response_sender.send(response).is_ok());
        cx.run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), server_names.len());

        for response_sender in response_senders {
            assert!(
                response_sender
                    .send(test_server_http_response(&test_server(
                        "",
                        "1.0.0",
                        "Installed server",
                    )))
                    .is_ok()
            );
        }
        cx.run_until_parked();
        registry_store.read_with(cx, |store, _cx| {
            assert!(store.pending_installed_refresh.is_none());
        });
    }

    #[gpui::test]
    fn settings_observer_does_not_refetch_remembered_server(cx: &mut TestAppContext) {
        let server_name = "io.example/server";
        initialize_project_settings(cx);
        let (http_client, _response_senders, request_count) = controlled_http_client(0);
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client,
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });

        registry_store.update(cx, |store, cx| {
            store.remember_server(test_server(server_name, "1.0.0", "Remembered server"), cx);
        });
        set_installed_registry_servers(cx, &[server_name]);
        cx.run_until_parked();

        assert_eq!(request_count.load(Ordering::SeqCst), 0);
        registry_store.read_with(cx, |store, _cx| {
            assert!(store.installed_server_names.contains(server_name));
            assert_eq!(
                store
                    .cached_server(server_name)
                    .map(ServerResponse::version),
                Some("1.0.0")
            );
        });
    }

    #[gpui::test]
    async fn remembered_server_wins_over_cache_miss(cx: &mut TestAppContext) {
        let server_name = "io.example/server";
        initialize_project_settings(cx);
        let (http_client, mut response_senders, request_count) = controlled_http_client(1);
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client,
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });
        cx.executor().allow_parking();

        let details_task =
            registry_store.update(cx, |store, cx| store.server_details(server_name, cx));
        cx.run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);

        registry_store.update(cx, |store, cx| {
            store.remember_server(test_server(server_name, "2.0.0", "Remembered response"), cx);
        });

        let stale_server = test_server(server_name, "1.0.0", "Stale detail response");
        let response_sender = response_senders
            .pop_front()
            .expect("detail response sender should exist");
        assert!(
            response_sender
                .send(test_server_http_response(&stale_server))
                .is_ok()
        );
        let details = details_task
            .await
            .expect("the current cached details should be returned");
        assert_eq!(details.version(), "2.0.0");

        registry_store.read_with(cx, |store, _cx| {
            assert_eq!(
                store
                    .cached_server(server_name)
                    .map(ServerResponse::version),
                Some("2.0.0")
            );
        });
    }

    #[gpui::test]
    async fn remembered_server_wins_over_background_refresh(cx: &mut TestAppContext) {
        let server_name = "io.example/server";
        initialize_project_settings(cx);
        let (http_client, mut response_senders, request_count) = controlled_http_client(1);
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client,
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });
        registry_store.update(cx, |store, cx| {
            store.set_cached_servers(
                vec![test_server(server_name, "1.0.0", "Cached response")],
                cx,
            );
        });
        cx.executor().allow_parking();

        let cached_details = registry_store
            .update(cx, |store, cx| store.server_details(server_name, cx))
            .await
            .expect("cached details should be returned");
        assert_eq!(cached_details.version(), "1.0.0");
        cx.run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);

        registry_store.update(cx, |store, cx| {
            store.remember_server(test_server(server_name, "3.0.0", "Remembered response"), cx);
        });

        let stale_server = test_server(server_name, "2.0.0", "Stale background response");
        let response_sender = response_senders
            .pop_front()
            .expect("background response sender should exist");
        assert!(
            response_sender
                .send(test_server_http_response(&stale_server))
                .is_ok()
        );
        cx.run_until_parked();

        registry_store.read_with(cx, |store, _cx| {
            assert_eq!(
                store
                    .cached_server(server_name)
                    .map(ServerResponse::version),
                Some("3.0.0")
            );
        });
    }

    #[gpui::test]
    async fn installation_source_hint_round_trips_through_the_cache(cx: &mut TestAppContext) {
        initialize_project_settings(cx);
        let server_name = "io.example/cached-installation";
        let server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": server_name,
                "version": "1.0.0",
                "packages": [{
                    "registryType": "npm",
                    "identifier": "@example/cached-installation",
                    "version": "1.0.0",
                    "transport": {"type": "stdio"}
                }],
                "remotes": [{
                    "type": "streamable-http",
                    "url": "https://example.com/cached-mcp"
                }]
            }
        }))
        .expect("cached installation should parse");
        let selected_source = McpRegistryInstallationSource::Remote {
            url: "https://example.com/cached-mcp".to_owned(),
        };
        let fs: Arc<dyn Fs> = fs::FakeFs::new(cx.executor());
        let registry_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs.clone(),
                http_client::FakeHttpClient::with_404_response(),
                "https://registry.example.test/v0.1".to_owned(),
                false,
                cx,
            )
        });
        registry_store.update(cx, |store, cx| {
            store.remember_server_installation(server, selected_source.clone(), cx);
        });
        cx.run_until_parked();

        let cached_file: CachedServersFile = serde_json::from_slice(
            &fs.load_bytes(&registry_cache_path())
                .await
                .expect("registry cache should be written"),
        )
        .expect("registry cache should parse");
        assert_eq!(
            cached_file.source_hints.get(server_name),
            Some(&selected_source)
        );

        let reloaded_store = cx.new(|cx| {
            McpRegistryStore::new(
                fs,
                http_client::FakeHttpClient::with_404_response(),
                "https://registry.example.test/v0.1".to_owned(),
                true,
                cx,
            )
        });
        cx.run_until_parked();
        let (_, reloaded_source) = reloaded_store
            .update(cx, |store, cx| store.server_installation(server_name, cx))
            .await
            .expect("cached installation should resolve");
        assert_eq!(reloaded_source, selected_source);
    }

    #[gpui::test]
    async fn init_publishes_cache_without_fetching_the_list(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            ProjectSettings::register(cx);
            let mut project_settings = ProjectSettings::get_global(cx).clone();
            project_settings.context_servers.insert(
                "io.example/cached".into(),
                ContextServerSettings::Registry {
                    enabled: true,
                    remote: false,
                    registry: settings::McpRegistryServerSettings {
                        credential_id: None,
                        inputs: HashMap::default(),
                    },
                },
            );
            ProjectSettings::override_global(project_settings, cx);
        });
        assert_eq!(
            cx.update(|cx| installed_registry_server_names(cx)),
            HashSet::from_iter(["io.example/cached".to_owned()])
        );

        let cached_server: ServerResponse = serde_json::from_value(serde_json::json!({
            "server": {
                "name": "io.example/cached",
                "description": "Cached description",
                "version": "1.0.0"
            }
        }))
        .expect("cached server should parse");
        let cache_json = serde_json::to_vec(&CachedServersFile {
            servers: vec![cached_server],
            source_hints: HashMap::default(),
        })
        .expect("cache should serialize");
        let fs = fs::FakeFs::new(cx.executor());
        let cache_path = registry_cache_path();
        fs.create_dir(
            cache_path
                .parent()
                .expect("registry cache path should have a parent"),
        )
        .await
        .expect("registry cache directory should be created");
        fs.insert_file(cache_path, cache_json).await;

        let requests = Arc::new(Mutex::new(Vec::new()));
        let requests_for_client = requests.clone();
        let http_client = FakeHttpClient::create(move |request| {
            requests_for_client.lock().push(request.uri().to_string());
            future::pending::<anyhow::Result<Response<AsyncBody>>>()
        }) as Arc<dyn HttpClient>;

        let registry_store =
            cx.update(|cx| McpRegistryStore::init_global(cx, fs.clone(), http_client));
        cx.executor().allow_parking();
        let details_task = registry_store.update(cx, |store, cx| {
            store.server_details("io.example/cached", cx)
        });
        let details = details_task
            .await
            .expect("cached details should be available while refresh is pending");
        assert_eq!(details.description(), "Cached description");
        cx.run_until_parked();

        registry_store.read_with(cx, |store, _cx| {
            assert_eq!(
                store
                    .cached_server("io.example/cached")
                    .map(ServerResponse::description),
                Some("Cached description")
            );
            assert!(store.servers().is_empty());
            assert!(!store.is_fetching());
        });

        let requests = requests.lock();
        assert_eq!(requests.len(), 1);
        assert!(
            requests[0].ends_with(
                "/v0.1/servers/io.example%2Fcached/versions/latest?include_deleted=true"
            )
        );
    }
}

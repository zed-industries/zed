use anyhow::{Context as _, Result, bail};
use collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryServer {
    /// JSON Schema URI for this server.json format.
    #[serde(rename = "$schema", default)]
    pub schema: Option<String>,

    pub name: String,

    pub description: String,

    #[serde(default)]
    pub title: Option<String>,

    pub version: String,

    #[serde(default)]
    pub website_url: Option<String>,

    #[serde(default)]
    pub repository: Option<RegistryRepository>,

    #[serde(default)]
    pub icons: Vec<RegistryIcon>,

    #[serde(default)]
    pub packages: Vec<RegistryPackage>,

    #[serde(default)]
    pub remotes: Vec<RegistryRemoteTransport>,

    #[serde(rename = "_meta", default)]
    pub meta: Option<RegistryMeta>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryRepository {
    pub url: String,

    pub source: String,

    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub subfolder: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryIcon {
    pub src: String,

    #[serde(default)]
    pub sizes: Vec<String>,

    #[serde(default)]
    pub mime_type: Option<RegistryIconMimeType>,

    #[serde(default)]
    pub theme: Option<RegistryIconTheme>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistryIconTheme {
    Light,
    Dark,

    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RegistryIconMimeType {
    #[serde(rename = "image/png")]
    Png,

    #[serde(rename = "image/jpeg")]
    Jpeg,

    #[serde(rename = "image/jpg")]
    Jpg,

    #[serde(rename = "image/svg+xml")]
    Svg,

    #[serde(rename = "image/webp")]
    Webp,

    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryPackage {
    /// Package registry type, e.g. npm, pypi, oci, nuget, mcpb.
    pub registry_type: String,

    /// Package name or direct download URL.
    pub identifier: String,

    #[serde(default)]
    pub version: Option<String>,

    #[serde(default)]
    pub runtime_hint: Option<String>,

    #[serde(default)]
    pub runtime_arguments: Vec<RegistryArgument>,

    #[serde(default)]
    pub package_arguments: Vec<RegistryArgument>,

    #[serde(default)]
    pub environment_variables: Vec<RegistryKeyValueInput>,

    #[serde(default)]
    pub registry_base_url: Option<String>,

    #[serde(default)]
    pub file_sha256: Option<String>,

    pub transport: RegistryLocalTransport,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum RegistryArgument {
    #[serde(rename = "positional")]
    Positional {
        #[serde(flatten)]
        input: RegistryInput,

        #[serde(default)]
        is_repeated: bool,

        #[serde(default)]
        value_hint: Option<String>,
    },

    #[serde(rename = "named")]
    Named {
        #[serde(flatten)]
        input: RegistryInput,

        name: String,

        #[serde(default)]
        is_repeated: bool,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryInput {
    #[serde(default)]
    pub value: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub format: Option<RegistryInputFormat>,

    #[serde(default)]
    pub is_required: bool,

    #[serde(default)]
    pub is_secret: bool,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub default: Option<String>,

    #[serde(default)]
    pub choices: Vec<String>,
}

/// Input which can additionally contain named variables.
///
/// `variables` values are plain `RegistryInput`, so the structure is
/// intentionally non-recursive.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryInputWithVariables {
    #[serde(flatten)]
    pub input: RegistryInput,

    #[serde(default)]
    pub variables: BTreeMap<String, RegistryInput>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistryInputFormat {
    String,
    Number,
    Boolean,

    #[serde(rename = "filepath")]
    FilePath,

    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryKeyValueInput {
    pub name: String,

    #[serde(flatten)]
    pub input: RegistryInputWithVariables,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RegistryLocalTransport {
    #[serde(rename = "stdio")]
    Stdio,

    #[serde(rename = "streamable-http")]
    StreamableHttp {
        url: String,

        #[serde(default)]
        headers: Vec<RegistryKeyValueInput>,
    },

    #[serde(rename = "sse")]
    Sse {
        url: String,

        #[serde(default)]
        headers: Vec<RegistryKeyValueInput>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RegistryRemoteTransport {
    #[serde(rename = "streamable-http")]
    StreamableHttp {
        url: String,

        #[serde(default)]
        headers: Vec<RegistryKeyValueInput>,

        #[serde(default)]
        variables: BTreeMap<String, RegistryInput>,
    },

    #[serde(rename = "sse")]
    Sse {
        url: String,

        #[serde(default)]
        headers: Vec<RegistryKeyValueInput>,

        #[serde(default)]
        variables: BTreeMap<String, RegistryInput>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RegistryMeta {
    #[serde(rename = "io.modelcontextprotocol.registry/publisher-provided")]
    pub publisher_provided: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerListResponse {
    pub servers: Vec<ServerResponse>,

    #[serde(default)]
    pub metadata: ResponseMetadata,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    pub next_cursor: Option<String>,
    pub count: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerResponse {
    pub server: RegistryServer,

    #[serde(rename = "_meta", default)]
    pub meta: Option<RegistryOfficialMeta>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct RegistryOfficialMeta {
    #[serde(rename = "io.modelcontextprotocol.registry/official", default)]
    pub official: Option<RegistryOfficial>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryOfficial {
    pub published_at: Option<String>,

    #[serde(default)]
    pub is_latest: bool,

    #[serde(default)]
    pub status: Option<String>,
}

/// A registry server resolved into a runnable configuration, independent of
/// how Zed stores it in settings.
#[derive(Clone, Debug, PartialEq)]
pub enum MaterializedServer {
    Remote {
        url: String,
        headers: BTreeMap<String, String>,
    },
    Stdio {
        command: String,
        args: Vec<String>,
        env: BTreeMap<String, String>,
    },
}

/// An input that a user must (or may) provide before a registry server can be
/// materialized: it has neither a fixed `value` nor a `default`.
#[derive(Clone, Debug, PartialEq)]
pub struct RequiredInput {
    pub key: String,
    pub description: Option<String>,
    pub is_secret: bool,
    pub is_required: bool,
    pub choices: Vec<String>,
}

impl RegistryServer {
    /// All inputs without a fixed value or default, across every remote and
    /// package the server publishes.
    pub fn required_inputs(&self) -> Vec<RequiredInput> {
        let mut inputs = Vec::new();

        for remote in &self.remotes {
            let (headers, variables) = match remote {
                RegistryRemoteTransport::StreamableHttp {
                    headers, variables, ..
                }
                | RegistryRemoteTransport::Sse {
                    headers, variables, ..
                } => (headers, variables),
            };
            for (name, input) in variables {
                collect_required_input(&mut inputs, name, input);
            }
            for header in headers {
                collect_required_input(&mut inputs, &header.name, &header.input.input);
            }
        }

        for package in &self.packages {
            for variable in &package.environment_variables {
                collect_required_input(&mut inputs, &variable.name, &variable.input.input);
            }
            let arguments = package
                .package_arguments
                .iter()
                .chain(package.runtime_arguments.iter());
            for argument in arguments {
                match argument {
                    RegistryArgument::Positional {
                        input,
                        value_hint: Some(hint),
                        ..
                    } => collect_required_input(&mut inputs, hint, input),
                    RegistryArgument::Named { input, name, .. } => {
                        collect_required_input(&mut inputs, name, input)
                    }
                    RegistryArgument::Positional {
                        value_hint: None, ..
                    } => {}
                }
            }
        }

        inputs.sort_by(|first, second| first.key.cmp(&second.key));
        inputs.dedup_by(|first, second| first.key == second.key);
        inputs
    }

    /// Resolve this server into a runnable configuration.
    ///
    /// `remote_index`/`package_index` select which distribution to use when a
    /// server publishes several. When both are `None`, a remote (preferring
    /// streamable-http) is chosen over a package.
    ///
    /// Values in `values` are keyed by environment variable name, URL template
    /// variable name, positional argument `valueHint`, or named argument name
    /// (including leading dashes). Fixed `value`s and `default`s from the
    /// registry take precedence over user-provided values, per the
    /// `server.json` schema.
    ///
    /// Note: repeated arguments currently contribute a single occurrence, and
    /// packages that serve HTTP locally are not supported yet.
    pub fn materialize(
        &self,
        remote_index: Option<usize>,
        package_index: Option<usize>,
        values: &BTreeMap<String, String>,
    ) -> Result<MaterializedServer> {
        if let Some(index) = remote_index {
            let remote = self
                .remotes
                .get(index)
                .with_context(|| format!("no remote at index {index} for {}", self.name))?;
            return materialize_remote(remote, values);
        }
        if let Some(index) = package_index {
            let package = self
                .packages
                .get(index)
                .with_context(|| format!("no package at index {index} for {}", self.name))?;
            return materialize_package(package, values);
        }

        if let Some(remote) = self
            .remotes
            .iter()
            .find(|remote| matches!(remote, RegistryRemoteTransport::StreamableHttp { .. }))
            .or_else(|| self.remotes.first())
        {
            return materialize_remote(remote, values);
        }
        if let Some(package) = self.packages.first() {
            return materialize_package(package, values);
        }
        bail!("server {} has no remotes or packages", self.name);
    }
}

fn collect_required_input(inputs: &mut Vec<RequiredInput>, key: &str, input: &RegistryInput) {
    if input.value.is_some() || input.default.is_some() {
        return;
    }
    inputs.push(RequiredInput {
        key: key.to_string(),
        description: input.description.clone(),
        is_secret: input.is_secret,
        is_required: input.is_required,
        choices: input.choices.clone(),
    });
}

/// Resolves an input following the `server.json` precedence: fixed `value`,
/// then the user-provided value for `key`, then `default`. `Ok(None)` means
/// the input is optional and unresolved, so it can be omitted.
fn resolve_input(
    input: &RegistryInput,
    key: Option<&str>,
    values: &BTreeMap<String, String>,
) -> Result<Option<String>> {
    if let Some(value) = &input.value {
        return Ok(Some(value.clone()));
    }
    let resolved = key
        .and_then(|key| values.get(key))
        .cloned()
        .or_else(|| input.default.clone());
    match resolved {
        Some(value) => Ok(Some(value)),
        None if input.is_required => {
            bail!("missing value for required input {}", key.unwrap_or("?"))
        }
        None => Ok(None),
    }
}

/// Replaces `{name}` placeholders in `value` using the input's own `variables`
/// map. Unknown placeholders are left untouched, per the schema.
fn substitute_variables(value: &str, variables: &BTreeMap<String, RegistryInput>) -> String {
    let mut result = value.to_string();
    for (name, input) in variables {
        if let Some(replacement) = input.value.clone().or_else(|| input.default.clone()) {
            result = result.replace(&format!("{{{name}}}"), &replacement);
        }
    }
    result
}

fn resolve_argument(
    argument: &RegistryArgument,
    values: &BTreeMap<String, String>,
) -> Result<Option<String>> {
    match argument {
        RegistryArgument::Positional {
            input,
            value_hint: hint,
            ..
        } => resolve_input(input, hint.as_deref(), values),
        RegistryArgument::Named { input, name, .. } => {
            Ok(resolve_input(input, Some(name), values)?.map(|value| format!("{name}={value}")))
        }
    }
}

fn materialize_remote(
    remote: &RegistryRemoteTransport,
    values: &BTreeMap<String, String>,
) -> Result<MaterializedServer> {
    let (url, headers, variables) = match remote {
        RegistryRemoteTransport::StreamableHttp {
            url,
            headers,
            variables,
        }
        | RegistryRemoteTransport::Sse {
            url,
            headers,
            variables,
        } => (url, headers, variables),
    };

    let mut resolved_url = url.clone();
    for (name, input) in variables {
        if let Some(value) = resolve_input(input, Some(name), values)? {
            resolved_url = resolved_url.replace(&format!("{{{name}}}"), &value);
        }
    }
    url::Url::parse(&resolved_url)
        .with_context(|| format!("invalid URL {resolved_url:?} after variable substitution"))?;

    let mut resolved_headers = BTreeMap::new();
    for header in headers {
        if let Some(value) = resolve_input(&header.input.input, Some(&header.name), values)? {
            let value = substitute_variables(&value, &header.input.variables);
            resolved_headers.insert(header.name.clone(), value);
        }
    }

    Ok(MaterializedServer::Remote {
        url: resolved_url,
        headers: resolved_headers,
    })
}

fn materialize_package(
    package: &RegistryPackage,
    values: &BTreeMap<String, String>,
) -> Result<MaterializedServer> {
    let (command, mut args) = match (&package.runtime_hint, package.registry_type.as_str()) {
        (Some(runtime_hint), _) => (runtime_hint.clone(), Vec::new()),
        (None, "npm") => ("npx".to_string(), vec!["-y".to_string()]),
        (None, "pypi") => ("uvx".to_string(), Vec::new()),
        (None, "oci") => ("docker".to_string(), vec!["run".to_string()]),
        (None, registry_type) => {
            bail!(
                "the {registry_type} package type is not supported yet ({})",
                package.identifier
            )
        }
    };

    for argument in &package.runtime_arguments {
        if let Some(argument) = resolve_argument(argument, values)? {
            args.push(argument);
        }
    }
    args.push(package.identifier.clone());
    for argument in &package.package_arguments {
        if let Some(argument) = resolve_argument(argument, values)? {
            args.push(argument);
        }
    }

    let mut env = BTreeMap::new();
    for variable in &package.environment_variables {
        if let Some(value) = resolve_input(&variable.input.input, Some(&variable.name), values)? {
            let value = substitute_variables(&value, &variable.input.variables);
            env.insert(variable.name.clone(), value);
        }
    }

    match &package.transport {
        RegistryLocalTransport::Stdio => Ok(MaterializedServer::Stdio { command, args, env }),
        RegistryLocalTransport::StreamableHttp { url, .. }
        | RegistryLocalTransport::Sse { url, .. } => {
            bail!(
                "package {} starts a local HTTP server ({url}); \
                 packages serving HTTP locally are not supported yet",
                package.identifier
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NPM_STDIO_SERVER: &str = r#"
        {
          "$schema": "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json",
          "name": "io.github.modelcontextprotocol/filesystem",
          "title": "Filesystem",
          "description": "Node.js server implementing Model Context Protocol for filesystem operations.",
          "version": "1.0.2",
          "websiteUrl": "https://modelcontextprotocol.io/examples",
          "repository": {
            "url": "https://github.com/modelcontextprotocol/servers",
            "source": "github",
            "id": "b94b5f7e-c7c6-d760-2c78-a5e9b8a5b8c9",
            "subfolder": "src/filesystem"
          },
          "icons": [
            {
              "src": "https://example.com/icon.png",
              "sizes": ["48x48"],
              "mimeType": "image/png"
            },
            {
              "src": "https://example.com/icon.svg",
              "sizes": ["any"],
              "mimeType": "image/svg+xml",
              "theme": "light"
            }
          ],
          "packages": [
            {
              "registryType": "npm",
              "registryBaseUrl": "https://registry.npmjs.org",
              "identifier": "@modelcontextprotocol/server-filesystem",
              "version": "1.0.2",
              "transport": { "type": "stdio" },
              "packageArguments": [
                {
                  "type": "positional",
                  "valueHint": "allowed_directory",
                  "isRepeated": true,
                  "description": "Directories the server may access"
                },
                {
                  "type": "named",
                  "name": "--mode",
                  "default": "restricted",
                  "choices": ["restricted", "unrestricted"]
                }
              ],
              "environmentVariables": [
                {
                  "name": "FILESYSTEM_API_KEY",
                  "description": "API key for the filesystem service",
                  "isRequired": true,
                  "isSecret": true
                },
                {
                  "name": "FILESYSTEM_BASE_URL",
                  "value": "{host}/v1",
                  "variables": {
                    "host": { "default": "https://example.com" }
                  }
                }
              ]
            }
          ]
        }
    "#;

    const REMOTE_SERVER: &str = r#"
        {
          "name": "ac.inference.sh/mcp",
          "title": "inference.sh",
          "description": "Run 150+ AI apps",
          "version": "1.0.1",
          "remotes": [
            {
              "type": "streamable-http",
              "url": "https://api.{region}.inference.sh/mcp",
              "headers": [
                { "name": "X-Api-Version", "value": "2026-01-01" }
              ],
              "variables": {
                "region": {
                  "description": "Service region",
                  "default": "us-east-1",
                  "choices": ["us-east-1", "eu-west-1", "ap-southeast-1"],
                  "isRequired": true
                }
              }
            },
            {
              "type": "sse",
              "url": "https://api.inference.sh/sse"
            }
          ]
        }
    "#;

    // The live API returns entries in this shape (captured from
    // registry.modelcontextprotocol.io). Note that an unfiltered list returns
    // every published version of a server, not just the latest one.
    const SERVER_LIST_RESPONSE: &str = r#"
        {
          "servers": [
            {
              "server": {
                "$schema": "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json",
                "name": "ac.inference.sh/mcp",
                "description": "Run 150+ AI apps",
                "title": "inference.sh",
                "version": "1.0.0",
                "remotes": [
                  { "type": "streamable-http", "url": "https://sh.inference.ac" },
                  { "type": "streamable-http", "url": "https://api.inference.sh/mcp" }
                ]
              },
              "_meta": {
                "io.modelcontextprotocol.registry/official": {
                  "status": "active",
                  "statusChangedAt": "2026-04-13T17:32:20.852269Z",
                  "publishedAt": "2026-04-13T17:32:20.852269Z",
                  "updatedAt": "2026-04-13T17:32:20.852269Z",
                  "isLatest": false
                }
              }
            },
            {
              "server": {
                "$schema": "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json",
                "name": "ac.inference.sh/mcp",
                "description": "Run 150+ AI apps",
                "title": "inference.sh",
                "version": "1.0.1",
                "remotes": [
                  { "type": "streamable-http", "url": "https://sh.inference.ac" },
                  { "type": "streamable-http", "url": "https://api.inference.sh/mcp" }
                ]
              },
              "_meta": {
                "io.modelcontextprotocol.registry/official": {
                  "status": "active",
                  "statusChangedAt": "2026-04-13T17:33:26.613537Z",
                  "publishedAt": "2026-04-13T17:33:26.613537Z",
                  "updatedAt": "2026-04-13T17:33:26.613537Z",
                  "isLatest": false
                }
              }
            }
          ],
          "metadata": {
            "nextCursor": "ac.inference.sh/mcp:1.0.1",
            "count": 2
          }
        }
    "#;

    #[test]
    fn test_deserialize_npm_stdio_server() {
        let server: RegistryServer = serde_json::from_str(NPM_STDIO_SERVER).unwrap();
        assert_eq!(
            server.schema.as_deref(),
            Some("https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json")
        );
        assert_eq!(server.name, "io.github.modelcontextprotocol/filesystem");
        assert_eq!(server.title.as_deref(), Some("Filesystem"));
        assert_eq!(server.version, "1.0.2");

        let repository = server.repository.as_ref().unwrap();
        assert_eq!(
            repository.url,
            "https://github.com/modelcontextprotocol/servers"
        );
        assert_eq!(repository.source, "github");
        assert_eq!(repository.subfolder.as_deref(), Some("src/filesystem"));

        assert_eq!(server.icons.len(), 2);
        assert!(matches!(
            server.icons[0].mime_type,
            Some(RegistryIconMimeType::Png)
        ));
        assert!(matches!(
            server.icons[1].theme,
            Some(RegistryIconTheme::Light)
        ));

        assert_eq!(server.packages.len(), 1);
        let package = &server.packages[0];
        assert_eq!(package.registry_type, "npm");
        assert_eq!(
            package.identifier,
            "@modelcontextprotocol/server-filesystem"
        );
        assert!(matches!(package.transport, RegistryLocalTransport::Stdio));

        assert_eq!(package.package_arguments.len(), 2);
        assert!(
            matches!(&package.package_arguments[0], RegistryArgument::Positional { value_hint, is_repeated, .. }
                if value_hint.as_deref() == Some("allowed_directory") && *is_repeated)
        );
        assert!(
            matches!(&package.package_arguments[1], RegistryArgument::Named { name, input, .. }
                if name == "--mode" && input.default.as_deref() == Some("restricted"))
        );
        assert!(
            matches!(&package.package_arguments[1], RegistryArgument::Named { input, .. }
                if input.choices.len() == 2)
        );

        assert_eq!(package.environment_variables.len(), 2);
        assert_eq!(package.environment_variables[0].name, "FILESYSTEM_API_KEY");
        assert!(package.environment_variables[0].input.input.is_required);
        assert!(package.environment_variables[0].input.input.is_secret);
        assert_eq!(package.environment_variables[1].name, "FILESYSTEM_BASE_URL");
        assert_eq!(
            package.environment_variables[1]
                .input
                .input
                .value
                .as_deref(),
            Some("{host}/v1")
        );
        assert!(
            package.environment_variables[1]
                .input
                .variables
                .contains_key("host")
        );
    }

    #[test]
    fn test_deserialize_remote_server() {
        let server: RegistryServer = serde_json::from_str(REMOTE_SERVER).unwrap();
        assert!(server.packages.is_empty());
        assert_eq!(server.remotes.len(), 2);

        let RegistryRemoteTransport::StreamableHttp {
            url,
            headers,
            variables,
        } = &server.remotes[0]
        else {
            panic!("expected streamable-http remote");
        };
        assert_eq!(url, "https://api.{region}.inference.sh/mcp");
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0].name, "X-Api-Version");
        assert_eq!(headers[0].input.input.value.as_deref(), Some("2026-01-01"));

        let region = variables.get("region").unwrap();
        assert_eq!(region.description.as_deref(), Some("Service region"));
        assert_eq!(region.default.as_deref(), Some("us-east-1"));
        assert_eq!(
            region.choices,
            vec!["us-east-1", "eu-west-1", "ap-southeast-1"]
        );
        assert!(region.is_required);

        assert!(
            matches!(&server.remotes[1], RegistryRemoteTransport::Sse { url, .. } if url == "https://api.inference.sh/sse")
        );
    }

    const HYBRID_SERVER: &str = r#"
        {
          "name": "com.example/hybrid",
          "description": "Hybrid server",
          "version": "2.0.0",
          "remotes": [
            { "type": "streamable-http", "url": "https://mcp.example.com/http" }
          ],
          "packages": [
            {
              "registryType": "oci",
              "registryBaseUrl": "docker.io",
              "identifier": "example/hybrid-mcp",
              "version": "2.0.0",
              "runtimeHint": "docker",
              "runtimeArguments": [
                { "type": "positional", "value": "run" },
                { "type": "positional", "value": "-p" },
                { "type": "positional", "value": "8080:8080" },
                { "type": "named", "name": "--env", "value": "PRODUCTION", "isRepeated": true }
              ],
              "transport": { "type": "stdio" }
            }
          ]
        }
    "#;

    #[test]
    fn test_deserialize_hybrid_server_with_runtime_arguments() {
        let server: RegistryServer = serde_json::from_str(HYBRID_SERVER).unwrap();
        assert_eq!(server.remotes.len(), 1);
        assert_eq!(server.packages.len(), 1);

        let package = &server.packages[0];
        assert_eq!(package.runtime_hint.as_deref(), Some("docker"));
        assert!(matches!(package.transport, RegistryLocalTransport::Stdio));
        assert_eq!(package.runtime_arguments.len(), 4);
        assert!(
            matches!(&package.runtime_arguments[0], RegistryArgument::Positional { input, .. }
                if input.value.as_deref() == Some("run"))
        );
        assert!(
            matches!(&package.runtime_arguments[3], RegistryArgument::Named { name, is_repeated, .. }
                if name == "--env" && *is_repeated)
        );
    }

    #[test]
    fn test_deserialize_minimal_server() {
        let json = r#"
            { "name": "io.example/minimal", "description": "Minimal", "version": "0.1.0" }
        "#;
        let server: RegistryServer = serde_json::from_str(json).unwrap();
        assert_eq!(server.name, "io.example/minimal");
        assert_eq!(server.description, "Minimal");
        assert_eq!(server.version, "0.1.0");
        assert!(server.schema.is_none());
        assert!(server.title.is_none());
        assert!(server.website_url.is_none());
        assert!(server.repository.is_none());
        assert!(server.meta.is_none());
        assert!(server.icons.is_empty());
        assert!(server.packages.is_empty());
        assert!(server.remotes.is_empty());
    }

    #[test]
    fn test_deserialize_publisher_provided_meta() {
        let json = r#"
            {
              "name": "io.example/publisher-meta",
              "description": "Publisher metadata",
              "version": "1.0.0",
              "_meta": {
                "io.modelcontextprotocol.registry/publisher-provided": {
                  "tool": "publisher-cli",
                  "version": "1.2.3"
                }
              }
            }
        "#;
        let server: RegistryServer = serde_json::from_str(json).unwrap();
        let meta = server.meta.as_ref().unwrap();
        let publisher_provided = meta.publisher_provided.as_ref().unwrap();
        assert_eq!(publisher_provided["tool"], "publisher-cli");
        assert_eq!(publisher_provided["version"], "1.2.3");
    }

    #[test]
    fn test_deserialize_server_list_response() {
        let response: ServerListResponse = serde_json::from_str(SERVER_LIST_RESPONSE).unwrap();
        assert_eq!(response.servers.len(), 2);

        // The same server appears twice with different versions; only the
        // envelope's official metadata tells us which entry is current.
        assert_eq!(response.servers[0].server.name, "ac.inference.sh/mcp");
        assert_eq!(response.servers[1].server.name, "ac.inference.sh/mcp");
        assert_eq!(response.servers[0].server.version, "1.0.0");
        assert_eq!(response.servers[1].server.version, "1.0.1");

        let official = response.servers[0]
            .meta
            .as_ref()
            .unwrap()
            .official
            .as_ref()
            .unwrap();
        assert_eq!(
            official.published_at.as_deref(),
            Some("2026-04-13T17:32:20.852269Z")
        );
        assert!(!official.is_latest);
        assert_eq!(official.status.as_deref(), Some("active"));

        assert_eq!(
            response.metadata.next_cursor.as_deref(),
            Some("ac.inference.sh/mcp:1.0.1")
        );
        assert_eq!(response.metadata.count, Some(2));
    }

    #[test]
    fn test_unknown_enum_values_are_tolerated() {
        let json = r#"
            {
              "name": "io.example/future",
              "description": "Server using enum values Zed does not know about",
              "version": "1.0.0",
              "icons": [
                { "src": "https://example.com/icon.avif", "mimeType": "image/avif", "theme": "high-contrast" }
              ],
              "packages": [
                {
                  "registryType": "mcpb-v2",
                  "identifier": "example/future",
                  "transport": { "type": "stdio" },
                  "environmentVariables": [
                    { "name": "DURATION", "format": "duration" }
                  ]
                }
              ]
            }
        "#;
        let server: RegistryServer = serde_json::from_str(json).unwrap();
        assert!(matches!(
            server.icons[0].mime_type,
            Some(RegistryIconMimeType::Unknown)
        ));
        assert!(matches!(
            server.icons[0].theme,
            Some(RegistryIconTheme::Unknown)
        ));
        assert!(matches!(
            server.packages[0].environment_variables[0]
                .input
                .input
                .format,
            Some(RegistryInputFormat::Unknown)
        ));
        assert_eq!(server.packages[0].registry_type, "mcpb-v2");
    }

    #[test]
    fn test_package_missing_transport_fails() {
        let json = r#"
            {
              "name": "io.example/bad",
              "description": "Package without required transport",
              "version": "1.0.0",
              "packages": [
                { "registryType": "npm", "identifier": "example/bad" }
              ]
            }
        "#;
        assert!(serde_json::from_str::<RegistryServer>(json).is_err());
    }

    #[test]
    fn test_round_trip_serialization() {
        let server: RegistryServer = serde_json::from_str(NPM_STDIO_SERVER).unwrap();
        let serialized = serde_json::to_value(&server).unwrap();
        let deserialized: RegistryServer = serde_json::from_value(serialized.clone()).unwrap();
        assert_eq!(serde_json::to_value(&deserialized).unwrap(), serialized);
    }

    #[test]
    fn test_required_inputs() {
        let npm_server: RegistryServer = serde_json::from_str(NPM_STDIO_SERVER).unwrap();
        let inputs = npm_server.required_inputs();
        assert_eq!(inputs.len(), 2);
        assert!(inputs.iter().any(|input| input.key == "FILESYSTEM_API_KEY"
            && input.is_required
            && input.is_secret));
        assert!(
            inputs
                .iter()
                .any(|input| input.key == "allowed_directory" && !input.is_required)
        );

        // Everything else resolves without user input: `--mode` has a default,
        // `FILESYSTEM_BASE_URL` has a fixed value.
        let remote_server: RegistryServer = serde_json::from_str(REMOTE_SERVER).unwrap();
        // The `region` variable has a default and the header a fixed value, so
        // nothing needs prompting.
        assert!(remote_server.required_inputs().is_empty());
    }

    #[test]
    fn test_materialize_npm_stdio() {
        let server: RegistryServer = serde_json::from_str(NPM_STDIO_SERVER).unwrap();
        let values = BTreeMap::from([
            ("FILESYSTEM_API_KEY".to_string(), "secret-key".to_string()),
            ("allowed_directory".to_string(), "/tmp".to_string()),
            ("--mode".to_string(), "unrestricted".to_string()),
        ]);
        let MaterializedServer::Stdio { command, args, env } =
            server.materialize(None, Some(0), &values).unwrap()
        else {
            panic!("expected stdio server");
        };
        assert_eq!(command, "npx");
        assert_eq!(
            args,
            vec![
                "-y",
                "@modelcontextprotocol/server-filesystem",
                "/tmp",
                "--mode=unrestricted"
            ]
        );
        assert_eq!(
            env.get("FILESYSTEM_API_KEY").map(String::as_str),
            Some("secret-key")
        );
        // `{host}/v1` is substituted using the `variables.host` default.
        assert_eq!(
            env.get("FILESYSTEM_BASE_URL").map(String::as_str),
            Some("https://example.com/v1")
        );
    }

    #[test]
    fn test_materialize_missing_required_value_fails() {
        let server: RegistryServer = serde_json::from_str(NPM_STDIO_SERVER).unwrap();
        let error = server
            .materialize(None, Some(0), &BTreeMap::new())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("FILESYSTEM_API_KEY"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn test_materialize_remote_with_variables() {
        let server: RegistryServer = serde_json::from_str(REMOTE_SERVER).unwrap();

        // User-provided value wins over the variable default.
        let values = BTreeMap::from([("region".to_string(), "eu-west-1".to_string())]);
        let MaterializedServer::Remote { url, headers } =
            server.materialize(None, None, &values).unwrap()
        else {
            panic!("expected remote server");
        };
        assert_eq!(url, "https://api.eu-west-1.inference.sh/mcp");
        assert_eq!(
            headers.get("X-Api-Version").map(String::as_str),
            Some("2026-01-01")
        );

        // Without user input, the declared default is used.
        let MaterializedServer::Remote { url, .. } =
            server.materialize(None, None, &BTreeMap::new()).unwrap()
        else {
            panic!("expected remote server");
        };
        assert_eq!(url, "https://api.us-east-1.inference.sh/mcp");
    }

    #[test]
    fn test_materialize_prefers_remote_over_package() {
        let server: RegistryServer = serde_json::from_str(HYBRID_SERVER).unwrap();
        assert!(matches!(
            server.materialize(None, None, &BTreeMap::new()).unwrap(),
            MaterializedServer::Remote { .. }
        ));
        assert!(matches!(
            server.materialize(None, Some(0), &BTreeMap::new()).unwrap(),
            MaterializedServer::Stdio { .. }
        ));
    }

    #[test]
    fn test_materialize_oci_package() {
        let server: RegistryServer = serde_json::from_str(HYBRID_SERVER).unwrap();
        let MaterializedServer::Stdio { command, args, env } =
            server.materialize(None, Some(0), &BTreeMap::new()).unwrap()
        else {
            panic!("expected stdio server");
        };
        // `runtimeHint` provides the command; runtime arguments precede the
        // image, package arguments would follow it.
        assert_eq!(command, "docker");
        assert_eq!(
            args,
            vec![
                "run",
                "-p",
                "8080:8080",
                "--env=PRODUCTION",
                "example/hybrid-mcp"
            ]
        );
        assert!(env.is_empty());
    }

    #[test]
    fn test_materialize_oci_package_without_runtime_hint() {
        let json = r#"
            {
              "name": "io.example/docker",
              "description": "Docker server",
              "version": "1.0.0",
              "packages": [
                {
                  "registryType": "oci",
                  "identifier": "example/server",
                  "transport": { "type": "stdio" }
                }
              ]
            }
        "#;
        let server: RegistryServer = serde_json::from_str(json).unwrap();
        let MaterializedServer::Stdio { command, args, .. } =
            server.materialize(None, None, &BTreeMap::new()).unwrap()
        else {
            panic!("expected stdio server");
        };
        assert_eq!(command, "docker");
        assert_eq!(args, vec!["run", "example/server"]);
    }

    #[test]
    fn test_materialize_unsupported_package_type_fails() {
        let json = r#"
            {
              "name": "io.example/unsupported",
              "description": "Unsupported package type",
              "version": "1.0.0",
              "packages": [
                {
                  "registryType": "mcpb",
                  "identifier": "example/unsupported",
                  "transport": { "type": "stdio" }
                }
              ]
            }
        "#;
        let server: RegistryServer = serde_json::from_str(json).unwrap();
        let error = server
            .materialize(None, None, &BTreeMap::new())
            .unwrap_err()
            .to_string();
        assert!(error.contains("not supported"), "unexpected error: {error}");
    }

    #[test]
    fn test_materialize_local_http_package_fails() {
        let json = r#"
            {
              "name": "io.example/local-http",
              "description": "Package serving HTTP locally",
              "version": "1.0.0",
              "packages": [
                {
                  "registryType": "npm",
                  "identifier": "@example/local-http",
                  "transport": {
                    "type": "streamable-http",
                    "url": "http://localhost:{--port}/mcp"
                  },
                  "packageArguments": [
                    { "type": "named", "name": "--port", "value": "8080" }
                  ]
                }
              ]
            }
        "#;
        let server: RegistryServer = serde_json::from_str(json).unwrap();
        let error = server
            .materialize(None, None, &BTreeMap::new())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("not supported yet"),
            "unexpected error: {error}"
        );
    }
}

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

use http_client::HttpClient;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json_lenient::Value;
use smol::process::Command;
use util::ResultExt;

use crate::{
    DevContainerConfig, DevContainerErrorV2,
    command_json::evaluate_json_command,
    devcontainer_api::DevContainerUp,
    docker::{DockerInspect, DockerPs, docker_cli, inspect_image},
    features::{
        FeatureManifest, download_and_extract_oci_feature, fetch_oci_feature_manifest,
        parse_oci_feature_ref,
    },
    get_oci_token_for_repo, safe_id_lower,
};

/**
 * What's needed next:
 * - Create a DevContainer manifest
 * - Load the features up front (and put them in that manifest)
 * - Move merged stuff into that struct
 * - Move variable expansion into that struct
 * - Continue on with lifecycle scripts
 *
 */

/*
 * - What's left now:
 * INITIALIZING the dev container (next week)
 * - Pulling from the known sources
 * - Expanding the template into appropriate files
 * - Adding the features to devcontainer.json as they are defined (TODO ensure you understand whether they can interoperate with Dockerfiles, etc)
 * CUSTOMIZING the dev container (following week)
 * - Defining how extensions can be added
 * EASE OF USE (following week)
 * - Detect when devcontainer.json/definition is changed, offer to rebuild
 * - Provide option to rebuild in any event
 * - When installing an extension from within a dev container, offer to add it to the json definition
 */
// So, when remoteUser is specified here, it seems that this is _not_ propagated to the labels in the docker container
// Which is interesting. I guess the read-configuration API just need to talk to the file, not the docker itself
// And the configuration doesn't make any promises about creating the user. Still weird though
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainer {
    pub(crate) image: Option<String>,
    pub(crate) name: Option<String>,
    remote_user: Option<String>,
    forward_ports: Option<Vec<ForwardPort>>,
    ports_attributes: Option<HashMap<String, PortAttributes>>, // TODO key here should be the same object as what's used above for forwardPorts
    other_ports_attributes: Option<PortAttributes>, // TODO I think that's right? Confirm the spec when you get to it
    container_env: Option<HashMap<String, String>>,
    remote_env: Option<HashMap<String, String>>,
    container_user: Option<String>,
    #[serde(rename = "updateRemoteUserUID")]
    update_remote_user_uid: Option<bool>,
    user_env_probe: Option<UserEnvProbe>,
    override_command: Option<bool>,
    shutdown_action: Option<ShutdownAction>,
    init: Option<bool>,
    privileged: Option<bool>,
    cap_add: Option<Vec<String>>,
    security_opt: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_mount_definitions")]
    mounts: Option<Vec<MountDefinition>>,
    features: Option<HashMap<String, FeatureOptions>>,
    override_feature_install_order: Option<Vec<String>>,
    // TODO
    customizations: Option<HashMap<String, Value>>,
    build: Option<ContainerBuild>,
    #[serde(default, deserialize_with = "deserialize_string_or_int")]
    app_port: Option<String>,
    workspace_mount: Option<String>,
    workspace_folder: Option<String>,
    run_args: Option<Vec<String>>,
    // Docker compose stuff:
    #[serde(default, deserialize_with = "deserialize_string_or_array")]
    docker_compose_file: Option<Vec<String>>,
    service: Option<String>,
    run_services: Option<Vec<String>>,
    // Scripts
    initialize_command: Option<LifecyleScript>,
    on_create_command: Option<LifecyleScript>,
    update_content_command: Option<LifecyleScript>,
    post_create_command: Option<LifecyleScript>,
    post_start_command: Option<LifecyleScript>,
    post_attach_command: Option<LifecyleScript>,
    wait_for: Option<LifecycleCommand>,
    host_requirements: Option<HostRequirements>,
}

#[derive(Debug, Eq, PartialEq, Default)]
struct DevContainerManifest {
    config: DevContainer,
    directory: PathBuf,
    file_name: String,
    root_image: Option<DockerInspect>,
    features_build_info: Option<FeaturesBuildInfo>,
    features: Vec<FeatureManifest>,
}
impl DevContainerManifest {
    fn new(config_path: &Path) -> Result<Self, DevContainerErrorV2> {
        log::info!("parsing devcontainer json found in {:?}", &config_path);
        let devcontainer_contents = std::fs::read_to_string(&config_path).map_err(|e| {
            log::error!("Unable to read devcontainer contents: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        let devcontainer = deserialize_devcontainer_json(&devcontainer_contents)?;

        let devcontainer_directory = config_path.parent().ok_or_else(|| {
            log::error!("Dev container file should be in a directory");
            DevContainerErrorV2::UnmappedError
        })?;
        let file_name = config_path
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or_else(|| {
                log::error!("Dev container file has no file name, or is invalid unicode");
                DevContainerErrorV2::UnmappedError
            })?;

        Ok(Self {
            config: devcontainer,
            directory: devcontainer_directory.to_path_buf(),
            file_name: file_name.to_string(),
            root_image: None,
            features_build_info: None,
            features: Vec::new(),
        })
    }

    fn validate_config(&self) -> Result<(), DevContainerErrorV2> {
        // TODO
        Ok(())
    }
    fn config_file(&self) -> PathBuf {
        self.directory.join(&self.file_name).clone()
    }

    // Ugh don't let this out. Needs simplification
    async fn dockerfile_location(&self) -> Option<PathBuf> {
        match self.config.build_type() {
            DevContainerBuildType::Image => None,
            DevContainerBuildType::Dockerfile => self
                .config
                .build
                .as_ref()
                .map(|build| self.directory.join(&build.dockerfile)),
            DevContainerBuildType::DockerCompose => {
                let Ok(docker_compose_manifest) = docker_compose_manifest(self).await else {
                    return None;
                };
                let Ok((_, main_service)) = find_primary_service(&docker_compose_manifest, self)
                else {
                    return None;
                };
                main_service
                    .build
                    .and_then(|b| b.dockerfile)
                    .map(|dockerfile| self.directory.join(dockerfile))
            }
            DevContainerBuildType::None => None,
        }
    }

    fn generate_features_image_tag(&self, dockerfile_build_path: String) -> String {
        let mut hasher = DefaultHasher::new();
        let prefix = match &self.config.name {
            Some(name) => &safe_id_lower(name),
            None => "zed-dc",
        };
        let prefix = prefix.get(..6).unwrap_or(prefix);

        dockerfile_build_path.hash(&mut hasher);

        let hash = hasher.finish();
        format!("{}-{:x}-features", prefix, hash)
    }

    fn feature_resource_directory(&self) -> Result<PathBuf, DevContainerErrorV2> {
        self.features_build_info
            .as_ref()
            .map(|build_info| build_info.features_content_dir.clone())
            .ok_or_else(|| {
                log::error!(
                    "Cannot get feature resource directory: Features build info not yet constructed"
                );
                DevContainerErrorV2::UnmappedError
            })
    }

    fn build_info_build_image(&self) -> Option<String> {
        self.features_build_info
            .as_ref()
            .and_then(|build_info| build_info.build_image.clone())
    }

    fn build_dockerfile_path(&self) -> Result<PathBuf, DevContainerErrorV2> {
        self.features_build_info
            .as_ref()
            .map(|build_info| build_info.dockerfile_path.clone())
            .ok_or_else(|| {
                log::error!(
                    "Cannot get build dockerfile path: Features build info not yet constructed"
                );
                DevContainerErrorV2::UnmappedError
            })
    }
    fn build_image_tag(&self) -> Result<String, DevContainerErrorV2> {
        self.features_build_info
            .as_ref()
            .map(|build_info| build_info.image_tag.clone())
            .ok_or_else(|| {
                log::error!(
                    "Cannot get build dockerfile path: Features build info not yet constructed"
                );
                DevContainerErrorV2::UnmappedError
            })
    }
    fn build_empty_context_dir(&self) -> Result<PathBuf, DevContainerErrorV2> {
        self.features_build_info
            .as_ref()
            .map(|build_info| build_info.empty_context_dir.clone())
            .ok_or_else(|| {
                log::error!(
                    "Cannot get build dockerfile path: Features build info not yet constructed"
                );
                DevContainerErrorV2::UnmappedError
            })
    }

    fn base_image(&self) -> Option<DockerInspect> {
        self.root_image.clone()
    }

    async fn download_resources(&mut self) -> Result<(), DevContainerErrorV2> {
        let root_image_tag = get_base_image_from_config(self).await?;
        if let Ok(image) = inspect_image(&root_image_tag).await {
            self.root_image = Some(image);
        }

        if self.config.build_type() == DevContainerBuildType::Image && !self.config.has_features() {
            log::info!("No resources to download. Proceeding with just the image");
            return Ok(());
        }
        // Covers both image and Dockefile cases, not yet docker-compose
        let temp_base = std::env::temp_dir().join("devcontainer-zed");
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let features_content_dir = temp_base.join(format!("container-features-{}", timestamp));
        let empty_context_dir = temp_base.join("empty-folder");

        std::fs::create_dir_all(&features_content_dir).map_err(|e| {
            log::error!("Failed to create features content dir: {e}");
            DevContainerErrorV2::UnmappedError
        })?;
        std::fs::create_dir_all(&empty_context_dir).map_err(|e| {
            log::error!("Failed to create empty context dir: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        let dockerfile_path = features_content_dir.join("Dockerfile.extended");
        let image_tag =
            self.generate_features_image_tag(dockerfile_path.clone().display().to_string());

        self.features_build_info = Some(FeaturesBuildInfo {
            dockerfile_path,
            features_content_dir,
            empty_context_dir,
            build_image: self.config.image.clone(),
            image_tag,
        });

        Ok(())
    }

    async fn construct_build_resources(
        &mut self,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<(), DevContainerErrorV2> {
        let Some(build_info) = &self.features_build_info else {
            log::error!("No build info prepared, cannot download resources");
            return Err(DevContainerErrorV2::UnmappedError);
        };
        // TODO probably a more elegant way of doing this
        let features = match &self.config.features {
            Some(features) => features,
            None => &HashMap::new(),
        };

        let base_image = get_base_image_from_config(self).await?;
        let base_image = inspect_image(&base_image).await?;

        let container_user = get_container_user_from_config(&base_image, self)?;
        let remote_user = get_remote_user_from_config(&base_image, self)?;

        // --- 1. Write devcontainer-features.builtin.env ---
        // So this is a slight problem - containerUser should be what's specified in devcontainer or root
        // remoteUser should be what's specified in devcontainer or what's specified in image or containerUser
        let builtin_env_content = format!(
            "_CONTAINER_USER={}\n_REMOTE_USER={}\n",
            container_user, remote_user
        );
        let builtin_env_path = build_info
            .features_content_dir
            .join("devcontainer-features.builtin.env");
        std::fs::write(&builtin_env_path, &builtin_env_content).map_err(|e| {
            log::error!("Failed to write builtin env file: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        // --- 2. Determine installation order ---
        let ordered_features =
            resolve_feature_order(features, &self.config.override_feature_install_order);

        // --- 3. Stage each feature's directory and files ---
        let mut feature_layers = String::new();

        for (index, (feature_ref, options)) in ordered_features.iter().enumerate() {
            if matches!(options, FeatureOptions::Bool(false)) {
                log::info!(
                    "Feature '{}' is disabled (set to false), skipping",
                    feature_ref
                );
                continue;
            }

            let feature_id = extract_feature_id(feature_ref);
            let consecutive_id = format!("{}_{}", feature_id, index);
            let feature_dir = build_info.features_content_dir.join(&consecutive_id);

            std::fs::create_dir_all(&feature_dir).map_err(|e| {
                log::error!(
                    "Failed to create feature directory for {}: {e}",
                    feature_ref
                );
                DevContainerErrorV2::UnmappedError
            })?;

            // --- Download the feature's OCI tarball first, so we can read
            // devcontainer-feature.json for option defaults before writing the
            // env file.
            let oci_ref = parse_oci_feature_ref(feature_ref).ok_or_else(|| {
                log::error!(
                    "Feature '{}' is not a supported OCI feature reference",
                    feature_ref
                );
                DevContainerErrorV2::UnmappedError
            })?;
            let token = get_oci_token_for_repo(&oci_ref.registry, &oci_ref.path, http_client)
                .await
                .map_err(|e| {
                    log::error!("Failed to get OCI token for feature '{}': {e}", feature_ref);
                    DevContainerErrorV2::UnmappedError
                })?;
            let manifest = fetch_oci_feature_manifest(&oci_ref, &token, http_client)
                .await
                .map_err(|e| {
                    log::error!(
                        "Failed to fetch OCI manifest for feature '{}': {e}",
                        feature_ref
                    );
                    DevContainerErrorV2::UnmappedError
                })?;
            let digest = &manifest
                .layers
                .first()
                .ok_or_else(|| {
                    log::error!(
                        "OCI manifest for feature '{}' contains no layers",
                        feature_ref
                    );
                    DevContainerErrorV2::UnmappedError
                })?
                .digest;
            let feature_manifest = download_and_extract_oci_feature(
                &oci_ref,
                digest,
                &token,
                &feature_dir,
                http_client,
            )
            .await
            .map_err(|e| {
                log::error!("Failed to download OCI feature '{}': {e}", feature_ref);
                DevContainerErrorV2::UnmappedError
            })?;

            log::info!("Downloaded OCI feature content for '{}'", feature_ref);

            let env_content = feature_manifest.write_feature_env(options)?;

            let wrapper_content = generate_install_wrapper(feature_ref, feature_id, &env_content);
            std::fs::write(
                feature_dir.join("devcontainer-features-install.sh"),
                &wrapper_content,
            )
            .map_err(|e| {
                log::error!("Failed to write install wrapper for {}: {e}", feature_ref);
                DevContainerErrorV2::UnmappedError
            })?;

            feature_layers.push_str(&generate_feature_layer(&consecutive_id));
            self.features.push(feature_manifest);
        }

        let dockerfile_base_content = self
            .dockerfile_location()
            .await
            .and_then(|path_buf| std::fs::read_to_string(path_buf).log_err());

        // --- 4. Generate and write Dockerfile.extended ---
        let dockerfile_content = generate_dockerfile_extended(
            &feature_layers,
            &container_user,
            &remote_user,
            dockerfile_base_content,
        );
        std::fs::write(&build_info.dockerfile_path, &dockerfile_content).map_err(|e| {
            log::error!("Failed to write Dockerfile.extended: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        log::info!(
            "Features build resources written to {:?}",
            build_info.features_content_dir
        );

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DevContainerBuildType {
    Image,
    Dockerfile,
    DockerCompose,
    None,
}

impl DevContainer {
    fn _validate_structure(&self) -> Result<(), DevContainerErrorV2> {
        // TODO
        Ok(())
    }
    fn _validate_features(&self) -> Result<(), DevContainerErrorV2> {
        // TODO
        Ok(())
    }

    fn build_type(&self) -> DevContainerBuildType {
        if self.image.is_some() {
            return DevContainerBuildType::Image;
        } else if self.docker_compose_file.is_some() {
            return DevContainerBuildType::DockerCompose;
        } else if self.build.is_some() {
            return DevContainerBuildType::Dockerfile;
        }
        return DevContainerBuildType::None;
    }

    fn has_features(&self) -> bool {
        self.features
            .as_ref()
            .map(|features| !features.is_empty())
            .unwrap_or(false)
    }
}

fn deserialize_string_or_array<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrArray {
        String(String),
        Array(Vec<String>),
    }

    match StringOrArray::deserialize(deserializer)? {
        StringOrArray::String(s) => Ok(Some(vec![s])),
        StringOrArray::Array(b) => Ok(Some(b)),
    }
}

fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(u32),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => Ok(Some(s)),
        StringOrInt::Int(b) => Ok(Some(b.to_string())),
    }
}

fn deserialize_mount_definitions<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<MountDefinition>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MountItem {
        Object(MountDefinition),
        String(String),
    }

    let items = Vec::<MountItem>::deserialize(deserializer)?;
    let mut mounts = Vec::new();

    for item in items {
        match item {
            MountItem::Object(mount) => mounts.push(mount),
            MountItem::String(s) => {
                let mut source = None;
                let mut target = None;
                let mut mount_type = None;

                for part in s.split(',') {
                    let part = part.trim();
                    if let Some((key, value)) = part.split_once('=') {
                        match key.trim() {
                            "source" => source = Some(value.trim().to_string()),
                            "target" => target = Some(value.trim().to_string()),
                            "type" => mount_type = Some(value.trim().to_string()),
                            _ => {} // Ignore unknown keys
                        }
                    }
                }

                let source = source.ok_or_else(|| {
                    D::Error::custom(format!("mount string missing 'source': {}", s))
                })?;
                let target = target.ok_or_else(|| {
                    D::Error::custom(format!("mount string missing 'target': {}", s))
                })?;

                mounts.push(MountDefinition {
                    source,
                    target,
                    mount_type,
                });
            }
        }
    }

    Ok(Some(mounts))
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MountDefinition {
    source: String,
    target: String,
    #[serde(rename = "type")]
    mount_type: Option<String>,
}

// TODO generalize this to include DevContainer predefined variables
// Probably just need to create a new crate for this
impl Display for MountDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type={},source={},target={},consistency=cached",
            self.mount_type.clone().unwrap_or_else(|| {
                if self.source.starts_with('/') {
                    "bind".to_string()
                } else {
                    "volume".to_string()
                }
            }),
            self.source,
            self.target
        )
    }
}

/// Represents the value associated with a feature ID in the `features` map of devcontainer.json.
///
/// Per the spec, the value can be:
/// - A boolean (`true` to enable with defaults)
/// - A string (shorthand for `{"version": "<value>"}`)
/// - An object mapping option names to string or boolean values
///
/// See: https://containers.dev/implementors/features/#devcontainerjson-properties
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub(crate) enum FeatureOptions {
    Bool(bool),
    String(String),
    Options(HashMap<String, FeatureOptionValue>),
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub(crate) enum FeatureOptionValue {
    Bool(bool),
    String(String),
}

impl std::fmt::Display for FeatureOptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureOptionValue::Bool(b) => write!(f, "{}", b),
            FeatureOptionValue::String(s) => write!(f, "{}", s),
        }
    }
}

/// Holds all the information needed to construct a `docker buildx build` command
/// that extends a base image with dev container features.
///
/// This mirrors the `ImageBuildOptions` interface in the CLI reference implementation
/// (cli/src/spec-node/containerFeatures.ts).
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct FeaturesBuildInfo {
    /// Path to the generated Dockerfile.extended
    pub dockerfile_path: PathBuf,
    /// Path to the features content directory (used as a BuildKit build context)
    pub features_content_dir: PathBuf,
    /// Path to an empty directory used as the Docker build context
    pub empty_context_dir: PathBuf,
    /// The base image name (e.g. "mcr.microsoft.com/devcontainers/rust:2-1-bookworm")
    pub build_image: Option<String>,
    /// The tag to apply to the built image (e.g. "vsc-myproject-features")
    pub image_tag: String,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HostRequirements {
    cpus: Option<u16>,
    memory: Option<String>,
    storage: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum LifecycleCommand {
    InitializeCommand,
    OnCreateCommand,
    UpdateContentCommand,
    PostCreateCommand,
    PostStartCommand,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
struct LifecycleScriptInternal {
    command: Option<String>,
    args: Vec<String>,
}

impl LifecycleScriptInternal {
    fn from_args(args: Vec<String>) -> Self {
        let command = args.get(0).map(|a| a.to_string());
        let remaining = args.iter().skip(1).map(|a| a.to_string()).collect();
        Self {
            command,
            args: remaining,
        }
    }
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub(crate) struct LifecyleScript {
    scripts: HashMap<String, LifecycleScriptInternal>,
}

impl LifecyleScript {
    fn from_map(args: HashMap<String, Vec<String>>) -> Self {
        Self {
            scripts: args
                .into_iter()
                .map(|(k, v)| (k, LifecycleScriptInternal::from_args(v)))
                .collect(),
        }
    }
    fn from_str(args: &str) -> Self {
        let script: Vec<String> = args.split(" ").map(|a| a.to_string()).collect();

        Self::from_args(script)
    }
    fn from_args(args: Vec<String>) -> Self {
        Self::from_map(HashMap::from([("default".to_string(), args)]))
    }
}

impl<'de> Deserialize<'de> for LifecyleScript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct LifecycleScriptVisitor;

        impl<'de> Visitor<'de> for LifecycleScriptVisitor {
            type Value = LifecyleScript;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string, an array of strings, or a map of arrays")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LifecyleScript::from_str(value))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut array = Vec::new();
                while let Some(elem) = seq.next_element()? {
                    array.push(elem);
                }
                Ok(LifecyleScript::from_args(array))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut result = HashMap::new();
                while let Some(key) = map.next_key::<String>()? {
                    let value: Value = map.next_value()?;
                    let script_args = match value {
                        Value::String(s) => {
                            s.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()
                        }
                        Value::Array(arr) => {
                            let strings: Vec<String> = arr
                                .into_iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect();
                            strings
                        }
                        _ => continue,
                    };
                    result.insert(key, script_args);
                }
                Ok(LifecyleScript::from_map(result))
            }
        }

        deserializer.deserialize_any(LifecycleScriptVisitor)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContainerBuild {
    dockerfile: String,
    context: Option<String>,
    args: Option<HashMap<String, String>>,
    options: Option<Vec<String>>,
    target: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_array")]
    cache_from: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ShutdownAction {
    None,
    StopContainer,
    StopCompose,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum UserEnvProbe {
    None,
    InteractiveShell,
    LoginShell,
    LoginInteractiveShell,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub(crate) enum ForwardPort {
    Number(u16),
    String(String),
}
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PortAttributeProtocol {
    Https,
    Http,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum OnAutoForward {
    Notify,
    OpenBrowser,
    OpenBrowserOnce,
    OpenPreview,
    Silent,
    Ignore,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PortAttributes {
    label: String,
    on_auto_forward: OnAutoForward,
    elevate_if_needed: bool,
    require_local_port: bool,
    protocol: PortAttributeProtocol,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
struct DockerComposeServiceBuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_contexts: Option<HashMap<String, String>>, // TODO you gotta address this when you reformat feature stuff
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
struct DockerComposeService {
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cap_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_opt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build: Option<DockerComposeServiceBuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged: Option<bool>,
    volumes: Vec<MountDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
struct DockerComposeVolume {
    name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct DockerComposeManifest {
    files: Vec<PathBuf>,
    config: DockerComposeConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
struct DockerComposeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    services: HashMap<String, DockerComposeService>,
    volumes: HashMap<String, DockerComposeVolume>,
}

pub(crate) fn read_devcontainer_configuration(
    config: DevContainerConfig,
    local_project_path: Arc<&Path>,
) -> Result<DevContainer, DevContainerErrorV2> {
    let config_path = local_project_path.join(config.config_path);

    let devcontainer_contents = std::fs::read_to_string(&config_path).map_err(|e| {
        log::error!("Unable to read devcontainer contents: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    deserialize_devcontainer_json(&devcontainer_contents)
}

/// New and improved flow should look like this:
/// 1. parse the devcontainer file
/// 2. ensure that object is valid
/// 3. check for disallowed features (?)
/// 4. run any initializeCommands
/// 5. If dockerfile or image config
///     1. check for existing container by params + id labels (pending rebuild)
///     2. If exists and running, return it
///     3. If exists and not running, start it
///     4. If not exists
///         1. Build it
///         2. Run the built thing you just made
/// 6. If docker-compose config
///     1. TODO - this is the next thing
pub(crate) async fn spawn_dev_container(
    http_client: Arc<dyn HttpClient>,
    config: DevContainerConfig,
    local_project_path: Arc<&Path>,
) -> Result<DevContainerUp, DevContainerErrorV2> {
    // 1. parse the devcontainer file
    let config_path = local_project_path.join(config.config_path.clone());

    let mut devcontainer_manifest = DevContainerManifest::new(&config_path)?;
    // 2. ensure that object is valid
    devcontainer_manifest.validate_config()?;

    // 4. run any initializeCommands
    log::info!("TODO, run initialze commands");

    let labels = vec![
        (
            "devcontainer.local_folder",
            (&local_project_path.display()).to_string(),
        ),
        (
            "devcontainer.config_file",
            (&devcontainer_manifest.config_file().display()).to_string(),
        ),
    ];

    log::info!("Checking for existing container");
    if let Some(docker_ps) = check_for_existing_container(&labels).await? {
        log::info!("Dev container already found. Proceeding with it");
        //     2. If exists and running, return it
        //
        let docker_inspect = inspect_image(&docker_ps.id).await?;
        //     3. If exists and not running, start it
        log::info!("TODO start the container if it's not running");

        let remote_user = get_remote_user_from_config(&docker_inspect, &devcontainer_manifest)?;

        let remote_folder = get_remote_dir_from_config(
            &docker_inspect,
            (&local_project_path.display()).to_string(),
        )?;

        Ok(DevContainerUp {
            _outcome: "todo".to_string(),
            container_id: docker_ps.id,
            remote_user: remote_user,
            remote_workspace_folder: remote_folder,
        })
    } else {
        log::info!("Existing container not found. Building");

        devcontainer_manifest.download_resources().await?;
        devcontainer_manifest
            .construct_build_resources(&http_client)
            .await?;

        // So eventually, build resources should belong to DevContainerManifest
        let build_resources =
            build_dev_container_resources(&devcontainer_manifest, &labels).await?;

        run_dev_container(
            build_resources,
            &devcontainer_manifest,
            &local_project_path,
            &labels,
        )
        .await
    }
}

#[derive(Debug)]
struct DockerBuildResources {
    image: DockerInspect,
    additional_mounts: Vec<MountDefinition>,
    privileged: bool,
    entrypoints: Vec<String>,
}

#[derive(Debug)]
enum DevContainerBuildResources {
    DockerComposeFiles(Vec<String>),
    Docker(DockerBuildResources),
}

async fn build_dev_container_resources(
    dev_container: &DevContainerManifest,
    labels: &Vec<(&str, String)>,
) -> Result<DevContainerBuildResources, DevContainerErrorV2> {
    match dev_container.config.build_type() {
        DevContainerBuildType::Image | DevContainerBuildType::Dockerfile => {
            let built_docker_image = build_docker_image(dev_container).await?;

            let resources = build_merged_resources(built_docker_image, dev_container);
            Ok(DevContainerBuildResources::Docker(resources))
        }
        DevContainerBuildType::DockerCompose => {
            log::info!("Using docker compose. Building extended compose files");
            let docker_compose_files =
                build_and_extend_compose_files(dev_container, &labels).await?;

            log::info!(
                "Created {} docker_compose files",
                &docker_compose_files.len()
            );
            return Ok(DevContainerBuildResources::DockerComposeFiles(
                docker_compose_files,
            ));
        }
        DevContainerBuildType::None => {
            return Err(DevContainerErrorV2::UnmappedError);
        }
    }
}

async fn run_dev_container(
    build_resources: DevContainerBuildResources,
    dev_container: &DevContainerManifest,
    local_project_path: &Arc<&Path>,
    labels: &Vec<(&str, String)>,
) -> Result<DevContainerUp, DevContainerErrorV2> {
    let running_container = match build_resources {
        DevContainerBuildResources::DockerComposeFiles(docker_compose_files) => {
            dbg!(&docker_compose_files);
            run_docker_compose(docker_compose_files, &labels).await?
        }
        DevContainerBuildResources::Docker(resources) => {
            dbg!(&resources);
            run_docker_image(resources, &labels, &local_project_path).await?
        }
    };

    dbg!(&running_container);
    let remote_user = get_remote_user_from_config(&running_container, dev_container)?;
    let remote_workspace_folder = get_remote_dir_from_config(
        &running_container,
        (&local_project_path.display()).to_string(),
    )?;

    Ok(DevContainerUp {
        _outcome: "todo".to_string(),
        container_id: running_container.id,
        remote_user,
        remote_workspace_folder,
    })
}

async fn run_docker_compose(
    docker_compose_files: Vec<String>,
    labels: &Vec<(&str, String)>,
) -> Result<DockerInspect, DevContainerErrorV2> {
    let mut command = Command::new(docker_cli());
    // TODO project name how
    command.args(&["compose", "--project-name", "rustwebstarter_devcontainer"]);
    for docker_compose_file in docker_compose_files {
        command.args(&["-f", &docker_compose_file]);
    }
    command.args(&["up", "-d"]);

    dbg!(&command);

    let output = command.output().await.map_err(|e| {
        log::error!("Error running docker compose up: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("Non-success status from docker compose up: {}", stderr);
        return Err(DevContainerErrorV2::UnmappedError);
    }

    if let Some(docker_ps) = check_for_existing_container(&labels).await? {
        log::info!("Found newly created dev container");
        return inspect_image(&docker_ps.id).await;
    }

    log::error!("Could not find existing container after docker compose up");

    Err(DevContainerErrorV2::UnmappedError)
}

async fn build_and_extend_compose_files(
    devcontainer: &DevContainerManifest,
    labels: &Vec<(&str, String)>,
) -> Result<Vec<String>, DevContainerErrorV2> {
    let mut docker_compose_manifest = docker_compose_manifest(devcontainer).await?;

    let (main_service_name, main_service) =
        find_primary_service(&docker_compose_manifest, devcontainer)?;
    let built_service_image = if let Some(image) = &main_service.image {
        if devcontainer
            .config
            .features
            .as_ref()
            .is_none_or(|features| features.is_empty())
        {
            inspect_image(image).await?
        } else {
            let build_override = DockerComposeConfig {
                name: None,
                services: HashMap::from([(
                    main_service_name.clone(),
                    DockerComposeService {
                        image: Some(devcontainer.build_image_tag()?),
                        entrypoint: None,
                        cap_add: None,
                        security_opt: None,
                        labels: None,
                        build: Some(DockerComposeServiceBuild {
                            context: Some(
                                devcontainer
                                    .build_empty_context_dir()?
                                    .display()
                                    .to_string(),
                            ),
                            dockerfile: Some(
                                devcontainer.build_dockerfile_path()?.display().to_string(),
                            ),
                            args: Some(HashMap::from([
                                ("BUILDKIT_INLINE_CACHE".to_string(), "1".to_string()),
                                ("_DEV_CONTAINERS_BASE_IMAGE".to_string(), image.clone()),
                                ("_DEV_CONTAINERS_IMAGE_USER".to_string(), "root".to_string()), // TODO this has to get wired up
                            ])),
                            additional_contexts: Some(HashMap::from([(
                                "dev_containers_feature_content_source".to_string(),
                                devcontainer
                                    .feature_resource_directory()?
                                    .display()
                                    .to_string(),
                            )])),
                        }),
                        volumes: Vec::new(),
                        ..Default::default()
                    },
                )]),
                volumes: HashMap::new(),
            };

            let temp_base = std::env::temp_dir().join("devcontainer-zed");
            let config_location = temp_base.join("docker_compose_build.json");

            let config_json = serde_json_lenient::to_string(&build_override).map_err(|e| {
                log::error!("Error serializing docker compose runtime override: {e}");
                DevContainerErrorV2::UnmappedError
            })?;

            std::fs::write(&config_location, config_json).map_err(|e| {
                log::error!("Error writing the runtime override file: {e}");
                DevContainerErrorV2::UnmappedError
            })?;

            docker_compose_manifest.files.push(config_location);

            run_docker_compose_build(&docker_compose_manifest).await?;

            inspect_image(&devcontainer.build_image_tag()?).await?
        }
    } else if main_service // TODO this has to be reversed, I think?
        .build
        .as_ref()
        .map(|b| b.dockerfile.as_ref())
        .is_some()
    {
        let build_override = DockerComposeConfig {
            name: None,
            services: HashMap::from([(
                main_service_name.clone(),
                DockerComposeService {
                    image: Some(devcontainer.build_image_tag()?.clone()),
                    entrypoint: None,
                    cap_add: None,
                    security_opt: None,
                    labels: None,
                    build: Some(DockerComposeServiceBuild {
                        context: Some(
                            devcontainer
                                .build_empty_context_dir()?
                                .display()
                                .to_string(),
                        ),
                        dockerfile: Some(
                            devcontainer.build_dockerfile_path()?.display().to_string(),
                        ),
                        args: Some(HashMap::from([
                            ("BUILDKIT_INLINE_CACHE".to_string(), "1".to_string()),
                            (
                                "_DEV_CONTAINERS_BASE_IMAGE".to_string(),
                                "dev_container_auto_added_stage_label".to_string(),
                            ), // TODO Well this has gotta be cleaner
                            ("_DEV_CONTAINERS_IMAGE_USER".to_string(), "root".to_string()), // TODO this has to get wired up
                        ])),
                        additional_contexts: Some(HashMap::from([(
                            "dev_containers_feature_content_source".to_string(),
                            devcontainer
                                .feature_resource_directory()?
                                .display()
                                .to_string(),
                        )])),
                    }),
                    volumes: Vec::new(),
                    ..Default::default()
                },
            )]),
            volumes: HashMap::new(),
        };

        let temp_base = std::env::temp_dir().join("devcontainer-zed");
        let config_location = temp_base.join("docker_compose_build.json");

        let config_json = serde_json_lenient::to_string(&build_override).map_err(|e| {
            log::error!("Error serializing docker compose runtime override: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        std::fs::write(&config_location, config_json).map_err(|e| {
            log::error!("Error writing the runtime override file: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        docker_compose_manifest.files.push(config_location);

        run_docker_compose_build(&docker_compose_manifest).await?;

        inspect_image(&devcontainer.build_image_tag()?).await?
    } else {
        log::error!("Docker compose must have either image or dockerfile defined");
        return Err(DevContainerErrorV2::UnmappedError);
    };

    let resources = build_merged_resources(built_service_image, devcontainer);

    let runtime_override_file = build_runtime_override_file(&main_service_name, resources, labels)?;

    dbg!(&runtime_override_file);

    docker_compose_manifest.files.push(runtime_override_file);

    // TODO complete this refactor
    Ok(docker_compose_manifest
        .files
        .iter()
        .map(|file| file.display().to_string())
        .collect())
}

async fn docker_compose_manifest(
    devcontainer: &DevContainerManifest,
) -> Result<DockerComposeManifest, DevContainerErrorV2> {
    let Some(docker_compose_files) = devcontainer.config.docker_compose_file.clone() else {
        return Err(DevContainerErrorV2::UnmappedError);
    };
    let docker_compose_full_paths = docker_compose_files
        .iter()
        .map(|relative| devcontainer.directory.join(relative))
        .collect::<Vec<PathBuf>>();
    let docker_compose_config_command =
        create_docker_compose_config_command(&docker_compose_full_paths)?;

    let Some(config) =
        evaluate_json_command::<DockerComposeConfig>(docker_compose_config_command).await?
    else {
        log::error!("Output could not deserialize into DockerComposeConfig");
        return Err(DevContainerErrorV2::UnmappedError);
    };
    Ok(DockerComposeManifest {
        files: docker_compose_full_paths,
        config,
    })
}

// TODO move entrypoint into this
// TODO move this into the DevContainerManifest struct
fn build_merged_resources(
    base_image: DockerInspect,
    dev_container: &DevContainerManifest,
) -> DockerBuildResources {
    let mut mounts = dev_container.config.mounts.clone().unwrap_or(Vec::new());

    let mut feature_mounts = dev_container
        .features
        .iter()
        .flat_map(|f| f.mounts().clone())
        .collect();

    mounts.append(&mut feature_mounts);

    let privileged = dev_container.config.privileged.unwrap_or(false)
        || dev_container.features.iter().any(|f| f.privileged());

    let entrypoints = dev_container
        .features
        .iter()
        .filter_map(|f| f.entrypoint())
        .collect();

    DockerBuildResources {
        image: base_image,
        additional_mounts: mounts,
        privileged,
        entrypoints,
    }
}

async fn run_docker_compose_build(
    docker_compose: &DockerComposeManifest,
) -> Result<(), DevContainerErrorV2> {
    let mut command = Command::new(docker_cli());
    // TODO project name how
    command.args(&["compose", "--project-name", "rustwebstarter_devcontainer"]);
    for docker_compose_file in &docker_compose.files {
        command.args(&["-f", &docker_compose_file.display().to_string()]);
    }
    command.arg("build");

    dbg!(&command);

    let output = command.output().await.map_err(|e| {
        log::error!("Error running docker compose up: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("Non-success status from docker compose up: {}", stderr);
        return Err(DevContainerErrorV2::UnmappedError);
    }

    Ok(())
}

// TODO the mounts have to make it here
fn build_runtime_override_file(
    main_service_name: &str,
    resources: DockerBuildResources,
    // docker_image: &DockerInspect,
    labels: &Vec<(&str, String)>,
) -> Result<PathBuf, DevContainerErrorV2> {
    let config = build_runtime_override(main_service_name, resources, labels)?;
    let temp_base = std::env::temp_dir().join("devcontainer-zed");
    let config_location = temp_base.join("docker_compose_runtime.json");

    let config_json = serde_json_lenient::to_string(&config).map_err(|e| {
        log::error!("Error serializing docker compose runtime override: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    std::fs::write(&config_location, config_json).map_err(|e| {
        log::error!("Error writing the runtime override file: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    Ok(config_location)
}

fn build_runtime_override(
    main_service_name: &str,
    resources: DockerBuildResources,
    labels: &Vec<(&str, String)>,
) -> Result<DockerComposeConfig, DevContainerErrorV2> {
    let mut runtime_labels = vec![];

    if let Some(metadata) = &resources.image.config.labels.metadata {
        let serialized_metadata = serde_json_lenient::to_string(metadata).map_err(|e| {
            log::error!("Error serializing docker image metadata: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        runtime_labels.push(format!(
            "{}={}",
            "devcontainer.metadata", serialized_metadata
        ));
    }

    for (k, v) in labels {
        runtime_labels.push(format!("{}={}", k, v));
    }

    let config_volumes: HashMap<String, DockerComposeVolume> = resources
        .additional_mounts
        .iter()
        .filter_map(|mount| {
            if let Some(mount_type) = &mount.mount_type
                && mount_type.to_lowercase() == "volume"
            {
                Some((
                    mount
                        .source
                        .clone()
                        .replace("${devcontainerId}", "devcontainer123"),
                    DockerComposeVolume {
                        name: mount
                            .source
                            .clone()
                            .replace("${devcontainerId}", "devcontainer123"),
                    },
                ))
            } else {
                None
            }
        })
        .collect();
    // TODO Probably worth its own method
    let mut entrypoint_script_lines = vec![
        "echo Container started".to_string(),
        "trap \"exit 0\" 15".to_string(),
    ];
    for entrypoint in resources.entrypoints {
        entrypoint_script_lines.push(entrypoint.clone());
    }
    entrypoint_script_lines.append(&mut vec![
        "exec \"$@\"".to_string(),
        "while sleep 1 & wait $!; do :; done".to_string(),
    ]);

    let volumes: Vec<MountDefinition> = resources
        .additional_mounts
        .iter()
        .map(|v| MountDefinition {
            source: v
                .source
                .clone()
                .replace("${devcontainerId}", "devcontainer123"),
            target: v
                .target
                .clone()
                .replace("${devcontainerId}", "devcontainer123"),
            mount_type: v.mount_type.clone(),
        })
        .collect();

    let new_docker_compose_config = DockerComposeConfig {
        name: None,
        services: HashMap::from([(
            main_service_name.to_string(),
            DockerComposeService {
                entrypoint: Some(vec![
                    "/bin/sh".to_string(),
                    "-c".to_string(),
                    entrypoint_script_lines.join("\n").trim().to_string(),
                    "-".to_string(),
                ]),
                cap_add: Some(vec!["SYS_PTRACE".to_string()]),
                security_opt: Some(vec!["seccomp=unconfined".to_string()]),
                labels: Some(runtime_labels),
                volumes,
                privileged: Some(resources.privileged),
                ..Default::default()
            },
        )]),
        volumes: config_volumes,
        ..Default::default()
    };

    Ok(new_docker_compose_config)
}

fn find_primary_service(
    docker_compose: &DockerComposeManifest,
    devcontainer: &DevContainerManifest,
) -> Result<(String, DockerComposeService), DevContainerErrorV2> {
    let Some(service_name) = &devcontainer.config.service else {
        return Err(DevContainerErrorV2::UnmappedError);
    };

    match docker_compose.config.services.get(service_name) {
        Some(service) => Ok((service_name.clone(), service.clone())),
        None => Err(DevContainerErrorV2::UnmappedError),
    }
}

fn create_docker_compose_config_command(
    docker_compose_full_paths: &Vec<PathBuf>,
) -> Result<Command, DevContainerErrorV2> {
    let mut command = smol::process::Command::new(docker_cli());
    command.arg("compose");
    for file_path in docker_compose_full_paths {
        command.args(&["-f", &file_path.display().to_string()]);
    }
    command.args(&["config", "--format", "json"]);
    Ok(command)
}

async fn run_docker_image(
    build_resources: DockerBuildResources,
    labels: &Vec<(&str, String)>,
    local_project_path: &Arc<&Path>,
) -> Result<DockerInspect, DevContainerErrorV2> {
    let mut docker_run_command =
        create_docker_run_command(build_resources, local_project_path, Some(labels))?;

    let output = docker_run_command.output().await.map_err(|e| {
        log::error!("Error running docker run: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    if !output.status.success() {
        let std_err = String::from_utf8_lossy(&output.stderr);
        log::error!("Non-success status from docker run. StdErr: {std_err}");
        return Err(DevContainerErrorV2::UnmappedError);
    }

    log::info!("Checking for container that was started");
    let Some(docker_ps) = check_for_existing_container(labels).await? else {
        log::error!("Could not locate container just created");
        return Err(DevContainerErrorV2::UnmappedError);
    };
    inspect_image(&docker_ps.id).await
}

/// Destination folder inside the container where feature content is staged during build.
/// Mirrors the CLI's `FEATURES_CONTAINER_TEMP_DEST_FOLDER`.
// TODO does this need to be more generalized
const FEATURES_CONTAINER_TEMP_DEST_FOLDER: &str = "/tmp/dev-container-features";

/// Escapes single quotes for use inside shell single-quoted strings.
///
/// Ends the current single-quoted string, inserts an escaped single quote,
/// and reopens the string: `'` → `'\''`.
fn escape_single_quotes(input: &str) -> String {
    input.replace('\'', "'\\''")
}

/// Escapes regex special characters in a string.
fn escape_regex_chars(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        if ".*+?^${}()|[]\\".contains(c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

/// Extracts the short feature ID from a full feature reference string.
///
/// Examples:
/// - `ghcr.io/devcontainers/features/aws-cli:1` → `aws-cli`
/// - `ghcr.io/user/repo/go` → `go`
/// - `ghcr.io/devcontainers/features/rust@sha256:abc` → `rust`
/// - `./myFeature` → `myFeature`
fn extract_feature_id(feature_ref: &str) -> &str {
    let without_version = if let Some(at_idx) = feature_ref.rfind('@') {
        &feature_ref[..at_idx]
    } else {
        let last_slash = feature_ref.rfind('/');
        let last_colon = feature_ref.rfind(':');
        match (last_slash, last_colon) {
            (Some(slash), Some(colon)) if colon > slash => &feature_ref[..colon],
            _ => feature_ref,
        }
    };
    match without_version.rfind('/') {
        Some(idx) => &without_version[idx + 1..],
        None => without_version,
    }
}

/// Generates a shell command that looks up a user's passwd entry.
///
/// Mirrors the CLI's `getEntPasswdShellCommand` in `commonUtils.ts`.
/// Tries `getent passwd` first, then falls back to grepping `/etc/passwd`.
// TODO fairly sure this exists elsewhere, we should deduplicate
fn get_ent_passwd_shell_command(user: &str) -> String {
    let escaped_for_shell = user.replace('\\', "\\\\").replace('\'', "\\'");
    let escaped_for_regex = escape_regex_chars(user).replace('\'', "\\'");
    format!(
        " (command -v getent >/dev/null 2>&1 && getent passwd '{shell}' || grep -E '^{re}|^[^:]*:[^:]*:{re}:' /etc/passwd || true)",
        shell = escaped_for_shell,
        re = escaped_for_regex,
    )
}

/// Determines feature installation order, respecting `overrideFeatureInstallOrder`.
///
/// Features listed in the override come first (in the specified order), followed
/// by any remaining features sorted lexicographically by their full reference ID.
fn resolve_feature_order<'a>(
    features: &'a HashMap<String, FeatureOptions>,
    override_order: &Option<Vec<String>>,
) -> Vec<(&'a String, &'a FeatureOptions)> {
    if let Some(order) = override_order {
        let mut ordered: Vec<(&'a String, &'a FeatureOptions)> = Vec::new();
        for ordered_id in order {
            if let Some((key, options)) = features.get_key_value(ordered_id) {
                ordered.push((key, options));
            }
        }
        let mut remaining: Vec<_> = features
            .iter()
            .filter(|(id, _)| !order.iter().any(|o| o == *id))
            .collect();
        remaining.sort_by_key(|(id, _)| id.as_str());
        ordered.extend(remaining);
        ordered
    } else {
        let mut entries: Vec<_> = features.iter().collect();
        entries.sort_by_key(|(id, _)| id.as_str());
        entries
    }
}

/// Generates the `devcontainer-features-install.sh` wrapper script for one feature.
///
/// Mirrors the CLI's `getFeatureInstallWrapperScript` in
/// `containerFeaturesConfiguration.ts`.
fn generate_install_wrapper(feature_ref: &str, feature_id: &str, env_variables: &str) -> String {
    let escaped_id = escape_single_quotes(feature_ref);
    let escaped_name = escape_single_quotes(feature_id);
    let options_indented: String = env_variables
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| format!("    {}", l))
        .collect::<Vec<_>>()
        .join("\n");
    let escaped_options = escape_single_quotes(&options_indented);

    let mut script = String::new();
    script.push_str("#!/bin/sh\n");
    script.push_str("set -e\n");
    script.push_str("\n");
    script.push_str("on_exit () {\n");
    script.push_str("    [ $? -eq 0 ] && exit\n");
    script.push_str("    echo 'ERROR: Feature \"");
    script.push_str(&escaped_name);
    script.push_str("\" (");
    script.push_str(&escaped_id);
    script.push_str(") failed to install!'\n");
    script.push_str("}\n");
    script.push_str("\n");
    script.push_str("trap on_exit EXIT\n");
    script.push_str("\n");
    script.push_str(
        "echo ===========================================================================\n",
    );
    script.push_str("echo 'Feature       : ");
    script.push_str(&escaped_name);
    script.push_str("'\n");
    script.push_str("echo 'Id            : ");
    script.push_str(&escaped_id);
    script.push_str("'\n");
    script.push_str("echo 'Options       :'\n");
    script.push_str("echo '");
    script.push_str(&escaped_options);
    script.push_str("'\n");
    script.push_str(
        "echo ===========================================================================\n",
    );
    script.push_str("\n");
    script.push_str("set -a\n");
    script.push_str(". ../devcontainer-features.builtin.env\n");
    script.push_str(". ./devcontainer-features.env\n");
    script.push_str("set +a\n");
    script.push_str("\n");
    script.push_str("chmod +x ./install.sh\n");
    script.push_str("./install.sh\n");
    script
}

/// Generates a single Dockerfile `RUN` instruction that installs one feature
/// using a BuildKit bind mount.
///
/// Mirrors the v2 BuildKit branch of `getFeatureLayers` in
/// `containerFeaturesConfiguration.ts`.
fn generate_feature_layer(consecutive_id: &str) -> String {
    format!(
        r#"
RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./{id},target=/tmp/build-features-src/{id} \
    cp -ar /tmp/build-features-src/{id} {dest} \
 && chmod -R 0755 {dest}/{id} \
 && cd {dest}/{id} \
 && chmod +x ./devcontainer-features-install.sh \
 && ./devcontainer-features-install.sh \
 && rm -rf {dest}/{id}
"#,
        id = consecutive_id,
        dest = FEATURES_CONTAINER_TEMP_DEST_FOLDER,
    )
}

// Dockerfile actions need to be moved to their own file
fn dockerfile_alias(dockerfile_content: &str) -> Option<String> {
    dockerfile_content
        .lines()
        .find(|line| line.starts_with("FROM"))
        .and_then(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            if words.len() > 2 && words[words.len() - 2].to_lowercase() == "as" {
                return Some(words[words.len() - 1].to_string());
            } else {
                return None;
            }
        })
}

fn dockerfile_inject_alias(dockerfile_content: &str, alias: &str) -> String {
    if dockerfile_alias(dockerfile_content).is_some() {
        dockerfile_content.to_string()
    } else {
        dockerfile_content
            .lines()
            .map(|line| {
                if line.starts_with("FROM") {
                    format!("{} AS {}", line, alias)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

//////////////////////////////

/// Generates the full `Dockerfile.extended` content that extends a base image
/// with dev container features.
///
/// Mirrors the CLI's `getContainerFeaturesBaseDockerFile` combined with
/// `getFeatureLayers` (both in `containerFeaturesConfiguration.ts`), using
/// the BuildKit path (named build contexts, `--mount` bind mounts).
fn generate_dockerfile_extended(
    feature_layers: &str,
    container_user: &str,
    remote_user: &str,
    // TODO: use this to optionally include in the template
    // TODO also looks like this needs a test
    // From here, you really just need to change the docker build args to include any args from the build object, and point at the .devcontainer folder instead of the empty dir
    dockerfile_content: Option<String>,
) -> String {
    let container_home_cmd = get_ent_passwd_shell_command(container_user);
    let remote_home_cmd = get_ent_passwd_shell_command(remote_user);
    // So what happens is the reference implementation parses this content and aliases the "FROM" statement to `dev_container_auto_added_stage_label`, then using that as the _DEV_CONTAINERS_BASE_IMAGE arg
    // This is going to require actually parsing Dockerfile. Which means I probably need a docker crate. This is the worst.
    let dockerfile_content = dockerfile_content
        .map(|content| {
            if dockerfile_alias(&content).is_some() {
                content
            } else {
                dockerfile_inject_alias(&content, "dev_container_auto_added_stage_label")
            }
        })
        .unwrap_or("".to_string());

    dbg!(&dockerfile_content);

    let dest = FEATURES_CONTAINER_TEMP_DEST_FOLDER;

    format!(
        r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

{dockerfile_content}

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source ./devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p {dest}
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ {dest}

RUN \
echo "_CONTAINER_USER_HOME=$({container_home_cmd} | cut -d: -f6)" >> {dest}/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$({remote_home_cmd} | cut -d: -f6)" >> {dest}/devcontainer-features.builtin.env

{feature_layers}

ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER
"#
    )
}

async fn build_docker_image(
    dev_container: &DevContainerManifest,
) -> Result<DockerInspect, DevContainerErrorV2> {
    match dev_container.config.build_type() {
        DevContainerBuildType::Image => {
            let Some(image_tag) = &dev_container.config.image else {
                return Err(DevContainerErrorV2::UnmappedError);
            };
            let base_image = inspect_image(image_tag).await?;
            if dev_container
                .config
                .features
                .as_ref()
                .is_none_or(|features| features.is_empty())
            {
                log::info!("No features to add. Using base image");
                return Ok(base_image.clone());
            }
        }
        DevContainerBuildType::Dockerfile => {}
        DevContainerBuildType::DockerCompose => todo!("not yet implemented"),
        DevContainerBuildType::None => {
            return Err(DevContainerErrorV2::UnmappedError);
        }
    };

    let mut command = create_docker_build(dev_container)?;

    let output = command.output().await.map_err(|e| {
        log::error!("Error building docker image: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("docker buildx build failed: {stderr}");
        return Err(DevContainerErrorV2::UnmappedError);
    }

    // After a successful build, inspect the newly tagged image to get its metadata
    let image = inspect_image(&dev_container.build_image_tag()?).await?;

    Ok(image)
}

async fn check_for_existing_container(
    labels: &Vec<(&str, String)>,
) -> Result<Option<DockerPs>, DevContainerErrorV2> {
    let command = create_docker_query_containers(Some(labels))?;
    evaluate_json_command(command).await
}

fn deserialize_devcontainer_json(json: &str) -> Result<DevContainer, DevContainerErrorV2> {
    match serde_json_lenient::from_str(json) {
        Ok(devcontainer) => Ok(devcontainer),
        Err(e) => {
            dbg!(&e);
            Err(DevContainerErrorV2::UnmappedError)
        }
    }
}

/// Constructs a `docker buildx build` command that extends a base image with dev container
/// features, matching the behavior of the CLI reference implementation's `extendImage` function
/// in `cli/src/spec-node/containerFeatures.ts`.
///
/// The resulting command looks like:
/// ```text
/// docker buildx build --load \
///   --build-context dev_containers_feature_content_source=<features_content_dir> \
///   --build-arg _DEV_CONTAINERS_BASE_IMAGE=<base_image> \
///   --build-arg _DEV_CONTAINERS_IMAGE_USER=<image_user> \
///   --build-arg _DEV_CONTAINERS_FEATURE_CONTENT_SOURCE=dev_container_feature_content_temp \
///   --target dev_containers_target_stage \
///   -f <dockerfile_path> \
///   -t <image_tag> \
///   <empty_context_dir>
/// ```
fn create_docker_build(
    dev_container: &DevContainerManifest,
) -> Result<Command, DevContainerErrorV2> {
    let mut command = smol::process::Command::new(docker_cli());

    command.args(["buildx", "build"]);

    // --load is short for --output=docker, loading the built image into the local docker images
    command.arg("--load");

    // BuildKit build context: provides the features content directory as a named context
    // that the Dockerfile.extended can COPY from via `--from=dev_containers_feature_content_source`
    command.args([
        "--build-context",
        &format!(
            "dev_containers_feature_content_source={}",
            dev_container.feature_resource_directory()?.display()
        ),
    ]);

    // Build args matching the CLI reference implementation's `getFeaturesBuildOptions`
    if let Some(build_image) = dev_container.build_info_build_image() {
        command.args([
            "--build-arg",
            &format!("_DEV_CONTAINERS_BASE_IMAGE={}", build_image),
        ]);
    } else {
        command.args([
            "--build-arg",
            "_DEV_CONTAINERS_BASE_IMAGE=dev_container_auto_added_stage_label",
        ]);
    }

    command.args([
        "--build-arg",
        &format!(
            "_DEV_CONTAINERS_IMAGE_USER={}",
            dev_container
                .base_image()
                .and_then(|docker_image| docker_image.config.image_user)
                .unwrap_or("root".to_string())
        ),
    ]);

    // TODO if featuers exist, add this
    command.args([
        "--build-arg",
        "_DEV_CONTAINERS_FEATURE_CONTENT_SOURCE=dev_container_feature_content_temp",
    ]);

    if let Some(args) = dev_container
        .config
        .build
        .as_ref()
        .and_then(|b| b.args.as_ref())
    {
        for (key, value) in args {
            command.args(["--build-arg", &format!("{}={}", key, value)]);
        }
    }

    // Target the final stage in the generated Dockerfile
    command.args(["--target", "dev_containers_target_stage"]);

    // Point to the generated extended Dockerfile
    command.args([
        "-f",
        &dev_container.build_dockerfile_path()?.display().to_string(),
    ]);

    // Tag the resulting image
    command.args(["-t", &dev_container.build_image_tag()?]);

    if dev_container.config.build_type() == DevContainerBuildType::Dockerfile {
        command.arg(dev_container.directory.display().to_string());
    } else {
        // Use an empty folder as the build context to avoid pulling in unneeded files.
        // The actual feature content is supplied via the BuildKit build context above.
        command.arg(
            dev_container
                .build_empty_context_dir()?
                .display()
                .to_string(),
        );
    }

    dbg!(&command);

    Ok(command)
}

fn create_docker_query_containers(
    filter_labels: Option<&Vec<(&str, String)>>,
) -> Result<Command, DevContainerErrorV2> {
    let mut command = smol::process::Command::new(docker_cli());
    command.args(&["ps", "-a"]);

    if let Some(labels) = filter_labels {
        for (key, value) in labels {
            command.arg("--filter");
            command.arg(format!("label={key}={value}"));
        }
    }
    command.arg("--format=json");
    Ok(command)
}

fn create_docker_run_command(
    build_resources: DockerBuildResources,
    local_project_directory: &Arc<&Path>,
    labels: Option<&Vec<(&str, String)>>,
) -> Result<Command, DevContainerErrorV2> {
    let Some(project_directory) = local_project_directory.file_name() else {
        return Err(DevContainerErrorV2::UnmappedError);
    };
    let remote_workspace_folder = format!("/workspaces/{}", project_directory.display()); // TODO workspaces should be overridable

    let mut command = Command::new(docker_cli());

    command.arg("run");

    if build_resources.privileged {
        command.arg("--privileged");
    }

    command.arg("--sig-proxy=false");
    command.arg("-d");
    command.arg("--mount");
    command.arg(format!(
        "type=bind,source={},target={},consistency=cached",
        local_project_directory.display(),
        remote_workspace_folder
    ));

    for mount in &build_resources.additional_mounts {
        command.arg("--mount");
        command.arg(
            mount
                .to_string()
                .replace("${devcontainerId}", "devcontainer123"), // TODO So this is what we're doing tomorrow
        );
    }

    if let Some(labels) = labels {
        for (key, val) in labels {
            command.arg("-l");
            command.arg(format!("{}={}", key, val));
        }
    }

    if let Some(metadata) = &build_resources.image.config.labels.metadata {
        let serialized_metadata = serde_json_lenient::to_string(metadata).map_err(|e| {
            log::error!("Problem serializing image metadata: {e}");
            DevContainerErrorV2::UnmappedError
        })?;
        command.arg("-l");
        command.arg(format!(
            "{}={}",
            "devcontainer.metadata", serialized_metadata
        ));
    }

    command.arg("--entrypoint");
    command.arg("/bin/sh");
    command.arg(&build_resources.image.id);
    command.arg("-c");
    // TODO Probably worth its own method
    let mut entrypoint_script_lines = vec![
        "echo Container started".to_string(),
        "trap \"exit 0\" 15".to_string(),
    ];
    for entrypoint in build_resources.entrypoints {
        entrypoint_script_lines.push(entrypoint.clone());
    }
    entrypoint_script_lines.append(&mut vec![
        "exec \"$@\"".to_string(),
        "while sleep 1 & wait $!; do :; done".to_string(),
    ]);

    command.arg(entrypoint_script_lines.join("\n").trim());
    command.arg("-");

    Ok(command)
}

// Ok this needs some work, because we should be able to find a mount that works as an ancestor to this remote dir
// E.g. it might be /Source/myproject:workspaces/myproject
// But it might also be Source/:workspaces/
// In the latter case, we want to found that mount destination (e.g. workspaces/), and fill in the rest of the path to the workspace (so that it's workspaces/myproject)
fn get_remote_dir_from_config(
    config: &DockerInspect,
    local_dir: String,
) -> Result<String, DevContainerErrorV2> {
    let local_path = PathBuf::from(&local_dir);

    let Some(mounts) = &config.mounts else {
        log::error!("No mounts");
        return Err(DevContainerErrorV2::UnmappedError);
    };

    for mount in mounts {
        dbg!(&mount);
        // Sometimes docker will mount the local filesystem on host_mnt for system isolation
        let mount_source = PathBuf::from(&mount.source.trim_start_matches("/host_mnt"));
        // if mount source is an ancestor of local_path
        if let Ok(relative_path_to_project) = local_path.strip_prefix(&mount_source) {
            let remote_dir = format!(
                "{}/{}",
                &mount.destination,
                relative_path_to_project.display()
            );
            return Ok(remote_dir);
        }
        if mount.source == local_dir {
            return Ok(mount.destination.clone());
        }
    }
    log::error!("No mounts to local folder");
    Err(DevContainerErrorV2::UnmappedError)
}

// TODO to this whole section - might be a good thing to tackle while wally is asleep

/// Gets the base image from the devcontainer with the following precedence:
/// - The devcontainer image if an image is specified
/// - The image sourced in the Dockerfile if a Dockerfile is specified
/// - The image sourced in the docker-compose main service, if one is specified
/// - The image sourced in the docker-compose main service dockerfile, if one is specified
/// If no such image is available, return an error
async fn get_base_image_from_config(
    devcontainer: &DevContainerManifest,
) -> Result<String, DevContainerErrorV2> {
    if let Some(image) = &devcontainer.config.image {
        return Ok(image.to_string());
    }
    if let Some(dockerfile) = devcontainer.config.build.as_ref().map(|b| &b.dockerfile) {
        let dockerfile_contents = std::fs::read_to_string(devcontainer.directory.join(dockerfile))
            .map_err(|e| {
                log::error!("Error reading dockerfile: {e}");
                DevContainerErrorV2::UnmappedError
            })?;
        return image_from_dockerfile(devcontainer, dockerfile_contents);
    }
    if devcontainer.config.docker_compose_file.is_some() {
        let docker_compose_manifest = docker_compose_manifest(devcontainer).await?;
        let (_, main_service) = find_primary_service(&docker_compose_manifest, &devcontainer)?;

        if let Some(dockerfile) = main_service
            .build
            .as_ref()
            .and_then(|b| b.dockerfile.as_ref())
        {
            let dockerfile_contents =
                std::fs::read_to_string(devcontainer.directory.join(dockerfile)).map_err(|e| {
                    log::error!("Error reading dockerfile: {e}");
                    DevContainerErrorV2::UnmappedError
                })?;
            return image_from_dockerfile(devcontainer, dockerfile_contents);
        }
        if let Some(image) = &main_service.image {
            return Ok(image.to_string());
        }

        log::error!("No valid base image found in docker-compose configuration");
        return Err(DevContainerErrorV2::UnmappedError);
    }
    log::error!("No valid base image found in dev container configuration");
    Err(DevContainerErrorV2::UnmappedError)
}

/// TODO test
fn image_from_dockerfile(
    devcontainer: &DevContainerManifest,
    dockerfile_contents: String,
) -> Result<String, DevContainerErrorV2> {
    let mut raw_contents = dockerfile_contents
        .lines()
        .find(|line| line.starts_with("FROM"))
        .and_then(|from_line| {
            from_line
                .split(' ')
                .collect::<Vec<&str>>()
                .get(1)
                .map(|s| s.to_string())
        })
        .ok_or_else(|| {
            log::error!("Could not find an image definition in dockerfile");
            DevContainerErrorV2::UnmappedError
        })?;

    for (k, v) in devcontainer
        .config
        .build
        .as_ref()
        .and_then(|b| b.args.as_ref())
        .unwrap_or(&HashMap::new())
    {
        raw_contents = raw_contents.replace(&format!("${{{}}}", k), v);
    }
    Ok(raw_contents)
}

// Container user things
// This should come from spec - see the docs
fn get_remote_user_from_config(
    docker_config: &DockerInspect,
    devcontainer: &DevContainerManifest,
) -> Result<String, DevContainerErrorV2> {
    if let DevContainer {
        remote_user: Some(user),
        ..
    } = &devcontainer.config
    {
        return Ok(user.clone());
    }
    let Some(metadata) = &docker_config.config.labels.metadata else {
        log::error!("Could not locate metadata");
        return Err(DevContainerErrorV2::UnmappedError);
    };
    for metadatum in metadata {
        if let Some(remote_user) = metadatum.get("remoteUser") {
            if let Some(remote_user_str) = remote_user.as_str() {
                return Ok(remote_user_str.to_string());
            }
        }
    }
    Err(DevContainerErrorV2::UnmappedError)
}

// This should come from spec - see the docs
fn get_container_user_from_config(
    docker_config: &DockerInspect,
    devcontainer: &DevContainerManifest,
) -> Result<String, DevContainerErrorV2> {
    if let Some(user) = &devcontainer.config.container_user {
        return Ok(user.to_string());
    }
    if let Some(metadata) = &docker_config.config.labels.metadata {
        for metadatum in metadata {
            if let Some(container_user) = metadatum.get("containerUser") {
                if let Some(container_user_str) = container_user.as_str() {
                    return Ok(container_user_str.to_string());
                }
            }
        }
    }
    if let Some(image_user) = &docker_config.config.image_user {
        return Ok(image_user.to_string());
    }

    Err(DevContainerErrorV2::UnmappedError)
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        ffi::OsStr,
        path::{Path, PathBuf},
        process::{ExitStatus, Output},
        sync::Arc,
    };

    use http_client::{FakeHttpClient, HttpClient};

    use crate::{
        DevContainerErrorV2,
        command_json::deserialize_json_output,
        docker::{DockerConfigLabels, DockerInspectConfig, docker_cli},
        features::{DevContainerFeatureJson, FeatureManifest},
        model::{
            ContainerBuild, DevContainer, DevContainerBuildType, DevContainerManifest,
            DockerBuildResources, DockerComposeConfig, DockerComposeManifest, DockerComposeService,
            DockerInspect, DockerPs, FeatureOptions, ForwardPort, HostRequirements,
            LifecycleCommand, LifecyleScript, MountDefinition, OnAutoForward,
            PortAttributeProtocol, PortAttributes, ShutdownAction, UserEnvProbe,
            build_runtime_override, create_docker_build, create_docker_compose_config_command,
            create_docker_run_command, deserialize_devcontainer_json, extract_feature_id,
            find_primary_service, generate_dockerfile_extended, get_remote_dir_from_config,
            get_remote_user_from_config,
        },
        safe_id_upper,
    };

    fn _fake_http_client() -> Arc<dyn HttpClient> {
        FakeHttpClient::create(|_| async move {
            Ok(http::Response::builder()
                .status(404)
                .body(http_client::AsyncBody::default())
                .unwrap())
        })
    }

    fn _build_feature_tarball(install_sh_content: &str) -> Vec<u8> {
        smol::block_on(async {
            let buffer = futures::io::Cursor::new(Vec::new());
            let mut builder = async_tar::Builder::new(buffer);

            let data = install_sh_content.as_bytes();
            let mut header = async_tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(0o755);
            header.set_entry_type(async_tar::EntryType::Regular);
            header.set_cksum();
            builder
                .append_data(&mut header, "install.sh", data)
                .await
                .unwrap();

            let buffer = builder.into_inner().await.unwrap();
            buffer.into_inner()
        })
    }

    fn _fake_oci_http_client() -> Arc<dyn HttpClient> {
        let tarball = Arc::new(_build_feature_tarball(
            "#!/bin/sh\nset -e\necho 'Test feature installed'\n",
        ));
        FakeHttpClient::create(move |request| {
            let tarball = tarball.clone();
            async move {
                let uri = request.uri().to_string();
                if uri.contains("/token?") {
                    let body: Vec<u8> = br#"{"token":"fake-test-token"}"#.to_vec();
                    Ok(http::Response::builder()
                        .status(200)
                        .body(body.into())
                        .unwrap())
                } else if uri.contains("/manifests/") {
                    let body: Vec<u8> = br#"{"layers":[{"digest":"sha256:deadbeef"}]}"#.to_vec();
                    Ok(http::Response::builder()
                        .status(200)
                        .body(body.into())
                        .unwrap())
                } else if uri.contains("/blobs/") {
                    let body: Vec<u8> = (*tarball).clone();
                    Ok(http::Response::builder()
                        .status(200)
                        .body(body.into())
                        .unwrap())
                } else {
                    Ok(http::Response::builder()
                        .status(404)
                        .body(http_client::AsyncBody::default())
                        .unwrap())
                }
            }
        })
    }

    // Tests needed as I come across them
    // - portsAttributes should reference ports defined in forwardPorts
    //   - This can be either a specification (e.g. "db:5432"), a specific port (3000), or a port range (3000-5000)
    //   - So, we need to do a post-parsing validation there
    // - overrideFeatureInstallOrder should include only featuers listed
    // - Shutdownaction can only be none or stopContainer in the non-compose case. Can only be none or stopCompose in the compose case
    // - (docker compose) service needs to be an actually defined service in the yml file
    //   - Eh maybe this just becomes a runtime error that we handle appropriately
    //
    #[test]
    fn should_validate_incorrect_shutdown_action_for_devcontainer() {}
    #[test]
    fn should_deserialize_simple_devcontainer_json() {
        let given_bad_json = "{ \"image\": 123 }";

        let result = deserialize_devcontainer_json(given_bad_json);

        assert!(result.is_err());
        assert_eq!(result.expect_err("err"), DevContainerErrorV2::UnmappedError);

        let given_image_container_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
                "name": "myDevContainer",
                "remoteUser": "root",
                "forwardPorts": [
                    "db:5432",
                    3000
                ],
                "portsAttributes": {
                    "3000": {
                        "label": "This Port",
                        "onAutoForward": "notify",
                        "elevateIfNeeded": false,
                        "requireLocalPort": true,
                        "protocol": "https"
                    },
                    "db:5432": {
                        "label": "This Port too",
                        "onAutoForward": "silent",
                        "elevateIfNeeded": true,
                        "requireLocalPort": false,
                        "protocol": "http"
                    }
                },
                "otherPortsAttributes": {
                    "label": "Other Ports",
                    "onAutoForward": "openBrowser",
                    "elevateIfNeeded": true,
                    "requireLocalPort": true,
                    "protocol": "https"
                },
                "updateRemoteUserUID": true,
                "remoteEnv": {
                    "MYVAR1": "myvarvalue",
                    "MYVAR2": "myvarothervalue"
                },
                "initializeCommand": ["echo", "initialize_command"],
                "onCreateCommand": "echo on_create_command",
                "updateContentCommand": {
                    "first": "echo update_content_command",
                    "second": ["echo", "update_content_command"]
                },
                "postCreateCommand": ["echo", "post_create_command"],
                "postStartCommand": "echo post_start_command",
                "postAttachCommand": {
                    "something": "echo post_attach_command",
                    "something1": "echo something else",
                },
                "waitFor": "postStartCommand",
                "userEnvProbe": "loginShell",
                "features": {
              		"ghcr.io/devcontainers/features/aws-cli:1": {},
              		"ghcr.io/devcontainers/features/anaconda:1": {}
               	},
                "overrideFeatureInstallOrder": [
                    "ghcr.io/devcontainers/features/anaconda:1",
                    "ghcr.io/devcontainers/features/aws-cli:1"
                ],
                "hostRequirements": {
                    "cpus": 2,
                    "memory": "8gb",
                    "storage": "32gb",
                    // Note that we're not parsing this currently
                    "gpu": true,
                },
                "appPort": 8081,
                "containerEnv": {
                    "MYVAR3": "myvar3",
                    "MYVAR4": "myvar4"
                },
                "containerUser": "myUser",
                "mounts": [
                    {
                        "source": "/localfolder/app",
                        "target": "/workspaces/app",
                        "type": "volume"
                    }
                ],
                "runArgs": [
                    "-c",
                    "some_command"
                ],
                "shutdownAction": "stopContainer",
                "overrideCommand": true,
                "workspaceFolder": "/workspaces",
                "workspaceMount": "/workspaces/app"
            }
            "#;

        let result = deserialize_devcontainer_json(given_image_container_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer,
            DevContainer {
                image: Some(String::from("mcr.microsoft.com/devcontainers/base:ubuntu")),
                name: Some(String::from("myDevContainer")),
                remote_user: Some(String::from("root")),
                forward_ports: Some(vec![
                    ForwardPort::String("db:5432".to_string()),
                    ForwardPort::Number(3000),
                ]),
                ports_attributes: Some(HashMap::from([
                    (
                        "3000".to_string(),
                        PortAttributes {
                            label: "This Port".to_string(),
                            on_auto_forward: OnAutoForward::Notify,
                            elevate_if_needed: false,
                            require_local_port: true,
                            protocol: PortAttributeProtocol::Https
                        }
                    ),
                    (
                        "db:5432".to_string(),
                        PortAttributes {
                            label: "This Port too".to_string(),
                            on_auto_forward: OnAutoForward::Silent,
                            elevate_if_needed: true,
                            require_local_port: false,
                            protocol: PortAttributeProtocol::Http
                        }
                    )
                ])),
                other_ports_attributes: Some(PortAttributes {
                    label: "Other Ports".to_string(),
                    on_auto_forward: OnAutoForward::OpenBrowser,
                    elevate_if_needed: true,
                    require_local_port: true,
                    protocol: PortAttributeProtocol::Https
                }),
                update_remote_user_uid: Some(true),
                remote_env: Some(HashMap::from([
                    ("MYVAR1".to_string(), "myvarvalue".to_string()),
                    ("MYVAR2".to_string(), "myvarothervalue".to_string())
                ])),
                initialize_command: Some(LifecyleScript::from_args(vec![
                    "echo".to_string(),
                    "initialize_command".to_string()
                ])),
                on_create_command: Some(LifecyleScript::from_str("echo on_create_command")),
                update_content_command: Some(LifecyleScript::from_map(HashMap::from([
                    (
                        "first".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    ),
                    (
                        "second".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    )
                ]))),
                post_create_command: Some(LifecyleScript::from_str("echo post_create_command")),
                post_start_command: Some(LifecyleScript::from_args(vec![
                    "echo".to_string(),
                    "post_start_command".to_string()
                ])),
                post_attach_command: Some(LifecyleScript::from_map(HashMap::from([
                    (
                        "something".to_string(),
                        vec!["echo".to_string(), "post_attach_command".to_string()]
                    ),
                    (
                        "something1".to_string(),
                        vec![
                            "echo".to_string(),
                            "something".to_string(),
                            "else".to_string()
                        ]
                    )
                ]))),
                wait_for: Some(LifecycleCommand::PostStartCommand),
                user_env_probe: Some(UserEnvProbe::LoginShell),
                features: Some(HashMap::from([
                    (
                        "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    ),
                    (
                        "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    )
                ])),
                override_feature_install_order: Some(vec![
                    "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                    "ghcr.io/devcontainers/features/aws-cli:1".to_string()
                ]),
                host_requirements: Some(HostRequirements {
                    cpus: Some(2),
                    memory: Some("8gb".to_string()),
                    storage: Some("32gb".to_string()),
                }),
                app_port: Some("8081".to_string()),
                container_env: Some(HashMap::from([
                    ("MYVAR3".to_string(), "myvar3".to_string()),
                    ("MYVAR4".to_string(), "myvar4".to_string())
                ])),
                container_user: Some("myUser".to_string()),
                mounts: Some(vec![MountDefinition {
                    source: "/localfolder/app".to_string(),
                    target: "/workspaces/app".to_string(),
                    mount_type: Some("volume".to_string()),
                }]),
                run_args: Some(vec!["-c".to_string(), "some_command".to_string()]),
                shutdown_action: Some(ShutdownAction::StopContainer),
                override_command: Some(true),
                workspace_folder: Some("/workspaces".to_string()),
                workspace_mount: Some("/workspaces/app".to_string()),
                ..Default::default()
            }
        );

        assert_eq!(devcontainer.build_type(), DevContainerBuildType::Image);
    }

    #[test]
    fn should_deserialize_docker_compose_devcontainer_json() {
        let given_docker_compose_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "name": "myDevContainer",
                "remoteUser": "root",
                "forwardPorts": [
                    "db:5432",
                    3000
                ],
                "portsAttributes": {
                    "3000": {
                        "label": "This Port",
                        "onAutoForward": "notify",
                        "elevateIfNeeded": false,
                        "requireLocalPort": true,
                        "protocol": "https"
                    },
                    "db:5432": {
                        "label": "This Port too",
                        "onAutoForward": "silent",
                        "elevateIfNeeded": true,
                        "requireLocalPort": false,
                        "protocol": "http"
                    }
                },
                "otherPortsAttributes": {
                    "label": "Other Ports",
                    "onAutoForward": "openBrowser",
                    "elevateIfNeeded": true,
                    "requireLocalPort": true,
                    "protocol": "https"
                },
                "updateRemoteUserUID": true,
                "remoteEnv": {
                    "MYVAR1": "myvarvalue",
                    "MYVAR2": "myvarothervalue"
                },
                "initializeCommand": ["echo", "initialize_command"],
                "onCreateCommand": "echo on_create_command",
                "updateContentCommand": {
                    "first": "echo update_content_command",
                    "second": ["echo", "update_content_command"]
                },
                "postCreateCommand": ["echo", "post_create_command"],
                "postStartCommand": "echo post_start_command",
                "postAttachCommand": {
                    "something": "echo post_attach_command",
                    "something1": "echo something else",
                },
                "waitFor": "postStartCommand",
                "userEnvProbe": "loginShell",
                "features": {
              		"ghcr.io/devcontainers/features/aws-cli:1": {},
              		"ghcr.io/devcontainers/features/anaconda:1": {}
               	},
                "overrideFeatureInstallOrder": [
                    "ghcr.io/devcontainers/features/anaconda:1",
                    "ghcr.io/devcontainers/features/aws-cli:1"
                ],
                "hostRequirements": {
                    "cpus": 2,
                    "memory": "8gb",
                    "storage": "32gb",
                    // Note that we're not parsing this currently
                    "gpu": true,
                },
                "dockerComposeFile": "docker-compose.yml",
                "service": "myService",
                "runServices": [
                    "myService",
                    "mySupportingService"
                ],
                "workspaceFolder": "/workspaces/thing",
                "shutdownAction": "stopCompose",
                "overrideCommand": true
            }
            "#;
        let result = deserialize_devcontainer_json(given_docker_compose_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer,
            DevContainer {
                name: Some(String::from("myDevContainer")),
                remote_user: Some(String::from("root")),
                forward_ports: Some(vec![
                    ForwardPort::String("db:5432".to_string()),
                    ForwardPort::Number(3000),
                ]),
                ports_attributes: Some(HashMap::from([
                    (
                        "3000".to_string(),
                        PortAttributes {
                            label: "This Port".to_string(),
                            on_auto_forward: OnAutoForward::Notify,
                            elevate_if_needed: false,
                            require_local_port: true,
                            protocol: PortAttributeProtocol::Https
                        }
                    ),
                    (
                        "db:5432".to_string(),
                        PortAttributes {
                            label: "This Port too".to_string(),
                            on_auto_forward: OnAutoForward::Silent,
                            elevate_if_needed: true,
                            require_local_port: false,
                            protocol: PortAttributeProtocol::Http
                        }
                    )
                ])),
                other_ports_attributes: Some(PortAttributes {
                    label: "Other Ports".to_string(),
                    on_auto_forward: OnAutoForward::OpenBrowser,
                    elevate_if_needed: true,
                    require_local_port: true,
                    protocol: PortAttributeProtocol::Https
                }),
                update_remote_user_uid: Some(true),
                remote_env: Some(HashMap::from([
                    ("MYVAR1".to_string(), "myvarvalue".to_string()),
                    ("MYVAR2".to_string(), "myvarothervalue".to_string())
                ])),
                initialize_command: Some(LifecyleScript::from_args(vec![
                    "echo".to_string(),
                    "initialize_command".to_string()
                ])),
                on_create_command: Some(LifecyleScript::from_str("echo on_create_command")),
                update_content_command: Some(LifecyleScript::from_map(HashMap::from([
                    (
                        "first".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    ),
                    (
                        "second".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    )
                ]))),
                post_create_command: Some(LifecyleScript::from_str("echo post_create_command")),
                post_start_command: Some(LifecyleScript::from_args(vec![
                    "echo".to_string(),
                    "post_start_command".to_string()
                ])),
                post_attach_command: Some(LifecyleScript::from_map(HashMap::from([
                    (
                        "something".to_string(),
                        vec!["echo".to_string(), "post_attach_command".to_string()]
                    ),
                    (
                        "something1".to_string(),
                        vec![
                            "echo".to_string(),
                            "something".to_string(),
                            "else".to_string()
                        ]
                    )
                ]))),
                wait_for: Some(LifecycleCommand::PostStartCommand),
                user_env_probe: Some(UserEnvProbe::LoginShell),
                features: Some(HashMap::from([
                    (
                        "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    ),
                    (
                        "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    )
                ])),
                override_feature_install_order: Some(vec![
                    "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                    "ghcr.io/devcontainers/features/aws-cli:1".to_string()
                ]),
                host_requirements: Some(HostRequirements {
                    cpus: Some(2),
                    memory: Some("8gb".to_string()),
                    storage: Some("32gb".to_string()),
                }),
                docker_compose_file: Some(vec!["docker-compose.yml".to_string()]),
                service: Some("myService".to_string()),
                run_services: Some(vec![
                    "myService".to_string(),
                    "mySupportingService".to_string(),
                ]),
                workspace_folder: Some("/workspaces/thing".to_string()),
                shutdown_action: Some(ShutdownAction::StopCompose),
                override_command: Some(true),
                ..Default::default()
            }
        );

        assert_eq!(
            devcontainer.build_type(),
            DevContainerBuildType::DockerCompose
        );
    }

    #[test]
    fn should_deserialize_dockerfile_devcontainer_json() {
        let given_dockerfile_container_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "name": "myDevContainer",
                "remoteUser": "root",
                "forwardPorts": [
                    "db:5432",
                    3000
                ],
                "portsAttributes": {
                    "3000": {
                        "label": "This Port",
                        "onAutoForward": "notify",
                        "elevateIfNeeded": false,
                        "requireLocalPort": true,
                        "protocol": "https"
                    },
                    "db:5432": {
                        "label": "This Port too",
                        "onAutoForward": "silent",
                        "elevateIfNeeded": true,
                        "requireLocalPort": false,
                        "protocol": "http"
                    }
                },
                "otherPortsAttributes": {
                    "label": "Other Ports",
                    "onAutoForward": "openBrowser",
                    "elevateIfNeeded": true,
                    "requireLocalPort": true,
                    "protocol": "https"
                },
                "updateRemoteUserUID": true,
                "remoteEnv": {
                    "MYVAR1": "myvarvalue",
                    "MYVAR2": "myvarothervalue"
                },
                "initializeCommand": ["echo", "initialize_command"],
                "onCreateCommand": "echo on_create_command",
                "updateContentCommand": {
                    "first": "echo update_content_command",
                    "second": ["echo", "update_content_command"]
                },
                "postCreateCommand": ["echo", "post_create_command"],
                "postStartCommand": "echo post_start_command",
                "postAttachCommand": {
                    "something": "echo post_attach_command",
                    "something1": "echo something else",
                },
                "waitFor": "postStartCommand",
                "userEnvProbe": "loginShell",
                "features": {
              		"ghcr.io/devcontainers/features/aws-cli:1": {},
              		"ghcr.io/devcontainers/features/anaconda:1": {}
               	},
                "overrideFeatureInstallOrder": [
                    "ghcr.io/devcontainers/features/anaconda:1",
                    "ghcr.io/devcontainers/features/aws-cli:1"
                ],
                "hostRequirements": {
                    "cpus": 2,
                    "memory": "8gb",
                    "storage": "32gb",
                    // Note that we're not parsing this currently
                    "gpu": true,
                },
                "appPort": 8081,
                "containerEnv": {
                    "MYVAR3": "myvar3",
                    "MYVAR4": "myvar4"
                },
                "containerUser": "myUser",
                "mounts": [
                    {
                        "source": "/localfolder/app",
                        "target": "/workspaces/app",
                        "type": "volume"
                    },
                    "source=dev-containers-cli-bashhistory,target=/home/node/commandhistory",
                ],
                "runArgs": [
                    "-c",
                    "some_command"
                ],
                "shutdownAction": "stopContainer",
                "overrideCommand": true,
                "workspaceFolder": "/workspaces",
                "workspaceMount": "/workspaces/app",
                "build": {
                   	"dockerfile": "DockerFile",
                   	"context": "..",
                   	"args": {
                   	    "MYARG": "MYVALUE"
                   	},
                   	"options": [
                   	    "--some-option",
                   	    "--mount"
                   	],
                   	"target": "development",
                   	"cacheFrom": "some_image"
                }
            }
            "#;

        let result = deserialize_devcontainer_json(given_dockerfile_container_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer,
            DevContainer {
                name: Some(String::from("myDevContainer")),
                remote_user: Some(String::from("root")),
                forward_ports: Some(vec![
                    ForwardPort::String("db:5432".to_string()),
                    ForwardPort::Number(3000),
                ]),
                ports_attributes: Some(HashMap::from([
                    (
                        "3000".to_string(),
                        PortAttributes {
                            label: "This Port".to_string(),
                            on_auto_forward: OnAutoForward::Notify,
                            elevate_if_needed: false,
                            require_local_port: true,
                            protocol: PortAttributeProtocol::Https
                        }
                    ),
                    (
                        "db:5432".to_string(),
                        PortAttributes {
                            label: "This Port too".to_string(),
                            on_auto_forward: OnAutoForward::Silent,
                            elevate_if_needed: true,
                            require_local_port: false,
                            protocol: PortAttributeProtocol::Http
                        }
                    )
                ])),
                other_ports_attributes: Some(PortAttributes {
                    label: "Other Ports".to_string(),
                    on_auto_forward: OnAutoForward::OpenBrowser,
                    elevate_if_needed: true,
                    require_local_port: true,
                    protocol: PortAttributeProtocol::Https
                }),
                update_remote_user_uid: Some(true),
                remote_env: Some(HashMap::from([
                    ("MYVAR1".to_string(), "myvarvalue".to_string()),
                    ("MYVAR2".to_string(), "myvarothervalue".to_string())
                ])),
                initialize_command: Some(LifecyleScript::from_args(vec![
                    "echo".to_string(),
                    "initialize_command".to_string()
                ])),
                on_create_command: Some(LifecyleScript::from_str("echo on_create_command")),
                update_content_command: Some(LifecyleScript::from_map(HashMap::from([
                    (
                        "first".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    ),
                    (
                        "second".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    )
                ]))),
                post_create_command: Some(LifecyleScript::from_str("echo post_create_command")),
                post_start_command: Some(LifecyleScript::from_args(vec![
                    "echo".to_string(),
                    "post_start_command".to_string()
                ])),
                post_attach_command: Some(LifecyleScript::from_map(HashMap::from([
                    (
                        "something".to_string(),
                        vec!["echo".to_string(), "post_attach_command".to_string()]
                    ),
                    (
                        "something1".to_string(),
                        vec![
                            "echo".to_string(),
                            "something".to_string(),
                            "else".to_string()
                        ]
                    )
                ]))),
                wait_for: Some(LifecycleCommand::PostStartCommand),
                user_env_probe: Some(UserEnvProbe::LoginShell),
                features: Some(HashMap::from([
                    (
                        "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    ),
                    (
                        "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    )
                ])),
                override_feature_install_order: Some(vec![
                    "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                    "ghcr.io/devcontainers/features/aws-cli:1".to_string()
                ]),
                host_requirements: Some(HostRequirements {
                    cpus: Some(2),
                    memory: Some("8gb".to_string()),
                    storage: Some("32gb".to_string()),
                }),
                app_port: Some("8081".to_string()),
                container_env: Some(HashMap::from([
                    ("MYVAR3".to_string(), "myvar3".to_string()),
                    ("MYVAR4".to_string(), "myvar4".to_string())
                ])),
                container_user: Some("myUser".to_string()),
                mounts: Some(vec![
                    MountDefinition {
                        source: "/localfolder/app".to_string(),
                        target: "/workspaces/app".to_string(),
                        mount_type: Some("volume".to_string()),
                    },
                    MountDefinition {
                        source: "dev-containers-cli-bashhistory".to_string(),
                        target: "/home/node/commandhistory".to_string(),
                        mount_type: None,
                    }
                ]),
                run_args: Some(vec!["-c".to_string(), "some_command".to_string()]),
                shutdown_action: Some(ShutdownAction::StopContainer),
                override_command: Some(true),
                workspace_folder: Some("/workspaces".to_string()),
                workspace_mount: Some("/workspaces/app".to_string()),
                build: Some(ContainerBuild {
                    dockerfile: "DockerFile".to_string(),
                    context: Some("..".to_string()),
                    args: Some(HashMap::from([(
                        "MYARG".to_string(),
                        "MYVALUE".to_string()
                    )])),
                    options: Some(vec!["--some-option".to_string(), "--mount".to_string()]),
                    target: Some("development".to_string()),
                    cache_from: Some(vec!["some_image".to_string()]),
                }),
                ..Default::default()
            }
        );

        assert_eq!(devcontainer.build_type(), DevContainerBuildType::Dockerfile);
    }

    #[test]
    fn should_get_remote_user_from_devcontainer_if_available() {
        let given_dev_container = DevContainerManifest {
            config: DevContainer {
                image: Some("image".to_string()),
                name: None,
                remote_user: Some("root".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let mut metadata = HashMap::new();
        metadata.insert(
            "remoteUser".to_string(),
            serde_json_lenient::Value::String("vsCode".to_string()),
        );
        let given_docker_config = DockerInspect {
            id: "docker_id".to_string(),
            config: DockerInspectConfig {
                labels: DockerConfigLabels {
                    metadata: Some(vec![metadata]),
                },
                image_user: None,
            },
            mounts: None,
        };

        let remote_user =
            get_remote_user_from_config(&given_docker_config, &given_dev_container).unwrap();

        assert_eq!(remote_user, "root".to_string())
    }

    #[test]
    fn should_get_remote_user_from_docker_config() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "remoteUser".to_string(),
            serde_json_lenient::Value::String("vsCode".to_string()),
        );
        let given_docker_config = DockerInspect {
            id: "docker_id".to_string(),
            config: DockerInspectConfig {
                labels: DockerConfigLabels {
                    metadata: Some(vec![metadata]),
                },
                image_user: None,
            },
            mounts: None,
        };

        let remote_user = get_remote_user_from_config(
            &given_docker_config,
            &DevContainerManifest {
                config: DevContainer {
                    image: None,
                    name: None,
                    remote_user: None,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        assert!(remote_user.is_ok());
        let remote_user = remote_user.expect("ok");
        assert_eq!(&remote_user, "vsCode")
    }

    #[test]
    fn should_create_correct_docker_build_command() {
        let features_content_dir =
            PathBuf::from("/tmp/devcontainercli/container-features/0.82.0-1234567890");
        let dockerfile_path = features_content_dir.join("Dockerfile.extended");
        let empty_context_dir = PathBuf::from("/tmp/devcontainercli/empty-folder");

        let docker_build_command = create_docker_build(&DevContainerManifest {
            config: DevContainer {
                image: Some("mcr.microsoft.com/devcontainers/rust:2-1-bookworm".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();

        assert_eq!(docker_build_command.get_program(), "docker");
        assert_eq!(
            docker_build_command.get_args().collect::<Vec<&OsStr>>(),
            vec![
                OsStr::new("buildx"),
                OsStr::new("build"),
                OsStr::new("--load"),
                OsStr::new("--build-context"),
                OsStr::new(&format!(
                    "dev_containers_feature_content_source={}",
                    features_content_dir.display()
                )),
                OsStr::new("--build-arg"),
                OsStr::new(
                    "_DEV_CONTAINERS_BASE_IMAGE=mcr.microsoft.com/devcontainers/rust:2-1-bookworm"
                ),
                OsStr::new("--build-arg"),
                OsStr::new("_DEV_CONTAINERS_IMAGE_USER=root"),
                OsStr::new("--build-arg"),
                OsStr::new(
                    "_DEV_CONTAINERS_FEATURE_CONTENT_SOURCE=dev_container_feature_content_temp"
                ),
                OsStr::new("--target"),
                OsStr::new("dev_containers_target_stage"),
                OsStr::new("-f"),
                OsStr::new(&dockerfile_path.display().to_string()),
                OsStr::new("-t"),
                OsStr::new("vsc-cli-abc123-features"),
                OsStr::new(&empty_context_dir.display().to_string()),
            ]
        );
    }

    #[test]
    fn should_extract_feature_id_from_references() {
        assert_eq!(
            extract_feature_id("ghcr.io/devcontainers/features/aws-cli:1"),
            "aws-cli"
        );
        assert_eq!(
            extract_feature_id("ghcr.io/devcontainers/features/go"),
            "go"
        );
        assert_eq!(extract_feature_id("ghcr.io/user/repo/node:18.0.0"), "node");
        assert_eq!(extract_feature_id("./myFeature"), "myFeature");
        assert_eq!(
            extract_feature_id("ghcr.io/devcontainers/features/rust@sha256:abc123"),
            "rust"
        );
    }

    #[test]
    fn should_get_safe_id() {
        assert_eq!(safe_id_upper("version"), "VERSION");
        assert_eq!(safe_id_upper("aws-cli"), "AWS_CLI");
        assert_eq!(safe_id_upper("optionA"), "OPTIONA");
        assert_eq!(safe_id_upper("123abc"), "_ABC");
        assert_eq!(safe_id_upper("___test"), "_TEST");
        assert_eq!(
            safe_id_upper("DevContainer Name for (Greatness"),
            "DEVCONTAINER_NAME_FOR__GREATNESS"
        );
    }

    // Keeping these around since an example of this can probably translate to DevContainerManifest
    //
    // #[test]
    // fn should_construct_features_build_resources() {
    //     let client = fake_oci_http_client();
    //     smol::block_on(async {
    //         let temp_dir = std::env::temp_dir().join("devcontainer-test-features-build");
    //         let features_dir = temp_dir.join("features-content");
    //         let empty_dir = temp_dir.join("empty");
    //         let dockerfile_path = features_dir.join("Dockerfile.extended");

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //         std::fs::create_dir_all(&features_dir).unwrap();
    //         std::fs::create_dir_all(&empty_dir).unwrap();

    //         let build_info = FeaturesBuildInfo {
    //             dockerfile_path: dockerfile_path.clone(),
    //             features_content_dir: features_dir.clone(),
    //             empty_context_dir: empty_dir,
    //             base_image: Some("mcr.microsoft.com/devcontainers/rust:2-1-bookworm".to_string()),
    //             image_tag: "vsc-test-features".to_string(),
    //         };

    //         let dev_container = DevContainerManifest {
    //             config: DevContainer {
    //                 image: Some("mcr.microsoft.com/devcontainers/rust:2-1-bookworm".to_string()),
    //                 features: Some(HashMap::from([
    //                     (
    //                         "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
    //                         FeatureOptions::Options(HashMap::new()),
    //                     ),
    //                     (
    //                         "ghcr.io/devcontainers/features/node:1".to_string(),
    //                         FeatureOptions::String("18".to_string()),
    //                     ),
    //                 ])),
    //                 remote_user: Some("vscode".to_string()),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         };

    //         let result =
    //             construct_features_build_resources(&dev_container, &build_info, &client, None)
    //                 .await;
    //         assert!(
    //             result.is_ok(),
    //             "construct_features_build_resources failed: {:?}",
    //             result
    //         );

    //         // Verify builtin env file
    //         let builtin_env =
    //             std::fs::read_to_string(features_dir.join("devcontainer-features.builtin.env"))
    //                 .unwrap();
    //         assert!(builtin_env.contains("_CONTAINER_USER=root"));
    //         assert!(builtin_env.contains("_REMOTE_USER=vscode"));

    //         // Verify Dockerfile.extended exists and contains expected structures
    //         let dockerfile = std::fs::read_to_string(&dockerfile_path).unwrap();
    //         assert!(
    //             dockerfile
    //                 .contains("FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage")
    //         );
    //         assert!(dockerfile.contains("dev_containers_feature_content_source"));
    //         assert!(dockerfile.contains("devcontainer-features-install.sh"));
    //         assert!(dockerfile.contains("_DEV_CONTAINERS_IMAGE_USER"));

    //         // Verify feature directories (sorted: aws-cli at index 0, node at index 1)
    //         assert!(features_dir.join("aws-cli_0").exists());
    //         assert!(features_dir.join("node_1").exists());

    //         // Verify aws-cli feature files — env should contain defaults from the
    //         // fake tarball's devcontainer-feature.json (which has none since our
    //         // test tarball doesn't include one), so it will be empty.
    //         let aws_env =
    //             std::fs::read_to_string(features_dir.join("aws-cli_0/devcontainer-features.env"))
    //                 .unwrap();
    //         assert!(
    //             aws_env.is_empty(),
    //             "aws-cli with empty options and no feature json defaults should produce an empty env file, got: {}",
    //             aws_env,
    //         );

    //         let aws_wrapper = std::fs::read_to_string(
    //             features_dir.join("aws-cli_0/devcontainer-features-install.sh"),
    //         )
    //         .unwrap();
    //         assert!(aws_wrapper.contains("#!/bin/sh"));
    //         assert!(aws_wrapper.contains("./install.sh"));
    //         assert!(aws_wrapper.contains("../devcontainer-features.builtin.env"));

    //         let aws_install =
    //             std::fs::read_to_string(features_dir.join("aws-cli_0/install.sh")).unwrap();
    //         assert!(
    //             aws_install.contains("Test feature installed"),
    //             "install.sh should contain content from the OCI tarball, got: {}",
    //             aws_install
    //         );

    //         // Verify node feature files (String("18") → VERSION="18")
    //         let node_env =
    //             std::fs::read_to_string(features_dir.join("node_1/devcontainer-features.env"))
    //                 .unwrap();
    //         assert!(
    //             node_env.contains("VERSION=\"18\""),
    //             "Expected VERSION=\"18\" in node env, got: {}",
    //             node_env
    //         );

    //         // Verify Dockerfile layers reference both features
    //         assert!(dockerfile.contains("aws-cli_0"));
    //         assert!(dockerfile.contains("node_1"));

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //     });
    // }
    // #[test]
    // fn should_construct_features_with_override_order() {
    //     let client = fake_oci_http_client();
    //     smol::block_on(async {
    //         let temp_dir = std::env::temp_dir().join("devcontainer-test-features-order");
    //         let features_dir = temp_dir.join("features-content");
    //         let empty_dir = temp_dir.join("empty");
    //         let dockerfile_path = features_dir.join("Dockerfile.extended");

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //         std::fs::create_dir_all(&features_dir).unwrap();
    //         std::fs::create_dir_all(&empty_dir).unwrap();

    //         let build_info = FeaturesBuildInfo {
    //             dockerfile_path: dockerfile_path.clone(),
    //             features_content_dir: features_dir.clone(),
    //             empty_context_dir: empty_dir,
    //             build_image: Some("mcr.microsoft.com/devcontainers/base:ubuntu".to_string()),
    //             image_tag: "vsc-test-order".to_string(),
    //         };

    //         let dev_container = DevContainerManifest {
    //             config: DevContainer {
    //                 image: Some("mcr.microsoft.com/devcontainers/base:ubuntu".to_string()),
    //                 features: Some(HashMap::from([
    //                     (
    //                         "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
    //                         FeatureOptions::Options(HashMap::new()),
    //                     ),
    //                     (
    //                         "ghcr.io/devcontainers/features/node:1".to_string(),
    //                         FeatureOptions::Options(HashMap::from([(
    //                             "version".to_string(),
    //                             FeatureOptionValue::String("20".to_string()),
    //                         )])),
    //                     ),
    //                 ])),
    //                 override_feature_install_order: Some(vec![
    //                     "ghcr.io/devcontainers/features/node:1".to_string(),
    //                     "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
    //                 ]),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         };

    //         let result =
    //             construct_features_build_resources(&dev_container, &build_info, &client, None)
    //                 .await;
    //         assert!(result.is_ok());

    //         // With override order: node first (index 0), aws-cli second (index 1)
    //         assert!(features_dir.join("node_0").exists());
    //         assert!(features_dir.join("aws-cli_1").exists());

    //         let node_env =
    //             std::fs::read_to_string(features_dir.join("node_0/devcontainer-features.env"))
    //                 .unwrap();
    //         assert!(
    //             node_env.contains("version=\"20\""),
    //             "Expected version=\"20\" in node env, got: {}",
    //             node_env
    //         );

    //         // Verify the Dockerfile layers appear in the right order
    //         let dockerfile = std::fs::read_to_string(&dockerfile_path).unwrap();
    //         let node_pos = dockerfile.find("node_0").expect("node_0 layer missing");
    //         let aws_pos = dockerfile
    //             .find("aws-cli_1")
    //             .expect("aws-cli_1 layer missing");
    //         assert!(
    //             node_pos < aws_pos,
    //             "node should appear before aws-cli in the Dockerfile"
    //         );

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //     });
    // }

    // #[test]
    // fn should_skip_disabled_features() {
    //     let client = fake_oci_http_client();
    //     smol::block_on(async {
    //         let temp_dir = std::env::temp_dir().join("devcontainer-test-features-disabled");
    //         let features_dir = temp_dir.join("features-content");
    //         let empty_dir = temp_dir.join("empty");
    //         let dockerfile_path = features_dir.join("Dockerfile.extended");

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //         std::fs::create_dir_all(&features_dir).unwrap();
    //         std::fs::create_dir_all(&empty_dir).unwrap();

    //         let build_info = FeaturesBuildInfo {
    //             dockerfile_path: dockerfile_path.clone(),
    //             features_content_dir: features_dir.clone(),
    //             empty_context_dir: empty_dir,
    //             build_image: Some("mcr.microsoft.com/devcontainers/base:ubuntu".to_string()),
    //             image_tag: "vsc-test-disabled".to_string(),
    //         };

    //         let dev_container = DevContainerManifest {
    //             config: DevContainer {
    //                 image: Some("mcr.microsoft.com/devcontainers/base:ubuntu".to_string()),
    //                 features: Some(HashMap::from([
    //                     (
    //                         "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
    //                         FeatureOptions::Bool(false),
    //                     ),
    //                     (
    //                         "ghcr.io/devcontainers/features/node:1".to_string(),
    //                         FeatureOptions::Bool(true),
    //                     ),
    //                 ])),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         };

    //         let result =
    //             construct_features_build_resources(&dev_container, &build_info, &client, None)
    //                 .await;
    //         assert!(result.is_ok());

    //         // aws-cli is disabled (false) — its directory should not exist
    //         assert!(!features_dir.join("aws-cli_0").exists());
    //         // node is enabled (true) — its directory should exist
    //         assert!(features_dir.join("node_1").exists());

    //         let dockerfile = std::fs::read_to_string(&dockerfile_path).unwrap();
    //         assert!(!dockerfile.contains("aws-cli_0"));
    //         assert!(dockerfile.contains("node_1"));

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //     });
    // }

    // #[test]
    // fn should_fail_when_oci_download_fails() {
    //     let client = fake_http_client();
    //     smol::block_on(async {
    //         let temp_dir = std::env::temp_dir().join("devcontainer-test-features-fail");
    //         let features_dir = temp_dir.join("features-content");
    //         let empty_dir = temp_dir.join("empty");
    //         let dockerfile_path = features_dir.join("Dockerfile.extended");

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //         std::fs::create_dir_all(&features_dir).unwrap();
    //         std::fs::create_dir_all(&empty_dir).unwrap();

    //         let build_info = FeaturesBuildInfo {
    //             dockerfile_path: dockerfile_path.clone(),
    //             features_content_dir: features_dir.clone(),
    //             empty_context_dir: empty_dir,
    //             build_image: Some("mcr.microsoft.com/devcontainers/base:ubuntu".to_string()),
    //             image_tag: "vsc-test-fail".to_string(),
    //         };

    //         let dev_container = DevContainerManifest {
    //             config: DevContainer {
    //                 image: Some("mcr.microsoft.com/devcontainers/base:ubuntu".to_string()),
    //                 features: Some(HashMap::from([(
    //                     "ghcr.io/devcontainers/features/go:1".to_string(),
    //                     FeatureOptions::Options(HashMap::new()),
    //                 )])),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         };

    //         let result =
    //             construct_features_build_resources(&dev_container, &build_info, &client, None)
    //                 .await;
    //         assert!(
    //             result.is_err(),
    //             "Expected error when OCI download fails, but got Ok"
    //         );

    //         let _ = std::fs::remove_dir_all(&temp_dir);
    //     });
    // }

    #[test]
    fn should_create_correct_docker_run_command() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "remoteUser".to_string(),
            serde_json_lenient::Value::String("vsCode".to_string()),
        );
        let labels = vec![
            ("label1", "value1".to_string()),
            ("label2", "value2".to_string()),
        ];

        let build_resources = DockerBuildResources {
            image: DockerInspect {
                id: "mcr.microsoft.com/devcontainers/base:ubuntu".to_string(),
                config: DockerInspectConfig {
                    labels: DockerConfigLabels { metadata: None },
                    image_user: None,
                },
                mounts: None,
            },
            additional_mounts: vec![],
            privileged: false,
            entrypoints: vec![],
        };
        let docker_run_command = create_docker_run_command(
            build_resources,
            &Arc::new(Path::new("/local/project_app")),
            Some(&labels),
        );

        assert!(docker_run_command.is_ok());
        let docker_run_command = docker_run_command.expect("ok");

        assert_eq!(docker_run_command.get_program(), "docker");
        assert_eq!(
            docker_run_command.get_args().collect::<Vec<&OsStr>>(),
            vec![
                OsStr::new("run"),
                OsStr::new("--sig-proxy=false"),
                OsStr::new("-d"),
                OsStr::new("--mount"),
                OsStr::new(
                    "type=bind,source=/local/project_app,target=/workspaces/project_app,consistency=cached"
                ),
                OsStr::new("-l"),
                OsStr::new("label1=value1"),
                OsStr::new("-l"),
                OsStr::new("label2=value2"),
                OsStr::new("-l"),
                OsStr::new(
                    r#"devcontainer.metadata=[{"id":"ghcr.io/devcontainers/features/common-utils:2"},{"id":"ghcr.io/devcontainers/features/git:1","customizations":{"vscode":{"settings":{"github.copilot.chat.codeGeneration.instructions":[{"text":"This dev container includes an up-to-date version of Git, built from source as needed, pre-installed and available on the `PATH`."}]}}}},{"remoteUser":"vscode"}]"#
                ),
                OsStr::new("--entrypoint"),
                OsStr::new("/bin/sh"),
                OsStr::new("mcr.microsoft.com/devcontainers/base:ubuntu"),
                OsStr::new("-c"),
                OsStr::new(
                    "
echo Container started
trap \"exit 0\" 15
exec \"$@\"
while sleep 1 & wait $!; do :; done
                    "
                    .trim()
                ),
                OsStr::new("-"),
            ]
        )
    }

    #[test]
    fn should_deserialize_docker_ps_with_filters() {
        // First, deserializes empty
        let empty_output = Output {
            status: ExitStatus::default(),
            stderr: vec![],
            stdout: String::from("").into_bytes(),
        };

        let result: Option<DockerPs> = deserialize_json_output(empty_output).unwrap();

        assert!(result.is_none());

        let full_output = Output {
            status: ExitStatus::default(),
            stderr: vec![],
            stdout: String::from(r#"
{
    "Command": "\"/bin/sh -c 'echo Co…\"",
    "CreatedAt": "2026-02-04 15:44:21 -0800 PST",
    "ID": "abdb6ab59573",
    "Image": "mcr.microsoft.com/devcontainers/base:ubuntu",
    "Labels": "desktop.docker.io/mounts/0/Source=/somepath/cli,desktop.docker.io/mounts/0/SourceKind=hostFile,desktop.docker.io/mounts/0/Target=/workspaces/cli,desktop.docker.io/ports.scheme=v2,dev.containers.features=common,dev.containers.id=base-ubuntu,dev.containers.release=v0.4.24,dev.containers.source=https://github.com/devcontainers/images,dev.containers.timestamp=Fri, 30 Jan 2026 16:52:34 GMT,dev.containers.variant=noble,devcontainer.config_file=/somepath/cli/.devcontainer/dev_container_2/devcontainer.json,devcontainer.local_folder=/somepath/cli,devcontainer.metadata=[{\"id\":\"ghcr.io/devcontainers/features/common-utils:2\"},{\"id\":\"ghcr.io/devcontainers/features/git:1\",\"customizations\":{\"vscode\":{\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes an up-to-date version of Git, built from source as needed, pre-installed and available on the `PATH`.\"}]}}}},{\"remoteUser\":\"vscode\"}],org.opencontainers.image.ref.name=ubuntu,org.opencontainers.image.version=24.04,version=2.1.6",
    "LocalVolumes": "0",
    "Mounts": "/host_mnt/User…",
    "Names": "objective_haslett",
    "Networks": "bridge",
    "Platform": {
    "architecture": "arm64",
    "os": "linux"
    },
    "Ports": "",
    "RunningFor": "47 hours ago",
    "Size": "0B",
    "State": "running",
    "Status": "Up 47 hours"
}
                "#).into_bytes(),
        };

        let result: Option<DockerPs> = deserialize_json_output(full_output).unwrap();

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.id, "abdb6ab59573".to_string());
    }

    #[test]
    fn should_deserialize_docker_labels() {
        let given_config = r#"
{"Id":"fca38334e88f9045a8cc41ebe0dc94e955a74dda2e526ed7546cf7a0f27b5b75","Created":"2026-02-09T23:22:15.585555798Z","Path":"/bin/sh","Args":["-c","echo Container started\ntrap \"exit 0\" 15\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done","-"],"State":{"Status":"running","Running":true,"Paused":false,"Restarting":false,"OOMKilled":false,"Dead":false,"Pid":94196,"ExitCode":0,"Error":"","StartedAt":"2026-02-09T23:22:15.628810548Z","FinishedAt":"0001-01-01T00:00:00Z"},"Image":"sha256:3dcb059253b2ebb44de3936620e1cff3dadcd2c1c982d579081ca8128c1eb319","ResolvConfPath":"/var/lib/docker/containers/fca38334e88f9045a8cc41ebe0dc94e955a74dda2e526ed7546cf7a0f27b5b75/resolv.conf","HostnamePath":"/var/lib/docker/containers/fca38334e88f9045a8cc41ebe0dc94e955a74dda2e526ed7546cf7a0f27b5b75/hostname","HostsPath":"/var/lib/docker/containers/fca38334e88f9045a8cc41ebe0dc94e955a74dda2e526ed7546cf7a0f27b5b75/hosts","LogPath":"/var/lib/docker/containers/fca38334e88f9045a8cc41ebe0dc94e955a74dda2e526ed7546cf7a0f27b5b75/fca38334e88f9045a8cc41ebe0dc94e955a74dda2e526ed7546cf7a0f27b5b75-json.log","Name":"/magical_easley","RestartCount":0,"Driver":"overlayfs","Platform":"linux","MountLabel":"","ProcessLabel":"","AppArmorProfile":"","ExecIDs":null,"HostConfig":{"Binds":null,"ContainerIDFile":"","LogConfig":{"Type":"json-file","Config":{}},"NetworkMode":"bridge","PortBindings":{},"RestartPolicy":{"Name":"no","MaximumRetryCount":0},"AutoRemove":false,"VolumeDriver":"","VolumesFrom":null,"ConsoleSize":[0,0],"CapAdd":null,"CapDrop":null,"CgroupnsMode":"private","Dns":[],"DnsOptions":[],"DnsSearch":[],"ExtraHosts":null,"GroupAdd":null,"IpcMode":"private","Cgroup":"","Links":null,"OomScoreAdj":0,"PidMode":"","Privileged":false,"PublishAllPorts":false,"ReadonlyRootfs":false,"SecurityOpt":null,"UTSMode":"","UsernsMode":"","ShmSize":67108864,"Runtime":"runc","Isolation":"","CpuShares":0,"Memory":0,"NanoCpus":0,"CgroupParent":"","BlkioWeight":0,"BlkioWeightDevice":[],"BlkioDeviceReadBps":[],"BlkioDeviceWriteBps":[],"BlkioDeviceReadIOps":[],"BlkioDeviceWriteIOps":[],"CpuPeriod":0,"CpuQuota":0,"CpuRealtimePeriod":0,"CpuRealtimeRuntime":0,"CpusetCpus":"","CpusetMems":"","Devices":[],"DeviceCgroupRules":null,"DeviceRequests":null,"MemoryReservation":0,"MemorySwap":0,"MemorySwappiness":null,"OomKillDisable":null,"PidsLimit":null,"Ulimits":[],"CpuCount":0,"CpuPercent":0,"IOMaximumIOps":0,"IOMaximumBandwidth":0,"Mounts":[{"Type":"bind","Source":"/somepath/rustwebstarter","Target":"/workspaces/rustwebstarter","Consistency":"cached"}],"MaskedPaths":["/proc/asound","/proc/acpi","/proc/interrupts","/proc/kcore","/proc/keys","/proc/latency_stats","/proc/timer_list","/proc/timer_stats","/proc/sched_debug","/proc/scsi","/sys/firmware","/sys/devices/virtual/powercap"],"ReadonlyPaths":["/proc/bus","/proc/fs","/proc/irq","/proc/sys","/proc/sysrq-trigger"]},"GraphDriver":{"Data":null,"Name":"overlayfs"},"Mounts":[{"Type":"bind","Source":"/somepath/rustwebstarter","Destination":"/workspaces/rustwebstarter","Mode":"","RW":true,"Propagation":"rprivate"}],"Config":{"Hostname":"fca38334e88f","Domainname":"","User":"root","AttachStdin":false,"AttachStdout":false,"AttachStderr":false,"Tty":false,"OpenStdin":false,"StdinOnce":false,"Env":["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"],"Cmd":["-c","echo Container started\ntrap \"exit 0\" 15\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done","-"],"Image":"mcr.microsoft.com/devcontainers/base:ubuntu","Volumes":null,"WorkingDir":"","Entrypoint":["/bin/sh"],"OnBuild":null,"Labels":{"dev.containers.features":"common","dev.containers.id":"base-ubuntu","dev.containers.release":"v0.4.24","dev.containers.source":"https://github.com/devcontainers/images","dev.containers.timestamp":"Fri, 30 Jan 2026 16:52:34 GMT","dev.containers.variant":"noble","devcontainer.config_file":".devcontainer/devcontainer.json","devcontainer.local_folder":"/somepath/rustwebstarter","devcontainer.metadata":"[ {\"id\":\"ghcr.io/devcontainers/features/common-utils:2\"}, {\"id\":\"ghcr.io/devcontainers/features/git:1\",\"customizations\":{\"vscode\":{\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes an up-to-date version of Git, built from source as needed, pre-installed and available on the `PATH`.\"}]}}}}, {\"remoteUser\":\"vscode\"} ]","org.opencontainers.image.ref.name":"ubuntu","org.opencontainers.image.version":"24.04","version":"2.1.6"},"StopTimeout":1},"NetworkSettings":{"Bridge":"","SandboxID":"ef2f9f610d87de6bf6061627a0cadb2b89e918bafba92e0e4e9e877d092315c7","SandboxKey":"/var/run/docker/netns/ef2f9f610d87","Ports":{},"HairpinMode":false,"LinkLocalIPv6Address":"","LinkLocalIPv6PrefixLen":0,"SecondaryIPAddresses":null,"SecondaryIPv6Addresses":null,"EndpointID":"50b3501ee308c36e212a025b4f4ddd4ffbd6aeeafa986350ea7d9fe5e16e2c8c","Gateway":"172.17.0.1","GlobalIPv6Address":"","GlobalIPv6PrefixLen":0,"IPAddress":"172.17.0.4","IPPrefixLen":16,"IPv6Gateway":"","MacAddress":"ca:02:9f:22:fd:8e","Networks":{"bridge":{"IPAMConfig":null,"Links":null,"Aliases":null,"MacAddress":"ca:02:9f:22:fd:8e","DriverOpts":null,"GwPriority":0,"NetworkID":"51bb8ccc4d1281db44f16d915963fc728619d4a68e2f90e5ea8f1cb94885063e","EndpointID":"50b3501ee308c36e212a025b4f4ddd4ffbd6aeeafa986350ea7d9fe5e16e2c8c","Gateway":"172.17.0.1","IPAddress":"172.17.0.4","IPPrefixLen":16,"IPv6Gateway":"","GlobalIPv6Address":"","GlobalIPv6PrefixLen":0,"DNSNames":null}}},"ImageManifestDescriptor":{"mediaType":"application/vnd.oci.image.manifest.v1+json","digest":"sha256:39c3436527190561948236894c55b59fa58aa08d68d8867e703c8d5ab72a3593","size":2195,"platform":{"architecture":"arm64","os":"linux"}}}
            "#;

        let deserialized = serde_json_lenient::from_str::<DockerInspect>(given_config);
        assert!(deserialized.is_ok());
        let config = deserialized.unwrap();
        let remote_user = get_remote_user_from_config(
            &config,
            &DevContainerManifest {
                config: DevContainer {
                    image: None,
                    name: None,
                    remote_user: None,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        assert!(remote_user.is_ok());
        assert_eq!(remote_user.unwrap(), "vscode".to_string())
    }

    #[test]
    fn should_get_target_dir_from_docker_inspect() {
        let given_config = r#"
{
  "Id": "abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85",
  "Created": "2026-02-04T23:44:21.802688084Z",
  "Path": "/bin/sh",
  "Args": [
    "-c",
    "echo Container started\ntrap \"exit 0\" 15\n\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done",
    "-"
  ],
  "State": {
    "Status": "running",
    "Running": true,
    "Paused": false,
    "Restarting": false,
    "OOMKilled": false,
    "Dead": false,
    "Pid": 23087,
    "ExitCode": 0,
    "Error": "",
    "StartedAt": "2026-02-04T23:44:21.954875084Z",
    "FinishedAt": "0001-01-01T00:00:00Z"
  },
  "Image": "sha256:3dcb059253b2ebb44de3936620e1cff3dadcd2c1c982d579081ca8128c1eb319",
  "ResolvConfPath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/resolv.conf",
  "HostnamePath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/hostname",
  "HostsPath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/hosts",
  "LogPath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85-json.log",
  "Name": "/objective_haslett",
  "RestartCount": 0,
  "Driver": "overlayfs",
  "Platform": "linux",
  "MountLabel": "",
  "ProcessLabel": "",
  "AppArmorProfile": "",
  "ExecIDs": [
    "008019d93df4107fcbba78bcc6e1ed7e121844f36c26aca1a56284655a6adb53"
  ],
  "HostConfig": {
    "Binds": null,
    "ContainerIDFile": "",
    "LogConfig": {
      "Type": "json-file",
      "Config": {}
    },
    "NetworkMode": "bridge",
    "PortBindings": {},
    "RestartPolicy": {
      "Name": "no",
      "MaximumRetryCount": 0
    },
    "AutoRemove": false,
    "VolumeDriver": "",
    "VolumesFrom": null,
    "ConsoleSize": [
      0,
      0
    ],
    "CapAdd": null,
    "CapDrop": null,
    "CgroupnsMode": "private",
    "Dns": [],
    "DnsOptions": [],
    "DnsSearch": [],
    "ExtraHosts": null,
    "GroupAdd": null,
    "IpcMode": "private",
    "Cgroup": "",
    "Links": null,
    "OomScoreAdj": 0,
    "PidMode": "",
    "Privileged": false,
    "PublishAllPorts": false,
    "ReadonlyRootfs": false,
    "SecurityOpt": null,
    "UTSMode": "",
    "UsernsMode": "",
    "ShmSize": 67108864,
    "Runtime": "runc",
    "Isolation": "",
    "CpuShares": 0,
    "Memory": 0,
    "NanoCpus": 0,
    "CgroupParent": "",
    "BlkioWeight": 0,
    "BlkioWeightDevice": [],
    "BlkioDeviceReadBps": [],
    "BlkioDeviceWriteBps": [],
    "BlkioDeviceReadIOps": [],
    "BlkioDeviceWriteIOps": [],
    "CpuPeriod": 0,
    "CpuQuota": 0,
    "CpuRealtimePeriod": 0,
    "CpuRealtimeRuntime": 0,
    "CpusetCpus": "",
    "CpusetMems": "",
    "Devices": [],
    "DeviceCgroupRules": null,
    "DeviceRequests": null,
    "MemoryReservation": 0,
    "MemorySwap": 0,
    "MemorySwappiness": null,
    "OomKillDisable": null,
    "PidsLimit": null,
    "Ulimits": [],
    "CpuCount": 0,
    "CpuPercent": 0,
    "IOMaximumIOps": 0,
    "IOMaximumBandwidth": 0,
    "Mounts": [
      {
        "Type": "bind",
        "Source": "/somepath/cli",
        "Target": "/workspaces/cli",
        "Consistency": "cached"
      }
    ],
    "MaskedPaths": [
      "/proc/asound",
      "/proc/acpi",
      "/proc/interrupts",
      "/proc/kcore",
      "/proc/keys",
      "/proc/latency_stats",
      "/proc/timer_list",
      "/proc/timer_stats",
      "/proc/sched_debug",
      "/proc/scsi",
      "/sys/firmware",
      "/sys/devices/virtual/powercap"
    ],
    "ReadonlyPaths": [
      "/proc/bus",
      "/proc/fs",
      "/proc/irq",
      "/proc/sys",
      "/proc/sysrq-trigger"
    ]
  },
  "GraphDriver": {
    "Data": null,
    "Name": "overlayfs"
  },
  "Mounts": [
    {
      "Type": "bind",
      "Source": "/somepath/cli",
      "Destination": "/workspaces/cli",
      "Mode": "",
      "RW": true,
      "Propagation": "rprivate"
    }
  ],
  "Config": {
    "Hostname": "abdb6ab59573",
    "Domainname": "",
    "User": "root",
    "AttachStdin": false,
    "AttachStdout": true,
    "AttachStderr": true,
    "Tty": false,
    "OpenStdin": false,
    "StdinOnce": false,
    "Env": [
      "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
    ],
    "Cmd": [
      "-c",
      "echo Container started\ntrap \"exit 0\" 15\n\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done",
      "-"
    ],
    "Image": "mcr.microsoft.com/devcontainers/base:ubuntu",
    "Volumes": null,
    "WorkingDir": "",
    "Entrypoint": [
      "/bin/sh"
    ],
    "OnBuild": null,
    "Labels": {
      "dev.containers.features": "common",
      "dev.containers.id": "base-ubuntu",
      "dev.containers.release": "v0.4.24",
      "dev.containers.source": "https://github.com/devcontainers/images",
      "dev.containers.timestamp": "Fri, 30 Jan 2026 16:52:34 GMT",
      "dev.containers.variant": "noble",
      "devcontainer.config_file": "/somepath/cli/.devcontainer/dev_container_2/devcontainer.json",
      "devcontainer.local_folder": "/somepath/cli",
      "devcontainer.metadata": "[{\"id\":\"ghcr.io/devcontainers/features/common-utils:2\"},{\"id\":\"ghcr.io/devcontainers/features/git:1\",\"customizations\":{\"vscode\":{\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes an up-to-date version of Git, built from source as needed, pre-installed and available on the `PATH`.\"}]}}}},{\"remoteUser\":\"vscode\"}]",
      "org.opencontainers.image.ref.name": "ubuntu",
      "org.opencontainers.image.version": "24.04",
      "version": "2.1.6"
    },
    "StopTimeout": 1
  },
  "NetworkSettings": {
    "Bridge": "",
    "SandboxID": "2a94990d542fe532deb75f1cc67f761df2d669e3b41161f914079e88516cc54b",
    "SandboxKey": "/var/run/docker/netns/2a94990d542f",
    "Ports": {},
    "HairpinMode": false,
    "LinkLocalIPv6Address": "",
    "LinkLocalIPv6PrefixLen": 0,
    "SecondaryIPAddresses": null,
    "SecondaryIPv6Addresses": null,
    "EndpointID": "ef5b35a8fbb145565853e1a1d960e737fcc18c20920e96494e4c0cfc55683570",
    "Gateway": "172.17.0.1",
    "GlobalIPv6Address": "",
    "GlobalIPv6PrefixLen": 0,
    "IPAddress": "172.17.0.3",
    "IPPrefixLen": 16,
    "IPv6Gateway": "",
    "MacAddress": "",
    "Networks": {
      "bridge": {
        "IPAMConfig": null,
        "Links": null,
        "Aliases": null,
        "MacAddress": "9a:ec:af:8a:ac:81",
        "DriverOpts": null,
        "GwPriority": 0,
        "NetworkID": "51bb8ccc4d1281db44f16d915963fc728619d4a68e2f90e5ea8f1cb94885063e",
        "EndpointID": "ef5b35a8fbb145565853e1a1d960e737fcc18c20920e96494e4c0cfc55683570",
        "Gateway": "172.17.0.1",
        "IPAddress": "172.17.0.3",
        "IPPrefixLen": 16,
        "IPv6Gateway": "",
        "GlobalIPv6Address": "",
        "GlobalIPv6PrefixLen": 0,
        "DNSNames": null
      }
    }
  },
  "ImageManifestDescriptor": {
    "mediaType": "application/vnd.oci.image.manifest.v1+json",
    "digest": "sha256:39c3436527190561948236894c55b59fa58aa08d68d8867e703c8d5ab72a3593",
    "size": 2195,
    "platform": {
      "architecture": "arm64",
      "os": "linux"
    }
  }
}
            "#;
        let config = serde_json_lenient::from_str::<DockerInspect>(given_config).unwrap();

        let target_dir = get_remote_dir_from_config(&config, "/somepath/cli".to_string());

        assert!(target_dir.is_ok());
        assert_eq!(target_dir.unwrap(), "/workspaces/cli/".to_string());
    }

    #[test]
    fn should_inject_correct_parameters_into_dockerfile_extended() {
        let (feature_layers, container_user, remote_user) = (
            r#"RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./copilot-cli_0,target=/tmp/build-features-src/copilot-cli_0 \
    cp -ar /tmp/build-features-src/copilot-cli_0 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/copilot-cli_0 \
&& cd /tmp/dev-container-features/copilot-cli_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/copilot-cli_0
            "#.trim(),
            "container_user",
            "remote_user",
        );

        let dockerfile_extended =
            generate_dockerfile_extended(feature_layers, container_user, remote_user, None);

        assert_eq!(dockerfile_extended.trim(),
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder



FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source ./devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p /tmp/dev-container-features
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ /tmp/dev-container-features

RUN \
echo "_CONTAINER_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'container_user' || grep -E '^container_user|^[^:]*:[^:]*:container_user:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'remote_user' || grep -E '^remote_user|^[^:]*:[^:]*:remote_user:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env

RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./copilot-cli_0,target=/tmp/build-features-src/copilot-cli_0 \
    cp -ar /tmp/build-features-src/copilot-cli_0 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/copilot-cli_0 \
&& cd /tmp/dev-container-features/copilot-cli_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/copilot-cli_0

ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER
            "#.trim()
        );

        let dockerfile = r#"
ARG VARIANT="16-bullseye"
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT}

RUN mkdir -p /workspaces && chown node:node /workspaces

ARG USERNAME=node
USER $USERNAME

# Save command line history
RUN echo "hello, world""#
            .trim()
            .to_string();

        let dockerfile_extended = generate_dockerfile_extended(
            feature_layers,
            container_user,
            remote_user,
            Some(dockerfile),
        );

        assert_eq!(dockerfile_extended.trim(),
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

ARG VARIANT="16-bullseye"
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT} AS dev_container_auto_added_stage_label

RUN mkdir -p /workspaces && chown node:node /workspaces

ARG USERNAME=node
USER $USERNAME

# Save command line history
RUN echo "hello, world"

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source ./devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p /tmp/dev-container-features
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ /tmp/dev-container-features

RUN \
echo "_CONTAINER_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'container_user' || grep -E '^container_user|^[^:]*:[^:]*:container_user:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'remote_user' || grep -E '^remote_user|^[^:]*:[^:]*:remote_user:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env

RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./copilot-cli_0,target=/tmp/build-features-src/copilot-cli_0 \
    cp -ar /tmp/build-features-src/copilot-cli_0 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/copilot-cli_0 \
&& cd /tmp/dev-container-features/copilot-cli_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/copilot-cli_0

ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER
            "#.trim()
        );
    }

    #[test]
    fn should_create_docker_compose_command() {
        let docker_compose_files = vec![
            PathBuf::from("/var/test/docker-compose.yml"),
            PathBuf::from("/var/other/docker-compose2.yml"),
        ];

        let command = create_docker_compose_config_command(&docker_compose_files).unwrap();

        assert_eq!(command.get_program(), OsStr::new(docker_cli()));

        assert_eq!(
            command.get_args().collect::<Vec<&OsStr>>(),
            vec![
                OsStr::new("compose"),
                OsStr::new("-f"),
                OsStr::new("/var/test/docker-compose.yml"),
                OsStr::new("-f"),
                OsStr::new("/var/other/docker-compose2.yml"),
                OsStr::new("config"),
                OsStr::new("--format"),
                OsStr::new("json"),
            ]
        )
    }

    #[test]
    fn should_deserialize_docker_compose_config() {
        let given_config = r#"
{
    "name": "devcontainer",
    "networks": {
    "default": {
        "name": "devcontainer_default",
        "ipam": {}
    }
    },
    "services": {
        "app": {
            "command": [
            "sleep",
            "infinity"
            ],
            "depends_on": {
            "db": {
                "condition": "service_started",
                "restart": true,
                "required": true
            }
            },
            "entrypoint": null,
            "environment": {
            "POSTGRES_DB": "postgres",
            "POSTGRES_HOSTNAME": "localhost",
            "POSTGRES_PASSWORD": "postgres",
            "POSTGRES_PORT": "5432",
            "POSTGRES_USER": "postgres"
            },
            "image": "mcr.microsoft.com/devcontainers/rust:2-1-bookworm",
            "network_mode": "service:db",
            "volumes": [
            {
                "type": "bind",
                "source": "/Users/kylebarton/Source",
                "target": "/workspaces",
                "bind": {
                "create_host_path": true
                }
            }
            ]
        },
        "db": {
            "command": null,
            "entrypoint": null,
            "environment": {
            "POSTGRES_DB": "postgres",
            "POSTGRES_HOSTNAME": "localhost",
            "POSTGRES_PASSWORD": "postgres",
            "POSTGRES_PORT": "5432",
            "POSTGRES_USER": "postgres"
            },
            "image": "postgres:14.1",
            "networks": {
            "default": null
            },
            "restart": "unless-stopped",
            "volumes": [
            {
                "type": "volume",
                "source": "postgres-data",
                "target": "/var/lib/postgresql/data",
                "volume": {}
            }
            ]
        }
    },
    "volumes": {
    "postgres-data": {
        "name": "devcontainer_postgres-data"
    }
    }
}
            "#;

        let docker_compose_config: DockerComposeConfig =
            serde_json_lenient::from_str(given_config).unwrap();

        let expected_config = DockerComposeConfig {
            name: Some("devcontainer".to_string()),
            services: HashMap::from([
                (
                    "app".to_string(),
                    DockerComposeService {
                        image: Some(
                            "mcr.microsoft.com/devcontainers/rust:2-1-bookworm".to_string(),
                        ),
                        ..Default::default()
                    },
                ),
                (
                    "db".to_string(),
                    DockerComposeService {
                        image: Some("postgres:14.1".to_string()),
                        ..Default::default()
                    },
                ),
            ]),
            ..Default::default()
        };

        assert_eq!(docker_compose_config, expected_config);
    }

    #[test]
    fn should_find_primary_service_in_docker_compose() {
        // State where service not defined in dev container
        let given_dev_container = DevContainerManifest::default();
        let given_docker_compose_config = DockerComposeManifest {
            config: DockerComposeConfig {
                name: Some("devcontainers".to_string()),
                services: HashMap::new(),
                ..Default::default()
            },
            ..Default::default()
        };

        let bad_result = find_primary_service(&given_docker_compose_config, &given_dev_container);

        assert!(bad_result.is_err());

        // State where service defined in devcontainer, not found in DockerCompose config
        let given_dev_container = DevContainerManifest {
            config: DevContainer {
                service: Some("not_found_service".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let given_docker_compose_config = DockerComposeManifest {
            config: DockerComposeConfig {
                name: Some("devcontainers".to_string()),
                services: HashMap::new(),
                ..Default::default()
            },
            ..Default::default()
        };

        let bad_result = find_primary_service(&given_docker_compose_config, &given_dev_container);

        assert!(bad_result.is_err());
        // State where service defined in devcontainer and in DockerCompose config
        let given_dev_container = DevContainerManifest {
            config: DevContainer {
                service: Some("found_service".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let given_docker_compose_config = DockerComposeManifest {
            config: DockerComposeConfig {
                name: Some("devcontainers".to_string()),
                services: HashMap::from([(
                    "found_service".to_string(),
                    DockerComposeService {
                        ..Default::default()
                    },
                )]),
                ..Default::default()
            },
            ..Default::default()
        };

        let (service_name, _) =
            find_primary_service(&given_docker_compose_config, &given_dev_container).unwrap();

        assert_eq!(service_name, "found_service".to_string());
    }

    #[test]
    fn should_build_runtime_override() {
        let docker_image = DockerInspect {
            id: "id".to_string(),
            // Todo add some labels and make this test pass
            config: DockerInspectConfig {
                labels: DockerConfigLabels { metadata: None },
                image_user: None,
            },
            mounts: None,
        };

        let given_labels = vec![("label1", "label1val".to_string())];

        let resources = DockerBuildResources {
            image: docker_image,
            additional_mounts: vec![],
            privileged: false,
            entrypoints: vec![],
        };

        let runtime_override = build_runtime_override("app", resources, &given_labels).unwrap();

        // ugh how are we going to do labels
        let expected_runtime_override = DockerComposeConfig {
            name: None,
            services: HashMap::from([(
                "app".to_string(),
                DockerComposeService {
                    entrypoint: Some(vec![
                        "/bin/sh".to_string(),
                        "-c".to_string(),
                        "
echo Container started
trap \"exit 0\" 15
exec \"$@\"
while sleep 1 & wait $!; do :; done"
                            .to_string(),
                        "-".to_string(),
                    ]),
                    cap_add: Some(vec!["SYS_PTRACE".to_string()]),
                    security_opt: Some(vec!["seccomp=unconfined".to_string()]),
                    labels: Some(vec!["label1=label1val".to_string()]),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        };

        assert_eq!(runtime_override, expected_runtime_override)
    }

    // TODO turn this into a merged config test more broadly
    #[test]
    fn test_privileged() {
        let dev_container = DevContainer::default();

        let feature_manifests = vec![FeatureManifest::new(
            PathBuf::from("/"),
            DevContainerFeatureJson {
                _id: None,
                options: HashMap::new(),
                mounts: None,
                privileged: Some(true),
                entrypoint: None,
            },
        )];

        let privileged = dev_container.privileged.unwrap_or(false)
            || feature_manifests.iter().any(|f| f.privileged());

        assert!(privileged);
    }
}

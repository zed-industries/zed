use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

use regex::Regex;

use fs::Fs;
use http_client::HttpClient;
use util::{ResultExt, command::Command};

use crate::{
    DevContainerConfig, DevContainerContext,
    command_json::{CommandRunner, DefaultCommandRunner},
    devcontainer_api::{DevContainerError, DevContainerUp},
    devcontainer_json::{
        DevContainer, DevContainerBuildType, FeatureOptions, ForwardPort, MountDefinition,
        deserialize_devcontainer_json,
    },
    docker::{
        Docker, DockerClient, DockerComposeConfig, DockerComposeService, DockerComposeServiceBuild,
        DockerComposeServicePort, DockerComposeVolume, DockerInspect, DockerPs,
        get_remote_dir_from_config,
    },
    features::{DevContainerFeatureJson, FeatureManifest, parse_oci_feature_ref},
    get_oci_token,
    oci::{TokenResponse, download_oci_tarball, get_oci_manifest},
    safe_id_lower,
};

enum ConfigStatus {
    Deserialized(DevContainer),
    VariableParsed(DevContainer),
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeResources {
    files: Vec<PathBuf>,
    config: DockerComposeConfig,
}

struct DevContainerManifest {
    http_client: Arc<dyn HttpClient>,
    fs: Arc<dyn Fs>,
    docker_client: Arc<dyn DockerClient>,
    command_runner: Arc<dyn CommandRunner>,
    raw_config: String,
    config: ConfigStatus,
    local_environment: HashMap<String, String>,
    local_project_directory: PathBuf,
    config_directory: PathBuf,
    file_name: String,
    root_image: Option<DockerInspect>,
    features_build_info: Option<FeaturesBuildInfo>,
    features: Vec<FeatureManifest>,
}
const DEFAULT_REMOTE_PROJECT_DIR: &str = "/workspaces/";
impl DevContainerManifest {
    async fn new(
        context: &DevContainerContext,
        environment: HashMap<String, String>,
        docker_client: Arc<dyn DockerClient>,
        command_runner: Arc<dyn CommandRunner>,
        local_config: DevContainerConfig,
        local_project_path: &Path,
    ) -> Result<Self, DevContainerError> {
        let config_path = local_project_path.join(local_config.config_path.clone());
        log::debug!("parsing devcontainer json found in {:?}", &config_path);
        let devcontainer_contents = context.fs.load(&config_path).await.map_err(|e| {
            log::error!("Unable to read devcontainer contents: {e}");
            DevContainerError::DevContainerParseFailed
        })?;

        let devcontainer = deserialize_devcontainer_json(&devcontainer_contents)?;

        let devcontainer_directory = config_path.parent().ok_or_else(|| {
            log::error!("Dev container file should be in a directory");
            DevContainerError::NotInValidProject
        })?;
        let file_name = config_path
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or_else(|| {
                log::error!("Dev container file has no file name, or is invalid unicode");
                DevContainerError::DevContainerParseFailed
            })?;

        Ok(Self {
            fs: context.fs.clone(),
            http_client: context.http_client.clone(),
            docker_client,
            command_runner,
            raw_config: devcontainer_contents,
            config: ConfigStatus::Deserialized(devcontainer),
            local_project_directory: local_project_path.to_path_buf(),
            local_environment: environment,
            config_directory: devcontainer_directory.to_path_buf(),
            file_name: file_name.to_string(),
            root_image: None,
            features_build_info: None,
            features: Vec::new(),
        })
    }

    fn devcontainer_id(&self) -> String {
        let mut labels = self.identifying_labels();
        labels.sort_by_key(|(key, _)| *key);

        let mut hasher = DefaultHasher::new();
        for (key, value) in &labels {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }

        format!("{:016x}", hasher.finish())
    }

    fn identifying_labels(&self) -> Vec<(&str, String)> {
        let labels = vec![
            (
                "devcontainer.local_folder",
                (self.local_project_directory.display()).to_string(),
            ),
            (
                "devcontainer.config_file",
                (self.config_file().display()).to_string(),
            ),
        ];
        labels
    }

    fn parse_nonremote_vars_for_content(&self, content: &str) -> Result<String, DevContainerError> {
        let mut replaced_content = content
            .replace("${devcontainerId}", &self.devcontainer_id())
            .replace(
                "${containerWorkspaceFolderBasename}",
                &self.remote_workspace_base_name().unwrap_or_default(),
            )
            .replace(
                "${localWorkspaceFolderBasename}",
                &self.local_workspace_base_name()?,
            )
            .replace(
                "${containerWorkspaceFolder}",
                &self
                    .remote_workspace_folder()
                    .map(|path| path.display().to_string())
                    .unwrap_or_default()
                    .replace('\\', "/"),
            )
            .replace(
                "${localWorkspaceFolder}",
                &self.local_workspace_folder().replace('\\', "/"),
            );
        for (k, v) in &self.local_environment {
            let find = format!("${{localEnv:{k}}}");
            replaced_content = replaced_content.replace(&find, &v.replace('\\', "/"));
        }

        Ok(replaced_content)
    }

    fn parse_nonremote_vars(&mut self) -> Result<(), DevContainerError> {
        let replaced_content = self.parse_nonremote_vars_for_content(&self.raw_config)?;
        let parsed_config = deserialize_devcontainer_json(&replaced_content)?;

        self.config = ConfigStatus::VariableParsed(parsed_config);

        Ok(())
    }

    fn runtime_remote_env(
        &self,
        container_env: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, DevContainerError> {
        let mut merged_remote_env = container_env.clone();
        // HOME is user-specific, and we will often not run as the image user
        merged_remote_env.remove("HOME");
        if let Some(remote_env) = self.dev_container().remote_env.clone() {
            let mut raw = serde_json_lenient::to_string(&remote_env).map_err(|e| {
                log::error!(
                    "Unexpected error serializing dev container remote_env: {e} - {:?}",
                    remote_env
                );
                DevContainerError::DevContainerParseFailed
            })?;
            for (k, v) in container_env {
                raw = raw.replace(&format!("${{containerEnv:{k}}}"), v);
            }
            let reserialized: HashMap<String, String> = serde_json_lenient::from_str(&raw)
                .map_err(|e| {
                    log::error!(
                        "Unexpected error reserializing dev container remote env: {e} - {:?}",
                        &raw
                    );
                    DevContainerError::DevContainerParseFailed
                })?;
            for (k, v) in reserialized {
                merged_remote_env.insert(k, v);
            }
        }
        Ok(merged_remote_env)
    }

    fn config_file(&self) -> PathBuf {
        self.config_directory.join(&self.file_name)
    }

    fn dev_container(&self) -> &DevContainer {
        match &self.config {
            ConfigStatus::Deserialized(dev_container) => dev_container,
            ConfigStatus::VariableParsed(dev_container) => dev_container,
        }
    }

    async fn dockerfile_location(&self) -> Option<PathBuf> {
        let dev_container = self.dev_container();
        match dev_container.build_type() {
            DevContainerBuildType::Image(_) => None,
            DevContainerBuildType::Dockerfile(build) => {
                Some(self.config_directory.join(&build.dockerfile))
            }
            DevContainerBuildType::DockerCompose => {
                let Ok(docker_compose_manifest) = self.docker_compose_manifest().await else {
                    return None;
                };
                let Ok((_, main_service)) = find_primary_service(&docker_compose_manifest, self)
                else {
                    return None;
                };
                main_service
                    .build
                    .and_then(|b| b.dockerfile)
                    .map(|dockerfile| self.config_directory.join(dockerfile))
            }
            DevContainerBuildType::None => None,
        }
    }

    fn generate_features_image_tag(&self, dockerfile_build_path: String) -> String {
        let mut hasher = DefaultHasher::new();
        let prefix = match &self.dev_container().name {
            Some(name) => &safe_id_lower(name),
            None => "zed-dc",
        };
        let prefix = prefix.get(..6).unwrap_or(prefix);

        dockerfile_build_path.hash(&mut hasher);

        let hash = hasher.finish();
        format!("{}-{:x}-features", prefix, hash)
    }

    /// Gets the base image from the devcontainer with the following precedence:
    /// - The devcontainer image if an image is specified
    /// - The image sourced in the Dockerfile if a Dockerfile is specified
    /// - The image sourced in the docker-compose main service, if one is specified
    /// - The image sourced in the docker-compose main service dockerfile, if one is specified
    /// If no such image is available, return an error
    async fn get_base_image_from_config(&self) -> Result<String, DevContainerError> {
        match self.dev_container().build_type() {
            DevContainerBuildType::Image(image) => {
                return Ok(image);
            }
            DevContainerBuildType::Dockerfile(build) => {
                let dockerfile_contents = self.expanded_dockerfile_content().await?;
                return image_from_dockerfile(dockerfile_contents, &build.target).ok_or_else(
                    || {
                        log::error!("Unable to find base image in Dockerfile");
                        DevContainerError::DevContainerParseFailed
                    },
                );
            }
            DevContainerBuildType::DockerCompose => {
                let docker_compose_manifest = self.docker_compose_manifest().await?;
                let (_, main_service) = find_primary_service(&docker_compose_manifest, &self)?;

                if let Some(_) = main_service
                    .build
                    .as_ref()
                    .and_then(|b| b.dockerfile.as_ref())
                {
                    let dockerfile_contents = self.expanded_dockerfile_content().await?;
                    return image_from_dockerfile(
                        dockerfile_contents,
                        &main_service.build.as_ref().and_then(|b| b.target.clone()),
                    )
                    .ok_or_else(|| {
                        log::error!("Unable to find base image in Dockerfile");
                        DevContainerError::DevContainerParseFailed
                    });
                }
                if let Some(image) = &main_service.image {
                    return Ok(image.to_string());
                }

                log::error!("No valid base image found in docker-compose configuration");
                return Err(DevContainerError::DevContainerParseFailed);
            }
            DevContainerBuildType::None => {
                log::error!("Not a valid devcontainer config for build");
                return Err(DevContainerError::NotInValidProject);
            }
        }
    }

    async fn download_feature_and_dockerfile_resources(&mut self) -> Result<(), DevContainerError> {
        let dev_container = match &self.config {
            ConfigStatus::Deserialized(_) => {
                log::error!(
                    "Dev container has not yet been parsed for variable expansion. Cannot yet download resources"
                );
                return Err(DevContainerError::DevContainerParseFailed);
            }
            ConfigStatus::VariableParsed(dev_container) => dev_container,
        };
        let root_image_tag = self.get_base_image_from_config().await?;
        let root_image = self.docker_client.inspect(&root_image_tag).await?;

        let temp_base = std::env::temp_dir().join("devcontainer-zed");
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        let features_content_dir = temp_base.join(format!("container-features-{}", timestamp));
        let empty_context_dir = temp_base.join("empty-folder");

        self.fs
            .create_dir(&features_content_dir)
            .await
            .map_err(|e| {
                log::error!("Failed to create features content dir: {e}");
                DevContainerError::FilesystemError
            })?;

        self.fs.create_dir(&empty_context_dir).await.map_err(|e| {
            log::error!("Failed to create empty context dir: {e}");
            DevContainerError::FilesystemError
        })?;

        let dockerfile_path = features_content_dir.join("Dockerfile.extended");
        let image_tag =
            self.generate_features_image_tag(dockerfile_path.clone().display().to_string());

        let build_info = FeaturesBuildInfo {
            dockerfile_path,
            features_content_dir,
            empty_context_dir,
            build_image: dev_container.image.clone(),
            image_tag,
        };

        let features = match &dev_container.features {
            Some(features) => features,
            None => &HashMap::new(),
        };

        let container_user = get_container_user_from_config(&root_image, self)?;
        let remote_user = get_remote_user_from_config(&root_image, self)?;

        let builtin_env_content = format!(
            "_CONTAINER_USER={}\n_REMOTE_USER={}\n",
            container_user, remote_user
        );

        let builtin_env_path = build_info
            .features_content_dir
            .join("devcontainer-features.builtin.env");

        self.fs
            .write(&builtin_env_path, &builtin_env_content.as_bytes())
            .await
            .map_err(|e| {
                log::error!("Failed to write builtin env file: {e}");
                DevContainerError::FilesystemError
            })?;

        let ordered_features =
            resolve_feature_order(features, &dev_container.override_feature_install_order);

        for (index, (feature_ref, options)) in ordered_features.iter().enumerate() {
            if matches!(options, FeatureOptions::Bool(false)) {
                log::debug!(
                    "Feature '{}' is disabled (set to false), skipping",
                    feature_ref
                );
                continue;
            }

            let feature_id = extract_feature_id(feature_ref);
            let consecutive_id = format!("{}_{}", feature_id, index);
            let feature_dir = build_info.features_content_dir.join(&consecutive_id);

            self.fs.create_dir(&feature_dir).await.map_err(|e| {
                log::error!(
                    "Failed to create feature directory for {}: {e}",
                    feature_ref
                );
                DevContainerError::FilesystemError
            })?;

            let oci_ref = parse_oci_feature_ref(feature_ref).ok_or_else(|| {
                log::error!(
                    "Feature '{}' is not a supported OCI feature reference",
                    feature_ref
                );
                DevContainerError::DevContainerParseFailed
            })?;
            let TokenResponse { token } =
                get_oci_token(&oci_ref.registry, &oci_ref.path, &self.http_client)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to get OCI token for feature '{}': {e}", feature_ref);
                        DevContainerError::ResourceFetchFailed
                    })?;
            let manifest = get_oci_manifest(
                &oci_ref.registry,
                &oci_ref.path,
                &token,
                &self.http_client,
                &oci_ref.version,
                None,
            )
            .await
            .map_err(|e| {
                log::error!(
                    "Failed to fetch OCI manifest for feature '{}': {e}",
                    feature_ref
                );
                DevContainerError::ResourceFetchFailed
            })?;
            let digest = &manifest
                .layers
                .first()
                .ok_or_else(|| {
                    log::error!(
                        "OCI manifest for feature '{}' contains no layers",
                        feature_ref
                    );
                    DevContainerError::ResourceFetchFailed
                })?
                .digest;
            download_oci_tarball(
                &token,
                &oci_ref.registry,
                &oci_ref.path,
                digest,
                "application/vnd.devcontainers.layer.v1+tar",
                &feature_dir,
                &self.http_client,
                &self.fs,
                None,
            )
            .await?;

            let feature_json_path = &feature_dir.join("devcontainer-feature.json");
            if !self.fs.is_file(feature_json_path).await {
                let message = format!(
                    "No devcontainer-feature.json found in {:?}, no defaults to apply",
                    feature_json_path
                );
                log::error!("{}", &message);
                return Err(DevContainerError::ResourceFetchFailed);
            }

            let contents = self.fs.load(&feature_json_path).await.map_err(|e| {
                log::error!("error reading devcontainer-feature.json: {:?}", e);
                DevContainerError::FilesystemError
            })?;

            let contents_parsed = self.parse_nonremote_vars_for_content(&contents)?;

            let feature_json: DevContainerFeatureJson =
                serde_json_lenient::from_str(&contents_parsed).map_err(|e| {
                    log::error!("Failed to parse devcontainer-feature.json: {e}");
                    DevContainerError::ResourceFetchFailed
                })?;

            let feature_manifest = FeatureManifest::new(consecutive_id, feature_dir, feature_json);

            log::debug!("Downloaded OCI feature content for '{}'", feature_ref);

            let env_content = feature_manifest
                .write_feature_env(&self.fs, options)
                .await?;

            let wrapper_content = generate_install_wrapper(feature_ref, feature_id, &env_content)?;

            self.fs
                .write(
                    &feature_manifest
                        .file_path()
                        .join("devcontainer-features-install.sh"),
                    &wrapper_content.as_bytes(),
                )
                .await
                .map_err(|e| {
                    log::error!("Failed to write install wrapper for {}: {e}", feature_ref);
                    DevContainerError::FilesystemError
                })?;

            self.features.push(feature_manifest);
        }

        // --- Phase 3: Generate extended Dockerfile from the inflated manifests ---

        let is_compose = match dev_container.build_type() {
            DevContainerBuildType::DockerCompose => true,
            _ => false,
        };
        let use_buildkit = self.docker_client.supports_compose_buildkit() || !is_compose;

        let dockerfile_base_content = if let Some(location) = &self.dockerfile_location().await {
            self.fs.load(location).await.log_err()
        } else {
            None
        };

        let build_target = if is_compose {
            find_primary_service(&self.docker_compose_manifest().await?, self)?
                .1
                .build
                .and_then(|b| b.target)
        } else {
            dev_container.build.as_ref().and_then(|b| b.target.clone())
        };

        let dockerfile_content = dockerfile_base_content
            .map(|content| {
                dockerfile_inject_alias(
                    &content,
                    "dev_container_auto_added_stage_label",
                    build_target,
                )
            })
            .unwrap_or_default();

        let dockerfile_content = self.generate_dockerfile_extended(
            &container_user,
            &remote_user,
            dockerfile_content,
            use_buildkit,
        );

        self.fs
            .write(&build_info.dockerfile_path, &dockerfile_content.as_bytes())
            .await
            .map_err(|e| {
                log::error!("Failed to write Dockerfile.extended: {e}");
                DevContainerError::FilesystemError
            })?;

        log::debug!(
            "Features build resources written to {:?}",
            build_info.features_content_dir
        );

        self.root_image = Some(root_image);
        self.features_build_info = Some(build_info);

        Ok(())
    }

    fn generate_dockerfile_extended(
        &self,
        container_user: &str,
        remote_user: &str,
        dockerfile_content: String,
        use_buildkit: bool,
    ) -> String {
        #[cfg(not(target_os = "windows"))]
        let update_remote_user_uid = self.dev_container().update_remote_user_uid.unwrap_or(true);
        #[cfg(target_os = "windows")]
        let update_remote_user_uid = false;
        let feature_layers: String = self
            .features
            .iter()
            .map(|manifest| {
                manifest.generate_dockerfile_feature_layer(
                    use_buildkit,
                    FEATURES_CONTAINER_TEMP_DEST_FOLDER,
                )
            })
            .collect();

        let container_home_cmd = get_ent_passwd_shell_command(container_user);
        let remote_home_cmd = get_ent_passwd_shell_command(remote_user);

        let dest = FEATURES_CONTAINER_TEMP_DEST_FOLDER;

        let feature_content_source_stage = if use_buildkit {
            "".to_string()
        } else {
            "\nFROM dev_container_feature_content_temp as dev_containers_feature_content_source\n"
                .to_string()
        };

        let builtin_env_source_path = if use_buildkit {
            "./devcontainer-features.builtin.env"
        } else {
            "/tmp/build-features/devcontainer-features.builtin.env"
        };

        let mut extended_dockerfile = format!(
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

{dockerfile_content}
{feature_content_source_stage}
FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source {builtin_env_source_path} /tmp/build-features/
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
        );

        // If we're not adding a uid update layer, then we should add env vars to this layer instead
        if !update_remote_user_uid {
            extended_dockerfile = format!(
                r#"{extended_dockerfile}
# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${{PATH:-\3}}/g' /etc/profile || true
"#
            );

            for feature in &self.features {
                let container_env_layer = feature.generate_dockerfile_env();
                extended_dockerfile = format!("{extended_dockerfile}\n{container_env_layer}");
            }

            if let Some(env) = &self.dev_container().container_env {
                for (key, value) in env {
                    extended_dockerfile = format!("{extended_dockerfile}ENV {key}={value}\n");
                }
            }
        }

        extended_dockerfile
    }

    fn build_merged_resources(
        &self,
        base_image: DockerInspect,
    ) -> Result<DockerBuildResources, DevContainerError> {
        let dev_container = match &self.config {
            ConfigStatus::Deserialized(_) => {
                log::error!(
                    "Dev container has not yet been parsed for variable expansion. Cannot yet merge resources"
                );
                return Err(DevContainerError::DevContainerParseFailed);
            }
            ConfigStatus::VariableParsed(dev_container) => dev_container,
        };
        let mut mounts = dev_container.mounts.clone().unwrap_or(Vec::new());

        let mut feature_mounts = self.features.iter().flat_map(|f| f.mounts()).collect();

        mounts.append(&mut feature_mounts);

        let privileged = dev_container.privileged.unwrap_or(false)
            || self.features.iter().any(|f| f.privileged());

        let mut entrypoint_script_lines = vec![
            "echo Container started".to_string(),
            "trap \"exit 0\" 15".to_string(),
        ];

        for entrypoint in self.features.iter().filter_map(|f| f.entrypoint()) {
            entrypoint_script_lines.push(entrypoint.clone());
        }
        entrypoint_script_lines.append(&mut vec![
            "exec \"$@\"".to_string(),
            "while sleep 1 & wait $!; do :; done".to_string(),
        ]);

        Ok(DockerBuildResources {
            image: base_image,
            additional_mounts: mounts,
            privileged,
            entrypoint_script: entrypoint_script_lines.join("\n").trim().to_string(),
        })
    }

    async fn build_resources(&self) -> Result<DevContainerBuildResources, DevContainerError> {
        if let ConfigStatus::Deserialized(_) = &self.config {
            log::error!(
                "Dev container has not yet been parsed for variable expansion. Cannot yet build resources"
            );
            return Err(DevContainerError::DevContainerParseFailed);
        }
        let dev_container = self.dev_container();
        match dev_container.build_type() {
            DevContainerBuildType::Image(base_image) => {
                let built_docker_image = self.build_docker_image().await?;

                let built_docker_image = self
                    .update_remote_user_uid(built_docker_image, &base_image)
                    .await?;

                let resources = self.build_merged_resources(built_docker_image)?;
                Ok(DevContainerBuildResources::Docker(resources))
            }
            DevContainerBuildType::Dockerfile(_) => {
                let built_docker_image = self.build_docker_image().await?;
                let Some(features_build_info) = &self.features_build_info else {
                    log::error!(
                        "Can't attempt to build update UID dockerfile before initial docker build"
                    );
                    return Err(DevContainerError::DevContainerParseFailed);
                };
                let built_docker_image = self
                    .update_remote_user_uid(built_docker_image, &features_build_info.image_tag)
                    .await?;

                let resources = self.build_merged_resources(built_docker_image)?;
                Ok(DevContainerBuildResources::Docker(resources))
            }
            DevContainerBuildType::DockerCompose => {
                log::debug!("Using docker compose. Building extended compose files");
                let docker_compose_resources = self.build_and_extend_compose_files().await?;

                return Ok(DevContainerBuildResources::DockerCompose(
                    docker_compose_resources,
                ));
            }
            DevContainerBuildType::None => {
                return Err(DevContainerError::DevContainerParseFailed);
            }
        }
    }

    async fn run_dev_container(
        &self,
        build_resources: DevContainerBuildResources,
    ) -> Result<DevContainerUp, DevContainerError> {
        let ConfigStatus::VariableParsed(_) = &self.config else {
            log::error!(
                "Variables have not been parsed; cannot proceed with running the dev container"
            );
            return Err(DevContainerError::DevContainerParseFailed);
        };
        let running_container = match build_resources {
            DevContainerBuildResources::DockerCompose(resources) => {
                self.run_docker_compose(resources).await?
            }
            DevContainerBuildResources::Docker(resources) => {
                self.run_docker_image(resources).await?
            }
        };

        let remote_user = get_remote_user_from_config(&running_container, self)?;
        let remote_workspace_folder = get_remote_dir_from_config(
            &running_container,
            (&self.local_project_directory.display()).to_string(),
        )?;

        let remote_env = self.runtime_remote_env(&running_container.config.env_as_map()?)?;

        Ok(DevContainerUp {
            container_id: running_container.id,
            remote_user,
            remote_workspace_folder,
            extension_ids: self.extension_ids(),
            remote_env,
        })
    }

    async fn docker_compose_manifest(&self) -> Result<DockerComposeResources, DevContainerError> {
        let dev_container = match &self.config {
            ConfigStatus::Deserialized(_) => {
                log::error!(
                    "Dev container has not yet been parsed for variable expansion. Cannot yet get docker compose files"
                );
                return Err(DevContainerError::DevContainerParseFailed);
            }
            ConfigStatus::VariableParsed(dev_container) => dev_container,
        };
        let Some(docker_compose_files) = dev_container.docker_compose_file.clone() else {
            return Err(DevContainerError::DevContainerParseFailed);
        };
        let docker_compose_full_paths = docker_compose_files
            .iter()
            .map(|relative| self.config_directory.join(relative))
            .collect::<Vec<PathBuf>>();

        let Some(config) = self
            .docker_client
            .get_docker_compose_config(&docker_compose_full_paths)
            .await?
        else {
            log::error!("Output could not deserialize into DockerComposeConfig");
            return Err(DevContainerError::DevContainerParseFailed);
        };
        Ok(DockerComposeResources {
            files: docker_compose_full_paths,
            config,
        })
    }

    async fn build_and_extend_compose_files(
        &self,
    ) -> Result<DockerComposeResources, DevContainerError> {
        let dev_container = match &self.config {
            ConfigStatus::Deserialized(_) => {
                log::error!(
                    "Dev container has not yet been parsed for variable expansion. Cannot yet build from compose files"
                );
                return Err(DevContainerError::DevContainerParseFailed);
            }
            ConfigStatus::VariableParsed(dev_container) => dev_container,
        };

        let Some(features_build_info) = &self.features_build_info else {
            log::error!(
                "Cannot build and extend compose files: features build info is not yet constructed"
            );
            return Err(DevContainerError::DevContainerParseFailed);
        };
        let mut docker_compose_resources = self.docker_compose_manifest().await?;
        let supports_buildkit = self.docker_client.supports_compose_buildkit();

        let (main_service_name, main_service) =
            find_primary_service(&docker_compose_resources, self)?;
        let (built_service_image, built_service_image_tag) = if main_service
            .build
            .as_ref()
            .map(|b| b.dockerfile.as_ref())
            .is_some()
        {
            if !supports_buildkit {
                self.build_feature_content_image().await?;
            }

            let dockerfile_path = &features_build_info.dockerfile_path;

            let build_args = if !supports_buildkit {
                HashMap::from([
                    (
                        "_DEV_CONTAINERS_BASE_IMAGE".to_string(),
                        "dev_container_auto_added_stage_label".to_string(),
                    ),
                    ("_DEV_CONTAINERS_IMAGE_USER".to_string(), "root".to_string()),
                ])
            } else {
                HashMap::from([
                    ("BUILDKIT_INLINE_CACHE".to_string(), "1".to_string()),
                    (
                        "_DEV_CONTAINERS_BASE_IMAGE".to_string(),
                        "dev_container_auto_added_stage_label".to_string(),
                    ),
                    ("_DEV_CONTAINERS_IMAGE_USER".to_string(), "root".to_string()),
                ])
            };

            let additional_contexts = if !supports_buildkit {
                None
            } else {
                Some(HashMap::from([(
                    "dev_containers_feature_content_source".to_string(),
                    features_build_info
                        .features_content_dir
                        .display()
                        .to_string(),
                )]))
            };

            let build_override = DockerComposeConfig {
                name: None,
                services: HashMap::from([(
                    main_service_name.clone(),
                    DockerComposeService {
                        image: Some(features_build_info.image_tag.clone()),
                        entrypoint: None,
                        cap_add: None,
                        security_opt: None,
                        labels: None,
                        build: Some(DockerComposeServiceBuild {
                            context: Some(
                                main_service
                                    .build
                                    .as_ref()
                                    .and_then(|b| b.context.clone())
                                    .unwrap_or_else(|| {
                                        features_build_info.empty_context_dir.display().to_string()
                                    }),
                            ),
                            dockerfile: Some(dockerfile_path.display().to_string()),
                            target: Some("dev_containers_target_stage".to_string()),
                            args: Some(build_args),
                            additional_contexts,
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
                DevContainerError::DevContainerParseFailed
            })?;

            self.fs
                .write(&config_location, config_json.as_bytes())
                .await
                .map_err(|e| {
                    log::error!("Error writing the runtime override file: {e}");
                    DevContainerError::FilesystemError
                })?;

            docker_compose_resources.files.push(config_location);

            self.docker_client
                .docker_compose_build(&docker_compose_resources.files, &self.project_name())
                .await?;
            (
                self.docker_client
                    .inspect(&features_build_info.image_tag)
                    .await?,
                &features_build_info.image_tag,
            )
        } else if let Some(image) = &main_service.image {
            if dev_container
                .features
                .as_ref()
                .is_none_or(|features| features.is_empty())
            {
                (self.docker_client.inspect(image).await?, image)
            } else {
                if !supports_buildkit {
                    self.build_feature_content_image().await?;
                }

                let dockerfile_path = &features_build_info.dockerfile_path;

                let build_args = if !supports_buildkit {
                    HashMap::from([
                        ("_DEV_CONTAINERS_BASE_IMAGE".to_string(), image.clone()),
                        ("_DEV_CONTAINERS_IMAGE_USER".to_string(), "root".to_string()),
                    ])
                } else {
                    HashMap::from([
                        ("BUILDKIT_INLINE_CACHE".to_string(), "1".to_string()),
                        ("_DEV_CONTAINERS_BASE_IMAGE".to_string(), image.clone()),
                        ("_DEV_CONTAINERS_IMAGE_USER".to_string(), "root".to_string()),
                    ])
                };

                let additional_contexts = if !supports_buildkit {
                    None
                } else {
                    Some(HashMap::from([(
                        "dev_containers_feature_content_source".to_string(),
                        features_build_info
                            .features_content_dir
                            .display()
                            .to_string(),
                    )]))
                };

                let build_override = DockerComposeConfig {
                    name: None,
                    services: HashMap::from([(
                        main_service_name.clone(),
                        DockerComposeService {
                            image: Some(features_build_info.image_tag.clone()),
                            entrypoint: None,
                            cap_add: None,
                            security_opt: None,
                            labels: None,
                            build: Some(DockerComposeServiceBuild {
                                context: Some(
                                    features_build_info.empty_context_dir.display().to_string(),
                                ),
                                dockerfile: Some(dockerfile_path.display().to_string()),
                                target: Some("dev_containers_target_stage".to_string()),
                                args: Some(build_args),
                                additional_contexts,
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
                    DevContainerError::DevContainerParseFailed
                })?;

                self.fs
                    .write(&config_location, config_json.as_bytes())
                    .await
                    .map_err(|e| {
                        log::error!("Error writing the runtime override file: {e}");
                        DevContainerError::FilesystemError
                    })?;

                docker_compose_resources.files.push(config_location);

                self.docker_client
                    .docker_compose_build(&docker_compose_resources.files, &self.project_name())
                    .await?;

                (
                    self.docker_client
                        .inspect(&features_build_info.image_tag)
                        .await?,
                    &features_build_info.image_tag,
                )
            }
        } else {
            log::error!("Docker compose must have either image or dockerfile defined");
            return Err(DevContainerError::DevContainerParseFailed);
        };

        let built_service_image = self
            .update_remote_user_uid(built_service_image, built_service_image_tag)
            .await?;

        let resources = self.build_merged_resources(built_service_image)?;

        let network_mode = main_service.network_mode.as_ref();
        let network_mode_service = network_mode.and_then(|mode| mode.strip_prefix("service:"));
        let runtime_override_file = self
            .write_runtime_override_file(&main_service_name, network_mode_service, resources)
            .await?;

        docker_compose_resources.files.push(runtime_override_file);

        Ok(docker_compose_resources)
    }

    async fn write_runtime_override_file(
        &self,
        main_service_name: &str,
        network_mode_service: Option<&str>,
        resources: DockerBuildResources,
    ) -> Result<PathBuf, DevContainerError> {
        let config =
            self.build_runtime_override(main_service_name, network_mode_service, resources)?;
        let temp_base = std::env::temp_dir().join("devcontainer-zed");
        let config_location = temp_base.join("docker_compose_runtime.json");

        let config_json = serde_json_lenient::to_string(&config).map_err(|e| {
            log::error!("Error serializing docker compose runtime override: {e}");
            DevContainerError::DevContainerParseFailed
        })?;

        self.fs
            .write(&config_location, config_json.as_bytes())
            .await
            .map_err(|e| {
                log::error!("Error writing the runtime override file: {e}");
                DevContainerError::FilesystemError
            })?;

        Ok(config_location)
    }

    fn build_runtime_override(
        &self,
        main_service_name: &str,
        network_mode_service: Option<&str>,
        resources: DockerBuildResources,
    ) -> Result<DockerComposeConfig, DevContainerError> {
        let mut runtime_labels = HashMap::new();

        if let Some(metadata) = &resources.image.config.labels.metadata {
            let serialized_metadata = serde_json_lenient::to_string(metadata).map_err(|e| {
                log::error!("Error serializing docker image metadata: {e}");
                DevContainerError::ContainerNotValid(resources.image.id.clone())
            })?;

            runtime_labels.insert("devcontainer.metadata".to_string(), serialized_metadata);
        }

        for (k, v) in self.identifying_labels() {
            runtime_labels.insert(k.to_string(), v.to_string());
        }

        let config_volumes: HashMap<String, DockerComposeVolume> = resources
            .additional_mounts
            .iter()
            .filter_map(|mount| {
                if let Some(mount_type) = &mount.mount_type
                    && mount_type.to_lowercase() == "volume"
                    && let Some(source) = &mount.source
                {
                    Some((
                        source.clone(),
                        DockerComposeVolume {
                            name: source.clone(),
                        },
                    ))
                } else {
                    None
                }
            })
            .collect();

        let volumes: Vec<MountDefinition> = resources
            .additional_mounts
            .iter()
            .map(|v| MountDefinition {
                source: v.source.clone(),
                target: v.target.clone(),
                mount_type: v.mount_type.clone(),
            })
            .collect();

        let mut main_service = DockerComposeService {
            entrypoint: Some(vec![
                "/bin/sh".to_string(),
                "-c".to_string(),
                resources.entrypoint_script,
                "-".to_string(),
            ]),
            cap_add: Some(vec!["SYS_PTRACE".to_string()]),
            security_opt: Some(vec!["seccomp=unconfined".to_string()]),
            labels: Some(runtime_labels),
            volumes,
            privileged: Some(resources.privileged),
            ..Default::default()
        };
        // let mut extra_service_port_declarations: Vec<(String, DockerComposeService)> = Vec::new();
        let mut service_declarations: HashMap<String, DockerComposeService> = HashMap::new();
        if let Some(forward_ports) = &self.dev_container().forward_ports {
            let main_service_ports: Vec<String> = forward_ports
                .iter()
                .filter_map(|f| match f {
                    ForwardPort::Number(port) => Some(port.to_string()),
                    ForwardPort::String(port) => {
                        let parts: Vec<&str> = port.split(":").collect();
                        if parts.len() <= 1 {
                            Some(port.to_string())
                        } else if parts.len() == 2 {
                            if parts[0] == main_service_name {
                                Some(parts[1].to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
                .collect();
            for port in main_service_ports {
                // If the main service uses a different service's network bridge, append to that service's ports instead
                if let Some(network_service_name) = network_mode_service {
                    if let Some(service) = service_declarations.get_mut(network_service_name) {
                        service.ports.push(DockerComposeServicePort {
                            target: port.clone(),
                            published: port.clone(),
                            ..Default::default()
                        });
                    } else {
                        service_declarations.insert(
                            network_service_name.to_string(),
                            DockerComposeService {
                                ports: vec![DockerComposeServicePort {
                                    target: port.clone(),
                                    published: port.clone(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        );
                    }
                } else {
                    main_service.ports.push(DockerComposeServicePort {
                        target: port.clone(),
                        published: port.clone(),
                        ..Default::default()
                    });
                }
            }
            let other_service_ports: Vec<(&str, &str)> = forward_ports
                .iter()
                .filter_map(|f| match f {
                    ForwardPort::Number(_) => None,
                    ForwardPort::String(port) => {
                        let parts: Vec<&str> = port.split(":").collect();
                        if parts.len() != 2 {
                            None
                        } else {
                            if parts[0] == main_service_name {
                                None
                            } else {
                                Some((parts[0], parts[1]))
                            }
                        }
                    }
                })
                .collect();
            for (service_name, port) in other_service_ports {
                if let Some(service) = service_declarations.get_mut(service_name) {
                    service.ports.push(DockerComposeServicePort {
                        target: port.to_string(),
                        published: port.to_string(),
                        ..Default::default()
                    });
                } else {
                    service_declarations.insert(
                        service_name.to_string(),
                        DockerComposeService {
                            ports: vec![DockerComposeServicePort {
                                target: port.to_string(),
                                published: port.to_string(),
                                ..Default::default()
                            }],
                            ..Default::default()
                        },
                    );
                }
            }
        }

        service_declarations.insert(main_service_name.to_string(), main_service);
        let new_docker_compose_config = DockerComposeConfig {
            name: None,
            services: service_declarations,
            volumes: config_volumes,
        };

        Ok(new_docker_compose_config)
    }

    async fn build_docker_image(&self) -> Result<DockerInspect, DevContainerError> {
        let dev_container = match &self.config {
            ConfigStatus::Deserialized(_) => {
                log::error!(
                    "Dev container has not yet been parsed for variable expansion. Cannot yet build image"
                );
                return Err(DevContainerError::DevContainerParseFailed);
            }
            ConfigStatus::VariableParsed(dev_container) => dev_container,
        };

        match dev_container.build_type() {
            DevContainerBuildType::Image(image_tag) => {
                let base_image = self.docker_client.inspect(&image_tag).await?;
                if dev_container
                    .features
                    .as_ref()
                    .is_none_or(|features| features.is_empty())
                {
                    log::debug!("No features to add. Using base image");
                    return Ok(base_image);
                }
            }
            DevContainerBuildType::Dockerfile(_) => {}
            DevContainerBuildType::DockerCompose | DevContainerBuildType::None => {
                return Err(DevContainerError::DevContainerParseFailed);
            }
        };

        let mut command = self.create_docker_build()?;

        let output = self
            .command_runner
            .run_command(&mut command)
            .await
            .map_err(|e| {
                log::error!("Error building docker image: {e}");
                DevContainerError::CommandFailed(command.get_program().display().to_string())
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("docker buildx build failed: {stderr}");
            return Err(DevContainerError::CommandFailed(
                command.get_program().display().to_string(),
            ));
        }

        // After a successful build, inspect the newly tagged image to get its metadata
        let Some(features_build_info) = &self.features_build_info else {
            log::error!("Features build info expected, but not created");
            return Err(DevContainerError::DevContainerParseFailed);
        };
        let image = self
            .docker_client
            .inspect(&features_build_info.image_tag)
            .await?;

        Ok(image)
    }

    #[cfg(target_os = "windows")]
    async fn update_remote_user_uid(
        &self,
        image: DockerInspect,
        _base_image: &str,
    ) -> Result<DockerInspect, DevContainerError> {
        Ok(image)
    }
    #[cfg(not(target_os = "windows"))]
    async fn update_remote_user_uid(
        &self,
        image: DockerInspect,
        base_image: &str,
    ) -> Result<DockerInspect, DevContainerError> {
        let dev_container = self.dev_container();

        let Some(features_build_info) = &self.features_build_info else {
            return Ok(image);
        };

        // updateRemoteUserUID defaults to true per the devcontainers spec
        if dev_container.update_remote_user_uid == Some(false) {
            return Ok(image);
        }

        let remote_user = get_remote_user_from_config(&image, self)?;
        if remote_user == "root" || remote_user.chars().all(|c| c.is_ascii_digit()) {
            return Ok(image);
        }

        let image_user = image
            .config
            .image_user
            .as_deref()
            .unwrap_or("root")
            .to_string();

        let host_uid = Command::new("id")
            .arg("-u")
            .output()
            .await
            .map_err(|e| {
                log::error!("Failed to get host UID: {e}");
                DevContainerError::CommandFailed("id -u".to_string())
            })
            .and_then(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<u32>()
                    .map_err(|e| {
                        log::error!("Failed to parse host UID: {e}");
                        DevContainerError::CommandFailed("id -u".to_string())
                    })
            })?;

        let host_gid = Command::new("id")
            .arg("-g")
            .output()
            .await
            .map_err(|e| {
                log::error!("Failed to get host GID: {e}");
                DevContainerError::CommandFailed("id -g".to_string())
            })
            .and_then(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<u32>()
                    .map_err(|e| {
                        log::error!("Failed to parse host GID: {e}");
                        DevContainerError::CommandFailed("id -g".to_string())
                    })
            })?;

        let dockerfile_content = self.generate_update_uid_dockerfile();

        let dockerfile_path = features_build_info
            .features_content_dir
            .join("updateUID.Dockerfile");
        self.fs
            .write(&dockerfile_path, dockerfile_content.as_bytes())
            .await
            .map_err(|e| {
                log::error!("Failed to write updateUID Dockerfile: {e}");
                DevContainerError::FilesystemError
            })?;

        let updated_image_tag = features_build_info.image_tag.clone();

        let mut command = Command::new(self.docker_client.docker_cli());
        command.args(["build"]);
        command.args(["-f", &dockerfile_path.display().to_string()]);
        command.args(["-t", &updated_image_tag]);
        command.args(["--build-arg", &format!("BASE_IMAGE={}", base_image)]);
        command.args(["--build-arg", &format!("REMOTE_USER={}", remote_user)]);
        command.args(["--build-arg", &format!("NEW_UID={}", host_uid)]);
        command.args(["--build-arg", &format!("NEW_GID={}", host_gid)]);
        command.args(["--build-arg", &format!("IMAGE_USER={}", image_user)]);
        command.arg(features_build_info.empty_context_dir.display().to_string());

        let output = self
            .command_runner
            .run_command(&mut command)
            .await
            .map_err(|e| {
                log::error!("Error building UID update image: {e}");
                DevContainerError::CommandFailed(command.get_program().display().to_string())
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("UID update build failed: {stderr}");
            return Err(DevContainerError::CommandFailed(
                command.get_program().display().to_string(),
            ));
        }

        self.docker_client.inspect(&updated_image_tag).await
    }

    #[cfg(not(target_os = "windows"))]
    fn generate_update_uid_dockerfile(&self) -> String {
        let mut dockerfile = r#"ARG BASE_IMAGE
FROM $BASE_IMAGE

USER root

ARG REMOTE_USER
ARG NEW_UID
ARG NEW_GID
SHELL ["/bin/sh", "-c"]
RUN eval $(sed -n "s/${REMOTE_USER}:[^:]*:\([^:]*\):\([^:]*\):[^:]*:\([^:]*\).*/OLD_UID=\1;OLD_GID=\2;HOME_FOLDER=\3/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_UID}:.*/EXISTING_USER=\1/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_GID}:.*/EXISTING_GROUP=\1/p" /etc/group); \
	if [ -z "$OLD_UID" ]; then \
		echo "Remote user not found in /etc/passwd ($REMOTE_USER)."; \
	elif [ "$OLD_UID" = "$NEW_UID" -a "$OLD_GID" = "$NEW_GID" ]; then \
		echo "UIDs and GIDs are the same ($NEW_UID:$NEW_GID)."; \
	elif [ "$OLD_UID" != "$NEW_UID" -a -n "$EXISTING_USER" ]; then \
		echo "User with UID exists ($EXISTING_USER=$NEW_UID)."; \
	else \
		if [ "$OLD_GID" != "$NEW_GID" -a -n "$EXISTING_GROUP" ]; then \
			FREE_GID=65532; \
			while grep -q ":[^:]*:${FREE_GID}:" /etc/group; do FREE_GID=$((FREE_GID - 1)); done; \
			echo "Reassigning group $EXISTING_GROUP from GID $NEW_GID to $FREE_GID."; \
			sed -i -e "s/\(${EXISTING_GROUP}:[^:]*:\)${NEW_GID}:/\1${FREE_GID}:/" /etc/group; \
		fi; \
		echo "Updating UID:GID from $OLD_UID:$OLD_GID to $NEW_UID:$NEW_GID."; \
		sed -i -e "s/\(${REMOTE_USER}:[^:]*:\)[^:]*:[^:]*/\1${NEW_UID}:${NEW_GID}/" /etc/passwd; \
		if [ "$OLD_GID" != "$NEW_GID" ]; then \
			sed -i -e "s/\([^:]*:[^:]*:\)${OLD_GID}:/\1${NEW_GID}:/" /etc/group; \
		fi; \
		chown -R $NEW_UID:$NEW_GID $HOME_FOLDER; \
	fi;

ARG IMAGE_USER
USER $IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true
"#.to_string();
        for feature in &self.features {
            let container_env_layer = feature.generate_dockerfile_env();
            dockerfile = format!("{dockerfile}\n{container_env_layer}");
        }

        if let Some(env) = &self.dev_container().container_env {
            for (key, value) in env {
                dockerfile = format!("{dockerfile}ENV {key}={value}\n");
            }
        }
        dockerfile
    }

    async fn build_feature_content_image(&self) -> Result<(), DevContainerError> {
        let Some(features_build_info) = &self.features_build_info else {
            log::error!("Features build info not available for building feature content image");
            return Err(DevContainerError::DevContainerParseFailed);
        };
        let features_content_dir = &features_build_info.features_content_dir;

        let dockerfile_content = "FROM scratch\nCOPY . /tmp/build-features/\n";
        let dockerfile_path = features_content_dir.join("Dockerfile.feature-content");

        self.fs
            .write(&dockerfile_path, dockerfile_content.as_bytes())
            .await
            .map_err(|e| {
                log::error!("Failed to write feature content Dockerfile: {e}");
                DevContainerError::FilesystemError
            })?;

        let mut command = Command::new(self.docker_client.docker_cli());
        command.args([
            "build",
            "-t",
            "dev_container_feature_content_temp",
            "-f",
            &dockerfile_path.display().to_string(),
            &features_content_dir.display().to_string(),
        ]);

        let output = self
            .command_runner
            .run_command(&mut command)
            .await
            .map_err(|e| {
                log::error!("Error building feature content image: {e}");
                DevContainerError::CommandFailed(self.docker_client.docker_cli())
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Feature content image build failed: {stderr}");
            return Err(DevContainerError::CommandFailed(
                self.docker_client.docker_cli(),
            ));
        }

        Ok(())
    }

    fn create_docker_build(&self) -> Result<Command, DevContainerError> {
        let dev_container = match &self.config {
            ConfigStatus::Deserialized(_) => {
                log::error!(
                    "Dev container has not yet been parsed for variable expansion. Cannot yet proceed with docker build"
                );
                return Err(DevContainerError::DevContainerParseFailed);
            }
            ConfigStatus::VariableParsed(dev_container) => dev_container,
        };

        let Some(features_build_info) = &self.features_build_info else {
            log::error!(
                "Cannot create docker build command; features build info has not been constructed"
            );
            return Err(DevContainerError::DevContainerParseFailed);
        };
        let mut command = Command::new(self.docker_client.docker_cli());

        command.args(["buildx", "build"]);

        // --load is short for --output=docker, loading the built image into the local docker images
        command.arg("--load");

        // BuildKit build context: provides the features content directory as a named context
        // that the Dockerfile.extended can COPY from via `--from=dev_containers_feature_content_source`
        command.args([
            "--build-context",
            &format!(
                "dev_containers_feature_content_source={}",
                features_build_info.features_content_dir.display()
            ),
        ]);

        // Build args matching the CLI reference implementation's `getFeaturesBuildOptions`
        if let Some(build_image) = &features_build_info.build_image {
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
                self.root_image
                    .as_ref()
                    .and_then(|docker_image| docker_image.config.image_user.as_ref())
                    .unwrap_or(&"root".to_string())
            ),
        ]);

        command.args([
            "--build-arg",
            "_DEV_CONTAINERS_FEATURE_CONTENT_SOURCE=dev_container_feature_content_temp",
        ]);

        if let Some(args) = dev_container.build.as_ref().and_then(|b| b.args.as_ref()) {
            for (key, value) in args {
                command.args(["--build-arg", &format!("{}={}", key, value)]);
            }
        }

        command.args(["--target", "dev_containers_target_stage"]);

        command.args([
            "-f",
            &features_build_info.dockerfile_path.display().to_string(),
        ]);

        command.args(["-t", &features_build_info.image_tag]);

        if let DevContainerBuildType::Dockerfile(_) = dev_container.build_type() {
            command.arg(self.config_directory.display().to_string());
        } else {
            // Use an empty folder as the build context to avoid pulling in unneeded files.
            // The actual feature content is supplied via the BuildKit build context above.
            command.arg(features_build_info.empty_context_dir.display().to_string());
        }

        Ok(command)
    }

    async fn run_docker_compose(
        &self,
        resources: DockerComposeResources,
    ) -> Result<DockerInspect, DevContainerError> {
        let mut command = Command::new(self.docker_client.docker_cli());
        command.args(&["compose", "--project-name", &self.project_name()]);
        for docker_compose_file in resources.files {
            command.args(&["-f", &docker_compose_file.display().to_string()]);
        }
        command.args(&["up", "-d"]);

        let output = self
            .command_runner
            .run_command(&mut command)
            .await
            .map_err(|e| {
                log::error!("Error running docker compose up: {e}");
                DevContainerError::CommandFailed(command.get_program().display().to_string())
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from docker compose up: {}", stderr);
            return Err(DevContainerError::CommandFailed(
                command.get_program().display().to_string(),
            ));
        }

        if let Some(docker_ps) = self.check_for_existing_container().await? {
            log::debug!("Found newly created dev container");
            return self.docker_client.inspect(&docker_ps.id).await;
        }

        log::error!("Could not find existing container after docker compose up");

        Err(DevContainerError::DevContainerParseFailed)
    }

    async fn run_docker_image(
        &self,
        build_resources: DockerBuildResources,
    ) -> Result<DockerInspect, DevContainerError> {
        let mut docker_run_command = self.create_docker_run_command(build_resources)?;

        let output = self
            .command_runner
            .run_command(&mut docker_run_command)
            .await
            .map_err(|e| {
                log::error!("Error running docker run: {e}");
                DevContainerError::CommandFailed(
                    docker_run_command.get_program().display().to_string(),
                )
            })?;

        if !output.status.success() {
            let std_err = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from docker run. StdErr: {std_err}");
            return Err(DevContainerError::CommandFailed(
                docker_run_command.get_program().display().to_string(),
            ));
        }

        log::debug!("Checking for container that was started");
        let Some(docker_ps) = self.check_for_existing_container().await? else {
            log::error!("Could not locate container just created");
            return Err(DevContainerError::DevContainerParseFailed);
        };
        self.docker_client.inspect(&docker_ps.id).await
    }

    fn local_workspace_folder(&self) -> String {
        self.local_project_directory.display().to_string()
    }
    fn local_workspace_base_name(&self) -> Result<String, DevContainerError> {
        self.local_project_directory
            .file_name()
            .map(|f| f.display().to_string())
            .ok_or(DevContainerError::DevContainerParseFailed)
    }

    fn remote_workspace_folder(&self) -> Result<PathBuf, DevContainerError> {
        self.dev_container()
            .workspace_folder
            .as_ref()
            .map(|folder| PathBuf::from(folder))
            .or(Some(
                PathBuf::from(DEFAULT_REMOTE_PROJECT_DIR).join(self.local_workspace_base_name()?),
            ))
            .ok_or(DevContainerError::DevContainerParseFailed)
    }
    fn remote_workspace_base_name(&self) -> Result<String, DevContainerError> {
        self.remote_workspace_folder().and_then(|f| {
            f.file_name()
                .map(|file_name| file_name.display().to_string())
                .ok_or(DevContainerError::DevContainerParseFailed)
        })
    }

    fn remote_workspace_mount(&self) -> Result<MountDefinition, DevContainerError> {
        if let Some(mount) = &self.dev_container().workspace_mount {
            return Ok(mount.clone());
        }
        let Some(project_directory_name) = self.local_project_directory.file_name() else {
            return Err(DevContainerError::DevContainerParseFailed);
        };

        Ok(MountDefinition {
            source: Some(self.local_workspace_folder()),
            target: format!("/workspaces/{}", project_directory_name.display()),
            mount_type: None,
        })
    }

    fn create_docker_run_command(
        &self,
        build_resources: DockerBuildResources,
    ) -> Result<Command, DevContainerError> {
        let remote_workspace_mount = self.remote_workspace_mount()?;

        let docker_cli = self.docker_client.docker_cli();
        let mut command = Command::new(&docker_cli);

        command.arg("run");

        if build_resources.privileged {
            command.arg("--privileged");
        }

        if &docker_cli == "podman" {
            command.args(&["--security-opt", "label=disable", "--userns=keep-id"]);
        }

        command.arg("--sig-proxy=false");
        command.arg("-d");
        command.arg("--mount");
        command.arg(remote_workspace_mount.to_string());

        for mount in &build_resources.additional_mounts {
            command.arg("--mount");
            command.arg(mount.to_string());
        }

        for (key, val) in self.identifying_labels() {
            command.arg("-l");
            command.arg(format!("{}={}", key, val));
        }

        if let Some(metadata) = &build_resources.image.config.labels.metadata {
            let serialized_metadata = serde_json_lenient::to_string(metadata).map_err(|e| {
                log::error!("Problem serializing image metadata: {e}");
                DevContainerError::ContainerNotValid(build_resources.image.id.clone())
            })?;
            command.arg("-l");
            command.arg(format!(
                "{}={}",
                "devcontainer.metadata", serialized_metadata
            ));
        }

        if let Some(forward_ports) = &self.dev_container().forward_ports {
            for port in forward_ports {
                if let ForwardPort::Number(port_number) = port {
                    command.arg("-p");
                    command.arg(format!("{port_number}:{port_number}"));
                }
            }
        }
        for app_port in &self.dev_container().app_port {
            command.arg("-p");
            command.arg(app_port);
        }

        command.arg("--entrypoint");
        command.arg("/bin/sh");
        command.arg(&build_resources.image.id);
        command.arg("-c");

        command.arg(build_resources.entrypoint_script);
        command.arg("-");

        Ok(command)
    }

    fn extension_ids(&self) -> Vec<String> {
        self.dev_container()
            .customizations
            .as_ref()
            .map(|c| c.zed.extensions.clone())
            .unwrap_or_default()
    }

    async fn build_and_run(&mut self) -> Result<DevContainerUp, DevContainerError> {
        self.run_initialize_commands().await?;

        self.download_feature_and_dockerfile_resources().await?;

        let build_resources = self.build_resources().await?;

        let devcontainer_up = self.run_dev_container(build_resources).await?;

        self.run_remote_scripts(&devcontainer_up, true).await?;

        Ok(devcontainer_up)
    }

    async fn run_remote_scripts(
        &self,
        devcontainer_up: &DevContainerUp,
        new_container: bool,
    ) -> Result<(), DevContainerError> {
        let ConfigStatus::VariableParsed(config) = &self.config else {
            log::error!("Config not yet parsed, cannot proceed with remote scripts");
            return Err(DevContainerError::DevContainerScriptsFailed);
        };
        let remote_folder = self.remote_workspace_folder()?.display().to_string();

        if new_container {
            if let Some(on_create_command) = &config.on_create_command {
                for (command_name, command) in on_create_command.script_commands() {
                    log::debug!("Running on create command {command_name}");
                    self.docker_client
                        .run_docker_exec(
                            &devcontainer_up.container_id,
                            &remote_folder,
                            "root",
                            &devcontainer_up.remote_env,
                            command,
                        )
                        .await?;
                }
            }
            if let Some(update_content_command) = &config.update_content_command {
                for (command_name, command) in update_content_command.script_commands() {
                    log::debug!("Running update content command {command_name}");
                    self.docker_client
                        .run_docker_exec(
                            &devcontainer_up.container_id,
                            &remote_folder,
                            "root",
                            &devcontainer_up.remote_env,
                            command,
                        )
                        .await?;
                }
            }

            if let Some(post_create_command) = &config.post_create_command {
                for (command_name, command) in post_create_command.script_commands() {
                    log::debug!("Running post create command {command_name}");
                    self.docker_client
                        .run_docker_exec(
                            &devcontainer_up.container_id,
                            &remote_folder,
                            &devcontainer_up.remote_user,
                            &devcontainer_up.remote_env,
                            command,
                        )
                        .await?;
                }
            }
            if let Some(post_start_command) = &config.post_start_command {
                for (command_name, command) in post_start_command.script_commands() {
                    log::debug!("Running post start command {command_name}");
                    self.docker_client
                        .run_docker_exec(
                            &devcontainer_up.container_id,
                            &remote_folder,
                            &devcontainer_up.remote_user,
                            &devcontainer_up.remote_env,
                            command,
                        )
                        .await?;
                }
            }
        }
        if let Some(post_attach_command) = &config.post_attach_command {
            for (command_name, command) in post_attach_command.script_commands() {
                log::debug!("Running post attach command {command_name}");
                self.docker_client
                    .run_docker_exec(
                        &devcontainer_up.container_id,
                        &remote_folder,
                        &devcontainer_up.remote_user,
                        &devcontainer_up.remote_env,
                        command,
                    )
                    .await?;
            }
        }

        Ok(())
    }

    async fn run_initialize_commands(&self) -> Result<(), DevContainerError> {
        let ConfigStatus::VariableParsed(config) = &self.config else {
            log::error!("Config not yet parsed, cannot proceed with initializeCommand");
            return Err(DevContainerError::DevContainerParseFailed);
        };

        if let Some(initialize_command) = &config.initialize_command {
            log::debug!("Running initialize command");
            initialize_command
                .run(&self.command_runner, &self.local_project_directory)
                .await
        } else {
            log::warn!("No initialize command found");
            Ok(())
        }
    }

    async fn check_for_existing_devcontainer(
        &self,
    ) -> Result<Option<DevContainerUp>, DevContainerError> {
        if let Some(docker_ps) = self.check_for_existing_container().await? {
            log::debug!("Dev container already found. Proceeding with it");

            let docker_inspect = self.docker_client.inspect(&docker_ps.id).await?;

            if !docker_inspect.is_running() {
                log::debug!("Container not running. Will attempt to start, and then proceed");
                self.docker_client.start_container(&docker_ps.id).await?;
            }

            let remote_user = get_remote_user_from_config(&docker_inspect, self)?;

            let remote_folder = get_remote_dir_from_config(
                &docker_inspect,
                (&self.local_project_directory.display()).to_string(),
            )?;

            let remote_env = self.runtime_remote_env(&docker_inspect.config.env_as_map()?)?;

            let dev_container_up = DevContainerUp {
                container_id: docker_ps.id,
                remote_user: remote_user,
                remote_workspace_folder: remote_folder,
                extension_ids: self.extension_ids(),
                remote_env,
            };

            self.run_remote_scripts(&dev_container_up, false).await?;

            Ok(Some(dev_container_up))
        } else {
            log::debug!("Existing container not found.");

            Ok(None)
        }
    }

    async fn check_for_existing_container(&self) -> Result<Option<DockerPs>, DevContainerError> {
        self.docker_client
            .find_process_by_filters(
                self.identifying_labels()
                    .iter()
                    .map(|(k, v)| format!("label={k}={v}"))
                    .collect(),
            )
            .await
    }

    fn project_name(&self) -> String {
        if let Some(name) = &self.dev_container().name {
            safe_id_lower(name)
        } else {
            let alternate_name = &self
                .local_workspace_base_name()
                .unwrap_or(self.local_workspace_folder());
            safe_id_lower(alternate_name)
        }
    }

    async fn expanded_dockerfile_content(&self) -> Result<String, DevContainerError> {
        let Some(dockerfile_path) = self.dockerfile_location().await else {
            log::error!("Tried to expand dockerfile for an image-type config");
            return Err(DevContainerError::DevContainerParseFailed);
        };

        let devcontainer_args = self
            .dev_container()
            .build
            .as_ref()
            .and_then(|b| b.args.clone())
            .unwrap_or_default();
        let contents = self.fs.load(&dockerfile_path).await.map_err(|e| {
            log::error!("Failed to load Dockerfile: {e}");
            DevContainerError::FilesystemError
        })?;
        let mut parsed_lines: Vec<String> = Vec::new();
        let mut inline_args: Vec<(String, String)> = Vec::new();
        let key_regex = Regex::new(r"(?:^|\s)(\w+)=").expect("valid regex");

        for line in contents.lines() {
            let mut parsed_line = line.to_string();
            // Replace from devcontainer args first, since they take precedence
            for (key, value) in &devcontainer_args {
                parsed_line = parsed_line.replace(&format!("${{{key}}}"), value)
            }
            for (key, value) in &inline_args {
                parsed_line = parsed_line.replace(&format!("${{{key}}}"), value);
            }
            if let Some(arg_directives) = parsed_line.strip_prefix("ARG ") {
                let trimmed = arg_directives.trim();
                let key_matches: Vec<_> = key_regex.captures_iter(trimmed).collect();
                for (i, captures) in key_matches.iter().enumerate() {
                    let key = captures[1].to_string();
                    // Insert the devcontainer overrides here if needed
                    let value_start = captures.get(0).expect("full match").end();
                    let value_end = if i + 1 < key_matches.len() {
                        key_matches[i + 1].get(0).expect("full match").start()
                    } else {
                        trimmed.len()
                    };
                    let raw_value = trimmed[value_start..value_end].trim();
                    let value = if raw_value.starts_with('"')
                        && raw_value.ends_with('"')
                        && raw_value.len() > 1
                    {
                        &raw_value[1..raw_value.len() - 1]
                    } else {
                        raw_value
                    };
                    inline_args.push((key, value.to_string()));
                }
            }
            parsed_lines.push(parsed_line);
        }

        Ok(parsed_lines.join("\n"))
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

pub(crate) async fn read_devcontainer_configuration(
    config: DevContainerConfig,
    context: &DevContainerContext,
    environment: HashMap<String, String>,
) -> Result<DevContainer, DevContainerError> {
    let docker = if context.use_podman {
        Docker::new("podman")
    } else {
        Docker::new("docker")
    };
    let mut dev_container = DevContainerManifest::new(
        context,
        environment,
        Arc::new(docker),
        Arc::new(DefaultCommandRunner::new()),
        config,
        &context.project_directory.as_ref(),
    )
    .await?;
    dev_container.parse_nonremote_vars()?;
    Ok(dev_container.dev_container().clone())
}

pub(crate) async fn spawn_dev_container(
    context: &DevContainerContext,
    environment: HashMap<String, String>,
    config: DevContainerConfig,
    local_project_path: &Path,
) -> Result<DevContainerUp, DevContainerError> {
    let docker = if context.use_podman {
        Docker::new("podman")
    } else {
        Docker::new("docker")
    };
    let mut devcontainer_manifest = DevContainerManifest::new(
        context,
        environment,
        Arc::new(docker),
        Arc::new(DefaultCommandRunner::new()),
        config,
        local_project_path,
    )
    .await?;

    devcontainer_manifest.parse_nonremote_vars()?;

    log::debug!("Checking for existing container");
    if let Some(devcontainer) = devcontainer_manifest
        .check_for_existing_devcontainer()
        .await?
    {
        Ok(devcontainer)
    } else {
        log::debug!("Existing container not found. Building");

        devcontainer_manifest.build_and_run().await
    }
}

#[derive(Debug)]
struct DockerBuildResources {
    image: DockerInspect,
    additional_mounts: Vec<MountDefinition>,
    privileged: bool,
    entrypoint_script: String,
}

#[derive(Debug)]
enum DevContainerBuildResources {
    DockerCompose(DockerComposeResources),
    Docker(DockerBuildResources),
}

fn find_primary_service(
    docker_compose: &DockerComposeResources,
    devcontainer: &DevContainerManifest,
) -> Result<(String, DockerComposeService), DevContainerError> {
    let Some(service_name) = &devcontainer.dev_container().service else {
        return Err(DevContainerError::DevContainerParseFailed);
    };

    match docker_compose.config.services.get(service_name) {
        Some(service) => Ok((service_name.clone(), service.clone())),
        None => Err(DevContainerError::DevContainerParseFailed),
    }
}

/// Destination folder inside the container where feature content is staged during build.
/// Mirrors the CLI's `FEATURES_CONTAINER_TEMP_DEST_FOLDER`.
const FEATURES_CONTAINER_TEMP_DEST_FOLDER: &str = "/tmp/dev-container-features";

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
fn generate_install_wrapper(
    feature_ref: &str,
    feature_id: &str,
    env_variables: &str,
) -> Result<String, DevContainerError> {
    let escaped_id = shlex::try_quote(feature_ref).map_err(|e| {
        log::error!("Error escaping feature ref {feature_ref}: {e}");
        DevContainerError::DevContainerParseFailed
    })?;
    let escaped_name = shlex::try_quote(feature_id).map_err(|e| {
        log::error!("Error escaping feature {feature_id}: {e}");
        DevContainerError::DevContainerParseFailed
    })?;
    let options_indented: String = env_variables
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| format!("    {}", l))
        .collect::<Vec<_>>()
        .join("\n");
    let escaped_options = shlex::try_quote(&options_indented).map_err(|e| {
        log::error!("Error escaping options {options_indented}: {e}");
        DevContainerError::DevContainerParseFailed
    })?;

    let script = format!(
        r#"#!/bin/sh
set -e

on_exit () {{
    [ $? -eq 0 ] && exit
    echo 'ERROR: Feature "{escaped_name}" ({escaped_id}) failed to install!'
}}

trap on_exit EXIT

echo ===========================================================================
echo 'Feature       : {escaped_name}'
echo 'Id            : {escaped_id}'
echo 'Options       :'
echo {escaped_options}
echo ===========================================================================

set -a
. ../devcontainer-features.builtin.env
. ./devcontainer-features.env
set +a

chmod +x ./install.sh
./install.sh
"#
    );

    Ok(script)
}

fn dockerfile_inject_alias(
    dockerfile_content: &str,
    alias: &str,
    build_target: Option<String>,
) -> String {
    match image_from_dockerfile(dockerfile_content.to_string(), &build_target) {
        Some(target) => format!(
            r#"{dockerfile_content}
FROM {target} AS {alias}"#
        ),
        None => dockerfile_content.to_string(),
    }
}

fn image_from_dockerfile(dockerfile_contents: String, target: &Option<String>) -> Option<String> {
    dockerfile_contents
        .lines()
        .filter(|line| line.starts_with("FROM"))
        .rfind(|from_line| match &target {
            Some(target) => {
                let parts = from_line.split(' ').collect::<Vec<&str>>();
                if parts.len() >= 3
                    && parts.get(parts.len() - 2).unwrap_or(&"").to_lowercase() == "as"
                {
                    parts.last().unwrap_or(&"").to_lowercase() == target.to_lowercase()
                } else {
                    false
                }
            }
            None => true,
        })
        .and_then(|from_line| {
            from_line
                .split(' ')
                .collect::<Vec<&str>>()
                .get(1)
                .map(|s| s.to_string())
        })
}

// Container user things
// This should come from spec - see the docs
fn get_remote_user_from_config(
    docker_config: &DockerInspect,
    devcontainer: &DevContainerManifest,
) -> Result<String, DevContainerError> {
    if let DevContainer {
        remote_user: Some(user),
        ..
    } = &devcontainer.dev_container()
    {
        return Ok(user.clone());
    }
    if let Some(metadata) = &docker_config.config.labels.metadata {
        for metadatum in metadata {
            if let Some(remote_user) = metadatum.get("remoteUser") {
                if let Some(remote_user_str) = remote_user.as_str() {
                    return Ok(remote_user_str.to_string());
                }
            }
        }
    }
    if let Some(image_user) = &docker_config.config.image_user {
        if !image_user.is_empty() {
            return Ok(image_user.to_string());
        }
    }
    Ok("root".to_string())
}

// This should come from spec - see the docs
fn get_container_user_from_config(
    docker_config: &DockerInspect,
    devcontainer: &DevContainerManifest,
) -> Result<String, DevContainerError> {
    if let Some(user) = &devcontainer.dev_container().container_user {
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

    Ok("root".to_string())
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        ffi::OsStr,
        path::PathBuf,
        process::{ExitStatus, Output},
        sync::{Arc, Mutex},
    };

    use async_trait::async_trait;
    use fs::{FakeFs, Fs};
    use gpui::{AppContext, TestAppContext};
    use http_client::{AsyncBody, FakeHttpClient, HttpClient};
    use project::{
        ProjectEnvironment,
        worktree_store::{WorktreeIdCounter, WorktreeStore},
    };
    use serde_json_lenient::Value;
    use util::{command::Command, paths::SanitizedPath};

    #[cfg(not(target_os = "windows"))]
    use crate::docker::DockerComposeServicePort;
    use crate::{
        DevContainerConfig, DevContainerContext,
        command_json::CommandRunner,
        devcontainer_api::DevContainerError,
        devcontainer_json::MountDefinition,
        devcontainer_manifest::{
            ConfigStatus, DevContainerManifest, DockerBuildResources, DockerComposeResources,
            DockerInspect, extract_feature_id, find_primary_service, get_remote_user_from_config,
            image_from_dockerfile,
        },
        docker::{
            DockerClient, DockerComposeConfig, DockerComposeService, DockerComposeServiceBuild,
            DockerComposeVolume, DockerConfigLabels, DockerInspectConfig, DockerInspectMount,
            DockerPs,
        },
        oci::TokenResponse,
    };
    const TEST_PROJECT_PATH: &str = "/path/to/local/project";

    async fn build_tarball(content: Vec<(&str, &str)>) -> Vec<u8> {
        let buffer = futures::io::Cursor::new(Vec::new());
        let mut builder = async_tar::Builder::new(buffer);
        for (file_name, content) in content {
            if content.is_empty() {
                let mut header = async_tar::Header::new_gnu();
                header.set_size(0);
                header.set_mode(0o755);
                header.set_entry_type(async_tar::EntryType::Directory);
                header.set_cksum();
                builder
                    .append_data(&mut header, file_name, &[] as &[u8])
                    .await
                    .unwrap();
            } else {
                let data = content.as_bytes();
                let mut header = async_tar::Header::new_gnu();
                header.set_size(data.len() as u64);
                header.set_mode(0o755);
                header.set_entry_type(async_tar::EntryType::Regular);
                header.set_cksum();
                builder
                    .append_data(&mut header, file_name, data)
                    .await
                    .unwrap();
            }
        }
        let buffer = builder.into_inner().await.unwrap();
        buffer.into_inner()
    }

    fn test_project_filename() -> String {
        PathBuf::from(TEST_PROJECT_PATH)
            .file_name()
            .expect("is valid")
            .display()
            .to_string()
    }

    async fn init_devcontainer_config(
        fs: &Arc<FakeFs>,
        devcontainer_contents: &str,
    ) -> DevContainerConfig {
        fs.insert_tree(
            format!("{TEST_PROJECT_PATH}/.devcontainer"),
            serde_json::json!({"devcontainer.json": devcontainer_contents}),
        )
        .await;

        DevContainerConfig::default_config()
    }

    struct TestDependencies {
        fs: Arc<FakeFs>,
        _http_client: Arc<dyn HttpClient>,
        docker: Arc<FakeDocker>,
        command_runner: Arc<TestCommandRunner>,
    }

    async fn init_default_devcontainer_manifest(
        cx: &mut TestAppContext,
        devcontainer_contents: &str,
    ) -> Result<(TestDependencies, DevContainerManifest), DevContainerError> {
        let fs = FakeFs::new(cx.executor());
        let http_client = fake_http_client();
        let command_runner = Arc::new(TestCommandRunner::new());
        let docker = Arc::new(FakeDocker::new());
        let environment = HashMap::new();

        init_devcontainer_manifest(
            cx,
            fs,
            http_client,
            docker,
            command_runner,
            environment,
            devcontainer_contents,
        )
        .await
    }

    async fn init_devcontainer_manifest(
        cx: &mut TestAppContext,
        fs: Arc<FakeFs>,
        http_client: Arc<dyn HttpClient>,
        docker_client: Arc<FakeDocker>,
        command_runner: Arc<TestCommandRunner>,
        environment: HashMap<String, String>,
        devcontainer_contents: &str,
    ) -> Result<(TestDependencies, DevContainerManifest), DevContainerError> {
        let local_config = init_devcontainer_config(&fs, devcontainer_contents).await;
        let project_path = SanitizedPath::new_arc(&PathBuf::from(TEST_PROJECT_PATH));
        let worktree_store =
            cx.new(|_cx| WorktreeStore::local(false, fs.clone(), WorktreeIdCounter::default()));
        let project_environment =
            cx.new(|cx| ProjectEnvironment::new(None, worktree_store.downgrade(), None, false, cx));

        let context = DevContainerContext {
            project_directory: SanitizedPath::cast_arc(project_path),
            use_podman: false,
            fs: fs.clone(),
            http_client: http_client.clone(),
            environment: project_environment.downgrade(),
        };

        let test_dependencies = TestDependencies {
            fs: fs.clone(),
            _http_client: http_client.clone(),
            docker: docker_client.clone(),
            command_runner: command_runner.clone(),
        };
        let manifest = DevContainerManifest::new(
            &context,
            environment,
            docker_client,
            command_runner,
            local_config,
            &PathBuf::from(TEST_PROJECT_PATH),
        )
        .await?;

        Ok((test_dependencies, manifest))
    }

    #[gpui::test]
    async fn should_get_remote_user_from_devcontainer_if_available(cx: &mut TestAppContext) {
        let (_, devcontainer_manifest) = init_default_devcontainer_manifest(
            cx,
            r#"
// These are some external comments. serde_lenient should handle them
{
    // These are some internal comments
    "image": "image",
    "remoteUser": "root",
}
            "#,
        )
        .await
        .unwrap();

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
                env: Vec::new(),
            },
            mounts: None,
            state: None,
        };

        let remote_user =
            get_remote_user_from_config(&given_docker_config, &devcontainer_manifest).unwrap();

        assert_eq!(remote_user, "root".to_string())
    }

    #[gpui::test]
    async fn should_get_remote_user_from_docker_config(cx: &mut TestAppContext) {
        let (_, devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, "{}").await.unwrap();
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
                env: Vec::new(),
            },
            mounts: None,
            state: None,
        };

        let remote_user = get_remote_user_from_config(&given_docker_config, &devcontainer_manifest);

        assert!(remote_user.is_ok());
        let remote_user = remote_user.expect("ok");
        assert_eq!(&remote_user, "vsCode")
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

    #[gpui::test]
    async fn should_create_correct_docker_run_command(cx: &mut TestAppContext) {
        let mut metadata = HashMap::new();
        metadata.insert(
            "remoteUser".to_string(),
            serde_json_lenient::Value::String("vsCode".to_string()),
        );

        let (_, devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, "{}").await.unwrap();
        let build_resources = DockerBuildResources {
            image: DockerInspect {
                id: "mcr.microsoft.com/devcontainers/base:ubuntu".to_string(),
                config: DockerInspectConfig {
                    labels: DockerConfigLabels { metadata: None },
                    image_user: None,
                    env: Vec::new(),
                },
                mounts: None,
                state: None,
            },
            additional_mounts: vec![],
            privileged: false,
            entrypoint_script: "echo Container started\n    trap \"exit 0\" 15\n    exec \"$@\"\n    while sleep 1 & wait $!; do :; done".to_string(),
        };
        let docker_run_command = devcontainer_manifest.create_docker_run_command(build_resources);

        assert!(docker_run_command.is_ok());
        let docker_run_command = docker_run_command.expect("ok");

        assert_eq!(docker_run_command.get_program(), "docker");
        let expected_config_file_label = PathBuf::from(TEST_PROJECT_PATH)
            .join(".devcontainer")
            .join("devcontainer.json");
        let expected_config_file_label = expected_config_file_label.display();
        assert_eq!(
            docker_run_command.get_args().collect::<Vec<&OsStr>>(),
            vec![
                OsStr::new("run"),
                OsStr::new("--sig-proxy=false"),
                OsStr::new("-d"),
                OsStr::new("--mount"),
                OsStr::new(
                    "type=bind,source=/path/to/local/project,target=/workspaces/project,consistency=cached"
                ),
                OsStr::new("-l"),
                OsStr::new("devcontainer.local_folder=/path/to/local/project"),
                OsStr::new("-l"),
                OsStr::new(&format!(
                    "devcontainer.config_file={expected_config_file_label}"
                )),
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

    #[gpui::test]
    async fn should_find_primary_service_in_docker_compose(cx: &mut TestAppContext) {
        // State where service not defined in dev container
        let (_, given_dev_container) = init_default_devcontainer_manifest(cx, "{}").await.unwrap();
        let given_docker_compose_config = DockerComposeResources {
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
        let (_, given_dev_container) =
            init_default_devcontainer_manifest(cx, r#"{"service": "not_found_service"}"#)
                .await
                .unwrap();
        let given_docker_compose_config = DockerComposeResources {
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

        let (_, given_dev_container) =
            init_default_devcontainer_manifest(cx, r#"{"service": "found_service"}"#)
                .await
                .unwrap();
        let given_docker_compose_config = DockerComposeResources {
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

    #[gpui::test]
    async fn test_nonremote_variable_replacement_with_default_mount(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let given_devcontainer_contents = r#"
// These are some external comments. serde_lenient should handle them
{
    // These are some internal comments
    "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
    "name": "myDevContainer-${devcontainerId}",
    "remoteUser": "root",
    "remoteEnv": {
        "DEVCONTAINER_ID": "${devcontainerId}",
        "MYVAR2": "myvarothervalue",
        "REMOTE_WORKSPACE_FOLDER_BASENAME": "${containerWorkspaceFolderBasename}",
        "LOCAL_WORKSPACE_FOLDER_BASENAME": "${localWorkspaceFolderBasename}",
        "REMOTE_WORKSPACE_FOLDER": "${containerWorkspaceFolder}",
        "LOCAL_WORKSPACE_FOLDER": "${localWorkspaceFolder}",
        "LOCAL_ENV_VAR_1": "${localEnv:local_env_1}",
        "LOCAL_ENV_VAR_2": "${localEnv:my_other_env}"

    }
}
                    "#;
        let (_, mut devcontainer_manifest) = init_devcontainer_manifest(
            cx,
            fs,
            fake_http_client(),
            Arc::new(FakeDocker::new()),
            Arc::new(TestCommandRunner::new()),
            HashMap::from([
                ("local_env_1".to_string(), "local_env_value1".to_string()),
                ("my_other_env".to_string(), "THISVALUEHERE".to_string()),
            ]),
            given_devcontainer_contents,
        )
        .await
        .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let ConfigStatus::VariableParsed(variable_replaced_devcontainer) =
            &devcontainer_manifest.config
        else {
            panic!("Config not parsed");
        };

        // ${devcontainerId}
        let devcontainer_id = devcontainer_manifest.devcontainer_id();
        assert_eq!(
            variable_replaced_devcontainer.name,
            Some(format!("myDevContainer-{devcontainer_id}"))
        );
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("DEVCONTAINER_ID")),
            Some(&devcontainer_id)
        );

        // ${containerWorkspaceFolderBasename}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("REMOTE_WORKSPACE_FOLDER_BASENAME")),
            Some(&test_project_filename())
        );

        // ${localWorkspaceFolderBasename}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("LOCAL_WORKSPACE_FOLDER_BASENAME")),
            Some(&test_project_filename())
        );

        // ${containerWorkspaceFolder}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("REMOTE_WORKSPACE_FOLDER")),
            Some(&format!("/workspaces/{}", test_project_filename()))
        );

        // ${localWorkspaceFolder}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("LOCAL_WORKSPACE_FOLDER")),
            Some(&TEST_PROJECT_PATH.to_string())
        );

        // ${localEnv:VARIABLE_NAME}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("LOCAL_ENV_VAR_1")),
            Some(&"local_env_value1".to_string())
        );
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("LOCAL_ENV_VAR_2")),
            Some(&"THISVALUEHERE".to_string())
        );
    }

    #[gpui::test]
    async fn test_nonremote_variable_replacement_with_explicit_mount(cx: &mut TestAppContext) {
        let given_devcontainer_contents = r#"
                // These are some external comments. serde_lenient should handle them
                {
                    // These are some internal comments
                    "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
                    "name": "myDevContainer-${devcontainerId}",
                    "remoteUser": "root",
                    "remoteEnv": {
                        "DEVCONTAINER_ID": "${devcontainerId}",
                        "MYVAR2": "myvarothervalue",
                        "REMOTE_WORKSPACE_FOLDER_BASENAME": "${containerWorkspaceFolderBasename}",
                        "LOCAL_WORKSPACE_FOLDER_BASENAME": "${localWorkspaceFolderBasename}",
                        "REMOTE_WORKSPACE_FOLDER": "${containerWorkspaceFolder}",
                        "LOCAL_WORKSPACE_FOLDER": "${localWorkspaceFolder}"

                    },
                    "workspaceMount": "source=/local/folder,target=/workspace/subfolder,type=bind,consistency=cached",
                    "workspaceFolder": "/workspace/customfolder"
                }
            "#;

        let (_, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let ConfigStatus::VariableParsed(variable_replaced_devcontainer) =
            &devcontainer_manifest.config
        else {
            panic!("Config not parsed");
        };

        // ${devcontainerId}
        let devcontainer_id = devcontainer_manifest.devcontainer_id();
        assert_eq!(
            variable_replaced_devcontainer.name,
            Some(format!("myDevContainer-{devcontainer_id}"))
        );
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("DEVCONTAINER_ID")),
            Some(&devcontainer_id)
        );

        // ${containerWorkspaceFolderBasename}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("REMOTE_WORKSPACE_FOLDER_BASENAME")),
            Some(&"customfolder".to_string())
        );

        // ${localWorkspaceFolderBasename}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("LOCAL_WORKSPACE_FOLDER_BASENAME")),
            Some(&"project".to_string())
        );

        // ${containerWorkspaceFolder}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("REMOTE_WORKSPACE_FOLDER")),
            Some(&"/workspace/customfolder".to_string())
        );

        // ${localWorkspaceFolder}
        assert_eq!(
            variable_replaced_devcontainer
                .remote_env
                .as_ref()
                .and_then(|env| env.get("LOCAL_WORKSPACE_FOLDER")),
            Some(&TEST_PROJECT_PATH.to_string())
        );
    }

    // updateRemoteUserUID is treated as false in Windows, so this test will fail
    // It is covered by test_spawns_devcontainer_with_dockerfile_and_no_update_uid
    #[cfg(not(target_os = "windows"))]
    #[gpui::test]
    async fn test_spawns_devcontainer_with_dockerfile_and_features(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            /*---------------------------------------------------------------------------------------------
             *  Copyright (c) Microsoft Corporation. All rights reserved.
             *  Licensed under the MIT License. See License.txt in the project root for license information.
             *--------------------------------------------------------------------------------------------*/
            {
              "name": "cli-${devcontainerId}",
              // "image": "mcr.microsoft.com/devcontainers/typescript-node:16-bullseye",
              "build": {
                "dockerfile": "Dockerfile",
                "args": {
                  "VARIANT": "18-bookworm",
                  "FOO": "bar",
                },
              },
              "workspaceMount": "source=${localWorkspaceFolder},target=${containerWorkspaceFolder},type=bind,consistency=cached",
              "workspaceFolder": "/workspace2",
              "mounts": [
                // Keep command history across instances
                "source=dev-containers-cli-bashhistory,target=/home/node/commandhistory",
              ],

              "forwardPorts": [
                8082,
                8083,
              ],
              "appPort": [
                8084,
                "8085:8086",
              ],

              "containerEnv": {
                "VARIABLE_VALUE": "value",
              },

              "initializeCommand": "touch IAM.md",

              "onCreateCommand": "echo 'onCreateCommand' >> ON_CREATE_COMMAND.md",

              "updateContentCommand": "echo 'updateContentCommand' >> UPDATE_CONTENT_COMMAND.md",

              "postCreateCommand": {
                "yarn": "yarn install",
                "debug": "echo 'postStartCommand' >> POST_START_COMMAND.md",
              },

              "postStartCommand": "echo 'postStartCommand' >> POST_START_COMMAND.md",

              "postAttachCommand": "echo 'postAttachCommand' >> POST_ATTACH_COMMAND.md",

              "remoteUser": "node",

              "remoteEnv": {
                "PATH": "${containerEnv:PATH}:/some/other/path",
                "OTHER_ENV": "other_env_value"
              },

              "features": {
                "ghcr.io/devcontainers/features/docker-in-docker:2": {
                  "moby": false,
                },
                "ghcr.io/devcontainers/features/go:1": {},
              },

              "customizations": {
                "vscode": {
                  "extensions": [
                    "dbaeumer.vscode-eslint",
                    "GitHub.vscode-pull-request-github",
                  ],
                },
                "zed": {
                  "extensions": ["vue", "ruby"],
                },
                "codespaces": {
                  "repositories": {
                    "devcontainers/features": {
                      "permissions": {
                        "contents": "write",
                        "workflows": "write",
                      },
                    },
                  },
                },
              },
            }
            "#;

        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
            .fs
            .atomic_write(
                PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
                r#"
#  Copyright (c) Microsoft Corporation. All rights reserved.
#  Licensed under the MIT License. See License.txt in the project root for license information.
ARG VARIANT="16-bullseye"
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT}

RUN mkdir -p /workspaces && chown node:node /workspaces

ARG USERNAME=node
USER $USERNAME

# Save command line history
RUN echo "export HISTFILE=/home/$USERNAME/commandhistory/.bash_history" >> "/home/$USERNAME/.bashrc" \
&& echo "export PROMPT_COMMAND='history -a'" >> "/home/$USERNAME/.bashrc" \
&& mkdir -p /home/$USERNAME/commandhistory \
&& touch /home/$USERNAME/commandhistory/.bash_history \
&& chown -R $USERNAME /home/$USERNAME/commandhistory
                    "#.trim().to_string(),
            )
            .await
            .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let devcontainer_up = devcontainer_manifest.build_and_run().await.unwrap();

        assert_eq!(
            devcontainer_up.extension_ids,
            vec!["vue".to_string(), "ruby".to_string()]
        );

        let files = test_dependencies.fs.files();
        let feature_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "Dockerfile.extended")
            })
            .expect("to be found");
        let feature_dockerfile = test_dependencies.fs.load(feature_dockerfile).await.unwrap();
        assert_eq!(
            &feature_dockerfile,
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

#  Copyright (c) Microsoft Corporation. All rights reserved.
#  Licensed under the MIT License. See License.txt in the project root for license information.
ARG VARIANT="16-bullseye"
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT}

RUN mkdir -p /workspaces && chown node:node /workspaces

ARG USERNAME=node
USER $USERNAME

# Save command line history
RUN echo "export HISTFILE=/home/$USERNAME/commandhistory/.bash_history" >> "/home/$USERNAME/.bashrc" \
&& echo "export PROMPT_COMMAND='history -a'" >> "/home/$USERNAME/.bashrc" \
&& mkdir -p /home/$USERNAME/commandhistory \
&& touch /home/$USERNAME/commandhistory/.bash_history \
&& chown -R $USERNAME /home/$USERNAME/commandhistory
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT} AS dev_container_auto_added_stage_label

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source ./devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p /tmp/dev-container-features
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ /tmp/dev-container-features

RUN \
echo "_CONTAINER_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'root' || grep -E '^root|^[^:]*:[^:]*:root:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'node' || grep -E '^node|^[^:]*:[^:]*:node:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env


RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./docker-in-docker_0,target=/tmp/build-features-src/docker-in-docker_0 \
cp -ar /tmp/build-features-src/docker-in-docker_0 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/docker-in-docker_0 \
&& cd /tmp/dev-container-features/docker-in-docker_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/docker-in-docker_0

RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./go_1,target=/tmp/build-features-src/go_1 \
cp -ar /tmp/build-features-src/go_1 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/go_1 \
&& cd /tmp/dev-container-features/go_1 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/go_1


ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER
"#
        );

        let uid_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "updateUID.Dockerfile")
            })
            .expect("to be found");
        let uid_dockerfile = test_dependencies.fs.load(uid_dockerfile).await.unwrap();

        assert_eq!(
            &uid_dockerfile,
            r#"ARG BASE_IMAGE
FROM $BASE_IMAGE

USER root

ARG REMOTE_USER
ARG NEW_UID
ARG NEW_GID
SHELL ["/bin/sh", "-c"]
RUN eval $(sed -n "s/${REMOTE_USER}:[^:]*:\([^:]*\):\([^:]*\):[^:]*:\([^:]*\).*/OLD_UID=\1;OLD_GID=\2;HOME_FOLDER=\3/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_UID}:.*/EXISTING_USER=\1/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_GID}:.*/EXISTING_GROUP=\1/p" /etc/group); \
	if [ -z "$OLD_UID" ]; then \
		echo "Remote user not found in /etc/passwd ($REMOTE_USER)."; \
	elif [ "$OLD_UID" = "$NEW_UID" -a "$OLD_GID" = "$NEW_GID" ]; then \
		echo "UIDs and GIDs are the same ($NEW_UID:$NEW_GID)."; \
	elif [ "$OLD_UID" != "$NEW_UID" -a -n "$EXISTING_USER" ]; then \
		echo "User with UID exists ($EXISTING_USER=$NEW_UID)."; \
	else \
		if [ "$OLD_GID" != "$NEW_GID" -a -n "$EXISTING_GROUP" ]; then \
			FREE_GID=65532; \
			while grep -q ":[^:]*:${FREE_GID}:" /etc/group; do FREE_GID=$((FREE_GID - 1)); done; \
			echo "Reassigning group $EXISTING_GROUP from GID $NEW_GID to $FREE_GID."; \
			sed -i -e "s/\(${EXISTING_GROUP}:[^:]*:\)${NEW_GID}:/\1${FREE_GID}:/" /etc/group; \
		fi; \
		echo "Updating UID:GID from $OLD_UID:$OLD_GID to $NEW_UID:$NEW_GID."; \
		sed -i -e "s/\(${REMOTE_USER}:[^:]*:\)[^:]*:[^:]*/\1${NEW_UID}:${NEW_GID}/" /etc/passwd; \
		if [ "$OLD_GID" != "$NEW_GID" ]; then \
			sed -i -e "s/\([^:]*:[^:]*:\)${OLD_GID}:/\1${NEW_GID}:/" /etc/group; \
		fi; \
		chown -R $NEW_UID:$NEW_GID $HOME_FOLDER; \
	fi;

ARG IMAGE_USER
USER $IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true

ENV DOCKER_BUILDKIT=1

ENV GOPATH=/go
ENV GOROOT=/usr/local/go
ENV PATH=/usr/local/go/bin:/go/bin:${PATH}
ENV VARIABLE_VALUE=value
"#
        );

        let golang_install_wrapper = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "devcontainer-features-install.sh")
                    && f.to_str().is_some_and(|s| s.contains("/go_"))
            })
            .expect("to be found");
        let golang_install_wrapper = test_dependencies
            .fs
            .load(golang_install_wrapper)
            .await
            .unwrap();
        assert_eq!(
            &golang_install_wrapper,
            r#"#!/bin/sh
set -e

on_exit () {
    [ $? -eq 0 ] && exit
    echo 'ERROR: Feature "go" (ghcr.io/devcontainers/features/go:1) failed to install!'
}

trap on_exit EXIT

echo ===========================================================================
echo 'Feature       : go'
echo 'Id            : ghcr.io/devcontainers/features/go:1'
echo 'Options       :'
echo '    GOLANGCILINTVERSION=latest
    VERSION=latest'
echo ===========================================================================

set -a
. ../devcontainer-features.builtin.env
. ./devcontainer-features.env
set +a

chmod +x ./install.sh
./install.sh
"#
        );

        let docker_commands = test_dependencies
            .command_runner
            .commands_by_program("docker");

        let docker_run_command = docker_commands
            .iter()
            .find(|c| c.args.get(0).is_some_and(|a| a == "run"))
            .expect("found");

        assert_eq!(
            docker_run_command.args,
            vec![
                "run".to_string(),
                "--privileged".to_string(),
                "--sig-proxy=false".to_string(),
                "-d".to_string(),
                "--mount".to_string(),
                "type=bind,source=/path/to/local/project,target=/workspace2,consistency=cached".to_string(),
                "--mount".to_string(),
                "type=volume,source=dev-containers-cli-bashhistory,target=/home/node/commandhistory,consistency=cached".to_string(),
                "--mount".to_string(),
                "type=volume,source=dind-var-lib-docker-42dad4b4ca7b8ced,target=/var/lib/docker,consistency=cached".to_string(),
                "-l".to_string(),
                "devcontainer.local_folder=/path/to/local/project".to_string(),
                "-l".to_string(),
                "devcontainer.config_file=/path/to/local/project/.devcontainer/devcontainer.json".to_string(),
                "-l".to_string(),
                "devcontainer.metadata=[{\"remoteUser\":\"node\"}]".to_string(),
                "-p".to_string(),
                "8082:8082".to_string(),
                "-p".to_string(),
                "8083:8083".to_string(),
                "-p".to_string(),
                "8084:8084".to_string(),
                "-p".to_string(),
                "8085:8086".to_string(),
                "--entrypoint".to_string(),
                "/bin/sh".to_string(),
                "sha256:610e6cfca95280188b021774f8cf69dd6f49bdb6eebc34c5ee2010f4d51cc105".to_string(),
                "-c".to_string(),
                "echo Container started\ntrap \"exit 0\" 15\n/usr/local/share/docker-init.sh\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done".to_string(),
                "-".to_string()
            ]
        );

        let docker_exec_commands = test_dependencies
            .docker
            .exec_commands_recorded
            .lock()
            .unwrap();

        assert!(docker_exec_commands.iter().all(|exec| {
            exec.env
                == HashMap::from([
                    ("OTHER_ENV".to_string(), "other_env_value".to_string()),
                    (
                        "PATH".to_string(),
                        "/initial/path:/some/other/path".to_string(),
                    ),
                ])
        }))
    }

    // updateRemoteUserUID is treated as false in Windows, so this test will fail
    // It is covered by test_spawns_devcontainer_with_docker_compose_and_no_update_uid
    #[cfg(not(target_os = "windows"))]
    #[gpui::test]
    async fn test_spawns_devcontainer_with_docker_compose(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            // For format details, see https://aka.ms/devcontainer.json. For config options, see the
            // README at: https://github.com/devcontainers/templates/tree/main/src/rust-postgres
            {
              "features": {
                "ghcr.io/devcontainers/features/aws-cli:1": {},
                "ghcr.io/devcontainers/features/docker-in-docker:2": {},
              },
              "name": "Rust and PostgreSQL",
              "dockerComposeFile": "docker-compose.yml",
              "service": "app",
              "workspaceFolder": "/workspaces/${localWorkspaceFolderBasename}",

              // Features to add to the dev container. More info: https://containers.dev/features.
              // "features": {},

              // Use 'forwardPorts' to make a list of ports inside the container available locally.
              "forwardPorts": [
                8083,
                "db:5432",
                "db:1234",
              ],

              // Use 'postCreateCommand' to run commands after the container is created.
              // "postCreateCommand": "rustc --version",

              // Configure tool-specific properties.
              // "customizations": {},

              // Uncomment to connect as root instead. More info: https://aka.ms/dev-containers-non-root.
              // "remoteUser": "root"
            }
            "#;
        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
            .fs
            .atomic_write(
                PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/docker-compose.yml"),
                r#"
version: '3.8'

volumes:
    postgres-data:

services:
    app:
        build:
            context: .
            dockerfile: Dockerfile
        env_file:
            # Ensure that the variables in .env match the same variables in devcontainer.json
            - .env

        volumes:
            - ../..:/workspaces:cached

        # Overrides default command so things don't shut down after the process ends.
        command: sleep infinity

        # Runs app on the same network as the database container, allows "forwardPorts" in devcontainer.json function.
        network_mode: service:db

        # Use "forwardPorts" in **devcontainer.json** to forward an app port locally.
        # (Adding the "ports" property to this file will not forward from a Codespace.)

    db:
        image: postgres:14.1
        restart: unless-stopped
        volumes:
            - postgres-data:/var/lib/postgresql/data
        env_file:
            # Ensure that the variables in .env match the same variables in devcontainer.json
            - .env

        # Add "forwardPorts": ["5432"] to **devcontainer.json** to forward PostgreSQL locally.
        # (Adding the "ports" property to this file will not forward from a Codespace.)
                    "#.trim().to_string(),
            )
            .await
            .unwrap();

        test_dependencies.fs.atomic_write(
            PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
            r#"
FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y
            "#.trim().to_string()).await.unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let _devcontainer_up = devcontainer_manifest.build_and_run().await.unwrap();

        let files = test_dependencies.fs.files();
        let feature_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "Dockerfile.extended")
            })
            .expect("to be found");
        let feature_dockerfile = test_dependencies.fs.load(feature_dockerfile).await.unwrap();
        assert_eq!(
            &feature_dockerfile,
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y
FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm AS dev_container_auto_added_stage_label

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source ./devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p /tmp/dev-container-features
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ /tmp/dev-container-features

RUN \
echo "_CONTAINER_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'root' || grep -E '^root|^[^:]*:[^:]*:root:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'vscode' || grep -E '^vscode|^[^:]*:[^:]*:vscode:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env


RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./aws-cli_0,target=/tmp/build-features-src/aws-cli_0 \
cp -ar /tmp/build-features-src/aws-cli_0 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/aws-cli_0 \
&& cd /tmp/dev-container-features/aws-cli_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/aws-cli_0

RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./docker-in-docker_1,target=/tmp/build-features-src/docker-in-docker_1 \
cp -ar /tmp/build-features-src/docker-in-docker_1 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/docker-in-docker_1 \
&& cd /tmp/dev-container-features/docker-in-docker_1 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/docker-in-docker_1


ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER
"#
        );

        let uid_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "updateUID.Dockerfile")
            })
            .expect("to be found");
        let uid_dockerfile = test_dependencies.fs.load(uid_dockerfile).await.unwrap();

        assert_eq!(
            &uid_dockerfile,
            r#"ARG BASE_IMAGE
FROM $BASE_IMAGE

USER root

ARG REMOTE_USER
ARG NEW_UID
ARG NEW_GID
SHELL ["/bin/sh", "-c"]
RUN eval $(sed -n "s/${REMOTE_USER}:[^:]*:\([^:]*\):\([^:]*\):[^:]*:\([^:]*\).*/OLD_UID=\1;OLD_GID=\2;HOME_FOLDER=\3/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_UID}:.*/EXISTING_USER=\1/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_GID}:.*/EXISTING_GROUP=\1/p" /etc/group); \
	if [ -z "$OLD_UID" ]; then \
		echo "Remote user not found in /etc/passwd ($REMOTE_USER)."; \
	elif [ "$OLD_UID" = "$NEW_UID" -a "$OLD_GID" = "$NEW_GID" ]; then \
		echo "UIDs and GIDs are the same ($NEW_UID:$NEW_GID)."; \
	elif [ "$OLD_UID" != "$NEW_UID" -a -n "$EXISTING_USER" ]; then \
		echo "User with UID exists ($EXISTING_USER=$NEW_UID)."; \
	else \
		if [ "$OLD_GID" != "$NEW_GID" -a -n "$EXISTING_GROUP" ]; then \
			FREE_GID=65532; \
			while grep -q ":[^:]*:${FREE_GID}:" /etc/group; do FREE_GID=$((FREE_GID - 1)); done; \
			echo "Reassigning group $EXISTING_GROUP from GID $NEW_GID to $FREE_GID."; \
			sed -i -e "s/\(${EXISTING_GROUP}:[^:]*:\)${NEW_GID}:/\1${FREE_GID}:/" /etc/group; \
		fi; \
		echo "Updating UID:GID from $OLD_UID:$OLD_GID to $NEW_UID:$NEW_GID."; \
		sed -i -e "s/\(${REMOTE_USER}:[^:]*:\)[^:]*:[^:]*/\1${NEW_UID}:${NEW_GID}/" /etc/passwd; \
		if [ "$OLD_GID" != "$NEW_GID" ]; then \
			sed -i -e "s/\([^:]*:[^:]*:\)${OLD_GID}:/\1${NEW_GID}:/" /etc/group; \
		fi; \
		chown -R $NEW_UID:$NEW_GID $HOME_FOLDER; \
	fi;

ARG IMAGE_USER
USER $IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true


ENV DOCKER_BUILDKIT=1
"#
        );

        let build_override = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "docker_compose_build.json")
            })
            .expect("to be found");
        let build_override = test_dependencies.fs.load(build_override).await.unwrap();
        let build_config: DockerComposeConfig =
            serde_json_lenient::from_str(&build_override).unwrap();
        let build_context = build_config
            .services
            .get("app")
            .and_then(|s| s.build.as_ref())
            .and_then(|b| b.context.clone())
            .expect("build override should have a context");
        assert_eq!(
            build_context, ".",
            "build override should preserve the original build context from docker-compose.yml"
        );

        let runtime_override = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "docker_compose_runtime.json")
            })
            .expect("to be found");
        let runtime_override = test_dependencies.fs.load(runtime_override).await.unwrap();

        let expected_runtime_override = DockerComposeConfig {
            name: None,
            services: HashMap::from([
                (
                    "app".to_string(),
                    DockerComposeService {
                        entrypoint: Some(vec![
                            "/bin/sh".to_string(),
                            "-c".to_string(),
                            "echo Container started\ntrap \"exit 0\" 15\n/usr/local/share/docker-init.sh\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done".to_string(),
                            "-".to_string(),
                        ]),
                        cap_add: Some(vec!["SYS_PTRACE".to_string()]),
                        security_opt: Some(vec!["seccomp=unconfined".to_string()]),
                        privileged: Some(true),
                        labels: Some(HashMap::from([
                            ("devcontainer.metadata".to_string(), "[{\"remoteUser\":\"vscode\"}]".to_string()),
                            ("devcontainer.local_folder".to_string(), "/path/to/local/project".to_string()),
                            ("devcontainer.config_file".to_string(), "/path/to/local/project/.devcontainer/devcontainer.json".to_string())
                        ])),
                        volumes: vec![
                            MountDefinition {
                                source: Some("dind-var-lib-docker-42dad4b4ca7b8ced".to_string()),
                                target: "/var/lib/docker".to_string(),
                                mount_type: Some("volume".to_string())
                            }
                        ],
                        ..Default::default()
                    },
                ),
                (
                    "db".to_string(),
                    DockerComposeService {
                        ports: vec![
                            DockerComposeServicePort {
                                target: "8083".to_string(),
                                published: "8083".to_string(),
                                ..Default::default()
                            },
                            DockerComposeServicePort {
                                target: "5432".to_string(),
                                published: "5432".to_string(),
                                ..Default::default()
                            },
                            DockerComposeServicePort {
                                target: "1234".to_string(),
                                published: "1234".to_string(),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                ),
            ]),
            volumes: HashMap::from([(
                "dind-var-lib-docker-42dad4b4ca7b8ced".to_string(),
                DockerComposeVolume {
                    name: "dind-var-lib-docker-42dad4b4ca7b8ced".to_string(),
                },
            )]),
        };

        assert_eq!(
            serde_json_lenient::from_str::<DockerComposeConfig>(&runtime_override).unwrap(),
            expected_runtime_override
        )
    }

    #[gpui::test]
    async fn test_spawns_devcontainer_with_docker_compose_and_no_update_uid(
        cx: &mut TestAppContext,
    ) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
        // For format details, see https://aka.ms/devcontainer.json. For config options, see the
        // README at: https://github.com/devcontainers/templates/tree/main/src/rust-postgres
        {
          "features": {
            "ghcr.io/devcontainers/features/aws-cli:1": {},
            "ghcr.io/devcontainers/features/docker-in-docker:2": {},
          },
          "name": "Rust and PostgreSQL",
          "dockerComposeFile": "docker-compose.yml",
          "service": "app",
          "workspaceFolder": "/workspaces/${localWorkspaceFolderBasename}",

          // Features to add to the dev container. More info: https://containers.dev/features.
          // "features": {},

          // Use 'forwardPorts' to make a list of ports inside the container available locally.
          "forwardPorts": [
            8083,
            "db:5432",
            "db:1234",
          ],
          "updateRemoteUserUID": false,
          "appPort": "8084",

          // Use 'postCreateCommand' to run commands after the container is created.
          // "postCreateCommand": "rustc --version",

          // Configure tool-specific properties.
          // "customizations": {},

          // Uncomment to connect as root instead. More info: https://aka.ms/dev-containers-non-root.
          // "remoteUser": "root"
        }
        "#;
        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
        .fs
        .atomic_write(
            PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/docker-compose.yml"),
            r#"
version: '3.8'

volumes:
postgres-data:

services:
app:
    build:
        context: .
        dockerfile: Dockerfile
    env_file:
        # Ensure that the variables in .env match the same variables in devcontainer.json
        - .env

    volumes:
        - ../..:/workspaces:cached

    # Overrides default command so things don't shut down after the process ends.
    command: sleep infinity

    # Runs app on the same network as the database container, allows "forwardPorts" in devcontainer.json function.
    network_mode: service:db

    # Use "forwardPorts" in **devcontainer.json** to forward an app port locally.
    # (Adding the "ports" property to this file will not forward from a Codespace.)

db:
    image: postgres:14.1
    restart: unless-stopped
    volumes:
        - postgres-data:/var/lib/postgresql/data
    env_file:
        # Ensure that the variables in .env match the same variables in devcontainer.json
        - .env

    # Add "forwardPorts": ["5432"] to **devcontainer.json** to forward PostgreSQL locally.
    # (Adding the "ports" property to this file will not forward from a Codespace.)
                "#.trim().to_string(),
        )
        .await
        .unwrap();

        test_dependencies.fs.atomic_write(
        PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
        r#"
FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
&& apt-get -y install clang lld \
&& apt-get autoremove -y && apt-get clean -y
        "#.trim().to_string()).await.unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let _devcontainer_up = devcontainer_manifest.build_and_run().await.unwrap();

        let files = test_dependencies.fs.files();
        let feature_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "Dockerfile.extended")
            })
            .expect("to be found");
        let feature_dockerfile = test_dependencies.fs.load(feature_dockerfile).await.unwrap();
        assert_eq!(
            &feature_dockerfile,
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
&& apt-get -y install clang lld \
&& apt-get autoremove -y && apt-get clean -y
FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm AS dev_container_auto_added_stage_label

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source ./devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p /tmp/dev-container-features
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ /tmp/dev-container-features

RUN \
echo "_CONTAINER_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'root' || grep -E '^root|^[^:]*:[^:]*:root:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'vscode' || grep -E '^vscode|^[^:]*:[^:]*:vscode:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env


RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./aws-cli_0,target=/tmp/build-features-src/aws-cli_0 \
cp -ar /tmp/build-features-src/aws-cli_0 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/aws-cli_0 \
&& cd /tmp/dev-container-features/aws-cli_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/aws-cli_0

RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./docker-in-docker_1,target=/tmp/build-features-src/docker-in-docker_1 \
cp -ar /tmp/build-features-src/docker-in-docker_1 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/docker-in-docker_1 \
&& cd /tmp/dev-container-features/docker-in-docker_1 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/docker-in-docker_1


ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true


ENV DOCKER_BUILDKIT=1
"#
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[gpui::test]
    async fn test_spawns_devcontainer_with_docker_compose_and_podman(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
        // For format details, see https://aka.ms/devcontainer.json. For config options, see the
        // README at: https://github.com/devcontainers/templates/tree/main/src/rust-postgres
        {
          "features": {
            "ghcr.io/devcontainers/features/aws-cli:1": {},
            "ghcr.io/devcontainers/features/docker-in-docker:2": {},
          },
          "name": "Rust and PostgreSQL",
          "dockerComposeFile": "docker-compose.yml",
          "service": "app",
          "workspaceFolder": "/workspaces/${localWorkspaceFolderBasename}",

          // Features to add to the dev container. More info: https://containers.dev/features.
          // "features": {},

          // Use 'forwardPorts' to make a list of ports inside the container available locally.
          // "forwardPorts": [5432],

          // Use 'postCreateCommand' to run commands after the container is created.
          // "postCreateCommand": "rustc --version",

          // Configure tool-specific properties.
          // "customizations": {},

          // Uncomment to connect as root instead. More info: https://aka.ms/dev-containers-non-root.
          // "remoteUser": "root"
        }
        "#;
        let mut fake_docker = FakeDocker::new();
        fake_docker.set_podman(true);
        let (test_dependencies, mut devcontainer_manifest) = init_devcontainer_manifest(
            cx,
            FakeFs::new(cx.executor()),
            fake_http_client(),
            Arc::new(fake_docker),
            Arc::new(TestCommandRunner::new()),
            HashMap::new(),
            given_devcontainer_contents,
        )
        .await
        .unwrap();

        test_dependencies
        .fs
        .atomic_write(
            PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/docker-compose.yml"),
            r#"
version: '3.8'

volumes:
postgres-data:

services:
app:
build:
    context: .
    dockerfile: Dockerfile
env_file:
    # Ensure that the variables in .env match the same variables in devcontainer.json
    - .env

volumes:
    - ../..:/workspaces:cached

# Overrides default command so things don't shut down after the process ends.
command: sleep infinity

# Runs app on the same network as the database container, allows "forwardPorts" in devcontainer.json function.
network_mode: service:db

# Use "forwardPorts" in **devcontainer.json** to forward an app port locally.
# (Adding the "ports" property to this file will not forward from a Codespace.)

db:
image: postgres:14.1
restart: unless-stopped
volumes:
    - postgres-data:/var/lib/postgresql/data
env_file:
    # Ensure that the variables in .env match the same variables in devcontainer.json
    - .env

# Add "forwardPorts": ["5432"] to **devcontainer.json** to forward PostgreSQL locally.
# (Adding the "ports" property to this file will not forward from a Codespace.)
                "#.trim().to_string(),
        )
        .await
        .unwrap();

        test_dependencies.fs.atomic_write(
        PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
        r#"
FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
&& apt-get -y install clang lld \
&& apt-get autoremove -y && apt-get clean -y
        "#.trim().to_string()).await.unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let _devcontainer_up = devcontainer_manifest.build_and_run().await.unwrap();

        let files = test_dependencies.fs.files();

        let feature_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "Dockerfile.extended")
            })
            .expect("to be found");
        let feature_dockerfile = test_dependencies.fs.load(feature_dockerfile).await.unwrap();
        assert_eq!(
            &feature_dockerfile,
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
&& apt-get -y install clang lld \
&& apt-get autoremove -y && apt-get clean -y
FROM mcr.microsoft.com/devcontainers/rust:2-1-bookworm AS dev_container_auto_added_stage_label

FROM dev_container_feature_content_temp as dev_containers_feature_content_source

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source /tmp/build-features/devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p /tmp/dev-container-features
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ /tmp/dev-container-features

RUN \
echo "_CONTAINER_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'root' || grep -E '^root|^[^:]*:[^:]*:root:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'vscode' || grep -E '^vscode|^[^:]*:[^:]*:vscode:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env


COPY --chown=root:root --from=dev_containers_feature_content_source /tmp/build-features/aws-cli_0 /tmp/dev-container-features/aws-cli_0
RUN chmod -R 0755 /tmp/dev-container-features/aws-cli_0 \
&& cd /tmp/dev-container-features/aws-cli_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh

COPY --chown=root:root --from=dev_containers_feature_content_source /tmp/build-features/docker-in-docker_1 /tmp/dev-container-features/docker-in-docker_1
RUN chmod -R 0755 /tmp/dev-container-features/docker-in-docker_1 \
&& cd /tmp/dev-container-features/docker-in-docker_1 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh


ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER
"#
        );

        let uid_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "updateUID.Dockerfile")
            })
            .expect("to be found");
        let uid_dockerfile = test_dependencies.fs.load(uid_dockerfile).await.unwrap();

        assert_eq!(
            &uid_dockerfile,
            r#"ARG BASE_IMAGE
FROM $BASE_IMAGE

USER root

ARG REMOTE_USER
ARG NEW_UID
ARG NEW_GID
SHELL ["/bin/sh", "-c"]
RUN eval $(sed -n "s/${REMOTE_USER}:[^:]*:\([^:]*\):\([^:]*\):[^:]*:\([^:]*\).*/OLD_UID=\1;OLD_GID=\2;HOME_FOLDER=\3/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_UID}:.*/EXISTING_USER=\1/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_GID}:.*/EXISTING_GROUP=\1/p" /etc/group); \
	if [ -z "$OLD_UID" ]; then \
		echo "Remote user not found in /etc/passwd ($REMOTE_USER)."; \
	elif [ "$OLD_UID" = "$NEW_UID" -a "$OLD_GID" = "$NEW_GID" ]; then \
		echo "UIDs and GIDs are the same ($NEW_UID:$NEW_GID)."; \
	elif [ "$OLD_UID" != "$NEW_UID" -a -n "$EXISTING_USER" ]; then \
		echo "User with UID exists ($EXISTING_USER=$NEW_UID)."; \
	else \
		if [ "$OLD_GID" != "$NEW_GID" -a -n "$EXISTING_GROUP" ]; then \
			FREE_GID=65532; \
			while grep -q ":[^:]*:${FREE_GID}:" /etc/group; do FREE_GID=$((FREE_GID - 1)); done; \
			echo "Reassigning group $EXISTING_GROUP from GID $NEW_GID to $FREE_GID."; \
			sed -i -e "s/\(${EXISTING_GROUP}:[^:]*:\)${NEW_GID}:/\1${FREE_GID}:/" /etc/group; \
		fi; \
		echo "Updating UID:GID from $OLD_UID:$OLD_GID to $NEW_UID:$NEW_GID."; \
		sed -i -e "s/\(${REMOTE_USER}:[^:]*:\)[^:]*:[^:]*/\1${NEW_UID}:${NEW_GID}/" /etc/passwd; \
		if [ "$OLD_GID" != "$NEW_GID" ]; then \
			sed -i -e "s/\([^:]*:[^:]*:\)${OLD_GID}:/\1${NEW_GID}:/" /etc/group; \
		fi; \
		chown -R $NEW_UID:$NEW_GID $HOME_FOLDER; \
	fi;

ARG IMAGE_USER
USER $IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true


ENV DOCKER_BUILDKIT=1
"#
        );
    }

    #[gpui::test]
    async fn test_spawns_devcontainer_with_dockerfile_and_no_update_uid(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            /*---------------------------------------------------------------------------------------------
             *  Copyright (c) Microsoft Corporation. All rights reserved.
             *  Licensed under the MIT License. See License.txt in the project root for license information.
             *--------------------------------------------------------------------------------------------*/
            {
              "name": "cli-${devcontainerId}",
              // "image": "mcr.microsoft.com/devcontainers/typescript-node:16-bullseye",
              "build": {
                "dockerfile": "Dockerfile",
                "args": {
                  "VARIANT": "18-bookworm",
                  "FOO": "bar",
                },
                "target": "development",
              },
              "workspaceMount": "source=${localWorkspaceFolder},target=${containerWorkspaceFolder},type=bind,consistency=cached",
              "workspaceFolder": "/workspace2",
              "mounts": [
                // Keep command history across instances
                "source=dev-containers-cli-bashhistory,target=/home/node/commandhistory",
              ],

              "forwardPorts": [
                8082,
                8083,
              ],
              "appPort": "8084",
              "updateRemoteUserUID": false,

              "containerEnv": {
                "VARIABLE_VALUE": "value",
              },

              "initializeCommand": "touch IAM.md",

              "onCreateCommand": "echo 'onCreateCommand' >> ON_CREATE_COMMAND.md",

              "updateContentCommand": "echo 'updateContentCommand' >> UPDATE_CONTENT_COMMAND.md",

              "postCreateCommand": {
                "yarn": "yarn install",
                "debug": "echo 'postStartCommand' >> POST_START_COMMAND.md",
              },

              "postStartCommand": "echo 'postStartCommand' >> POST_START_COMMAND.md",

              "postAttachCommand": "echo 'postAttachCommand' >> POST_ATTACH_COMMAND.md",

              "remoteUser": "node",

              "remoteEnv": {
                "PATH": "${containerEnv:PATH}:/some/other/path",
                "OTHER_ENV": "other_env_value"
              },

              "features": {
                "ghcr.io/devcontainers/features/docker-in-docker:2": {
                  "moby": false,
                },
                "ghcr.io/devcontainers/features/go:1": {},
              },

              "customizations": {
                "vscode": {
                  "extensions": [
                    "dbaeumer.vscode-eslint",
                    "GitHub.vscode-pull-request-github",
                  ],
                },
                "zed": {
                  "extensions": ["vue", "ruby"],
                },
                "codespaces": {
                  "repositories": {
                    "devcontainers/features": {
                      "permissions": {
                        "contents": "write",
                        "workflows": "write",
                      },
                    },
                  },
                },
              },
            }
            "#;

        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
            .fs
            .atomic_write(
                PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
                r#"
#  Copyright (c) Microsoft Corporation. All rights reserved.
#  Licensed under the MIT License. See License.txt in the project root for license information.
ARG VARIANT="16-bullseye"
FROM mcr.microsoft.com/devcontainers/typescript-node:latest as predev
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT} as development

RUN mkdir -p /workspaces && chown node:node /workspaces

ARG USERNAME=node
USER $USERNAME

# Save command line history
RUN echo "export HISTFILE=/home/$USERNAME/commandhistory/.bash_history" >> "/home/$USERNAME/.bashrc" \
&& echo "export PROMPT_COMMAND='history -a'" >> "/home/$USERNAME/.bashrc" \
&& mkdir -p /home/$USERNAME/commandhistory \
&& touch /home/$USERNAME/commandhistory/.bash_history \
&& chown -R $USERNAME /home/$USERNAME/commandhistory
                    "#.trim().to_string(),
            )
            .await
            .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let devcontainer_up = devcontainer_manifest.build_and_run().await.unwrap();

        assert_eq!(
            devcontainer_up.extension_ids,
            vec!["vue".to_string(), "ruby".to_string()]
        );

        let files = test_dependencies.fs.files();
        let feature_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "Dockerfile.extended")
            })
            .expect("to be found");
        let feature_dockerfile = test_dependencies.fs.load(feature_dockerfile).await.unwrap();
        assert_eq!(
            &feature_dockerfile,
            r#"ARG _DEV_CONTAINERS_BASE_IMAGE=placeholder

#  Copyright (c) Microsoft Corporation. All rights reserved.
#  Licensed under the MIT License. See License.txt in the project root for license information.
ARG VARIANT="16-bullseye"
FROM mcr.microsoft.com/devcontainers/typescript-node:latest as predev
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT} as development

RUN mkdir -p /workspaces && chown node:node /workspaces

ARG USERNAME=node
USER $USERNAME

# Save command line history
RUN echo "export HISTFILE=/home/$USERNAME/commandhistory/.bash_history" >> "/home/$USERNAME/.bashrc" \
&& echo "export PROMPT_COMMAND='history -a'" >> "/home/$USERNAME/.bashrc" \
&& mkdir -p /home/$USERNAME/commandhistory \
&& touch /home/$USERNAME/commandhistory/.bash_history \
&& chown -R $USERNAME /home/$USERNAME/commandhistory
FROM mcr.microsoft.com/devcontainers/typescript-node:1-${VARIANT} AS dev_container_auto_added_stage_label

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_feature_content_normalize
USER root
COPY --from=dev_containers_feature_content_source ./devcontainer-features.builtin.env /tmp/build-features/
RUN chmod -R 0755 /tmp/build-features/

FROM $_DEV_CONTAINERS_BASE_IMAGE AS dev_containers_target_stage

USER root

RUN mkdir -p /tmp/dev-container-features
COPY --from=dev_containers_feature_content_normalize /tmp/build-features/ /tmp/dev-container-features

RUN \
echo "_CONTAINER_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'root' || grep -E '^root|^[^:]*:[^:]*:root:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env && \
echo "_REMOTE_USER_HOME=$( (command -v getent >/dev/null 2>&1 && getent passwd 'node' || grep -E '^node|^[^:]*:[^:]*:node:' /etc/passwd || true) | cut -d: -f6)" >> /tmp/dev-container-features/devcontainer-features.builtin.env


RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./docker-in-docker_0,target=/tmp/build-features-src/docker-in-docker_0 \
cp -ar /tmp/build-features-src/docker-in-docker_0 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/docker-in-docker_0 \
&& cd /tmp/dev-container-features/docker-in-docker_0 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/docker-in-docker_0

RUN --mount=type=bind,from=dev_containers_feature_content_source,source=./go_1,target=/tmp/build-features-src/go_1 \
cp -ar /tmp/build-features-src/go_1 /tmp/dev-container-features \
&& chmod -R 0755 /tmp/dev-container-features/go_1 \
&& cd /tmp/dev-container-features/go_1 \
&& chmod +x ./devcontainer-features-install.sh \
&& ./devcontainer-features-install.sh \
&& rm -rf /tmp/dev-container-features/go_1


ARG _DEV_CONTAINERS_IMAGE_USER=root
USER $_DEV_CONTAINERS_IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true

ENV DOCKER_BUILDKIT=1

ENV GOPATH=/go
ENV GOROOT=/usr/local/go
ENV PATH=/usr/local/go/bin:/go/bin:${PATH}
ENV VARIABLE_VALUE=value
"#
        );

        let golang_install_wrapper = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "devcontainer-features-install.sh")
                    && f.to_str().is_some_and(|s| s.contains("go_"))
            })
            .expect("to be found");
        let golang_install_wrapper = test_dependencies
            .fs
            .load(golang_install_wrapper)
            .await
            .unwrap();
        assert_eq!(
            &golang_install_wrapper,
            r#"#!/bin/sh
set -e

on_exit () {
    [ $? -eq 0 ] && exit
    echo 'ERROR: Feature "go" (ghcr.io/devcontainers/features/go:1) failed to install!'
}

trap on_exit EXIT

echo ===========================================================================
echo 'Feature       : go'
echo 'Id            : ghcr.io/devcontainers/features/go:1'
echo 'Options       :'
echo '    GOLANGCILINTVERSION=latest
    VERSION=latest'
echo ===========================================================================

set -a
. ../devcontainer-features.builtin.env
. ./devcontainer-features.env
set +a

chmod +x ./install.sh
./install.sh
"#
        );

        let docker_commands = test_dependencies
            .command_runner
            .commands_by_program("docker");

        let docker_run_command = docker_commands
            .iter()
            .find(|c| c.args.get(0).is_some_and(|a| a == "run"));

        assert!(docker_run_command.is_some());

        let docker_exec_commands = test_dependencies
            .docker
            .exec_commands_recorded
            .lock()
            .unwrap();

        assert!(docker_exec_commands.iter().all(|exec| {
            exec.env
                == HashMap::from([
                    ("OTHER_ENV".to_string(), "other_env_value".to_string()),
                    (
                        "PATH".to_string(),
                        "/initial/path:/some/other/path".to_string(),
                    ),
                ])
        }))
    }

    #[cfg(not(target_os = "windows"))]
    #[gpui::test]
    async fn test_spawns_devcontainer_with_plain_image(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            {
              "name": "cli-${devcontainerId}",
              "image": "test_image:latest",
            }
            "#;

        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let _devcontainer_up = devcontainer_manifest.build_and_run().await.unwrap();

        let files = test_dependencies.fs.files();
        let uid_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "updateUID.Dockerfile")
            })
            .expect("to be found");
        let uid_dockerfile = test_dependencies.fs.load(uid_dockerfile).await.unwrap();

        assert_eq!(
            &uid_dockerfile,
            r#"ARG BASE_IMAGE
FROM $BASE_IMAGE

USER root

ARG REMOTE_USER
ARG NEW_UID
ARG NEW_GID
SHELL ["/bin/sh", "-c"]
RUN eval $(sed -n "s/${REMOTE_USER}:[^:]*:\([^:]*\):\([^:]*\):[^:]*:\([^:]*\).*/OLD_UID=\1;OLD_GID=\2;HOME_FOLDER=\3/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_UID}:.*/EXISTING_USER=\1/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_GID}:.*/EXISTING_GROUP=\1/p" /etc/group); \
	if [ -z "$OLD_UID" ]; then \
		echo "Remote user not found in /etc/passwd ($REMOTE_USER)."; \
	elif [ "$OLD_UID" = "$NEW_UID" -a "$OLD_GID" = "$NEW_GID" ]; then \
		echo "UIDs and GIDs are the same ($NEW_UID:$NEW_GID)."; \
	elif [ "$OLD_UID" != "$NEW_UID" -a -n "$EXISTING_USER" ]; then \
		echo "User with UID exists ($EXISTING_USER=$NEW_UID)."; \
	else \
		if [ "$OLD_GID" != "$NEW_GID" -a -n "$EXISTING_GROUP" ]; then \
			FREE_GID=65532; \
			while grep -q ":[^:]*:${FREE_GID}:" /etc/group; do FREE_GID=$((FREE_GID - 1)); done; \
			echo "Reassigning group $EXISTING_GROUP from GID $NEW_GID to $FREE_GID."; \
			sed -i -e "s/\(${EXISTING_GROUP}:[^:]*:\)${NEW_GID}:/\1${FREE_GID}:/" /etc/group; \
		fi; \
		echo "Updating UID:GID from $OLD_UID:$OLD_GID to $NEW_UID:$NEW_GID."; \
		sed -i -e "s/\(${REMOTE_USER}:[^:]*:\)[^:]*:[^:]*/\1${NEW_UID}:${NEW_GID}/" /etc/passwd; \
		if [ "$OLD_GID" != "$NEW_GID" ]; then \
			sed -i -e "s/\([^:]*:[^:]*:\)${OLD_GID}:/\1${NEW_GID}:/" /etc/group; \
		fi; \
		chown -R $NEW_UID:$NEW_GID $HOME_FOLDER; \
	fi;

ARG IMAGE_USER
USER $IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true
"#
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[gpui::test]
    async fn test_spawns_devcontainer_with_docker_compose_and_plain_image(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            {
              "name": "cli-${devcontainerId}",
              "dockerComposeFile": "docker-compose-plain.yml",
              "service": "app",
            }
            "#;

        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
            .fs
            .atomic_write(
                PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/docker-compose-plain.yml"),
                r#"
services:
    app:
        image: test_image:latest
        command: sleep infinity
        volumes:
            - ..:/workspace:cached
                "#
                .trim()
                .to_string(),
            )
            .await
            .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let _devcontainer_up = devcontainer_manifest.build_and_run().await.unwrap();

        let files = test_dependencies.fs.files();
        let uid_dockerfile = files
            .iter()
            .find(|f| {
                f.file_name()
                    .is_some_and(|s| s.display().to_string() == "updateUID.Dockerfile")
            })
            .expect("to be found");
        let uid_dockerfile = test_dependencies.fs.load(uid_dockerfile).await.unwrap();

        assert_eq!(
            &uid_dockerfile,
            r#"ARG BASE_IMAGE
FROM $BASE_IMAGE

USER root

ARG REMOTE_USER
ARG NEW_UID
ARG NEW_GID
SHELL ["/bin/sh", "-c"]
RUN eval $(sed -n "s/${REMOTE_USER}:[^:]*:\([^:]*\):\([^:]*\):[^:]*:\([^:]*\).*/OLD_UID=\1;OLD_GID=\2;HOME_FOLDER=\3/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_UID}:.*/EXISTING_USER=\1/p" /etc/passwd); \
	eval $(sed -n "s/\([^:]*\):[^:]*:${NEW_GID}:.*/EXISTING_GROUP=\1/p" /etc/group); \
	if [ -z "$OLD_UID" ]; then \
		echo "Remote user not found in /etc/passwd ($REMOTE_USER)."; \
	elif [ "$OLD_UID" = "$NEW_UID" -a "$OLD_GID" = "$NEW_GID" ]; then \
		echo "UIDs and GIDs are the same ($NEW_UID:$NEW_GID)."; \
	elif [ "$OLD_UID" != "$NEW_UID" -a -n "$EXISTING_USER" ]; then \
		echo "User with UID exists ($EXISTING_USER=$NEW_UID)."; \
	else \
		if [ "$OLD_GID" != "$NEW_GID" -a -n "$EXISTING_GROUP" ]; then \
			FREE_GID=65532; \
			while grep -q ":[^:]*:${FREE_GID}:" /etc/group; do FREE_GID=$((FREE_GID - 1)); done; \
			echo "Reassigning group $EXISTING_GROUP from GID $NEW_GID to $FREE_GID."; \
			sed -i -e "s/\(${EXISTING_GROUP}:[^:]*:\)${NEW_GID}:/\1${FREE_GID}:/" /etc/group; \
		fi; \
		echo "Updating UID:GID from $OLD_UID:$OLD_GID to $NEW_UID:$NEW_GID."; \
		sed -i -e "s/\(${REMOTE_USER}:[^:]*:\)[^:]*:[^:]*/\1${NEW_UID}:${NEW_GID}/" /etc/passwd; \
		if [ "$OLD_GID" != "$NEW_GID" ]; then \
			sed -i -e "s/\([^:]*:[^:]*:\)${OLD_GID}:/\1${NEW_GID}:/" /etc/group; \
		fi; \
		chown -R $NEW_UID:$NEW_GID $HOME_FOLDER; \
	fi;

ARG IMAGE_USER
USER $IMAGE_USER

# Ensure that /etc/profile does not clobber the existing path
RUN sed -i -E 's/((^|\s)PATH=)([^\$]*)$/\1\${PATH:-\3}/g' /etc/profile || true
"#
        );
    }

    #[gpui::test]
    async fn test_gets_base_image_from_dockerfile(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            {
              "name": "cli-${devcontainerId}",
              "build": {
                "dockerfile": "Dockerfile",
                "args": {
                    "VERSION": "1.22",
                }
              },
            }
            "#;

        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
            .fs
            .atomic_write(
                PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
                r#"
FROM dontgrabme as build_context
ARG VERSION=1.21
ARG REPOSITORY=mybuild
ARG REGISTRY=docker.io/stuff

ARG IMAGE=${REGISTRY}/${REPOSITORY}:${VERSION}

FROM ${IMAGE} AS devcontainer
                    "#
                .trim()
                .to_string(),
            )
            .await
            .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let dockerfile_contents = devcontainer_manifest
            .expanded_dockerfile_content()
            .await
            .unwrap();
        let base_image = image_from_dockerfile(
            dockerfile_contents,
            &devcontainer_manifest
                .dev_container()
                .build
                .as_ref()
                .and_then(|b| b.target.clone()),
        )
        .unwrap();

        assert_eq!(base_image, "docker.io/stuff/mybuild:1.22".to_string());
    }

    #[gpui::test]
    async fn test_gets_base_image_from_dockerfile_with_target_specified(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            {
              "name": "cli-${devcontainerId}",
              "build": {
                "dockerfile": "Dockerfile",
                "args": {
                    "VERSION": "1.22",
                },
                "target": "development"
              },
            }
            "#;

        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
            .fs
            .atomic_write(
                PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
                r#"
FROM dontgrabme as build_context
ARG VERSION=1.21
ARG REPOSITORY=mybuild
ARG REGISTRY=docker.io/stuff

ARG IMAGE=${REGISTRY}/${REPOSITORY}:${VERSION}
ARG DEV_IMAGE=${REGISTRY}/${REPOSITORY}:latest

FROM ${DEV_IMAGE} AS development
FROM ${IMAGE} AS production
                    "#
                .trim()
                .to_string(),
            )
            .await
            .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let dockerfile_contents = devcontainer_manifest
            .expanded_dockerfile_content()
            .await
            .unwrap();
        let base_image = image_from_dockerfile(
            dockerfile_contents,
            &devcontainer_manifest
                .dev_container()
                .build
                .as_ref()
                .and_then(|b| b.target.clone()),
        )
        .unwrap();

        assert_eq!(base_image, "docker.io/stuff/mybuild:latest".to_string());
    }

    #[gpui::test]
    async fn test_expands_args_in_dockerfile(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        env_logger::try_init().ok();
        let given_devcontainer_contents = r#"
            {
              "name": "cli-${devcontainerId}",
              "build": {
                "dockerfile": "Dockerfile",
                "args": {
                    "JSON_ARG": "some-value",
                    "ELIXIR_VERSION": "1.21",
                }
              },
            }
            "#;

        let (test_dependencies, mut devcontainer_manifest) =
            init_default_devcontainer_manifest(cx, given_devcontainer_contents)
                .await
                .unwrap();

        test_dependencies
            .fs
            .atomic_write(
                PathBuf::from(TEST_PROJECT_PATH).join(".devcontainer/Dockerfile"),
                r#"
ARG INVALID_FORWARD_REFERENCE=${OTP_VERSION}
ARG ELIXIR_VERSION=1.20.0-rc.4
ARG FOO=foo BAR=bar
ARG FOOBAR=${FOO}${BAR}
ARG OTP_VERSION=28.4.1
ARG DEBIAN_VERSION=trixie-20260316-slim
ARG IMAGE="docker.io/hexpm/elixir:${ELIXIR_VERSION}-erlang-${OTP_VERSION}-debian-${DEBIAN_VERSION}"
ARG NESTED_MAP="{"key1": "val1", "key2": "val2"}"
ARG WRAPPING_MAP={"nested_map": ${NESTED_MAP}}
ARG FROM_JSON=${JSON_ARG}

FROM ${IMAGE} AS devcontainer
                    "#
                .trim()
                .to_string(),
            )
            .await
            .unwrap();

        devcontainer_manifest.parse_nonremote_vars().unwrap();

        let expanded_dockerfile = devcontainer_manifest
            .expanded_dockerfile_content()
            .await
            .unwrap();

        assert_eq!(
            &expanded_dockerfile,
            r#"
ARG INVALID_FORWARD_REFERENCE=${OTP_VERSION}
ARG ELIXIR_VERSION=1.20.0-rc.4
ARG FOO=foo BAR=bar
ARG FOOBAR=foobar
ARG OTP_VERSION=28.4.1
ARG DEBIAN_VERSION=trixie-20260316-slim
ARG IMAGE="docker.io/hexpm/elixir:1.21-erlang-28.4.1-debian-trixie-20260316-slim"
ARG NESTED_MAP="{"key1": "val1", "key2": "val2"}"
ARG WRAPPING_MAP={"nested_map": {"key1": "val1", "key2": "val2"}}
ARG FROM_JSON=some-value

FROM docker.io/hexpm/elixir:1.21-erlang-28.4.1-debian-trixie-20260316-slim AS devcontainer
            "#
            .trim()
        )
    }

    #[test]
    fn test_aliases_dockerfile_with_pre_existing_aliases_for_build() {}

    #[test]
    fn test_aliases_dockerfile_with_no_aliases_for_build() {}

    #[test]
    fn test_aliases_dockerfile_with_build_target_specified() {}

    pub(crate) struct RecordedExecCommand {
        pub(crate) _container_id: String,
        pub(crate) _remote_folder: String,
        pub(crate) _user: String,
        pub(crate) env: HashMap<String, String>,
        pub(crate) _inner_command: Command,
    }

    pub(crate) struct FakeDocker {
        exec_commands_recorded: Mutex<Vec<RecordedExecCommand>>,
        podman: bool,
    }

    impl FakeDocker {
        pub(crate) fn new() -> Self {
            Self {
                podman: false,
                exec_commands_recorded: Mutex::new(Vec::new()),
            }
        }
        #[cfg(not(target_os = "windows"))]
        fn set_podman(&mut self, podman: bool) {
            self.podman = podman;
        }
    }

    #[async_trait]
    impl DockerClient for FakeDocker {
        async fn inspect(&self, id: &String) -> Result<DockerInspect, DevContainerError> {
            if id == "mcr.microsoft.com/devcontainers/typescript-node:1-18-bookworm" {
                return Ok(DockerInspect {
                    id: "sha256:610e6cfca95280188b021774f8cf69dd6f49bdb6eebc34c5ee2010f4d51cc104"
                        .to_string(),
                    config: DockerInspectConfig {
                        labels: DockerConfigLabels {
                            metadata: Some(vec![HashMap::from([(
                                "remoteUser".to_string(),
                                Value::String("node".to_string()),
                            )])]),
                        },
                        env: Vec::new(),
                        image_user: Some("root".to_string()),
                    },
                    mounts: None,
                    state: None,
                });
            }
            if id == "mcr.microsoft.com/devcontainers/rust:2-1-bookworm" {
                return Ok(DockerInspect {
                    id: "sha256:39ad1c7264794d60e3bc449d9d8877a8e486d19ad8fba80f5369def6a2408392"
                        .to_string(),
                    config: DockerInspectConfig {
                        labels: DockerConfigLabels {
                            metadata: Some(vec![HashMap::from([(
                                "remoteUser".to_string(),
                                Value::String("vscode".to_string()),
                            )])]),
                        },
                        image_user: Some("root".to_string()),
                        env: Vec::new(),
                    },
                    mounts: None,
                    state: None,
                });
            }
            if id.starts_with("cli_") {
                return Ok(DockerInspect {
                    id: "sha256:610e6cfca95280188b021774f8cf69dd6f49bdb6eebc34c5ee2010f4d51cc105"
                        .to_string(),
                    config: DockerInspectConfig {
                        labels: DockerConfigLabels {
                            metadata: Some(vec![HashMap::from([(
                                "remoteUser".to_string(),
                                Value::String("node".to_string()),
                            )])]),
                        },
                        image_user: Some("root".to_string()),
                        env: vec!["PATH=/initial/path".to_string()],
                    },
                    mounts: None,
                    state: None,
                });
            }
            if id == "found_docker_ps" {
                return Ok(DockerInspect {
                    id: "sha256:610e6cfca95280188b021774f8cf69dd6f49bdb6eebc34c5ee2010f4d51cc105"
                        .to_string(),
                    config: DockerInspectConfig {
                        labels: DockerConfigLabels {
                            metadata: Some(vec![HashMap::from([(
                                "remoteUser".to_string(),
                                Value::String("node".to_string()),
                            )])]),
                        },
                        image_user: Some("root".to_string()),
                        env: vec!["PATH=/initial/path".to_string()],
                    },
                    mounts: Some(vec![DockerInspectMount {
                        source: "/path/to/local/project".to_string(),
                        destination: "/workspaces/project".to_string(),
                    }]),
                    state: None,
                });
            }
            if id.starts_with("rust_a-") {
                return Ok(DockerInspect {
                    id: "sha256:9da65c34ab809e763b13d238fd7a0f129fcabd533627d340f293308cb63620a0"
                        .to_string(),
                    config: DockerInspectConfig {
                        labels: DockerConfigLabels {
                            metadata: Some(vec![HashMap::from([(
                                "remoteUser".to_string(),
                                Value::String("vscode".to_string()),
                            )])]),
                        },
                        image_user: Some("root".to_string()),
                        env: Vec::new(),
                    },
                    mounts: None,
                    state: None,
                });
            }
            if id == "test_image:latest" {
                return Ok(DockerInspect {
                    id: "sha256:610e6cfca95280188b021774f8cf69dd6f49bdb6eebc34c5ee2010f4d51cc104"
                        .to_string(),
                    config: DockerInspectConfig {
                        labels: DockerConfigLabels {
                            metadata: Some(vec![HashMap::from([(
                                "remoteUser".to_string(),
                                Value::String("node".to_string()),
                            )])]),
                        },
                        env: Vec::new(),
                        image_user: Some("root".to_string()),
                    },
                    mounts: None,
                    state: None,
                });
            }

            Err(DevContainerError::DockerNotAvailable)
        }
        async fn get_docker_compose_config(
            &self,
            config_files: &Vec<PathBuf>,
        ) -> Result<Option<DockerComposeConfig>, DevContainerError> {
            if config_files.len() == 1
                && config_files.get(0)
                    == Some(&PathBuf::from(
                        "/path/to/local/project/.devcontainer/docker-compose.yml",
                    ))
            {
                return Ok(Some(DockerComposeConfig {
                    name: None,
                    services: HashMap::from([
                        (
                            "app".to_string(),
                            DockerComposeService {
                                build: Some(DockerComposeServiceBuild {
                                    context: Some(".".to_string()),
                                    dockerfile: Some("Dockerfile".to_string()),
                                    args: None,
                                    additional_contexts: None,
                                    target: None,
                                }),
                                volumes: vec![MountDefinition {
                                    source: Some("../..".to_string()),
                                    target: "/workspaces".to_string(),
                                    mount_type: Some("bind".to_string()),
                                }],
                                network_mode: Some("service:db".to_string()),
                                ..Default::default()
                            },
                        ),
                        (
                            "db".to_string(),
                            DockerComposeService {
                                image: Some("postgres:14.1".to_string()),
                                volumes: vec![MountDefinition {
                                    source: Some("postgres-data".to_string()),
                                    target: "/var/lib/postgresql/data".to_string(),
                                    mount_type: Some("volume".to_string()),
                                }],
                                env_file: Some(vec![".env".to_string()]),
                                ..Default::default()
                            },
                        ),
                    ]),
                    volumes: HashMap::from([(
                        "postgres-data".to_string(),
                        DockerComposeVolume::default(),
                    )]),
                }));
            }
            if config_files.len() == 1
                && config_files.get(0)
                    == Some(&PathBuf::from(
                        "/path/to/local/project/.devcontainer/docker-compose-plain.yml",
                    ))
            {
                return Ok(Some(DockerComposeConfig {
                    name: None,
                    services: HashMap::from([(
                        "app".to_string(),
                        DockerComposeService {
                            image: Some("test_image:latest".to_string()),
                            command: vec!["sleep".to_string(), "infinity".to_string()],
                            ..Default::default()
                        },
                    )]),
                    ..Default::default()
                }));
            }
            Err(DevContainerError::DockerNotAvailable)
        }
        async fn docker_compose_build(
            &self,
            _config_files: &Vec<PathBuf>,
            _project_name: &str,
        ) -> Result<(), DevContainerError> {
            Ok(())
        }
        async fn run_docker_exec(
            &self,
            container_id: &str,
            remote_folder: &str,
            user: &str,
            env: &HashMap<String, String>,
            inner_command: Command,
        ) -> Result<(), DevContainerError> {
            let mut record = self
                .exec_commands_recorded
                .lock()
                .expect("should be available");
            record.push(RecordedExecCommand {
                _container_id: container_id.to_string(),
                _remote_folder: remote_folder.to_string(),
                _user: user.to_string(),
                env: env.clone(),
                _inner_command: inner_command,
            });
            Ok(())
        }
        async fn start_container(&self, _id: &str) -> Result<(), DevContainerError> {
            Err(DevContainerError::DockerNotAvailable)
        }
        async fn find_process_by_filters(
            &self,
            _filters: Vec<String>,
        ) -> Result<Option<DockerPs>, DevContainerError> {
            Ok(Some(DockerPs {
                id: "found_docker_ps".to_string(),
            }))
        }
        fn supports_compose_buildkit(&self) -> bool {
            !self.podman
        }
        fn docker_cli(&self) -> String {
            if self.podman {
                "podman".to_string()
            } else {
                "docker".to_string()
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct TestCommand {
        pub(crate) program: String,
        pub(crate) args: Vec<String>,
    }

    pub(crate) struct TestCommandRunner {
        commands_recorded: Mutex<Vec<TestCommand>>,
    }

    impl TestCommandRunner {
        fn new() -> Self {
            Self {
                commands_recorded: Mutex::new(Vec::new()),
            }
        }

        fn commands_by_program(&self, program: &str) -> Vec<TestCommand> {
            let record = self.commands_recorded.lock().expect("poisoned");
            record
                .iter()
                .filter(|r| r.program == program)
                .map(|r| r.clone())
                .collect()
        }
    }

    #[async_trait]
    impl CommandRunner for TestCommandRunner {
        async fn run_command(&self, command: &mut Command) -> Result<Output, std::io::Error> {
            let mut record = self.commands_recorded.lock().expect("poisoned");

            record.push(TestCommand {
                program: command.get_program().display().to_string(),
                args: command
                    .get_args()
                    .map(|a| a.display().to_string())
                    .collect(),
            });

            Ok(Output {
                status: ExitStatus::default(),
                stdout: vec![],
                stderr: vec![],
            })
        }
    }

    fn fake_http_client() -> Arc<dyn HttpClient> {
        FakeHttpClient::create(|request| async move {
            let (parts, _body) = request.into_parts();
            if parts.uri.path() == "/token" {
                let token_response = TokenResponse {
                    token: "token".to_string(),
                };
                return Ok(http::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(
                        serde_json_lenient::to_string(&token_response).unwrap(),
                    ))
                    .unwrap());
            }

            // OCI specific things
            if parts.uri.path() == "/v2/devcontainers/features/docker-in-docker/manifests/2" {
                let response = r#"
                    {
                        "schemaVersion": 2,
                        "mediaType": "application/vnd.oci.image.manifest.v1+json",
                        "config": {
                            "mediaType": "application/vnd.devcontainers",
                            "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
                            "size": 2
                        },
                        "layers": [
                            {
                                "mediaType": "application/vnd.devcontainers.layer.v1+tar",
                                "digest": "sha256:bc7ab0d8d8339416e1491419ab9ffe931458d0130110f4b18351b0fa184e67d5",
                                "size": 59392,
                                "annotations": {
                                    "org.opencontainers.image.title": "devcontainer-feature-docker-in-docker.tgz"
                                }
                            }
                        ],
                        "annotations": {
                            "dev.containers.metadata": "{\"id\":\"docker-in-docker\",\"version\":\"2.16.1\",\"name\":\"Docker (Docker-in-Docker)\",\"documentationURL\":\"https://github.com/devcontainers/features/tree/main/src/docker-in-docker\",\"description\":\"Create child containers *inside* a container, independent from the host's docker instance. Installs Docker extension in the container along with needed CLIs.\",\"options\":{\"version\":{\"type\":\"string\",\"proposals\":[\"latest\",\"none\",\"20.10\"],\"default\":\"latest\",\"description\":\"Select or enter a Docker/Moby Engine version. (Availability can vary by OS version.)\"},\"moby\":{\"type\":\"boolean\",\"default\":true,\"description\":\"Install OSS Moby build instead of Docker CE\"},\"mobyBuildxVersion\":{\"type\":\"string\",\"default\":\"latest\",\"description\":\"Install a specific version of moby-buildx when using Moby\"},\"dockerDashComposeVersion\":{\"type\":\"string\",\"enum\":[\"none\",\"v1\",\"v2\"],\"default\":\"v2\",\"description\":\"Default version of Docker Compose (v1, v2 or none)\"},\"azureDnsAutoDetection\":{\"type\":\"boolean\",\"default\":true,\"description\":\"Allow automatically setting the dockerd DNS server when the installation script detects it is running in Azure\"},\"dockerDefaultAddressPool\":{\"type\":\"string\",\"default\":\"\",\"proposals\":[],\"description\":\"Define default address pools for Docker networks. e.g. base=192.168.0.0/16,size=24\"},\"installDockerBuildx\":{\"type\":\"boolean\",\"default\":true,\"description\":\"Install Docker Buildx\"},\"installDockerComposeSwitch\":{\"type\":\"boolean\",\"default\":false,\"description\":\"Install Compose Switch (provided docker compose is available) which is a replacement to the Compose V1 docker-compose (python) executable. It translates the command line into Compose V2 docker compose then runs the latter.\"},\"disableIp6tables\":{\"type\":\"boolean\",\"default\":false,\"description\":\"Disable ip6tables (this option is only applicable for Docker versions 27 and greater)\"}},\"entrypoint\":\"/usr/local/share/docker-init.sh\",\"privileged\":true,\"containerEnv\":{\"DOCKER_BUILDKIT\":\"1\"},\"customizations\":{\"vscode\":{\"extensions\":[\"ms-azuretools.vscode-containers\"],\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes the Docker CLI (`docker`) pre-installed and available on the `PATH` for running and managing containers using a dedicated Docker daemon running inside the dev container.\"}]}}},\"mounts\":[{\"source\":\"dind-var-lib-docker-${devcontainerId}\",\"target\":\"/var/lib/docker\",\"type\":\"volume\"}],\"installsAfter\":[\"ghcr.io/devcontainers/features/common-utils\"]}",
                            "com.github.package.type": "devcontainer_feature"
                        }
                    }
                    "#;
                return Ok(http::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(response))
                    .unwrap());
            }

            if parts.uri.path()
                == "/v2/devcontainers/features/docker-in-docker/blobs/sha256:bc7ab0d8d8339416e1491419ab9ffe931458d0130110f4b18351b0fa184e67d5"
            {
                let response = build_tarball(vec![
                    ("./NOTES.md", r#"
                        ## Limitations

                        This docker-in-docker Dev Container Feature is roughly based on the [official docker-in-docker wrapper script](https://github.com/moby/moby/blob/master/hack/dind) that is part of the [Moby project](https://mobyproject.org/). With this in mind:
                        * As the name implies, the Feature is expected to work when the host is running Docker (or the OSS Moby container engine it is built on). It may be possible to get running in other container engines, but it has not been tested with them.
                        * The host and the container must be running on the same chip architecture. You will not be able to use it with an emulated x86 image with Docker Desktop on an Apple Silicon Mac, like in this example:
                          ```
                          FROM --platform=linux/amd64 mcr.microsoft.com/devcontainers/typescript-node:16
                          ```
                          See [Issue #219](https://github.com/devcontainers/features/issues/219) for more details.


                        ## OS Support

                        This Feature should work on recent versions of Debian/Ubuntu-based distributions with the `apt` package manager installed.

                        Debian Trixie (13) does not include moby-cli and related system packages, so the feature cannot install with "moby": "true". To use this feature on Trixie, please set "moby": "false" or choose a different base image (for example, Ubuntu 24.04).

                        `bash` is required to execute the `install.sh` script."#),
                    ("./README.md", r#"
                        # Docker (Docker-in-Docker) (docker-in-docker)

                        Create child containers *inside* a container, independent from the host's docker instance. Installs Docker extension in the container along with needed CLIs.

                        ## Example Usage

                        ```json
                        "features": {
                            "ghcr.io/devcontainers/features/docker-in-docker:2": {}
                        }
                        ```

                        ## Options

                        | Options Id | Description | Type | Default Value |
                        |-----|-----|-----|-----|
                        | version | Select or enter a Docker/Moby Engine version. (Availability can vary by OS version.) | string | latest |
                        | moby | Install OSS Moby build instead of Docker CE | boolean | true |
                        | mobyBuildxVersion | Install a specific version of moby-buildx when using Moby | string | latest |
                        | dockerDashComposeVersion | Default version of Docker Compose (v1, v2 or none) | string | v2 |
                        | azureDnsAutoDetection | Allow automatically setting the dockerd DNS server when the installation script detects it is running in Azure | boolean | true |
                        | dockerDefaultAddressPool | Define default address pools for Docker networks. e.g. base=192.168.0.0/16,size=24 | string | - |
                        | installDockerBuildx | Install Docker Buildx | boolean | true |
                        | installDockerComposeSwitch | Install Compose Switch (provided docker compose is available) which is a replacement to the Compose V1 docker-compose (python) executable. It translates the command line into Compose V2 docker compose then runs the latter. | boolean | false |
                        | disableIp6tables | Disable ip6tables (this option is only applicable for Docker versions 27 and greater) | boolean | false |

                        ## Customizations

                        ### VS Code Extensions

                        - `ms-azuretools.vscode-containers`

                        ## Limitations

                        This docker-in-docker Dev Container Feature is roughly based on the [official docker-in-docker wrapper script](https://github.com/moby/moby/blob/master/hack/dind) that is part of the [Moby project](https://mobyproject.org/). With this in mind:
                        * As the name implies, the Feature is expected to work when the host is running Docker (or the OSS Moby container engine it is built on). It may be possible to get running in other container engines, but it has not been tested with them.
                        * The host and the container must be running on the same chip architecture. You will not be able to use it with an emulated x86 image with Docker Desktop on an Apple Silicon Mac, like in this example:
                          ```
                          FROM --platform=linux/amd64 mcr.microsoft.com/devcontainers/typescript-node:16
                          ```
                          See [Issue #219](https://github.com/devcontainers/features/issues/219) for more details.


                        ## OS Support

                        This Feature should work on recent versions of Debian/Ubuntu-based distributions with the `apt` package manager installed.

                        `bash` is required to execute the `install.sh` script.


                        ---

                        _Note: This file was auto-generated from the [devcontainer-feature.json](https://github.com/devcontainers/features/blob/main/src/docker-in-docker/devcontainer-feature.json).  Add additional notes to a `NOTES.md`._"#),
                    ("./devcontainer-feature.json", r#"
                        {
                          "id": "docker-in-docker",
                          "version": "2.16.1",
                          "name": "Docker (Docker-in-Docker)",
                          "documentationURL": "https://github.com/devcontainers/features/tree/main/src/docker-in-docker",
                          "description": "Create child containers *inside* a container, independent from the host's docker instance. Installs Docker extension in the container along with needed CLIs.",
                          "options": {
                            "version": {
                              "type": "string",
                              "proposals": [
                                "latest",
                                "none",
                                "20.10"
                              ],
                              "default": "latest",
                              "description": "Select or enter a Docker/Moby Engine version. (Availability can vary by OS version.)"
                            },
                            "moby": {
                              "type": "boolean",
                              "default": true,
                              "description": "Install OSS Moby build instead of Docker CE"
                            },
                            "mobyBuildxVersion": {
                              "type": "string",
                              "default": "latest",
                              "description": "Install a specific version of moby-buildx when using Moby"
                            },
                            "dockerDashComposeVersion": {
                              "type": "string",
                              "enum": [
                                "none",
                                "v1",
                                "v2"
                              ],
                              "default": "v2",
                              "description": "Default version of Docker Compose (v1, v2 or none)"
                            },
                            "azureDnsAutoDetection": {
                              "type": "boolean",
                              "default": true,
                              "description": "Allow automatically setting the dockerd DNS server when the installation script detects it is running in Azure"
                            },
                            "dockerDefaultAddressPool": {
                              "type": "string",
                              "default": "",
                              "proposals": [],
                              "description": "Define default address pools for Docker networks. e.g. base=192.168.0.0/16,size=24"
                            },
                            "installDockerBuildx": {
                              "type": "boolean",
                              "default": true,
                              "description": "Install Docker Buildx"
                            },
                            "installDockerComposeSwitch": {
                              "type": "boolean",
                              "default": false,
                              "description": "Install Compose Switch (provided docker compose is available) which is a replacement to the Compose V1 docker-compose (python) executable. It translates the command line into Compose V2 docker compose then runs the latter."
                            },
                            "disableIp6tables": {
                              "type": "boolean",
                              "default": false,
                              "description": "Disable ip6tables (this option is only applicable for Docker versions 27 and greater)"
                            }
                          },
                          "entrypoint": "/usr/local/share/docker-init.sh",
                          "privileged": true,
                          "containerEnv": {
                            "DOCKER_BUILDKIT": "1"
                          },
                          "customizations": {
                            "vscode": {
                              "extensions": [
                                "ms-azuretools.vscode-containers"
                              ],
                              "settings": {
                                "github.copilot.chat.codeGeneration.instructions": [
                                  {
                                    "text": "This dev container includes the Docker CLI (`docker`) pre-installed and available on the `PATH` for running and managing containers using a dedicated Docker daemon running inside the dev container."
                                  }
                                ]
                              }
                            }
                          },
                          "mounts": [
                            {
                              "source": "dind-var-lib-docker-${devcontainerId}",
                              "target": "/var/lib/docker",
                              "type": "volume"
                            }
                          ],
                          "installsAfter": [
                            "ghcr.io/devcontainers/features/common-utils"
                          ]
                        }"#),
                    ("./install.sh", r#"
                    #!/usr/bin/env bash
                    #-------------------------------------------------------------------------------------------------------------
                    # Copyright (c) Microsoft Corporation. All rights reserved.
                    # Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
                    #-------------------------------------------------------------------------------------------------------------
                    #
                    # Docs: https://github.com/microsoft/vscode-dev-containers/blob/main/script-library/docs/docker-in-docker.md
                    # Maintainer: The Dev Container spec maintainers


                    DOCKER_VERSION="${VERSION:-"latest"}" # The Docker/Moby Engine + CLI should match in version
                    USE_MOBY="${MOBY:-"true"}"
                    MOBY_BUILDX_VERSION="${MOBYBUILDXVERSION:-"latest"}"
                    DOCKER_DASH_COMPOSE_VERSION="${DOCKERDASHCOMPOSEVERSION:-"v2"}" #v1, v2 or none
                    AZURE_DNS_AUTO_DETECTION="${AZUREDNSAUTODETECTION:-"true"}"
                    DOCKER_DEFAULT_ADDRESS_POOL="${DOCKERDEFAULTADDRESSPOOL:-""}"
                    USERNAME="${USERNAME:-"${_REMOTE_USER:-"automatic"}"}"
                    INSTALL_DOCKER_BUILDX="${INSTALLDOCKERBUILDX:-"true"}"
                    INSTALL_DOCKER_COMPOSE_SWITCH="${INSTALLDOCKERCOMPOSESWITCH:-"false"}"
                    MICROSOFT_GPG_KEYS_URI="https://packages.microsoft.com/keys/microsoft.asc"
                    MICROSOFT_GPG_KEYS_ROLLING_URI="https://packages.microsoft.com/keys/microsoft-rolling.asc"
                    DOCKER_MOBY_ARCHIVE_VERSION_CODENAMES="trixie bookworm buster bullseye bionic focal jammy noble"
                    DOCKER_LICENSED_ARCHIVE_VERSION_CODENAMES="trixie bookworm buster bullseye bionic focal hirsute impish jammy noble"
                    DISABLE_IP6_TABLES="${DISABLEIP6TABLES:-false}"

                    # Default: Exit on any failure.
                    set -e

                    # Clean up
                    rm -rf /var/lib/apt/lists/*

                    # Setup STDERR.
                    err() {
                        echo "(!) $*" >&2
                    }

                    if [ "$(id -u)" -ne 0 ]; then
                        err 'Script must be run as root. Use sudo, su, or add "USER root" to your Dockerfile before running this script.'
                        exit 1
                    fi

                    ###################
                    # Helper Functions
                    # See: https://github.com/microsoft/vscode-dev-containers/blob/main/script-library/shared/utils.sh
                    ###################

                    # Determine the appropriate non-root user
                    if [ "${USERNAME}" = "auto" ] || [ "${USERNAME}" = "automatic" ]; then
                        USERNAME=""
                        POSSIBLE_USERS=("vscode" "node" "codespace" "$(awk -v val=1000 -F ":" '$3==val{print $1}' /etc/passwd)")
                        for CURRENT_USER in "${POSSIBLE_USERS[@]}"; do
                            if id -u ${CURRENT_USER} > /dev/null 2>&1; then
                                USERNAME=${CURRENT_USER}
                                break
                            fi
                        done
                        if [ "${USERNAME}" = "" ]; then
                            USERNAME=root
                        fi
                    elif [ "${USERNAME}" = "none" ] || ! id -u ${USERNAME} > /dev/null 2>&1; then
                        USERNAME=root
                    fi

                    # Package manager update function
                    pkg_mgr_update() {
                        case ${ADJUSTED_ID} in
                            debian)
                                if [ "$(find /var/lib/apt/lists/* | wc -l)" = "0" ]; then
                                    echo "Running apt-get update..."
                                    apt-get update -y
                                fi
                                ;;
                            rhel)
                                if [ ${PKG_MGR_CMD} = "microdnf" ]; then
                                    cache_check_dir="/var/cache/yum"
                                else
                                    cache_check_dir="/var/cache/${PKG_MGR_CMD}"
                                fi
                                if [ "$(ls ${cache_check_dir}/* 2>/dev/null | wc -l)" = 0 ]; then
                                    echo "Running ${PKG_MGR_CMD} makecache ..."
                                    ${PKG_MGR_CMD} makecache
                                fi
                                ;;
                        esac
                    }

                    # Checks if packages are installed and installs them if not
                    check_packages() {
                        case ${ADJUSTED_ID} in
                            debian)
                                if ! dpkg -s "$@" > /dev/null 2>&1; then
                                    pkg_mgr_update
                                    apt-get -y install --no-install-recommends "$@"
                                fi
                                ;;
                            rhel)
                                if ! rpm -q "$@" > /dev/null 2>&1; then
                                    pkg_mgr_update
                                    ${PKG_MGR_CMD} -y install "$@"
                                fi
                                ;;
                        esac
                    }

                    # Figure out correct version of a three part version number is not passed
                    find_version_from_git_tags() {
                        local variable_name=$1
                        local requested_version=${!variable_name}
                        if [ "${requested_version}" = "none" ]; then return; fi
                        local repository=$2
                        local prefix=${3:-"tags/v"}
                        local separator=${4:-"."}
                        local last_part_optional=${5:-"false"}
                        if [ "$(echo "${requested_version}" | grep -o "." | wc -l)" != "2" ]; then
                            local escaped_separator=${separator//./\\.}
                            local last_part
                            if [ "${last_part_optional}" = "true" ]; then
                                last_part="(${escaped_separator}[0-9]+)?"
                            else
                                last_part="${escaped_separator}[0-9]+"
                            fi
                            local regex="${prefix}\\K[0-9]+${escaped_separator}[0-9]+${last_part}$"
                            local version_list="$(git ls-remote --tags ${repository} | grep -oP "${regex}" | tr -d ' ' | tr "${separator}" "." | sort -rV)"
                            if [ "${requested_version}" = "latest" ] || [ "${requested_version}" = "current" ] || [ "${requested_version}" = "lts" ]; then
                                declare -g ${variable_name}="$(echo "${version_list}" | head -n 1)"
                            else
                                set +e
                                    declare -g ${variable_name}="$(echo "${version_list}" | grep -E -m 1 "^${requested_version//./\\.}([\\.\\s]|$)")"
                                set -e
                            fi
                        fi
                        if [ -z "${!variable_name}" ] || ! echo "${version_list}" | grep "^${!variable_name//./\\.}$" > /dev/null 2>&1; then
                            err "Invalid ${variable_name} value: ${requested_version}\nValid values:\n${version_list}" >&2
                            exit 1
                        fi
                        echo "${variable_name}=${!variable_name}"
                    }

                    # Use semver logic to decrement a version number then look for the closest match
                    find_prev_version_from_git_tags() {
                        local variable_name=$1
                        local current_version=${!variable_name}
                        local repository=$2
                        # Normally a "v" is used before the version number, but support alternate cases
                        local prefix=${3:-"tags/v"}
                        # Some repositories use "_" instead of "." for version number part separation, support that
                        local separator=${4:-"."}
                        # Some tools release versions that omit the last digit (e.g. go)
                        local last_part_optional=${5:-"false"}
                        # Some repositories may have tags that include a suffix (e.g. actions/node-versions)
                        local version_suffix_regex=$6
                        # Try one break fix version number less if we get a failure. Use "set +e" since "set -e" can cause failures in valid scenarios.
                        set +e
                            major="$(echo "${current_version}" | grep -oE '^[0-9]+' || echo '')"
                            minor="$(echo "${current_version}" | grep -oP '^[0-9]+\.\K[0-9]+' || echo '')"
                            breakfix="$(echo "${current_version}" | grep -oP '^[0-9]+\.[0-9]+\.\K[0-9]+' 2>/dev/null || echo '')"

                            if [ "${minor}" = "0" ] && [ "${breakfix}" = "0" ]; then
                                ((major=major-1))
                                declare -g ${variable_name}="${major}"
                                # Look for latest version from previous major release
                                find_version_from_git_tags "${variable_name}" "${repository}" "${prefix}" "${separator}" "${last_part_optional}"
                            # Handle situations like Go's odd version pattern where "0" releases omit the last part
                            elif [ "${breakfix}" = "" ] || [ "${breakfix}" = "0" ]; then
                                ((minor=minor-1))
                                declare -g ${variable_name}="${major}.${minor}"
                                # Look for latest version from previous minor release
                                find_version_from_git_tags "${variable_name}" "${repository}" "${prefix}" "${separator}" "${last_part_optional}"
                            else
                                ((breakfix=breakfix-1))
                                if [ "${breakfix}" = "0" ] && [ "${last_part_optional}" = "true" ]; then
                                    declare -g ${variable_name}="${major}.${minor}"
                                else
                                    declare -g ${variable_name}="${major}.${minor}.${breakfix}"
                                fi
                            fi
                        set -e
                    }

                    # Function to fetch the version released prior to the latest version
                    get_previous_version() {
                        local url=$1
                        local repo_url=$2
                        local variable_name=$3
                        prev_version=${!variable_name}

                        output=$(curl -s "$repo_url");
                        if echo "$output" | jq -e 'type == "object"' > /dev/null; then
                          message=$(echo "$output" | jq -r '.message')

                          if [[ $message == "API rate limit exceeded"* ]]; then
                                echo -e "\nAn attempt to find latest version using GitHub Api Failed... \nReason: ${message}"
                                echo -e "\nAttempting to find latest version using GitHub tags."
                                find_prev_version_from_git_tags prev_version "$url" "tags/v"
                                declare -g ${variable_name}="${prev_version}"
                           fi
                        elif echo "$output" | jq -e 'type == "array"' > /dev/null; then
                            echo -e "\nAttempting to find latest version using GitHub Api."
                            version=$(echo "$output" | jq -r '.[1].tag_name')
                            declare -g ${variable_name}="${version#v}"
                        fi
                        echo "${variable_name}=${!variable_name}"
                    }

                    get_github_api_repo_url() {
                        local url=$1
                        echo "${url/https:\/\/github.com/https:\/\/api.github.com\/repos}/releases"
                    }

                    ###########################################
                    # Start docker-in-docker installation
                    ###########################################

                    # Ensure apt is in non-interactive to avoid prompts
                    export DEBIAN_FRONTEND=noninteractive

                    # Source /etc/os-release to get OS info
                    . /etc/os-release

                    # Determine adjusted ID and package manager
                    if [ "${ID}" = "debian" ] || [ "${ID_LIKE}" = "debian" ]; then
                        ADJUSTED_ID="debian"
                        PKG_MGR_CMD="apt-get"
                        # Use dpkg for Debian-based systems
                        architecture="$(dpkg --print-architecture 2>/dev/null || uname -m)"
                    elif [[ "${ID}" = "rhel" || "${ID}" = "fedora" || "${ID}" = "azurelinux" || "${ID}" = "mariner" || "${ID_LIKE}" = *"rhel"* || "${ID_LIKE}" = *"fedora"* || "${ID_LIKE}" = *"azurelinux"* || "${ID_LIKE}" = *"mariner"* ]]; then
                        ADJUSTED_ID="rhel"
                        # Determine the appropriate package manager for RHEL-based systems
                        for pkg_mgr in tdnf dnf microdnf yum; do
                            if command -v "$pkg_mgr" >/dev/null 2>&1; then
                                PKG_MGR_CMD="$pkg_mgr"
                                break
                            fi
                        done

                        if [ -z "${PKG_MGR_CMD}" ]; then
                            err "Unable to find a supported package manager (tdnf, dnf, microdnf, yum)"
                            exit 1
                        fi

                        architecture="$(rpm --eval '%{_arch}' 2>/dev/null || uname -m)"
                    else
                        err "Linux distro ${ID} not supported."
                        exit 1
                    fi

                    # Azure Linux specific setup
                    if [ "${ID}" = "azurelinux" ]; then
                        VERSION_CODENAME="azurelinux${VERSION_ID}"
                    fi

                    # Prevent attempting to install Moby on Debian trixie (packages removed)
                    if [ "${USE_MOBY}" = "true" ] && [ "${ID}" = "debian" ] && [ "${VERSION_CODENAME}" = "trixie" ]; then
                        err "The 'moby' option is not supported on Debian 'trixie' because 'moby-cli' and related system packages have been removed from that distribution."
                        err "To continue, either set the feature option '\"moby\": false' or use a different base image (for example: 'debian:bookworm' or 'ubuntu-24.04')."
                        exit 1
                    fi

                    # Check if distro is supported
                    if [ "${USE_MOBY}" = "true" ]; then
                        if [ "${ADJUSTED_ID}" = "debian" ]; then
                            if [[ "${DOCKER_MOBY_ARCHIVE_VERSION_CODENAMES}" != *"${VERSION_CODENAME}"* ]]; then
                                err "Unsupported distribution version '${VERSION_CODENAME}'. To resolve, either: (1) set feature option '\"moby\": false' , or (2) choose a compatible OS distribution"
                                err "Supported distributions include: ${DOCKER_MOBY_ARCHIVE_VERSION_CODENAMES}"
                                exit 1
                            fi
                            echo "(*) ${VERSION_CODENAME} is supported for Moby installation  - setting up Microsoft repository"
                        elif [ "${ADJUSTED_ID}" = "rhel" ]; then
                            if [ "${ID}" = "azurelinux" ] || [ "${ID}" = "mariner" ]; then
                                echo " (*) ${ID} ${VERSION_ID} detected - using Microsoft repositories for Moby packages"
                            else
                                echo "RHEL-based system (${ID}) detected - Moby packages may require additional configuration"
                            fi
                        fi
                    else
                        if [ "${ADJUSTED_ID}" = "debian" ]; then
                            if [[ "${DOCKER_LICENSED_ARCHIVE_VERSION_CODENAMES}" != *"${VERSION_CODENAME}"* ]]; then
                                err "Unsupported distribution version '${VERSION_CODENAME}'. To resolve, please choose a compatible OS distribution"
                                err "Supported distributions include: ${DOCKER_LICENSED_ARCHIVE_VERSION_CODENAMES}"
                                exit 1
                            fi
                            echo "(*) ${VERSION_CODENAME} is supported for Docker CE installation (supported: ${DOCKER_LICENSED_ARCHIVE_VERSION_CODENAMES}) - setting up Docker repository"
                        elif [ "${ADJUSTED_ID}" = "rhel" ]; then

                            echo "RHEL-based system (${ID}) detected - using Docker CE packages"
                        fi
                    fi

                    # Install base dependencies
                    base_packages="curl ca-certificates pigz iptables gnupg2 wget jq"
                    case ${ADJUSTED_ID} in
                        debian)
                            check_packages apt-transport-https $base_packages dirmngr
                            ;;
                        rhel)
                            check_packages $base_packages tar gawk shadow-utils policycoreutils  procps-ng systemd-libs systemd-devel

                            ;;
                    esac

                    # Install git if not already present
                    if ! command -v git >/dev/null 2>&1; then
                        check_packages git
                    fi

                    # Update CA certificates to ensure HTTPS connections work properly
                    # This is especially important for Ubuntu 24.04 (Noble) and Debian Trixie
                    # Only run for Debian-based systems (RHEL uses update-ca-trust instead)
                    if [ "${ADJUSTED_ID}" = "debian" ] && command -v update-ca-certificates > /dev/null 2>&1; then
                        update-ca-certificates
                    fi

                    # Swap to legacy iptables for compatibility (Debian only)
                    if [ "${ADJUSTED_ID}" = "debian" ] && type iptables-legacy > /dev/null 2>&1; then
                        update-alternatives --set iptables /usr/sbin/iptables-legacy
                        update-alternatives --set ip6tables /usr/sbin/ip6tables-legacy
                    fi

                    # Set up the necessary repositories
                    if [ "${USE_MOBY}" = "true" ]; then
                        # Name of open source engine/cli
                        engine_package_name="moby-engine"
                        cli_package_name="moby-cli"

                        case ${ADJUSTED_ID} in
                            debian)
                                # Import key safely and import Microsoft apt repo
                                {
                                    curl -sSL ${MICROSOFT_GPG_KEYS_URI}
                                    curl -sSL ${MICROSOFT_GPG_KEYS_ROLLING_URI}
                                } | gpg --dearmor > /usr/share/keyrings/microsoft-archive-keyring.gpg
                                echo "deb [arch=${architecture} signed-by=/usr/share/keyrings/microsoft-archive-keyring.gpg] https://packages.microsoft.com/repos/microsoft-${ID}-${VERSION_CODENAME}-prod ${VERSION_CODENAME} main" > /etc/apt/sources.list.d/microsoft.list
                                ;;
                            rhel)
                                echo "(*) ${ID} detected - checking for Moby packages..."

                                # Check if moby packages are available in default repos
                                if ${PKG_MGR_CMD} list available moby-engine >/dev/null 2>&1; then
                                    echo "(*) Using built-in ${ID} Moby packages"
                                else
                                    case "${ID}" in
                                        azurelinux)
                                            echo "(*) Moby packages not found in Azure Linux repositories"
                                            echo "(*) For Azure Linux, Docker CE ('moby': false) is recommended"
                                            err "Moby packages are not available for Azure Linux ${VERSION_ID}."
                                            err "Recommendation: Use '\"moby\": false' to install Docker CE instead."
                                            exit 1
                                            ;;
                                        mariner)
                                            echo "(*) Adding Microsoft repository for CBL-Mariner..."
                                            # Add Microsoft repository if packages aren't available locally
                                            curl -sSL ${MICROSOFT_GPG_KEYS_URI} | gpg --dearmor > /etc/pki/rpm-gpg/microsoft.gpg
                                            cat > /etc/yum.repos.d/microsoft.repo << EOF
                    [microsoft]
                    name=Microsoft Repository
                    baseurl=https://packages.microsoft.com/repos/microsoft-cbl-mariner-2.0-prod-base/
                    enabled=1
                    gpgcheck=1
                    gpgkey=file:///etc/pki/rpm-gpg/microsoft.gpg
                    EOF
                                    # Verify packages are available after adding repo
                                    pkg_mgr_update
                                    if ! ${PKG_MGR_CMD} list available moby-engine >/dev/null 2>&1; then
                                        echo "(*) Moby packages not found in Microsoft repository either"
                                        err "Moby packages are not available for CBL-Mariner ${VERSION_ID}."
                                        err "Recommendation: Use '\"moby\": false' to install Docker CE instead."
                                        exit 1
                                    fi
                                    ;;
                                *)
                                    err "Moby packages are not available for ${ID}. Please use 'moby': false option."
                                    exit 1
                                    ;;
                                esac
                            fi
                            ;;
                        esac
                    else
                        # Name of licensed engine/cli
                        engine_package_name="docker-ce"
                        cli_package_name="docker-ce-cli"
                        case ${ADJUSTED_ID} in
                            debian)
                                curl -fsSL https://download.docker.com/linux/${ID}/gpg | gpg --dearmor > /usr/share/keyrings/docker-archive-keyring.gpg
                                echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/${ID} ${VERSION_CODENAME} stable" > /etc/apt/sources.list.d/docker.list
                                ;;
                            rhel)
                                # Docker CE repository setup for RHEL-based systems
                                setup_docker_ce_repo() {
                                    curl -fsSL https://download.docker.com/linux/centos/gpg > /etc/pki/rpm-gpg/docker-ce.gpg
                                    cat > /etc/yum.repos.d/docker-ce.repo << EOF
                    [docker-ce-stable]
                    name=Docker CE Stable
                    baseurl=https://download.docker.com/linux/centos/9/\$basearch/stable
                    enabled=1
                    gpgcheck=1
                    gpgkey=file:///etc/pki/rpm-gpg/docker-ce.gpg
                    skip_if_unavailable=1
                    module_hotfixes=1
                    EOF
                                }
                                install_azure_linux_deps() {
                                    echo "(*) Installing device-mapper libraries for Docker CE..."
                                    [ "${ID}" != "mariner" ] && ${PKG_MGR_CMD} -y install device-mapper-libs 2>/dev/null || echo "(*) Device-mapper install failed, proceeding"
                                    echo "(*) Installing additional Docker CE dependencies..."
                                    ${PKG_MGR_CMD} -y install libseccomp libtool-ltdl systemd-libs libcgroup tar xz || {
                                        echo "(*) Some optional dependencies could not be installed, continuing..."
                                    }
                                }
                                setup_selinux_context() {
                                    if command -v getenforce >/dev/null 2>&1 && [ "$(getenforce 2>/dev/null)" != "Disabled" ]; then
                                        echo "(*) Creating minimal SELinux context for Docker compatibility..."
                                        mkdir -p /etc/selinux/targeted/contexts/files/ 2>/dev/null || true
                                        echo "/var/lib/docker(/.*)? system_u:object_r:container_file_t:s0" >> /etc/selinux/targeted/contexts/files/file_contexts.local 2>/dev/null || true
                                    fi
                                }

                                # Special handling for RHEL Docker CE installation
                                case "${ID}" in
                                    azurelinux|mariner)
                                        echo "(*) ${ID} detected"
                                        echo "(*) Note: Moby packages work better on Azure Linux. Consider using 'moby': true"
                                        echo "(*) Setting up Docker CE repository..."

                                        setup_docker_ce_repo
                                        install_azure_linux_deps

                                        if [ "${USE_MOBY}" != "true" ]; then
                                            echo "(*) Docker CE installation for Azure Linux - skipping container-selinux"
                                            echo "(*) Note: SELinux policies will be minimal but Docker will function normally"
                                            setup_selinux_context
                                        else
                                            echo "(*) Using Moby - container-selinux not required"
                                        fi
                                        ;;
                                    *)
                                        # Standard RHEL/CentOS/Fedora approach
                                        if command -v dnf >/dev/null 2>&1; then
                                            dnf config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo
                                        elif command -v yum-config-manager >/dev/null 2>&1; then
                                            yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo
                                        else
                                            # Manual fallback
                                            setup_docker_ce_repo
                                fi
                                ;;
                            esac
                            ;;
                        esac
                    fi

                    # Refresh package database
                    case ${ADJUSTED_ID} in
                        debian)
                            apt-get update
                            ;;
                        rhel)
                            pkg_mgr_update
                            ;;
                    esac

                    # Soft version matching
                    if [ "${DOCKER_VERSION}" = "latest" ] || [ "${DOCKER_VERSION}" = "lts" ] || [ "${DOCKER_VERSION}" = "stable" ]; then
                        # Empty, meaning grab whatever "latest" is in apt repo
                        engine_version_suffix=""
                        cli_version_suffix=""
                    else
                        case ${ADJUSTED_ID} in
                            debian)
                        # Fetch a valid version from the apt-cache (eg: the Microsoft repo appends +azure, breakfix, etc...)
                        docker_version_dot_escaped="${DOCKER_VERSION//./\\.}"
                        docker_version_dot_plus_escaped="${docker_version_dot_escaped//+/\\+}"
                        # Regex needs to handle debian package version number format: https://www.systutorials.com/docs/linux/man/5-deb-version/
                        docker_version_regex="^(.+:)?${docker_version_dot_plus_escaped}([\\.\\+ ~:-]|$)"
                        set +e # Don't exit if finding version fails - will handle gracefully
                            cli_version_suffix="=$(apt-cache madison ${cli_package_name} | awk -F"|" '{print $2}' | sed -e 's/^[ \t]*//' | grep -E -m 1 "${docker_version_regex}")"
                            engine_version_suffix="=$(apt-cache madison ${engine_package_name} | awk -F"|" '{print $2}' | sed -e 's/^[ \t]*//' | grep -E -m 1 "${docker_version_regex}")"
                        set -e
                        if [ -z "${engine_version_suffix}" ] || [ "${engine_version_suffix}" = "=" ] || [ -z "${cli_version_suffix}" ] || [ "${cli_version_suffix}" = "=" ] ; then
                            err "No full or partial Docker / Moby version match found for \"${DOCKER_VERSION}\" on OS ${ID} ${VERSION_CODENAME} (${architecture}). Available versions:"
                            apt-cache madison ${cli_package_name} | awk -F"|" '{print $2}' | grep -oP '^(.+:)?\K.+'
                            exit 1
                        fi
                        ;;
                    rhel)
                         # For RHEL-based systems, use dnf/yum to find versions
                                docker_version_escaped="${DOCKER_VERSION//./\\.}"
                                set +e # Don't exit if finding version fails - will handle gracefully
                                    if [ "${USE_MOBY}" = "true" ]; then
                                        available_versions=$(${PKG_MGR_CMD} list --available moby-engine 2>/dev/null | grep -v "Available Packages" | awk '{print $2}' | grep -E "^${docker_version_escaped}" | head -1)
                                    else
                                        available_versions=$(${PKG_MGR_CMD} list --available docker-ce 2>/dev/null | grep -v "Available Packages" | awk '{print $2}' | grep -E "^${docker_version_escaped}" | head -1)
                                    fi
                                set -e
                                if [ -n "${available_versions}" ]; then
                                    engine_version_suffix="-${available_versions}"
                                    cli_version_suffix="-${available_versions}"
                                else
                                    echo "(*) Exact version ${DOCKER_VERSION} not found, using latest available"
                                    engine_version_suffix=""
                                    cli_version_suffix=""
                                fi
                                ;;
                        esac
                    fi

                    # Version matching for moby-buildx
                    if [ "${USE_MOBY}" = "true" ]; then
                        if [ "${MOBY_BUILDX_VERSION}" = "latest" ]; then
                            # Empty, meaning grab whatever "latest" is in apt repo
                            buildx_version_suffix=""
                        else
                            case ${ADJUSTED_ID} in
                                debian)
                            buildx_version_dot_escaped="${MOBY_BUILDX_VERSION//./\\.}"
                            buildx_version_dot_plus_escaped="${buildx_version_dot_escaped//+/\\+}"
                            buildx_version_regex="^(.+:)?${buildx_version_dot_plus_escaped}([\\.\\+ ~:-]|$)"
                            set +e
                                buildx_version_suffix="=$(apt-cache madison moby-buildx | awk -F"|" '{print $2}' | sed -e 's/^[ \t]*//' | grep -E -m 1 "${buildx_version_regex}")"
                            set -e
                            if [ -z "${buildx_version_suffix}" ] || [ "${buildx_version_suffix}" = "=" ]; then
                                err "No full or partial moby-buildx version match found for \"${MOBY_BUILDX_VERSION}\" on OS ${ID} ${VERSION_CODENAME} (${architecture}). Available versions:"
                                apt-cache madison moby-buildx | awk -F"|" '{print $2}' | grep -oP '^(.+:)?\K.+'
                                exit 1
                            fi
                            ;;
                                rhel)
                                    # For RHEL-based systems, try to find buildx version or use latest
                                    buildx_version_escaped="${MOBY_BUILDX_VERSION//./\\.}"
                                    set +e
                                    available_buildx=$(${PKG_MGR_CMD} list --available moby-buildx 2>/dev/null | grep -v "Available Packages" | awk '{print $2}' | grep -E "^${buildx_version_escaped}" | head -1)
                                    set -e
                                    if [ -n "${available_buildx}" ]; then
                                        buildx_version_suffix="-${available_buildx}"
                                    else
                                        echo "(*) Exact buildx version ${MOBY_BUILDX_VERSION} not found, using latest available"
                                        buildx_version_suffix=""
                                    fi
                                    ;;
                            esac
                            echo "buildx_version_suffix ${buildx_version_suffix}"
                        fi
                    fi

                    # Install Docker / Moby CLI if not already installed
                    if type docker > /dev/null 2>&1 && type dockerd > /dev/null 2>&1; then
                        echo "Docker / Moby CLI and Engine already installed."
                    else
                            case ${ADJUSTED_ID} in
                            debian)
                                if [ "${USE_MOBY}" = "true" ]; then
                                    # Install engine
                                    set +e # Handle error gracefully
                                        apt-get -y install --no-install-recommends moby-cli${cli_version_suffix} moby-buildx${buildx_version_suffix} moby-engine${engine_version_suffix}
                                        exit_code=$?
                                    set -e

                                    if [ ${exit_code} -ne 0 ]; then
                                        err "Packages for moby not available in OS ${ID} ${VERSION_CODENAME} (${architecture}). To resolve, either: (1) set feature option '\"moby\": false' , or (2) choose a compatible OS version (eg: 'ubuntu-24.04')."
                                        exit 1
                                    fi

                                    # Install compose
                                    apt-get -y install --no-install-recommends moby-compose || err "Package moby-compose (Docker Compose v2) not available for OS ${ID} ${VERSION_CODENAME} (${architecture}). Skipping."
                                else
                                    apt-get -y install --no-install-recommends docker-ce-cli${cli_version_suffix} docker-ce${engine_version_suffix}
                                    # Install compose
                                    apt-mark hold docker-ce docker-ce-cli
                                    apt-get -y install --no-install-recommends docker-compose-plugin || echo "(*) Package docker-compose-plugin (Docker Compose v2) not available for OS ${ID} ${VERSION_CODENAME} (${architecture}). Skipping."
                                fi
                                ;;
                            rhel)
                                if [ "${USE_MOBY}" = "true" ]; then
                                    set +e # Handle error gracefully
                                        ${PKG_MGR_CMD} -y install moby-cli${cli_version_suffix} moby-engine${engine_version_suffix}
                                        exit_code=$?
                                    set -e

                                    if [ ${exit_code} -ne 0 ]; then
                                        err "Packages for moby not available in OS ${ID} ${VERSION_CODENAME} (${architecture}). To resolve, either: (1) set feature option '\"moby\": false' , or (2) choose a compatible OS version."
                                        exit 1
                                    fi

                                    # Install compose
                                    if [ "${DOCKER_DASH_COMPOSE_VERSION}" != "none" ]; then
                                        ${PKG_MGR_CMD} -y install moby-compose || echo "(*) Package moby-compose not available for ${ID} ${VERSION_CODENAME} (${architecture}). Skipping."
                                    fi
                                else
                                                   # Special handling for Azure Linux Docker CE installation
                                    if [ "${ID}" = "azurelinux" ] || [ "${ID}" = "mariner" ]; then
                                        echo "(*) Installing Docker CE on Azure Linux (bypassing container-selinux dependency)..."

                                        # Use rpm with --force and --nodeps for Azure Linux
                                        set +e  # Don't exit on error for this section
                                        ${PKG_MGR_CMD} -y install docker-ce${cli_version_suffix} docker-ce-cli${engine_version_suffix} containerd.io
                                        install_result=$?
                                        set -e

                                        if [ $install_result -ne 0 ]; then
                                            echo "(*) Standard installation failed, trying manual installation..."

                                            echo "(*) Standard installation failed, trying manual installation..."

                                            # Create directory for downloading packages
                                            mkdir -p /tmp/docker-ce-install

                                            # Download packages manually using curl since tdnf doesn't support download
                                            echo "(*) Downloading Docker CE packages manually..."

                                            # Get the repository baseurl
                                            repo_baseurl="https://download.docker.com/linux/centos/9/x86_64/stable"

                                            # Download packages directly
                                            cd /tmp/docker-ce-install

                                            # Get package names with versions
                                            if [ -n "${cli_version_suffix}" ]; then
                                                docker_ce_version="${cli_version_suffix#-}"
                                                docker_cli_version="${engine_version_suffix#-}"
                                            else
                                                # Get latest version from repository
                                                docker_ce_version="latest"
                                            fi

                                            echo "(*) Attempting to download Docker CE packages from repository..."

                                            # Try to download latest packages if specific version fails
                                            if ! curl -fsSL "${repo_baseurl}/Packages/docker-ce-${docker_ce_version}.el9.x86_64.rpm" -o docker-ce.rpm 2>/dev/null; then
                                                # Fallback: try to get latest available version
                                                echo "(*) Specific version not found, trying latest..."
                                                latest_docker=$(curl -s "${repo_baseurl}/Packages/" | grep -o 'docker-ce-[0-9][^"]*\.el9\.x86_64\.rpm' | head -1)
                                                latest_cli=$(curl -s "${repo_baseurl}/Packages/" | grep -o 'docker-ce-cli-[0-9][^"]*\.el9\.x86_64\.rpm' | head -1)
                                                latest_containerd=$(curl -s "${repo_baseurl}/Packages/" | grep -o 'containerd\.io-[0-9][^"]*\.el9\.x86_64\.rpm' | head -1)

                                                if [ -n "${latest_docker}" ]; then
                                                    curl -fsSL "${repo_baseurl}/Packages/${latest_docker}" -o docker-ce.rpm
                                                    curl -fsSL "${repo_baseurl}/Packages/${latest_cli}" -o docker-ce-cli.rpm
                                                    curl -fsSL "${repo_baseurl}/Packages/${latest_containerd}" -o containerd.io.rpm
                                                else
                                                    echo "(*) ERROR: Could not find Docker CE packages in repository"
                                                    echo "(*) Please check repository configuration or use 'moby': true"
                                                    exit 1
                                                fi
                                            fi
                                            # Install systemd libraries required by Docker CE
                                            echo "(*) Installing systemd libraries required by Docker CE..."
                                            ${PKG_MGR_CMD} -y install systemd-libs || ${PKG_MGR_CMD} -y install systemd-devel || {
                                                echo "(*) WARNING: Could not install systemd libraries"
                                                echo "(*) Docker may fail to start without these"
                                            }

                                            # Install with rpm --force --nodeps
                                            echo "(*) Installing Docker CE packages with dependency override..."
                                            rpm -Uvh --force --nodeps *.rpm

                                            # Cleanup
                                            cd /
                                            rm -rf /tmp/docker-ce-install

                                            echo "(*) Docker CE installation completed with dependency bypass"
                                            echo "(*) Note: Some SELinux functionality may be limited without container-selinux"
                                        fi
                                    else
                                        # Standard installation for other RHEL-based systems
                                        ${PKG_MGR_CMD} -y install docker-ce${cli_version_suffix} docker-ce-cli${engine_version_suffix} containerd.io
                                    fi
                                    # Install compose
                                    if [ "${DOCKER_DASH_COMPOSE_VERSION}" != "none" ]; then
                                        ${PKG_MGR_CMD} -y install docker-compose-plugin || echo "(*) Package docker-compose-plugin not available for ${ID} ${VERSION_CODENAME} (${architecture}). Skipping."
                                    fi
                                fi
                                ;;
                        esac
                    fi

                    echo "Finished installing docker / moby!"

                    docker_home="/usr/libexec/docker"
                    cli_plugins_dir="${docker_home}/cli-plugins"

                    # fallback for docker-compose
                    fallback_compose(){
                        local url=$1
                        local repo_url=$(get_github_api_repo_url "$url")
                        echo -e "\n(!) Failed to fetch the latest artifacts for docker-compose v${compose_version}..."
                        get_previous_version "${url}" "${repo_url}" compose_version
                        echo -e "\nAttempting to install v${compose_version}"
                        curl -fsSL "https://github.com/docker/compose/releases/download/v${compose_version}/docker-compose-linux-${target_compose_arch}" -o ${docker_compose_path}
                    }

                    # If 'docker-compose' command is to be included
                    if [ "${DOCKER_DASH_COMPOSE_VERSION}" != "none" ]; then
                        case "${architecture}" in
                        amd64|x86_64) target_compose_arch=x86_64 ;;
                        arm64|aarch64) target_compose_arch=aarch64 ;;
                        *)
                            echo "(!) Docker in docker does not support machine architecture '$architecture'. Please use an x86-64 or ARM64 machine."
                            exit 1
                        esac

                        docker_compose_path="/usr/local/bin/docker-compose"
                        if [ "${DOCKER_DASH_COMPOSE_VERSION}" = "v1" ]; then
                            err "The final Compose V1 release, version 1.29.2, was May 10, 2021. These packages haven't received any security updates since then. Use at your own risk."
                            INSTALL_DOCKER_COMPOSE_SWITCH="false"

                            if [ "${target_compose_arch}" = "x86_64" ]; then
                                echo "(*) Installing docker compose v1..."
                                curl -fsSL "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-Linux-x86_64" -o ${docker_compose_path}
                                chmod +x ${docker_compose_path}

                                # Download the SHA256 checksum
                                DOCKER_COMPOSE_SHA256="$(curl -sSL "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-Linux-x86_64.sha256" | awk '{print $1}')"
                                echo "${DOCKER_COMPOSE_SHA256}  ${docker_compose_path}" > docker-compose.sha256sum
                                sha256sum -c docker-compose.sha256sum --ignore-missing
                            elif [ "${VERSION_CODENAME}" = "bookworm" ]; then
                                err "Docker compose v1 is unavailable for 'bookworm' on Arm64. Kindly switch to use v2"
                                exit 1
                            else
                                # Use pip to get a version that runs on this architecture
                                check_packages python3-minimal python3-pip libffi-dev python3-venv
                                echo "(*) Installing docker compose v1 via pip..."
                                export PYTHONUSERBASE=/usr/local
                                pip3 install --disable-pip-version-check --no-cache-dir --user "Cython<3.0" pyyaml wheel docker-compose --no-build-isolation
                            fi
                        else
                            compose_version=${DOCKER_DASH_COMPOSE_VERSION#v}
                            docker_compose_url="https://github.com/docker/compose"
                            find_version_from_git_tags compose_version "$docker_compose_url" "tags/v"
                            echo "(*) Installing docker-compose ${compose_version}..."
                            curl -fsSL "https://github.com/docker/compose/releases/download/v${compose_version}/docker-compose-linux-${target_compose_arch}" -o ${docker_compose_path} || {
                                     echo -e "\n(!) Failed to fetch the latest artifacts for docker-compose v${compose_version}..."
                                     fallback_compose "$docker_compose_url"
                            }

                            chmod +x ${docker_compose_path}

                            # Download the SHA256 checksum
                            DOCKER_COMPOSE_SHA256="$(curl -sSL "https://github.com/docker/compose/releases/download/v${compose_version}/docker-compose-linux-${target_compose_arch}.sha256" | awk '{print $1}')"
                            echo "${DOCKER_COMPOSE_SHA256}  ${docker_compose_path}" > docker-compose.sha256sum
                            sha256sum -c docker-compose.sha256sum --ignore-missing

                            mkdir -p ${cli_plugins_dir}
                            cp ${docker_compose_path} ${cli_plugins_dir}
                        fi
                    fi

                    # fallback method for compose-switch
                    fallback_compose-switch() {
                        local url=$1
                        local repo_url=$(get_github_api_repo_url "$url")
                        echo -e "\n(!) Failed to fetch the latest artifacts for compose-switch v${compose_switch_version}..."
                        get_previous_version "$url" "$repo_url" compose_switch_version
                        echo -e "\nAttempting to install v${compose_switch_version}"
                        curl -fsSL "https://github.com/docker/compose-switch/releases/download/v${compose_switch_version}/docker-compose-linux-${target_switch_arch}" -o /usr/local/bin/compose-switch
                    }
                    # Install docker-compose switch if not already installed - https://github.com/docker/compose-switch#manual-installation
                    if [ "${INSTALL_DOCKER_COMPOSE_SWITCH}" = "true" ] && ! type compose-switch > /dev/null 2>&1; then
                        if type docker-compose > /dev/null 2>&1; then
                            echo "(*) Installing compose-switch..."
                            current_compose_path="$(command -v docker-compose)"
                            target_compose_path="$(dirname "${current_compose_path}")/docker-compose-v1"
                            compose_switch_version="latest"
                            compose_switch_url="https://github.com/docker/compose-switch"
                            # Try to get latest version, fallback to known stable version if GitHub API fails
                            set +e
                            find_version_from_git_tags compose_switch_version "$compose_switch_url"
                            if [ $? -ne 0 ] || [ -z "${compose_switch_version}" ] || [ "${compose_switch_version}" = "latest" ]; then
                                echo "(*) GitHub API rate limited or failed, using fallback method"
                                fallback_compose-switch "$compose_switch_url"
                            fi
                            set -e

                            # Map architecture for compose-switch downloads
                            case "${architecture}" in
                                amd64|x86_64) target_switch_arch=amd64 ;;
                                arm64|aarch64) target_switch_arch=arm64 ;;
                                *) target_switch_arch=${architecture} ;;
                            esac
                            curl -fsSL "https://github.com/docker/compose-switch/releases/download/v${compose_switch_version}/docker-compose-linux-${target_switch_arch}" -o /usr/local/bin/compose-switch || fallback_compose-switch "$compose_switch_url"
                            chmod +x /usr/local/bin/compose-switch
                            # TODO: Verify checksum once available: https://github.com/docker/compose-switch/issues/11
                            # Setup v1 CLI as alternative in addition to compose-switch (which maps to v2)
                            mv "${current_compose_path}" "${target_compose_path}"
                            update-alternatives --install ${docker_compose_path} docker-compose /usr/local/bin/compose-switch 99
                            update-alternatives --install ${docker_compose_path} docker-compose "${target_compose_path}" 1
                        else
                            err "Skipping installation of compose-switch as docker compose is unavailable..."
                        fi
                    fi

                    # If init file already exists, exit
                    if [ -f "/usr/local/share/docker-init.sh" ]; then
                        echo "/usr/local/share/docker-init.sh already exists, so exiting."
                        # Clean up
                        rm -rf /var/lib/apt/lists/*
                        exit 0
                    fi
                    echo "docker-init doesn't exist, adding..."

                    if ! cat /etc/group | grep -e "^docker:" > /dev/null 2>&1; then
                            groupadd -r docker
                    fi

                    usermod -aG docker ${USERNAME}

                    # fallback for docker/buildx
                    fallback_buildx() {
                        local url=$1
                        local repo_url=$(get_github_api_repo_url "$url")
                        echo -e "\n(!) Failed to fetch the latest artifacts for docker buildx v${buildx_version}..."
                        get_previous_version "$url" "$repo_url" buildx_version
                        buildx_file_name="buildx-v${buildx_version}.linux-${target_buildx_arch}"
                        echo -e "\nAttempting to install v${buildx_version}"
                        wget https://github.com/docker/buildx/releases/download/v${buildx_version}/${buildx_file_name}
                    }

                    if [ "${INSTALL_DOCKER_BUILDX}" = "true" ]; then
                        buildx_version="latest"
                        docker_buildx_url="https://github.com/docker/buildx"
                        find_version_from_git_tags buildx_version "$docker_buildx_url" "refs/tags/v"
                        echo "(*) Installing buildx ${buildx_version}..."

                          # Map architecture for buildx downloads
                        case "${architecture}" in
                            amd64|x86_64) target_buildx_arch=amd64 ;;
                            arm64|aarch64) target_buildx_arch=arm64 ;;
                            *) target_buildx_arch=${architecture} ;;
                        esac

                        buildx_file_name="buildx-v${buildx_version}.linux-${target_buildx_arch}"

                        cd /tmp
                        wget https://github.com/docker/buildx/releases/download/v${buildx_version}/${buildx_file_name} || fallback_buildx "$docker_buildx_url"

                        docker_home="/usr/libexec/docker"
                        cli_plugins_dir="${docker_home}/cli-plugins"

                        mkdir -p ${cli_plugins_dir}
                        mv ${buildx_file_name} ${cli_plugins_dir}/docker-buildx
                        chmod +x ${cli_plugins_dir}/docker-buildx

                        chown -R "${USERNAME}:docker" "${docker_home}"
                        chmod -R g+r+w "${docker_home}"
                        find "${docker_home}" -type d -print0 | xargs -n 1 -0 chmod g+s
                    fi

                    DOCKER_DEFAULT_IP6_TABLES=""
                    if [ "$DISABLE_IP6_TABLES" == true ]; then
                        requested_version=""
                        # checking whether the version requested either is in semver format or just a number denoting the major version
                        # and, extracting the major version number out of the two scenarios
                        semver_regex="^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-([0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*))?(\+([0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*))?$"
                        if echo "$DOCKER_VERSION" | grep -Eq $semver_regex; then
                            requested_version=$(echo $DOCKER_VERSION | cut -d. -f1)
                        elif echo "$DOCKER_VERSION" | grep -Eq "^[1-9][0-9]*$"; then
                            requested_version=$DOCKER_VERSION
                        fi
                        if [ "$DOCKER_VERSION" = "latest" ] || [[ -n "$requested_version" && "$requested_version" -ge 27 ]] ; then
                            DOCKER_DEFAULT_IP6_TABLES="--ip6tables=false"
                            echo "(!) As requested, passing '${DOCKER_DEFAULT_IP6_TABLES}'"
                        fi
                    fi

                    if [ ! -d /usr/local/share ]; then
                        mkdir -p /usr/local/share
                    fi

                    tee /usr/local/share/docker-init.sh > /dev/null \
                    << EOF
                    #!/bin/sh
                    #-------------------------------------------------------------------------------------------------------------
                    # Copyright (c) Microsoft Corporation. All rights reserved.
                    # Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
                    #-------------------------------------------------------------------------------------------------------------

                    set -e

                    AZURE_DNS_AUTO_DETECTION=${AZURE_DNS_AUTO_DETECTION}
                    DOCKER_DEFAULT_ADDRESS_POOL=${DOCKER_DEFAULT_ADDRESS_POOL}
                    DOCKER_DEFAULT_IP6_TABLES=${DOCKER_DEFAULT_IP6_TABLES}
                    EOF

                    tee -a /usr/local/share/docker-init.sh > /dev/null \
                    << 'EOF'
                    dockerd_start="AZURE_DNS_AUTO_DETECTION=${AZURE_DNS_AUTO_DETECTION} DOCKER_DEFAULT_ADDRESS_POOL=${DOCKER_DEFAULT_ADDRESS_POOL} DOCKER_DEFAULT_IP6_TABLES=${DOCKER_DEFAULT_IP6_TABLES} $(cat << 'INNEREOF'
                        # explicitly remove dockerd and containerd PID file to ensure that it can start properly if it was stopped uncleanly
                        find /run /var/run -iname 'docker*.pid' -delete || :
                        find /run /var/run -iname 'container*.pid' -delete || :

                        # -- Start: dind wrapper script --
                        # Maintained: https://github.com/moby/moby/blob/master/hack/dind

                        export container=docker

                        if [ -d /sys/kernel/security ] && ! mountpoint -q /sys/kernel/security; then
                            mount -t securityfs none /sys/kernel/security || {
                                echo >&2 'Could not mount /sys/kernel/security.'
                                echo >&2 'AppArmor detection and --privileged mode might break.'
                            }
                        fi

                        # Mount /tmp (conditionally)
                        if ! mountpoint -q /tmp; then
                            mount -t tmpfs none /tmp
                        fi

                        set_cgroup_nesting()
                        {
                            # cgroup v2: enable nesting
                            if [ -f /sys/fs/cgroup/cgroup.controllers ]; then
                                # move the processes from the root group to the /init group,
                                # otherwise writing subtree_control fails with EBUSY.
                                # An error during moving non-existent process (i.e., "cat") is ignored.
                                mkdir -p /sys/fs/cgroup/init
                                xargs -rn1 < /sys/fs/cgroup/cgroup.procs > /sys/fs/cgroup/init/cgroup.procs || :
                                # enable controllers
                                sed -e 's/ / +/g' -e 's/^/+/' < /sys/fs/cgroup/cgroup.controllers \
                                    > /sys/fs/cgroup/cgroup.subtree_control
                            fi
                        }

                        # Set cgroup nesting, retrying if necessary
                        retry_cgroup_nesting=0

                        until [ "${retry_cgroup_nesting}" -eq "5" ];
                        do
                            set +e
                                set_cgroup_nesting

                                if [ $? -ne 0 ]; then
                                    echo "(*) cgroup v2: Failed to enable nesting, retrying..."
                                else
                                    break
                                fi

                                retry_cgroup_nesting=`expr $retry_cgroup_nesting + 1`
                            set -e
                        done

                        # -- End: dind wrapper script --

                        # Handle DNS
                        set +e
                            cat /etc/resolv.conf | grep -i 'internal.cloudapp.net' > /dev/null 2>&1
                            if [ $? -eq 0 ] && [ "${AZURE_DNS_AUTO_DETECTION}" = "true" ]
                            then
                                echo "Setting dockerd Azure DNS."
                                CUSTOMDNS="--dns 168.63.129.16"
                            else
                                echo "Not setting dockerd DNS manually."
                                CUSTOMDNS=""
                            fi
                        set -e

                        if [ -z "$DOCKER_DEFAULT_ADDRESS_POOL" ]
                        then
                            DEFAULT_ADDRESS_POOL=""
                        else
                            DEFAULT_ADDRESS_POOL="--default-address-pool $DOCKER_DEFAULT_ADDRESS_POOL"
                        fi

                        # Start docker/moby engine
                        ( dockerd $CUSTOMDNS $DEFAULT_ADDRESS_POOL $DOCKER_DEFAULT_IP6_TABLES > /tmp/dockerd.log 2>&1 ) &
                    INNEREOF
                    )"

                    sudo_if() {
                        COMMAND="$*"

                        if [ "$(id -u)" -ne 0 ]; then
                            sudo $COMMAND
                        else
                            $COMMAND
                        fi
                    }

                    retry_docker_start_count=0
                    docker_ok="false"

                    until [ "${docker_ok}" = "true"  ] || [ "${retry_docker_start_count}" -eq "5" ];
                    do
                        # Start using sudo if not invoked as root
                        if [ "$(id -u)" -ne 0 ]; then
                            sudo /bin/sh -c "${dockerd_start}"
                        else
                            eval "${dockerd_start}"
                        fi

                        retry_count=0
                        until [ "${docker_ok}" = "true"  ] || [ "${retry_count}" -eq "5" ];
                        do
                            sleep 1s
                            set +e
                                docker info > /dev/null 2>&1 && docker_ok="true"
                            set -e

                            retry_count=`expr $retry_count + 1`
                        done

                        if [ "${docker_ok}" != "true" ] && [ "${retry_docker_start_count}" != "4" ]; then
                            echo "(*) Failed to start docker, retrying..."
                            set +e
                                sudo_if pkill dockerd
                                sudo_if pkill containerd
                            set -e
                        fi

                        retry_docker_start_count=`expr $retry_docker_start_count + 1`
                    done

                    # Execute whatever commands were passed in (if any). This allows us
                    # to set this script to ENTRYPOINT while still executing the default CMD.
                    exec "$@"
                    EOF

                    chmod +x /usr/local/share/docker-init.sh
                    chown ${USERNAME}:root /usr/local/share/docker-init.sh

                    # Clean up
                    rm -rf /var/lib/apt/lists/*

                    echo 'docker-in-docker-debian script has completed!'"#),
                ]).await;

                return Ok(http::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(response))
                    .unwrap());
            }
            if parts.uri.path() == "/v2/devcontainers/features/go/manifests/1" {
                let response = r#"
                    {
                        "schemaVersion": 2,
                        "mediaType": "application/vnd.oci.image.manifest.v1+json",
                        "config": {
                            "mediaType": "application/vnd.devcontainers",
                            "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
                            "size": 2
                        },
                        "layers": [
                            {
                                "mediaType": "application/vnd.devcontainers.layer.v1+tar",
                                "digest": "sha256:eadd8a4757ee8ea6c1bc0aae22da49b7e5f2f1e32a87a5eac3cadeb7d2ccdad1",
                                "size": 20992,
                                "annotations": {
                                    "org.opencontainers.image.title": "devcontainer-feature-go.tgz"
                                }
                            }
                        ],
                        "annotations": {
                            "dev.containers.metadata": "{\"id\":\"go\",\"version\":\"1.3.3\",\"name\":\"Go\",\"documentationURL\":\"https://github.com/devcontainers/features/tree/main/src/go\",\"description\":\"Installs Go and common Go utilities. Auto-detects latest version and installs needed dependencies.\",\"options\":{\"version\":{\"type\":\"string\",\"proposals\":[\"latest\",\"none\",\"1.24\",\"1.23\"],\"default\":\"latest\",\"description\":\"Select or enter a Go version to install\"},\"golangciLintVersion\":{\"type\":\"string\",\"default\":\"latest\",\"description\":\"Version of golangci-lint to install\"}},\"init\":true,\"customizations\":{\"vscode\":{\"extensions\":[\"golang.Go\"],\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes Go and common Go utilities pre-installed and available on the `PATH`, along with the Go language extension for Go development.\"}]}}},\"containerEnv\":{\"GOROOT\":\"/usr/local/go\",\"GOPATH\":\"/go\",\"PATH\":\"/usr/local/go/bin:/go/bin:${PATH}\"},\"capAdd\":[\"SYS_PTRACE\"],\"securityOpt\":[\"seccomp=unconfined\"],\"installsAfter\":[\"ghcr.io/devcontainers/features/common-utils\"]}",
                            "com.github.package.type": "devcontainer_feature"
                        }
                    }
                    "#;

                return Ok(http::Response::builder()
                    .status(200)
                    .body(http_client::AsyncBody::from(response))
                    .unwrap());
            }
            if parts.uri.path()
                == "/v2/devcontainers/features/go/blobs/sha256:eadd8a4757ee8ea6c1bc0aae22da49b7e5f2f1e32a87a5eac3cadeb7d2ccdad1"
            {
                let response = build_tarball(vec![
                    ("./devcontainer-feature.json", r#"
                        {
                            "id": "go",
                            "version": "1.3.3",
                            "name": "Go",
                            "documentationURL": "https://github.com/devcontainers/features/tree/main/src/go",
                            "description": "Installs Go and common Go utilities. Auto-detects latest version and installs needed dependencies.",
                            "options": {
                                "version": {
                                    "type": "string",
                                    "proposals": [
                                        "latest",
                                        "none",
                                        "1.24",
                                        "1.23"
                                    ],
                                    "default": "latest",
                                    "description": "Select or enter a Go version to install"
                                },
                                "golangciLintVersion": {
                                    "type": "string",
                                    "default": "latest",
                                    "description": "Version of golangci-lint to install"
                                }
                            },
                            "init": true,
                            "customizations": {
                                "vscode": {
                                    "extensions": [
                                        "golang.Go"
                                    ],
                                    "settings": {
                                        "github.copilot.chat.codeGeneration.instructions": [
                                            {
                                                "text": "This dev container includes Go and common Go utilities pre-installed and available on the `PATH`, along with the Go language extension for Go development."
                                            }
                                        ]
                                    }
                                }
                            },
                            "containerEnv": {
                                "GOROOT": "/usr/local/go",
                                "GOPATH": "/go",
                                "PATH": "/usr/local/go/bin:/go/bin:${PATH}"
                            },
                            "capAdd": [
                                "SYS_PTRACE"
                            ],
                            "securityOpt": [
                                "seccomp=unconfined"
                            ],
                            "installsAfter": [
                                "ghcr.io/devcontainers/features/common-utils"
                            ]
                        }
                        "#),
                    ("./install.sh", r#"
                    #!/usr/bin/env bash
                    #-------------------------------------------------------------------------------------------------------------
                    # Copyright (c) Microsoft Corporation. All rights reserved.
                    # Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information
                    #-------------------------------------------------------------------------------------------------------------
                    #
                    # Docs: https://github.com/microsoft/vscode-dev-containers/blob/main/script-library/docs/go.md
                    # Maintainer: The VS Code and Codespaces Teams

                    TARGET_GO_VERSION="${VERSION:-"latest"}"
                    GOLANGCILINT_VERSION="${GOLANGCILINTVERSION:-"latest"}"

                    TARGET_GOROOT="${TARGET_GOROOT:-"/usr/local/go"}"
                    TARGET_GOPATH="${TARGET_GOPATH:-"/go"}"
                    USERNAME="${USERNAME:-"${_REMOTE_USER:-"automatic"}"}"
                    INSTALL_GO_TOOLS="${INSTALL_GO_TOOLS:-"true"}"

                    # https://www.google.com/linuxrepositories/
                    GO_GPG_KEY_URI="https://dl.google.com/linux/linux_signing_key.pub"

                    set -e

                    if [ "$(id -u)" -ne 0 ]; then
                        echo -e 'Script must be run as root. Use sudo, su, or add "USER root" to your Dockerfile before running this script.'
                        exit 1
                    fi

                    # Bring in ID, ID_LIKE, VERSION_ID, VERSION_CODENAME
                    . /etc/os-release
                    # Get an adjusted ID independent of distro variants
                    MAJOR_VERSION_ID=$(echo ${VERSION_ID} | cut -d . -f 1)
                    if [ "${ID}" = "debian" ] || [ "${ID_LIKE}" = "debian" ]; then
                        ADJUSTED_ID="debian"
                    elif [[ "${ID}" = "rhel" || "${ID}" = "fedora" || "${ID}" = "mariner" || "${ID_LIKE}" = *"rhel"* || "${ID_LIKE}" = *"fedora"* || "${ID_LIKE}" = *"mariner"* ]]; then
                        ADJUSTED_ID="rhel"
                        if [[ "${ID}" = "rhel" ]] || [[ "${ID}" = *"alma"* ]] || [[ "${ID}" = *"rocky"* ]]; then
                            VERSION_CODENAME="rhel${MAJOR_VERSION_ID}"
                        else
                            VERSION_CODENAME="${ID}${MAJOR_VERSION_ID}"
                        fi
                    else
                        echo "Linux distro ${ID} not supported."
                        exit 1
                    fi

                    if [ "${ADJUSTED_ID}" = "rhel" ] && [ "${VERSION_CODENAME-}" = "centos7" ]; then
                        # As of 1 July 2024, mirrorlist.centos.org no longer exists.
                        # Update the repo files to reference vault.centos.org.
                        sed -i s/mirror.centos.org/vault.centos.org/g /etc/yum.repos.d/*.repo
                        sed -i s/^#.*baseurl=http/baseurl=http/g /etc/yum.repos.d/*.repo
                        sed -i s/^mirrorlist=http/#mirrorlist=http/g /etc/yum.repos.d/*.repo
                    fi

                    # Setup INSTALL_CMD & PKG_MGR_CMD
                    if type apt-get > /dev/null 2>&1; then
                        PKG_MGR_CMD=apt-get
                        INSTALL_CMD="${PKG_MGR_CMD} -y install --no-install-recommends"
                    elif type microdnf > /dev/null 2>&1; then
                        PKG_MGR_CMD=microdnf
                        INSTALL_CMD="${PKG_MGR_CMD} ${INSTALL_CMD_ADDL_REPOS} -y install --refresh --best --nodocs --noplugins --setopt=install_weak_deps=0"
                    elif type dnf > /dev/null 2>&1; then
                        PKG_MGR_CMD=dnf
                        INSTALL_CMD="${PKG_MGR_CMD} ${INSTALL_CMD_ADDL_REPOS} -y install --refresh --best --nodocs --noplugins --setopt=install_weak_deps=0"
                    else
                        PKG_MGR_CMD=yum
                        INSTALL_CMD="${PKG_MGR_CMD} ${INSTALL_CMD_ADDL_REPOS} -y install --noplugins --setopt=install_weak_deps=0"
                    fi

                    # Clean up
                    clean_up() {
                        case ${ADJUSTED_ID} in
                            debian)
                                rm -rf /var/lib/apt/lists/*
                                ;;
                            rhel)
                                rm -rf /var/cache/dnf/* /var/cache/yum/*
                                rm -rf /tmp/yum.log
                                rm -rf ${GPG_INSTALL_PATH}
                                ;;
                        esac
                    }
                    clean_up


                    # Figure out correct version of a three part version number is not passed
                    find_version_from_git_tags() {
                        local variable_name=$1
                        local requested_version=${!variable_name}
                        if [ "${requested_version}" = "none" ]; then return; fi
                        local repository=$2
                        local prefix=${3:-"tags/v"}
                        local separator=${4:-"."}
                        local last_part_optional=${5:-"false"}
                        if [ "$(echo "${requested_version}" | grep -o "." | wc -l)" != "2" ]; then
                            local escaped_separator=${separator//./\\.}
                            local last_part
                            if [ "${last_part_optional}" = "true" ]; then
                                last_part="(${escaped_separator}[0-9]+)?"
                            else
                                last_part="${escaped_separator}[0-9]+"
                            fi
                            local regex="${prefix}\\K[0-9]+${escaped_separator}[0-9]+${last_part}$"
                            local version_list="$(git ls-remote --tags ${repository} | grep -oP "${regex}" | tr -d ' ' | tr "${separator}" "." | sort -rV)"
                            if [ "${requested_version}" = "latest" ] || [ "${requested_version}" = "current" ] || [ "${requested_version}" = "lts" ]; then
                                declare -g ${variable_name}="$(echo "${version_list}" | head -n 1)"
                            else
                                set +e
                                declare -g ${variable_name}="$(echo "${version_list}" | grep -E -m 1 "^${requested_version//./\\.}([\\.\\s]|$)")"
                                set -e
                            fi
                        fi
                        if [ -z "${!variable_name}" ] || ! echo "${version_list}" | grep "^${!variable_name//./\\.}$" > /dev/null 2>&1; then
                            echo -e "Invalid ${variable_name} value: ${requested_version}\nValid values:\n${version_list}" >&2
                            exit 1
                        fi
                        echo "${variable_name}=${!variable_name}"
                    }

                    pkg_mgr_update() {
                        case $ADJUSTED_ID in
                            debian)
                                if [ "$(find /var/lib/apt/lists/* | wc -l)" = "0" ]; then
                                    echo "Running apt-get update..."
                                    ${PKG_MGR_CMD} update -y
                                fi
                                ;;
                            rhel)
                                if [ ${PKG_MGR_CMD} = "microdnf" ]; then
                                    if [ "$(ls /var/cache/yum/* 2>/dev/null | wc -l)" = 0 ]; then
                                        echo "Running ${PKG_MGR_CMD} makecache ..."
                                        ${PKG_MGR_CMD} makecache
                                    fi
                                else
                                    if [ "$(ls /var/cache/${PKG_MGR_CMD}/* 2>/dev/null | wc -l)" = 0 ]; then
                                        echo "Running ${PKG_MGR_CMD} check-update ..."
                                        set +e
                                        ${PKG_MGR_CMD} check-update
                                        rc=$?
                                        if [ $rc != 0 ] && [ $rc != 100 ]; then
                                            exit 1
                                        fi
                                        set -e
                                    fi
                                fi
                                ;;
                        esac
                    }

                    # Checks if packages are installed and installs them if not
                    check_packages() {
                        case ${ADJUSTED_ID} in
                            debian)
                                if ! dpkg -s "$@" > /dev/null 2>&1; then
                                    pkg_mgr_update
                                    ${INSTALL_CMD} "$@"
                                fi
                                ;;
                            rhel)
                                if ! rpm -q "$@" > /dev/null 2>&1; then
                                    pkg_mgr_update
                                    ${INSTALL_CMD} "$@"
                                fi
                                ;;
                        esac
                    }

                    # Ensure that login shells get the correct path if the user updated the PATH using ENV.
                    rm -f /etc/profile.d/00-restore-env.sh
                    echo "export PATH=${PATH//$(sh -lc 'echo $PATH')/\$PATH}" > /etc/profile.d/00-restore-env.sh
                    chmod +x /etc/profile.d/00-restore-env.sh

                    # Some distributions do not install awk by default (e.g. Mariner)
                    if ! type awk >/dev/null 2>&1; then
                        check_packages awk
                    fi

                    # Determine the appropriate non-root user
                    if [ "${USERNAME}" = "auto" ] || [ "${USERNAME}" = "automatic" ]; then
                        USERNAME=""
                        POSSIBLE_USERS=("vscode" "node" "codespace" "$(awk -v val=1000 -F ":" '$3==val{print $1}' /etc/passwd)")
                        for CURRENT_USER in "${POSSIBLE_USERS[@]}"; do
                            if id -u ${CURRENT_USER} > /dev/null 2>&1; then
                                USERNAME=${CURRENT_USER}
                                break
                            fi
                        done
                        if [ "${USERNAME}" = "" ]; then
                            USERNAME=root
                        fi
                    elif [ "${USERNAME}" = "none" ] || ! id -u ${USERNAME} > /dev/null 2>&1; then
                        USERNAME=root
                    fi

                    export DEBIAN_FRONTEND=noninteractive

                    check_packages ca-certificates gnupg2 tar gcc make pkg-config

                    if [ $ADJUSTED_ID = "debian" ]; then
                        check_packages g++ libc6-dev
                    else
                        check_packages gcc-c++ glibc-devel
                    fi
                    # Install curl, git, other dependencies if missing
                    if ! type curl > /dev/null 2>&1; then
                        check_packages curl
                    fi
                    if ! type git > /dev/null 2>&1; then
                        check_packages git
                    fi
                    # Some systems, e.g. Mariner, still a few more packages
                    if ! type as > /dev/null 2>&1; then
                        check_packages binutils
                    fi
                    if ! [ -f /usr/include/linux/errno.h ]; then
                        check_packages kernel-headers
                    fi
                    # Minimal RHEL install may need findutils installed
                    if ! [ -f /usr/bin/find ]; then
                        check_packages findutils
                    fi

                    # Get closest match for version number specified
                    find_version_from_git_tags TARGET_GO_VERSION "https://go.googlesource.com/go" "tags/go" "." "true"

                    architecture="$(uname -m)"
                    case $architecture in
                        x86_64) architecture="amd64";;
                        aarch64 | armv8*) architecture="arm64";;
                        aarch32 | armv7* | armvhf*) architecture="armv6l";;
                        i?86) architecture="386";;
                        *) echo "(!) Architecture $architecture unsupported"; exit 1 ;;
                    esac

                    # Install Go
                    umask 0002
                    if ! cat /etc/group | grep -e "^golang:" > /dev/null 2>&1; then
                        groupadd -r golang
                    fi
                    usermod -a -G golang "${USERNAME}"
                    mkdir -p "${TARGET_GOROOT}" "${TARGET_GOPATH}"

                    if [[ "${TARGET_GO_VERSION}" != "none" ]] && [[ "$(go version 2>/dev/null)" != *"${TARGET_GO_VERSION}"* ]]; then
                        # Use a temporary location for gpg keys to avoid polluting image
                        export GNUPGHOME="/tmp/tmp-gnupg"
                        mkdir -p ${GNUPGHOME}
                        chmod 700 ${GNUPGHOME}
                        curl -sSL -o /tmp/tmp-gnupg/golang_key "${GO_GPG_KEY_URI}"
                        gpg -q --import /tmp/tmp-gnupg/golang_key
                        echo "Downloading Go ${TARGET_GO_VERSION}..."
                        set +e
                        curl -fsSL -o /tmp/go.tar.gz "https://golang.org/dl/go${TARGET_GO_VERSION}.linux-${architecture}.tar.gz"
                        exit_code=$?
                        set -e
                        if [ "$exit_code" != "0" ]; then
                            echo "(!) Download failed."
                            # Try one break fix version number less if we get a failure. Use "set +e" since "set -e" can cause failures in valid scenarios.
                            set +e
                            major="$(echo "${TARGET_GO_VERSION}" | grep -oE '^[0-9]+' || echo '')"
                            minor="$(echo "${TARGET_GO_VERSION}" | grep -oP '^[0-9]+\.\K[0-9]+' || echo '')"
                            breakfix="$(echo "${TARGET_GO_VERSION}" | grep -oP '^[0-9]+\.[0-9]+\.\K[0-9]+' 2>/dev/null || echo '')"
                            # Handle Go's odd version pattern where "0" releases omit the last part
                            if [ "${breakfix}" = "" ] || [ "${breakfix}" = "0" ]; then
                                ((minor=minor-1))
                                TARGET_GO_VERSION="${major}.${minor}"
                                # Look for latest version from previous minor release
                                find_version_from_git_tags TARGET_GO_VERSION "https://go.googlesource.com/go" "tags/go" "." "true"
                            else
                                ((breakfix=breakfix-1))
                                if [ "${breakfix}" = "0" ]; then
                                    TARGET_GO_VERSION="${major}.${minor}"
                                else
                                    TARGET_GO_VERSION="${major}.${minor}.${breakfix}"
                                fi
                            fi
                            set -e
                            echo "Trying ${TARGET_GO_VERSION}..."
                            curl -fsSL -o /tmp/go.tar.gz "https://golang.org/dl/go${TARGET_GO_VERSION}.linux-${architecture}.tar.gz"
                        fi
                        curl -fsSL -o /tmp/go.tar.gz.asc "https://golang.org/dl/go${TARGET_GO_VERSION}.linux-${architecture}.tar.gz.asc"
                        gpg --verify /tmp/go.tar.gz.asc /tmp/go.tar.gz
                        echo "Extracting Go ${TARGET_GO_VERSION}..."
                        tar -xzf /tmp/go.tar.gz -C "${TARGET_GOROOT}" --strip-components=1
                        rm -rf /tmp/go.tar.gz /tmp/go.tar.gz.asc /tmp/tmp-gnupg
                    else
                        echo "(!) Go is already installed with version ${TARGET_GO_VERSION}. Skipping."
                    fi

                    # Install Go tools that are isImportant && !replacedByGopls based on
                    # https://github.com/golang/vscode-go/blob/v0.38.0/src/goToolsInformation.ts
                    GO_TOOLS="\
                        golang.org/x/tools/gopls@latest \
                        honnef.co/go/tools/cmd/staticcheck@latest \
                        golang.org/x/lint/golint@latest \
                        github.com/mgechev/revive@latest \
                        github.com/go-delve/delve/cmd/dlv@latest \
                        github.com/fatih/gomodifytags@latest \
                        github.com/haya14busa/goplay/cmd/goplay@latest \
                        github.com/cweill/gotests/gotests@latest \
                        github.com/josharian/impl@latest"

                    if [ "${INSTALL_GO_TOOLS}" = "true" ]; then
                        echo "Installing common Go tools..."
                        export PATH=${TARGET_GOROOT}/bin:${PATH}
                        export GOPATH=/tmp/gotools
                        export GOCACHE="${GOPATH}/cache"

                        mkdir -p "${GOPATH}" /usr/local/etc/vscode-dev-containers "${TARGET_GOPATH}/bin"
                        cd "${GOPATH}"

                        # Use go get for versions of go under 1.16
                        go_install_command=install
                        if [[ "1.16" > "$(go version | grep -oP 'go\K[0-9]+\.[0-9]+(\.[0-9]+)?')" ]]; then
                            export GO111MODULE=on
                            go_install_command=get
                            echo "Go version < 1.16, using go get."
                        fi

                        (echo "${GO_TOOLS}" | xargs -n 1 go ${go_install_command} -v )2>&1 | tee -a /usr/local/etc/vscode-dev-containers/go.log

                        # Move Go tools into path
                        if [ -d "${GOPATH}/bin" ]; then
                            mv "${GOPATH}/bin"/* "${TARGET_GOPATH}/bin/"
                        fi

                        # Install golangci-lint from precompiled binaries
                        if [ "$GOLANGCILINT_VERSION" = "latest" ] || [ "$GOLANGCILINT_VERSION" = "" ]; then
                            echo "Installing golangci-lint latest..."
                            curl -fsSL https://raw.githubusercontent.com/golangci/golangci-lint/master/install.sh | \
                                sh -s -- -b "${TARGET_GOPATH}/bin"
                        else
                            echo "Installing golangci-lint ${GOLANGCILINT_VERSION}..."
                            curl -fsSL https://raw.githubusercontent.com/golangci/golangci-lint/master/install.sh | \
                                sh -s -- -b "${TARGET_GOPATH}/bin" "v${GOLANGCILINT_VERSION}"
                        fi

                        # Remove Go tools temp directory
                        rm -rf "${GOPATH}"
                    fi


                    chown -R "${USERNAME}:golang" "${TARGET_GOROOT}" "${TARGET_GOPATH}"
                    chmod -R g+r+w "${TARGET_GOROOT}" "${TARGET_GOPATH}"
                    find "${TARGET_GOROOT}" -type d -print0 | xargs -n 1 -0 chmod g+s
                    find "${TARGET_GOPATH}" -type d -print0 | xargs -n 1 -0 chmod g+s

                    # Clean up
                    clean_up

                    echo "Done!"
                        "#),
                ])
                .await;
                return Ok(http::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(response))
                    .unwrap());
            }
            if parts.uri.path() == "/v2/devcontainers/features/aws-cli/manifests/1" {
                let response = r#"
                    {
                        "schemaVersion": 2,
                        "mediaType": "application/vnd.oci.image.manifest.v1+json",
                        "config": {
                            "mediaType": "application/vnd.devcontainers",
                            "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
                            "size": 2
                        },
                        "layers": [
                            {
                                "mediaType": "application/vnd.devcontainers.layer.v1+tar",
                                "digest": "sha256:4e9b04b394fb63e297b3d5f58185406ea45bddb639c2ba83b5a8394643cd5b13",
                                "size": 19968,
                                "annotations": {
                                    "org.opencontainers.image.title": "devcontainer-feature-aws-cli.tgz"
                                }
                            }
                        ],
                        "annotations": {
                            "dev.containers.metadata": "{\"id\":\"aws-cli\",\"version\":\"1.1.3\",\"name\":\"AWS CLI\",\"documentationURL\":\"https://github.com/devcontainers/features/tree/main/src/aws-cli\",\"description\":\"Installs the AWS CLI along with needed dependencies. Useful for base Dockerfiles that often are missing required install dependencies like gpg.\",\"options\":{\"version\":{\"type\":\"string\",\"proposals\":[\"latest\"],\"default\":\"latest\",\"description\":\"Select or enter an AWS CLI version.\"},\"verbose\":{\"type\":\"boolean\",\"default\":true,\"description\":\"Suppress verbose output.\"}},\"customizations\":{\"vscode\":{\"extensions\":[\"AmazonWebServices.aws-toolkit-vscode\"],\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes the AWS CLI along with needed dependencies pre-installed and available on the `PATH`, along with the AWS Toolkit extensions for AWS development.\"}]}}},\"installsAfter\":[\"ghcr.io/devcontainers/features/common-utils\"]}",
                            "com.github.package.type": "devcontainer_feature"
                        }
                    }"#;
                return Ok(http::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(response))
                    .unwrap());
            }
            if parts.uri.path()
                == "/v2/devcontainers/features/aws-cli/blobs/sha256:4e9b04b394fb63e297b3d5f58185406ea45bddb639c2ba83b5a8394643cd5b13"
            {
                let response = build_tarball(vec![
                    (
                        "./devcontainer-feature.json",
                        r#"
{
    "id": "aws-cli",
    "version": "1.1.3",
    "name": "AWS CLI",
    "documentationURL": "https://github.com/devcontainers/features/tree/main/src/aws-cli",
    "description": "Installs the AWS CLI along with needed dependencies. Useful for base Dockerfiles that often are missing required install dependencies like gpg.",
    "options": {
        "version": {
            "type": "string",
            "proposals": [
                "latest"
            ],
            "default": "latest",
            "description": "Select or enter an AWS CLI version."
        },
        "verbose": {
            "type": "boolean",
            "default": true,
            "description": "Suppress verbose output."
        }
    },
    "customizations": {
        "vscode": {
            "extensions": [
                "AmazonWebServices.aws-toolkit-vscode"
            ],
            "settings": {
                "github.copilot.chat.codeGeneration.instructions": [
                    {
                        "text": "This dev container includes the AWS CLI along with needed dependencies pre-installed and available on the `PATH`, along with the AWS Toolkit extensions for AWS development."
                    }
                ]
            }
        }
    },
    "installsAfter": [
        "ghcr.io/devcontainers/features/common-utils"
    ]
}
                    "#,
                    ),
                    (
                        "./install.sh",
                        r#"#!/usr/bin/env bash
                    #-------------------------------------------------------------------------------------------------------------
                    # Copyright (c) Microsoft Corporation. All rights reserved.
                    # Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
                    #-------------------------------------------------------------------------------------------------------------
                    #
                    # Docs: https://github.com/microsoft/vscode-dev-containers/blob/main/script-library/docs/awscli.md
                    # Maintainer: The VS Code and Codespaces Teams

                    set -e

                    # Clean up
                    rm -rf /var/lib/apt/lists/*

                    VERSION=${VERSION:-"latest"}
                    VERBOSE=${VERBOSE:-"true"}

                    AWSCLI_GPG_KEY=FB5DB77FD5C118B80511ADA8A6310ACC4672475C
                    AWSCLI_GPG_KEY_MATERIAL="-----BEGIN PGP PUBLIC KEY BLOCK-----

                    mQINBF2Cr7UBEADJZHcgusOJl7ENSyumXh85z0TRV0xJorM2B/JL0kHOyigQluUG
                    ZMLhENaG0bYatdrKP+3H91lvK050pXwnO/R7fB/FSTouki4ciIx5OuLlnJZIxSzx
                    PqGl0mkxImLNbGWoi6Lto0LYxqHN2iQtzlwTVmq9733zd3XfcXrZ3+LblHAgEt5G
                    TfNxEKJ8soPLyWmwDH6HWCnjZ/aIQRBTIQ05uVeEoYxSh6wOai7ss/KveoSNBbYz
                    gbdzoqI2Y8cgH2nbfgp3DSasaLZEdCSsIsK1u05CinE7k2qZ7KgKAUIcT/cR/grk
                    C6VwsnDU0OUCideXcQ8WeHutqvgZH1JgKDbznoIzeQHJD238GEu+eKhRHcz8/jeG
                    94zkcgJOz3KbZGYMiTh277Fvj9zzvZsbMBCedV1BTg3TqgvdX4bdkhf5cH+7NtWO
                    lrFj6UwAsGukBTAOxC0l/dnSmZhJ7Z1KmEWilro/gOrjtOxqRQutlIqG22TaqoPG
                    fYVN+en3Zwbt97kcgZDwqbuykNt64oZWc4XKCa3mprEGC3IbJTBFqglXmZ7l9ywG
                    EEUJYOlb2XrSuPWml39beWdKM8kzr1OjnlOm6+lpTRCBfo0wa9F8YZRhHPAkwKkX
                    XDeOGpWRj4ohOx0d2GWkyV5xyN14p2tQOCdOODmz80yUTgRpPVQUtOEhXQARAQAB
                    tCFBV1MgQ0xJIFRlYW0gPGF3cy1jbGlAYW1hem9uLmNvbT6JAlQEEwEIAD4WIQT7
                    Xbd/1cEYuAURraimMQrMRnJHXAUCXYKvtQIbAwUJB4TOAAULCQgHAgYVCgkICwIE
                    FgIDAQIeAQIXgAAKCRCmMQrMRnJHXJIXEAChLUIkg80uPUkGjE3jejvQSA1aWuAM
                    yzy6fdpdlRUz6M6nmsUhOExjVIvibEJpzK5mhuSZ4lb0vJ2ZUPgCv4zs2nBd7BGJ
                    MxKiWgBReGvTdqZ0SzyYH4PYCJSE732x/Fw9hfnh1dMTXNcrQXzwOmmFNNegG0Ox
                    au+VnpcR5Kz3smiTrIwZbRudo1ijhCYPQ7t5CMp9kjC6bObvy1hSIg2xNbMAN/Do
                    ikebAl36uA6Y/Uczjj3GxZW4ZWeFirMidKbtqvUz2y0UFszobjiBSqZZHCreC34B
                    hw9bFNpuWC/0SrXgohdsc6vK50pDGdV5kM2qo9tMQ/izsAwTh/d/GzZv8H4lV9eO
                    tEis+EpR497PaxKKh9tJf0N6Q1YLRHof5xePZtOIlS3gfvsH5hXA3HJ9yIxb8T0H
                    QYmVr3aIUse20i6meI3fuV36VFupwfrTKaL7VXnsrK2fq5cRvyJLNzXucg0WAjPF
                    RrAGLzY7nP1xeg1a0aeP+pdsqjqlPJom8OCWc1+6DWbg0jsC74WoesAqgBItODMB
                    rsal1y/q+bPzpsnWjzHV8+1/EtZmSc8ZUGSJOPkfC7hObnfkl18h+1QtKTjZme4d
                    H17gsBJr+opwJw/Zio2LMjQBOqlm3K1A4zFTh7wBC7He6KPQea1p2XAMgtvATtNe
                    YLZATHZKTJyiqA==
                    =vYOk
                    -----END PGP PUBLIC KEY BLOCK-----"

                    if [ "$(id -u)" -ne 0 ]; then
                        echo -e 'Script must be run as root. Use sudo, su, or add "USER root" to your Dockerfile before running this script.'
                        exit 1
                    fi

                    apt_get_update()
                    {
                        if [ "$(find /var/lib/apt/lists/* | wc -l)" = "0" ]; then
                            echo "Running apt-get update..."
                            apt-get update -y
                        fi
                    }

                    # Checks if packages are installed and installs them if not
                    check_packages() {
                        if ! dpkg -s "$@" > /dev/null 2>&1; then
                            apt_get_update
                            apt-get -y install --no-install-recommends "$@"
                        fi
                    }

                    export DEBIAN_FRONTEND=noninteractive

                    check_packages curl ca-certificates gpg dirmngr unzip bash-completion less

                    verify_aws_cli_gpg_signature() {
                        local filePath=$1
                        local sigFilePath=$2
                        local awsGpgKeyring=aws-cli-public-key.gpg

                        echo "${AWSCLI_GPG_KEY_MATERIAL}" | gpg --dearmor > "./${awsGpgKeyring}"
                        gpg --batch --quiet --no-default-keyring --keyring "./${awsGpgKeyring}" --verify "${sigFilePath}" "${filePath}"
                        local status=$?

                        rm "./${awsGpgKeyring}"

                        return ${status}
                    }

                    install() {
                        local scriptZipFile=awscli.zip
                        local scriptSigFile=awscli.sig

                        # See Linux install docs at https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
                        if [ "${VERSION}" != "latest" ]; then
                            local versionStr=-${VERSION}
                        fi
                        architecture=$(dpkg --print-architecture)
                        case "${architecture}" in
                            amd64) architectureStr=x86_64 ;;
                            arm64) architectureStr=aarch64 ;;
                            *)
                                echo "AWS CLI does not support machine architecture '$architecture'. Please use an x86-64 or ARM64 machine."
                                exit 1
                        esac
                        local scriptUrl=https://awscli.amazonaws.com/awscli-exe-linux-${architectureStr}${versionStr}.zip
                        curl "${scriptUrl}" -o "${scriptZipFile}"
                        curl "${scriptUrl}.sig" -o "${scriptSigFile}"

                        verify_aws_cli_gpg_signature "$scriptZipFile" "$scriptSigFile"
                        if (( $? > 0 )); then
                            echo "Could not verify GPG signature of AWS CLI install script. Make sure you provided a valid version."
                            exit 1
                        fi

                        if [ "${VERBOSE}" = "false" ]; then
                            unzip -q "${scriptZipFile}"
                        else
                            unzip "${scriptZipFile}"
                        fi

                        ./aws/install

                        # kubectl bash completion
                        mkdir -p /etc/bash_completion.d
                        cp ./scripts/vendor/aws_bash_completer /etc/bash_completion.d/aws

                        # kubectl zsh completion
                        if [ -e "${USERHOME}/.oh-my-zsh" ]; then
                            mkdir -p "${USERHOME}/.oh-my-zsh/completions"
                            cp ./scripts/vendor/aws_zsh_completer.sh "${USERHOME}/.oh-my-zsh/completions/_aws"
                            chown -R "${USERNAME}" "${USERHOME}/.oh-my-zsh"
                        fi

                        rm -rf ./aws
                    }

                    echo "(*) Installing AWS CLI..."

                    install

                    # Clean up
                    rm -rf /var/lib/apt/lists/*

                    echo "Done!""#,
                    ),
                    ("./scripts/", r#""#),
                    (
                        "./scripts/fetch-latest-completer-scripts.sh",
                        r#"
                        #!/bin/bash
                        #-------------------------------------------------------------------------------------------------------------
                        # Copyright (c) Microsoft Corporation. All rights reserved.
                        # Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
                        #-------------------------------------------------------------------------------------------------------------
                        #
                        # Docs: https://github.com/devcontainers/features/tree/main/src/aws-cli
                        # Maintainer: The Dev Container spec maintainers
                        #
                        # Run this script to replace aws_bash_completer and aws_zsh_completer.sh with the latest and greatest available version
                        #
                        COMPLETER_SCRIPTS=$(dirname "${BASH_SOURCE[0]}")
                        BASH_COMPLETER_SCRIPT="$COMPLETER_SCRIPTS/vendor/aws_bash_completer"
                        ZSH_COMPLETER_SCRIPT="$COMPLETER_SCRIPTS/vendor/aws_zsh_completer.sh"

                        wget https://raw.githubusercontent.com/aws/aws-cli/v2/bin/aws_bash_completer -O "$BASH_COMPLETER_SCRIPT"
                        chmod +x "$BASH_COMPLETER_SCRIPT"

                        wget https://raw.githubusercontent.com/aws/aws-cli/v2/bin/aws_zsh_completer.sh -O "$ZSH_COMPLETER_SCRIPT"
                        chmod +x "$ZSH_COMPLETER_SCRIPT"
                        "#,
                    ),
                    ("./scripts/vendor/", r#""#),
                    (
                        "./scripts/vendor/aws_bash_completer",
                        r#"
                        # Typically that would be added under one of the following paths:
                        # - /etc/bash_completion.d
                        # - /usr/local/etc/bash_completion.d
                        # - /usr/share/bash-completion/completions

                        complete -C aws_completer aws
                        "#,
                    ),
                    (
                        "./scripts/vendor/aws_zsh_completer.sh",
                        r#"
                        # Source this file to activate auto completion for zsh using the bash
                        # compatibility helper.  Make sure to run `compinit` before, which should be
                        # given usually.
                        #
                        # % source /path/to/zsh_complete.sh
                        #
                        # Typically that would be called somewhere in your .zshrc.
                        #
                        # Note, the overwrite of _bash_complete() is to export COMP_LINE and COMP_POINT
                        # That is only required for zsh <= edab1d3dbe61da7efe5f1ac0e40444b2ec9b9570
                        #
                        # https://github.com/zsh-users/zsh/commit/edab1d3dbe61da7efe5f1ac0e40444b2ec9b9570
                        #
                        # zsh releases prior to that version do not export the required env variables!

                        autoload -Uz bashcompinit
                        bashcompinit -i

                        _bash_complete() {
                          local ret=1
                          local -a suf matches
                          local -x COMP_POINT COMP_CWORD
                          local -a COMP_WORDS COMPREPLY BASH_VERSINFO
                          local -x COMP_LINE="$words"
                          local -A savejobstates savejobtexts

                          (( COMP_POINT = 1 + ${#${(j. .)words[1,CURRENT]}} + $#QIPREFIX + $#IPREFIX + $#PREFIX ))
                          (( COMP_CWORD = CURRENT - 1))
                          COMP_WORDS=( $words )
                          BASH_VERSINFO=( 2 05b 0 1 release )

                          savejobstates=( ${(kv)jobstates} )
                          savejobtexts=( ${(kv)jobtexts} )

                          [[ ${argv[${argv[(I)nospace]:-0}-1]} = -o ]] && suf=( -S '' )

                          matches=( ${(f)"$(compgen $@ -- ${words[CURRENT]})"} )

                          if [[ -n $matches ]]; then
                            if [[ ${argv[${argv[(I)filenames]:-0}-1]} = -o ]]; then
                              compset -P '*/' && matches=( ${matches##*/} )
                              compset -S '/*' && matches=( ${matches%%/*} )
                              compadd -Q -f "${suf[@]}" -a matches && ret=0
                            else
                              compadd -Q "${suf[@]}" -a matches && ret=0
                            fi
                          fi

                          if (( ret )); then
                            if [[ ${argv[${argv[(I)default]:-0}-1]} = -o ]]; then
                              _default "${suf[@]}" && ret=0
                            elif [[ ${argv[${argv[(I)dirnames]:-0}-1]} = -o ]]; then
                              _directories "${suf[@]}" && ret=0
                            fi
                          fi

                          return ret
                        }

                        complete -C aws_completer aws
                        "#,
                    ),
                ]).await;

                return Ok(http::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(response))
                    .unwrap());
            }

            Ok(http::Response::builder()
                .status(404)
                .body(http_client::AsyncBody::default())
                .unwrap())
        })
    }
}

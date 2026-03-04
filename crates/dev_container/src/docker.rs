use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use smol::process::Command;

use crate::{DevContainerErrorV2, command_json::evaluate_json_command};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerPs {
    #[serde(rename = "ID")]
    pub(crate) id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerInspect {
    pub(crate) id: String,
    pub(crate) config: DockerInspectConfig,
    pub(crate) mounts: Option<Vec<DockerInspectMount>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub(crate) struct DockerConfigLabels {
    #[serde(
        rename = "devcontainer.metadata",
        deserialize_with = "deserialize_metadata"
    )]
    pub(crate) metadata: Option<Vec<HashMap<String, serde_json_lenient::Value>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerInspectConfig {
    pub(crate) labels: DockerConfigLabels,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerInspectMount {
    pub(crate) source: String,
    pub(crate) destination: String,
}

pub(crate) async fn inspect_image(image: &String) -> Result<DockerInspect, DevContainerErrorV2> {
    let command = create_docker_inspect(image);

    let Some(docker_inspect): Option<DockerInspect> = evaluate_json_command(command).await? else {
        log::error!("Error TODO");
        return Err(DevContainerErrorV2::UnmappedError);
    };
    Ok(docker_inspect)
}

fn deserialize_metadata<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<HashMap<String, serde_json_lenient::Value>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(json_string) => {
            let parsed: Vec<HashMap<String, serde_json_lenient::Value>> =
                serde_json_lenient::from_str(&json_string).map_err(|e| {
                    log::error!("Error deserializing metadata: {e}");
                    serde::de::Error::custom(e)
                })?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

// I can avoid making this public, right?
fn create_docker_inspect(id: &str) -> Command {
    let mut command = smol::process::Command::new(docker_cli());
    command.args(&["inspect", "--format={{json . }}", id]);
    command
}

// TODO podman
pub(crate) fn docker_cli() -> &'static str {
    "docker"
}

#[cfg(test)]
mod test {
    use std::ffi::OsStr;

    use crate::docker::create_docker_inspect;

    #[test]
    fn should_create_docker_inspect_command() {
        let given_id = "given_docker_id";

        let command = create_docker_inspect(given_id);

        assert_eq!(
            command.get_args().collect::<Vec<&OsStr>>(),
            vec![
                OsStr::new("inspect"),
                OsStr::new("--format={{json . }}"),
                OsStr::new(given_id)
            ]
        )
    }
}

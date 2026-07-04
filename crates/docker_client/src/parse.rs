use serde::Deserialize;

use crate::{ComposeProject, ComposeService, Container, ContainerState, Image};

pub fn parse_container_state(raw: &str) -> ContainerState {
    match raw {
        "running" => ContainerState::Running,
        "exited" => ContainerState::Exited,
        "paused" => ContainerState::Paused,
        "created" => ContainerState::Created,
        "restarting" => ContainerState::Restarting,
        "dead" => ContainerState::Dead,
        _ => ContainerState::Unknown,
    }
}

#[derive(Debug, Deserialize)]
struct ContainerRow {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Names")]
    names: String,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Ports")]
    ports: String,
}

#[derive(Debug, Deserialize)]
struct ImageRow {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Repository")]
    repository: String,
    #[serde(rename = "Tag")]
    tag: String,
    #[serde(rename = "Size")]
    size: String,
    #[serde(rename = "CreatedSince")]
    created_since: String,
}

#[derive(Debug, Deserialize)]
struct ComposeProjectRow {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "ConfigFiles")]
    config_files: String,
}

#[derive(Debug, Deserialize)]
struct ComposeServiceRow {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Project")]
    project: String,
}

fn parse_json_lines<T, F, U>(stdout: &str, map: F) -> anyhow::Result<Vec<U>>
where
    T: for<'de> Deserialize<'de>,
    F: Fn(T) -> U,
{
    let mut items = Vec::new();
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<T>(line) {
            Ok(row) => items.push(map(row)),
            Err(error) => {
                log::warn!("docker_client: failed to parse json line: {error}");
            }
        }
    }
    Ok(items)
}

pub fn parse_containers(stdout: &str) -> anyhow::Result<Vec<Container>> {
    parse_json_lines(stdout, |row: ContainerRow| Container {
        id: row.id,
        names: row.names,
        image: row.image,
        state: parse_container_state(&row.state),
        status: row.status,
        ports: row.ports,
    })
}

pub fn parse_images(stdout: &str) -> anyhow::Result<Vec<Image>> {
    parse_json_lines(stdout, |row: ImageRow| Image {
        id: row.id,
        repository: row.repository,
        tag: row.tag,
        size: row.size,
        created: row.created_since,
    })
}

pub fn parse_compose_projects(stdout: &str) -> anyhow::Result<Vec<ComposeProject>> {
    parse_json_lines(stdout, |row: ComposeProjectRow| ComposeProject {
        name: row.name,
        status: row.status,
        config_files: row.config_files,
    })
}

pub fn parse_compose_services(stdout: &str) -> anyhow::Result<Vec<ComposeService>> {
    parse_json_lines(stdout, |row: ComposeServiceRow| ComposeService {
        name: row.name,
        state: row.state,
        project: row.project,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_containers_jsonlines() {
        let out = concat!(
            r#"{"ID":"abc123","Names":"api","Image":"myapi:latest","State":"running","Status":"Up 3 hours","Ports":"0.0.0.0:8080->8080/tcp"}"#,
            "\n",
            r#"{"ID":"def456","Names":"db","Image":"postgres:16","State":"exited","Status":"Exited (0) 1 hour ago","Ports":""}"#,
            "\n",
        );
        let containers = parse_containers(out).unwrap();
        assert_eq!(containers.len(), 2);
        assert_eq!(containers[0].names, "api");
        assert_eq!(containers[0].state, ContainerState::Running);
        assert_eq!(containers[1].state, ContainerState::Exited);
    }

    #[test]
    fn parse_containers_empty_is_empty_vec() {
        assert!(parse_containers("").unwrap().is_empty());
        assert!(parse_containers("   \n\n").unwrap().is_empty());
    }

    #[test]
    fn parse_images_and_compose() {
        let imgs = parse_images(r#"{"ID":"sha256:aaa","Repository":"myapi","Tag":"latest","Size":"120MB","CreatedSince":"2 days ago"}"#).unwrap();
        assert_eq!(imgs[0].repository, "myapi");
        assert_eq!(imgs[0].tag, "latest");
        let projs = parse_compose_projects(
            r#"{"Name":"shop","Status":"running(3)","ConfigFiles":"/app/docker-compose.yml"}"#,
        )
        .unwrap();
        assert_eq!(projs[0].name, "shop");
        let svcs =
            parse_compose_services(r#"{"Name":"web","State":"running","Project":"shop"}"#).unwrap();
        assert_eq!(svcs[0].state, "running");
    }
}

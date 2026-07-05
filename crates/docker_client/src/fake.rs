use std::sync::Mutex;

use anyhow::{Result, anyhow};

use crate::*;

/// In-memory [`DockerClient`] returning canned data and recording each call.
///
/// Available in tests and behind the `test-support` feature so that UI crates
/// can drive the panel without a live Docker daemon.
pub struct FakeDockerClient {
    pub containers: Vec<Container>,
    pub images: Vec<Image>,
    pub compose_projects: Vec<ComposeProject>,
    pub compose_services: Vec<ComposeService>,
    pub contexts: Vec<DockerContext>,
    pub inspect: String,
    pub log_lines: Vec<String>,
    pub error: Option<String>,
    test_endpoint_error: Mutex<Option<String>>,
    list_containers_error: Mutex<Option<String>>,
    list_images_error: Mutex<Option<String>>,
    list_compose_projects_error: Mutex<Option<String>>,
    list_compose_services_error: Mutex<Option<String>>,
    list_contexts_error: Mutex<Option<String>>,
    inspect_container_error: Mutex<Option<String>>,
    start_container_error: Mutex<Option<String>>,
    stop_container_error: Mutex<Option<String>>,
    restart_container_error: Mutex<Option<String>>,
    pull_image_error: Mutex<Option<String>>,
    remove_image_error: Mutex<Option<String>>,
    compose_up_error: Mutex<Option<String>>,
    compose_down_error: Mutex<Option<String>>,
    compose_restart_error: Mutex<Option<String>>,
    container_logs_error: Mutex<Option<String>>,
    calls: Mutex<Vec<String>>,
}

impl Default for FakeDockerClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeDockerClient {
    pub fn new() -> Self {
        Self {
            containers: Vec::new(),
            images: Vec::new(),
            compose_projects: Vec::new(),
            compose_services: Vec::new(),
            contexts: Vec::new(),
            inspect: String::new(),
            log_lines: Vec::new(),
            error: None,
            test_endpoint_error: Mutex::new(None),
            list_containers_error: Mutex::new(None),
            list_images_error: Mutex::new(None),
            list_compose_projects_error: Mutex::new(None),
            list_compose_services_error: Mutex::new(None),
            list_contexts_error: Mutex::new(None),
            inspect_container_error: Mutex::new(None),
            start_container_error: Mutex::new(None),
            stop_container_error: Mutex::new(None),
            restart_container_error: Mutex::new(None),
            pull_image_error: Mutex::new(None),
            remove_image_error: Mutex::new(None),
            compose_up_error: Mutex::new(None),
            compose_down_error: Mutex::new(None),
            compose_restart_error: Mutex::new(None),
            container_logs_error: Mutex::new(None),
            calls: Mutex::new(Vec::new()),
        }
    }

    /// Constructs a client whose every method fails with `message`.
    pub fn with_error(message: &str) -> Self {
        Self {
            error: Some(message.to_string()),
            ..Self::new()
        }
    }

    /// Constructs a client whose `list_containers` returns a single running
    /// container named `name`.
    pub fn new_with_container(name: &str) -> Self {
        Self {
            containers: vec![Container {
                id: format!("{name}-id"),
                names: name.to_string(),
                image: format!("{name}-image"),
                state: ContainerState::Running,
                status: "Up".into(),
                ports: String::new(),
            }],
            ..Self::new()
        }
    }

    /// Sets or clears an error that fails only `test_endpoint`, unlike `error`
    /// which fails every method.
    pub fn set_test_endpoint_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.test_endpoint_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `list_containers`, unlike
    /// `error` which fails every method.
    pub fn set_list_containers_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.list_containers_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `list_images`, unlike `error`
    /// which fails every method.
    pub fn set_list_images_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.list_images_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `list_compose_projects`,
    /// unlike `error` which fails every method.
    pub fn set_list_compose_projects_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.list_compose_projects_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `list_compose_services`,
    /// unlike `error` which fails every method.
    pub fn set_list_compose_services_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.list_compose_services_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `list_contexts`, unlike
    /// `error` which fails every method.
    pub fn set_list_contexts_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.list_contexts_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `inspect_container`, unlike
    /// `error` which fails every method.
    pub fn set_inspect_container_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.inspect_container_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `start_container`, unlike
    /// `error` which fails every method.
    pub fn set_start_container_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.start_container_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `stop_container`, unlike
    /// `error` which fails every method.
    pub fn set_stop_container_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.stop_container_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `restart_container`, unlike
    /// `error` which fails every method.
    pub fn set_restart_container_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.restart_container_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `pull_image`, unlike `error`
    /// which fails every method.
    pub fn set_pull_image_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.pull_image_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `remove_image`, unlike `error`
    /// which fails every method.
    pub fn set_remove_image_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.remove_image_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `compose_up`, unlike `error`
    /// which fails every method.
    pub fn set_compose_up_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.compose_up_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `compose_down`, unlike `error`
    /// which fails every method.
    pub fn set_compose_down_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.compose_down_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `compose_restart`, unlike
    /// `error` which fails every method.
    pub fn set_compose_restart_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.compose_restart_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `container_logs`, unlike
    /// `error` which fails every method.
    pub fn set_container_logs_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.container_logs_error.lock() {
            *slot = error;
        }
    }

    /// Returns the recorded calls in order.
    pub fn calls(&self) -> Vec<String> {
        self.calls
            .lock()
            .map(|calls| calls.clone())
            .unwrap_or_default()
    }

    fn record(&self, call: impl Into<String>) {
        if let Ok(mut calls) = self.calls.lock() {
            calls.push(call.into());
        }
    }

    fn check_error(&self) -> Result<()> {
        if let Some(message) = &self.error {
            return Err(anyhow!("{message}"));
        }
        Ok(())
    }

    fn check_override(&self, slot: &Mutex<Option<String>>) -> Result<()> {
        if let Ok(slot) = slot.lock()
            && let Some(message) = slot.as_ref()
        {
            return Err(anyhow!("{message}"));
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl DockerClient for FakeDockerClient {
    async fn list_contexts(&self) -> Result<Vec<DockerContext>> {
        self.check_error()?;
        self.record("list_contexts");
        self.check_override(&self.list_contexts_error)?;
        Ok(self.contexts.clone())
    }

    async fn test_endpoint(&self, endpoint: &DockerEndpoint) -> Result<()> {
        self.check_error()?;
        self.record(format!("test_endpoint {}", endpoint.name));
        self.check_override(&self.test_endpoint_error)?;
        Ok(())
    }

    async fn list_containers(&self, endpoint: &DockerEndpoint) -> Result<Vec<Container>> {
        self.check_error()?;
        self.record(format!("list_containers {}", endpoint.name));
        self.check_override(&self.list_containers_error)?;
        Ok(self.containers.clone())
    }

    async fn list_images(&self, endpoint: &DockerEndpoint) -> Result<Vec<Image>> {
        self.check_error()?;
        self.record(format!("list_images {}", endpoint.name));
        self.check_override(&self.list_images_error)?;
        Ok(self.images.clone())
    }

    async fn list_compose_projects(
        &self,
        endpoint: &DockerEndpoint,
    ) -> Result<Vec<ComposeProject>> {
        self.check_error()?;
        self.record(format!("list_compose_projects {}", endpoint.name));
        self.check_override(&self.list_compose_projects_error)?;
        Ok(self.compose_projects.clone())
    }

    async fn list_compose_services(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
    ) -> Result<Vec<ComposeService>> {
        self.check_error()?;
        self.record(format!("list_compose_services {} {project}", endpoint.name));
        self.check_override(&self.list_compose_services_error)?;
        Ok(self.compose_services.clone())
    }

    async fn inspect_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<String> {
        self.check_error()?;
        self.record(format!("inspect_container {} {id}", endpoint.name));
        self.check_override(&self.inspect_container_error)?;
        Ok(self.inspect.clone())
    }

    async fn start_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        self.check_error()?;
        self.record(format!("start_container {} {id}", endpoint.name));
        self.check_override(&self.start_container_error)?;
        Ok(())
    }

    async fn stop_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        self.check_error()?;
        self.record(format!("stop_container {} {id}", endpoint.name));
        self.check_override(&self.stop_container_error)?;
        Ok(())
    }

    async fn restart_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        self.check_error()?;
        self.record(format!("restart_container {} {id}", endpoint.name));
        self.check_override(&self.restart_container_error)?;
        Ok(())
    }

    async fn pull_image(&self, endpoint: &DockerEndpoint, reference: &str) -> Result<()> {
        self.check_error()?;
        self.record(format!("pull_image {} {reference}", endpoint.name));
        self.check_override(&self.pull_image_error)?;
        Ok(())
    }

    async fn remove_image(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        self.check_error()?;
        self.record(format!("remove_image {} {id}", endpoint.name));
        self.check_override(&self.remove_image_error)?;
        Ok(())
    }

    async fn compose_up(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
        service: Option<&str>,
    ) -> Result<()> {
        self.check_error()?;
        self.record(format!(
            "compose_up {} {project} service={service:?}",
            endpoint.name
        ));
        self.check_override(&self.compose_up_error)?;
        Ok(())
    }

    async fn compose_down(&self, endpoint: &DockerEndpoint, project: &str) -> Result<()> {
        self.check_error()?;
        self.record(format!("compose_down {} {project}", endpoint.name));
        self.check_override(&self.compose_down_error)?;
        Ok(())
    }

    async fn compose_restart(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
        service: Option<&str>,
    ) -> Result<()> {
        self.check_error()?;
        self.record(format!(
            "compose_restart {} {project} service={service:?}",
            endpoint.name
        ));
        self.check_override(&self.compose_restart_error)?;
        Ok(())
    }

    async fn container_logs(
        &self,
        endpoint: &DockerEndpoint,
        id: &str,
        tail: usize,
    ) -> Result<futures::channel::mpsc::UnboundedReceiver<LogChunk>> {
        self.check_error()?;
        self.record(format!("container_logs {} {id} tail={tail}", endpoint.name));
        self.check_override(&self.container_logs_error)?;
        let (tx, rx) = futures::channel::mpsc::unbounded();
        for line in &self.log_lines {
            tx.unbounded_send(LogChunk { line: line.clone() })
                .map_err(|error| anyhow!("failed to send canned log line: {error}"))?;
        }
        drop(tx);
        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt as _;

    use super::*;

    #[tokio::test]
    async fn fake_lists_and_records_calls() {
        let mut fake = FakeDockerClient::new();
        fake.containers = vec![Container {
            id: "a".into(),
            names: "api".into(),
            image: "img".into(),
            state: ContainerState::Running,
            status: "Up".into(),
            ports: "".into(),
        }];
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        let got = fake.list_containers(&ep).await.unwrap();
        assert_eq!(got.len(), 1);
        assert!(
            fake.calls()
                .iter()
                .any(|c| c.starts_with("list_containers local"))
        );
    }

    #[tokio::test]
    async fn fake_error_override_propagates() {
        let fake = FakeDockerClient::new();
        fake.set_stop_container_error(Some("boom".into()));
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        assert!(fake.stop_container(&ep, "a").await.is_err());
    }

    #[tokio::test]
    async fn fake_logs_stream_yields_canned_lines() {
        let mut fake = FakeDockerClient::new();
        fake.log_lines = vec!["line1".into(), "line2".into()];
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        let mut rx = fake.container_logs(&ep, "a", 100).await.unwrap();
        let mut lines = vec![];
        while let Some(chunk) = rx.next().await {
            lines.push(chunk.line);
        }
        assert_eq!(lines, vec!["line1", "line2"]);
    }

    #[tokio::test]
    async fn fake_client_error_mode() {
        let fake = FakeDockerClient::with_error("boom");
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        let error = fake.list_containers(&ep).await.unwrap_err();
        assert!(error.to_string().contains("boom"));
    }

    #[tokio::test]
    async fn fake_lists_contexts_and_records_call() {
        let mut fake = FakeDockerClient::new();
        fake.contexts = vec![DockerContext {
            name: "staging".into(),
            docker_endpoint: "ssh://deploy@stg".into(),
        }];
        let got = fake.list_contexts().await.unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].name, "staging");
        assert!(fake.calls().iter().any(|c| c == "list_contexts"));
    }

    #[tokio::test]
    async fn fake_error_override_isolated_to_one_method() {
        let fake = FakeDockerClient::new();
        fake.set_compose_up_error(Some("boom".into()));
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        assert!(fake.compose_up(&ep, "shop", None).await.is_err());
        assert!(fake.list_containers(&ep).await.is_ok());
    }
}

//! Container configuration types for GitHub workflow jobs.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::env::Env;

/// Represents a container configuration for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Container {
    /// The image to use for the container.
    #[setters(skip)]
    pub image: String,

    /// Credentials for accessing the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,

    /// Environment variables for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    /// Ports to expose from the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,

    /// Volumes to mount in the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,

    /// Additional options for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,

    /// Hostname for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

/// Represents credentials for accessing a container.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Credentials {
    /// The username for authentication.
    pub username: String,

    /// The password for authentication.
    pub password: String,
}

/// Represents a network port.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Port {
    /// A port specified by its number.
    Number(u16),

    /// A port specified by its name/mapping (e.g., "8080:80").
    Name(String),
}

impl Container {
    /// Creates a new `Container` with the specified image.
    pub fn new<S: ToString>(image: S) -> Self {
        Self {
            image: image.to_string(),
            ..Default::default()
        }
    }

    /// Adds an environment variable to the container.
    pub fn add_env<E: Into<Env>>(mut self, new_env: E) -> Self {
        let mut env = self.env.take().unwrap_or_default();
        env.0.extend(new_env.into().0);
        self.env = Some(env);
        self
    }
}

/// Represents a volume configuration for containers.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Volume {
    /// The source path of the volume.
    pub source: String,

    /// The destination path of the volume.
    pub destination: String,
}

impl Volume {
    /// Creates a new `Volume` from a string representation.
    pub fn new(volume_str: &str) -> Option<Self> {
        let parts: Vec<&str> = volume_str.split(':').collect();
        if parts.len() == 2 {
            Some(Volume {
                source: parts[0].to_string(),
                destination: parts[1].to_string(),
            })
        } else {
            None
        }
    }
}

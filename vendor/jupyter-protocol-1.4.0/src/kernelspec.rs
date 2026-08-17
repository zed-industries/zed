//! Defines structures for working with Jupyter kernel specifications.
//!
//! This module provides types for representing and working with Jupyter kernel
//! specifications, which describe the properties and launch parameters for
//! Jupyter kernels.
//!
//! The main struct in this module is `JupyterKernelspec`, which corresponds to
//! the contents of a Jupyter JSON kernelspec file.
//!
//! # Examples
//!
//! ```rust
//! use jupyter_protocol::JupyterKernelspec;
//! use std::collections::HashMap;
//!
//! let kernelspec = JupyterKernelspec {
//!     argv: vec!["python3", "-m", "ipykernel_launcher", "-f", "{connection_file}"].into_iter().map(String::from).collect(),
//!     display_name: "Python 3".to_string(),
//!     language: "python".to_string(),
//!     metadata: None,
//!     interrupt_mode: Some("signal".to_string()),
//!     env: Some(HashMap::new()),
//! };
//! ```
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the contents of a Jupyter JSON kernelspec file.
///
/// A kernelspec file defines the properties and launch parameters for a Jupyter kernel.
/// This struct is used to serialize and deserialize kernelspec data.
///
/// # Examples
///
/// ```rust
/// use jupyter_protocol::JupyterKernelspec;
/// use std::collections::HashMap;
///
/// let kernelspec = JupyterKernelspec {
///     argv: vec![
///         "python3".to_string(),
///         "-m".to_string(),
///         "ipykernel_launcher".to_string(),
///         "-f".to_string(),
///         "{connection_file}".to_string()
///     ],
///     display_name: "Python 3".to_string(),
///     language: "python".to_string(),
///     metadata: None,
///     interrupt_mode: Some("signal".to_string()),
///     env: Some(HashMap::new()),
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JupyterKernelspec {
    /// The command line arguments used to launch the kernel.
    ///
    /// This vector must contain `{connection_file}` as a placeholder, which will be
    /// replaced by the actual connection file path when the client launches the kernel.
    #[serde(default)]
    pub argv: Vec<String>,
    /// The human-readable name for the kernel.
    ///
    /// This name is typically displayed in the Jupyter interface when selecting a kernel.
    pub display_name: String,
    /// The programming language supported by the kernel.
    ///
    /// This should be a string identifier for the language, such as "python", "r", or "julia".
    pub language: String,
    /// Additional metadata associated with the kernel.
    ///
    /// This field can contain arbitrary key-value pairs for kernel-specific information.
    /// The values can be of any JSON-compatible type.
    pub metadata: Option<HashMap<String, Value>>,
    /// Specifies how the kernel should be interrupted.
    ///
    /// Common values are "signal" (use SIGINT) or "message" (use kernel protocol).
    /// If not specified, the client will use a default interrupt method.
    pub interrupt_mode: Option<String>,
    /// Environment variables to set for the kernel process.
    ///
    /// These key-value pairs will be added to the environment when launching the kernel.
    pub env: Option<HashMap<String, String>>,
}

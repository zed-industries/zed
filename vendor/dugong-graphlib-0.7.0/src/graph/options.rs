//! Graph configuration options.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GraphOptions {
    pub multigraph: bool,
    pub compound: bool,
    pub directed: bool,
}

impl Default for GraphOptions {
    fn default() -> Self {
        Self {
            multigraph: false,
            compound: false,
            directed: true,
        }
    }
}

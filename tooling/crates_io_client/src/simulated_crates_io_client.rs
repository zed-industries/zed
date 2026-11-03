use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::Mutex;
use url::Url;

use crate::*;

pub struct SimulatedCratesIoClient {
    state: Mutex<CratesIoState>,
}

struct CratesIoState {
    site_metadata: SiteMetadata,
}

impl SimulatedCratesIoClient {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(CratesIoState {
                site_metadata: SiteMetadata {
                    deployed_sha: "unknown".to_owned(),
                    commit: "unknown".to_owned(),
                    cdn_base: Url::parse("https://static.example.com").expect("test URL is valid"),
                    read_only: false,
                    banner_message: None,
                },
            }),
        })
    }

    pub fn test_site_metadata(&self, site_metadata: SiteMetadata) {
        self.state.lock().site_metadata = site_metadata;
    }
}

#[async_trait]
impl CratesIoClient for SimulatedCratesIoClient {
    async fn get_site_metadata(&self) -> Result<SiteMetadata> {
        Ok(self.state.lock().site_metadata.clone())
    }
}

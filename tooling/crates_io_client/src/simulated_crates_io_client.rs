use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use http_client::StatusCode;
use parking_lot::Mutex;
use url::Url;

use crate::*;

pub struct SimulatedCratesIoClient {
    state: Mutex<CratesIoState>,
}

struct CratesIoState {
    site_metadata: SiteMetadata,
    token_user: UserId,
    crates: BTreeMap<CrateName, SimulatedCrate>,
    latest_id: i32,
}

struct SimulatedCrate {
    krate: Crate,
    owners: Vec<UserId>,
    versions: Vec<Version>,
}

impl CratesIoState {
    fn next_id(&mut self) -> i32 {
        self.latest_id += 1;
        self.latest_id
    }

    fn simulated_crate(&mut self, name: &CrateName) -> Result<&mut SimulatedCrate> {
        self.crates
            .get_mut(name)
            .ok_or_else(|| CratesIoError::NotFound {
                message: format!("crate `{name}` does not exist"),
            })
    }
}

/// Keyword and category IDs are only part of single-crate responses.
fn listed(krate: &Crate) -> Crate {
    Crate {
        keywords: None,
        categories: None,
        ..krate.clone()
    }
}

fn test_timestamp() -> DateTime<Utc> {
    DateTime::from_timestamp(1_767_225_600, 0).unwrap_or_default()
}

impl SimulatedCratesIoClient {
    /// Creates a registry whose API token belongs to [`Self::token_user`].
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
                token_user: UserId(1),
                crates: BTreeMap::new(),
                latest_id: 1,
            }),
        })
    }

    pub fn token_user(&self) -> UserId {
        self.state.lock().token_user
    }

    pub fn test_user(&self) -> UserId {
        UserId(self.state.lock().next_id())
    }

    pub fn test_site_metadata(&self, site_metadata: SiteMetadata) {
        self.state.lock().site_metadata = site_metadata;
    }

    /// Adds a crate without versions, replacing any crate with the same name.
    pub fn test_crate(&self, name: &str, owner: UserId) -> CrateName {
        let name = CrateName::new(name);
        self.state.lock().crates.insert(
            name.clone(),
            SimulatedCrate {
                krate: Crate {
                    name: name.clone(),
                    description: None,
                    homepage: None,
                    documentation: None,
                    repository: None,
                    created_at: test_timestamp(),
                    updated_at: test_timestamp(),
                    downloads: 0,
                    recent_downloads: Some(0),
                    default_version: None,
                    num_versions: 0,
                    yanked: false,
                    trustpub_only: false,
                    keywords: Some(Vec::new()),
                    categories: Some(Vec::new()),
                },
                owners: vec![owner],
                versions: Vec::new(),
            },
        );
        name
    }

    /// Changes crate fields that have no dedicated helper, such as its
    /// keywords or download counts.
    pub fn update_test_crate(
        &self,
        name: &CrateName,
        update: impl FnOnce(&mut Crate),
    ) -> Result<()> {
        let mut state = self.state.lock();
        update(&mut state.simulated_crate(name)?.krate);
        Ok(())
    }

    /// Publishes a version, which becomes the crate's default version.
    pub fn test_version(&self, name: &CrateName, num: semver::Version) -> Result<VersionId> {
        let mut state = self.state.lock();
        let id = VersionId(state.next_id());
        let publisher = state.token_user;
        let simulated_crate = state.simulated_crate(name)?;
        simulated_crate.versions.insert(
            0,
            Version {
                id,
                crate_name: name.clone(),
                num: num.clone(),
                created_at: test_timestamp(),
                updated_at: test_timestamp(),
                downloads: 0,
                yanked: false,
                yank_message: None,
                license: None,
                rust_version: None,
                edition: None,
                crate_size: 0,
                checksum: String::new(),
                published_by: Some(User {
                    id: publisher,
                    login: format!("user-{}", publisher.0),
                    name: None,
                }),
            },
        );
        simulated_crate.krate.default_version = Some(num);
        simulated_crate.krate.num_versions += 1;
        Ok(id)
    }
}

#[async_trait]
impl CratesIoClient for SimulatedCratesIoClient {
    async fn get_site_metadata(&self) -> Result<SiteMetadata> {
        Ok(self.state.lock().site_metadata.clone())
    }

    async fn list_crates(&self, query: &CrateListQuery) -> Result<Vec<Crate>> {
        let state = self.state.lock();
        let has_value = |values: &Option<Vec<String>>, value: &String| {
            values.as_ref().is_some_and(|values| values.contains(value))
        };
        let mut crates = state
            .crates
            .values()
            .filter(|simulated_crate| {
                let krate = &simulated_crate.krate;
                let matches_owner = match (&query.keyword, query.user_id) {
                    (Some(keyword), _) => has_value(&krate.keywords, keyword),
                    (None, Some(user_id)) => simulated_crate.owners.contains(&user_id),
                    (None, None) => true,
                };
                matches_owner
                    && query.search.as_ref().is_none_or(|search| {
                        krate
                            .name
                            .as_str()
                            .to_lowercase()
                            .contains(&search.to_lowercase())
                    })
                    && query
                        .category
                        .as_ref()
                        .is_none_or(|category| has_value(&krate.categories, category))
                    && !(query.exclude_yanked && krate.yanked)
            })
            .map(|simulated_crate| listed(&simulated_crate.krate))
            .collect::<Vec<_>>();

        // Crates are stored by name, so they are already alphabetical. An exact
        // match stands in for crates.io's relevance order when searching.
        if let Some(search) = &query.search {
            crates.sort_by_key(|krate| krate.name.as_str() != search);
        }
        Ok(crates)
    }

    async fn get_crate(&self, name: &CrateName) -> Result<CrateDetails> {
        let mut state = self.state.lock();
        let simulated_crate = state.simulated_crate(name)?;
        Ok(CrateDetails {
            krate: simulated_crate.krate.clone(),
            versions: simulated_crate.versions.clone(),
        })
    }

    async fn update_crate(&self, name: &CrateName, update: &CrateUpdate) -> Result<Crate> {
        let mut state = self.state.lock();
        let token_user = state.token_user;
        let simulated_crate = state.simulated_crate(name)?;
        if !simulated_crate.owners.contains(&token_user) {
            return Err(CratesIoError::PermissionDenied {
                status: StatusCode::FORBIDDEN,
                message: "only owners have permission to modify crate settings".to_owned(),
            });
        }
        if let Some(trustpub_only) = update.trustpub_only {
            simulated_crate.krate.trustpub_only = trustpub_only;
        }
        Ok(listed(&simulated_crate.krate))
    }
}

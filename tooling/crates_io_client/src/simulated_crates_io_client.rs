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
    github_configs: BTreeMap<GitHubConfigId, GitHubConfig>,
    latest_id: i32,
}

const MAX_GITHUB_CONFIGS_PER_CRATE: usize = 5;

struct SimulatedCrate {
    krate: Crate,
    owners: Vec<UserId>,
    team_owners: Vec<Team>,
    invited_owners: Vec<OwnerLogin>,
    versions: Vec<Version>,
}

impl SimulatedCrate {
    fn require_owner_rights(&self, user: UserId) -> Result<()> {
        if self.owners.contains(&user) {
            Ok(())
        } else {
            Err(CratesIoError::PermissionDenied {
                status: StatusCode::FORBIDDEN,
                message: "only owners have permission to modify owners".to_owned(),
            })
        }
    }

    fn matches(login: &OwnerLogin, owner: &Owner) -> bool {
        match (login, owner) {
            (
                OwnerLogin::Username(username)
                | OwnerLogin::CratesIoUser(username)
                | OwnerLogin::GitHubUser(username),
                Owner::User(user),
            ) => user.login.eq_ignore_ascii_case(username),
            (OwnerLogin::GitHubTeam { .. }, Owner::Team(team)) => {
                team.login.eq_ignore_ascii_case(&login.to_string())
            }
            _ => false,
        }
    }

    fn all_owners(&self) -> impl Iterator<Item = Owner> + '_ {
        let users = self
            .owners
            .iter()
            .map(|id| Owner::User(simulated_user(*id)));
        let teams = self.team_owners.iter().cloned().map(Owner::Team);
        users.chain(teams)
    }
}

fn bad_request(message: String) -> CratesIoError {
    CratesIoError::Upstream {
        status: StatusCode::BAD_REQUEST,
        message,
    }
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

    /// crates.io answers non-owners of a crate with a bad request rather than
    /// a permission error on the Trusted Publishing endpoints.
    fn require_token_user_owns(&mut self, name: &CrateName) -> Result<()> {
        let token_user = self.token_user;
        if self.simulated_crate(name)?.owners.contains(&token_user) {
            Ok(())
        } else {
            Err(CratesIoError::Upstream {
                status: StatusCode::BAD_REQUEST,
                message: "You are not an owner of this crate".to_owned(),
            })
        }
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

fn simulated_user(id: UserId) -> User {
    User {
        id,
        login: format!("user-{}", id.0),
        name: None,
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
                github_configs: BTreeMap::new(),
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
                team_owners: Vec::new(),
                invited_owners: Vec::new(),
                versions: Vec::new(),
            },
        );
        name
    }

    pub fn test_owner(&self, name: &CrateName, owner: UserId) -> Result<()> {
        self.state.lock().simulated_crate(name)?.owners.push(owner);
        Ok(())
    }

    pub fn test_team_owner(&self, name: &CrateName, login: &str) -> Result<TeamId> {
        let mut state = self.state.lock();
        let id = TeamId(state.next_id());
        state.simulated_crate(name)?.team_owners.push(Team {
            id,
            login: login.to_owned(),
            name: None,
        });
        Ok(id)
    }

    /// Users invited through [`CratesIoClient::add_owners`] that have not
    /// accepted yet.
    pub fn invited_owners(&self, name: &CrateName) -> Result<Vec<OwnerLogin>> {
        Ok(self
            .state
            .lock()
            .simulated_crate(name)?
            .invited_owners
            .clone())
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
                published_by: Some(simulated_user(publisher)),
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

    async fn list_owners(&self, name: &CrateName) -> Result<Vec<Owner>> {
        let mut state = self.state.lock();
        Ok(state.simulated_crate(name)?.all_owners().collect())
    }

    async fn add_owners(
        &self,
        name: &CrateName,
        owners: &[OwnerLogin],
    ) -> Result<Vec<OwnerAddition>> {
        let mut state = self.state.lock();
        let token_user = state.token_user;
        let simulated_crate = state.simulated_crate(name)?;
        simulated_crate.require_owner_rights(token_user)?;
        if owners.len() > 10 {
            return Err(bad_request(
                "too many invites for this request - maximum 10".to_owned(),
            ));
        }
        if let Some(login) = owners.iter().find(|login| {
            simulated_crate
                .all_owners()
                .any(|owner| SimulatedCrate::matches(login, &owner))
        }) {
            return Err(bad_request(format!("`{login}` is already an owner")));
        }

        let mut additions = Vec::new();
        for login in owners {
            let team_id = TeamId(state.next_id());
            let simulated_crate = state.simulated_crate(name)?;
            let addition = match login {
                OwnerLogin::GitHubTeam { .. } => {
                    simulated_crate.team_owners.push(Team {
                        id: team_id,
                        login: login.to_string(),
                        name: None,
                    });
                    OwnerAddition::TeamAdded {
                        login: login.to_string(),
                    }
                }
                OwnerLogin::Username(username)
                | OwnerLogin::CratesIoUser(username)
                | OwnerLogin::GitHubUser(username) => {
                    let username = username.clone();
                    if simulated_crate.invited_owners.contains(login) {
                        OwnerAddition::UserAlreadyInvited { username }
                    } else {
                        simulated_crate.invited_owners.push(login.clone());
                        OwnerAddition::UserInvited { username }
                    }
                }
            };
            additions.push(addition);
        }
        Ok(additions)
    }

    async fn remove_owners(&self, name: &CrateName, owners: &[OwnerLogin]) -> Result<()> {
        let mut state = self.state.lock();
        let token_user = state.token_user;
        let simulated_crate = state.simulated_crate(name)?;
        simulated_crate.require_owner_rights(token_user)?;

        let mut removed = Vec::new();
        for login in owners {
            let matching = simulated_crate
                .all_owners()
                .filter(|owner| SimulatedCrate::matches(login, owner))
                .collect::<Vec<_>>();
            if matching.is_empty() {
                return Err(bad_request(format!(
                    "could not find owner with login `{login}`"
                )));
            }
            removed.extend(matching);
        }
        let is_removed_user = |id: &UserId| removed.contains(&Owner::User(simulated_user(*id)));
        if simulated_crate.owners.iter().all(is_removed_user) {
            return Err(bad_request(
                "cannot remove all individual owners of a crate. Team member don't have permission to modify owners, so at least one individual owner is required.".to_owned(),
            ));
        }

        simulated_crate.owners.retain(|id| !is_removed_user(id));
        simulated_crate
            .team_owners
            .retain(|team| !removed.contains(&Owner::Team(team.clone())));
        Ok(())
    }

    async fn list_github_configs(&self, crate_name: &CrateName) -> Result<Vec<GitHubConfig>> {
        let mut state = self.state.lock();
        state.require_token_user_owns(crate_name)?;
        Ok(state
            .github_configs
            .values()
            .filter(|config| &config.crate_name == crate_name)
            .cloned()
            .collect())
    }

    async fn create_github_config(&self, config: &NewGitHubConfig) -> Result<GitHubConfig> {
        let mut state = self.state.lock();
        state.require_token_user_owns(&config.crate_name)?;
        let existing_configs = state
            .github_configs
            .values()
            .filter(|existing| existing.crate_name == config.crate_name)
            .count();
        if existing_configs >= MAX_GITHUB_CONFIGS_PER_CRATE {
            return Err(CratesIoError::Upstream {
                status: StatusCode::CONFLICT,
                message: format!(
                    "This crate already has the maximum number of GitHub Trusted Publishing configurations ({MAX_GITHUB_CONFIGS_PER_CRATE})"
                ),
            });
        }

        let id = GitHubConfigId(state.next_id());
        let repository_owner_id = GitHubAccountId(state.next_id());
        let github_config = GitHubConfig {
            id,
            crate_name: config.crate_name.clone(),
            repository_owner: config.repository_owner.clone(),
            repository_owner_id,
            repository_name: config.repository_name.clone(),
            workflow_filename: config.workflow_filename.clone(),
            environment: config.environment.clone(),
            created_at: test_timestamp(),
        };
        state.github_configs.insert(id, github_config.clone());
        Ok(github_config)
    }

    async fn delete_github_config(&self, id: GitHubConfigId) -> Result<()> {
        let mut state = self.state.lock();
        let crate_name = state
            .github_configs
            .get(&id)
            .map(|config| config.crate_name.clone())
            .ok_or_else(|| CratesIoError::NotFound {
                message: "Not Found".to_owned(),
            })?;
        state.require_token_user_owns(&crate_name)?;
        state.github_configs.remove(&id);
        Ok(())
    }
}

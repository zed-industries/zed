use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

use anyhow::{Context as _, Result, bail, ensure};
use crates_io_client::{
    Crate, CrateListQuery, CrateName, CrateUpdate, CratesIoClient, CratesIoError, NewGitHubConfig,
    Owner, OwnerLogin, TeamId,
};

use super::publish_plan::PublishPlanEntry;

const TEAM_ID: TeamId = TeamId(12586);
const WORKFLOW: &str = "publish_gpui.yml";

const REQUIRED_OWNERS: &[OwnerLogin] = &[
    OwnerLogin::CratesIoUser(Cow::Borrowed("maxbrunsfeld")),
    OwnerLogin::CratesIoUser(Cow::Borrowed("mikayla-maki")),
    OwnerLogin::CratesIoUser(Cow::Borrowed("MrSubidubi")),
    OwnerLogin::GitHubTeam {
        organization: Cow::Borrowed("zed-industries"),
        team: Cow::Borrowed("crates-io"),
    },
];

fn owner_matches(required: &OwnerLogin, owner: &Owner) -> bool {
    match (required, owner) {
        (OwnerLogin::GitHubTeam { .. }, Owner::Team(team)) => team.login == required.to_string(),
        (
            OwnerLogin::Username(login)
            | OwnerLogin::CratesIoUser(login)
            | OwnerLogin::GitHubUser(login),
            Owner::User(user),
        ) => user.login == login.as_ref(),
        _ => false,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PublishState {
    NotPublished {
        target_crate_name: CrateName,
    },
    RequiresCratesIoUpdates {
        missing_owners: Vec<OwnerLogin>,
        missing_workflow: bool,
        enable_trusted_only: bool,
    },
    ReadyForUpdate,
}

pub struct CheckedCrate {
    pub name: CrateName,
    pub state: PublishState,
}

fn expected_workflow(name: &CrateName) -> NewGitHubConfig {
    NewGitHubConfig {
        crate_name: name.clone(),
        repository_owner: "zed-industries".into(),
        repository_name: "zed".into(),
        workflow_filename: WORKFLOW.into(),
        environment: None,
    }
}

pub async fn check_plan(
    client: &dyn CratesIoClient,
    plan: &[PublishPlanEntry<'_>],
) -> Result<Vec<CheckedCrate>> {
    let names = plan
        .iter()
        .map(|entry| CrateName::new(entry.repository.target_name(entry.package.name())));
    check_crates(client, names, TEAM_ID).await
}

async fn check_crates(
    client: &dyn CratesIoClient,
    names: impl IntoIterator<Item = CrateName>,
    team_id: TeamId,
) -> Result<Vec<CheckedCrate>> {
    let team_crates: BTreeMap<_, _> = client
        .list_crates(&CrateListQuery {
            team_id: Some(team_id),
            ..Default::default()
        })
        .await
        .context("failed to list crates owned by the Zed crates.io team")?
        .into_iter()
        .map(|krate| (krate.name.clone(), krate))
        .collect();
    let mut checked = Vec::new();
    for name in names {
        let resolved = if let Some(krate) = team_crates.get(&name) {
            let owners = client
                .list_owners(&name)
                .await
                .with_context(|| format!("failed to list owners of {name}"))?;
            PublishTarget::ExistingCrate {
                krate: krate.clone(),
                owners,
            }
        } else {
            resolve_target_crate_name(client, &name).await?
        };
        let (krate, owners) = match resolved {
            PublishTarget::NewCrate(target_crate_name) => {
                checked.push(CheckedCrate {
                    name,
                    state: PublishState::NotPublished { target_crate_name },
                });
                continue;
            }
            PublishTarget::ExistingCrate { krate, owners } => (krate, owners),
        };
        let name = krate.name.clone();
        let missing_owners: Vec<_> = REQUIRED_OWNERS
            .iter()
            .filter(|required| !owners.iter().any(|owner| owner_matches(required, owner)))
            .cloned()
            .collect();
        let expected = expected_workflow(&name);
        let configs = client
            .list_github_configs(&name)
            .await
            .with_context(|| format!("failed to list trusted publishers for {name}"))?;
        let missing_workflow = !configs.iter().any(|config| {
            config.repository_owner == expected.repository_owner
                && config.repository_name == expected.repository_name
                && config.workflow_filename == expected.workflow_filename
                && config.environment == expected.environment
        });
        let state = if missing_owners.is_empty() && !missing_workflow && krate.trustpub_only {
            PublishState::ReadyForUpdate
        } else {
            PublishState::RequiresCratesIoUpdates {
                missing_owners,
                missing_workflow,
                enable_trusted_only: !krate.trustpub_only,
            }
        };
        checked.push(CheckedCrate { name, state });
    }
    let mut targets = BTreeSet::new();
    for krate in &checked {
        let target = match &krate.state {
            PublishState::NotPublished { target_crate_name } => target_crate_name,
            _ => &krate.name,
        };
        ensure!(
            targets.insert(target.as_str().to_ascii_lowercase().replace('_', "-")),
            "multiple planned crates resolve to crates.io name {target}"
        );
    }
    Ok(checked)
}

enum PublishTarget {
    NewCrate(CrateName),
    ExistingCrate { krate: Crate, owners: Vec<Owner> },
}

async fn resolve_target_crate_name(
    client: &dyn CratesIoClient,
    planned_name: &CrateName,
) -> Result<PublishTarget> {
    if let Some(resolved) = check_crate_name(client, planned_name).await? {
        return Ok(resolved);
    }
    let normalized = planned_name.as_str().to_ascii_lowercase().replace('_', "-");
    ensure!(
        !normalized.starts_with("zed-"),
        "planned crates.io name {planned_name} is already taken and already has a zed prefix"
    );
    let prefixed = CrateName::new(format!("zed-{planned_name}"));
    if let Some(resolved) = check_crate_name(client, &prefixed).await? {
        return Ok(resolved);
    }
    bail!("neither planned crates.io name {planned_name} nor fallback {prefixed} is available")
}

async fn check_crate_name(
    client: &dyn CratesIoClient,
    name: &CrateName,
) -> Result<Option<PublishTarget>> {
    match client.get_crate(name).await {
        Ok(details) => {
            let owners = client
                .list_owners(&details.krate.name)
                .await
                .with_context(|| format!("failed to list owners of {name}"))?;
            if owners.iter().any(|owner| {
                REQUIRED_OWNERS
                    .iter()
                    .any(|required| owner_matches(required, owner))
            }) {
                Ok(Some(PublishTarget::ExistingCrate {
                    krate: details.krate,
                    owners,
                }))
            } else {
                Ok(None)
            }
        }
        Err(CratesIoError::NotFound { .. }) => Ok(Some(PublishTarget::NewCrate(name.clone()))),
        Err(error) => Err(error)
            .with_context(|| format!("failed to check crates.io name availability for {name}")),
    }
}

pub async fn update_crates(client: &dyn CratesIoClient, checked: &[CheckedCrate]) -> Result<()> {
    for krate in checked {
        let PublishState::RequiresCratesIoUpdates {
            missing_owners,
            missing_workflow,
            enable_trusted_only,
        } = &krate.state
        else {
            continue;
        };
        let name = &krate.name;
        if !missing_owners.is_empty() {
            let outcomes = client
                .add_owners(name, missing_owners)
                .await
                .with_context(|| format!("failed to add owners to {name}"))?;
            println!("{name}: {outcomes:?}");
        }
        if *missing_workflow {
            client
                .create_github_config(&expected_workflow(name))
                .await
                .with_context(|| format!("failed to configure trusted publishing for {name}"))?;
        }
        if *enable_trusted_only {
            client
                .update_crate(
                    name,
                    &CrateUpdate {
                        trustpub_only: Some(true),
                    },
                )
                .await
                .with_context(|| format!("failed to enable trusted-only publishing for {name}"))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crates_io_client::simulated_crates_io_client::SimulatedCratesIoClient;

    #[tokio::test]
    async fn checks_and_repairs_without_treating_invitations_as_ownership() -> Result<()> {
        let client = SimulatedCratesIoClient::new();
        let name = client.test_crate("gpui", client.token_user());
        let team_id = client.test_team_owner(&name, "github:zed-industries:crates-io")?;
        let unassociated = client.test_crate("unassociated", client.token_user());
        let names = || {
            vec![
                name.clone(),
                CrateName::new("unpublished"),
                unassociated.clone(),
            ]
        };
        let checked = check_crates(client.as_ref(), names(), team_id).await?;
        assert!(matches!(
            checked[0].state,
            PublishState::RequiresCratesIoUpdates {
                missing_workflow: true,
                enable_trusted_only: true,
                ..
            }
        ));
        assert_eq!(
            checked[1].state,
            PublishState::NotPublished {
                target_crate_name: CrateName::new("unpublished"),
            }
        );
        assert_eq!(
            checked[2].state,
            PublishState::NotPublished {
                target_crate_name: CrateName::new("zed-unassociated"),
            }
        );
        update_crates(client.as_ref(), &checked).await?;
        let checked = check_crates(client.as_ref(), names(), team_id).await?;
        assert!(matches!(
            &checked[0].state,
            PublishState::RequiresCratesIoUpdates {
                missing_owners,
                missing_workflow: false,
                enable_trusted_only: false,
            } if missing_owners.len() == 3
        ));
        update_crates(client.as_ref(), &checked).await?;
        assert_eq!(client.list_github_configs(&name).await?.len(), 1);
        assert_eq!(client.invited_owners(&name)?.len(), 3);
        Ok(())
    }

    #[tokio::test]
    async fn updates_owned_crates_outside_the_team_list_including_fallbacks() -> Result<()> {
        for (planned, published) in [("owned", "owned"), ("taken", "zed-taken")] {
            let client = SimulatedCratesIoClient::new();
            if planned != published {
                client.test_crate(planned, client.token_user());
            }
            let name = client.test_crate(published, client.token_user());
            client.test_team_owner(&name, "github:zed-industries:crates-io")?;
            let checked =
                check_crates(client.as_ref(), vec![CrateName::new(planned)], TEAM_ID).await?;
            assert_eq!(checked[0].name, name);
            assert!(
                matches!(&checked[0].state, PublishState::RequiresCratesIoUpdates {
                missing_owners, missing_workflow: true, enable_trusted_only: true,
            } if missing_owners.len() == 3)
            );
            update_crates(client.as_ref(), &checked).await?;
            assert!(client.get_crate(&name).await?.krate.trustpub_only);
            assert_eq!(client.list_github_configs(&name).await?.len(), 1);
        }
        Ok(())
    }

    #[test]
    fn recognizes_required_individual_owners() {
        let owner = Owner::User(crates_io_client::User {
            id: crates_io_client::UserId(1),
            login: "maxbrunsfeld".into(),
            name: None,
        });
        assert!(
            REQUIRED_OWNERS
                .iter()
                .any(|required| owner_matches(required, &owner))
        );
    }

    #[tokio::test]
    async fn rejects_taken_fallbacks_and_already_prefixed_names() -> Result<()> {
        let client = SimulatedCratesIoClient::new();
        let name = client.test_crate("taken", client.token_user());
        let fallback = client.test_crate("zed-taken", client.token_user());
        for name in [name, fallback] {
            assert!(
                resolve_target_crate_name(client.as_ref(), &name)
                    .await
                    .is_err()
            );
        }
        let name = client.test_crate("Zed_existing", client.token_user());
        assert!(
            resolve_target_crate_name(client.as_ref(), &name)
                .await
                .is_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn rejects_conflicts_between_resolved_plan_names() -> Result<()> {
        let client = SimulatedCratesIoClient::new();
        let name = client.test_crate("taken", client.token_user());
        let result = check_crates(
            client.as_ref(),
            vec![name, CrateName::new("zed-taken")],
            TEAM_ID,
        )
        .await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn propagates_permission_errors() -> Result<()> {
        let client = SimulatedCratesIoClient::new();
        let other_user = client.test_user();
        let name = client.test_crate("gpui", other_user);
        let team_id = client.test_team_owner(&name, "github:zed-industries:crates-io")?;
        assert!(
            check_crates(client.as_ref(), vec![name], team_id)
                .await
                .is_err()
        );
        Ok(())
    }
}

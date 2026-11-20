use std::{borrow::Cow, collections::BTreeMap};

use anyhow::{Context as _, Result};
use crates_io_client::{
    CrateListQuery, CrateName, CrateUpdate, CratesIoClient, CratesIoError, NewGitHubConfig, Owner,
    OwnerLogin, TeamId,
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
    NotPublished,
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
    check_crates(client, names).await
}

async fn check_crates(
    client: &dyn CratesIoClient,
    names: impl IntoIterator<Item = CrateName>,
) -> Result<Vec<CheckedCrate>> {
    let mut team_crates: BTreeMap<_, _> = client
        .list_crates(&CrateListQuery {
            team_id: Some(TEAM_ID),
            ..Default::default()
        })
        .await
        .context("failed to list crates owned by the Zed crates.io team")?
        .into_iter()
        .map(|krate| (krate.name.clone(), krate))
        .collect();
    let mut checked = Vec::new();
    for name in names {
        // A crate missing from the team list may exist under different owners.
        let krate = match team_crates.remove(&name) {
            Some(krate) => krate,
            None => match client.get_crate(&name).await {
                Ok(details) => details.krate,
                Err(CratesIoError::NotFound { .. }) => {
                    checked.push(CheckedCrate {
                        name,
                        state: PublishState::NotPublished,
                    });
                    continue;
                }
                Err(error) => {
                    return Err(error).with_context(|| format!("failed to look up {name}"));
                }
            },
        };
        let owners = client
            .list_owners(&name)
            .await
            .with_context(|| format!("failed to list owners of {name}"))?;
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
    Ok(checked)
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
        let names = || vec![name.clone(), CrateName::new("unpublished")];
        let checked = check_crates(client.as_ref(), names()).await?;
        assert!(matches!(
            checked[0].state,
            PublishState::RequiresCratesIoUpdates {
                missing_workflow: true,
                enable_trusted_only: true,
                ..
            }
        ));
        assert_eq!(checked[1].state, PublishState::NotPublished);
        update_crates(client.as_ref(), &checked).await?;
        let checked = check_crates(client.as_ref(), names()).await?;
        assert!(
            matches!(&checked[0].state, PublishState::RequiresCratesIoUpdates { missing_owners, missing_workflow: false, enable_trusted_only: false } if missing_owners.len() == 3)
        );
        update_crates(client.as_ref(), &checked).await?;
        assert_eq!(client.list_github_configs(&name).await?.len(), 1);
        assert_eq!(client.invited_owners(&name)?.len(), 3);
        Ok(())
    }

    #[tokio::test]
    async fn propagates_permission_errors() {
        let client = SimulatedCratesIoClient::new();
        let other_user = client.test_user();
        let name = client.test_crate("gpui", other_user);
        assert!(check_crates(client.as_ref(), vec![name]).await.is_err());
    }
}

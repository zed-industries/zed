use std::fmt;

use anyhow::{Context as _, Result, bail, ensure};
use guppy::graph::{DependencyDirection, ExternalSource, PackageGraph, PackageMetadata};
use url::Url;

use crate::tasks::{gpui::GpuiArgs, workflows::GitSha};

pub enum Repository {
    Workspace,
    Git(GitRepository),
}

pub struct GitRepository {
    pub owner: String,
    pub repo: String,
    pub revision: GitSha,
}

impl GitRepository {
    fn new(repository: &str, revision: &str) -> Result<Self> {
        let url = Url::parse(repository).context("invalid Git repository URL")?;
        ensure!(
            url.host_str()
                .is_some_and(|host| host.eq_ignore_ascii_case("github.com")),
            "expected a GitHub repository, got {repository}"
        );
        let (owner, repo) = url
            .path()
            .trim_matches('/')
            .split_once('/')
            .context("repository URL must contain an owner and repository name")?;
        let repo = repo.strip_suffix(".git").unwrap_or(repo);
        ensure!(
            !owner.is_empty() && !repo.is_empty() && !repo.contains('/'),
            "expected an owner/repo path, got {repository}"
        );
        let revision = revision
            .parse::<GitSha>()
            .map_err(anyhow::Error::msg)
            .with_context(|| format!("invalid resolved revision for {repository}"))?;

        Ok(Self {
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            revision,
        })
    }
}

impl Repository {
    pub fn fork(repository: &str, revision: &str) -> Result<Self> {
        Ok(Self::Git(GitRepository::new(repository, revision)?))
    }

    pub fn publish_info<'a>(&'a self, name: &'a str) -> CratePublishInfo<'a> {
        CratePublishInfo {
            repository: self,
            original_name: name,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Workspace => "zed-industries/zed".to_owned(),
            Self::Git(repository) => format!(
                "{owner}/{repo}",
                owner = repository.owner,
                repo = repository.repo
            ),
        }
    }

    pub fn target_name(&self, name: &str) -> String {
        match self {
            Self::Workspace => {
                if name == "gpui_macros" {
                    "gpui-macros".to_owned()
                } else if name == "gpui" || name.starts_with("gpui_") {
                    name.to_owned()
                } else {
                    format!("gpui_{name}")
                }
            }
            Self::Git(_) => {
                if name.starts_with("zed-") {
                    name.to_owned()
                } else {
                    format!("zed-{name}")
                }
            }
        }
    }

    pub fn display<'a>(&'a self, args: &'a GpuiArgs) -> impl fmt::Display + 'a {
        let revision = match self {
            Self::Workspace => &args.sha,
            Self::Git(repository) => &repository.revision,
        };
        fmt::from_fn(move |formatter| {
            write!(
                formatter,
                "{name}@{revision}",
                name = self.name(),
                revision = revision.as_ref()
            )
        })
    }
}

pub struct CratePublishInfo<'a> {
    pub repository: &'a Repository,
    pub original_name: &'a str,
}

pub struct PublishPlanEntry<'graph> {
    pub package: PackageMetadata<'graph>,
    pub repository: Repository,
}

pub fn build_publish_plan(graph: &PackageGraph) -> Result<Vec<PublishPlanEntry<'_>>> {
    let packages = super::crate_graph::gpui_crates(graph)?;
    packages
        .packages(DependencyDirection::Reverse)
        .map(|package| {
            let repository = if package.in_workspace() {
                Repository::Workspace
            } else if let Some(ExternalSource::Git {
                repository,
                resolved,
                ..
            }) = package.source().parse_external()
            {
                Repository::fork(repository, resolved)?
            } else {
                bail!(
                    "unexpected source for GPUI crate {}: {}",
                    package.name(),
                    package.source()
                );
            };
            Ok(PublishPlanEntry {
                package,
                repository,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REVISION: &str = "0123456789abcdef0123456789abcdef01234567";
    const WORKSPACE_REVISION: &str = "fedcba9876543210fedcba9876543210fedcba98";

    #[test]
    fn publishing_info_borrows_its_repository_and_crate_name() {
        let repository = Repository::Workspace;
        let name = String::from("collections");
        let package = repository.publish_info(&name);

        assert!(std::ptr::eq(package.repository, &repository));
        assert!(std::ptr::eq(package.original_name, name.as_str()));
    }

    #[test]
    fn workspace_names_follow_existing_publish_names() {
        let repository = Repository::Workspace;
        for (original, target) in [
            ("collections", "gpui_collections"),
            ("perf", "gpui_perf"),
            ("util_macros", "gpui_util_macros"),
            ("util", "gpui_util"),
            ("gpui_macros", "gpui-macros"),
            ("http_client", "gpui_http_client"),
            ("derive_refineable", "gpui_derive_refineable"),
            ("refineable", "gpui_refineable"),
            ("semantic_version", "gpui_semantic_version"),
            ("sum_tree", "gpui_sum_tree"),
            ("media", "gpui_media"),
            ("gpui", "gpui"),
            ("gpui_platform", "gpui_platform"),
            ("gpui-macros", "gpui_gpui-macros"),
            ("scheduler", "gpui_scheduler"),
        ] {
            let package = repository.publish_info(original);
            assert_eq!(package.original_name, original);
            assert_eq!(
                package.repository.target_name(package.original_name),
                target
            );
            assert!(matches!(package.repository, Repository::Workspace));
        }
    }

    #[test]
    fn fork_names_have_a_single_zed_prefix() -> Result<()> {
        let repository = Repository::fork("https://github.com/zed-industries/fork", REVISION)?;
        for (original, target) in [
            ("calloop", "zed-calloop"),
            ("wasm_thread", "zed-wasm_thread"),
            ("zed-font-kit", "zed-font-kit"),
            ("zed-scap", "zed-scap"),
        ] {
            let package = repository.publish_info(original);
            assert_eq!(package.original_name, original);
            assert_eq!(
                package.repository.target_name(package.original_name),
                target
            );
        }
        Ok(())
    }

    #[test]
    fn workspace_repository_uses_the_argument_sha() -> Result<()> {
        let repository = Repository::Workspace;
        let package = repository.publish_info("gpui");
        assert_eq!(package.repository.name(), "zed-industries/zed");
        for revision in [REVISION, WORKSPACE_REVISION] {
            let args = GpuiArgs {
                sha: revision.parse().map_err(anyhow::Error::msg)?,
            };
            assert_eq!(
                package.repository.display(&args).to_string(),
                format!("zed-industries/zed@{revision}")
            );
        }
        Ok(())
    }

    #[test]
    fn git_repositories_have_checkout_coordinates() -> Result<()> {
        let args = GpuiArgs {
            sha: WORKSPACE_REVISION.parse().map_err(anyhow::Error::msg)?,
        };
        for url in [
            "https://github.com/zed-industries/fork",
            "https://github.com/zed-industries/fork.git",
            "https://github.com/zed-industries/fork.git/",
            "ssh://git@github.com/zed-industries/fork.git",
        ] {
            let repository = Repository::fork(url, REVISION)?;
            let package = repository.publish_info("fork");
            assert_eq!(package.repository.name(), "zed-industries/fork");
            assert_eq!(
                package.repository.display(&args).to_string(),
                format!("zed-industries/fork@{REVISION}")
            );
            let Repository::Git(repository) = package.repository else {
                bail!("expected a Git repository");
            };
            assert_eq!(repository.owner, "zed-industries");
            assert_eq!(repository.repo, "fork");
            assert_eq!(repository.revision.as_ref(), REVISION);
        }
        Ok(())
    }

    #[test]
    fn git_repositories_require_an_owner_repo_and_full_sha() {
        for url in [
            "not a URL",
            "https://example.com/zed-industries/fork",
            "https://github.com/zed-industries",
            "https://github.com/zed-industries/",
            "https://github.com/zed-industries/.git",
            "https://github.com/zed-industries/fork/tree/main",
        ] {
            assert!(Repository::fork(url, REVISION).is_err(), "{url}");
        }
        for revision in [
            "",
            "main",
            "v1.0",
            "abc",
            "g123456789abcdef0123456789abcdef01234567",
        ] {
            assert!(
                Repository::fork("https://github.com/zed-industries/fork", revision).is_err(),
                "{revision}"
            );
        }
    }
}

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result, ensure};
use guppy::graph::{ExternalSource, PackageGraph, PackageMetadata};
use serde::{Deserialize, Serialize};

use super::{
    publish_plan::{PublishPlanEntry, Repository},
    registry::{CheckedCrate, PublishState},
};
use crate::tasks::workflows::GitSha;

#[derive(Serialize, Deserialize)]
pub(super) struct Release {
    pub(super) version: semver::Version,
    pub(super) repositories: Vec<RepositoryRelease>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct RepositoryRelease {
    pub(super) repository: String,
    pub(super) revision: String,
    pub(super) crates: Vec<CrateRelease>,
    pub(super) dependencies: BTreeMap<PathBuf, BTreeMap<String, DependencyEdit>>,
    #[serde(skip)]
    prerequisites: BTreeSet<String>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct CrateRelease {
    pub(super) original_name: String,
    pub(super) name: String,
    pub(super) manifest: PathBuf,
    first_publish: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub(super) struct DependencyEdit {
    pub(super) original_name: String,
    pub(super) name: String,
    pub(super) same_repository: bool,
}

#[derive(Clone, Copy, Serialize)]
struct RepositoryMatrixEntry<'a> {
    repository: &'a str,
    repo: &'a str,
    revision: &'a str,
    tag: &'a str,
}

#[derive(Serialize)]
struct CrateMatrixEntry<'a> {
    #[serde(flatten)]
    repository: RepositoryMatrixEntry<'a>,
    name: &'a str,
    manifest: &'a Path,
    first_publish: bool,
    prepare_ref: bool,
    runner: &'static str,
}

impl Release {
    pub(super) fn write_workflow_output(&self, output: &Path) -> Result<()> {
        use std::io::Write as _;

        let tag = format!("gpui-v{}", self.version);
        let mut repositories = Vec::new();
        let mut crates = Vec::new();
        for group in &self.repositories {
            let (_, repo) = group
                .repository
                .split_once('/')
                .context("missing repository owner")?;
            let repository = RepositoryMatrixEntry {
                repository: &group.repository,
                repo,
                revision: &group.revision,
                tag: &tag,
            };
            repositories.push(repository);
            for (index, krate) in group.crates.iter().enumerate() {
                crates.push(CrateMatrixEntry {
                    repository,
                    name: &krate.name,
                    manifest: &krate.manifest,
                    first_publish: krate.first_publish,
                    prepare_ref: index == 0,
                    runner: match krate.original_name.as_str() {
                        "windows-capture" | "gpui_windows" => "windows-2022",
                        "gpui_apple" | "gpui_macos" => "namespace-profile-mac-large-xcode26",
                        _ => "namespace-profile-16x32-ubuntu-2204",
                    },
                });
            }
        }
        let mut output = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output)?;
        writeln!(
            output,
            "repositories={}",
            serde_json::to_string(&repositories)?
        )?;
        writeln!(output, "crates={}", serde_json::to_string(&crates)?)?;
        writeln!(output, "release_plan={}", serde_json::to_string(self)?)?;
        Ok(())
    }
}

pub(super) fn build(
    graph: &PackageGraph,
    plan: &[PublishPlanEntry<'_>],
    checked: &[CheckedCrate],
    sha: &GitSha,
    version: semver::Version,
) -> Result<Release> {
    let mut repositories = indexmap::IndexMap::<String, RepositoryRelease>::new();
    let mut targets = BTreeMap::new();
    ensure!(plan.len() == checked.len(), "incomplete GPUI registry plan");
    for (entry, checked) in plan.iter().zip(checked) {
        let (name, first_publish) = match &checked.state {
            PublishState::NotPublished { target_crate_name } => (target_crate_name, true),
            PublishState::ReadyForUpdate => (&checked.name, false),
            PublishState::RequiresCratesIoUpdates { .. } => anyhow::bail!(
                "{} is not ready for trusted publishing: {:?}. Run `gpui plan --update-registry` to configure it.",
                checked.name,
                checked.state
            ),
        };
        let repository = entry.repository.name();
        let revision = entry.repository.revision(sha).as_ref();
        let group = repositories
            .entry(repository.clone())
            .or_insert_with(|| RepositoryRelease {
                repository: repository.clone(),
                revision: revision.to_owned(),
                crates: Vec::new(),
                dependencies: BTreeMap::new(),
                prerequisites: BTreeSet::new(),
            });
        ensure!(
            group.revision == revision,
            "{repository} is required at multiple revisions; cannot create one GPUI tag"
        );
        group.crates.push(CrateRelease {
            original_name: entry.package.name().to_owned(),
            name: name.to_string(),
            manifest: relative_manifest(graph, entry.package)?,
            first_publish,
        });
        targets.insert(entry.package.id(), (repository, name));
    }

    for package in graph.packages() {
        let repository = if package.in_workspace() {
            Repository::Workspace
        } else if let Some(ExternalSource::Git {
            repository,
            resolved,
            ..
        }) = package.source().parse_external()
            && super::crate_graph::is_zed_fork(package.source())
        {
            Repository::fork(repository, resolved)?
        } else {
            continue;
        };
        let repository_name = repository.name();
        let Some(group) = repositories.get_mut(&repository_name) else {
            continue;
        };
        if repository.revision(sha).as_ref() != group.revision {
            continue;
        }
        for link in package.direct_links() {
            let dependency = link.to();
            let Some((target_repository, name)) = targets.get(dependency.id()) else {
                continue;
            };
            let same_repository = *target_repository == repository_name;
            if !same_repository {
                group.prerequisites.insert(target_repository.clone());
            }
            let edits = group
                .dependencies
                .entry(relative_manifest(graph, package)?)
                .or_default();
            let edit = DependencyEdit {
                original_name: dependency.name().to_owned(),
                name: name.to_string(),
                same_repository,
            };
            if let Some(previous) = edits.insert(link.dep_name().to_owned(), edit) {
                ensure!(
                    edits.get(link.dep_name()) == Some(&previous),
                    "conflicting release targets for {} in {}",
                    link.dep_name(),
                    package.manifest_path()
                );
            }
        }
    }

    // Cargo resolves the entire workspace when publishing even a single package.
    let mut ordered = Vec::new();
    while !repositories.is_empty() {
        let ready = repositories
            .values()
            .find(|repository| {
                repository
                    .prerequisites
                    .iter()
                    .all(|name| !repositories.contains_key(name))
            })
            .map(|repository| repository.repository.clone())
            .context("GPUI release repositories have cyclic Cargo dependencies")?;
        ordered.push(
            repositories
                .shift_remove(&ready)
                .context("release repository disappeared")?,
        );
    }
    Ok(Release {
        version,
        repositories: ordered,
    })
}

fn relative_manifest(graph: &PackageGraph, package: PackageMetadata<'_>) -> Result<PathBuf> {
    let manifest = package.manifest_path().as_std_path();
    let root = if package.in_workspace() {
        graph.workspace().root().as_std_path()
    } else {
        manifest
            .ancestors()
            .find(|path| path.join(".git").exists())
            .with_context(|| format!("cannot find Git checkout for {}", manifest.display()))?
    };
    Ok(manifest.strip_prefix(root)?.to_owned())
}

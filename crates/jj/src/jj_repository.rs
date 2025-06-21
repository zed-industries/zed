use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use gpui::{BackgroundExecutor, SharedString};
use jj_lib::backend::BackendResult;
use jj_lib::config::StackedConfig;
use jj_lib::merge::MergedTreeValue;
use jj_lib::ref_name::WorkspaceNameBuf;
use jj_lib::repo::{Repo as _, StoreFactories};
use jj_lib::repo_path::RepoPathBuf;
use jj_lib::settings::UserSettings;
use jj_lib::workspace::{self, DefaultWorkspaceLoaderFactory, Workspace, WorkspaceLoaderFactory};

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub ref_name: SharedString,
}

pub trait JujutsuRepository: Send + Sync {
    fn list_bookmarks(&self) -> Vec<Bookmark>;
}

#[allow(dead_code)]
pub struct RealJujutsuRepository {
    cwd: PathBuf,
    ws_name: WorkspaceNameBuf,
    repository: Arc<jj_lib::repo::ReadonlyRepo>,
    executor: BackgroundExecutor,
}

impl RealJujutsuRepository {
    fn load_workspace(cwd: &Path) -> Result<Workspace> {
        let workspace_loader_factory = DefaultWorkspaceLoaderFactory;
        let workspace_loader = workspace_loader_factory.create(Self::find_workspace_dir(cwd))?;

        let config = StackedConfig::with_defaults();
        let settings = UserSettings::from_config(config)?;

        Ok(workspace_loader.load(
            &settings,
            &StoreFactories::default(),
            &workspace::default_working_copy_factories(),
        )?)
    }

    pub fn new(cwd: &Path, executor: BackgroundExecutor) -> Result<Self> {
        let workspace = Self::load_workspace(cwd)?;
        let repo_loader = workspace.repo_loader();
        let repository = repo_loader.load_at_head()?;

        Ok(Self {
            repository,
            executor,
            ws_name: workspace.workspace_name().to_owned(),
            cwd: cwd.to_path_buf(),
        })
    }

    fn find_workspace_dir(cwd: &Path) -> &Path {
        cwd.ancestors()
            .find(|path| path.join(".jj").is_dir())
            .unwrap_or(cwd)
    }
    pub fn status(&self) -> impl Iterator<Item = (RepoPathBuf, BackendResult<MergedTreeValue>)> {
        let wc_commit_id = self.repository.view().get_wc_commit_id(&self.ws_name);
        let wc_commit = self
            .repository
            .store()
            .get_commit(wc_commit_id.unwrap())
            .unwrap();
        wc_commit.tree().unwrap().entries()
    }
}

impl JujutsuRepository for RealJujutsuRepository {
    fn list_bookmarks(&self) -> Vec<Bookmark> {
        let bookmarks = self
            .repository
            .view()
            .bookmarks()
            .map(|(ref_name, _target)| Bookmark {
                ref_name: ref_name.as_str().to_string().into(),
            })
            .collect();

        bookmarks
    }
}

pub struct FakeJujutsuRepository {}

impl JujutsuRepository for FakeJujutsuRepository {
    fn list_bookmarks(&self) -> Vec<Bookmark> {
        Vec::new()
    }
}

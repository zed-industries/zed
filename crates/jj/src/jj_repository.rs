use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use gpui::SharedString;
use jj_lib::config::StackedConfig;
use jj_lib::repo::StoreFactories;
use jj_lib::settings::UserSettings;
use jj_lib::workspace::{self, DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory};

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub ref_name: SharedString,
}

pub trait JujutsuRepository: Send + Sync {
    fn list_bookmarks(&self) -> Vec<Bookmark>;
}

pub struct RealJujutsuRepository {
    repository: Arc<jj_lib::repo::ReadonlyRepo>,
}

impl RealJujutsuRepository {
    pub fn new(cwd: &Path) -> Result<Self> {
        let workspace_loader_factory = DefaultWorkspaceLoaderFactory;
        let workspace_loader = workspace_loader_factory.create(Self::find_workspace_dir(cwd))?;

        let config = StackedConfig::with_defaults();
        let settings = UserSettings::from_config(config)?;

        let workspace = workspace_loader.load(
            &settings,
            &StoreFactories::default(),
            &workspace::default_working_copy_factories(),
        )?;

        let repo_loader = workspace.repo_loader();
        let repository = repo_loader.load_at_head()?;

        Ok(Self { repository })
    }

    fn find_workspace_dir(cwd: &Path) -> &Path {
        cwd.ancestors()
            .find(|path| path.join(".jj").is_dir())
            .unwrap_or(cwd)
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

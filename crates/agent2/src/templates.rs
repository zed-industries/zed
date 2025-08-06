use std::{path::Path, sync::Arc};

use anyhow::Result;
use handlebars::Handlebars;
use prompt_store::UserPromptId;
use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "src/templates"]
#[include = "*.hbs"]
struct Assets;

pub struct Templates(Handlebars<'static>);

impl Templates {
    pub fn new() -> Arc<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.register_embed_templates::<Assets>().unwrap();
        Arc::new(Self(handlebars))
    }
}

pub trait Template: Sized {
    const TEMPLATE_NAME: &'static str;

    fn render(&self, templates: &Templates) -> Result<String>
    where
        Self: Serialize + Sized,
    {
        Ok(templates.0.render(Self::TEMPLATE_NAME, self)?)
    }
}

#[derive(Serialize)]
pub struct BaseTemplate {
    pub os: String,
    pub shell: String,
    pub worktrees: Vec<WorktreeData>,
}

impl Template for BaseTemplate {
    const TEMPLATE_NAME: &'static str = "base.hbs";
}

#[derive(Serialize)]
pub struct GlobTemplate {
    pub project_roots: String,
}

impl Template for GlobTemplate {
    const TEMPLATE_NAME: &'static str = "glob.hbs";
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemPromptTemplate {
    pub worktrees: Vec<WorktreeData>,
    /// Whether any worktree has a rules_file. Provided as a field because handlebars can't do this.
    pub has_rules: bool,
    pub user_rules: Vec<UserRulesData>,
    /// `!user_rules.is_empty()` - provided as a field because handlebars can't do this.
    pub has_user_rules: bool,
    pub os: String,
    pub arch: String,
    pub shell: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorktreeData {
    pub root_name: String,
    pub abs_path: Arc<Path>,
    pub rules_file: Option<RulesFileData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RulesFileData {
    pub path_in_worktree: Arc<Path>,
    pub text: String,
    // This used for opening rules files. TODO: Since it isn't related to prompt templating, this
    // should be moved elsewhere.
    // #[serde(skip)]
    // pub project_entry_id: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserRulesData {
    pub uuid: UserPromptId,
    pub title: Option<String>,
    pub contents: String,
}

use std::sync::Arc;

use anyhow::Result;
use handlebars::Handlebars;
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
pub struct WorktreeData {
    pub root_name: String,
}

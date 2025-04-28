use anyhow::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents site content for templating
#[derive(Serialize)]
pub struct SiteContent {
    pub title: String,
    pub content: String,
    pub examples: Vec<ExampleInfo>,
    pub docs: Vec<DocInfo>,
}

/// Information about a code example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleInfo {
    pub name: String,
    pub title: String,
    pub description: String,
    pub path: String,
}

/// Information about a documentation page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocInfo {
    pub name: String,
    pub title: String,
    pub path: String,
}

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new(_output_dir: &Path) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        
        // Get the path to our template files in the source directory
        let template_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/templates");
        
        // Register partials and templates
        handlebars.register_template_file("base", template_dir.join("base.hbs"))?;
        handlebars.register_template_file("base_subdir", template_dir.join("base_subdir.hbs"))?;
        handlebars.register_template_file("index", template_dir.join("index.hbs"))?;
        handlebars.register_template_file("example", template_dir.join("example.hbs"))?;
        handlebars.register_template_file("doc", template_dir.join("doc.hbs"))?;
        
        Ok(Self { handlebars })
    }

    /// Render the index page
    pub fn render_index(&self, content: &SiteContent) -> Result<String> {
        let rendered = self.handlebars.render("index", content)?;
        Ok(rendered)
    }

    /// Render an example page
    pub fn render_example(
        &self,
        example: &ExampleInfo,
        code: &str,
        content: &SiteContent,
    ) -> Result<String> {
        let ctx = serde_json::json!({
            "title": &example.title,
            "example": example,
            "code": code,
            "examples": &content.examples,
            "docs": &content.docs,
        });
        
        let rendered = self.handlebars.render("example", &ctx)?;
        Ok(rendered)
    }

    /// Render a documentation page
    pub fn render_doc(
        &self,
        doc: &DocInfo,
        content: &str,
        site_content: &SiteContent,
    ) -> Result<String> {
        let ctx = serde_json::json!({
            "title": &doc.title,
            "doc": doc,
            "content": content,
            "examples": &site_content.examples,
            "docs": &site_content.docs,
        });
        
        let rendered = self.handlebars.render("doc", &ctx)?;
        Ok(rendered)
    }
}

use anyhow::Result;
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Represents site content for templating
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
    env: Environment<'static>,
}

impl TemplateEngine {
    pub fn new(templates_dir: &Path) -> Result<Self> {
        let mut env = Environment::new();
        
        // Load the templates we've created from the output directory
        // Since MiniJinja 1.0 doesn't have path_loader, we'll manually load them
        let base_html = fs::read_to_string(templates_dir.join("base.html"))?;
        let base_subdir_html = fs::read_to_string(templates_dir.join("base_subdir.html"))?;
        let index_html = fs::read_to_string(templates_dir.join("index.html"))?;
        let example_html = fs::read_to_string(templates_dir.join("example.html"))?;
        let doc_html = fs::read_to_string(templates_dir.join("doc.html"))?;
        
        // Convert to static lifetime strings with Box leak
        let base_html = Box::leak(base_html.into_boxed_str());
        let base_subdir_html = Box::leak(base_subdir_html.into_boxed_str());
        let index_html = Box::leak(index_html.into_boxed_str());
        let example_html = Box::leak(example_html.into_boxed_str());
        let doc_html = Box::leak(doc_html.into_boxed_str());
        
        env.add_template("base.html", base_html)?;
        env.add_template("base_subdir.html", base_subdir_html)?;
        env.add_template("index.html", index_html)?;
        env.add_template("example.html", example_html)?;
        env.add_template("doc.html", doc_html)?;
        
        Ok(Self { env })
    }
    
    /// Render the index page
    pub fn render_index(&self, content: &SiteContent) -> Result<String> {
        let template = self.env.get_template("index.html")?;
        let rendered = template.render(context! {
            title => &content.title,
            content => &content.content,
            examples => &content.examples,
            docs => &content.docs,
        })?;
        Ok(rendered)
    }
    
    /// Render an example page
    pub fn render_example(&self, example: &ExampleInfo, code: &str, content: &SiteContent) -> Result<String> {
        let template = self.env.get_template("example.html")?;
        let rendered = template.render(context! {
            title => &example.title,
            example => example,
            code => code,
            examples => &content.examples,
            docs => &content.docs,
        })?;
        Ok(rendered)
    }
    
    /// Render a documentation page
    pub fn render_doc(&self, doc: &DocInfo, content: &str, site_content: &SiteContent) -> Result<String> {
        let template = self.env.get_template("doc.html")?;
        let rendered = template.render(context! {
            title => &doc.title,
            doc => doc,
            content => content,
            examples => &site_content.examples,
            docs => &site_content.docs,
        })?;
        Ok(rendered)
    }
}

// Create template stubs that we'll need to include
pub fn create_template_stubs(output_dir: &Path) -> Result<()> {
    let templates_dir = output_dir.join("templates");
    fs::create_dir_all(&templates_dir)?;
    
    // Create basic templates
    // First create the main base template
    fs::write(
        templates_dir.join("base.html"),
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }} - gpui</title>
    <link rel="stylesheet" href="css/styles.css">
</head>
<body>
    <header>
        <div class="container">
            <a href="index.html" class="logo">gpui</a>
            <nav>
                <ul>
                    <li><a href="index.html">Home</a></li>
                    <li>
                        <span>Examples</span>
                        <ul>
                            {% for example in examples %}
                            <li><a href="examples/{{ example.path }}">{{ example.title }}</a></li>
                            {% endfor %}
                        </ul>
                    </li>
                    <li>
                        <span>Docs</span>
                        <ul>
                            {% for doc in docs %}
                            <li><a href="docs/{{ doc.path }}">{{ doc.title }}</a></li>
                            {% endfor %}
                        </ul>
                    </li>
                    <li><a href="https://github.com/zed-industries/zed/tree/main/crates/gpui">GitHub</a></li>
                </ul>
            </nav>
        </div>
    </header>

    <main class="container">
        {% block content %}{% endblock %}
    </main>

    <footer>
        <div class="container">
            <p>gpui is part of the <a href="https://github.com/zed-industries/zed">Zed</a> project © Zed Industries, Inc.</p>
        </div>
    </footer>
    
    <script src="js/main.js"></script>
</body>
</html>
"#,
    )?;
    
    // Create a base template for pages in subdirectories (examples/, docs/)
    fs::write(
        templates_dir.join("base_subdir.html"),
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }} - gpui</title>
    <link rel="stylesheet" href="../css/styles.css">
</head>
<body>
    <header>
        <div class="container">
            <a href="../index.html" class="logo">gpui</a>
            <nav>
                <ul>
                    <li><a href="../index.html">Home</a></li>
                    <li>
                        <span>Examples</span>
                        <ul>
                            {% for example in examples %}
                            <li><a href="../examples/{{ example.path }}">{{ example.title }}</a></li>
                            {% endfor %}
                        </ul>
                    </li>
                    <li>
                        <span>Docs</span>
                        <ul>
                            {% for doc in docs %}
                            <li><a href="../docs/{{ doc.path }}">{{ doc.title }}</a></li>
                            {% endfor %}
                        </ul>
                    </li>
                    <li><a href="https://github.com/zed-industries/zed/tree/main/crates/gpui">GitHub</a></li>
                </ul>
            </nav>
        </div>
    </header>

    <main class="container">
        {% block content %}{% endblock %}
    </main>

    <footer>
        <div class="container">
            <p>gpui is part of the <a href="https://github.com/zed-industries/zed">Zed</a> project © Zed Industries, Inc.</p>
        </div>
    </footer>
    
    <script src="../js/main.js"></script>
</body>
</html>
"#,
    )?;
    
    fs::write(
        templates_dir.join("index.html"),
        r#"{% extends "base.html" %}

{% block content %}
<section class="hero">
    <h1>gpui</h1>
    <p class="tagline">A fast, productive UI framework for Rust from the creators of Zed.</p>
    <div class="cta-buttons">
        <a href="docs/intro.html" class="button primary">Get Started →</a>
        <a href="https://github.com/zed-industries/zed/tree/main/crates/gpui" class="button secondary">GitHub</a>
    </div>
</section>

<section class="content">
    {{ content | safe }}
</section>

<section class="examples-grid">
    <h2>Examples</h2>
    <div class="grid">
        {% for example in examples %}
        <a href="examples/{{ example.path }}" class="example-card">
            <h3>{{ example.title }}</h3>
            <p>{{ example.description }}</p>
        </a>
        {% endfor %}
    </div>
</section>
{% endblock %}
"#,
    )?;
    
    fs::write(
        templates_dir.join("example.html"),
        r#"{% extends "base_subdir.html" %}

{% block content %}
<article class="example">
    <h1>{{ example.title }}</h1>
    <p>{{ example.description }}</p>
    
    <div class="code-container">
        {{ code | safe }}
    </div>
    
    <div class="example-info">
        <h3>Running this example</h3>
        <p>You can run this example with:</p>
        <pre><code>cargo run --example {{ example.name }}</code></pre>
    </div>
</article>
{% endblock %}
"#,
    )?;
    
    fs::write(
        templates_dir.join("doc.html"),
        r#"{% extends "base_subdir.html" %}

{% block content %}
<article class="documentation">
    <h1>{{ doc.title }}</h1>
    <div class="content">
        {{ content | safe }}
    </div>
</article>
{% endblock %}
"#,
    )?;
    
    Ok(())
}
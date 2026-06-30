use anyhow::{Context, Result};
use mdbook::BookItem;
use mdbook::book::Book;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::FRONT_MATTER_COMMENT;

#[derive(Debug)]
pub(crate) struct DocsPage {
    section: String,
    title: String,
    description: Option<String>,
    pub(crate) last_updated: Option<String>,
    pub(crate) source_path: PathBuf,
    content: String,
}

pub(crate) fn write_ai_discovery_artifacts(
    pages: &[DocsPage],
    destination: &Path,
    site_url: &str,
) -> Result<()> {
    copy_markdown_sources(destination, site_url, pages)?;
    write_llms_txt(destination, site_url, pages)?;
    write_sitemap_xml(destination, site_url, pages)?;
    Ok(())
}

pub(crate) fn docs_pages(book: &Book, docs_root: &Path) -> Result<Vec<DocsPage>> {
    let mut pages = Vec::new();
    let mut section = "Docs".to_string();
    let git_last_updated = docs_page_last_updated_from_git(docs_root);
    let last_updated_fallbacks = docs_page_last_updated_fallbacks(docs_root)?;
    let mut missing_last_updated = Vec::new();
    for item in book.iter() {
        let BookItem::Chapter(chapter) = item else {
            if let BookItem::PartTitle(part_title) = item {
                section.clone_from(part_title);
            }
            continue;
        };
        let Some(source_path) = chapter.source_path.as_ref() else {
            continue;
        };
        if source_path == Path::new("SUMMARY.md") {
            continue;
        }
        let source_path_key = source_path.to_string_lossy().replace('\\', "/");
        let last_updated = git_last_updated
            .get(&source_path_key)
            .or_else(|| last_updated_fallbacks.get(&source_path_key))
            .cloned();
        if last_updated.is_none() {
            missing_last_updated.push(source_path_key);
        }
        pages.push(DocsPage {
            section: section.clone(),
            title: chapter.name.clone(),
            description: docs_page_description(&chapter.content),
            last_updated,
            source_path: source_path.clone(),
            content: chapter.content.clone(),
        });
    }
    if !missing_last_updated.is_empty() {
        anyhow::bail!(
            "missing last-updated metadata for docs pages: {}",
            missing_last_updated.join(", ")
        );
    }
    Ok(pages)
}

fn copy_markdown_sources(destination: &Path, site_url: &str, pages: &[DocsPage]) -> Result<()> {
    for page in pages {
        let destination = destination.join(&page.source_path);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("failed to create markdown destination {}", parent.display())
            })?;
        }
        std::fs::write(
            &destination,
            add_llms_markdown_directive(
                &markdown_source_contents(&page.content),
                site_url,
                page.last_updated.as_deref(),
            ),
        )
        .with_context(|| {
            format!(
                "failed to write markdown page {} to {}",
                page.source_path.display(),
                destination.display()
            )
        })?;
    }
    let getting_started = destination.join("getting-started.md");
    if getting_started.exists() {
        std::fs::copy(&getting_started, destination.join("index.md"))
            .context("failed to write index.md markdown alias")?;
    }
    Ok(())
}

fn markdown_source_contents(contents: &str) -> String {
    front_matter_comment_regex()
        .replace(contents, "")
        .trim_start()
        .to_string()
}

fn docs_page_last_updated_from_git(docs_root: &Path) -> HashMap<String, String> {
    let output = git_log_last_updated(docs_root).ok();
    let Some(output) = output else {
        return HashMap::default();
    };
    if !output.status.success() {
        return HashMap::default();
    }
    let Ok(output) = String::from_utf8(output.stdout) else {
        return HashMap::default();
    };

    let mut last_updated_by_path = HashMap::new();
    let mut current_date = None;
    for line in output.lines() {
        if let Some(date) = line.strip_prefix("--") {
            current_date = Some(date.to_string());
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let Some(date) = current_date.as_ref() else {
            continue;
        };
        let Some(source_path) = line.strip_prefix("src/") else {
            continue;
        };
        last_updated_by_path
            .entry(source_path.to_string())
            .or_insert_with(|| date.clone());
    }
    last_updated_by_path
}

#[allow(clippy::disallowed_methods)]
fn git_log_last_updated(docs_root: &Path) -> std::io::Result<std::process::Output> {
    std::process::Command::new("git")
        .current_dir(docs_root)
        .args(["log", "--format=--%cs", "--name-only", "--", "src"])
        .output()
}

fn docs_page_last_updated_fallbacks(docs_root: &Path) -> Result<HashMap<String, String>> {
    let path = docs_root.join("last-updated.json");
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Ok(HashMap::default());
    };
    serde_json::from_str(&contents).with_context(|| format!("failed to parse {}", path.display()))
}

fn docs_page_description(contents: &str) -> Option<String> {
    docs_page_metadata(contents).and_then(|metadata| {
        metadata
            .get("description")
            .map(|description| {
                description
                    .trim()
                    .trim_matches('"')
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|description| !description.is_empty())
    })
}

fn docs_page_metadata(contents: &str) -> Option<HashMap<String, String>> {
    let captures = front_matter_comment_regex().captures(contents)?;
    serde_json::from_str(&captures[1]).ok()
}

fn front_matter_comment_regex() -> &'static Regex {
    static FRONT_MATTER_COMMENT_REGEX: OnceLock<Regex> = OnceLock::new();
    FRONT_MATTER_COMMENT_REGEX
        .get_or_init(|| Regex::new(&FRONT_MATTER_COMMENT.replace("{}", "([^\\n]*)")).unwrap())
}

fn write_llms_txt(destination: &Path, site_url: &str, pages: &[DocsPage]) -> Result<()> {
    let mut contents = String::new();
    contents.push_str("# Zed Docs\n\n");
    contents.push_str(
        "> Official Zed documentation index with links to Markdown versions of each docs page.\n\n",
    );
    contents.push_str(
        "Use these links for concise Markdown copies of Zed documentation pages. Each linked page mirrors the corresponding `/docs/*.html` page without site navigation or styling.\n\n",
    );
    let mut current_section = None;
    for page in pages {
        if current_section != Some(page.section.as_str()) {
            if current_section.is_some() {
                contents.push('\n');
            }
            contents.push_str("## ");
            contents.push_str(&markdown_text(&page.section));
            contents.push_str("\n\n");
            current_section = Some(page.section.as_str());
        }
        contents.push_str("- [");
        contents.push_str(&markdown_text(&page.title));
        contents.push_str("](");
        contents.push_str(&absolute_docs_url(site_url, &page.source_path));
        contents.push(')');
        if let Some(description) = &page.description {
            contents.push_str(": ");
            contents.push_str(&markdown_text(description));
        }
        if let Some(last_updated) = &page.last_updated {
            contents.push_str(" (Last updated: ");
            contents.push_str(last_updated);
            contents.push(')');
        }
        contents.push('\n');
    }
    std::fs::write(destination.join("llms.txt"), contents).context("failed to write llms.txt")?;
    Ok(())
}

fn markdown_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

fn write_sitemap_xml(destination: &Path, site_url: &str, pages: &[DocsPage]) -> Result<()> {
    let mut contents = String::new();
    contents.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    contents.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for page in pages {
        contents.push_str("  <url><loc>");
        contents.push_str(&xml_escape(&absolute_docs_url(
            site_url,
            &page.source_path.with_extension("html"),
        )));
        contents.push_str("</loc>");
        if let Some(last_updated) = &page.last_updated {
            contents.push_str("<lastmod>");
            contents.push_str(&xml_escape(last_updated));
            contents.push_str("</lastmod>");
        }
        contents.push_str("</url>\n");
    }
    contents.push_str("</urlset>\n");
    std::fs::write(destination.join("sitemap.xml"), contents)
        .context("failed to write sitemap.xml")?;
    Ok(())
}

pub(crate) fn write_pages_redirects(
    destination: &Path,
    redirects: &[(String, String)],
    site_url: &str,
) -> Result<()> {
    let Some(deploy_root) = destination.parent() else {
        return Ok(());
    };
    let mut contents = String::new();
    for (source, destination) in redirects {
        write_redirect_line(
            &mut contents,
            &docs_path(site_url, source),
            &redirect_destination(site_url, destination),
        );
        if let Some(extensionless_source) = strip_html_suffix(source) {
            write_redirect_line(
                &mut contents,
                &docs_path(site_url, &extensionless_source),
                &redirect_destination(
                    site_url,
                    &strip_html_suffix(destination).unwrap_or_else(|| destination.to_string()),
                ),
            );
        }
        if let Some(markdown_source) = html_path_to_markdown(source) {
            if let Some(markdown_destination) = html_path_to_markdown(destination) {
                write_redirect_line(
                    &mut contents,
                    &docs_path(site_url, &markdown_source),
                    &redirect_destination(site_url, &markdown_destination),
                );
            }
        }
    }
    std::fs::write(deploy_root.join("_redirects"), contents)
        .context("failed to write Cloudflare Pages _redirects")?;
    Ok(())
}

pub(crate) fn write_markdown_redirect_aliases(
    destination: &Path,
    redirects: &[(String, String)],
    site_url: &str,
) -> Result<()> {
    for (source, redirect_destination_path) in redirects {
        let Some(source_markdown) = html_path_to_markdown(source) else {
            continue;
        };
        let Some(destination_markdown) = html_path_to_markdown(redirect_destination_path) else {
            continue;
        };
        let source_markdown = destination.join(source_markdown.trim_start_matches('/'));
        let destination_markdown =
            destination.join(destination_markdown.trim_start_matches("/docs/"));
        if !destination_markdown.exists() {
            continue;
        }
        if let Some(parent) = source_markdown.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create markdown alias directory {}",
                    parent.display()
                )
            })?;
        }
        let contents = format!(
            "# Moved\n\n> For the complete documentation index and Markdown links, see [llms.txt]({}).\n\nThis page moved to [the current docs page]({}).\n",
            docs_url(site_url, Path::new("llms.txt")),
            html_path_to_markdown(redirect_destination_path)
                .map(|path| redirect_destination(site_url, &path))
                .unwrap_or_else(|| redirect_destination(site_url, redirect_destination_path))
        );
        std::fs::write(&source_markdown, contents).with_context(|| {
            format!(
                "failed to write markdown redirect alias from {} to {}",
                redirect_destination_path,
                source_markdown.display()
            )
        })?;
    }
    Ok(())
}

fn write_redirect_line(contents: &mut String, source: &str, destination: &str) {
    contents.push_str(source);
    contents.push(' ');
    contents.push_str(destination);
    contents.push_str(" 301\n");
}

fn docs_path(site_url: &str, path: &str) -> String {
    docs_url(site_url, Path::new(path.trim_start_matches('/')))
}

fn redirect_destination(site_url: &str, destination: &str) -> String {
    if let Some(path) = destination.strip_prefix("/docs/") {
        docs_url(site_url, Path::new(path))
    } else if destination == "/docs" {
        docs_url(site_url, Path::new(""))
    } else {
        destination.to_string()
    }
}

fn strip_html_suffix(path: &str) -> Option<String> {
    let (path, fragment) = split_fragment(path);
    let path = path.strip_suffix(".html")?;
    Some(format!("{path}{fragment}"))
}

fn html_path_to_markdown(path: &str) -> Option<String> {
    let (path, fragment) = split_fragment(path);
    if !path.starts_with("/docs/") && path != "/docs" && !path.ends_with(".html") {
        return None;
    }
    let markdown_path = path.strip_suffix(".html").unwrap_or(path);
    Some(format!("{markdown_path}.md{fragment}"))
}

fn split_fragment(path: &str) -> (&str, &str) {
    match path.find('#') {
        Some(index) => (&path[..index], &path[index..]),
        None => (path, ""),
    }
}

pub(crate) fn add_markdown_alternate_link(
    contents: &str,
    html_file: &Path,
    root_dir: &Path,
    site_url: &str,
) -> String {
    let Ok(relative_path) = html_file.strip_prefix(root_dir) else {
        return contents.to_string();
    };
    let markdown_path = relative_path.with_extension("md");
    if !root_dir.join(&markdown_path).exists() {
        return contents.to_string();
    }
    let markdown_url = docs_url(site_url, &markdown_path);
    let link = format!(
        "        <link rel=\"alternate\" type=\"text/markdown\" href=\"{}\">\n",
        markdown_url
    );
    contents.replacen("</head>", &(link + "    </head>"), 1)
}

fn add_llms_markdown_directive(
    contents: &str,
    site_url: &str,
    last_updated: Option<&str>,
) -> String {
    let directive = format!(
        "> For the complete documentation index and Markdown links, see [llms.txt]({}).{}\n\n",
        docs_url(site_url, Path::new("llms.txt")),
        last_updated
            .map(|last_updated| format!(" Last updated: {last_updated}."))
            .unwrap_or_default()
    );
    if let Some(rest) = contents.strip_prefix("---\n") {
        if let Some(frontmatter_end) = rest.find("\n---\n") {
            let split_at = "---\n".len() + frontmatter_end + "\n---\n".len();
            let mut output = String::with_capacity(contents.len() + directive.len());
            output.push_str(&contents[..split_at]);
            output.push('\n');
            output.push_str(&directive);
            output.push_str(&contents[split_at..]);
            return output;
        }
    }

    let mut output = String::with_capacity(contents.len() + directive.len());
    output.push_str(&directive);
    output.push_str(contents);
    output
}

pub(crate) fn add_last_updated_meta(contents: &str, last_updated: Option<&str>) -> String {
    let Some(last_updated) = last_updated else {
        return contents.to_string();
    };
    let meta = format!(
        "        <meta name=\"last-modified\" content=\"{}\">\n        <meta property=\"article:modified_time\" content=\"{}\">\n",
        xml_escape(last_updated),
        xml_escape(last_updated),
    );
    contents.replacen("</head>", &(meta + "    </head>"), 1)
}

fn docs_url(site_url: &str, path: &Path) -> String {
    let mut url = site_url.to_string();
    if !url.ends_with('/') {
        url.push('/');
    }
    url.push_str(&path.to_string_lossy().replace('\\', "/"));
    url
}

fn absolute_docs_url(site_url: &str, path: &Path) -> String {
    let url = docs_url(site_url, path);
    if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("https://zed.dev{}", url)
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_llms_markdown_directive_includes_last_updated_after_frontmatter() {
        let contents = "---\ntitle: Example\n---\n# Example\n";
        let output = add_llms_markdown_directive(contents, "/docs/", Some("2026-06-19"));

        assert!(output.starts_with("---\ntitle: Example\n---\n\n"));
        assert!(output.contains(
            "> For the complete documentation index and Markdown links, see [llms.txt](/docs/llms.txt). Last updated: 2026-06-19."
        ));
    }

    #[test]
    fn test_add_last_updated_meta_inserts_machine_readable_dates() {
        let output = add_last_updated_meta(
            "<html><head></head><body></body></html>",
            Some("2026-06-19"),
        );

        assert!(output.contains("<meta name=\"last-modified\" content=\"2026-06-19\">"));
        assert!(
            output.contains("<meta property=\"article:modified_time\" content=\"2026-06-19\">")
        );
    }

    #[test]
    fn test_redirect_destination_uses_channel_site_url_for_docs_paths() {
        assert_eq!(
            redirect_destination("/docs/preview/", "/docs/ai/overview.html"),
            "/docs/preview/ai/overview.html"
        );
        assert_eq!(
            redirect_destination("/docs/preview/", "/community-links"),
            "/community-links"
        );
    }

    #[test]
    fn test_docs_path_uses_channel_site_url() {
        assert_eq!(
            docs_path("/docs/preview/", "/assistant.md"),
            "/docs/preview/assistant.md"
        );
    }
}

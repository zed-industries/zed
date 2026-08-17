use crate::book::{Book, BookItem};
use crate::config::{BookConfig, Code, Config, HtmlConfig, Playground, RustEdition};
use crate::errors::*;
use crate::renderer::html_handlebars::helpers;
use crate::renderer::{RenderContext, Renderer};
use crate::theme::{self, playground_editor, Theme};
use crate::utils;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::utils::fs::get_404_output_file;
use handlebars::Handlebars;
use log::{debug, trace, warn};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde_json::json;

#[derive(Default)]
pub struct HtmlHandlebars;

impl HtmlHandlebars {
    pub fn new() -> Self {
        HtmlHandlebars
    }

    fn render_item(
        &self,
        item: &BookItem,
        mut ctx: RenderItemContext<'_>,
        print_content: &mut String,
    ) -> Result<()> {
        // FIXME: This should be made DRY-er and rely less on mutable state

        let (ch, path) = match item {
            BookItem::Chapter(ch) if !ch.is_draft_chapter() => (ch, ch.path.as_ref().unwrap()),
            _ => return Ok(()),
        };

        if let Some(ref edit_url_template) = ctx.html_config.edit_url_template {
            let full_path = ctx.book_config.src.to_str().unwrap_or_default().to_owned()
                + "/"
                + ch.source_path
                    .clone()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();

            let edit_url = edit_url_template.replace("{path}", &full_path);
            ctx.data
                .insert("git_repository_edit_url".to_owned(), json!(edit_url));
        }

        let content = utils::render_markdown(&ch.content, ctx.html_config.smart_punctuation());

        let fixed_content = utils::render_markdown_with_path(
            &ch.content,
            ctx.html_config.smart_punctuation(),
            Some(path),
        );
        if !ctx.is_index && ctx.html_config.print.page_break {
            // Add page break between chapters
            // See https://developer.mozilla.org/en-US/docs/Web/CSS/break-before and https://developer.mozilla.org/en-US/docs/Web/CSS/page-break-before
            // Add both two CSS properties because of the compatibility issue
            print_content
                .push_str(r#"<div style="break-before: page; page-break-before: always;"></div>"#);
        }
        print_content.push_str(&fixed_content);

        // Update the context with data for this file
        let ctx_path = path
            .to_str()
            .with_context(|| "Could not convert path to str")?;
        let filepath = Path::new(&ctx_path).with_extension("html");

        // "print.html" is used for the print page.
        if path == Path::new("print.md") {
            bail!("{} is reserved for internal use", path.display());
        };

        let book_title = ctx
            .data
            .get("book_title")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");

        let title = if let Some(title) = ctx.chapter_titles.get(path) {
            title.clone()
        } else if book_title.is_empty() {
            ch.name.clone()
        } else {
            ch.name.clone() + " - " + book_title
        };

        ctx.data.insert("path".to_owned(), json!(path));
        ctx.data.insert("content".to_owned(), json!(content));
        ctx.data.insert("chapter_title".to_owned(), json!(ch.name));
        ctx.data.insert("title".to_owned(), json!(title));
        ctx.data.insert(
            "path_to_root".to_owned(),
            json!(utils::fs::path_to_root(path)),
        );
        if let Some(ref section) = ch.number {
            ctx.data
                .insert("section".to_owned(), json!(section.to_string()));
        }

        // Render the handlebars template with the data
        debug!("Render template");
        let rendered = ctx.handlebars.render("index", &ctx.data)?;

        let rendered = self.post_process(
            rendered,
            &ctx.html_config.playground,
            &ctx.html_config.code,
            ctx.edition,
        );

        // Write to file
        debug!("Creating {}", filepath.display());
        utils::fs::write_file(&ctx.destination, &filepath, rendered.as_bytes())?;

        if ctx.is_index {
            ctx.data.insert("path".to_owned(), json!("index.md"));
            ctx.data.insert("path_to_root".to_owned(), json!(""));
            ctx.data.insert("is_index".to_owned(), json!(true));
            let rendered_index = ctx.handlebars.render("index", &ctx.data)?;
            let rendered_index = self.post_process(
                rendered_index,
                &ctx.html_config.playground,
                &ctx.html_config.code,
                ctx.edition,
            );
            debug!("Creating index.html from {}", ctx_path);
            utils::fs::write_file(&ctx.destination, "index.html", rendered_index.as_bytes())?;
        }

        Ok(())
    }

    fn render_404(
        &self,
        ctx: &RenderContext,
        html_config: &HtmlConfig,
        src_dir: &Path,
        handlebars: &mut Handlebars<'_>,
        data: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<()> {
        let destination = &ctx.destination;
        let content_404 = if let Some(ref filename) = html_config.input_404 {
            let path = src_dir.join(filename);
            std::fs::read_to_string(&path)
                .with_context(|| format!("unable to open 404 input file {:?}", path))?
        } else {
            // 404 input not explicitly configured try the default file 404.md
            let default_404_location = src_dir.join("404.md");
            if default_404_location.exists() {
                std::fs::read_to_string(&default_404_location).with_context(|| {
                    format!("unable to open 404 input file {:?}", default_404_location)
                })?
            } else {
                "# Document not found (404)\n\nThis URL is invalid, sorry. Please use the \
                navigation bar or search to continue."
                    .to_string()
            }
        };
        let html_content_404 =
            utils::render_markdown(&content_404, html_config.smart_punctuation());

        let mut data_404 = data.clone();
        let base_url = if let Some(site_url) = &html_config.site_url {
            site_url
        } else {
            debug!(
                "HTML 'site-url' parameter not set, defaulting to '/'. Please configure \
                this to ensure the 404 page work correctly, especially if your site is hosted in a \
                subdirectory on the HTTP server."
            );
            "/"
        };
        data_404.insert("base_url".to_owned(), json!(base_url));
        // Set a dummy path to ensure other paths (e.g. in the TOC) are generated correctly
        data_404.insert("path".to_owned(), json!("404.md"));
        data_404.insert("content".to_owned(), json!(html_content_404));

        let mut title = String::from("Page not found");
        if let Some(book_title) = &ctx.config.book.title {
            title.push_str(" - ");
            title.push_str(book_title);
        }
        data_404.insert("title".to_owned(), json!(title));
        let rendered = handlebars.render("index", &data_404)?;

        let rendered = self.post_process(
            rendered,
            &html_config.playground,
            &html_config.code,
            ctx.config.rust.edition,
        );
        let output_file = get_404_output_file(&html_config.input_404);
        utils::fs::write_file(destination, output_file, rendered.as_bytes())?;
        debug!("Creating 404.html ✓");
        Ok(())
    }

    #[allow(clippy::let_and_return)]
    fn post_process(
        &self,
        rendered: String,
        playground_config: &Playground,
        code_config: &Code,
        edition: Option<RustEdition>,
    ) -> String {
        let rendered = build_header_links(&rendered);
        let rendered = fix_code_blocks(&rendered);
        let rendered = add_playground_pre(&rendered, playground_config, edition);
        let rendered = hide_lines(&rendered, code_config);

        rendered
    }

    fn copy_static_files(
        &self,
        destination: &Path,
        theme: &Theme,
        html_config: &HtmlConfig,
    ) -> Result<()> {
        use crate::utils::fs::write_file;

        write_file(
            destination,
            ".nojekyll",
            b"This file makes sure that Github Pages doesn't process mdBook's output.\n",
        )?;

        if let Some(cname) = &html_config.cname {
            write_file(destination, "CNAME", format!("{}\n", cname).as_bytes())?;
        }

        write_file(destination, "book.js", &theme.js)?;
        write_file(destination, "css/general.css", &theme.general_css)?;
        write_file(destination, "css/chrome.css", &theme.chrome_css)?;
        if html_config.print.enable {
            write_file(destination, "css/print.css", &theme.print_css)?;
        }
        write_file(destination, "css/variables.css", &theme.variables_css)?;
        if let Some(contents) = &theme.favicon_png {
            write_file(destination, "favicon.png", contents)?;
        }
        if let Some(contents) = &theme.favicon_svg {
            write_file(destination, "favicon.svg", contents)?;
        }
        write_file(destination, "highlight.css", &theme.highlight_css)?;
        write_file(destination, "tomorrow-night.css", &theme.tomorrow_night_css)?;
        write_file(destination, "ayu-highlight.css", &theme.ayu_highlight_css)?;
        write_file(destination, "highlight.js", &theme.highlight_js)?;
        write_file(destination, "clipboard.min.js", &theme.clipboard_js)?;
        write_file(
            destination,
            "FontAwesome/css/font-awesome.css",
            theme::FONT_AWESOME,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.eot",
            theme::FONT_AWESOME_EOT,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.svg",
            theme::FONT_AWESOME_SVG,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.ttf",
            theme::FONT_AWESOME_TTF,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.woff",
            theme::FONT_AWESOME_WOFF,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.woff2",
            theme::FONT_AWESOME_WOFF2,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/FontAwesome.ttf",
            theme::FONT_AWESOME_TTF,
        )?;
        // Don't copy the stock fonts if the user has specified their own fonts to use.
        if html_config.copy_fonts && theme.fonts_css.is_none() {
            write_file(destination, "fonts/fonts.css", theme::fonts::CSS)?;
            for (file_name, contents) in theme::fonts::LICENSES.iter() {
                write_file(destination, file_name, contents)?;
            }
            for (file_name, contents) in theme::fonts::OPEN_SANS.iter() {
                write_file(destination, file_name, contents)?;
            }
            write_file(
                destination,
                theme::fonts::SOURCE_CODE_PRO.0,
                theme::fonts::SOURCE_CODE_PRO.1,
            )?;
        }
        if let Some(fonts_css) = &theme.fonts_css {
            if !fonts_css.is_empty() {
                write_file(destination, "fonts/fonts.css", fonts_css)?;
            }
        }
        if !html_config.copy_fonts && theme.fonts_css.is_none() {
            warn!(
                "output.html.copy-fonts is deprecated.\n\
                This book appears to have copy-fonts=false in book.toml without a fonts.css file.\n\
                Add an empty `theme/fonts/fonts.css` file to squelch this warning."
            );
        }
        for font_file in &theme.font_files {
            let contents = fs::read(font_file)?;
            let filename = font_file.file_name().unwrap();
            let filename = Path::new("fonts").join(filename);
            write_file(destination, filename, &contents)?;
        }

        let playground_config = &html_config.playground;

        // Ace is a very large dependency, so only load it when requested
        if playground_config.editable && playground_config.copy_js {
            // Load the editor
            write_file(destination, "editor.js", playground_editor::JS)?;
            write_file(destination, "ace.js", playground_editor::ACE_JS)?;
            write_file(destination, "mode-rust.js", playground_editor::MODE_RUST_JS)?;
            write_file(
                destination,
                "theme-dawn.js",
                playground_editor::THEME_DAWN_JS,
            )?;
            write_file(
                destination,
                "theme-tomorrow_night.js",
                playground_editor::THEME_TOMORROW_NIGHT_JS,
            )?;
        }

        Ok(())
    }

    /// Update the context with data for this file
    fn configure_print_version(
        &self,
        data: &mut serde_json::Map<String, serde_json::Value>,
        print_content: &str,
    ) {
        // Make sure that the Print chapter does not display the title from
        // the last rendered chapter by removing it from its context
        data.remove("title");
        data.insert("is_print".to_owned(), json!(true));
        data.insert("path".to_owned(), json!("print.md"));
        data.insert("content".to_owned(), json!(print_content));
        data.insert(
            "path_to_root".to_owned(),
            json!(utils::fs::path_to_root(Path::new("print.md"))),
        );
    }

    fn register_hbs_helpers(&self, handlebars: &mut Handlebars<'_>, html_config: &HtmlConfig) {
        handlebars.register_helper(
            "toc",
            Box::new(helpers::toc::RenderToc {
                no_section_label: html_config.no_section_label,
            }),
        );
        handlebars.register_helper("previous", Box::new(helpers::navigation::previous));
        handlebars.register_helper("next", Box::new(helpers::navigation::next));
        // TODO: remove theme_option in 0.5, it is not needed.
        handlebars.register_helper("theme_option", Box::new(helpers::theme::theme_option));
    }

    /// Copy across any additional CSS and JavaScript files which the book
    /// has been configured to use.
    fn copy_additional_css_and_js(
        &self,
        html: &HtmlConfig,
        root: &Path,
        destination: &Path,
    ) -> Result<()> {
        let custom_files = html.additional_css.iter().chain(html.additional_js.iter());

        debug!("Copying additional CSS and JS");

        for custom_file in custom_files {
            let input_location = root.join(custom_file);
            let output_location = destination.join(custom_file);
            if let Some(parent) = output_location.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Unable to create {}", parent.display()))?;
            }
            debug!(
                "Copying {} -> {}",
                input_location.display(),
                output_location.display()
            );

            fs::copy(&input_location, &output_location).with_context(|| {
                format!(
                    "Unable to copy {} to {}",
                    input_location.display(),
                    output_location.display()
                )
            })?;
        }

        Ok(())
    }

    fn emit_redirects(
        &self,
        root: &Path,
        handlebars: &Handlebars<'_>,
        redirects: &HashMap<String, String>,
    ) -> Result<()> {
        if redirects.is_empty() {
            return Ok(());
        }

        log::debug!("Emitting redirects");

        for (original, new) in redirects {
            log::debug!("Redirecting \"{}\" → \"{}\"", original, new);
            // Note: all paths are relative to the build directory, so the
            // leading slash in an absolute path means nothing (and would mess
            // up `root.join(original)`).
            let original = original.trim_start_matches('/');
            let filename = root.join(original);
            self.emit_redirect(handlebars, &filename, new)?;
        }

        Ok(())
    }

    fn emit_redirect(
        &self,
        handlebars: &Handlebars<'_>,
        original: &Path,
        destination: &str,
    ) -> Result<()> {
        if original.exists() {
            // sanity check to avoid accidentally overwriting a real file.
            let msg = format!(
                "Not redirecting \"{}\" to \"{}\" because it already exists. Are you sure it needs to be redirected?",
                original.display(),
                destination,
            );
            return Err(Error::msg(msg));
        }

        if let Some(parent) = original.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Unable to ensure \"{}\" exists", parent.display()))?;
        }

        let ctx = json!({
            "url": destination,
        });
        let f = File::create(original)?;
        handlebars
            .render_to_write("redirect", &ctx, f)
            .with_context(|| {
                format!(
                    "Unable to create a redirect file at \"{}\"",
                    original.display()
                )
            })?;

        Ok(())
    }
}

impl Renderer for HtmlHandlebars {
    fn name(&self) -> &str {
        "html"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let book_config = &ctx.config.book;
        let html_config = ctx.config.html_config().unwrap_or_default();
        let src_dir = ctx.root.join(&ctx.config.book.src);
        let destination = &ctx.destination;
        let book = &ctx.book;
        let build_dir = ctx.root.join(&ctx.config.build.build_dir);

        if destination.exists() {
            utils::fs::remove_dir_content(destination)
                .with_context(|| "Unable to remove stale HTML output")?;
        }

        trace!("render");
        let mut handlebars = Handlebars::new();

        let theme_dir = match html_config.theme {
            Some(ref theme) => {
                let dir = ctx.root.join(theme);
                if !dir.is_dir() {
                    bail!("theme dir {} does not exist", dir.display());
                }
                dir
            }
            None => ctx.root.join("theme"),
        };

        let theme = theme::Theme::new(theme_dir);

        debug!("Register the index handlebars template");
        handlebars.register_template_string("index", String::from_utf8(theme.index.clone())?)?;

        debug!("Register the head handlebars template");
        handlebars.register_partial("head", String::from_utf8(theme.head.clone())?)?;

        debug!("Register the redirect handlebars template");
        handlebars
            .register_template_string("redirect", String::from_utf8(theme.redirect.clone())?)?;

        debug!("Register the header handlebars template");
        handlebars.register_partial("header", String::from_utf8(theme.header.clone())?)?;

        debug!("Register handlebars helpers");
        self.register_hbs_helpers(&mut handlebars, &html_config);

        let mut data = make_data(&ctx.root, book, &ctx.config, &html_config, &theme)?;

        // Print version
        let mut print_content = String::new();

        fs::create_dir_all(destination)
            .with_context(|| "Unexpected error when constructing destination path")?;

        let mut is_index = true;
        for item in book.iter() {
            let ctx = RenderItemContext {
                handlebars: &handlebars,
                destination: destination.to_path_buf(),
                data: data.clone(),
                is_index,
                book_config: book_config.clone(),
                html_config: html_config.clone(),
                edition: ctx.config.rust.edition,
                chapter_titles: &ctx.chapter_titles,
            };
            self.render_item(item, ctx, &mut print_content)?;
            // Only the first non-draft chapter item should be treated as the "index"
            is_index &= !matches!(item, BookItem::Chapter(ch) if !ch.is_draft_chapter());
        }

        // Render 404 page
        if html_config.input_404 != Some("".to_string()) {
            self.render_404(ctx, &html_config, &src_dir, &mut handlebars, &mut data)?;
        }

        // Print version
        self.configure_print_version(&mut data, &print_content);
        if let Some(ref title) = ctx.config.book.title {
            data.insert("title".to_owned(), json!(title));
        }

        // Render the handlebars template with the data
        if html_config.print.enable {
            debug!("Render template");
            let rendered = handlebars.render("index", &data)?;

            let rendered = self.post_process(
                rendered,
                &html_config.playground,
                &html_config.code,
                ctx.config.rust.edition,
            );

            utils::fs::write_file(destination, "print.html", rendered.as_bytes())?;
            debug!("Creating print.html ✓");
        }

        debug!("Copy static files");
        self.copy_static_files(destination, &theme, &html_config)
            .with_context(|| "Unable to copy across static files")?;
        self.copy_additional_css_and_js(&html_config, &ctx.root, destination)
            .with_context(|| "Unable to copy across additional CSS and JS")?;

        // Render search index
        #[cfg(feature = "search")]
        {
            let search = html_config.search.unwrap_or_default();
            if search.enable {
                super::search::create_files(&search, destination, book)?;
            }
        }

        self.emit_redirects(&ctx.destination, &handlebars, &html_config.redirect)
            .context("Unable to emit redirects")?;

        // Copy all remaining files, avoid a recursive copy from/to the book build dir
        utils::fs::copy_files_except_ext(&src_dir, destination, true, Some(&build_dir), &["md"])?;

        Ok(())
    }
}

fn make_data(
    root: &Path,
    book: &Book,
    config: &Config,
    html_config: &HtmlConfig,
    theme: &Theme,
) -> Result<serde_json::Map<String, serde_json::Value>> {
    trace!("make_data");

    let mut data = serde_json::Map::new();
    data.insert(
        "language".to_owned(),
        json!(config.book.language.clone().unwrap_or_default()),
    );
    data.insert(
        "text_direction".to_owned(),
        json!(config.book.realized_text_direction()),
    );
    data.insert(
        "book_title".to_owned(),
        json!(config.book.title.clone().unwrap_or_default()),
    );
    data.insert(
        "description".to_owned(),
        json!(config.book.description.clone().unwrap_or_default()),
    );
    if theme.favicon_png.is_some() {
        data.insert("favicon_png".to_owned(), json!("favicon.png"));
    }
    if theme.favicon_svg.is_some() {
        data.insert("favicon_svg".to_owned(), json!("favicon.svg"));
    }
    if let Some(ref live_reload_endpoint) = html_config.live_reload_endpoint {
        data.insert(
            "live_reload_endpoint".to_owned(),
            json!(live_reload_endpoint),
        );
    }

    // TODO: remove default_theme in 0.5, it is not needed.
    let default_theme = match html_config.default_theme {
        Some(ref theme) => theme.to_lowercase(),
        None => "light".to_string(),
    };
    data.insert("default_theme".to_owned(), json!(default_theme));

    let preferred_dark_theme = match html_config.preferred_dark_theme {
        Some(ref theme) => theme.to_lowercase(),
        None => "navy".to_string(),
    };
    data.insert(
        "preferred_dark_theme".to_owned(),
        json!(preferred_dark_theme),
    );

    // Add google analytics tag
    if let Some(ref ga) = html_config.google_analytics {
        data.insert("google_analytics".to_owned(), json!(ga));
    }

    if html_config.mathjax_support {
        data.insert("mathjax_support".to_owned(), json!(true));
    }

    // This `matches!` checks for a non-empty file.
    if html_config.copy_fonts || matches!(theme.fonts_css.as_deref(), Some([_, ..])) {
        data.insert("copy_fonts".to_owned(), json!(true));
    }

    // Add check to see if there is an additional style
    if !html_config.additional_css.is_empty() {
        let mut css = Vec::new();
        for style in &html_config.additional_css {
            match style.strip_prefix(root) {
                Ok(p) => css.push(p.to_str().expect("Could not convert to str")),
                Err(_) => css.push(style.to_str().expect("Could not convert to str")),
            }
        }
        data.insert("additional_css".to_owned(), json!(css));
    }

    // Add check to see if there is an additional script
    if !html_config.additional_js.is_empty() {
        let mut js = Vec::new();
        for script in &html_config.additional_js {
            match script.strip_prefix(root) {
                Ok(p) => js.push(p.to_str().expect("Could not convert to str")),
                Err(_) => js.push(script.to_str().expect("Could not convert to str")),
            }
        }
        data.insert("additional_js".to_owned(), json!(js));
    }

    if html_config.playground.editable && html_config.playground.copy_js {
        data.insert("playground_js".to_owned(), json!(true));
        if html_config.playground.line_numbers {
            data.insert("playground_line_numbers".to_owned(), json!(true));
        }
    }
    if html_config.playground.copyable {
        data.insert("playground_copyable".to_owned(), json!(true));
    }

    data.insert("print_enable".to_owned(), json!(html_config.print.enable));
    data.insert("fold_enable".to_owned(), json!(html_config.fold.enable));
    data.insert("fold_level".to_owned(), json!(html_config.fold.level));

    let search = html_config.search.clone();
    if cfg!(feature = "search") {
        let search = search.unwrap_or_default();
        data.insert("search_enabled".to_owned(), json!(search.enable));
        data.insert(
            "search_js".to_owned(),
            json!(search.enable && search.copy_js),
        );
    } else if search.is_some() {
        warn!("mdBook compiled without search support, ignoring `output.html.search` table");
        warn!(
            "please reinstall with `cargo install mdbook --force --features search`to use the \
             search feature"
        )
    }

    if let Some(ref git_repository_url) = html_config.git_repository_url {
        data.insert("git_repository_url".to_owned(), json!(git_repository_url));
    }

    let git_repository_icon = match html_config.git_repository_icon {
        Some(ref git_repository_icon) => git_repository_icon,
        None => "fa-github",
    };
    data.insert("git_repository_icon".to_owned(), json!(git_repository_icon));

    let mut chapters = vec![];

    for item in book.iter() {
        // Create the data to inject in the template
        let mut chapter = BTreeMap::new();

        match *item {
            BookItem::PartTitle(ref title) => {
                chapter.insert("part".to_owned(), json!(title));
            }
            BookItem::Chapter(ref ch) => {
                if let Some(ref section) = ch.number {
                    chapter.insert("section".to_owned(), json!(section.to_string()));
                }

                chapter.insert(
                    "has_sub_items".to_owned(),
                    json!((!ch.sub_items.is_empty()).to_string()),
                );

                chapter.insert("name".to_owned(), json!(ch.name));
                if let Some(ref path) = ch.path {
                    let p = path
                        .to_str()
                        .with_context(|| "Could not convert path to str")?;
                    chapter.insert("path".to_owned(), json!(p));
                }
            }
            BookItem::Separator => {
                chapter.insert("spacer".to_owned(), json!("_spacer_"));
            }
        }

        chapters.push(chapter);
    }

    data.insert("chapters".to_owned(), json!(chapters));

    debug!("[*]: JSON constructed");
    Ok(data)
}

/// Goes through the rendered HTML, making sure all header tags have
/// an anchor respectively so people can link to sections directly.
fn build_header_links(html: &str) -> String {
    static BUILD_HEADER_LINKS: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"<h(\d)(?: id="([^"]+)")?(?: class="([^"]+)")?>(.*?)</h\d>"#).unwrap()
    });
    static IGNORE_CLASS: &[&str] = &["menu-title"];

    let mut id_counter = HashMap::new();

    BUILD_HEADER_LINKS
        .replace_all(html, |caps: &Captures<'_>| {
            let level = caps[1]
                .parse()
                .expect("Regex should ensure we only ever get numbers here");

            // Ignore .menu-title because now it's getting detected by the regex.
            if let Some(classes) = caps.get(3) {
                for class in classes.as_str().split(" ") {
                    if IGNORE_CLASS.contains(&class) {
                        return caps[0].to_string();
                    }
                }
            }

            insert_link_into_header(
                level,
                &caps[4],
                caps.get(2).map(|x| x.as_str().to_string()),
                caps.get(3).map(|x| x.as_str().to_string()),
                &mut id_counter,
            )
        })
        .into_owned()
}

/// Insert a sinle link into a header, making sure each link gets its own
/// unique ID by appending an auto-incremented number (if necessary).
fn insert_link_into_header(
    level: usize,
    content: &str,
    id: Option<String>,
    classes: Option<String>,
    id_counter: &mut HashMap<String, usize>,
) -> String {
    let id = id.unwrap_or_else(|| utils::unique_id_from_content(content, id_counter));
    let classes = classes
        .map(|s| format!(" class=\"{s}\""))
        .unwrap_or_default();

    format!(
        r##"<h{level} id="{id}"{classes}><a class="header" href="#{id}">{text}</a></h{level}>"##,
        level = level,
        id = id,
        text = content,
        classes = classes
    )
}

// The rust book uses annotations for rustdoc to test code snippets,
// like the following:
// ```rust,should_panic
// fn main() {
//     // Code here
// }
// ```
// This function replaces all commas by spaces in the code block classes
fn fix_code_blocks(html: &str) -> String {
    static FIX_CODE_BLOCKS: Lazy<Regex> =
        Lazy::new(|| Regex::new(r##"<code([^>]+)class="([^"]+)"([^>]*)>"##).unwrap());

    FIX_CODE_BLOCKS
        .replace_all(html, |caps: &Captures<'_>| {
            let before = &caps[1];
            let classes = &caps[2].replace(',', " ");
            let after = &caps[3];

            format!(
                r#"<code{before}class="{classes}"{after}>"#,
                before = before,
                classes = classes,
                after = after
            )
        })
        .into_owned()
}

static CODE_BLOCK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r##"((?s)<code[^>]?class="([^"]+)".*?>(.*?)</code>)"##).unwrap());

fn add_playground_pre(
    html: &str,
    playground_config: &Playground,
    edition: Option<RustEdition>,
) -> String {
    CODE_BLOCK_RE
        .replace_all(html, |caps: &Captures<'_>| {
            let text = &caps[1];
            let classes = &caps[2];
            let code = &caps[3];

            if classes.contains("language-rust")
                && ((!classes.contains("ignore")
                    && !classes.contains("noplayground")
                    && !classes.contains("noplaypen")
                    && playground_config.runnable)
                    || classes.contains("mdbook-runnable"))
            {
                let contains_e2015 = classes.contains("edition2015");
                let contains_e2018 = classes.contains("edition2018");
                let contains_e2021 = classes.contains("edition2021");
                let edition_class = if contains_e2015 || contains_e2018 || contains_e2021 {
                    // the user forced edition, we should not overwrite it
                    ""
                } else {
                    match edition {
                        Some(RustEdition::E2015) => " edition2015",
                        Some(RustEdition::E2018) => " edition2018",
                        Some(RustEdition::E2021) => " edition2021",
                        None => "",
                    }
                };

                // wrap the contents in an external pre block
                format!(
                    "<pre class=\"playground\"><code class=\"{}{}\">{}</code></pre>",
                    classes,
                    edition_class,
                    {
                        let content: Cow<'_, str> = if playground_config.editable
                            && classes.contains("editable")
                            || text.contains("fn main")
                            || text.contains("quick_main!")
                        {
                            code.into()
                        } else {
                            // we need to inject our own main
                            let (attrs, code) = partition_source(code);

                            format!("# #![allow(unused)]\n{}#fn main() {{\n{}#}}", attrs, code)
                                .into()
                        };
                        content
                    }
                )
            } else {
                // not language-rust, so no-op
                text.to_owned()
            }
        })
        .into_owned()
}

/// Modifies all `<code>` blocks to convert "hidden" lines and to wrap them in
/// a `<span class="boring">`.
fn hide_lines(html: &str, code_config: &Code) -> String {
    static LANGUAGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\blanguage-(\w+)\b").unwrap());
    static HIDELINES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\bhidelines=(\S+)").unwrap());

    CODE_BLOCK_RE
        .replace_all(html, |caps: &Captures<'_>| {
            let text = &caps[1];
            let classes = &caps[2];
            let code = &caps[3];

            if classes.contains("language-rust") {
                format!(
                    "<code class=\"{}\">{}</code>",
                    classes,
                    hide_lines_rust(code)
                )
            } else {
                // First try to get the prefix from the code block
                let hidelines_capture = HIDELINES_REGEX.captures(classes);
                let hidelines_prefix = match &hidelines_capture {
                    Some(capture) => Some(&capture[1]),
                    None => {
                        // Then look up the prefix by language
                        LANGUAGE_REGEX.captures(classes).and_then(|capture| {
                            code_config.hidelines.get(&capture[1]).map(|p| p.as_str())
                        })
                    }
                };

                match hidelines_prefix {
                    Some(prefix) => format!(
                        "<code class=\"{}\">{}</code>",
                        classes,
                        hide_lines_with_prefix(code, prefix)
                    ),
                    None => text.to_owned(),
                }
            }
        })
        .into_owned()
}

fn hide_lines_rust(content: &str) -> String {
    static BORING_LINES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\s*)#(.?)(.*)$").unwrap());

    let mut result = String::with_capacity(content.len());
    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        // Don't include newline on the last line.
        let newline = if lines.peek().is_none() { "" } else { "\n" };
        if let Some(caps) = BORING_LINES_REGEX.captures(line) {
            if &caps[2] == "#" {
                result += &caps[1];
                result += &caps[2];
                result += &caps[3];
                result += newline;
                continue;
            } else if &caps[2] != "!" && &caps[2] != "[" {
                result += "<span class=\"boring\">";
                result += &caps[1];
                if &caps[2] != " " {
                    result += &caps[2];
                }
                result += &caps[3];
                result += newline;
                result += "</span>";
                continue;
            }
        }
        result += line;
        result += newline;
    }
    result
}

fn hide_lines_with_prefix(content: &str, prefix: &str) -> String {
    let mut result = String::with_capacity(content.len());
    for line in content.lines() {
        if line.trim_start().starts_with(prefix) {
            let pos = line.find(prefix).unwrap();
            let (ws, rest) = (&line[..pos], &line[pos + prefix.len()..]);

            result += "<span class=\"boring\">";
            result += ws;
            result += rest;
            result += "\n";
            result += "</span>";
            continue;
        }
        result += line;
        result += "\n";
    }
    result
}

fn partition_source(s: &str) -> (String, String) {
    let mut after_header = false;
    let mut before = String::new();
    let mut after = String::new();

    for line in s.lines() {
        let trimline = line.trim();
        let header = trimline.chars().all(char::is_whitespace) || trimline.starts_with("#![");
        if !header || after_header {
            after_header = true;
            after.push_str(line);
            after.push('\n');
        } else {
            before.push_str(line);
            before.push('\n');
        }
    }

    (before, after)
}

struct RenderItemContext<'a> {
    handlebars: &'a Handlebars<'a>,
    destination: PathBuf,
    data: serde_json::Map<String, serde_json::Value>,
    is_index: bool,
    book_config: BookConfig,
    html_config: HtmlConfig,
    edition: Option<RustEdition>,
    chapter_titles: &'a HashMap<PathBuf, String>,
}

#[cfg(test)]
mod tests {
    use crate::config::TextDirection;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn original_build_header_links() {
        let inputs = vec![
            (
                "blah blah <h1>Foo</h1>",
                r##"blah blah <h1 id="foo"><a class="header" href="#foo">Foo</a></h1>"##,
            ),
            (
                "<h1>Foo</h1>",
                r##"<h1 id="foo"><a class="header" href="#foo">Foo</a></h1>"##,
            ),
            (
                "<h3>Foo^bar</h3>",
                r##"<h3 id="foobar"><a class="header" href="#foobar">Foo^bar</a></h3>"##,
            ),
            (
                "<h4></h4>",
                r##"<h4 id=""><a class="header" href="#"></a></h4>"##,
            ),
            (
                "<h4><em>Hï</em></h4>",
                r##"<h4 id="hï"><a class="header" href="#hï"><em>Hï</em></a></h4>"##,
            ),
            (
                "<h1>Foo</h1><h3>Foo</h3>",
                r##"<h1 id="foo"><a class="header" href="#foo">Foo</a></h1><h3 id="foo-1"><a class="header" href="#foo-1">Foo</a></h3>"##,
            ),
            // id only
            (
                r##"<h1 id="foobar">Foo</h1>"##,
                r##"<h1 id="foobar"><a class="header" href="#foobar">Foo</a></h1>"##,
            ),
            // class only
            (
                r##"<h1 class="class1 class2">Foo</h1>"##,
                r##"<h1 id="foo" class="class1 class2"><a class="header" href="#foo">Foo</a></h1>"##,
            ),
            // both id and class
            (
                r##"<h1 id="foobar" class="class1 class2">Foo</h1>"##,
                r##"<h1 id="foobar" class="class1 class2"><a class="header" href="#foobar">Foo</a></h1>"##,
            ),
        ];

        for (src, should_be) in inputs {
            let got = build_header_links(src);
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn add_playground() {
        let inputs = [
          ("<code class=\"language-rust\">x()</code>",
           "<pre class=\"playground\"><code class=\"language-rust\"># #![allow(unused)]\n#fn main() {\nx()\n#}</code></pre>"),
          ("<code class=\"language-rust\">fn main() {}</code>",
           "<pre class=\"playground\"><code class=\"language-rust\">fn main() {}</code></pre>"),
          ("<code class=\"language-rust editable\">let s = \"foo\n # bar\n\";</code>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n # bar\n\";</code></pre>"),
          ("<code class=\"language-rust editable\">let s = \"foo\n ## bar\n\";</code>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n ## bar\n\";</code></pre>"),
          ("<code class=\"language-rust editable\">let s = \"foo\n # bar\n#\n\";</code>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n # bar\n#\n\";</code></pre>"),
          ("<code class=\"language-rust ignore\">let s = \"foo\n # bar\n\";</code>",
           "<code class=\"language-rust ignore\">let s = \"foo\n # bar\n\";</code>"),
          ("<code class=\"language-rust editable\">#![no_std]\nlet s = \"foo\";\n #[some_attr]</code>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">#![no_std]\nlet s = \"foo\";\n #[some_attr]</code></pre>"),
        ];
        for (src, should_be) in &inputs {
            let got = add_playground_pre(
                src,
                &Playground {
                    editable: true,
                    ..Playground::default()
                },
                None,
            );
            assert_eq!(&*got, *should_be);
        }
    }
    #[test]
    fn add_playground_edition2015() {
        let inputs = [
          ("<code class=\"language-rust\">x()</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2015\"># #![allow(unused)]\n#fn main() {\nx()\n#}</code></pre>"),
          ("<code class=\"language-rust\">fn main() {}</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2015\">fn main() {}</code></pre>"),
          ("<code class=\"language-rust edition2015\">fn main() {}</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2015\">fn main() {}</code></pre>"),
          ("<code class=\"language-rust edition2018\">fn main() {}</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2018\">fn main() {}</code></pre>"),
        ];
        for (src, should_be) in &inputs {
            let got = add_playground_pre(
                src,
                &Playground {
                    editable: true,
                    ..Playground::default()
                },
                Some(RustEdition::E2015),
            );
            assert_eq!(&*got, *should_be);
        }
    }
    #[test]
    fn add_playground_edition2018() {
        let inputs = [
          ("<code class=\"language-rust\">x()</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2018\"># #![allow(unused)]\n#fn main() {\nx()\n#}</code></pre>"),
          ("<code class=\"language-rust\">fn main() {}</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2018\">fn main() {}</code></pre>"),
          ("<code class=\"language-rust edition2015\">fn main() {}</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2015\">fn main() {}</code></pre>"),
          ("<code class=\"language-rust edition2018\">fn main() {}</code>",
           "<pre class=\"playground\"><code class=\"language-rust edition2018\">fn main() {}</code></pre>"),
        ];
        for (src, should_be) in &inputs {
            let got = add_playground_pre(
                src,
                &Playground {
                    editable: true,
                    ..Playground::default()
                },
                Some(RustEdition::E2018),
            );
            assert_eq!(&*got, *should_be);
        }
    }
    #[test]
    fn add_playground_edition2021() {
        let inputs = [
            ("<code class=\"language-rust\">x()</code>",
             "<pre class=\"playground\"><code class=\"language-rust edition2021\"># #![allow(unused)]\n#fn main() {\nx()\n#}</code></pre>"),
            ("<code class=\"language-rust\">fn main() {}</code>",
             "<pre class=\"playground\"><code class=\"language-rust edition2021\">fn main() {}</code></pre>"),
            ("<code class=\"language-rust edition2015\">fn main() {}</code>",
             "<pre class=\"playground\"><code class=\"language-rust edition2015\">fn main() {}</code></pre>"),
            ("<code class=\"language-rust edition2018\">fn main() {}</code>",
             "<pre class=\"playground\"><code class=\"language-rust edition2018\">fn main() {}</code></pre>"),
        ];
        for (src, should_be) in &inputs {
            let got = add_playground_pre(
                src,
                &Playground {
                    editable: true,
                    ..Playground::default()
                },
                Some(RustEdition::E2021),
            );
            assert_eq!(&*got, *should_be);
        }
    }

    #[test]
    fn hide_lines_language_rust() {
        let inputs = [
          (
           "<pre class=\"playground\"><code class=\"language-rust\">\n# #![allow(unused)]\n#fn main() {\nx()\n#}</code></pre>",
           "<pre class=\"playground\"><code class=\"language-rust\">\n<span class=\"boring\">#![allow(unused)]\n</span><span class=\"boring\">fn main() {\n</span>x()\n<span class=\"boring\">}</span></code></pre>",),
          (
           "<pre class=\"playground\"><code class=\"language-rust\">fn main() {}</code></pre>",
           "<pre class=\"playground\"><code class=\"language-rust\">fn main() {}</code></pre>",),
          (
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n # bar\n\";</code></pre>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n<span class=\"boring\"> bar\n</span>\";</code></pre>",),
          (
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n ## bar\n\";</code></pre>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n # bar\n\";</code></pre>",),
          (
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n # bar\n#\n\";</code></pre>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">let s = \"foo\n<span class=\"boring\"> bar\n</span><span class=\"boring\">\n</span>\";</code></pre>",),
          (
           "<code class=\"language-rust ignore\">let s = \"foo\n # bar\n\";</code>",
           "<code class=\"language-rust ignore\">let s = \"foo\n<span class=\"boring\"> bar\n</span>\";</code>",),
          (
           "<pre class=\"playground\"><code class=\"language-rust editable\">#![no_std]\nlet s = \"foo\";\n #[some_attr]</code></pre>",
           "<pre class=\"playground\"><code class=\"language-rust editable\">#![no_std]\nlet s = \"foo\";\n #[some_attr]</code></pre>",),
        ];
        for (src, should_be) in &inputs {
            let got = hide_lines(src, &Code::default());
            assert_eq!(&*got, *should_be);
        }
    }

    #[test]
    fn hide_lines_language_other() {
        let inputs = [
          (
           "<code class=\"language-python\">~hidden()\nnothidden():\n~    hidden()\n    ~hidden()\n    nothidden()</code>",
           "<code class=\"language-python\"><span class=\"boring\">hidden()\n</span>nothidden():\n<span class=\"boring\">    hidden()\n</span><span class=\"boring\">    hidden()\n</span>    nothidden()\n</code>",),
           (
            "<code class=\"language-python hidelines=!!!\">!!!hidden()\nnothidden():\n!!!    hidden()\n    !!!hidden()\n    nothidden()</code>",
            "<code class=\"language-python hidelines=!!!\"><span class=\"boring\">hidden()\n</span>nothidden():\n<span class=\"boring\">    hidden()\n</span><span class=\"boring\">    hidden()\n</span>    nothidden()\n</code>",),
        ];
        for (src, should_be) in &inputs {
            let got = hide_lines(
                src,
                &Code {
                    hidelines: {
                        let mut map = HashMap::new();
                        map.insert("python".to_string(), "~".to_string());
                        map
                    },
                },
            );
            assert_eq!(&*got, *should_be);
        }
    }

    #[test]
    fn test_json_direction() {
        assert_eq!(json!(TextDirection::RightToLeft), json!("rtl"));
        assert_eq!(json!(TextDirection::LeftToRight), json!("ltr"));
    }
}

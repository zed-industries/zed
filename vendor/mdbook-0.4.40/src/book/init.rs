use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use super::MDBook;
use crate::config::Config;
use crate::errors::*;
use crate::theme;
use crate::utils::fs::write_file;
use log::{debug, error, info, trace};

/// A helper for setting up a new book and its directory structure.
#[derive(Debug, Clone, PartialEq)]
pub struct BookBuilder {
    root: PathBuf,
    create_gitignore: bool,
    config: Config,
    copy_theme: bool,
}

impl BookBuilder {
    /// Create a new `BookBuilder` which will generate a book in the provided
    /// root directory.
    pub fn new<P: Into<PathBuf>>(root: P) -> BookBuilder {
        BookBuilder {
            root: root.into(),
            create_gitignore: false,
            config: Config::default(),
            copy_theme: false,
        }
    }

    /// Set the [`Config`] to be used.
    pub fn with_config(&mut self, cfg: Config) -> &mut BookBuilder {
        self.config = cfg;
        self
    }

    /// Get the config used by the `BookBuilder`.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Should the theme be copied into the generated book (so users can tweak
    /// it)?
    pub fn copy_theme(&mut self, copy: bool) -> &mut BookBuilder {
        self.copy_theme = copy;
        self
    }

    /// Should we create a `.gitignore` file?
    pub fn create_gitignore(&mut self, create: bool) -> &mut BookBuilder {
        self.create_gitignore = create;
        self
    }

    /// Generate the actual book. This will:
    ///
    /// - Create the directory structure.
    /// - Stub out some dummy chapters and the `SUMMARY.md`.
    /// - Create a `.gitignore` (if applicable)
    /// - Create a themes directory and populate it (if applicable)
    /// - Generate a `book.toml` file,
    /// - Then load the book so we can build it or run tests.
    pub fn build(&self) -> Result<MDBook> {
        info!("Creating a new book with stub content");

        self.create_directory_structure()
            .with_context(|| "Unable to create directory structure")?;

        self.create_stub_files()
            .with_context(|| "Unable to create stub files")?;

        if self.create_gitignore {
            self.build_gitignore()
                .with_context(|| "Unable to create .gitignore")?;
        }

        if self.copy_theme {
            self.copy_across_theme()
                .with_context(|| "Unable to copy across the theme")?;
        }

        self.write_book_toml()?;

        match MDBook::load(&self.root) {
            Ok(book) => Ok(book),
            Err(e) => {
                error!("{}", e);

                panic!(
                    "The BookBuilder should always create a valid book. If you are seeing this it \
                     is a bug and should be reported."
                );
            }
        }
    }

    fn write_book_toml(&self) -> Result<()> {
        debug!("Writing book.toml");
        let book_toml = self.root.join("book.toml");
        let cfg = toml::to_vec(&self.config).with_context(|| "Unable to serialize the config")?;

        File::create(book_toml)
            .with_context(|| "Couldn't create book.toml")?
            .write_all(&cfg)
            .with_context(|| "Unable to write config to book.toml")?;
        Ok(())
    }

    fn copy_across_theme(&self) -> Result<()> {
        debug!("Copying theme");

        let html_config = self.config.html_config().unwrap_or_default();
        let themedir = html_config.theme_dir(&self.root);

        if !themedir.exists() {
            debug!(
                "{} does not exist, creating the directory",
                themedir.display()
            );
            fs::create_dir(&themedir)?;
        }

        let mut index = File::create(themedir.join("index.hbs"))?;
        index.write_all(theme::INDEX)?;

        let cssdir = themedir.join("css");
        if !cssdir.exists() {
            fs::create_dir(&cssdir)?;
        }

        let mut general_css = File::create(cssdir.join("general.css"))?;
        general_css.write_all(theme::GENERAL_CSS)?;

        let mut chrome_css = File::create(cssdir.join("chrome.css"))?;
        chrome_css.write_all(theme::CHROME_CSS)?;

        if html_config.print.enable {
            let mut print_css = File::create(cssdir.join("print.css"))?;
            print_css.write_all(theme::PRINT_CSS)?;
        }

        let mut variables_css = File::create(cssdir.join("variables.css"))?;
        variables_css.write_all(theme::VARIABLES_CSS)?;

        let mut favicon = File::create(themedir.join("favicon.png"))?;
        favicon.write_all(theme::FAVICON_PNG)?;

        let mut favicon = File::create(themedir.join("favicon.svg"))?;
        favicon.write_all(theme::FAVICON_SVG)?;

        let mut js = File::create(themedir.join("book.js"))?;
        js.write_all(theme::JS)?;

        let mut highlight_css = File::create(themedir.join("highlight.css"))?;
        highlight_css.write_all(theme::HIGHLIGHT_CSS)?;

        let mut highlight_js = File::create(themedir.join("highlight.js"))?;
        highlight_js.write_all(theme::HIGHLIGHT_JS)?;

        write_file(&themedir.join("fonts"), "fonts.css", theme::fonts::CSS)?;
        for (file_name, contents) in theme::fonts::LICENSES {
            write_file(&themedir, file_name, contents)?;
        }
        for (file_name, contents) in theme::fonts::OPEN_SANS.iter() {
            write_file(&themedir, file_name, contents)?;
        }
        write_file(
            &themedir,
            theme::fonts::SOURCE_CODE_PRO.0,
            theme::fonts::SOURCE_CODE_PRO.1,
        )?;

        Ok(())
    }

    fn build_gitignore(&self) -> Result<()> {
        debug!("Creating .gitignore");

        let mut f = File::create(self.root.join(".gitignore"))?;

        writeln!(f, "{}", self.config.build.build_dir.display())?;

        Ok(())
    }

    fn create_stub_files(&self) -> Result<()> {
        debug!("Creating example book contents");
        let src_dir = self.root.join(&self.config.book.src);

        let summary = src_dir.join("SUMMARY.md");
        if !summary.exists() {
            trace!("No summary found creating stub summary and chapter_1.md.");
            let mut f = File::create(&summary).with_context(|| "Unable to create SUMMARY.md")?;
            writeln!(f, "# Summary")?;
            writeln!(f)?;
            writeln!(f, "- [Chapter 1](./chapter_1.md)")?;

            let chapter_1 = src_dir.join("chapter_1.md");
            let mut f = File::create(chapter_1).with_context(|| "Unable to create chapter_1.md")?;
            writeln!(f, "# Chapter 1")?;
        } else {
            trace!("Existing summary found, no need to create stub files.");
        }
        Ok(())
    }

    fn create_directory_structure(&self) -> Result<()> {
        debug!("Creating directory tree");
        fs::create_dir_all(&self.root)?;

        let src = self.root.join(&self.config.book.src);
        fs::create_dir_all(src)?;

        let build = self.root.join(&self.config.build.build_dir);
        fs::create_dir_all(build)?;

        Ok(())
    }
}

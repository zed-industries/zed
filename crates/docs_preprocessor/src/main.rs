use anyhow::{Context, Result};
use mdbook::BookItem;
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::CmdPreprocessor;
use regex::Regex;
use settings::KeymapFile;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::process;
use std::sync::{LazyLock, OnceLock};
use util::paths::PathExt;

static KEYMAP_MACOS: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-macos.json").expect("Failed to load MacOS keymap")
});

static KEYMAP_LINUX: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-linux.json").expect("Failed to load Linux keymap")
});

static KEYMAP_WINDOWS: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-windows.json").expect("Failed to load Windows keymap")
});

static ALL_ACTIONS: LazyLock<Vec<ActionDef>> = LazyLock::new(dump_all_gpui_actions);

const FRONT_MATTER_COMMENT: &str = "<!-- ZED_META {} -->";

fn main() -> Result<()> {
    zlog::init();
    zlog::init_output_stderr();
    // call a zed:: function so everything in `zed` crate is linked and
    // all actions in the actual app are registered
    zed::stdout_is_a_pty();
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    match args.get(0).map(String::as_str) {
        Some("supports") => {
            let renderer = args.get(1).expect("Required argument");
            let supported = renderer != "not-supported";
            if supported {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
        Some("postprocess") => handle_postprocessing()?,
        _ => handle_preprocessing()?,
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PreprocessorError {
    ActionNotFound { action_name: String },
    DeprecatedActionUsed { used: String, should_be: String },
    InvalidFrontmatterLine(String),
}

impl PreprocessorError {
    fn new_for_not_found_action(action_name: String) -> Self {
        for action in &*ALL_ACTIONS {
            for alias in action.deprecated_aliases {
                if alias == &action_name {
                    return PreprocessorError::DeprecatedActionUsed {
                        used: action_name,
                        should_be: action.name.to_string(),
                    };
                }
            }
        }
        PreprocessorError::ActionNotFound { action_name }
    }
}

impl std::fmt::Display for PreprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreprocessorError::InvalidFrontmatterLine(line) => {
                write!(f, "Invalid frontmatter line: {}", line)
            }
            PreprocessorError::ActionNotFound { action_name } => {
                write!(f, "Action not found: {}", action_name)
            }
            PreprocessorError::DeprecatedActionUsed { used, should_be } => write!(
                f,
                "Deprecated action used: {} should be {}",
                used, should_be
            ),
        }
    }
}

fn handle_preprocessing() -> Result<()> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    let (_ctx, mut book) = CmdPreprocessor::parse_input(input.as_bytes())?;

    let mut errors = HashSet::<PreprocessorError>::new();

    handle_frontmatter(&mut book, &mut errors);
    template_big_table_of_actions(&mut book);
    template_and_validate_keybindings(&mut book, &mut errors);
    template_and_validate_actions(&mut book, &mut errors);

    if !errors.is_empty() {
        const ANSI_RED: &str = "\x1b[31m";
        const ANSI_RESET: &str = "\x1b[0m";
        for error in &errors {
            eprintln!("{ANSI_RED}ERROR{ANSI_RESET}: {}", error);
        }
        return Err(anyhow::anyhow!("Found {} errors in docs", errors.len()));
    }

    serde_json::to_writer(io::stdout(), &book)?;

    Ok(())
}

fn handle_frontmatter(book: &mut Book, errors: &mut HashSet<PreprocessorError>) {
    let frontmatter_regex = Regex::new(r"(?s)^\s*---(.*?)---").unwrap();
    for_each_chapter_mut(book, |chapter| {
        let new_content = frontmatter_regex.replace(&chapter.content, |caps: &regex::Captures| {
            let frontmatter = caps[1].trim();
            let frontmatter = frontmatter.trim_matches(&[' ', '-', '\n']);
            let mut metadata = HashMap::<String, String>::default();
            for line in frontmatter.lines() {
                let Some((name, value)) = line.split_once(':') else {
                    errors.insert(PreprocessorError::InvalidFrontmatterLine(format!(
                        "{}: {}",
                        chapter_breadcrumbs(chapter),
                        line
                    )));
                    continue;
                };
                let name = name.trim();
                let value = value.trim();
                metadata.insert(name.to_string(), value.to_string());
            }
            FRONT_MATTER_COMMENT.replace(
                "{}",
                &serde_json::to_string(&metadata).expect("Failed to serialize metadata"),
            )
        });
        if let Cow::Owned(content) = new_content {
            chapter.content = content;
        }
    });
}

fn template_big_table_of_actions(book: &mut Book) {
    for_each_chapter_mut(book, |chapter| {
        let needle = "{#ACTIONS_TABLE#}";
        if let Some(start) = chapter.content.rfind(needle) {
            chapter.content.replace_range(
                start..start + needle.len(),
                &generate_big_table_of_actions(),
            );
        }
    });
}

fn template_and_validate_keybindings(book: &mut Book, errors: &mut HashSet<PreprocessorError>) {
    let regex = Regex::new(r"\{#kb (.*?)\}").unwrap();

    for_each_chapter_mut(book, |chapter| {
        chapter.content = regex
            .replace_all(&chapter.content, |caps: &regex::Captures| {
                let action = caps[1].trim();
                if find_action_by_name(action).is_none() {
                    errors.insert(PreprocessorError::new_for_not_found_action(
                        action.to_string(),
                    ));
                    return String::new();
                }
                let macos_binding = find_binding("macos", action).unwrap_or_default();
                let linux_binding = find_binding("linux", action).unwrap_or_default();

                if macos_binding.is_empty() && linux_binding.is_empty() {
                    return "<div>No default binding</div>".to_string();
                }

                format!("<kbd class=\"keybinding\">{macos_binding}|{linux_binding}</kbd>")
            })
            .into_owned()
    });
}

fn template_and_validate_actions(book: &mut Book, errors: &mut HashSet<PreprocessorError>) {
    let regex = Regex::new(r"\{#action (.*?)\}").unwrap();

    for_each_chapter_mut(book, |chapter| {
        chapter.content = regex
            .replace_all(&chapter.content, |caps: &regex::Captures| {
                let name = caps[1].trim();
                let Some(action) = find_action_by_name(name) else {
                    errors.insert(PreprocessorError::new_for_not_found_action(
                        name.to_string(),
                    ));
                    return String::new();
                };
                format!("<code class=\"hljs\">{}</code>", &action.human_name)
            })
            .into_owned()
    });
}

fn find_action_by_name(name: &str) -> Option<&ActionDef> {
    ALL_ACTIONS
        .binary_search_by(|action| action.name.cmp(name))
        .ok()
        .map(|index| &ALL_ACTIONS[index])
}

fn find_binding(os: &str, action: &str) -> Option<String> {
    let keymap = match os {
        "macos" => &KEYMAP_MACOS,
        "linux" | "freebsd" => &KEYMAP_LINUX,
        "windows" => &KEYMAP_WINDOWS,
        _ => unreachable!("Not a valid OS: {}", os),
    };

    // Find the binding in reverse order, as the last binding takes precedence.
    keymap.sections().rev().find_map(|section| {
        section.bindings().rev().find_map(|(keystroke, a)| {
            if name_for_action(a.to_string()) == action {
                Some(keystroke.to_string())
            } else {
                None
            }
        })
    })
}

/// Removes any configurable options from the stringified action if existing,
/// ensuring that only the actual action name is returned. If the action consists
/// only of a string and nothing else, the string is returned as-is.
///
/// Example:
///
/// This will return the action name unmodified.
///
/// ```
/// let action_as_str = "assistant::Assist";
/// let action_name = name_for_action(action_as_str);
/// assert_eq!(action_name, "assistant::Assist");
/// ```
///
/// This will return the action name with any trailing options removed.
///
///
/// ```
/// let action_as_str = "\"editor::ToggleComments\", {\"advance_downwards\":false}";
/// let action_name = name_for_action(action_as_str);
/// assert_eq!(action_name, "editor::ToggleComments");
/// ```
fn name_for_action(action_as_str: String) -> String {
    action_as_str
        .split(",")
        .next()
        .map(|name| name.trim_matches('"').to_string())
        .unwrap_or(action_as_str)
}

fn chapter_breadcrumbs(chapter: &Chapter) -> String {
    let mut breadcrumbs = Vec::with_capacity(chapter.parent_names.len() + 1);
    breadcrumbs.extend(chapter.parent_names.iter().map(String::as_str));
    breadcrumbs.push(chapter.name.as_str());
    format!("[{:?}] {}", chapter.source_path, breadcrumbs.join(" > "))
}

fn load_keymap(asset_path: &str) -> Result<KeymapFile> {
    let content = util::asset_str::<settings::SettingsAssets>(asset_path);
    KeymapFile::parse(content.as_ref())
}

fn for_each_chapter_mut<F>(book: &mut Book, mut func: F)
where
    F: FnMut(&mut Chapter),
{
    book.for_each_mut(|item| {
        let BookItem::Chapter(chapter) = item else {
            return;
        };
        func(chapter);
    });
}

#[derive(Debug, serde::Serialize)]
struct ActionDef {
    name: &'static str,
    human_name: String,
    deprecated_aliases: &'static [&'static str],
    docs: Option<&'static str>,
}

fn dump_all_gpui_actions() -> Vec<ActionDef> {
    let mut actions = gpui::generate_list_of_all_registered_actions()
        .map(|action| ActionDef {
            name: action.name,
            human_name: command_palette::humanize_action_name(action.name),
            deprecated_aliases: action.deprecated_aliases,
            docs: action.documentation,
        })
        .collect::<Vec<ActionDef>>();

    actions.sort_by_key(|a| a.name);

    actions
}

fn handle_postprocessing() -> Result<()> {
    let logger = zlog::scoped!("render");
    let mut ctx = mdbook::renderer::RenderContext::from_json(io::stdin())?;
    let output = ctx
        .config
        .get_mut("output")
        .expect("has output")
        .as_table_mut()
        .expect("output is table");
    let zed_html = output.remove("zed-html").expect("zed-html output defined");
    let default_description = zed_html
        .get("default-description")
        .expect("Default description not found")
        .as_str()
        .expect("Default description not a string")
        .to_string();
    let default_title = zed_html
        .get("default-title")
        .expect("Default title not found")
        .as_str()
        .expect("Default title not a string")
        .to_string();

    output.insert("html".to_string(), zed_html);
    mdbook::Renderer::render(&mdbook::renderer::HtmlHandlebars::new(), &ctx)?;
    let ignore_list = ["toc.html"];

    let root_dir = ctx.destination.clone();
    let mut files = Vec::with_capacity(128);
    let mut queue = Vec::with_capacity(64);
    queue.push(root_dir.clone());
    while let Some(dir) = queue.pop() {
        for entry in std::fs::read_dir(&dir).context(dir.to_sanitized_string())? {
            let Ok(entry) = entry else {
                continue;
            };
            let file_type = entry.file_type().context("Failed to determine file type")?;
            if file_type.is_dir() {
                queue.push(entry.path());
            }
            if file_type.is_file()
                && matches!(
                    entry.path().extension().and_then(std::ffi::OsStr::to_str),
                    Some("html")
                )
            {
                if ignore_list.contains(&&*entry.file_name().to_string_lossy()) {
                    zlog::info!(logger => "Ignoring {}", entry.path().to_string_lossy());
                } else {
                    files.push(entry.path());
                }
            }
        }
    }

    zlog::info!(logger => "Processing {} `.html` files", files.len());
    let meta_regex = Regex::new(&FRONT_MATTER_COMMENT.replace("{}", "(.*)")).unwrap();
    for file in files {
        let contents = std::fs::read_to_string(&file)?;
        let mut meta_description = None;
        let mut meta_title = None;
        let contents = meta_regex.replace(&contents, |caps: &regex::Captures| {
            let metadata: HashMap<String, String> = serde_json::from_str(&caps[1]).with_context(|| format!("JSON Metadata: {:?}", &caps[1])).expect("Failed to deserialize metadata");
            for (kind, content) in metadata {
                match kind.as_str() {
                    "description" => {
                        meta_description = Some(content);
                    }
                    "title" => {
                        meta_title = Some(content);
                    }
                    _ => {
                        zlog::warn!(logger => "Unrecognized frontmatter key: {} in {:?}", kind, pretty_path(&file, &root_dir));
                    }
                }
            }
            String::new()
        });
        let meta_description = meta_description.as_ref().unwrap_or_else(|| {
            zlog::warn!(logger => "No meta description found for {:?}", pretty_path(&file, &root_dir));
            &default_description
        });
        let page_title = extract_title_from_page(&contents, pretty_path(&file, &root_dir));
        let meta_title = meta_title.as_ref().unwrap_or_else(|| {
            zlog::debug!(logger => "No meta title found for {:?}", pretty_path(&file, &root_dir));
            &default_title
        });
        let meta_title = format!("{} | {}", page_title, meta_title);
        zlog::trace!(logger => "Updating {:?}", pretty_path(&file, &root_dir));
        let contents = contents.replace("#description#", meta_description);
        let contents = title_regex()
            .replace(&contents, |_: &regex::Captures| {
                format!("<title>{}</title>", meta_title)
            })
            .to_string();
        // let contents = contents.replace("#title#", &meta_title);
        std::fs::write(file, contents)?;
    }
    return Ok(());

    fn pretty_path<'a>(
        path: &'a std::path::PathBuf,
        root: &'a std::path::PathBuf,
    ) -> &'a std::path::Path {
        path.strip_prefix(&root).unwrap_or(path)
    }
    fn extract_title_from_page(contents: &str, pretty_path: &std::path::Path) -> String {
        let title_tag_contents = &title_regex()
            .captures(contents)
            .with_context(|| format!("Failed to find title in {:?}", pretty_path))
            .expect("Page has <title> element")[1];

        title_tag_contents
            .trim()
            .strip_suffix("- Zed")
            .unwrap_or(title_tag_contents)
            .trim()
            .to_string()
    }
}

fn title_regex() -> &'static Regex {
    static TITLE_REGEX: OnceLock<Regex> = OnceLock::new();
    TITLE_REGEX.get_or_init(|| Regex::new(r"<title>\s*(.*?)\s*</title>").unwrap())
}

fn generate_big_table_of_actions() -> String {
    let actions = &*ALL_ACTIONS;
    let mut output = String::new();

    let mut actions_sorted = actions.iter().collect::<Vec<_>>();
    actions_sorted.sort_by_key(|a| a.name);

    // Start the definition list with custom styling for better spacing
    output.push_str("<dl style=\"line-height: 1.8;\">\n");

    for action in actions_sorted.into_iter() {
        // Add the humanized action name as the term with margin
        output.push_str(
            "<dt style=\"margin-top: 1.5em; margin-bottom: 0.5em; font-weight: bold;\"><code>",
        );
        output.push_str(&action.human_name);
        output.push_str("</code></dt>\n");

        // Add the definition with keymap name and description
        output.push_str("<dd style=\"margin-left: 2em; margin-bottom: 1em;\">\n");

        // Add the description, escaping HTML if needed
        if let Some(description) = action.docs {
            output.push_str(
                &description
                    .replace("&", "&amp;")
                    .replace("<", "&lt;")
                    .replace(">", "&gt;"),
            );
            output.push_str("<br>\n");
        }
        output.push_str("Keymap Name: <code>");
        output.push_str(action.name);
        output.push_str("</code><br>\n");
        if !action.deprecated_aliases.is_empty() {
            output.push_str("Deprecated Aliases:");
            for alias in action.deprecated_aliases.iter() {
                output.push_str("<code>");
                output.push_str(alias);
                output.push_str("</code>, ");
            }
        }
        output.push_str("\n</dd>\n");
    }

    // Close the definition list
    output.push_str("</dl>\n");

    output
}

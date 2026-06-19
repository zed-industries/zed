use anyhow::{Context, Result};
use mdbook::BookItem;
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::CmdPreprocessor;
use regex::Regex;
use settings::{KeymapFile, SettingsJsonSchemaParams, SettingsStore};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{LazyLock, OnceLock};

static KEYMAP_MACOS: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-macos.json").expect("Failed to load MacOS keymap")
});

static KEYMAP_LINUX: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-linux.json").expect("Failed to load Linux keymap")
});

static KEYMAP_WINDOWS: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-windows.json").expect("Failed to load Windows keymap")
});

static KEYMAP_JETBRAINS_MACOS: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/macos/jetbrains.json").expect("Failed to load JetBrains macOS keymap")
});

static KEYMAP_JETBRAINS_LINUX: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/linux/jetbrains.json").expect("Failed to load JetBrains Linux keymap")
});

static ALL_ACTIONS: LazyLock<ActionManifest> = LazyLock::new(load_all_actions);

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Os {
    MacOs,
    Linux,
    Windows,
}

#[derive(Clone, Copy)]
enum KeymapOverlay {
    JetBrains,
}

impl KeymapOverlay {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "jetbrains" => Some(Self::JetBrains),
            _ => None,
        }
    }

    fn keymap(self, os: Os) -> &'static KeymapFile {
        match (self, os) {
            (Self::JetBrains, Os::MacOs) => &KEYMAP_JETBRAINS_MACOS,
            (Self::JetBrains, Os::Linux | Os::Windows) => &KEYMAP_JETBRAINS_LINUX,
        }
    }
}

const FRONT_MATTER_COMMENT: &str = "<!-- ZED_META {} -->";

fn main() -> Result<()> {
    zlog::init();
    zlog::init_output_stderr();
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
    ActionNotFound {
        action_name: String,
    },
    DeprecatedActionUsed {
        used: String,
        should_be: String,
    },
    InvalidFrontmatterLine(String),
    InvalidSettingsJson {
        file: std::path::PathBuf,
        line: usize,
        snippet: String,
        error: String,
    },
    UnknownKeymapOverlay {
        overlay_name: String,
    },
}

impl PreprocessorError {
    fn new_for_not_found_action(action_name: String) -> Self {
        for action in &ALL_ACTIONS.actions {
            for alias in &action.deprecated_aliases {
                if alias == action_name.as_str() {
                    return PreprocessorError::DeprecatedActionUsed {
                        used: action_name,
                        should_be: action.name.to_string(),
                    };
                }
            }
        }
        PreprocessorError::ActionNotFound { action_name }
    }

    fn new_for_invalid_settings_json(
        chapter: &Chapter,
        location: usize,
        snippet: String,
        error: String,
    ) -> Self {
        PreprocessorError::InvalidSettingsJson {
            file: chapter.path.clone().expect("chapter has path"),
            line: chapter.content[..location].lines().count() + 1,
            snippet,
            error,
        }
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
            PreprocessorError::InvalidSettingsJson {
                file,
                line,
                snippet,
                error,
            } => {
                write!(
                    f,
                    "Invalid settings JSON at {}:{}\nError: {}\n\n{}",
                    file.display(),
                    line,
                    error,
                    snippet
                )
            }
            PreprocessorError::UnknownKeymapOverlay { overlay_name } => {
                write!(
                    f,
                    "Unknown keymap overlay: '{}'. Supported overlays: jetbrains",
                    overlay_name
                )
            }
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
    template_and_validate_json_snippets(&mut book, &mut errors)?;

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

fn format_binding(binding: String) -> String {
    binding.replace("\\", "\\\\")
}

fn template_and_validate_keybindings(book: &mut Book, errors: &mut HashSet<PreprocessorError>) {
    let regex = Regex::new(r"\{#kb(?::(\w+))?\s+(.*?)\}").unwrap();

    for_each_chapter_mut(book, |chapter| {
        chapter.content = regex
            .replace_all(&chapter.content, |caps: &regex::Captures| {
                let overlay_name = caps.get(1).map(|m| m.as_str());
                let action = caps[2].trim();

                if is_missing_action(action) {
                    errors.insert(PreprocessorError::new_for_not_found_action(
                        action.to_string(),
                    ));
                    return String::new();
                }

                let overlay = if let Some(name) = overlay_name {
                    let Some(overlay) = KeymapOverlay::parse(name) else {
                        errors.insert(PreprocessorError::UnknownKeymapOverlay {
                            overlay_name: name.to_string(),
                        });
                        return String::new();
                    };
                    Some(overlay)
                } else {
                    None
                };

                let macos_binding =
                    find_binding_with_overlay(Os::MacOs, action, overlay)
                        .unwrap_or_default();
                let linux_binding =
                    find_binding_with_overlay(Os::Linux, action, overlay)
                        .unwrap_or_default();

                if macos_binding.is_empty() && linux_binding.is_empty() {
                    return "<div>No default binding</div>".to_string();
                }

                let formatted_macos_binding = format_binding(macos_binding);
                let formatted_linux_binding = format_binding(linux_binding);

                format!("<kbd class=\"keybinding\">{formatted_macos_binding}&#124;{formatted_linux_binding}</kbd>")
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
                    if actions_available() {
                        errors.insert(PreprocessorError::new_for_not_found_action(
                            name.to_string(),
                        ));
                    }
                    return format!("<code class=\"hljs\">{}</code>", name);
                };
                format!("<code class=\"hljs\">{}</code>", &action.human_name)
            })
            .into_owned()
    });
}

fn find_action_by_name(name: &str) -> Option<&ActionDef> {
    ALL_ACTIONS
        .actions
        .binary_search_by(|action| action.name.as_str().cmp(name))
        .ok()
        .map(|index| &ALL_ACTIONS.actions[index])
}

fn actions_available() -> bool {
    !ALL_ACTIONS.actions.is_empty()
}

fn is_missing_action(name: &str) -> bool {
    actions_available() && find_action_by_name(name).is_none()
}

// Find the last binding (in keymap order) for the given action.
// Exact action matches are preferred over parameterized variants.
fn find_binding_in_keymap(keymap: &KeymapFile, action: &str) -> Option<String> {
    let find = |predicate: &dyn Fn(&str) -> bool| {
        keymap.sections().rev().find_map(|section| {
            section.bindings().rev().find_map(|(keystroke, a)| {
                if predicate(&a.to_string()) {
                    Some(keystroke.to_string())
                } else {
                    None
                }
            })
        })
    };

    // Look for exact match
    if let Some(binding) = find(&|a| a == action) {
        return Some(binding);
    }

    // Look for parameterized match
    find(&|a| name_for_action(a.to_string()) == action)
}

fn find_binding(os: Os, action: &str) -> Option<String> {
    let keymap = match os {
        Os::MacOs => &KEYMAP_MACOS,
        Os::Linux => &KEYMAP_LINUX,
        Os::Windows => &KEYMAP_WINDOWS,
    };
    find_binding_in_keymap(keymap, action)
}

fn find_binding_with_overlay(
    os: Os,
    action: &str,
    overlay: Option<KeymapOverlay>,
) -> Option<String> {
    overlay
        .and_then(|overlay| find_binding_in_keymap(overlay.keymap(os), action))
        .or_else(|| find_binding(os, action))
}

fn template_and_validate_json_snippets(
    book: &mut Book,
    errors: &mut HashSet<PreprocessorError>,
) -> Result<()> {
    let params = SettingsJsonSchemaParams {
        language_names: &[],
        font_names: &[],
        theme_names: &[],
        icon_theme_names: &[],
        lsp_adapter_names: &[],
        action_names: &[],
        action_documentation: &HashMap::default(),
        deprecations: &HashMap::default(),
        deprecation_messages: &HashMap::default(),
    };
    let settings_schema = SettingsStore::json_schema(&params);
    let settings_validator = jsonschema::validator_for(&settings_schema)
        .context("failed to compile settings JSON schema")?;

    let keymap_validator = if actions_available() {
        let keymap_schema =
            keymap_schema_for_actions(&ALL_ACTIONS.actions, &ALL_ACTIONS.schema_definitions);
        Some(
            jsonschema::validator_for(&keymap_schema)
                .context("failed to compile keymap JSON schema")?,
        )
    } else {
        None
    };

    fn for_each_labeled_code_block_mut(
        book: &mut Book,
        errors: &mut HashSet<PreprocessorError>,
        f: &dyn Fn(&str, &str) -> anyhow::Result<()>,
    ) {
        const TAGGED_JSON_BLOCK_START: &'static str = "```json [";
        const JSON_BLOCK_END: &'static str = "```";

        for_each_chapter_mut(book, |chapter| {
            let mut offset = 0;
            while let Some(loc) = chapter.content[offset..].find(TAGGED_JSON_BLOCK_START) {
                let loc = loc + offset;
                let tag_start = loc + TAGGED_JSON_BLOCK_START.len();
                offset = tag_start;
                let Some(tag_end) = chapter.content[tag_start..].find(']') else {
                    errors.insert(PreprocessorError::new_for_invalid_settings_json(
                        chapter,
                        loc,
                        chapter.content[loc..tag_start].to_string(),
                        "Unclosed JSON block tag".to_string(),
                    ));
                    continue;
                };
                let tag_end = tag_end + tag_start;

                let tag = &chapter.content[tag_start..tag_end];

                if tag.contains('\n') {
                    errors.insert(PreprocessorError::new_for_invalid_settings_json(
                        chapter,
                        loc,
                        chapter.content[loc..tag_start].to_string(),
                        "Unclosed JSON block tag".to_string(),
                    ));
                    continue;
                }

                let snippet_start = tag_end + 1;
                offset = snippet_start;

                let Some(snippet_end) = chapter.content[snippet_start..].find(JSON_BLOCK_END)
                else {
                    errors.insert(PreprocessorError::new_for_invalid_settings_json(
                        chapter,
                        loc,
                        chapter.content[loc..tag_end + 1].to_string(),
                        "Missing closing code block".to_string(),
                    ));
                    continue;
                };
                let snippet_end = snippet_start + snippet_end;
                let snippet_json = &chapter.content[snippet_start..snippet_end];
                offset = snippet_end + 3;

                if let Err(err) = f(tag, snippet_json) {
                    errors.insert(PreprocessorError::new_for_invalid_settings_json(
                        chapter,
                        loc,
                        chapter.content[loc..snippet_end + 3].to_string(),
                        err.to_string(),
                    ));
                    continue;
                };
                let tag_range_complete = tag_start - 1..tag_end + 1;
                offset -= tag_range_complete.len();
                chapter.content.replace_range(tag_range_complete, "");
            }
        });
    }

    for_each_labeled_code_block_mut(book, errors, &|label, snippet_json| {
        let mut snippet_json_fixed = snippet_json
            .to_string()
            .replace("\n>", "\n")
            .trim()
            .to_string();
        while snippet_json_fixed.starts_with("//") {
            if let Some(line_end) = snippet_json_fixed.find('\n') {
                snippet_json_fixed.replace_range(0..line_end, "");
                snippet_json_fixed = snippet_json_fixed.trim().to_string();
            }
        }
        match label {
            "settings" => {
                if !snippet_json_fixed.starts_with('{') || !snippet_json_fixed.ends_with('}') {
                    snippet_json_fixed.insert(0, '{');
                    snippet_json_fixed.push_str("\n}");
                }
                let value =
                    settings::parse_json_with_comments::<serde_json::Value>(&snippet_json_fixed)?;
                let validation_errors: Vec<String> = settings_validator
                    .iter_errors(&value)
                    .map(|err| err.to_string())
                    .collect();
                if !validation_errors.is_empty() {
                    anyhow::bail!("{}", validation_errors.join("\n"));
                }
            }
            "keymap" => {
                if !snippet_json_fixed.starts_with('[') || !snippet_json_fixed.ends_with(']') {
                    snippet_json_fixed.insert(0, '[');
                    snippet_json_fixed.push_str("\n]");
                }

                let value =
                    settings::parse_json_with_comments::<serde_json::Value>(&snippet_json_fixed)?;
                if let Some(keymap_validator) = &keymap_validator {
                    let validation_errors: Vec<String> = keymap_validator
                        .iter_errors(&value)
                        .map(|err| err.to_string())
                        .collect();
                    if !validation_errors.is_empty() {
                        anyhow::bail!("{}", validation_errors.join("\n"));
                    }
                }
            }
            "debug" => {
                if !snippet_json_fixed.starts_with('[') || !snippet_json_fixed.ends_with(']') {
                    snippet_json_fixed.insert(0, '[');
                    snippet_json_fixed.push_str("\n]");
                }

                settings::parse_json_with_comments::<task::DebugTaskFile>(&snippet_json_fixed)?;
            }
            "tasks" => {
                if !snippet_json_fixed.starts_with('[') || !snippet_json_fixed.ends_with(']') {
                    snippet_json_fixed.insert(0, '[');
                    snippet_json_fixed.push_str("\n]");
                }

                settings::parse_json_with_comments::<task::TaskTemplates>(&snippet_json_fixed)?;
            }
            "icon-theme" => {
                if !snippet_json_fixed.starts_with('{') || !snippet_json_fixed.ends_with('}') {
                    snippet_json_fixed.insert(0, '{');
                    snippet_json_fixed.push_str("\n}");
                }

                settings::parse_json_with_comments::<theme::IconThemeFamilyContent>(
                    &snippet_json_fixed,
                )?;
            }
            "semantic_token_rules" => {
                if !snippet_json_fixed.starts_with('[') || !snippet_json_fixed.ends_with(']') {
                    snippet_json_fixed.insert(0, '[');
                    snippet_json_fixed.push_str("\n]");
                }

                settings::parse_json_with_comments::<settings::SemanticTokenRules>(
                    &snippet_json_fixed,
                )?;
            }
            label => anyhow::bail!("Unexpected JSON code block tag: {label}"),
        };
        Ok(())
    });

    Ok(())
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
/// let action_as_str = "workspace::Save";
/// let action_name = name_for_action(action_as_str);
/// assert_eq!(action_name, "workspace::Save");
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ActionDef {
    name: String,
    human_name: String,
    #[serde(default)]
    schema: Option<serde_json::Value>,
    deprecated_aliases: Vec<String>,
    #[serde(default)]
    deprecation_message: Option<String>,
    #[serde(rename = "documentation")]
    docs: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct ActionManifest {
    actions: Vec<ActionDef>,
    #[serde(default)]
    schema_definitions: serde_json::Map<String, serde_json::Value>,
}

fn load_all_actions() -> ActionManifest {
    let asset_path = concat!(env!("CARGO_MANIFEST_DIR"), "/actions.json");
    match std::fs::read_to_string(asset_path) {
        Ok(content) => {
            let mut manifest: ActionManifest =
                serde_json::from_str(&content).expect("Failed to parse actions.json");
            manifest.actions.sort_by(|a, b| a.name.cmp(&b.name));
            manifest
        }
        Err(err) => {
            if std::env::var("CI").is_ok() {
                panic!("actions.json not found at {}: {}", asset_path, err);
            }
            eprintln!(
                "Warning: actions.json not found, action validation will be skipped: {}",
                err
            );
            ActionManifest {
                actions: Vec::new(),
                schema_definitions: serde_json::Map::new(),
            }
        }
    }
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
    let redirects = zed_html
        .get("redirect")
        .and_then(|redirects| redirects.as_table())
        .map(|redirects| {
            redirects
                .iter()
                .filter_map(|(source, destination)| {
                    destination
                        .as_str()
                        .map(|destination| (source.clone(), destination.to_string()))
                })
                .collect::<Vec<_>>()
        });
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
    let amplitude_key = std::env::var("DOCS_AMPLITUDE_API_KEY").unwrap_or_default();
    let consent_io_instance = std::env::var("DOCS_CONSENT_IO_INSTANCE").unwrap_or_default();
    let docs_channel = std::env::var("DOCS_CHANNEL").unwrap_or_else(|_| "stable".to_string());
    let noindex = if docs_channel == "nightly" || docs_channel == "preview" {
        "<meta name=\"robots\" content=\"noindex, nofollow\">"
    } else {
        ""
    };

    output.insert("html".to_string(), zed_html);
    mdbook::Renderer::render(&mdbook::renderer::HtmlHandlebars::new(), &ctx)?;
    let ignore_list = ["toc.html"];

    let root_dir = ctx.destination.clone();
    let mut files = Vec::with_capacity(128);
    let mut queue = Vec::with_capacity(64);
    queue.push(root_dir.clone());
    while let Some(dir) = queue.pop() {
        for entry in std::fs::read_dir(&dir).context("failed to read docs dir")? {
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
    let site_url = ctx
        .config
        .get("book.site-url")
        .and_then(|site_url| site_url.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| "/docs/".to_string());
    write_ai_discovery_artifacts(&ctx.book, &ctx.root, &root_dir, &site_url)?;
    let meta_regex = Regex::new(&FRONT_MATTER_COMMENT.replace("{}", "(.*)")).unwrap();
    for file in &files {
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
        let contents = contents.replace("#amplitude_key#", &amplitude_key);
        let contents = contents.replace("#consent_io_instance#", &consent_io_instance);
        let contents = contents.replace("#noindex#", noindex);
        let contents = add_markdown_alternate_link(&contents, file, &root_dir, &site_url);
        let contents = title_regex()
            .replace(&contents, |_: &regex::Captures| {
                format!("<title>{}</title>", meta_title)
            })
            .to_string();
        std::fs::write(file, contents)?;
    }
    if let Some(redirects) = redirects {
        write_markdown_redirect_aliases(&root_dir, &redirects)?;
        write_pages_redirects(&root_dir, &redirects)?;
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

#[derive(Debug)]
struct DocsPage {
    title: String,
    source_path: PathBuf,
}

fn write_ai_discovery_artifacts(
    book: &Book,
    book_root: &Path,
    destination: &Path,
    site_url: &str,
) -> Result<()> {
    let pages = docs_pages(book);
    copy_markdown_sources(book_root, destination, site_url, &pages)?;
    write_llms_txt(destination, site_url, &pages)?;
    write_sitemap_xml(destination, site_url, &pages)?;
    Ok(())
}

fn docs_pages(book: &Book) -> Vec<DocsPage> {
    let mut pages = Vec::new();
    for item in book.iter() {
        let BookItem::Chapter(chapter) = item else {
            continue;
        };
        let Some(source_path) = chapter.source_path.as_ref() else {
            continue;
        };
        if source_path == Path::new("SUMMARY.md") {
            continue;
        }
        pages.push(DocsPage {
            title: chapter.name.clone(),
            source_path: source_path.clone(),
        });
    }
    pages
}

fn copy_markdown_sources(
    book_root: &Path,
    destination: &Path,
    site_url: &str,
    pages: &[DocsPage],
) -> Result<()> {
    let source_root = book_root.join("src");
    for page in pages {
        let source = source_root.join(&page.source_path);
        let destination = destination.join(&page.source_path);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("failed to create markdown destination {}", parent.display())
            })?;
        }
        let contents = std::fs::read_to_string(&source)
            .with_context(|| format!("failed to read markdown source {}", source.display()))?;
        std::fs::write(
            &destination,
            add_llms_markdown_directive(&contents, site_url),
        )
        .with_context(|| {
            format!(
                "failed to write markdown source {} to {}",
                source.display(),
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

fn write_llms_txt(destination: &Path, site_url: &str, pages: &[DocsPage]) -> Result<()> {
    let mut contents = String::new();
    contents.push_str("# Zed Docs\n\n");
    contents.push_str("> Official Zed documentation pages with same-origin Markdown links.\n\n");
    contents.push_str("## Docs\n\n");
    for page in pages {
        contents.push_str("- [");
        contents.push_str(&page.title);
        contents.push_str("](");
        contents.push_str(&absolute_docs_url(site_url, &page.source_path));
        contents.push_str(")\n");
    }
    std::fs::write(destination.join("llms.txt"), contents).context("failed to write llms.txt")?;
    Ok(())
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
        contents.push_str("</loc></url>\n");
    }
    contents.push_str("</urlset>\n");
    std::fs::write(destination.join("sitemap.xml"), contents)
        .context("failed to write sitemap.xml")?;
    Ok(())
}

fn write_pages_redirects(destination: &Path, redirects: &[(String, String)]) -> Result<()> {
    let Some(deploy_root) = destination.parent() else {
        return Ok(());
    };
    let mut contents = String::new();
    for (source, destination) in redirects {
        write_redirect_line(&mut contents, &format!("/docs{source}"), destination);
        if let Some(extensionless_source) = strip_html_suffix(source) {
            write_redirect_line(
                &mut contents,
                &format!("/docs{extensionless_source}"),
                &strip_html_suffix(destination).unwrap_or_else(|| destination.to_string()),
            );
        }
        if let Some(markdown_source) = html_path_to_markdown(source) {
            if let Some(markdown_destination) = html_path_to_markdown(destination) {
                write_redirect_line(
                    &mut contents,
                    &format!("/docs{markdown_source}"),
                    &markdown_destination,
                );
            }
        }
    }
    std::fs::write(deploy_root.join("_redirects"), contents)
        .context("failed to write Cloudflare Pages _redirects")?;
    Ok(())
}

fn write_markdown_redirect_aliases(
    destination: &Path,
    redirects: &[(String, String)],
) -> Result<()> {
    for (source, redirect_destination) in redirects {
        let Some(source_markdown) = html_path_to_markdown(source) else {
            continue;
        };
        let Some(destination_markdown) = html_path_to_markdown(redirect_destination) else {
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
            "# Moved\n\n> For the complete documentation index and Markdown links, see [llms.txt](/docs/llms.txt).\n\nThis page moved to [the current docs page](https://zed.dev{}).\n",
            html_path_to_markdown(redirect_destination)
                .unwrap_or_else(|| redirect_destination.to_string())
        );
        std::fs::write(&source_markdown, contents).with_context(|| {
            format!(
                "failed to write markdown redirect alias from {} to {}",
                redirect_destination,
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

fn add_markdown_alternate_link(
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

fn add_llms_markdown_directive(contents: &str, site_url: &str) -> String {
    let directive = format!(
        "> For the complete documentation index and Markdown links, see [llms.txt]({}).\n\n",
        docs_url(site_url, Path::new("llms.txt"))
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

fn title_regex() -> &'static Regex {
    static TITLE_REGEX: OnceLock<Regex> = OnceLock::new();
    TITLE_REGEX.get_or_init(|| Regex::new(r"<title>\s*(.*?)\s*</title>").unwrap())
}

fn generate_big_table_of_actions() -> String {
    let actions = &ALL_ACTIONS.actions;
    let mut output = String::new();

    let mut actions_sorted = actions.iter().collect::<Vec<_>>();
    actions_sorted.sort_by_key(|a| a.name.as_str());

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
        if let Some(description) = action.docs.as_ref() {
            output.push_str(
                &description
                    .replace("&", "&amp;")
                    .replace("<", "&lt;")
                    .replace(">", "&gt;"),
            );
            output.push_str("<br>\n");
        }
        output.push_str("Keymap Name: <code>");
        output.push_str(&action.name);
        output.push_str("</code><br>\n");
        if !action.deprecated_aliases.is_empty() {
            output.push_str("Deprecated Alias(es): ");
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

fn keymap_schema_for_actions(
    actions: &[ActionDef],
    schema_definitions: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Value {
    let mut generator = KeymapFile::action_schema_generator();

    for (name, definition) in schema_definitions {
        generator
            .definitions_mut()
            .insert(name.clone(), definition.clone());
    }

    let mut action_schemas = Vec::new();
    let mut documentation = collections::HashMap::<&str, &str>::default();
    let mut deprecations = collections::HashMap::<&str, &str>::default();
    let mut deprecation_messages = collections::HashMap::<&str, &str>::default();

    for action in actions {
        let schema = action
            .schema
            .as_ref()
            .and_then(|v| serde_json::from_value::<schemars::Schema>(v.clone()).ok());
        action_schemas.push((action.name.as_str(), schema));
        if let Some(doc) = &action.docs {
            documentation.insert(action.name.as_str(), doc.as_str());
        }
        if let Some(msg) = &action.deprecation_message {
            deprecation_messages.insert(action.name.as_str(), msg.as_str());
        }
        for alias in &action.deprecated_aliases {
            deprecations.insert(alias.as_str(), action.name.as_str());
            let alias_schema = action
                .schema
                .as_ref()
                .and_then(|v| serde_json::from_value::<schemars::Schema>(v.clone()).ok());
            action_schemas.push((alias.as_str(), alias_schema));
        }
    }

    KeymapFile::generate_json_schema(
        generator,
        action_schemas,
        &documentation,
        &deprecations,
        &deprecation_messages,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_find_binding_prefers_exact_match_over_parameterized() {
        let keymap: KeymapFile = serde_json::from_value(json!([
            {
                "bindings": {
                    "ctrl-tab": "agents_sidebar::ToggleThreadSwitcher",
                    "ctrl-shift-tab": ["agents_sidebar::ToggleThreadSwitcher", { "select_last": true }]
                }
            }
        ]))
        .unwrap();

        let binding = find_binding_in_keymap(&keymap, "agents_sidebar::ToggleThreadSwitcher");
        assert_eq!(binding.as_deref(), Some("ctrl-tab"));
    }

    #[test]
    fn test_find_binding_falls_back_to_parameterized_match() {
        let keymap: KeymapFile = serde_json::from_value(json!([
            {
                "bindings": {
                    "ctrl-shift-tab": ["agents_sidebar::ToggleThreadSwitcher", { "select_last": true }]
                }
            }
        ]))
        .unwrap();

        let binding = find_binding_in_keymap(&keymap, "agents_sidebar::ToggleThreadSwitcher");
        assert_eq!(binding.as_deref(), Some("ctrl-shift-tab"));
    }

    #[test]
    fn test_find_binding_prefers_exact_match_regardless_of_order() {
        let keymap: KeymapFile = serde_json::from_value(json!([
            {
                "bindings": {
                    "ctrl-shift-tab": ["agents_sidebar::ToggleThreadSwitcher", { "select_last": true }],
                    "ctrl-tab": "agents_sidebar::ToggleThreadSwitcher"
                }
            }
        ]))
        .unwrap();

        let binding = find_binding_in_keymap(&keymap, "agents_sidebar::ToggleThreadSwitcher");
        assert_eq!(binding.as_deref(), Some("ctrl-tab"));
    }

    #[test]
    fn test_find_binding_later_section_overrides_earlier() {
        let keymap: KeymapFile = serde_json::from_value(json!([
            { "bindings": { "ctrl-a": "some::Action" } },
            { "bindings": { "ctrl-b": "some::Action" } }
        ]))
        .unwrap();

        let binding = find_binding_in_keymap(&keymap, "some::Action");
        assert_eq!(binding.as_deref(), Some("ctrl-b"));
    }
}

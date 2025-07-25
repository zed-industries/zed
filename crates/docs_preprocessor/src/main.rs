use anyhow::{Context, Result};
use mdbook::BookItem;
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::CmdPreprocessor;
use regex::Regex;
use settings::KeymapFile;
use std::collections::HashSet;
use std::io::{self, Read};
use std::process;
use std::sync::LazyLock;
use util::paths::PathExt;

static KEYMAP_MACOS: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-macos.json").expect("Failed to load MacOS keymap")
});

static KEYMAP_LINUX: LazyLock<KeymapFile> = LazyLock::new(|| {
    load_keymap("keymaps/default-linux.json").expect("Failed to load Linux keymap")
});

static ALL_ACTIONS: LazyLock<Vec<ActionDef>> = LazyLock::new(dump_all_gpui_actions);

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
        Some("postprocess") => {
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
            output.insert("html".to_string(), zed_html);
            mdbook::Renderer::render(&mdbook::renderer::HtmlHandlebars::new(), &ctx)?;

            let root_dir = ctx.destination.clone();
            let mut files = Vec::with_capacity(128);
            let mut queue = Vec::with_capacity(64);
            queue.push(root_dir.clone());
            while let Some(dir) = queue.pop() {
                for entry in std::fs::read_dir(&dir).context(dir.to_sanitized_string())? {
                    let Ok(entry) = entry else {
                        continue;
                    };
                    let file_type = entry.file_type()?;
                    if file_type.is_dir() {
                        queue.push(entry.path());
                    }
                    if file_type.is_file()
                        && matches!(
                            entry.path().extension().and_then(std::ffi::OsStr::to_str),
                            Some("html")
                        )
                    {
                        files.push(entry.path());
                    }
                }
            }

            zlog::info!(logger => "Processing {} `.html` files", files.len());
            let regex = Regex::new(r"<p>\s*\{#zed-meta \s*\n?\s*(.*?)\s*\n?\s*\}\s*</p>").unwrap();
            for file in files {
                let contents = std::fs::read_to_string(&file)?;
                let mut meta_description = None;
                let contents = regex.replace(&contents, |caps: &regex::Captures| {
                    meta_description = Some(caps[1].trim().to_string());
                    String::new()
                });
                let meta_description = meta_description.as_ref().unwrap_or_else(|| {
                    if contents.find("#zed-meta").is_some() {
                        zlog::error!(logger => "Failed to parse meta for {:?}", pretty_path(&file, &root_dir));
                    } else {
                        zlog::warn!(logger => "No meta found for {:?}", pretty_path(&file, &root_dir));
                    }
                    &default_description
                });
                zlog::trace!(logger => "Updating {:?}", pretty_path(&file, &root_dir));
                let contents = contents.replace("#description#", meta_description);
                std::fs::write(file, contents)?;
            }
            fn pretty_path<'a>(
                path: &'a std::path::PathBuf,
                root: &'a std::path::PathBuf,
            ) -> &'a std::path::Path {
                &path.strip_prefix(&root).unwrap_or(&path)
            }
        }
        _ => handle_preprocessing()?,
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Error {
    ActionNotFound { action_name: String },
    DeprecatedActionUsed { used: String, should_be: String },
}

impl Error {
    fn new_for_not_found_action(action_name: String) -> Self {
        for action in &*ALL_ACTIONS {
            for alias in action.deprecated_aliases {
                if alias == &action_name {
                    return Error::DeprecatedActionUsed {
                        used: action_name.clone(),
                        should_be: action.name.to_string(),
                    };
                }
            }
        }
        Error::ActionNotFound {
            action_name: action_name.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ActionNotFound { action_name } => write!(f, "Action not found: {}", action_name),
            Error::DeprecatedActionUsed { used, should_be } => write!(
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

    let mut errors = HashSet::<Error>::new();

    template_and_validate_keybindings(&mut book, &mut errors);
    template_and_validate_actions(&mut book, &mut errors);

    if !errors.is_empty() {
        const ANSI_RED: &'static str = "\x1b[31m";
        const ANSI_RESET: &'static str = "\x1b[0m";
        for error in &errors {
            eprintln!("{ANSI_RED}ERROR{ANSI_RESET}: {}", error);
        }
        return Err(anyhow::anyhow!("Found {} errors in docs", errors.len()));
    }

    serde_json::to_writer(io::stdout(), &book)?;

    Ok(())
}

fn template_and_validate_keybindings(book: &mut Book, errors: &mut HashSet<Error>) {
    let regex = Regex::new(r"\{#kb (.*?)\}").unwrap();

    for_each_chapter_mut(book, |chapter| {
        chapter.content = regex
            .replace_all(&chapter.content, |caps: &regex::Captures| {
                let action = caps[1].trim();
                if find_action_by_name(action).is_none() {
                    errors.insert(Error::new_for_not_found_action(action.to_string()));
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

fn template_and_validate_actions(book: &mut Book, errors: &mut HashSet<Error>) {
    let regex = Regex::new(r"\{#action (.*?)\}").unwrap();

    for_each_chapter_mut(book, |chapter| {
        chapter.content = regex
            .replace_all(&chapter.content, |caps: &regex::Captures| {
                let name = caps[1].trim();
                let Some(action) = find_action_by_name(name) else {
                    errors.insert(Error::new_for_not_found_action(name.to_string()));
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
}

fn dump_all_gpui_actions() -> Vec<ActionDef> {
    let mut actions = gpui::generate_list_of_all_registered_actions()
        .map(|action| ActionDef {
            name: action.name,
            human_name: command_palette::humanize_action_name(action.name),
            deprecated_aliases: action.deprecated_aliases,
        })
        .collect::<Vec<ActionDef>>();

    actions.sort_by_key(|a| a.name);

    return actions;
}

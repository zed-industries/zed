use std::sync::{Arc, LazyLock};

use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, AsyncApp, BorrowAppContext as _, Entity, Task, WeakEntity};
use language::{LanguageRegistry, LspAdapterDelegate, language_settings::AllLanguageSettings};
use parking_lot::RwLock;
use project::{LspStore, lsp_store::LocalLspAdapterDelegate};
use settings::{LSP_SETTINGS_SCHEMA_URL_PREFIX, Settings as _, SettingsLocation};
use util::schemars::{AllowTrailingCommas, DefaultDenyUnknownFields};

const SCHEMA_URI_PREFIX: &str = "zed://schemas/";

const TSCONFIG_SCHEMA: &str = include_str!("schemas/tsconfig.json");
const PACKAGE_JSON_SCHEMA: &str = include_str!("schemas/package.json");

static TASKS_SCHEMA: LazyLock<String> = LazyLock::new(|| {
    serde_json::to_string(&task::TaskTemplates::generate_json_schema())
        .expect("TaskTemplates schema should serialize")
});

static SNIPPETS_SCHEMA: LazyLock<String> = LazyLock::new(|| {
    serde_json::to_string(&snippet_provider::format::VsSnippetsFile::generate_json_schema())
        .expect("VsSnippetsFile schema should serialize")
});

static JSONC_SCHEMA: LazyLock<String> = LazyLock::new(|| {
    serde_json::to_string(&generate_jsonc_schema()).expect("JSONC schema should serialize")
});

#[cfg(debug_assertions)]
static INSPECTOR_STYLE_SCHEMA: LazyLock<String> = LazyLock::new(|| {
    serde_json::to_string(&generate_inspector_style_schema())
        .expect("Inspector style schema should serialize")
});

static KEYMAP_SCHEMA: LazyLock<String> = LazyLock::new(|| {
    serde_json::to_string(&settings::KeymapFile::generate_json_schema_from_inventory())
        .expect("Keymap schema should serialize")
});

static ACTION_SCHEMA_CACHE: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::default()));

// Runtime cache for dynamic schemas that depend on runtime state:
// - "settings": depends on installed fonts, themes, languages, LSP adapters (extensions can add these)
// - "settings/lsp/*": depends on LSP adapter initialization options
// - "debug_tasks": depends on DAP adapters (extensions can add these)
// Cache is invalidated via notify_schema_changed() when extensions or DAP registry change.
static DYNAMIC_SCHEMA_CACHE: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::default()));

pub fn init(cx: &mut App) {
    cx.set_global(SchemaStore::default());
    project::lsp_store::json_language_server_ext::register_schema_handler(
        handle_schema_request,
        cx,
    );

    cx.observe_new(|_, _, cx| {
        let lsp_store = cx.weak_entity();
        cx.global_mut::<SchemaStore>().lsp_stores.push(lsp_store);
    })
    .detach();

    if let Some(extension_events) = extension::ExtensionEvents::try_global(cx) {
        cx.subscribe(&extension_events, move |_, evt, cx| {
            match evt {
                extension::Event::ExtensionInstalled(_)
                | extension::Event::ExtensionUninstalled(_)
                | extension::Event::ConfigureExtensionRequested(_) => return,
                extension::Event::ExtensionsInstalledChanged => {}
            }
            cx.update_global::<SchemaStore, _>(|schema_store, cx| {
                schema_store.notify_schema_changed(&format!("{SCHEMA_URI_PREFIX}settings"), cx);
                schema_store
                    .notify_schema_changed(&format!("{SCHEMA_URI_PREFIX}project_settings"), cx);
            });
        })
        .detach();
    }

    cx.observe_global::<dap::DapRegistry>(move |cx| {
        cx.update_global::<SchemaStore, _>(|schema_store, cx| {
            schema_store.notify_schema_changed(&format!("{SCHEMA_URI_PREFIX}debug_tasks"), cx);
        });
    })
    .detach();
}

#[derive(Default)]
pub struct SchemaStore {
    lsp_stores: Vec<WeakEntity<LspStore>>,
}

impl gpui::Global for SchemaStore {}

impl SchemaStore {
    fn notify_schema_changed(&mut self, uri: &str, cx: &mut App) {
        DYNAMIC_SCHEMA_CACHE.write().remove(uri);

        let uri = uri.to_string();
        self.lsp_stores.retain(|lsp_store| {
            let Some(lsp_store) = lsp_store.upgrade() else {
                return false;
            };
            project::lsp_store::json_language_server_ext::notify_schema_changed(
                lsp_store,
                uri.clone(),
                cx,
            );
            true
        })
    }
}

pub fn handle_schema_request(
    lsp_store: Entity<LspStore>,
    uri: String,
    cx: &mut AsyncApp,
) -> Task<Result<String>> {
    let path = match uri.strip_prefix(SCHEMA_URI_PREFIX) {
        Some(path) => path,
        None => return Task::ready(Err(anyhow::anyhow!("Invalid schema URI: {}", uri))),
    };

    if let Some(json) = resolve_static_schema(path) {
        return Task::ready(Ok(json));
    }

    if let Some(cached) = DYNAMIC_SCHEMA_CACHE.read().get(&uri).cloned() {
        return Task::ready(Ok(cached));
    }

    let path = path.to_string();
    let uri_clone = uri.clone();
    cx.spawn(async move |cx| {
        let schema = resolve_dynamic_schema(lsp_store, &path, cx).await?;
        let json = serde_json::to_string(&schema).context("Failed to serialize schema")?;

        DYNAMIC_SCHEMA_CACHE.write().insert(uri_clone, json.clone());

        Ok(json)
    })
}

fn resolve_static_schema(path: &str) -> Option<String> {
    let (schema_name, rest) = path.split_once('/').unzip();
    let schema_name = schema_name.unwrap_or(path);

    match schema_name {
        "tsconfig" => Some(TSCONFIG_SCHEMA.to_string()),
        "package_json" => Some(PACKAGE_JSON_SCHEMA.to_string()),
        "tasks" => Some(TASKS_SCHEMA.clone()),
        "snippets" => Some(SNIPPETS_SCHEMA.clone()),
        "jsonc" => Some(JSONC_SCHEMA.clone()),
        "keymap" => Some(KEYMAP_SCHEMA.clone()),
        "zed_inspector_style" => {
            #[cfg(debug_assertions)]
            {
                Some(INSPECTOR_STYLE_SCHEMA.clone())
            }
            #[cfg(not(debug_assertions))]
            {
                Some(
                    serde_json::to_string(&schemars::json_schema!(true).to_value())
                        .expect("true schema should serialize"),
                )
            }
        }

        "action" => {
            let normalized_action_name = match rest {
                Some(name) => name,
                None => return None,
            };
            let action_name = denormalize_action_name(normalized_action_name);

            if let Some(cached) = ACTION_SCHEMA_CACHE.read().get(&action_name).cloned() {
                return Some(cached);
            }

            let mut generator = settings::KeymapFile::action_schema_generator();
            let schema =
                settings::KeymapFile::get_action_schema_by_name(&action_name, &mut generator);
            let json = serde_json::to_string(
                &root_schema_from_action_schema(schema, &mut generator).to_value(),
            )
            .expect("Action schema should serialize");

            ACTION_SCHEMA_CACHE
                .write()
                .insert(action_name, json.clone());
            Some(json)
        }

        _ => None,
    }
}

async fn resolve_dynamic_schema(
    lsp_store: Entity<LspStore>,
    path: &str,
    cx: &mut AsyncApp,
) -> Result<serde_json::Value> {
    let languages = lsp_store.read_with(cx, |lsp_store, _| lsp_store.languages.clone());
    let (schema_name, rest) = path.split_once('/').unzip();
    let schema_name = schema_name.unwrap_or(path);

    let schema = match schema_name {
        "settings" if rest.is_some_and(|r| r.starts_with("lsp/")) => {
            let lsp_name = rest
                .and_then(|r| {
                    r.strip_prefix(
                        LSP_SETTINGS_SCHEMA_URL_PREFIX
                            .strip_prefix(SCHEMA_URI_PREFIX)
                            .and_then(|s| s.strip_prefix("settings/"))
                            .unwrap_or("lsp/"),
                    )
                })
                .context("Invalid LSP schema path")?;

            let adapter = languages
                .all_lsp_adapters()
                .into_iter()
                .find(|adapter| adapter.name().as_ref() as &str == lsp_name)
                .with_context(|| format!("LSP adapter not found: {}", lsp_name))?;

            let delegate: Arc<dyn LspAdapterDelegate> = cx
                .update(|inner_cx| {
                    lsp_store.update(inner_cx, |lsp_store, cx| {
                        let Some(local) = lsp_store.as_local() else {
                            return None;
                        };
                        let Some(worktree) = local.worktree_store.read(cx).worktrees().next()
                        else {
                            return None;
                        };
                        Some(LocalLspAdapterDelegate::from_local_lsp(
                            local, &worktree, cx,
                        ))
                    })
                })
                .context(concat!(
                    "Failed to create adapter delegate - ",
                    "either LSP store is not in local mode or no worktree is available"
                ))?;

            adapter
                .initialization_options_schema(&delegate, cx)
                .await
                .unwrap_or_else(|| {
                    serde_json::json!({
                        "type": "object",
                        "additionalProperties": true
                    })
                })
        }
        "settings" => {
            let lsp_adapter_names = languages
                .all_lsp_adapters()
                .into_iter()
                .map(|adapter| adapter.name().to_string())
                .collect::<Vec<_>>();

            cx.update(|cx| {
                let font_names = &cx.text_system().all_font_names();
                let language_names = &languages
                    .language_names()
                    .into_iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<_>>();

                let mut icon_theme_names = vec![];
                let mut theme_names = vec![];
                if let Some(registry) = theme::ThemeRegistry::try_global(cx) {
                    icon_theme_names.extend(
                        registry
                            .list_icon_themes()
                            .into_iter()
                            .map(|icon_theme| icon_theme.name),
                    );
                    theme_names.extend(registry.list_names());
                }
                let icon_theme_names = icon_theme_names.as_slice();
                let theme_names = theme_names.as_slice();

                cx.global::<settings::SettingsStore>().json_schema(
                    &settings::SettingsJsonSchemaParams {
                        language_names,
                        font_names,
                        theme_names,
                        icon_theme_names,
                        lsp_adapter_names: &lsp_adapter_names,
                    },
                )
            })
        }
        "project_settings" => {
            let lsp_adapter_names = languages
                .all_lsp_adapters()
                .into_iter()
                .map(|adapter| adapter.name().to_string())
                .collect::<Vec<_>>();

            cx.update(|cx| {
                let language_names = &languages
                    .language_names()
                    .into_iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<_>>();

                cx.global::<settings::SettingsStore>().project_json_schema(
                    &settings::SettingsJsonSchemaParams {
                        language_names,
                        lsp_adapter_names: &lsp_adapter_names,
                        // These are not allowed in project-specific settings but
                        // they're still fields required by the
                        // `SettingsJsonSchemaParams` struct.
                        font_names: &[],
                        theme_names: &[],
                        icon_theme_names: &[],
                    },
                )
            })
        }
        "debug_tasks" => {
            let adapter_schemas = cx.read_global::<dap::DapRegistry, _>(|dap_registry, _| {
                dap_registry.adapters_schema()
            });
            task::DebugTaskFile::generate_json_schema(&adapter_schemas)
        }
        "keymap" => cx.update(settings::KeymapFile::generate_json_schema_for_registered_actions),
        "action" => {
            let normalized_action_name = rest.context("No Action name provided")?;
            let action_name = denormalize_action_name(normalized_action_name);
            let mut generator = settings::KeymapFile::action_schema_generator();
            let schema = cx
                .update(|cx| cx.action_schema_by_name(&action_name, &mut generator))
                .flatten();
            root_schema_from_action_schema(schema, &mut generator).to_value()
        }
        "tasks" => task::TaskTemplates::generate_json_schema(),
        _ => {
            anyhow::bail!("Unrecognized schema: {schema_name}");
        }
    };
    Ok(schema)
}

const JSONC_LANGUAGE_NAME: &str = "JSONC";

pub fn all_schema_file_associations(
    languages: &Arc<LanguageRegistry>,
    path: Option<SettingsLocation<'_>>,
    cx: &mut App,
) -> serde_json::Value {
    let extension_globs = languages
        .available_language_for_name(JSONC_LANGUAGE_NAME)
        .map(|language| language.matcher().path_suffixes.clone())
        .into_iter()
        .flatten()
        // Path suffixes can be entire file names or just their extensions.
        .flat_map(|path_suffix| [format!("*.{path_suffix}"), path_suffix]);
    let override_globs = AllLanguageSettings::get(path, cx)
        .file_types
        .get(JSONC_LANGUAGE_NAME)
        .into_iter()
        .flat_map(|(_, glob_strings)| glob_strings)
        .cloned();
    let jsonc_globs = extension_globs.chain(override_globs).collect::<Vec<_>>();

    let mut file_associations = serde_json::json!([
        {
            "fileMatch": [
                schema_file_match(paths::settings_file()),
            ],
            "url": format!("{SCHEMA_URI_PREFIX}settings"),
        },
        {
            "fileMatch": [
            paths::local_settings_file_relative_path()],
            "url": format!("{SCHEMA_URI_PREFIX}project_settings"),
        },
        {
            "fileMatch": [schema_file_match(paths::keymap_file())],
            "url": format!("{SCHEMA_URI_PREFIX}keymap"),
        },
        {
            "fileMatch": [
                schema_file_match(paths::tasks_file()),
                paths::local_tasks_file_relative_path()
            ],
            "url": format!("{SCHEMA_URI_PREFIX}tasks"),
        },
        {
            "fileMatch": [
                schema_file_match(paths::debug_scenarios_file()),
                paths::local_debug_file_relative_path()
            ],
            "url": format!("{SCHEMA_URI_PREFIX}debug_tasks"),
        },
        {
            "fileMatch": [
                schema_file_match(
                    paths::snippets_dir()
                        .join("*.json")
                        .as_path()
                )
            ],
            "url": format!("{SCHEMA_URI_PREFIX}snippets"),
        },
        {
            "fileMatch": ["tsconfig.json"],
            "url": format!("{SCHEMA_URI_PREFIX}tsconfig")
        },
        {
            "fileMatch": ["package.json"],
            "url": format!("{SCHEMA_URI_PREFIX}package_json")
        },
        {
            "fileMatch": &jsonc_globs,
            "url": format!("{SCHEMA_URI_PREFIX}jsonc")
        },
    ]);

    #[cfg(debug_assertions)]
    {
        file_associations
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "fileMatch": [
                    "zed-inspector-style.json"
                ],
                "url": format!("{SCHEMA_URI_PREFIX}zed_inspector_style")
            }));
    }

    file_associations
        .as_array_mut()
        .unwrap()
        .extend(cx.all_action_names().into_iter().map(|&name| {
            let normalized_name = normalize_action_name(name);
            let file_name = normalized_action_name_to_file_name(normalized_name.clone());
            serde_json::json!({
                "fileMatch": [file_name],
                "url": format!("{}action/{normalized_name}", SCHEMA_URI_PREFIX)
            })
        }));

    file_associations
}

fn generate_jsonc_schema() -> serde_json::Value {
    let generator = schemars::generate::SchemaSettings::draft2019_09()
        .with_transform(DefaultDenyUnknownFields)
        .with_transform(AllowTrailingCommas)
        .into_generator();
    let meta_schema = generator
        .settings()
        .meta_schema
        .as_ref()
        .expect("meta_schema should be present in schemars settings")
        .to_string();
    let defs = generator.definitions();
    let schema = schemars::json_schema!({
        "$schema": meta_schema,
        "allowTrailingCommas": true,
        "$defs": defs,
    });
    serde_json::to_value(schema).unwrap()
}

#[cfg(debug_assertions)]
fn generate_inspector_style_schema() -> serde_json::Value {
    let schema = schemars::generate::SchemaSettings::draft2019_09()
        .with_transform(util::schemars::DefaultDenyUnknownFields)
        .into_generator()
        .root_schema_for::<gpui::StyleRefinement>();

    serde_json::to_value(schema).unwrap()
}

pub fn normalize_action_name(action_name: &str) -> String {
    action_name.replace("::", "__")
}

pub fn denormalize_action_name(action_name: &str) -> String {
    action_name.replace("__", "::")
}

pub fn normalized_action_file_name(action_name: &str) -> String {
    normalized_action_name_to_file_name(normalize_action_name(action_name))
}

pub fn normalized_action_name_to_file_name(mut normalized_action_name: String) -> String {
    normalized_action_name.push_str(".json");
    normalized_action_name
}

fn root_schema_from_action_schema(
    action_schema: Option<schemars::Schema>,
    generator: &mut schemars::SchemaGenerator,
) -> schemars::Schema {
    let Some(mut action_schema) = action_schema else {
        return schemars::json_schema!(false);
    };
    let meta_schema = generator
        .settings()
        .meta_schema
        .as_ref()
        .expect("meta_schema should be present in schemars settings")
        .to_string();
    let defs = generator.definitions();
    let mut schema = schemars::json_schema!({
        "$schema": meta_schema,
        "allowTrailingCommas": true,
        "$defs": defs,
    });
    schema
        .ensure_object()
        .extend(std::mem::take(action_schema.ensure_object()));
    schema
}

#[inline]
fn schema_file_match(path: &std::path::Path) -> String {
    path.strip_prefix(path.parent().unwrap().parent().unwrap())
        .unwrap()
        .display()
        .to_string()
        .replace('\\', "/")
}

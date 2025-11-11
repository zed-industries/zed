//! # json_schema_store
use std::{str::FromStr, sync::Arc};

use anyhow::{Context as _, Result};
use gpui::{App, AsyncApp, BorrowAppContext as _, Entity, WeakEntity};
use language::LanguageRegistry;
use project::LspStore;

// Origin: https://github.com/SchemaStore/schemastore
const TSCONFIG_SCHEMA: &str = include_str!("schemas/tsconfig.json");
const PACKAGE_JSON_SCHEMA: &str = include_str!("schemas/package.json");

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
        cx.subscribe(&extension_events, |_, evt, cx| {
            match evt {
                extension::Event::ExtensionInstalled(_)
                | extension::Event::ExtensionUninstalled(_)
                | extension::Event::ConfigureExtensionRequested(_) => return,
                extension::Event::ExtensionsInstalledChanged => {}
            }
            cx.update_global::<SchemaStore, _>(|schema_store, cx| {
                schema_store.notify_schema_changed("zed://schemas/settings", cx);
            });
        })
        .detach();
    }

    cx.observe_global::<dap::DapRegistry>(|cx| {
        cx.update_global::<SchemaStore, _>(|schema_store, cx| {
            schema_store.notify_schema_changed("zed://schemas/debug_tasks", cx);
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

fn handle_schema_request(
    lsp_store: Entity<LspStore>,
    uri: String,
    cx: &mut AsyncApp,
) -> Result<String> {
    let languages = lsp_store.read_with(cx, |lsp_store, _| lsp_store.languages.clone())?;
    let schema = resolve_schema_request(&languages, uri, cx)?;
    serde_json::to_string(&schema).context("Failed to serialize schema")
}

pub fn resolve_schema_request(
    languages: &Arc<LanguageRegistry>,
    uri: String,
    cx: &mut AsyncApp,
) -> Result<serde_json::Value> {
    let path = uri.strip_prefix("zed://schemas/").context("Invalid URI")?;
    resolve_schema_request_inner(languages, path, cx)
}

pub fn resolve_schema_request_inner(
    languages: &Arc<LanguageRegistry>,
    path: &str,
    cx: &mut AsyncApp,
) -> Result<serde_json::Value> {
    let (schema_name, rest) = path.split_once('/').unzip();
    let schema_name = schema_name.unwrap_or(path);

    let schema = match schema_name {
        "settings" => cx.update(|cx| {
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
                },
            )
        })?,
        "keymap" => cx.update(settings::KeymapFile::generate_json_schema_for_registered_actions)?,
        "action" => {
            let normalized_action_name = rest.context("No Action name provided")?;
            let action_name = denormalize_action_name(normalized_action_name);
            let mut generator = settings::KeymapFile::action_schema_generator();
            let schema = cx
                // PERF: cx.action_schema_by_name(action_name, &mut generator)
                .update(|cx| cx.action_schemas(&mut generator))?
                .into_iter()
                .find_map(|(name, schema)| (name == action_name).then_some(schema))
                .flatten();
            root_schema_from_action_schema(schema, &mut generator).to_value()
        }
        "tasks" => task::TaskTemplates::generate_json_schema(),
        "debug_tasks" => {
            let adapter_schemas = cx.read_global::<dap::DapRegistry, _>(|dap_registry, _| {
                dap_registry.adapters_schema()
            })?;
            task::DebugTaskFile::generate_json_schema(&adapter_schemas)
        }
        "package_json" => package_json_schema(),
        "tsconfig" => tsconfig_schema(),
        "zed_inspector_style" => {
            if cfg!(debug_assertions) {
                generate_inspector_style_schema()
            } else {
                schemars::json_schema!(true).to_value()
            }
        }
        "snippets" => snippet_provider::format::VsSnippetsFile::generate_json_schema(),
        _ => {
            anyhow::bail!("Unrecognized builtin JSON schema: {}", schema_name);
        }
    };
    Ok(schema)
}

pub fn all_schema_file_associations(cx: &mut App) -> serde_json::Value {
    let mut file_associations = serde_json::json!([
        {
            "fileMatch": [
                schema_file_match(paths::settings_file()),
                paths::local_settings_file_relative_path()
            ],
            "url": "zed://schemas/settings",
        },
        {
            "fileMatch": [schema_file_match(paths::keymap_file())],
            "url": "zed://schemas/keymap",
        },
        {
            "fileMatch": [
                schema_file_match(paths::tasks_file()),
                paths::local_tasks_file_relative_path()
            ],
            "url": "zed://schemas/tasks",
        },
        {
            "fileMatch": [
                schema_file_match(paths::debug_scenarios_file()),
                paths::local_debug_file_relative_path()
            ],
            "url": "zed://schemas/debug_tasks",
        },
        {
            "fileMatch": [
                schema_file_match(
                    paths::snippets_dir()
                        .join("*.json")
                        .as_path()
                )
            ],
            "url": "zed://schemas/snippets",
        },
        {
            "fileMatch": ["tsconfig.json"],
            "url": "zed://schemas/tsconfig"
        },
        {
            "fileMatch": ["package.json"],
            "url": "zed://schemas/package_json"
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
                "url": "zed://schemas/zed_inspector_style"
            }));
    }

    file_associations.as_array_mut().unwrap().extend(
        // ?PERF: use all_action_schemas() and don't include action schemas with no arguments
        cx.all_action_names().into_iter().map(|&name| {
            let normalized_name = normalize_action_name(name);
            let file_name = normalized_action_name_to_file_name(normalized_name.clone());
            serde_json::json!({
                "fileMatch": [file_name],
                "url": format!("zed://schemas/action/{}", normalized_name)
            })
        }),
    );

    file_associations
}

fn tsconfig_schema() -> serde_json::Value {
    serde_json::Value::from_str(TSCONFIG_SCHEMA).unwrap()
}

fn package_json_schema() -> serde_json::Value {
    serde_json::Value::from_str(PACKAGE_JSON_SCHEMA).unwrap()
}

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

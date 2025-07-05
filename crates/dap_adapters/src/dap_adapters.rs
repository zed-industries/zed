mod codelldb;
mod gdb;
mod go;
mod javascript;
mod php;
mod python;

use std::sync::Arc;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
pub use codelldb::CodeLldbDebugAdapter;
use collections::HashMap;
use dap::{
    DapRegistry,
    adapters::{
        self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
        DownloadedFileType, GithubRepo,
    },
    configure_tcp_connection,
};
use fs::Fs as _;
use gdb::GdbDebugAdapter;
pub use go::GoDebugAdapter;
use gpui::{App, BorrowAppContext, http_client::github::GithubRelease};
pub use javascript::JsDebugAdapter;
use php::PhpDebugAdapter;
pub use python::PythonDebugAdapter;
use serde::{Deserialize, Serialize};
use serde_json::json;
use task::{DebugScenario, EnvVariableReplacer, VariableName, ZedDebugConfig};
use tempfile::TempDir;

pub fn init(cx: &mut App) {
    cx.update_default_global(|registry: &mut DapRegistry, _cx| {
        registry.add_adapter(Arc::from(CodeLldbDebugAdapter::default()));
        registry.add_adapter(Arc::from(PythonDebugAdapter::default()));
        registry.add_adapter(Arc::from(PhpDebugAdapter::default()));
        registry.add_adapter(Arc::from(JsDebugAdapter::default()));
        registry.add_adapter(Arc::from(GoDebugAdapter::default()));
        registry.add_adapter(Arc::from(GdbDebugAdapter));

        #[cfg(any(test, feature = "test-support"))]
        {
            registry.add_adapter(Arc::from(dap::FakeAdapter {}));
        }
    })
}

#[cfg(feature = "update-schemas")]
#[derive(Clone)]
pub struct UpdateSchemasDapDelegate {
    client: std::sync::Arc<reqwest_client::ReqwestClient>,
    fs: std::sync::Arc<fs::RealFs>,
    executor: gpui::BackgroundExecutor,
}

#[cfg(feature = "update-schemas")]
impl UpdateSchemasDapDelegate {
    pub fn new() -> Self {
        let executor = gpui::background_executor();
        // FIXME
        let client = Arc::new(reqwest_client::ReqwestClient::user_agent("Cole").unwrap());
        let fs = Arc::new(fs::RealFs::new(None, executor.clone()));
        Self {
            client,
            fs,
            executor,
        }
    }
}

#[cfg(feature = "update-schemas")]
#[async_trait]
impl dap::adapters::DapDelegate for UpdateSchemasDapDelegate {
    fn worktree_id(&self) -> settings::WorktreeId {
        unreachable!()
    }
    fn worktree_root_path(&self) -> &std::path::Path {
        unreachable!()
    }
    fn http_client(&self) -> Arc<dyn dap::adapters::HttpClient> {
        self.client.clone()
    }
    fn node_runtime(&self) -> node_runtime::NodeRuntime {
        unreachable!()
    }
    fn toolchain_store(&self) -> Arc<dyn language::LanguageToolchainStore> {
        unreachable!()
    }
    fn fs(&self) -> Arc<dyn fs::Fs> {
        self.fs.clone()
    }
    fn output_to_console(&self, msg: String) {
        eprintln!("{msg}")
    }
    async fn which(&self, _command: &std::ffi::OsStr) -> Option<std::path::PathBuf> {
        unreachable!()
    }
    async fn read_text_file(&self, _path: std::path::PathBuf) -> Result<String> {
        unreachable!()
    }
    async fn shell_env(&self) -> collections::HashMap<String, String> {
        unreachable!()
    }
}

#[cfg(feature = "update-schemas")]
#[derive(Debug, Serialize, Deserialize)]
struct PackageJsonConfigurationAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    launch: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    attach: Option<serde_json::Value>,
}

#[cfg(feature = "update-schemas")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageJsonDebugger {
    r#type: String,
    configuration_attributes: PackageJsonConfigurationAttributes,
}

#[cfg(feature = "update-schemas")]
#[derive(Debug, Serialize, Deserialize)]
struct PackageJsonContributes {
    debuggers: Vec<PackageJsonDebugger>,
}

#[cfg(feature = "update-schemas")]
#[derive(Debug, Serialize, Deserialize)]
struct PackageJson {
    contributes: PackageJsonContributes,
}

fn get_vsix_package_json(
    temp_dir: &TempDir,
    repo: &str,
    asset_name: impl FnOnce(&GithubRelease) -> anyhow::Result<String>,
    delegate: UpdateSchemasDapDelegate,
) -> anyhow::Result<(String, Option<String>)> {
    let temp_dir = std::fs::canonicalize(temp_dir.path())?;
    let fs = delegate.fs.clone();
    let client = delegate.client.clone();
    let executor = delegate.executor.clone();

    executor.block(async move {
        let release = adapters::latest_github_release(repo, true, false, client.clone()).await?;

        let asset_name = asset_name(&release)?;
        let version = AdapterVersion {
            tag_name: release.tag_name,
            url: release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .with_context(|| format!("no asset found matching {asset_name:?}"))?
                .browser_download_url
                .clone(),
        };

        let path = adapters::download_adapter_from_github(
            "schemas",
            version,
            DownloadedFileType::Vsix,
            &temp_dir,
            &delegate,
        )
        .await?;
        let package_json = fs
            .load(&path.join("extension").join("package.json"))
            .await?;
        let package_nls_json = fs
            .load(&path.join("extension").join("package.nls.json"))
            .await
            .ok();
        anyhow::Ok((package_json, package_nls_json))
    })
}

fn parse_package_json(
    package_json: String,
    package_nls_json: Option<String>,
) -> anyhow::Result<PackageJson> {
    let package_nls_json = package_nls_json
        .map(|package_nls_json| {
            let package_nls_json =
                serde_json::from_str::<HashMap<String, serde_json::Value>>(&package_nls_json)?;
            let package_nls_json = package_nls_json
                .into_iter()
                .filter_map(|(k, v)| {
                    let v = v.as_str()?;
                    Some((k, v.to_owned()))
                })
                .collect();
            anyhow::Ok(package_nls_json)
        })
        .transpose()?
        .unwrap_or_default();

    let package_json: serde_json::Value = serde_json::from_str(&package_json)?;

    struct Replacer {
        package_nls_json: HashMap<String, String>,
        env: EnvVariableReplacer,
    }

    impl Replacer {
        fn replace(&self, input: serde_json::Value) -> serde_json::Value {
            match input {
                serde_json::Value::String(s) => {
                    if s.starts_with("%") && s.ends_with("%") {
                        self.package_nls_json
                            .get(s.trim_matches('%'))
                            .map(|s| s.as_str().into())
                            .unwrap_or("(missing)".into())
                    } else {
                        self.env.replace(&s).into()
                    }
                }
                serde_json::Value::Array(arr) => {
                    serde_json::Value::Array(arr.into_iter().map(|v| self.replace(v)).collect())
                }
                serde_json::Value::Object(obj) => serde_json::Value::Object(
                    obj.into_iter().map(|(k, v)| (k, self.replace(v))).collect(),
                ),
                _ => input,
            }
        }
    }

    let env = EnvVariableReplacer::new(HashMap::from_iter([(
        "workspaceFolder".to_owned(),
        VariableName::WorktreeRoot.to_string(),
    )]));
    let replacer = Replacer {
        env,
        package_nls_json,
    };
    let package_json = replacer.replace(package_json);

    let package_json: PackageJson = serde_json::from_value(package_json)?;
    Ok(package_json)
}

fn schema_for_configuration_attributes(
    attrs: PackageJsonConfigurationAttributes,
) -> serde_json::Value {
    let conjuncts = attrs
        .launch
        .map(|schema| ("launch", schema))
        .into_iter()
        .chain(attrs.attach.map(|schema| ("attach", schema)))
        .map(|(request, schema)| {
            json!({
                "if": {
                    "properties": {
                        "request": {
                            "const": request
                        }
                    },
                    "required": ["request"]
                },
                "then": schema
            })
        })
        .collect::<Vec<_>>();

    json!({
        "allOf": conjuncts
    })
}

pub struct TypeScriptLspAdapter;

impl TypeScriptLspAdapter {
    const BIN_PATH: &'static str =
        "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";
}

impl super::LspAdapter for TypeScriptLspAdapter {
    fn name(&self) -> &'static str {
        "typescript-language-server"
    }

    fn server_args(&self) -> &[&str] {
        &["--stdio"]
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<LspBinaryVersion>> {
        async move {
            #[derive(Deserialize)]
            struct NpmInfo {
                versions: Vec<String>,
            }

            let output = smol::process::Command::new("npm")
                .args(["info", "vscode-json-languageserver", "--json"])
                .output()
                .await?;
            if !output.status.success() {
                Err(anyhow!("failed to execute npm info"))?;
            }
            let mut info: NpmInfo = serde_json::from_slice(&output.stdout)?;

            Ok(LspBinaryVersion {
                name: info
                    .versions
                    .pop()
                    .ok_or_else(|| anyhow!("no versions found in npm info"))?,
                url: Default::default(),
            })
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: LspBinaryVersion,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        async move {
            let version_dir = container_dir.join(&version.name);
            fs::create_dir_all(&version_dir)
                .await
                .context("failed to create version directory")?;
            let binary_path = version_dir.join(Self::BIN_PATH);

            if fs::metadata(&binary_path).await.is_err() {
                let output = smol::process::Command::new("npm")
                    .current_dir(&version_dir)
                    .arg("install")
                    .arg(format!("vscode-json-languageserver@{}", version.name))
                    .output()
                    .await
                    .context("failed to run npm install")?;
                if !output.status.success() {
                    Err(anyhow!("failed to install vscode-json-languageserver"))?;
                }

                if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
                    while let Some(entry) = entries.next().await {
                        if let Some(entry) = entry.log_err() {
                            let entry_path = entry.path();
                            if entry_path.as_path() != version_dir {
                                fs::remove_dir_all(&entry_path).await.log_err();
                            }
                        }
                    }
                }
            }

            Ok(binary_path)
        }
        .boxed()
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        async move {
            let mut last_version_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_version_dir = Some(entry.path());
                }
            }
            let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let bin_path = last_version_dir.join(Self::BIN_PATH);
            if bin_path.exists() {
                Ok(bin_path)
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    last_version_dir
                ))
            }
        }
        .log_err()
        .boxed()
    }

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }
}

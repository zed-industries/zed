use crate::WasmState;
use async_trait::async_trait;

#[async_trait]
impl LanguageServerExtensionImports for WasmState {
    async fn npm_package_latest_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<String, String>> {
        async fn inner(this: &mut WasmState, package_name: String) -> anyhow::Result<String> {
            this.node_runtime
                .npm_package_latest_version(&package_name)
                .await
        }

        Ok(inner(self, package_name)
            .await
            .map_err(|err| err.to_string()))
    }

    async fn latest_github_release(
        &mut self,
        repo: String,
        options: GithubReleaseOptions,
    ) -> wasmtime::Result<Result<GithubRelease, String>> {
        async fn inner(
            this: &mut WasmState,
            repo: String,
            options: GithubReleaseOptions,
        ) -> anyhow::Result<GithubRelease> {
            let release = util::github::latest_github_release(
                &repo,
                options.require_assets,
                options.pre_release,
                this.http_client.clone(),
            )
            .await?;
            Ok(GithubRelease {
                version: release.tag_name,
                assets: release
                    .assets
                    .into_iter()
                    .map(|asset| GithubReleaseAsset {
                        name: asset.name,
                        download_url: asset.browser_download_url,
                    })
                    .collect(),
            })
        }

        Ok(inner(self, repo, options).await.map_err(|e| e.to_string()))
    }
}

wasmtime::component::bindgen!({
    async: true,
});

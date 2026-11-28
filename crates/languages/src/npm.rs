use anyhow::Result;
use gpui::AsyncApp;
use language::LspAdapter;
use node_runtime::{
    ExecutionBillRoot, InstallNpmPackages, InstalledNpmPackageVersion, LatestNpmPackageVersion,
    NodeBinaryPath, NodeRuntime, ResourceExecutionBill, RunNpmSubcommand, VersionStrategy,
};
use semver::{Version, VersionReq};
use std::{
    path::{Path, PathBuf},
    process::Output,
};
use util::ResultExt;

pub(crate) struct NpmInstaller {
    node: NodeRuntime,
    bill: ResourceExecutionBill,
    cx: AsyncApp,
}

impl NpmInstaller {
    pub fn new(node: &NodeRuntime, adapter: &impl LspAdapter, cx: AsyncApp) -> Self {
        Self {
            node: node.clone(),
            bill: ResourceExecutionBill {
                root: ExecutionBillRoot::LanguageServer {
                    name: adapter.name().0,
                },
                reasoning: vec!["Set up the language server".into()],
            },
            cx,
        }
    }

    pub async fn binary_path(&self) -> Result<PathBuf> {
        self.node
            .execute_command(NodeBinaryPath, self.bill.clone(), &mut self.cx.clone())
            .await?
    }

    pub async fn npm_package_latest_version(&self, package: &str) -> Result<Version> {
        self.npm_package_latest_version_with_requirement(package, None)
            .await
    }

    pub async fn npm_package_latest_version_with_requirement(
        &self,
        package: &str,
        requirement: Option<&VersionReq>,
    ) -> Result<Version> {
        self.node
            .execute_command(
                LatestNpmPackageVersion {
                    package: package.into(),
                    requirement: requirement.cloned(),
                },
                self.bill.clone(),
                &mut self.cx.clone(),
            )
            .await?
    }

    pub async fn npm_install_latest_packages(
        &self,
        directory: &Path,
        packages: &[&str],
    ) -> Result<()> {
        let packages = packages
            .iter()
            .map(|package| (*package, "latest"))
            .collect::<Vec<_>>();
        self.npm_install_packages(directory, &packages).await
    }

    pub async fn npm_install_packages(
        &self,
        directory: &Path,
        packages: &[(&str, &str)],
    ) -> Result<()> {
        self.node
            .execute_command(
                InstallNpmPackages {
                    directory: directory.into(),
                    packages: packages
                        .iter()
                        .map(|(name, version)| ((*name).into(), (*version).into()))
                        .collect(),
                },
                self.bill.clone(),
                &mut self.cx.clone(),
            )
            .await?
    }

    pub async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        self.node
            .execute_command(
                RunNpmSubcommand {
                    directory: directory.map(Path::to_path_buf),
                    subcommand: subcommand.into(),
                    args: args.iter().map(|argument| (*argument).into()).collect(),
                },
                self.bill.clone(),
                &mut self.cx.clone(),
            )
            .await?
    }

    pub async fn should_install_npm_package(
        &self,
        package: &str,
        executable: &Path,
        directory: &Path,
        strategy: VersionStrategy<'_>,
    ) -> Result<bool> {
        let installed = self
            .node
            .execute_command(
                InstalledNpmPackageVersion {
                    directory: directory.into(),
                    package: package.into(),
                },
                self.bill.clone(),
                &mut self.cx.clone(),
            )
            .await?;
        let Some(installed) = installed.log_err().flatten() else {
            return Ok(true);
        };
        if smol::fs::metadata(executable).await.is_err() {
            return Ok(true);
        }
        Ok(match strategy {
            VersionStrategy::Pin(version) => installed != *version,
            VersionStrategy::Latest(version) => installed < *version,
        })
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use managed_resources::{ManagedResourceStore, ResourcePermission, ResourcePermissionRequest};
    use node_runtime::NodeBinaryOptions;
    use std::{cell::Cell, fs, os::unix::fs::PermissionsExt, rc::Rc, sync::Arc};

    #[gpui::test]
    async fn npm_installer_preserves_permissions_and_version_rules(cx: &mut TestAppContext) {
        // Node discovery resets a cache beneath the global data directory.
        if std::env::var_os("ZED_LSP_NPM_TEST_CHILD").is_none() {
            let output =
                std::process::Command::new(std::env::current_exe().expect("test executable"))
                    .args([
                        "--exact",
                        "npm::tests::npm_installer_preserves_permissions_and_version_rules",
                        "--nocapture",
                    ])
                    .env("ZED_LSP_NPM_TEST_CHILD", "1")
                    .env("ITERATIONS", "1")
                    .output()
                    .expect("isolated test");
            assert!(output.status.success(), "{output:?}");
            return;
        }
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("runtime directory");
        paths::set_custom_data_dir(&directory.path().join("data").to_string_lossy());
        let node = directory.path().join("node");
        let npm = directory.path().join("npm");
        fs::write(&node, "#!/bin/sh\necho v22.0.0\n").expect("node fixture");
        fs::write(&npm, "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$0.args\"\n").expect("npm fixture");
        for binary in [&node, &npm] {
            fs::set_permissions(binary, fs::Permissions::from_mode(0o755)).expect("executable");
        }
        let package = directory.path().join("node_modules/example");
        fs::create_dir_all(&package).expect("package directory");
        let executable = package.join("server.js");
        fs::write(&executable, "").expect("server");
        fs::write(package.join("package.json"), r#"{"version":"3.1.4"}"#).expect("metadata");
        let allow = Rc::new(Cell::new(false));
        let store = cx.new(|_| {
            let allow = allow.clone();
            ManagedResourceStore::new(Default::default(), move |request, _| {
                let ResourcePermissionRequest::ExecuteCommand { bill, .. } = request else {
                    panic!("configured Node should not need management");
                };
                assert_eq!(
                    bill.root,
                    ExecutionBillRoot::LanguageServer {
                        name: "vscode-css-language-server".into()
                    }
                );
                gpui::Task::ready(if allow.get() {
                    ResourcePermission::Allowed
                } else {
                    ResourcePermission::Denied
                })
            })
        });
        let runtime = NodeRuntime::new(
            Arc::new(http_client::BlockedHttpClient),
            None,
            watch::channel(Some(NodeBinaryOptions {
                use_paths: Some((node.clone(), npm.clone())),
                ..Default::default()
            }))
            .1,
        )
        .with_managed_resources(store);
        let adapter = crate::css::CssLspAdapter::new(runtime.clone());
        let installer = NpmInstaller::new(&runtime, &adapter, cx.to_async());
        let older = Version::new(3, 0, 0);
        assert!(
            installer
                .should_install_npm_package(
                    "example",
                    &executable,
                    directory.path(),
                    VersionStrategy::Latest(&older)
                )
                .await
                .is_err(),
            "denial must not become a cache miss"
        );
        assert!(
            installer
                .npm_install_packages(directory.path(), &[("example", "3.0.0")])
                .await
                .is_err()
        );
        assert!(!directory.path().join("npm.args").exists());
        allow.set(true);
        assert_eq!(
            installer.binary_path().await.expect("configured binary"),
            node
        );
        assert!(
            !installer
                .should_install_npm_package(
                    "example",
                    &executable,
                    directory.path(),
                    VersionStrategy::Latest(&older)
                )
                .await
                .expect("newer installed")
        );
        assert!(
            installer
                .should_install_npm_package(
                    "example",
                    &executable,
                    directory.path(),
                    VersionStrategy::Pin(&older)
                )
                .await
                .expect("pin requires downgrade")
        );
        assert!(
            !installer
                .should_install_npm_package(
                    "example",
                    &executable,
                    directory.path(),
                    VersionStrategy::Pin(&Version::new(3, 1, 4))
                )
                .await
                .expect("matching pin")
        );
        installer
            .npm_install_packages(directory.path(), &[("example", "3.0.0")])
            .await
            .expect("install");
        let arguments =
            fs::read_to_string(directory.path().join("npm.args")).expect("npm arguments");
        assert!(
            arguments
                .lines()
                .any(|argument| argument == "example@3.0.0")
        );
        fs::write(
            &npm,
            r#"#!/bin/sh
case " $* " in
  *" info "*) echo '{"dist-tags":{"latest":"7.0.0"},"versions":["6.2.0","7.0.0"]}' ;;
  *) echo null ;;
esac
"#,
        )
        .expect("registry fixture");
        assert_eq!(
            installer
                .npm_package_latest_version("typescript")
                .await
                .expect("latest version"),
            Version::new(7, 0, 0)
        );
        assert_eq!(
            installer
                .npm_package_latest_version_with_requirement(
                    "typescript",
                    Some(&VersionReq::parse("<7").expect("requirement"))
                )
                .await
                .expect("compatible version"),
            Version::new(6, 2, 0)
        );
        fs::write(package.join("package.json"), "invalid").expect("broken metadata");
        assert!(
            installer
                .should_install_npm_package(
                    "example",
                    &executable,
                    directory.path(),
                    VersionStrategy::Latest(&older)
                )
                .await
                .expect("bad metadata needs reinstall")
        );
    }
}

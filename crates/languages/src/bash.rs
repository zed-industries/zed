use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate, LspInstaller, Toolchain};
use lsp::LanguageServerBinary;
use node_runtime::{NodeRuntime, VersionStrategy};
use project::ContextProviderWithTasks;
use semver::Version;
use std::{future::Future, path::PathBuf, sync::Arc, vec};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::{ResultExt, maybe};

pub(super) fn bash_task_context() -> ContextProviderWithTasks {
    ContextProviderWithTasks::new(TaskTemplates(vec![
        TaskTemplate {
            label: "execute selection".to_owned(),
            command: VariableName::SelectedText.template_value(),
            ..TaskTemplate::default()
        },
        TaskTemplate {
            label: format!("run '{}'", VariableName::File.template_value()),
            command: VariableName::File.template_value(),
            tags: vec!["bash-script".to_owned()],
            ..TaskTemplate::default()
        },
    ]))
}

pub struct BashLspAdapter {
    node: NodeRuntime,
}

impl BashLspAdapter {
    const PACKAGE_NAME: &str = "bash-language-server";
    const NODE_MODULE_RELATIVE_SERVER_PATH: &str = "bash-language-server/out/cli.js";

    pub fn new(node: NodeRuntime) -> Self {
        Self { node }
    }

    async fn get_cached_server_binary(
        container_dir: PathBuf,
        env: HashMap<String, String>,
        node: &NodeRuntime,
    ) -> Option<lsp::LanguageServerBinary> {
        maybe!(async {
            let server_path = container_dir
                .join("node_modules")
                .join(Self::NODE_MODULE_RELATIVE_SERVER_PATH);
            anyhow::ensure!(
                server_path.exists(),
                "missing executable in directory {server_path:?}"
            );
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: Some(env),
                arguments: vec![server_path.into(), "start".into()],
            })
        })
        .await
        .log_err()
    }
}

impl LspInstaller for BashLspAdapter {
    type BinaryVersion = Version;

    async fn cached_server_binary(
        &self,
        container_dir: std::path::PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<lsp::LanguageServerBinary> {
        let env = delegate.shell_env().await;
        Self::get_cached_server_binary(container_dir, env, &self.node).await
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &gpui::AsyncApp,
    ) -> Option<lsp::LanguageServerBinary> {
        let path = delegate.which(Self::PACKAGE_NAME.as_ref()).await?;
        let env = delegate.shell_env().await;

        Some(LanguageServerBinary {
            path,
            env: Some(env),
            arguments: vec!["start".into()],
        })
    }

    fn check_if_version_installed(
        &self,
        version: &Self::BinaryVersion,
        container_dir: &PathBuf,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Option<lsp::LanguageServerBinary>> + use<> {
        let node = self.node.clone();
        let version = version.clone();
        let container_dir = container_dir.clone();
        let delegate = delegate.clone();

        async move {
            let server_path = container_dir
                .join("node_modules")
                .join(Self::NODE_MODULE_RELATIVE_SERVER_PATH);

            let should_install_language_server = node
                .should_install_npm_package(
                    Self::PACKAGE_NAME,
                    &server_path,
                    &container_dir,
                    VersionStrategy::Latest(&version),
                )
                .await;

            if should_install_language_server {
                None
            } else {
                let env = delegate.shell_env().await;
                Some(LanguageServerBinary {
                    path: node.binary_path().await.ok()?,
                    env: Some(env),
                    arguments: vec![server_path.into(), "start".into()],
                })
            }
        }
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut gpui::AsyncApp,
    ) -> Result<Self::BinaryVersion> {
        self.node
            .npm_package_latest_version(Self::PACKAGE_NAME)
            .await
    }

    fn fetch_server_binary(
        &self,
        latest_version: Self::BinaryVersion,
        container_dir: std::path::PathBuf,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Result<lsp::LanguageServerBinary>> + use<> {
        let node = self.node.clone();
        let delegate = delegate.clone();

        async move {
            let server_path = container_dir
                .join("node_modules")
                .join(Self::NODE_MODULE_RELATIVE_SERVER_PATH);
            let latest_version = latest_version.to_string();

            node.npm_install_packages(
                &container_dir,
                &[(Self::PACKAGE_NAME, latest_version.as_str())],
            )
            .await?;

            let env = delegate.shell_env().await;
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: Some(env),
                arguments: vec![server_path.into(), "start".into()],
            })
        }
    }
}

#[async_trait(?Send)]
impl LspAdapter for BashLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName::new_static(Self::PACKAGE_NAME)
    }
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, BorrowAppContext, Context, TestAppContext};
    use language::{AutoindentMode, Buffer};
    use settings::SettingsStore;
    use std::num::NonZeroU32;
    use unindent::Unindent;
    use util::test::marked_text_offsets;

    #[gpui::test]
    async fn test_bash_autoindent(cx: &mut TestAppContext) {
        cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        let language = crate::language("bash", tree_sitter_bash::LANGUAGE.into());
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2)
                });
            });
        });

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            let expect_indents_to =
                |buffer: &mut Buffer, cx: &mut Context<Buffer>, input: &str, expected: &str| {
                    buffer.edit(
                        [(0..buffer.len(), input)],
                        Some(AutoindentMode::EachLine),
                        cx,
                    );
                    assert_eq!(buffer.text(), expected);
                };

            // Do not indent after shebang
            expect_indents_to(
                &mut buffer,
                cx,
                "#!/usr/bin/env bash\n#",
                "#!/usr/bin/env bash\n#",
            );

            // indent function correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "function name() {\necho \"Hello, World!\"\n}",
                "function name() {\n  echo \"Hello, World!\"\n}",
            );

            // indent if-else correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "if true;then\nfoo\nelse\nbar\nfi",
                "if true;then\n  foo\nelse\n  bar\nfi",
            );

            // indent if-elif-else correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "if true;then\nfoo\nelif true;then\nbar\nelse\nbar\nfi",
                "if true;then\n  foo\nelif true;then\n  bar\nelse\n  bar\nfi",
            );

            // indent case-when-else correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "case $1 in\nfoo) echo \"Hello, World!\";;\n*) echo \"Unknown argument\";;\nesac",
                "case $1 in\n  foo) echo \"Hello, World!\";;\n  *) echo \"Unknown argument\";;\nesac",
            );

            // indent for-loop correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "for i in {1..10};do\nfoo\ndone",
                "for i in {1..10};do\n  foo\ndone",
            );

            // indent while-loop correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "while true; do\nfoo\ndone",
                "while true; do\n  foo\ndone",
            );

            // indent array correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "array=(\n1\n2\n3\n)",
                "array=(\n  1\n  2\n  3\n)",
            );

            // indents non-"function" function correctly
            expect_indents_to(
                &mut buffer,
                cx,
                "foo() {\necho \"Hello, World!\"\n}",
                "foo() {\n  echo \"Hello, World!\"\n}",
            );

            let (input, offsets) = marked_text_offsets(
                &r#"
                if foo; then
                  1ˇ
                else
                  3
                fi
                "#
                .unindent(),
            );

            buffer.edit([(0..buffer.len(), input)], None, cx);
            buffer.edit(
                [(offsets[0]..offsets[0], "\n")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            buffer.edit(
                [(offsets[0] + 3..offsets[0] + 3, "elif")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            let expected = r#"
                if foo; then
                  1
                elif
                else
                  3
                fi
                "#
            .unindent();

            pretty_assertions::assert_eq!(buffer.text(), expected);

            buffer
        });
    }
}

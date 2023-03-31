use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use language::{LanguageServerBinary, LanguageServerName, LspAdapter};
use node_runtime::NodeRuntime;
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::fs::remove_matching;
use util::http::HttpClient;
use util::ResultExt;

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct PythonLspAdapter {
    node: Arc<NodeRuntime>,
}

impl PythonLspAdapter {
    const SERVER_PATH: &'static str = "node_modules/pyright/langserver.index.js";

    pub fn new(node: Arc<NodeRuntime>) -> Self {
        PythonLspAdapter { node }
    }
}

#[async_trait]
impl LspAdapter for PythonLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("pyright".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(self.node.npm_package_latest_version("pyright").await?) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<String>().unwrap();
        let version_dir = container_dir.join(version.as_str());
        fs::create_dir_all(&version_dir)
            .await
            .context("failed to create version directory")?;
        let server_path = version_dir.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages([("pyright", version.as_str())], &version_dir)
                .await?;

            remove_matching(&container_dir, |entry| entry != version_dir).await;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<LanguageServerBinary> {
        (|| async move {
            let mut last_version_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_version_dir = Some(entry.path());
                }
            }
            let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let server_path = last_version_dir.join(Self::SERVER_PATH);
            if server_path.exists() {
                Ok(LanguageServerBinary {
                    path: self.node.binary_path().await?,
                    arguments: server_binary_arguments(&server_path),
                })
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    last_version_dir
                ))
            }
        })()
        .await
        .log_err()
    }

    async fn process_completion(&self, item: &mut lsp::CompletionItem) {
        // Pyright assigns each completion item a `sortText` of the form `XX.YYYY.name`.
        // Where `XX` is the sorting category, `YYYY` is based on most recent usage,
        // and `name` is the symbol name itself.
        //
        // Because the the symbol name is included, there generally are not ties when
        // sorting by the `sortText`, so the symbol's fuzzy match score is not taken
        // into account. Here, we remove the symbol name from the sortText in order
        // to allow our own fuzzy score to be used to break ties.
        //
        // see https://github.com/microsoft/pyright/blob/95ef4e103b9b2f129c9320427e51b73ea7cf78bd/packages/pyright-internal/src/languageService/completionProvider.ts#LL2873
        let Some(sort_text) = &mut item.sort_text else { return };
        let mut parts = sort_text.split('.');
        let Some(first) = parts.next() else { return };
        let Some(second) = parts.next() else { return };
        let Some(_) = parts.next() else { return };
        sort_text.replace_range(first.len() + second.len() + 1.., "");
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let label = &item.label;
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            lsp::CompletionItemKind::METHOD => grammar.highlight_id_for_name("function.method")?,
            lsp::CompletionItemKind::FUNCTION => grammar.highlight_id_for_name("function")?,
            lsp::CompletionItemKind::CLASS => grammar.highlight_id_for_name("type")?,
            lsp::CompletionItemKind::CONSTANT => grammar.highlight_id_for_name("constant")?,
            _ => return None,
        };
        Some(language::CodeLabel {
            text: label.clone(),
            runs: vec![(0..label.len(), highlight_id)],
            filter_range: 0..label.len(),
        })
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("def {}():\n", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CLASS => {
                let text = format!("class {}:", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("{} = 0", name);
                let filter_range = 0..name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(language::CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }
}

#[cfg(test)]
mod tests {
    use gpui::{ModelContext, TestAppContext};
    use language::{AutoindentMode, Buffer};
    use settings::Settings;

    #[gpui::test]
    async fn test_python_autoindent(cx: &mut TestAppContext) {
        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);
        let language =
            crate::languages::language("python", tree_sitter_python::language(), None).await;
        cx.update(|cx| {
            let mut settings = Settings::test(cx);
            settings.editor_overrides.tab_size = Some(2.try_into().unwrap());
            cx.set_global(settings);
        });

        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx).with_language(language, cx);
            let append = |buffer: &mut Buffer, text: &str, cx: &mut ModelContext<Buffer>| {
                let ix = buffer.len();
                buffer.edit([(ix..ix, text)], Some(AutoindentMode::EachLine), cx);
            };

            // indent after "def():"
            append(&mut buffer, "def a():\n", cx);
            assert_eq!(buffer.text(), "def a():\n  ");

            // preserve indent after blank line
            append(&mut buffer, "\n  ", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  ");

            // indent after "if"
            append(&mut buffer, "if a:\n  ", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    ");

            // preserve indent after statement
            append(&mut buffer, "b()\n", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    b()\n    ");

            // preserve indent after statement
            append(&mut buffer, "else", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    b()\n    else");

            // dedent "else""
            append(&mut buffer, ":", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    b()\n  else:");

            // indent lines after else
            append(&mut buffer, "\n", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    "
            );

            // indent after an open paren. the closing  paren is not indented
            // because there is another token before it on the same line.
            append(&mut buffer, "foo(\n1)", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n      1)"
            );

            // dedent the closing paren if it is shifted to the beginning of the line
            let argument_ix = buffer.text().find('1').unwrap();
            buffer.edit(
                [(argument_ix..argument_ix + 1, "")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )"
            );

            // preserve indent after the close paren
            append(&mut buffer, "\n", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )\n    "
            );

            // manually outdent the last line
            let end_whitespace_ix = buffer.len() - 4;
            buffer.edit(
                [(end_whitespace_ix..buffer.len(), "")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )\n"
            );

            // preserve the newly reduced indentation on the next newline
            append(&mut buffer, "\n", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )\n\n"
            );

            // reset to a simple if statement
            buffer.edit([(0..buffer.len(), "if a:\n  b(\n  )")], None, cx);

            // dedent "else" on the line after a closing paren
            append(&mut buffer, "\n  else:\n", cx);
            assert_eq!(buffer.text(), "if a:\n  b(\n  )\nelse:\n  ");

            buffer
        });
    }
}

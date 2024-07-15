use anyhow::Result;
use async_trait::async_trait;
use gpui::AppContext;
use gpui::AsyncAppContext;
use language::{ContextProvider, LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use project::project_settings::ProjectSettings;
use serde_json::Value;
use settings::Settings;
use std::{
    any::Any,
    borrow::Cow,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::ResultExt;

const SERVER_PATH: &str = "node_modules/pyright/langserver.index.js";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct PythonLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl PythonLspAdapter {
    const SERVER_NAME: &'static str = "pyright";

    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        PythonLspAdapter { node }
    }
}

#[async_trait(?Send)]
impl LspAdapter for PythonLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(Self::SERVER_NAME.into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version(Self::SERVER_NAME)
                .await?,
        ) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);
        let package_name = Self::SERVER_NAME;

        let should_install_language_server = self
            .node
            .should_install_npm_package(package_name, &server_path, &container_dir, &latest_version)
            .await;

        if should_install_language_server {
            self.node
                .npm_install_packages(&container_dir, &[(package_name, latest_version.as_str())])
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &*self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &*self.node).await
    }

    async fn process_completions(&self, items: &mut [lsp::CompletionItem]) {
        // Pyright assigns each completion item a `sortText` of the form `XX.YYYY.name`.
        // Where `XX` is the sorting category, `YYYY` is based on most recent usage,
        // and `name` is the symbol name itself.
        //
        // Because the symbol name is included, there generally are not ties when
        // sorting by the `sortText`, so the symbol's fuzzy match score is not taken
        // into account. Here, we remove the symbol name from the sortText in order
        // to allow our own fuzzy score to be used to break ties.
        //
        // see https://github.com/microsoft/pyright/blob/95ef4e103b9b2f129c9320427e51b73ea7cf78bd/packages/pyright-internal/src/languageService/completionProvider.ts#LL2873
        for item in items {
            let Some(sort_text) = &mut item.sort_text else {
                continue;
            };
            let mut parts = sort_text.split('.');
            let Some(first) = parts.next() else { continue };
            let Some(second) = parts.next() else { continue };
            let Some(_) = parts.next() else { continue };
            sort_text.replace_range(first.len() + second.len() + 1.., "");
        }
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

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .lsp
                .get(Self::SERVER_NAME)
                .and_then(|s| s.settings.clone())
                .unwrap_or_default()
        })
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    let server_path = container_dir.join(SERVER_PATH);
    if server_path.exists() {
        Some(LanguageServerBinary {
            path: node.binary_path().await.log_err()?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    } else {
        log::error!("missing executable in directory {:?}", server_path);
        None
    }
}

pub(crate) struct PythonContextProvider;

const PYTHON_UNITTEST_TARGET_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_UNITTEST_TARGET"));

impl ContextProvider for PythonContextProvider {
    fn build_context(
        &self,
        variables: &task::TaskVariables,
        _location: &project::Location,
        _cx: &mut gpui::AppContext,
    ) -> Result<task::TaskVariables> {
        let python_module_name = python_module_name_from_relative_path(
            variables.get(&VariableName::RelativeFile).unwrap_or(""),
        );
        let unittest_class_name =
            variables.get(&VariableName::Custom(Cow::Borrowed("_unittest_class_name")));
        let unittest_method_name = variables.get(&VariableName::Custom(Cow::Borrowed(
            "_unittest_method_name",
        )));

        let unittest_target_str = match (unittest_class_name, unittest_method_name) {
            (Some(class_name), Some(method_name)) => {
                format!("{}.{}.{}", python_module_name, class_name, method_name)
            }
            (Some(class_name), None) => format!("{}.{}", python_module_name, class_name),
            (None, None) => python_module_name,
            (None, Some(_)) => return Ok(task::TaskVariables::default()), // should never happen, a TestCase class is the unit of testing
        };

        let unittest_target = (
            PYTHON_UNITTEST_TARGET_TASK_VARIABLE.clone(),
            unittest_target_str,
        );

        Ok(task::TaskVariables::from_iter([unittest_target]))
    }

    fn associated_tasks(
        &self,
        _: Option<Arc<dyn language::File>>,
        _: &AppContext,
    ) -> Option<TaskTemplates> {
        Some(TaskTemplates(vec![
            TaskTemplate {
                label: "execute selection".to_owned(),
                command: "python3".to_owned(),
                args: vec!["-c".to_owned(), VariableName::SelectedText.template_value()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("run '{}'", VariableName::File.template_value()),
                command: "python3".to_owned(),
                args: vec![VariableName::File.template_value()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("unittest '{}'", VariableName::File.template_value()),
                command: "python3".to_owned(),
                args: vec![
                    "-m".to_owned(),
                    "unittest".to_owned(),
                    VariableName::File.template_value(),
                ],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "unittest $ZED_CUSTOM_PYTHON_UNITTEST_TARGET".to_owned(),
                command: "python3".to_owned(),
                args: vec![
                    "-m".to_owned(),
                    "unittest".to_owned(),
                    "$ZED_CUSTOM_PYTHON_UNITTEST_TARGET".to_owned(),
                ],
                tags: vec![
                    "python-unittest-class".to_owned(),
                    "python-unittest-method".to_owned(),
                ],
                ..TaskTemplate::default()
            },
        ]))
    }
}

fn python_module_name_from_relative_path(relative_path: &str) -> String {
    let path_with_dots = relative_path.replace('/', ".");
    path_with_dots
        .strip_suffix(".py")
        .unwrap_or(&path_with_dots)
        .to_string()
}

#[cfg(test)]
mod tests {
    use gpui::{BorrowAppContext, Context, ModelContext, TestAppContext};
    use language::{language_settings::AllLanguageSettings, AutoindentMode, Buffer};
    use settings::SettingsStore;
    use std::num::NonZeroU32;

    #[gpui::test]
    async fn test_python_autoindent(cx: &mut TestAppContext) {
        cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        let language = crate::language("python", tree_sitter_python::language());
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            language::init(cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |s| {
                    s.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });

        cx.new_model(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);
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

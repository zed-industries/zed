mod codelldb;
mod gdb;
mod go;
mod javascript;
mod python;

#[cfg(test)]
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use codelldb::CodeLldbDebugAdapter;
use dap::{
    DapRegistry,
    adapters::{
        self, AdapterVersion, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName,
    },
    configure_tcp_connection,
};
use gdb::GdbDebugAdapter;
use go::GoDebugAdapter;
use gpui::{App, BorrowAppContext};
use javascript::JsDebugAdapter;
use python::PythonDebugAdapter;
use serde_json::json;
use task::{DebugScenario, ZedDebugConfig};

pub fn init(cx: &mut App) {
    cx.update_default_global(|registry: &mut DapRegistry, _cx| {
        registry.add_adapter(Arc::from(CodeLldbDebugAdapter::default()));
        registry.add_adapter(Arc::from(PythonDebugAdapter::default()));
        registry.add_adapter(Arc::from(JsDebugAdapter::default()));
        registry.add_adapter(Arc::from(GoDebugAdapter::default()));
        registry.add_adapter(Arc::from(GdbDebugAdapter));

        #[cfg(any(test, feature = "test-support"))]
        {
            registry.add_adapter(Arc::from(dap::FakeAdapter {}));
        }
    })
}

#[cfg(test)]
mod test_mocks {
    use super::*;

    pub(crate) struct MockDelegate {
        worktree_root: PathBuf,
        allow_binary_downloads: bool,
        fs: Option<Arc<dyn fs::Fs>>,
    }

    impl MockDelegate {
        pub(crate) fn new() -> Arc<dyn adapters::DapDelegate> {
            Arc::new(Self {
                worktree_root: PathBuf::from("/tmp/test"),
                allow_binary_downloads: true,
                fs: None,
            })
        }

        pub(crate) fn new_with(
            allow_binary_downloads: bool,
            fs: Option<Arc<dyn fs::Fs>>,
        ) -> Arc<dyn adapters::DapDelegate> {
            Arc::new(Self {
                worktree_root: PathBuf::from("/tmp/test"),
                allow_binary_downloads,
                fs,
            })
        }
    }

    #[async_trait::async_trait]
    impl adapters::DapDelegate for MockDelegate {
        fn worktree_id(&self) -> settings::WorktreeId {
            settings::WorktreeId::from_usize(0)
        }

        fn worktree_root_path(&self) -> &std::path::Path {
            &self.worktree_root
        }

        fn http_client(&self) -> Arc<dyn http_client::HttpClient> {
            unimplemented!("Not needed for tests")
        }

        fn node_runtime(&self) -> node_runtime::NodeRuntime {
            unimplemented!("Not needed for tests")
        }

        fn toolchain_store(&self) -> Arc<dyn language::LanguageToolchainStore> {
            unimplemented!("Not needed for tests")
        }

        fn fs(&self) -> Arc<dyn fs::Fs> {
            self.fs
                .clone()
                .expect("`fs` was not provided to MockDelegate but is required for this test")
        }

        fn output_to_console(&self, _msg: String) {}

        async fn which(&self, _command: &std::ffi::OsStr) -> Option<PathBuf> {
            None
        }

        async fn read_text_file(&self, _path: &util::rel_path::RelPath) -> Result<String> {
            Ok(String::new())
        }

        async fn shell_env(&self) -> collections::HashMap<String, String> {
            collections::HashMap::default()
        }

        fn is_headless(&self) -> bool {
            false
        }

        fn allow_binary_downloads(&self) -> bool {
            self.allow_binary_downloads
        }
    }
}

#[cfg(test)]
mod allow_binary_downloads_tests {
    use super::*;
    use fs::FakeFs;
    use language::BinaryDownloadsDisabled;

    #[gpui::test]
    async fn test_go_adapter_errors_when_downloads_disabled_and_no_cache(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs: Arc<dyn fs::Fs> = FakeFs::new(cx.executor());
        let delegate = test_mocks::MockDelegate::new_with(false, Some(fs));
        let adapter = go::GoDebugAdapter::default();
        let err = adapter.install_shim(&delegate).await.unwrap_err();
        assert_eq!(err.is::<BinaryDownloadsDisabled>(), true);
    }

    #[gpui::test]
    async fn test_codelldb_adapter_errors_when_downloads_disabled_and_no_cache(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs: Arc<dyn fs::Fs> = FakeFs::new(cx.executor());
        let delegate = test_mocks::MockDelegate::new_with(false, Some(fs));
        let adapter = codelldb::CodeLldbDebugAdapter::default();
        let task_definition = dap::adapters::DebugTaskDefinition {
            adapter: "CodeLLDB".into(),
            label: "test".into(),
            config: serde_json::json!({}),
            tcp_connection: None,
        };
        let mut async_cx = cx.to_async();
        let err = adapter
            .get_binary(&delegate, &task_definition, None, None, None, &mut async_cx)
            .await
            .unwrap_err();
        assert_eq!(err.is::<BinaryDownloadsDisabled>(), true);
    }
}

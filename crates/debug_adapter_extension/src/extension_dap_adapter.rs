use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use dap::adapters::{
    DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
};
use extension::Extension;
use gpui::AsyncApp;

pub(crate) struct ExtensionDapAdapter {
    extension: Arc<dyn Extension>,
    debug_adapter_name: Arc<str>,
}

impl ExtensionDapAdapter {
    pub(crate) fn new(
        extension: Arc<dyn extension::Extension>,
        debug_adapter_name: Arc<str>,
    ) -> Self {
        Self {
            extension,
            debug_adapter_name,
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for ExtensionDapAdapter {
    fn name(&self) -> DebugAdapterName {
        self.debug_adapter_name.as_ref().into()
    }

    async fn get_binary(
        &self,
        _: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        self.extension
            .get_dap_binary(
                self.debug_adapter_name.clone(),
                config.clone(),
                user_installed_path,
            )
            .await
    }
}

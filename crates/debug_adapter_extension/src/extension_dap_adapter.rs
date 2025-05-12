use std::{path::PathBuf, sync::Arc};

use dap::adapters::DebugAdapter;
use extension::Extension;

pub(crate) struct ExtensionDapAdapter {
    extension: Arc<dyn Extension>,
    debug_adapter_name: Arc<str>,
}

impl ExtensionDapAdapter {
    pub(crate) fn new(extension: Arc<dyn extension::Extension>, debug_adapter_name: Arc<str>) {
        Self {
            extension,
            debug_adapter_name,
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for ExtensionDapAdapter {
    fn name(&self) -> DebugAdapterName {
        self.debug_adapter_name.clone().into()
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        async move {
            self.extension.get_dap_binary(
                self.debug_adapter_name,
                config.clone(),
                user_installed_path.map(|path| path.to_string_lossy().into_owned()),
            )
        }
    }
}

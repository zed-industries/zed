use std::sync::Arc;

use extension::{CustomAction as ExtensionCustomAction, Extension, ExtensionCustomActionProxy};
use gpui::{App, UpdateGlobal};

use crate::{CustomActionProvider, registry::GlobalCustomActionProvider};

pub fn init(cx: &mut App) {
    let proxy = extension::ExtensionHostProxy::default_global(cx);
    let provider = Arc::new(CustomActionProvider::new());
    GlobalCustomActionProvider::set_global(cx, GlobalCustomActionProvider(provider.clone()));
    proxy.register_custom_action_proxy(CustomActionRegistryProxy { provider });
}

struct CustomActionRegistryProxy {
    provider: Arc<CustomActionProvider>,
}

impl ExtensionCustomActionProxy for CustomActionRegistryProxy {
    fn register_custom_action(&self, extension: Arc<dyn Extension>, action: ExtensionCustomAction) {
        let custom_action = crate::CustomAction {
            extension: extension.clone(),
            name: action.name,
            description: action.description,
        };

        self.provider.register(custom_action);
    }

    fn unregister_custom_action(&self, action_name: Arc<str>) {
        // We need to find the extension for this action to unregister it properly
        // Since we don't have the extension context here, we'll iterate through all actions
        let actions = self.provider.all_actions();
        for action in actions {
            if action.name == action_name.as_ref() {
                let extension_id = action.extension.manifest().id.to_string();
                self.provider.unregister(&extension_id, &action_name);
                break;
            }
        }
    }
}

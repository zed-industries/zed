use collections::HashMap;
use std::sync::LazyLock;

/// Maps built-in provider IDs to their corresponding extension IDs.
/// When an extension with this ID is installed, the built-in provider should be hidden.
static BUILTIN_TO_EXTENSION_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        let mut map = HashMap::default();
        map.insert("anthropic", "anthropic");
        map.insert("openai", "openai");
        map.insert("google", "google-ai");
        map.insert("open_router", "open-router");
        map.insert("copilot_chat", "copilot-chat");
        map
    });

/// Returns the extension ID that should hide the given built-in provider.
pub fn extension_for_builtin_provider(provider_id: &str) -> Option<&'static str> {
    BUILTIN_TO_EXTENSION_MAP.get(provider_id).copied()
}

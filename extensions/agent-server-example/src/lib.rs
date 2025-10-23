use zed_extension_api as zed;

struct AgentServerExampleExtension;

impl zed::Extension for AgentServerExampleExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(AgentServerExampleExtension);

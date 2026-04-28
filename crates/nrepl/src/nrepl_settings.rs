use settings::{RegisterSetting, Settings};

/// Settings for configuring the Clojure nREPL client.
#[derive(Clone, Debug, RegisterSetting)]
pub struct NreplSettings {
    /// Default host to use when connecting to an nREPL server.
    ///
    /// Default: "localhost"
    pub default_host: String,
    /// Whether to automatically connect to a discovered nREPL server on
    /// workspace open.
    ///
    /// Default: true
    pub auto_connect: bool,
    /// File name (relative to the workspace root) to read the nREPL port
    /// from for auto-discovery. Common build tools (Leiningen, deps.edn,
    /// shadow-cljs, babashka) write `.nrepl-port` by default.
    ///
    /// Default: ".nrepl-port"
    pub port_file: String,
}

impl NreplSettings {
    pub fn enabled(cx: &gpui::App) -> bool {
        // To avoid a circular dependency between `editor` and `nrepl`, the
        // enable flag lives on `EditorSettings` (mirroring `JupyterSettings`).
        editor::EditorSettings::nrepl_enabled(cx)
    }
}

impl Settings for NreplSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let nrepl = content.nrepl.as_ref().unwrap();

        Self {
            default_host: nrepl.default_host.clone().unwrap(),
            auto_connect: nrepl.auto_connect.unwrap(),
            port_file: nrepl.port_file.clone().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings::SettingsContent;
    use std::rc::Rc;

    /// Round-trips the bundled `default.json` through `NreplSettings::from_settings`.
    /// Catches any field where the default is missing or malformed: the production
    /// path `unwrap()`s the same way, and we'd rather panic in CI than at startup.
    #[test]
    fn default_settings_round_trip() {
        let content: Rc<SettingsContent> =
            settings::parse_json_with_comments(&settings::default_settings()).unwrap();
        let nrepl = NreplSettings::from_settings(&content);

        assert_eq!(nrepl.default_host, "localhost");
        assert!(nrepl.auto_connect);
        assert_eq!(nrepl.port_file, ".nrepl-port");
    }
}

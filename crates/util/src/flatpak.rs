use anyhow::{Context as _, Result};
use collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

/// Metadata for the Flatpak sandbox the process is currently running in.
pub static CURRENT_SANDBOX_METADATA: LazyLock<Option<Metadata>> =
    LazyLock::new(|| match Metadata::load(Path::new("/.flatpak-info")) {
        Ok(result) if result.is_running_sandbox() => Some(result),
        _ => None,
    });

/// Whether the current process is running inside a Flatpak sandbox.
pub fn is_running_in_sandbox() -> bool {
    CURRENT_SANDBOX_METADATA.is_some()
}

/// Parsed data from a Flatpak metadata file.
///
/// These describe Flatpak app metadata and sandbox permissions and paths. The file uses
/// the same INI-style format used for XDG desktop files. See `man flatpak-metadata` for
/// further details.
///
/// Typically you want the [CURRENT_SANDBOX_METADATA].
pub struct Metadata {
    groups: HashMap<String, HashMap<String, String>>,
}

impl Metadata {
    /// Load metadata from the given file.
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.to_string_lossy()))?;
        Ok(Self::parse(&contents))
    }

    /// Parse metadata from the given string.
    ///
    /// Parsing errors from invalid lines are ignored.
    pub fn parse(contents: &str) -> Self {
        let mut groups: HashMap<String, HashMap<String, String>> = HashMap::default();
        let mut current_group: Option<String> = None;
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(group) = line
                .strip_prefix('[')
                .and_then(|line| line.strip_suffix(']'))
            {
                current_group = Some(group.to_string());
            } else if let Some((key, value)) = line.split_once('=')
                && let Some(group) = &current_group
            {
                groups
                    .entry(group.clone())
                    .or_default()
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Self { groups }
    }

    /// Get the value of a particular `key` in a `group`.
    ///
    /// Returns [None] if the key is not set.
    pub fn get(&self, group: &str, key: &str) -> Option<&str> {
        self.groups
            .get(group)
            .and_then(|group| group.get(key))
            .map(String::as_str)
    }

    /// Whether this Metadata describes an actively running Flatpak sandbox.
    pub fn is_running_sandbox(&self) -> bool {
        self.groups.contains_key("Instance")
    }

    /// Whether this Metadata grants permission to talk to a particular D-Bus `address`.
    pub fn can_access_session_bus_addr(&self, address: &str) -> bool {
        // If the session bus isn't proxied by the sandbox, we have full access to
        // the session bus and don't need to check the filter policy.
        if self.get("Instance", "session-bus-proxy").is_none() {
            return true;
        }

        // Check if we have a filter policy. A missing policy means no access permitted.
        let Some(policy) = self.groups.get("Session Bus Policy") else {
            return false;
        };

        // Scan the allowlist policy for a matching entry
        policy.iter().any(|(pattern, access)| {
            matches!(access.as_str(), "talk" | "own")
                && (pattern == address
                    || pattern.strip_suffix(".*").is_some_and(|filter| {
                        address == filter || address.starts_with(&format!("{filter}."))
                    }))
        })
    }

    /// Whether this Metadata grants permission to spawn commands on the host system.
    ///
    /// If true, commands like `flatpak-spawn --host` will work.
    pub fn can_spawn_on_host(&self) -> bool {
        self.can_access_session_bus_addr("org.freedesktop.Flatpak")
    }

    /// Host-side path for files bundled with the Flatpak.
    ///
    /// This is the directory where the installed Flatpak is unpacked. Inside the
    /// sandbox, this directory is typically mounted read-only as `/app`.
    pub fn app_host_path(&self) -> Result<&str> {
        self.get("Instance", "app-path")
            .with_context(|| "app-path not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_APPLICATION: &str = "\
[Application]
name=dev.zed.Zed
";
    const EXAMPLE_INSTANCE: &str = "\
[Instance]
instance-id=12345
# some comment
app-path=/var/lib/flatpak/app/dev.zed.Zed/current/active/files/
";
    const EXAMPLE_SESSION_BUS_POLICY: &str = "\
[Session Bus Policy]
org.freedesktop.Flatpak=talk
ca.desrt.dconf=talk
";

    #[test]
    fn parses_groups_and_keys() {
        let example = format!(
            "{}{}{}",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE, EXAMPLE_SESSION_BUS_POLICY
        );
        let info = Metadata::parse(example.as_ref());
        assert_eq!(info.get("Application", "name"), Some("dev.zed.Zed"));
        assert_eq!(
            info.get("Instance", "app-path"),
            Some("/var/lib/flatpak/app/dev.zed.Zed/current/active/files/")
        );
        assert_eq!(
            info.get("Session Bus Policy", "org.freedesktop.Flatpak"),
            Some("talk")
        );
        // Keys are scoped to their group; a missing group or key yields `None`.
        assert_eq!(info.get("Instance", "name"), None);
        assert_eq!(info.get("Missing", "name"), None);
    }

    #[test]
    fn detects_can_spawn_on_host() {
        // An open session bus socket means we have full access
        let example = format!(
            "{}{}{}",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE, EXAMPLE_SESSION_BUS_POLICY
        );
        assert!(Metadata::parse(example.as_ref()).can_spawn_on_host());

        // Proxied session bus access is insufficient by default
        let example = format!(
            "{}{}session-bus-proxy=true",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE
        );
        assert!(!Metadata::parse(example.as_ref()).can_spawn_on_host());

        // See-only access is insufficient
        let example = format!(
            "{}{}session-bus-proxy=true
[Session Bus Policy]
org.freedesktop.Flatpak=see
",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE
        );
        assert!(!Metadata::parse(example.as_ref()).can_spawn_on_host());

        // Talk permissions for the specific addr is sufficient
        let example = format!(
            "{}{}session-bus-proxy=true\n
[Session Bus Policy]
org.freedesktop.Flatpak=talk
",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE
        );
        assert!(Metadata::parse(example.as_ref()).can_spawn_on_host());

        // Owning permissions for the specific addr is also sufficient
        let example = format!(
            "{}{}session-bus-proxy=true
[Session Bus Policy]
org.freedesktop.Flatpak=own
",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE
        );
        assert!(Metadata::parse(example.as_ref()).can_spawn_on_host());

        // Talk permissions for a prefix are valid and sufficient
        let example = format!(
            "{}{}session-bus-proxy=true
[Session Bus Policy]
org.freedesktop.*=talk
",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE
        );
        assert!(Metadata::parse(example.as_ref()).can_spawn_on_host());
        let example = format!(
            "{}{}session-bus-proxy=true
[Session Bus Policy]
org.free.*=talk
",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE
        );
        assert!(!Metadata::parse(example.as_ref()).can_spawn_on_host());
        let example = format!(
            "{}{}session-bus-proxy=true
[Session Bus Policy]
org.freedesktop.Flatpak.*=talk
",
            EXAMPLE_APPLICATION, EXAMPLE_INSTANCE
        );
        assert!(Metadata::parse(example.as_ref()).can_spawn_on_host());
    }
}

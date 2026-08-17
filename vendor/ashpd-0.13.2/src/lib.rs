#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bilelmoussaoui/ashpd/main/ashpd-demo/data/icons/com.belmoussaoui.ashpd.demo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/bilelmoussaoui/ashpd/main/ashpd-demo/data/icons/com.belmoussaoui.ashpd.demo-symbolic.svg"
)]
#![doc = include_str!("../README.md")]
#[cfg(all(all(feature = "tokio", feature = "async-io"), not(doc)))]
compile_error!("You can't enable both async-io & tokio features at once");
#[cfg(all(not(feature = "tokio"), not(feature = "async-io"), not(doc)))]
compile_error!("Either the `async-io` or the `tokio` feature has to be enabled");

/// Alias for a [`Result`] with the error type `ashpd::Error`.
pub type Result<T> = std::result::Result<T, Error>;

static IS_SANDBOXED: OnceLock<bool> = OnceLock::new();

mod activation_token;
/// Interact with the user's desktop such as taking a screenshot, setting a
/// background or querying the user's location.
pub mod desktop;
/// Interact with the documents store or transfer files across apps.
#[cfg(feature = "documents")]
#[cfg_attr(docsrs, doc(cfg(feature = "documents")))]
pub mod documents;
mod error;
mod window_identifier;

pub use self::{activation_token::ActivationToken, window_identifier::WindowIdentifier};
mod app_id;
mod registry;
mod uri;
pub use self::{
    app_id::AppID,
    registry::{register_host_app, register_host_app_with_connection},
    uri::Uri,
};
mod file_path;
pub use self::file_path::FilePath;

mod proxy;

pub use self::window_identifier::WindowIdentifierType;
#[cfg(feature = "backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
#[allow(missing_docs)]
/// Build your custom portals backend.
pub mod backend;
/// Spawn commands outside the sandbox or monitor if the running application has
/// received an update & install it.
#[cfg(feature = "flatpak")]
#[cfg_attr(docsrs, doc(cfg(feature = "flatpak")))]
pub mod flatpak;
mod helpers;
use std::sync::OnceLock;

#[cfg(feature = "backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
pub use async_trait;
pub use enumflags2;
pub use zbus::{self, zvariant};

/// Check whether the application is running inside a sandbox.
///
/// The function checks whether the file `/.flatpak-info` exists, or if the app
/// is running as a snap, or if the environment variable `GTK_USE_PORTAL` is set
/// to `1`. As the return value of this function will not change during the
/// runtime of a program; it is cached for future calls.
pub fn is_sandboxed() -> bool {
    if let Some(cached_value) = IS_SANDBOXED.get() {
        return *cached_value;
    }
    let new_value = crate::helpers::is_flatpak() || crate::helpers::is_snap();

    *IS_SANDBOXED.get_or_init(|| new_value)
}

pub use self::error::{Error, PortalError};

mod sealed {
    /// Use as a supertrait for public traits that users should not be able to
    /// implement
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

/// Process ID.
///
/// Matches the type used in std.
pub type Pid = u32;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::PathBuf};

    use quick_xml::{Reader, events::Event};

    // Helper to convert PascalCase to snake_case
    fn pascal_to_snake_case(s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }
        result
    }

    fn extract_names_from_xml(xml_content: &str) -> HashMap<String, Vec<String>> {
        let mut interfaces = HashMap::new();
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut current_interface_name = String::new();
        let mut current_names = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    // Handle both Start and Empty events
                    if let Some(Ok(attr)) = e
                        .attributes()
                        .find(|a| a.as_ref().map_or(false, |a| a.key.as_ref() == b"name"))
                    {
                        if let Ok(value) = attr.decode_and_unescape_value(reader.decoder()) {
                            match e.name().as_ref() {
                                b"interface" => {
                                    current_interface_name = value.to_string();
                                    current_names.clear();
                                }
                                b"method" | b"property" | b"signal" => {
                                    if value != "version" {
                                        current_names.push(value.to_string());
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"interface" {
                        interfaces.insert(current_interface_name.clone(), current_names.clone());
                    }
                }
                _ => (),
            }
            buf.clear();
        }
        interfaces
    }

    struct TestConfig {
        interfaces_dir: PathBuf,
        rust_src_prefix: &'static str,
        rust_file_mappings: HashMap<&'static str, &'static str>,
        ignored_interfaces: &'static [&'static str],
        interface_prefix: &'static str,
    }

    fn check_doc_aliases_for_config(config: TestConfig) {
        assert!(
            config.interfaces_dir.exists(),
            "Interfaces directory not found at {}",
            config.interfaces_dir.display()
        );

        let entries =
            fs::read_dir(&config.interfaces_dir).expect("Failed to read interfaces directory");

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "xml") {
                println!("Checking XML file: {}", path.display());

                let xml_content = fs::read_to_string(&path).unwrap();
                let interfaces = extract_names_from_xml(&xml_content);

                for (interface_name, names_to_check) in interfaces {
                    // Map the D-Bus interface name to the corresponding Rust file path
                    let interface_name_suffix = interface_name
                        .strip_prefix(config.interface_prefix)
                        .expect("Interface name does not have the expected prefix.");

                    if config.ignored_interfaces.contains(&interface_name_suffix) {
                        continue;
                    }

                    let rust_path = if let Some(mapped_path) =
                        config.rust_file_mappings.get(interface_name.as_str())
                    {
                        PathBuf::from(mapped_path)
                    } else {
                        let rust_file_name_snake = pascal_to_snake_case(interface_name_suffix);
                        PathBuf::from(format!(
                            "{}{}.rs",
                            config.rust_src_prefix, rust_file_name_snake
                        ))
                    };

                    // Check if the Rust file exists
                    assert!(
                        rust_path.exists(),
                        "Corresponding Rust file not found for interface '{}' at {}",
                        interface_name,
                        rust_path.display()
                    );

                    // Read the Rust file content
                    let rust_content = fs::read_to_string(&rust_path).unwrap();

                    // Assert that each name has a corresponding doc alias
                    for name in &names_to_check {
                        let alias_str = format!("#[doc(alias = \"{}\")]", name);
                        assert!(
                            rust_content.contains(&alias_str),
                            "Missing doc alias '{}' for interface '{}' in file {}",
                            alias_str,
                            interface_name,
                            rust_path.display()
                        );
                    }
                }
            }
        }
    }

    #[cfg(feature = "backend")]
    #[test]
    fn all_interfaces_have_backend_implementations() {
        let rust_file_mappings: HashMap<&str, &str> = HashMap::from([(
            "org.freedesktop.impl.portal.ScreenCast",
            "src/backend/screencast.rs",
        )]);

        const IGNORED_BACKEND_PORTALS: &[&str; 7] = &[
            "Clipboard",
            "DynamicLauncher",
            "GlobalShortcuts",
            "Inhibit",
            "InputCapture",
            "Notification",
            "RemoteDesktop",
        ];

        let config = TestConfig {
            interfaces_dir: PathBuf::from("./../interfaces/backend"),
            rust_src_prefix: "src/backend/",
            rust_file_mappings,
            ignored_interfaces: IGNORED_BACKEND_PORTALS,
            interface_prefix: "org.freedesktop.impl.portal.",
        };

        check_doc_aliases_for_config(config);
    }

    #[test]
    fn all_interfaces_have_implementations() {
        let rust_file_mappings: HashMap<&str, &str> = HashMap::from([
            (
                "org.freedesktop.portal.ScreenCast",
                "src/desktop/screencast.rs",
            ),
            ("org.freedesktop.portal.OpenURI", "src/desktop/open_uri.rs"),
            (
                "org.freedesktop.portal.FileTransfer",
                "src/documents/file_transfer.rs",
            ),
            ("org.freedesktop.portal.Documents", "src/documents/mod.rs"),
            ("org.freedesktop.portal.Flatpak", "src/flatpak/mod.rs"),
            (
                "org.freedesktop.portal.Flatpak.UpdateMonitor",
                "src/flatpak/update_monitor.rs",
            ),
        ]);

        const NO_IGNORED_INTERFACES: &[&str; 0] = &[];

        let config = TestConfig {
            interfaces_dir: PathBuf::from("./../interfaces"),
            rust_src_prefix: "src/desktop/",
            rust_file_mappings,
            ignored_interfaces: NO_IGNORED_INTERFACES,
            interface_prefix: "org.freedesktop.portal.",
        };

        check_doc_aliases_for_config(config);
    }
}

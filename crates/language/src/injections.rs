//! Update injections based on user configuration.
//!
//! Users may create configuration files containing injection queries for specific languages.
//! When one of these languages is loaded by Zed, the corresponding file is read and contained
//! queries are added to the language's predefined injection queries.
//!
//! Additional injection queries may be configured both for languages from Zed extensions and
//! languages built into Zed.

use {super::LanguageQueries, std::path::Path};

/// Support updating injections queries based on user configuration.
pub trait UpdateInjections {
    /// Updates injections queries for a single language.
    fn update_injections(dir: &Path, language: &str, queries: &mut LanguageQueries) {
        let mut path = dir.join(language);
        path.set_extension("scm");
        match std::fs::read_to_string(&path) {
            Ok(configured) => match queries.injections.as_mut() {
                None => queries.injections = Some(configured.into()),
                Some(injections) => {
                    injections.to_mut().push_str(&configured);
                }
            },
            Err(error) => {
                if error.kind() != std::io::ErrorKind::NotFound {
                    log::error!(
                        "failed to read injection queries from \"{}\": {error}",
                        path.display()
                    );
                }
            }
        }
    }
}

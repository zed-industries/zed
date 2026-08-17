//! Active upstream Mermaid baseline metadata.
//!
//! These constants describe the source revision that production parsing, layout, rendering, and
//! parity fixtures target. Generated helper modules may still contain historical suffixes; use
//! [`LEGACY_GENERATED_BASELINE_SUFFIX`] only when matching those existing file/module names.

/// Upstream Mermaid tag pinned by this repository.
pub const PINNED_MERMAID_BASELINE_TAG: &str = "mermaid@11.15.0";

/// Upstream Mermaid semver pinned by this repository.
pub const PINNED_MERMAID_BASELINE_VERSION: &str = "11.15.0";

/// Filesystem/module-name-safe form of [`PINNED_MERMAID_BASELINE_VERSION`].
pub const PINNED_MERMAID_BASELINE_VERSION_SUFFIX: &str = "11_15_0";

// Generated override/file names still carry the old suffix in many places.
// Keep that fact explicit instead of pretending it is the active baseline.
/// Historical suffix retained by generated override modules that have not been renamed.
pub const LEGACY_GENERATED_BASELINE_SUFFIX: &str = "11_12_2";

/// Detector registry profile matching Mermaid's feature registration sets.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BaselineRegistryProfile {
    /// Base Mermaid diagrams without large feature registrations.
    Tiny,
    /// Full Mermaid registration set, including large feature diagrams when enabled.
    Full,
}

impl BaselineRegistryProfile {
    /// Returns a stable lowercase profile label for diagnostics and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Full => "full",
        }
    }
}

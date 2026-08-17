// Mermaid@11.15 Architecture currently has no accepted fixture-scoped root viewport overrides.
//
// Keep this legacy module name until the generated override plumbing is renamed across all
// diagram families. The previous Mermaid@11.12.x Architecture pins were intentionally removed
// during the 11.15 baseline refresh because they forced local roots back to stale upstream values.

pub fn lookup_architecture_root_viewport_override(
    _diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    None
}

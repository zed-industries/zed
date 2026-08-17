/// Parsing Options structure for the PE parser
#[derive(Debug, Copy, Clone)]
pub struct ParseOptions {
    /// Wether the parser should resolve rvas or not. Default: true
    pub resolve_rva: bool,
    /// Whether or not to parse attribute certificates.
    /// Set to false for in-memory representation, as the [loader does not map this info into
    /// memory](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#other-contents-of-the-file).
    /// For on-disk representations, leave as true. Default: true
    pub parse_attribute_certificates: bool,
}

impl ParseOptions {
    /// Returns a parse options structure with default values
    pub fn default() -> Self {
        ParseOptions {
            resolve_rva: true,
            parse_attribute_certificates: true,
        }
    }
}

//! WGSL directives. The focal point of this API is [`DirectiveKind`].
//!
//! See also <https://www.w3.org/TR/WGSL/#directives>.

pub mod enable_extension;
pub(crate) mod language_extension;

use alloc::boxed::Box;

/// A parsed sentinel word indicating the type of directive to be parsed next.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub(crate) enum DirectiveKind {
    /// A [`crate::diagnostic_filter`].
    Diagnostic,
    /// An [`enable_extension`].
    Enable,
    /// A [`language_extension`].
    Requires,
}

impl DirectiveKind {
    const DIAGNOSTIC: &'static str = "diagnostic";
    const ENABLE: &'static str = "enable";
    const REQUIRES: &'static str = "requires";

    /// Convert from a sentinel word in WGSL into its associated [`DirectiveKind`], if possible.
    pub fn from_ident(s: &str) -> Option<Self> {
        Some(match s {
            Self::DIAGNOSTIC => Self::Diagnostic,
            Self::ENABLE => Self::Enable,
            Self::REQUIRES => Self::Requires,
            _ => return None,
        })
    }
}

impl crate::diagnostic_filter::Severity {
    #[cfg(feature = "wgsl-in")]
    pub(crate) fn report_wgsl_parse_diag<'a>(
        self,
        err: Box<crate::front::wgsl::error::Error<'a>>,
        source: &str,
    ) -> crate::front::wgsl::Result<'a, ()> {
        self.report_diag(err, |e, level| {
            let e = e.as_parse_error(source);
            log::log!(level, "{}", e.emit_to_string(source));
        })
    }
}

#[cfg(test)]
mod test {
    use alloc::format;

    use strum::IntoEnumIterator;

    use super::DirectiveKind;
    use crate::front::wgsl::assert_parse_err;

    #[test]
    fn directive_after_global_decl() {
        for unsupported_shader in DirectiveKind::iter() {
            let directive;
            let expected_msg;
            match unsupported_shader {
                DirectiveKind::Diagnostic => {
                    directive = "diagnostic(off,derivative_uniformity)";
                    expected_msg = "\
error: expected global declaration, but found a global directive
  ┌─ wgsl:2:1
  │
2 │ diagnostic(off,derivative_uniformity);
  │ ^^^^^^^^^^ written after first global declaration
  │
  = note: global directives are only allowed before global declarations; maybe hoist this closer to the top of the shader module?

";
                }
                DirectiveKind::Enable => {
                    directive = "enable f16";
                    expected_msg = "\
error: expected global declaration, but found a global directive
  ┌─ wgsl:2:1
  │
2 │ enable f16;
  │ ^^^^^^ written after first global declaration
  │
  = note: global directives are only allowed before global declarations; maybe hoist this closer to the top of the shader module?

";
                }
                DirectiveKind::Requires => {
                    directive = "requires readonly_and_readwrite_storage_textures";
                    expected_msg = "\
error: expected global declaration, but found a global directive
  ┌─ wgsl:2:1
  │
2 │ requires readonly_and_readwrite_storage_textures;
  │ ^^^^^^^^ written after first global declaration
  │
  = note: global directives are only allowed before global declarations; maybe hoist this closer to the top of the shader module?

";
                }
            }

            let shader = format!(
                "\
@group(0) @binding(0) var<storage> thing: i32;
{directive};
"
            );
            assert_parse_err(&shader, expected_msg);
        }
    }
}

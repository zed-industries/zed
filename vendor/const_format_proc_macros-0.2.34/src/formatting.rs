use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Formatting {
    Debug(NumberFormatting),
    Display,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum NumberFormatting {
    Decimal,
    Hexadecimal,
    LowerHexadecimal,
    Binary,
}

impl NumberFormatting {
    pub(crate) fn is_regular(self) -> bool {
        matches!(self, NumberFormatting::Decimal)
    }
}

impl ToTokens for NumberFormatting {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        ts.append_all(match self {
            Self::Decimal => return,
            Self::Hexadecimal => quote!(.set_hexadecimal()),
            Self::LowerHexadecimal => quote!(.set_lower_hexadecimal()),
            Self::Binary => quote!(.set_binary()),
        });
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum IsAlternate {
    Yes,
    No,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct FormattingFlags {
    pub(crate) formatting: Formatting,
    pub(crate) is_alternate: IsAlternate,
}

impl FormattingFlags {
    #[inline]
    pub(crate) const fn display(is_alternate: IsAlternate) -> Self {
        Self {
            formatting: Formatting::Display,
            is_alternate,
        }
    }

    #[inline]
    pub(crate) const fn debug(num_fmt: NumberFormatting, is_alternate: IsAlternate) -> Self {
        Self {
            formatting: Formatting::Debug(num_fmt),
            is_alternate,
        }
    }
}

impl FormattingFlags {
    pub(crate) fn to_pargument_method_name(self) -> Ident {
        let name = match self.formatting {
            Formatting::Display => "to_pargument_display",
            Formatting::Debug { .. } => "to_pargument_debug",
        };

        Ident::new(name, Span::mixed_site())
    }

    #[allow(dead_code)]
    pub(crate) fn fmt_method_name(self) -> Ident {
        let name = match self.formatting {
            Formatting::Display => "const_display_fmt",
            Formatting::Debug { .. } => "const_debug_fmt",
        };

        Ident::new(name, Span::mixed_site())
    }

    #[allow(dead_code)]
    pub(crate) fn len_method_name(self) -> Ident {
        let name = match self.formatting {
            Formatting::Display => "const_display_fmt",
            Formatting::Debug { .. } => "const_debug_fmt",
        };

        Ident::new(name, Span::mixed_site())
    }
}

impl ToTokens for FormattingFlags {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        use self::{IsAlternate as IA, NumberFormatting as FM};

        let formatting = match self.formatting {
            Formatting::Display => NumberFormatting::Decimal,
            Formatting::Debug(num_fmt) => num_fmt,
        };

        ts.append_all(match (self.is_alternate, formatting) {
            (IA::No, FM::Decimal) => quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__REG),
            (IA::No, FM::Hexadecimal) => quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__HEX),
            (IA::No, FM::LowerHexadecimal) => {
                quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__LOWHEX)
            }
            (IA::No, FM::Binary) => quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__BIN),
            (IA::Yes, FM::Decimal) => quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__A_REG),
            (IA::Yes, FM::Hexadecimal) => quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__A_HEX),
            (IA::Yes, FM::LowerHexadecimal) => {
                quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__A_LOWHEX)
            }
            (IA::Yes, FM::Binary) => quote!(__cf_osRcTFl4A::pmr::FormattingFlags::__A_BIN),
        });
    }
}

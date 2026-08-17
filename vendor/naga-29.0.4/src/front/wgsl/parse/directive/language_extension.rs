//! `requires …;` extensions in WGSL.
//!
//! The focal point of this module is the [`LanguageExtension`] API.

/// A language extension recognized by Naga, but not guaranteed to be present in all environments.
///
/// WGSL spec.: <https://www.w3.org/TR/WGSL/#language-extensions-sec>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LanguageExtension {
    Implemented(ImplementedLanguageExtension),
    Unimplemented(UnimplementedLanguageExtension),
}

impl LanguageExtension {
    const READONLY_AND_READWRITE_STORAGE_TEXTURES: &'static str =
        "readonly_and_readwrite_storage_textures";
    const PACKED4X8_INTEGER_DOT_PRODUCT: &'static str = "packed_4x8_integer_dot_product";
    const UNRESTRICTED_POINTER_PARAMETERS: &'static str = "unrestricted_pointer_parameters";
    const POINTER_COMPOSITE_ACCESS: &'static str = "pointer_composite_access";

    /// Convert from a sentinel word in WGSL into its associated [`LanguageExtension`], if possible.
    pub fn from_ident(s: &str) -> Option<Self> {
        Some(match s {
            Self::READONLY_AND_READWRITE_STORAGE_TEXTURES => {
                Self::Implemented(ImplementedLanguageExtension::ReadOnlyAndReadWriteStorageTextures)
            }
            Self::PACKED4X8_INTEGER_DOT_PRODUCT => {
                Self::Implemented(ImplementedLanguageExtension::Packed4x8IntegerDotProduct)
            }
            Self::UNRESTRICTED_POINTER_PARAMETERS => {
                Self::Unimplemented(UnimplementedLanguageExtension::UnrestrictedPointerParameters)
            }
            Self::POINTER_COMPOSITE_ACCESS => {
                Self::Implemented(ImplementedLanguageExtension::PointerCompositeAccess)
            }
            _ => return None,
        })
    }

    /// Maps this [`LanguageExtension`] into the sentinel word associated with it in WGSL.
    pub const fn to_ident(self) -> &'static str {
        match self {
            Self::Implemented(kind) => kind.to_ident(),
            Self::Unimplemented(kind) => match kind {
                UnimplementedLanguageExtension::UnrestrictedPointerParameters => {
                    Self::UNRESTRICTED_POINTER_PARAMETERS
                }
            },
        }
    }
}

/// A variant of [`LanguageExtension::Implemented`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(test, derive(strum::VariantArray))]
pub enum ImplementedLanguageExtension {
    ReadOnlyAndReadWriteStorageTextures,
    Packed4x8IntegerDotProduct,
    PointerCompositeAccess,
}

impl ImplementedLanguageExtension {
    /// A slice of all variants of [`ImplementedLanguageExtension`].
    pub const VARIANTS: &'static [Self] = &[
        Self::ReadOnlyAndReadWriteStorageTextures,
        Self::Packed4x8IntegerDotProduct,
        Self::PointerCompositeAccess,
    ];

    /// Returns slice of all variants of [`ImplementedLanguageExtension`].
    pub const fn all() -> &'static [Self] {
        Self::VARIANTS
    }

    /// Maps this [`ImplementedLanguageExtension`] into the sentinel word associated with it in WGSL.
    pub const fn to_ident(self) -> &'static str {
        match self {
            ImplementedLanguageExtension::ReadOnlyAndReadWriteStorageTextures => {
                LanguageExtension::READONLY_AND_READWRITE_STORAGE_TEXTURES
            }
            ImplementedLanguageExtension::Packed4x8IntegerDotProduct => {
                LanguageExtension::PACKED4X8_INTEGER_DOT_PRODUCT
            }
            ImplementedLanguageExtension::PointerCompositeAccess => {
                LanguageExtension::POINTER_COMPOSITE_ACCESS
            }
        }
    }
}

#[test]
/// Asserts that the manual implementation of VARIANTS is the same as the derived strum version would be
/// while still allowing strum to be a dev-only dependency
fn test_manual_variants_array_is_correct() {
    assert_eq!(
        <ImplementedLanguageExtension as strum::VariantArray>::VARIANTS,
        ImplementedLanguageExtension::VARIANTS
    );
}

/// A variant of [`LanguageExtension::Unimplemented`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnimplementedLanguageExtension {
    UnrestrictedPointerParameters,
}

impl UnimplementedLanguageExtension {
    pub(crate) const fn tracking_issue_num(self) -> u16 {
        match self {
            Self::UnrestrictedPointerParameters => 5158,
        }
    }
}

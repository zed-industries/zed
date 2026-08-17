/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::ir::ty::{IntKind, PrimitiveType};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ReprStyle {
    #[default]
    Rust,
    C,
    Transparent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReprType {
    kind: IntKind,
    signed: bool,
}

impl ReprType {
    pub(crate) fn to_primitive(self) -> PrimitiveType {
        PrimitiveType::Integer {
            kind: self.kind,
            signed: self.signed,
            zeroable: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReprAlign {
    Packed,
    Align(u64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Repr {
    pub style: ReprStyle,
    pub ty: Option<ReprType>,
    pub align: Option<ReprAlign>,
}

impl Repr {
    pub fn load(attrs: &[syn::Attribute]) -> Result<Repr, String> {
        let mut ids = Vec::new();

        // We want only the `repr` attributes
        let iter = attrs.iter().filter(|attr| attr.path().is_ident("repr"));

        for repr_attr in iter {
            let reprs = repr_attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .map_err(|err| format!("Invalid `#[repr]` attribute: {err}"))?;

            for meta in reprs {
                match meta {
                    // #[repr(C)] and #[repr(transparent)] and #[repr(INT)]
                    syn::Meta::Path(path) => {
                        if let Some(ident) = path.get_ident() {
                            let ident_s = ident.to_string();
                            ids.push((ident_s, None))
                        } else {
                            return Err("Invalid `repr` attribute".to_string());
                        }
                    }
                    // #[repr(align(N))]
                    syn::Meta::List(meta) if meta.path.is_ident("align") => {
                        let lit: syn::LitInt = meta
                            .parse_args()
                            .map_err(|err| format!("Invalid align argument: {err}"))?;
                        ids.push(("align".to_string(), Some(lit.to_string())))
                    }
                    // #[repr(packed(N))]
                    syn::Meta::List(meta) if meta.path.is_ident("packed") => {
                        // no arguments
                        if meta.tokens.is_empty() {
                            ids.push(("packed".to_string(), None))
                        } else {
                            let lit: syn::LitInt = meta
                                .parse_args()
                                .map_err(|err| format!("Invalid packed argument: {err}"))?;
                            ids.push(("packed".to_string(), Some(lit.to_string())))
                        }
                    }
                    _ => return Err("Invalid `repr` attribute".to_string()),
                }
            }
        }

        let mut repr = Repr::default();
        for id in ids {
            let (int_kind, signed) = match (id.0.as_ref(), id.1) {
                ("u8", None) => (IntKind::B8, false),
                ("u16", None) => (IntKind::B16, false),
                ("u32", None) => (IntKind::B32, false),
                ("u64", None) => (IntKind::B64, false),
                ("usize", None) => (IntKind::Size, false),
                ("i8", None) => (IntKind::B8, true),
                ("i16", None) => (IntKind::B16, true),
                ("i32", None) => (IntKind::B32, true),
                ("i64", None) => (IntKind::B64, true),
                ("isize", None) => (IntKind::Size, true),
                ("C", None) => {
                    repr.style = ReprStyle::C;
                    continue;
                }
                ("transparent", None) => {
                    repr.style = ReprStyle::Transparent;
                    continue;
                }
                ("packed", args) => {
                    // #[repr(packed(n))] not supported because of some open questions about how
                    // to calculate the native alignment of types. See mozilla/cbindgen#433.
                    if args.is_some() {
                        return Err(
                            "Not-yet-implemented #[repr(packed(...))] encountered.".to_string()
                        );
                    }
                    let align = ReprAlign::Packed;
                    // Only permit a single alignment-setting repr.
                    if let Some(old_align) = repr.align {
                        return Err(format!(
                            "Conflicting #[repr(align(...))] type hints {:?} and {:?}.",
                            old_align, align
                        ));
                    }
                    repr.align = Some(align);
                    continue;
                }
                ("align", Some(arg)) => {
                    // Must be a positive integer.
                    let align = match arg.parse::<u64>() {
                        Ok(align) => align,
                        Err(_) => return Err(format!("Non-unsigned #[repr(align({}))].", arg)),
                    };
                    // Must be a power of 2.
                    if !align.is_power_of_two() || align == 0 {
                        return Err(format!("Invalid alignment to #[repr(align({}))].", align));
                    }
                    // Only permit a single alignment-setting repr.
                    if let Some(old_align) = repr.align {
                        return Err(format!(
                            "Conflicting #[repr(align(...))] type hints {:?} and {:?}.",
                            old_align,
                            ReprAlign::Align(align)
                        ));
                    }
                    repr.align = Some(ReprAlign::Align(align));
                    continue;
                }
                (path, arg) => match arg {
                    None => return Err(format!("Unsupported #[repr({})].", path)),
                    Some(arg) => {
                        return Err(format!("Unsupported #[repr({}({}))].", path, arg));
                    }
                },
            };
            let ty = ReprType {
                kind: int_kind,
                signed,
            };
            if let Some(old_ty) = repr.ty {
                return Err(format!(
                    "Conflicting #[repr(...)] type hints {:?} and {:?}.",
                    old_ty, ty
                ));
            }
            repr.ty = Some(ty);
        }
        Ok(repr)
    }
}

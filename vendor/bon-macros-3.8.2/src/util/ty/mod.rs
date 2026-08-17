mod match_types;

use crate::util::prelude::*;
use syn::punctuated::Punctuated;

pub(crate) trait TypeExt {
    /// Try downcasting the type to [`syn::Type::Path`]
    fn as_path(&self) -> Option<&syn::TypePath>;

    /// Try downcasting the type to [`syn::Type::Path`]. If it has a [`syn::QSelf`]
    /// then this method will return `None`.
    fn as_path_no_qself(&self) -> Option<&syn::Path>;

    /// Detects if the type is [`Option`] and returns its generic type parameter
    fn option_type_param(&self) -> Option<&syn::Type>;

    /// Validates that this type is a generic type (path without [`syn::QSelf`])
    /// which ends with the given last segment that passes the predicate
    /// `is_desired_last_segment`.
    fn as_generic_angle_bracketed_path(
        &self,
        is_desired_last_segment: impl FnOnce(&syn::Ident) -> bool,
    ) -> Option<GenericAngleBracketedPath<'_>>;

    /// Heuristically detects if the type is [`Option`]
    fn is_option(&self) -> bool;

    /// Recursively strips the [`syn::Type::Group`] and [`syn::Type::Paren`] wrappers
    fn peel(&self) -> &Self;

    /// Returns `true` if the given type matches the pattern. The types match only if
    /// their tokens are equal or if they differ in the places where the pattern has
    /// a wildcard [`syn::Type::Infer`] e.g. `Vec<i32>` matches the pattern `Vec<_>`.
    ///
    /// Any wildcards in `Self` will not be specially handled. Only wildcards in `pattern`
    /// have semantic meaning.
    fn matches(&self, pattern: &syn::Type) -> Result<bool>;
}

impl TypeExt for syn::Type {
    fn as_path(&self) -> Option<&syn::TypePath> {
        match self.peel() {
            Self::Path(path) => Some(path),
            _ => None,
        }
    }

    fn as_path_no_qself(&self) -> Option<&syn::Path> {
        let path = self.as_path()?;
        if path.qself.is_some() {
            return None;
        }
        Some(&path.path)
    }

    fn option_type_param(&self) -> Option<&syn::Type> {
        let ty = self.as_generic_angle_bracketed_path(|last_segment| last_segment == "Option")?;

        if ty.args.len() != 1 {
            return None;
        }

        let arg = ty.args.first()?;

        let arg = match arg {
            syn::GenericArgument::Type(arg) => arg,
            _ => return None,
        };

        Some(arg)
    }

    fn as_generic_angle_bracketed_path(
        &self,
        is_desired_last_segment: impl FnOnce(&syn::Ident) -> bool,
    ) -> Option<GenericAngleBracketedPath<'_>> {
        let path = self.as_path_no_qself()?;

        let last_segment = path.segments.last()?;

        if !is_desired_last_segment(&last_segment.ident) {
            return None;
        }

        let args = match &last_segment.arguments {
            syn::PathArguments::AngleBracketed(args) => &args.args,
            _ => return None,
        };

        Some(GenericAngleBracketedPath { path, args })
    }

    fn is_option(&self) -> bool {
        self.option_type_param().is_some()
    }

    fn peel(&self) -> &Self {
        match self {
            Self::Group(group) => group.elem.peel(),
            Self::Paren(paren) => paren.elem.peel(),
            _ => self,
        }
    }

    fn matches(&self, pattern: &syn::Type) -> Result<bool> {
        match_types::match_types(self, pattern)
    }
}

pub(crate) struct GenericAngleBracketedPath<'a> {
    pub(crate) path: &'a syn::Path,
    pub(crate) args: &'a Punctuated<syn::GenericArgument, syn::Token![,]>,
}

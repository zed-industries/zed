use crate::util::prelude::*;
use ident_case::RenameRule;
use syn::spanned::Spanned;

pub(crate) trait IdentExt {
    /// Converts the ident (assumed to be in `snake_case`) to `PascalCase` without
    /// preserving its span.
    ///
    /// Span loss is intentional to work around the semantic token type assignment
    /// ambiguity that may be experienced by IDEs. For example, rust analyzer
    /// assigns different colors to identifiers according to their semantic meaning.
    ///
    /// If identifiers with the same span were used in different contexts such as
    /// in function name and struct name positions, then rust-analyzer would chose
    /// the semantic meaning for syntax highlighting of the input identifier randomly
    /// out of these two contexts.
    ///
    /// By not preserving the span, we can ensure that the semantic meaning of the
    /// produced identifier won't influence the syntax highlighting of the original
    /// identifier.
    fn snake_to_pascal_case(&self) -> Self;

    /// Same thing as `snake_to_pascal_case` but converts `PascalCase` to `snake_case`.
    fn pascal_to_snake_case(&self) -> Self;

    /// Creates a new ident with the given name and span. If the name starts with
    /// `r#` then automatically creates a raw ident.
    fn new_maybe_raw(name: &str, span: Span) -> Self;

    /// Returns the name of the identifier stripping the `r#` prefix if it exists.
    fn raw_name(&self) -> String;
}

impl IdentExt for syn::Ident {
    fn snake_to_pascal_case(&self) -> Self {
        // There are no pascal case keywords in Rust except for `Self`, which
        // is anyway not allowed even as a raw identifier:
        // https://internals.rust-lang.org/t/raw-identifiers-dont-work-for-all-identifiers/9094
        //
        // So no need to handle raw identifiers here.
        let mut renamed = RenameRule::PascalCase.apply_to_field(self.raw_name());

        // Make sure `Self` keyword isn't generated.
        // This may happen if the input was `self_`, for example.
        if renamed == "Self" {
            renamed.push('_');
        }

        Self::new(&renamed, renamed.span())
    }

    fn pascal_to_snake_case(&self) -> Self {
        let renamed = RenameRule::SnakeCase.apply_to_variant(self.raw_name());
        Self::new_maybe_raw(&renamed, Span::call_site())
    }

    fn new_maybe_raw(name: &str, span: Span) -> Self {
        // If the ident is already raw (starts with `r#`) then just create a raw ident.
        if let Some(name) = name.strip_prefix("r#") {
            return Self::new_raw(name, span);
        }

        // ..otherwise validate if it is a valid identifier.
        // The `parse_str` method will return an error if the name is not a valid
        // identifier.
        if syn::parse_str::<Self>(name).is_ok() {
            return Self::new(name, span);
        }

        // Try to make it a raw ident by adding `r#` prefix.
        // This won't work for some keywords such as `super`, `crate`,
        // `Self`, which are not allowed as raw identifiers
        if syn::parse_str::<Self>(&format!("r#{name}")).is_ok() {
            return Self::new_raw(name, span);
        }

        // As the final fallback add a trailing `_` to create a valid identifier
        Self::new(&format!("{name}_"), span)
    }

    fn raw_name(&self) -> String {
        let name = self.to_string();
        if let Some(raw) = name.strip_prefix("r#") {
            raw.to_owned()
        } else {
            name
        }
    }
}

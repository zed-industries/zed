use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use syn::{Expr, Lit, Meta, NestedMeta};

use crate::{util::path_to_string, Error, Result};

/// Create an instance from an item in an attribute declaration.
///
/// # Implementing `FromMeta`
/// * Do not take a dependency on the `ident` of the passed-in meta item. The ident will be set by the field name of the containing struct.
/// * Implement only the `from_*` methods that you intend to support. The default implementations will return useful errors.
///
/// # Provided Implementations
/// ## bool
///
/// * Word with no value specified - becomes `true`.
/// * As a boolean literal, e.g. `foo = true`.
/// * As a string literal, e.g. `foo = "true"`.
///
/// ## char
/// * As a char literal, e.g. `foo = '#'`.
/// * As a string literal consisting of a single character, e.g. `foo = "#"`.
///
/// ## String
/// * As a string literal, e.g. `foo = "hello"`.
/// * As a raw string literal, e.g. `foo = r#"hello "world""#`.
///
/// ## Number
/// * As a string literal, e.g. `foo = "-25"`.
/// * As an unquoted positive value, e.g. `foo = 404`. Negative numbers must be in quotation marks.
///
/// ## ()
/// * Word with no value specified, e.g. `foo`. This is best used with `Option`.
///   See `darling::util::Flag` for a more strongly-typed alternative.
///
/// ## Option
/// * Any format produces `Some`.
///
/// ## `Result<T, darling::Error>`
/// * Allows for fallible parsing; will populate the target field with the result of the
///   parse attempt.
pub trait FromMeta: Sized {
    fn from_nested_meta(item: &NestedMeta) -> Result<Self> {
        (match *item {
            NestedMeta::Lit(ref lit) => Self::from_value(lit),
            NestedMeta::Meta(ref mi) => Self::from_meta(mi),
        })
        .map_err(|e| e.with_span(item))
    }

    /// Create an instance from a `syn::Meta` by dispatching to the format-appropriate
    /// trait function. This generally should not be overridden by implementers.
    ///
    /// # Error Spans
    /// If this method is overridden and can introduce errors that weren't passed up from
    /// other `from_meta` calls, the override must call `with_span` on the error using the
    /// `item` to make sure that the emitted diagnostic points to the correct location in
    /// source code.
    fn from_meta(item: &Meta) -> Result<Self> {
        (match *item {
            Meta::Path(_) => Self::from_word(),
            Meta::List(ref value) => Self::from_list(
                &value
                    .nested
                    .iter()
                    .cloned()
                    .collect::<Vec<syn::NestedMeta>>()[..],
            ),
            Meta::NameValue(ref value) => Self::from_value(&value.lit),
        })
        .map_err(|e| e.with_span(item))
    }

    /// When a field is omitted from a parent meta-item, `from_none` is used to attempt
    /// recovery before a missing field error is generated.
    ///
    /// **Most types should not override this method.** `darling` already allows field-level
    /// missing-field recovery using `#[darling(default)]` and `#[darling(default = "...")]`,
    /// and users who add a `String` field to their `FromMeta`-deriving struct would be surprised
    /// if they get back `""` instead of a missing field error when that field is omitted.
    ///
    /// The primary use-case for this is `Option<T>` fields gracefully handlling absence without
    /// needing `#[darling(default)]`.
    fn from_none() -> Option<Self> {
        None
    }

    /// Create an instance from the presence of the word in the attribute with no
    /// additional options specified.
    fn from_word() -> Result<Self> {
        Err(Error::unsupported_format("word"))
    }

    /// Create an instance from a list of nested meta items.
    #[allow(unused_variables)]
    fn from_list(items: &[NestedMeta]) -> Result<Self> {
        Err(Error::unsupported_format("list"))
    }

    /// Create an instance from a literal value of either `foo = "bar"` or `foo("bar")`.
    /// This dispatches to the appropriate method based on the type of literal encountered,
    /// and generally should not be overridden by implementers.
    ///
    /// # Error Spans
    /// If this method is overridden, the override must make sure to add `value`'s span
    /// information to the returned error by calling `with_span(value)` on the `Error` instance.
    fn from_value(value: &Lit) -> Result<Self> {
        (match *value {
            Lit::Bool(ref b) => Self::from_bool(b.value),
            Lit::Str(ref s) => Self::from_string(&s.value()),
            Lit::Char(ref ch) => Self::from_char(ch.value()),
            _ => Err(Error::unexpected_lit_type(value)),
        })
        .map_err(|e| e.with_span(value))
    }

    /// Create an instance from a char literal in a value position.
    #[allow(unused_variables)]
    fn from_char(value: char) -> Result<Self> {
        Err(Error::unexpected_type("char"))
    }

    /// Create an instance from a string literal in a value position.
    #[allow(unused_variables)]
    fn from_string(value: &str) -> Result<Self> {
        Err(Error::unexpected_type("string"))
    }

    /// Create an instance from a bool literal in a value position.
    #[allow(unused_variables)]
    fn from_bool(value: bool) -> Result<Self> {
        Err(Error::unexpected_type("bool"))
    }
}

// FromMeta impls for std and syn types.

impl FromMeta for () {
    fn from_word() -> Result<Self> {
        Ok(())
    }
}

impl FromMeta for bool {
    fn from_word() -> Result<Self> {
        Ok(true)
    }

    #[allow(clippy::wrong_self_convention)] // false positive
    fn from_bool(value: bool) -> Result<Self> {
        Ok(value)
    }

    fn from_string(value: &str) -> Result<Self> {
        value.parse().map_err(|_| Error::unknown_value(value))
    }
}

impl FromMeta for AtomicBool {
    fn from_meta(mi: &Meta) -> Result<Self> {
        FromMeta::from_meta(mi)
            .map(AtomicBool::new)
            .map_err(|e| e.with_span(mi))
    }
}

impl FromMeta for char {
    #[allow(clippy::wrong_self_convention)] // false positive
    fn from_char(value: char) -> Result<Self> {
        Ok(value)
    }

    fn from_string(s: &str) -> Result<Self> {
        let mut chars = s.chars();
        let char1 = chars.next();
        let char2 = chars.next();

        if let (Some(char), None) = (char1, char2) {
            Ok(char)
        } else {
            Err(Error::unexpected_type("string"))
        }
    }
}

impl FromMeta for String {
    fn from_string(s: &str) -> Result<Self> {
        Ok(s.to_string())
    }
}

/// Generate an impl of `FromMeta` that will accept strings which parse to numbers or
/// integer literals.
macro_rules! from_meta_num {
    ($ty:ident) => {
        impl FromMeta for $ty {
            fn from_string(s: &str) -> Result<Self> {
                s.parse().map_err(|_| Error::unknown_value(s))
            }

            fn from_value(value: &Lit) -> Result<Self> {
                (match *value {
                    Lit::Str(ref s) => Self::from_string(&s.value()),
                    Lit::Int(ref s) => Ok(s.base10_parse::<$ty>().unwrap()),
                    _ => Err(Error::unexpected_lit_type(value)),
                })
                .map_err(|e| e.with_span(value))
            }
        }
    };
}

from_meta_num!(u8);
from_meta_num!(u16);
from_meta_num!(u32);
from_meta_num!(u64);
from_meta_num!(usize);
from_meta_num!(i8);
from_meta_num!(i16);
from_meta_num!(i32);
from_meta_num!(i64);
from_meta_num!(isize);

/// Generate an impl of `FromMeta` that will accept strings which parse to floats or
/// float literals.
macro_rules! from_meta_float {
    ($ty:ident) => {
        impl FromMeta for $ty {
            fn from_string(s: &str) -> Result<Self> {
                s.parse().map_err(|_| Error::unknown_value(s))
            }

            fn from_value(value: &Lit) -> Result<Self> {
                (match *value {
                    Lit::Str(ref s) => Self::from_string(&s.value()),
                    Lit::Float(ref s) => Ok(s.base10_parse::<$ty>().unwrap()),
                    _ => Err(Error::unexpected_lit_type(value)),
                })
                .map_err(|e| e.with_span(value))
            }
        }
    };
}

from_meta_float!(f32);
from_meta_float!(f64);

/// Parsing support for punctuated. This attempts to preserve span information
/// when available, but also supports parsing strings with the call site as the
/// emitted span.
impl<T: syn::parse::Parse, P: syn::parse::Parse> FromMeta for syn::punctuated::Punctuated<T, P> {
    fn from_value(value: &Lit) -> Result<Self> {
        if let Lit::Str(ref ident) = *value {
            ident
                .parse_with(syn::punctuated::Punctuated::parse_terminated)
                .map_err(|_| Error::unknown_lit_str_value(ident))
        } else {
            Err(Error::unexpected_lit_type(value))
        }
    }
}

/// Adapter from `syn::parse::Parse` to `FromMeta`.
///
/// This cannot be a blanket impl, due to the `syn::Lit` family's need to handle non-string values.
/// Therefore, we use a macro and a lot of impls.
macro_rules! from_syn_parse {
    ($ty:path) => {
        impl FromMeta for $ty {
            fn from_string(value: &str) -> Result<Self> {
                syn::parse_str(value).map_err(|_| Error::unknown_value(value))
            }

            fn from_value(value: &::syn::Lit) -> Result<Self> {
                if let ::syn::Lit::Str(ref v) = *value {
                    v.parse::<$ty>()
                        .map_err(|_| Error::unknown_lit_str_value(v))
                } else {
                    Err(Error::unexpected_lit_type(value))
                }
            }
        }
    };
}

from_syn_parse!(syn::Ident);
from_syn_parse!(syn::Expr);
from_syn_parse!(syn::ExprArray);
from_syn_parse!(syn::ExprPath);
from_syn_parse!(syn::Path);
from_syn_parse!(syn::Type);
from_syn_parse!(syn::TypeArray);
from_syn_parse!(syn::TypeBareFn);
from_syn_parse!(syn::TypeGroup);
from_syn_parse!(syn::TypeImplTrait);
from_syn_parse!(syn::TypeInfer);
from_syn_parse!(syn::TypeMacro);
from_syn_parse!(syn::TypeNever);
from_syn_parse!(syn::TypeParam);
from_syn_parse!(syn::TypeParen);
from_syn_parse!(syn::TypePath);
from_syn_parse!(syn::TypePtr);
from_syn_parse!(syn::TypeReference);
from_syn_parse!(syn::TypeSlice);
from_syn_parse!(syn::TypeTraitObject);
from_syn_parse!(syn::TypeTuple);
from_syn_parse!(syn::Visibility);
from_syn_parse!(syn::WhereClause);

macro_rules! from_numeric_array {
    ($ty:ident) => {
        /// Parsing an unsigned integer array, i.e. `example = "[1, 2, 3, 4]"`.
        impl FromMeta for Vec<$ty> {
            fn from_value(value: &Lit) -> Result<Self> {
                let expr_array = syn::ExprArray::from_value(value)?;
                // To meet rust <1.36 borrow checker rules on expr_array.elems
                let v =
                    expr_array
                        .elems
                        .iter()
                        .map(|expr| match expr {
                            Expr::Lit(lit) => $ty::from_value(&lit.lit),
                            _ => Err(Error::custom("Expected array of unsigned integers")
                                .with_span(expr)),
                        })
                        .collect::<Result<Vec<$ty>>>();
                v
            }
        }
    };
}

from_numeric_array!(u8);
from_numeric_array!(u16);
from_numeric_array!(u32);
from_numeric_array!(u64);
from_numeric_array!(usize);

impl FromMeta for syn::Lit {
    fn from_value(value: &Lit) -> Result<Self> {
        Ok(value.clone())
    }
}

macro_rules! from_meta_lit {
    ($impl_ty:path, $lit_variant:path) => {
        impl FromMeta for $impl_ty {
            fn from_value(value: &Lit) -> Result<Self> {
                if let $lit_variant(ref value) = *value {
                    Ok(value.clone())
                } else {
                    Err(Error::unexpected_lit_type(value))
                }
            }
        }
    };
}

from_meta_lit!(syn::LitInt, Lit::Int);
from_meta_lit!(syn::LitFloat, Lit::Float);
from_meta_lit!(syn::LitStr, Lit::Str);
from_meta_lit!(syn::LitByte, Lit::Byte);
from_meta_lit!(syn::LitByteStr, Lit::ByteStr);
from_meta_lit!(syn::LitChar, Lit::Char);
from_meta_lit!(syn::LitBool, Lit::Bool);
from_meta_lit!(proc_macro2::Literal, Lit::Verbatim);

impl FromMeta for syn::Meta {
    fn from_meta(value: &syn::Meta) -> Result<Self> {
        Ok(value.clone())
    }
}

impl FromMeta for Vec<syn::WherePredicate> {
    fn from_string(value: &str) -> Result<Self> {
        syn::WhereClause::from_string(&format!("where {}", value))
            .map(|c| c.predicates.into_iter().collect())
    }

    fn from_value(value: &Lit) -> Result<Self> {
        if let syn::Lit::Str(s) = value {
            syn::WhereClause::from_value(&syn::Lit::Str(syn::LitStr::new(
                &format!("where {}", s.value()),
                value.span(),
            )))
            .map(|c| c.predicates.into_iter().collect())
        } else {
            Err(Error::unexpected_lit_type(value))
        }
    }
}

impl FromMeta for ident_case::RenameRule {
    fn from_string(value: &str) -> Result<Self> {
        value.parse().map_err(|_| Error::unknown_value(value))
    }
}

impl<T: FromMeta> FromMeta for Option<T> {
    fn from_none() -> Option<Self> {
        Some(None)
    }

    fn from_meta(item: &Meta) -> Result<Self> {
        FromMeta::from_meta(item).map(Some)
    }
}

impl<T: FromMeta> FromMeta for Box<T> {
    fn from_none() -> Option<Self> {
        T::from_none().map(Box::new)
    }

    fn from_meta(item: &Meta) -> Result<Self> {
        FromMeta::from_meta(item).map(Box::new)
    }
}

impl<T: FromMeta> FromMeta for Result<T> {
    fn from_none() -> Option<Self> {
        T::from_none().map(Ok)
    }

    fn from_meta(item: &Meta) -> Result<Self> {
        Ok(FromMeta::from_meta(item))
    }
}

/// Parses the meta-item, and in case of error preserves a copy of the input for
/// later analysis.
impl<T: FromMeta> FromMeta for ::std::result::Result<T, Meta> {
    fn from_meta(item: &Meta) -> Result<Self> {
        T::from_meta(item)
            .map(Ok)
            .or_else(|_| Ok(Err(item.clone())))
    }
}

impl<T: FromMeta> FromMeta for Rc<T> {
    fn from_none() -> Option<Self> {
        T::from_none().map(Rc::new)
    }

    fn from_meta(item: &Meta) -> Result<Self> {
        FromMeta::from_meta(item).map(Rc::new)
    }
}

impl<T: FromMeta> FromMeta for Arc<T> {
    fn from_none() -> Option<Self> {
        T::from_none().map(Arc::new)
    }

    fn from_meta(item: &Meta) -> Result<Self> {
        FromMeta::from_meta(item).map(Arc::new)
    }
}

impl<T: FromMeta> FromMeta for RefCell<T> {
    fn from_none() -> Option<Self> {
        T::from_none().map(RefCell::new)
    }

    fn from_meta(item: &Meta) -> Result<Self> {
        FromMeta::from_meta(item).map(RefCell::new)
    }
}

/// Trait to convert from a path into an owned key for a map.
trait KeyFromPath: Sized {
    fn from_path(path: &syn::Path) -> Result<Self>;
    fn to_display(&self) -> Cow<'_, str>;
}

impl KeyFromPath for String {
    fn from_path(path: &syn::Path) -> Result<Self> {
        Ok(path_to_string(path))
    }

    fn to_display(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}

impl KeyFromPath for syn::Path {
    fn from_path(path: &syn::Path) -> Result<Self> {
        Ok(path.clone())
    }

    fn to_display(&self) -> Cow<'_, str> {
        Cow::Owned(path_to_string(self))
    }
}

impl KeyFromPath for syn::Ident {
    fn from_path(path: &syn::Path) -> Result<Self> {
        if path.segments.len() == 1
            && path.leading_colon.is_none()
            && path.segments[0].arguments.is_empty()
        {
            Ok(path.segments[0].ident.clone())
        } else {
            Err(Error::custom("Key must be an identifier").with_span(path))
        }
    }

    fn to_display(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}

macro_rules! hash_map {
    ($key:ty) => {
        impl<V: FromMeta, S: BuildHasher + Default> FromMeta for HashMap<$key, V, S> {
            fn from_list(nested: &[syn::NestedMeta]) -> Result<Self> {
                // Convert the nested meta items into a sequence of (path, value result) result tuples.
                // An outer Err means no (key, value) structured could be found, while an Err in the
                // second position of the tuple means that value was rejected by FromMeta.
                //
                // We defer key conversion into $key so that we don't lose span information in the case
                // of String keys; we'll need it for good duplicate key errors later.
                let pairs = nested
                    .iter()
                    .map(|item| -> Result<(&syn::Path, Result<V>)> {
                        match *item {
                            syn::NestedMeta::Meta(ref inner) => {
                                let path = inner.path();
                                Ok((
                                    path,
                                    FromMeta::from_meta(inner).map_err(|e| e.at_path(&path)),
                                ))
                            }
                            syn::NestedMeta::Lit(_) => Err(Error::unsupported_format("literal")),
                        }
                    });

                let mut errors = Error::accumulator();
                // We need to track seen keys separately from the final map, since a seen key with an
                // Err value won't go into the final map but should trigger a duplicate field error.
                //
                // This is a set of $key rather than Path to avoid the possibility that a key type
                // parses two paths of different values to the same key value.
                let mut seen_keys = HashSet::with_capacity(nested.len());

                // The map to return in the Ok case. Its size will always be exactly nested.len(),
                // since otherwise ≥1 field had a problem and the entire map is dropped immediately
                // when the function returns `Err`.
                let mut map = HashMap::with_capacity_and_hasher(nested.len(), Default::default());

                for item in pairs {
                    if let Some((path, value)) = errors.handle(item) {
                        let key: $key = match KeyFromPath::from_path(path) {
                            Ok(k) => k,
                            Err(e) => {
                                errors.push(e);

                                // Surface value errors even under invalid keys
                                errors.handle(value);

                                continue;
                            }
                        };

                        let already_seen = seen_keys.contains(&key);

                        if already_seen {
                            errors.push(Error::duplicate_field(&key.to_display()).with_span(path));
                        }

                        match value {
                            Ok(_) if already_seen => {}
                            Ok(val) => {
                                map.insert(key.clone(), val);
                            }
                            Err(e) => {
                                errors.push(e);
                            }
                        }

                        seen_keys.insert(key);
                    }
                }

                errors.finish_with(map)
            }
        }
    };
}

// This is done as a macro rather than a blanket impl to avoid breaking backwards compatibility
// with 0.12.x, while still sharing the same impl.
hash_map!(String);
hash_map!(syn::Ident);
hash_map!(syn::Path);

/// Tests for `FromMeta` implementations. Wherever the word `ignore` appears in test input,
/// it should not be considered by the parsing.
#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::parse_quote;

    use crate::{Error, FromMeta, Result};

    /// parse a string as a syn::Meta instance.
    fn pm(tokens: TokenStream) -> ::std::result::Result<syn::Meta, String> {
        let attribute: syn::Attribute = parse_quote!(#[#tokens]);
        attribute.parse_meta().map_err(|_| "Unable to parse".into())
    }

    fn fm<T: FromMeta>(tokens: TokenStream) -> T {
        FromMeta::from_meta(&pm(tokens).expect("Tests should pass well-formed input"))
            .expect("Tests should pass valid input")
    }

    #[test]
    fn unit_succeeds() {
        fm::<()>(quote!(ignore));
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn bool_succeeds() {
        // word format
        assert_eq!(fm::<bool>(quote!(ignore)), true);

        // bool literal
        assert_eq!(fm::<bool>(quote!(ignore = true)), true);
        assert_eq!(fm::<bool>(quote!(ignore = false)), false);

        // string literals
        assert_eq!(fm::<bool>(quote!(ignore = "true")), true);
        assert_eq!(fm::<bool>(quote!(ignore = "false")), false);
    }

    #[test]
    fn char_succeeds() {
        // char literal
        assert_eq!(fm::<char>(quote!(ignore = '😬')), '😬');

        // string literal
        assert_eq!(fm::<char>(quote!(ignore = "😬")), '😬');
    }

    #[test]
    fn string_succeeds() {
        // cooked form
        assert_eq!(&fm::<String>(quote!(ignore = "world")), "world");

        // raw form
        assert_eq!(&fm::<String>(quote!(ignore = r#"world"#)), "world");
    }

    #[test]
    #[allow(clippy::float_cmp)] // we want exact equality
    fn number_succeeds() {
        assert_eq!(fm::<u8>(quote!(ignore = "2")), 2u8);
        assert_eq!(fm::<i16>(quote!(ignore = "-25")), -25i16);
        assert_eq!(fm::<f64>(quote!(ignore = "1.4e10")), 1.4e10);
    }

    #[test]
    fn int_without_quotes() {
        assert_eq!(fm::<u8>(quote!(ignore = 2)), 2u8);
        assert_eq!(fm::<u16>(quote!(ignore = 255)), 255u16);
        assert_eq!(fm::<u32>(quote!(ignore = 5000)), 5000u32);

        // Check that we aren't tripped up by incorrect suffixes
        assert_eq!(fm::<u32>(quote!(ignore = 5000i32)), 5000u32);
    }

    #[test]
    #[allow(clippy::float_cmp)] // we want exact equality
    fn float_without_quotes() {
        assert_eq!(fm::<f32>(quote!(ignore = 2.)), 2.0f32);
        assert_eq!(fm::<f32>(quote!(ignore = 2.0)), 2.0f32);
        assert_eq!(fm::<f64>(quote!(ignore = 1.4e10)), 1.4e10f64);
    }

    #[test]
    fn meta_succeeds() {
        use syn::Meta;

        assert_eq!(
            fm::<Meta>(quote!(hello(world, today))),
            pm(quote!(hello(world, today))).unwrap()
        );
    }

    #[test]
    fn hash_map_succeeds() {
        use std::collections::HashMap;

        let comparison = {
            let mut c = HashMap::new();
            c.insert("hello".to_string(), true);
            c.insert("world".to_string(), false);
            c.insert("there".to_string(), true);
            c
        };

        assert_eq!(
            fm::<HashMap<String, bool>>(quote!(ignore(hello, world = false, there = "true"))),
            comparison
        );
    }

    /// Check that a `HashMap` cannot have duplicate keys, and that the generated error
    /// is assigned a span to correctly target the diagnostic message.
    #[test]
    fn hash_map_duplicate() {
        use std::collections::HashMap;

        let err: Result<HashMap<String, bool>> =
            FromMeta::from_meta(&pm(quote!(ignore(hello, hello = false))).unwrap());

        let err = err.expect_err("Duplicate keys in HashMap should error");

        assert!(err.has_span());
        assert_eq!(err.to_string(), Error::duplicate_field("hello").to_string());
    }

    #[test]
    fn hash_map_multiple_errors() {
        use std::collections::HashMap;

        let err = HashMap::<String, bool>::from_meta(
            &pm(quote!(ignore(hello, hello = 3, hello = false))).unwrap(),
        )
        .expect_err("Duplicates and bad values should error");

        assert_eq!(err.len(), 3);
        let errors = err.into_iter().collect::<Vec<_>>();
        assert!(errors[0].has_span());
        assert!(errors[1].has_span());
        assert!(errors[2].has_span());
    }

    #[test]
    fn hash_map_ident_succeeds() {
        use std::collections::HashMap;
        use syn::parse_quote;

        let comparison = {
            let mut c = HashMap::<syn::Ident, bool>::new();
            c.insert(parse_quote!(first), true);
            c.insert(parse_quote!(second), false);
            c
        };

        assert_eq!(
            fm::<HashMap<syn::Ident, bool>>(quote!(ignore(first, second = false))),
            comparison
        );
    }

    #[test]
    fn hash_map_ident_rejects_non_idents() {
        use std::collections::HashMap;

        let err: Result<HashMap<syn::Ident, bool>> =
            FromMeta::from_meta(&pm(quote!(ignore(first, the::second))).unwrap());

        err.unwrap_err();
    }

    #[test]
    fn hash_map_path_succeeds() {
        use std::collections::HashMap;
        use syn::parse_quote;

        let comparison = {
            let mut c = HashMap::<syn::Path, bool>::new();
            c.insert(parse_quote!(first), true);
            c.insert(parse_quote!(the::second), false);
            c
        };

        assert_eq!(
            fm::<HashMap<syn::Path, bool>>(quote!(ignore(first, the::second = false))),
            comparison
        );
    }

    /// Tests that fallible parsing will always produce an outer `Ok` (from `fm`),
    /// and will accurately preserve the inner contents.
    #[test]
    fn darling_result_succeeds() {
        fm::<Result<()>>(quote!(ignore)).unwrap();
        fm::<Result<()>>(quote!(ignore(world))).unwrap_err();
    }

    /// Test punctuated
    #[test]
    fn test_punctuated() {
        fm::<syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>>(quote!(
            ignore = "a: u8, b: Type"
        ));
        fm::<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>>(quote!(ignore = "a, b, c"));
    }

    #[test]
    fn test_expr_array() {
        fm::<syn::ExprArray>(quote!(ignore = "[0x1, 0x2]"));
        fm::<syn::ExprArray>(quote!(ignore = "[\"Hello World\", \"Test Array\"]"));
    }

    #[test]
    fn test_expr() {
        fm::<syn::Expr>(quote!(ignore = "x + y"));
        fm::<syn::Expr>(quote!(ignore = "an_object.method_call()"));
        fm::<syn::Expr>(quote!(ignore = "{ a_statement(); in_a_block }"));
    }

    #[test]
    fn test_expr_path() {
        fm::<syn::ExprPath>(quote!(ignore = "std::mem::replace"));
        fm::<syn::ExprPath>(quote!(ignore = "x"));
        fm::<syn::ExprPath>(quote!(ignore = "example::<Test>"));
    }

    #[test]
    fn test_number_array() {
        assert_eq!(
            fm::<Vec<u8>>(quote!(ignore = "[16, 0xff]")),
            vec![0x10, 0xff]
        );
        assert_eq!(
            fm::<Vec<u16>>(quote!(ignore = "[32, 0xffff]")),
            vec![0x20, 0xffff]
        );
        assert_eq!(
            fm::<Vec<u32>>(quote!(ignore = "[48, 0xffffffff]")),
            vec![0x30, 0xffffffff]
        );
        assert_eq!(
            fm::<Vec<u64>>(quote!(ignore = "[64, 0xffffffffffffffff]")),
            vec![0x40, 0xffffffffffffffff]
        );
        assert_eq!(
            fm::<Vec<usize>>(quote!(ignore = "[80, 0xffffffff]")),
            vec![0x50, 0xffffffff]
        );
    }
}

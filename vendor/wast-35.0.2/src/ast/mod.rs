/// A macro to create a custom keyword parser.
///
/// This macro is invoked in one of two forms:
///
/// ```
/// // keyword derived from the Rust identifier:
/// wast::custom_keyword!(foo);
///
/// // or an explicitly specified string representation of the keyword:
/// wast::custom_keyword!(my_keyword = "the-wasm-keyword");
/// ```
///
/// This can then be used to parse custom keyword for custom items, such as:
///
/// ```
/// use wast::parser::{Parser, Result, Parse};
///
/// struct InlineModule<'a> {
///     inline_text: &'a str,
/// }
///
/// mod kw {
///     wast::custom_keyword!(inline);
/// }
///
/// // Parse an inline string module of the form:
/// //
/// //    (inline "(module (func))")
/// impl<'a> Parse<'a> for InlineModule<'a> {
///     fn parse(parser: Parser<'a>) -> Result<Self> {
///         parser.parse::<kw::inline>()?;
///         Ok(InlineModule {
///             inline_text: parser.parse()?,
///         })
///     }
/// }
/// ```
///
/// Note that the keyword name can only start with a lower-case letter, i.e. 'a'..'z'.
#[macro_export]
macro_rules! custom_keyword {
    ($name:ident) => {
        $crate::custom_keyword!($name = stringify!($name));
    };
    ($name:ident = $kw:expr) => {
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        #[derive(Debug, Copy, Clone)]
        pub struct $name(pub $crate::Span);

        impl<'a> $crate::parser::Parse<'a> for $name {
            fn parse(parser: $crate::parser::Parser<'a>) -> $crate::parser::Result<Self> {
                parser.step(|c| {
                    if let Some((kw, rest)) = c.keyword() {
                        if kw == $kw {
                            return Ok(($name(c.cur_span()), rest));
                        }
                    }
                    Err(c.error(concat!("expected keyword `", $kw, "`")))
                })
            }
        }

        impl $crate::parser::Peek for $name {
            fn peek(cursor: $crate::parser::Cursor<'_>) -> bool {
                if let Some((kw, _rest)) = cursor.keyword() {
                    kw == $kw
                } else {
                    false
                }
            }

            fn display() -> &'static str {
                concat!("`", $kw, "`")
            }
        }
    };
}

/// A macro for defining custom reserved symbols.
///
/// This is like `custom_keyword!` but for reserved symbols (`Token::Reserved`)
/// instead of keywords (`Token::Keyword`).
///
/// ```
/// use wast::parser::{Parser, Result, Parse};
///
/// // Define a custom reserved symbol, the "spaceship" operator: `<=>`.
/// wast::custom_reserved!(spaceship = "<=>");
///
/// /// A "three-way comparison" like `(<=> a b)` that returns -1 if `a` is less
/// /// than `b`, 0 if they're equal, and 1 if `a` is greater than `b`.
/// struct ThreeWayComparison<'a> {
///     lhs: wast::Expression<'a>,
///     rhs: wast::Expression<'a>,
/// }
///
/// impl<'a> Parse<'a> for ThreeWayComparison<'a> {
///     fn parse(parser: Parser<'a>) -> Result<Self> {
///         parser.parse::<spaceship>()?;
///         let lhs = parser.parse()?;
///         let rhs = parser.parse()?;
///         Ok(ThreeWayComparison { lhs, rhs })
///     }
/// }
/// ```
#[macro_export]
macro_rules! custom_reserved {
    ($name:ident) => {
        $crate::custom_reserved!($name = stringify!($name));
    };
    ($name:ident = $rsv:expr) => {
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        #[derive(Debug)]
        pub struct $name(pub $crate::Span);

        impl<'a> $crate::parser::Parse<'a> for $name {
            fn parse(parser: $crate::parser::Parser<'a>) -> $crate::parser::Result<Self> {
                parser.step(|c| {
                    if let Some((rsv, rest)) = c.reserved() {
                        if rsv == $rsv {
                            return Ok(($name(c.cur_span()), rest));
                        }
                    }
                    Err(c.error(concat!("expected reserved symbol `", $rsv, "`")))
                })
            }
        }

        impl $crate::parser::Peek for $name {
            fn peek(cursor: $crate::parser::Cursor<'_>) -> bool {
                if let Some((rsv, _rest)) = cursor.reserved() {
                    rsv == $rsv
                } else {
                    false
                }
            }

            fn display() -> &'static str {
                concat!("`", $rsv, "`")
            }
        }
    };
}

/// A macro, like [`custom_keyword`], to create a type which can be used to
/// parse/peek annotation directives.
///
/// Note that when you're parsing custom annotations it can be somewhat tricky
/// due to the nature that most of them are skipped. You'll want to be sure to
/// consult the documentation of [`Parser::register_annotation`][register] when
/// using this macro.
///
/// # Examples
///
/// To see an example of how to use this macro, let's invent our own syntax for
/// the [producers section][section] which looks like:
///
/// ```wat
/// (@producer "wat" "1.0.2")
/// ```
///
/// Here, for simplicity, we'll assume everything is a `processed-by` directive,
/// but you could get much more fancy with this as well.
///
/// ```
/// # use wast::*;
/// # use wast::parser::*;
///
/// // First we define the custom annotation keyword we're using, and by
/// // convention we define it in an `annotation` module.
/// mod annotation {
///     wast::annotation!(producer);
/// }
///
/// struct Producer<'a> {
///     name: &'a str,
///     version: &'a str,
/// }
///
/// impl<'a> Parse<'a> for Producer<'a> {
///     fn parse(parser: Parser<'a>) -> Result<Self> {
///         // Remember that parser conventionally parse the *interior* of an
///         // s-expression, so we parse our `@producer` annotation and then we
///         // parse the payload of our annotation.
///         parser.parse::<annotation::producer>()?;
///         Ok(Producer {
///             name: parser.parse()?,
///             version: parser.parse()?,
///         })
///     }
/// }
/// ```
///
/// Note though that this is only half of the parser for annotations. The other
/// half is calling the [`register_annotation`][register] method at the right
/// time to ensure the parser doesn't automatically skip our `@producer`
/// directive. Note that we *can't* call it inside the `Parse for Producer`
/// definition because that's too late and the annotation would already have
/// been skipped.
///
/// Instead we'll need to call it from a higher level-parser before the
/// parenthesis have been parsed, like so:
///
/// ```
/// # use wast::*;
/// # use wast::parser::*;
/// struct Module<'a> {
///     fields: Vec<ModuleField<'a>>,
/// }
///
/// impl<'a> Parse<'a> for Module<'a> {
///     fn parse(parser: Parser<'a>) -> Result<Self> {
///         // .. parse module header here ...
///
///         // register our custom `@producer` annotation before we start
///         // parsing the parentheses of each field
///         let _r = parser.register_annotation("producer");
///
///         let mut fields = Vec::new();
///         while !parser.is_empty() {
///             fields.push(parser.parens(|p| p.parse())?);
///         }
///         Ok(Module { fields })
///     }
/// }
///
/// enum ModuleField<'a> {
///     Producer(Producer<'a>),
///     // ...
/// }
/// # struct Producer<'a>(&'a str);
/// # impl<'a> Parse<'a> for Producer<'a> {
/// #   fn parse(parser: Parser<'a>) -> Result<Self> { Ok(Producer(parser.parse()?)) }
/// # }
/// # mod annotation { wast::annotation!(producer); }
///
/// impl<'a> Parse<'a> for ModuleField<'a> {
///     fn parse(parser: Parser<'a>) -> Result<Self> {
///         // and here `peek` works and our delegated parsing works because the
///         // annotation has been registered.
///         if parser.peek::<annotation::producer>() {
///             return Ok(ModuleField::Producer(parser.parse()?));
///         }
///
///         // .. typically we'd parse other module fields here...
///
///         Err(parser.error("unknown module field"))
///     }
/// }
/// ```
///
/// [register]: crate::parser::Parser::register_annotation
/// [section]: https://github.com/WebAssembly/tool-conventions/blob/master/ProducersSection.md
#[macro_export]
macro_rules! annotation {
    ($name:ident) => {
        $crate::annotation!($name = stringify!($name));
    };
    ($name:ident = $annotation:expr) => {
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        #[derive(Debug)]
        pub struct $name(pub $crate::Span);

        impl<'a> $crate::parser::Parse<'a> for $name {
            fn parse(parser: $crate::parser::Parser<'a>) -> $crate::parser::Result<Self> {
                parser.step(|c| {
                    if let Some((a, rest)) = c.annotation() {
                        if a == $annotation {
                            return Ok(($name(c.cur_span()), rest));
                        }
                    }
                    Err(c.error(concat!("expected annotation `@", $annotation, "`")))
                })
            }
        }

        impl $crate::parser::Peek for $name {
            fn peek(cursor: $crate::parser::Cursor<'_>) -> bool {
                if let Some((a, _rest)) = cursor.annotation() {
                    a == $annotation
                } else {
                    false
                }
            }

            fn display() -> &'static str {
                concat!("`@", $annotation, "`")
            }
        }
    };
}

macro_rules! reexport {
    ($(mod $name:ident;)*) => ($(mod $name; pub use self::$name::*;)*);
}

reexport! {
    mod token;
}

#[cfg(feature = "wasm-module")]
reexport! {
    mod alias;
    mod assert_expr;
    mod custom;
    mod event;
    mod export;
    mod expr;
    mod func;
    mod global;
    mod import;
    mod instance;
    mod memory;
    mod module;
    mod nested_module;
    mod table;
    mod types;
    mod wast;
}

/// Common keyword used to parse WebAssembly text files.
pub mod kw {
    custom_keyword!(after);
    custom_keyword!(alias);
    custom_keyword!(any);
    custom_keyword!(anyfunc);
    custom_keyword!(anyref);
    custom_keyword!(arg);
    custom_keyword!(array);
    custom_keyword!(assert_exhaustion);
    custom_keyword!(assert_invalid);
    custom_keyword!(assert_malformed);
    custom_keyword!(assert_return);
    custom_keyword!(assert_return_arithmetic_nan);
    custom_keyword!(assert_return_arithmetic_nan_f32x4);
    custom_keyword!(assert_return_arithmetic_nan_f64x2);
    custom_keyword!(assert_return_canonical_nan);
    custom_keyword!(assert_return_canonical_nan_f32x4);
    custom_keyword!(assert_return_canonical_nan_f64x2);
    custom_keyword!(assert_return_func);
    custom_keyword!(assert_trap);
    custom_keyword!(assert_unlinkable);
    custom_keyword!(before);
    custom_keyword!(binary);
    custom_keyword!(block);
    custom_keyword!(catch);
    custom_keyword!(catch_all);
    custom_keyword!(code);
    custom_keyword!(data);
    custom_keyword!(dataref);
    custom_keyword!(declare);
    custom_keyword!(delegate);
    custom_keyword!(r#do = "do");
    custom_keyword!(elem);
    custom_keyword!(end);
    custom_keyword!(event);
    custom_keyword!(export);
    custom_keyword!(r#extern = "extern");
    custom_keyword!(externref);
    custom_keyword!(eq);
    custom_keyword!(eqref);
    custom_keyword!(f32);
    custom_keyword!(f32x4);
    custom_keyword!(f64);
    custom_keyword!(f64x2);
    custom_keyword!(field);
    custom_keyword!(first);
    custom_keyword!(func);
    custom_keyword!(funcref);
    custom_keyword!(get);
    custom_keyword!(global);
    custom_keyword!(i16);
    custom_keyword!(i16x8);
    custom_keyword!(i31);
    custom_keyword!(i31ref);
    custom_keyword!(i32);
    custom_keyword!(i32x4);
    custom_keyword!(i64);
    custom_keyword!(i64x2);
    custom_keyword!(i8);
    custom_keyword!(i8x16);
    custom_keyword!(import);
    custom_keyword!(instance);
    custom_keyword!(instantiate);
    custom_keyword!(invoke);
    custom_keyword!(item);
    custom_keyword!(last);
    custom_keyword!(local);
    custom_keyword!(memory);
    custom_keyword!(module);
    custom_keyword!(modulecode);
    custom_keyword!(nan_arithmetic = "nan:arithmetic");
    custom_keyword!(nan_canonical = "nan:canonical");
    custom_keyword!(null);
    custom_keyword!(nullref);
    custom_keyword!(offset);
    custom_keyword!(outer);
    custom_keyword!(param);
    custom_keyword!(parent);
    custom_keyword!(passive);
    custom_keyword!(quote);
    custom_keyword!(r#else = "else");
    custom_keyword!(r#if = "if");
    custom_keyword!(r#loop = "loop");
    custom_keyword!(r#mut = "mut");
    custom_keyword!(r#type = "type");
    custom_keyword!(r#ref = "ref");
    custom_keyword!(ref_func = "ref.func");
    custom_keyword!(ref_null = "ref.null");
    custom_keyword!(register);
    custom_keyword!(result);
    custom_keyword!(rtt);
    custom_keyword!(shared);
    custom_keyword!(start);
    custom_keyword!(r#struct = "struct");
    custom_keyword!(table);
    custom_keyword!(then);
    custom_keyword!(r#try = "try");
    custom_keyword!(unwind);
    custom_keyword!(v128);
}

/// Common annotations used to parse WebAssembly text files.
pub mod annotation {
    annotation!(custom);
    annotation!(name);
}

use core::fmt;

use crate::helper::{compare_encodings, Helper, NestingLevel};
use crate::parse::Parser;
use crate::EncodingBox;

/// An Objective-C type-encoding.
///
/// Can be retrieved in Objective-C for a type `T` using the `@encode(T)`
/// directive.
/// ```objc
/// NSLog(@"Encoding of NSException: %s", @encode(NSException));
/// ```
///
/// The [`Display`][`fmt::Display`] implementation converts the [`Encoding`]
/// into its string representation, that the the `@encode` directive would
/// return. This can be used conveniently through the `to_string` method:
///
/// ```
/// use objc2_encode::Encoding;
/// assert_eq!(Encoding::Int.to_string(), "i");
/// ```
///
/// For more information on the string value of an encoding, see [Apple's
/// documentation][ocrtTypeEncodings].
///
/// [ocrtTypeEncodings]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html
///
/// # Examples
///
/// Comparing an encoding to a string from the Objective-C runtime:
///
/// ```
/// use objc2_encode::Encoding;
/// assert!(Encoding::Array(10, &Encoding::FloatComplex).equivalent_to_str("[10jf]"));
/// ```
// Not `Copy`, since this may one day be merged with `EncodingBox`
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
// See <https://en.cppreference.com/w/c/language/type>
#[non_exhaustive] // Maybe we're missing some encodings?
pub enum Encoding {
    /// A C `char`. Corresponds to the `"c"` code.
    Char,
    /// A C `short`. Corresponds to the `"s"` code.
    Short,
    /// A C `int`. Corresponds to the `"i"` code.
    Int,
    /// A C `long`. Corresponds to the `"l"` code.
    ///
    /// This is treated as a 32-bit quantity in 64-bit programs, see
    /// [`Encoding::C_LONG`].
    Long,
    /// A C `long long`. Corresponds to the `"q"` code.
    LongLong,
    /// A C `unsigned char`. Corresponds to the `"C"` code.
    UChar,
    /// A C `unsigned short`. Corresponds to the `"S"` code.
    UShort,
    /// A C `unsigned int`. Corresponds to the `"I"` code.
    UInt,
    /// A C `unsigned long`. Corresponds to the `"L"` code.
    ///
    /// See [`Encoding::C_ULONG`].
    ULong,
    /// A C `unsigned long long`. Corresponds to the `"Q"` code.
    ULongLong,
    /// A C `float`. Corresponds to the `"f"` code.
    Float,
    /// A C `double`. Corresponds to the `"d"` code.
    Double,
    /// A C `long double`. Corresponds to the `"D"` code.
    LongDouble,
    /// A C `float _Complex`. Corresponds to the `"j" "f"` code.
    FloatComplex,
    /// A C `_Complex` or `double _Complex`. Corresponds to the `"j" "d"` code.
    DoubleComplex,
    /// A C `long double _Complex`. Corresponds to the `"j" "D"` code.
    LongDoubleComplex,
    /// A C++ `bool` / C99 `_Bool`. Corresponds to the `"B"` code.
    Bool,
    /// A C `void`. Corresponds to the `"v"` code.
    Void,
    /// A C `char *`. Corresponds to the `"*"` code.
    String,
    /// An Objective-C object (`id`). Corresponds to the `"@"` code.
    ///
    /// Some compilers may choose to store the name of the class in instance
    /// variables and properties as `"@" class_name`, see [Extended Type Info
    /// in Objective-C][ext] (note that this does not include generics).
    ///
    /// Such class names are currently ignored by `objc2-encode`.
    ///
    /// [ext]: https://bou.io/ExtendedTypeInfoInObjC.html
    Object,
    /// An Objective-C block. Corresponds to the `"@" "?"` code.
    Block,
    /// An Objective-C class (`Class`). Corresponds to the `"#"` code.
    Class,
    /// An Objective-C selector (`SEL`). Corresponds to the `":"` code.
    Sel,
    /// An unknown type. Corresponds to the `"?"` code.
    ///
    /// This is usually used to encode functions.
    Unknown,
    /// A bitfield with the given number of bits, and the given type.
    ///
    /// Corresponds to the `"b" size` code.
    ///
    /// On GNUStep, this uses the `"b" offset type size` code, so this
    /// contains an `Option` that should be set for that. Only integral types
    /// are possible for the type.
    ///
    /// A `BitField(_, Some(_))` and a `BitField(_, None)` do _not_ compare
    /// equal; instead, you should set the bitfield correctly depending on the
    /// target platform.
    BitField(u8, Option<&'static (u64, Encoding)>),
    /// A pointer to the given type.
    ///
    /// Corresponds to the `"^" type` code.
    Pointer(&'static Encoding),
    /// A C11 [`_Atomic`] type.
    ///
    /// Corresponds to the `"A" type` code. Not all encodings are possible in
    /// this.
    ///
    /// [`_Atomic`]: https://en.cppreference.com/w/c/language/atomic
    Atomic(&'static Encoding),
    /// An array with the given length and type.
    ///
    /// Corresponds to the `"[" length type "]"` code.
    Array(u64, &'static Encoding),
    /// A struct with the given name and fields.
    ///
    /// The order of the fields must match the order of the order in this.
    ///
    /// It is not uncommon for the name to be `"?"`.
    ///
    /// Corresponds to the `"{" name "=" fields... "}"` code.
    ///
    /// Note that the `=` may be omitted in some situations; this is
    /// considered equal to the case where there are no fields.
    Struct(&'static str, &'static [Encoding]),
    /// A union with the given name and members.
    ///
    /// The order of the members must match the order of the order in this.
    ///
    /// Corresponds to the `"(" name "=" members... ")"` code.
    ///
    /// Note that the `=` may be omitted in some situations; this is
    /// considered equal to the case where there are no members.
    Union(&'static str, &'static [Encoding]),
    /// The type does not have an Objective-C encoding.
    ///
    /// This is usually only used on types where Clang fails to generate the
    /// Objective-C encoding, like SIMD types marked with
    /// `__attribute__((__ext_vector_type__(1)))`.
    None,
    // TODO: "Vector" types have the '!' encoding, but are not implemented in
    // clang

    // TODO: `t` and `T` codes for i128 and u128?
}

impl Encoding {
    /// The encoding of [`c_long`](`std::os::raw::c_long`) on the current
    /// target.
    ///
    /// Ideally the encoding of `long` would be the same as `int` when it's 32
    /// bits wide and the same as `long long` when it is 64 bits wide; then
    /// `c_long::ENCODING` would just work.
    ///
    /// Unfortunately, `long` have a different encoding than `int` when it is
    /// 32 bits wide; the [`l`][`Encoding::Long`] encoding.
    pub const C_LONG: Self = {
        // TODO once `core::ffi::c_long` is in MSRV
        // `mem::size_of::<c_long>() == 4`
        //
        // That would exactly match what `clang` does:
        // https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/AST/ASTContext.cpp#L7245
        if cfg!(any(target_pointer_width = "32", windows)) {
            // @encode(long) = 'l'
            Self::Long
        } else {
            // @encode(long) = 'q'
            Self::LongLong
        }
    };

    /// The encoding of [`c_ulong`](`std::os::raw::c_ulong`) on the current
    /// target.
    ///
    /// See [`Encoding::C_LONG`] for explanation.
    pub const C_ULONG: Self = {
        if cfg!(any(target_pointer_width = "32", windows)) {
            // @encode(unsigned long) = 'L'
            Self::ULong
        } else {
            // @encode(unsigned long) = 'Q'
            Self::ULongLong
        }
    };

    /// Check if one encoding is equivalent to another.
    ///
    /// Currently, equivalence testing mostly requires that the encodings are
    /// equal, except for:
    /// - Any leading qualifiers that the encoding may have.
    /// - Structs or unions behind multiple pointers are considered
    ///   equivalent, since Objective-C compilers strip this information to
    ///   avoid unnecessary nesting.
    /// - Structs or unions with no fields/members are considered to represent
    ///   "opqaue" types, and will therefore be equivalent to all other
    ///   structs / unions.
    /// - [`Object`], [`Block`] and [`Class`] compare as equivalent.
    ///
    /// The comparison may be changed in the future to e.g. ignore struct
    /// names or similar changes that may be required because of limitations
    /// in Objective-C compiler implementations.
    ///
    /// For example, you should not rely on two equivalent encodings to have
    /// the same size or ABI - that is provided on a best-effort basis.
    ///
    /// [`Object`]: Self::Object
    /// [`Block`]: Self::Block
    /// [`Class`]: Self::Class
    pub fn equivalent_to(&self, other: &Self) -> bool {
        compare_encodings(self, other, NestingLevel::new(), false)
    }

    /// Check if an encoding is equivalent to the given string representation.
    ///
    /// See [`Encoding::equivalent_to`] for details about the meaning of
    /// "equivalence".
    pub fn equivalent_to_str(&self, s: &str) -> bool {
        let mut parser = Parser::new(s);

        parser.strip_leading_qualifiers();

        if let Some(()) = parser.expect_encoding(self, NestingLevel::new()) {
            // if the given encoding can be successfully removed from the
            // start and an empty string remains, they were fully equivalent!
            parser.is_empty()
        } else {
            false
        }
    }

    /// Check if an encoding is equivalent to a boxed encoding.
    ///
    /// See [`Encoding::equivalent_to`] for details about the meaning of
    /// "equivalence".
    pub fn equivalent_to_box(&self, other: &EncodingBox) -> bool {
        compare_encodings(self, other, NestingLevel::new(), false)
    }

    /// Computes the theoretical size in bytes of the represented value type.
    ///
    /// The size is only valid for the current target.
    ///
    /// This does not currently consider alignment, i.e. everything is
    /// considered packed, but that may change in the future.
    pub fn size(&self) -> Option<usize> {
        Helper::new(self).size(NestingLevel::new())
    }
}

/// Formats this [`Encoding`] in a similar way that the `@encode` directive
/// would ordinarily do.
///
/// You should not rely on the output of this to be stable across versions. It
/// may change if found to be required to be compatible with existing
/// Objective-C compilers.
impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Helper::new(self).fmt(f, NestingLevel::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::static_str::{static_encoding_str_array, static_encoding_str_len};
    use alloc::boxed::Box;
    use alloc::string::ToString;
    use alloc::vec;
    use core::str::FromStr;

    fn send_sync<T: Send + Sync>() {}

    #[test]
    fn test_send_sync() {
        send_sync::<Encoding>();
    }

    #[test]
    fn smoke() {
        assert!(Encoding::Short.equivalent_to_str("s"));
    }

    #[test]
    fn qualifiers() {
        assert!(Encoding::Void.equivalent_to_str("v"));
        assert!(Encoding::Void.equivalent_to_str("Vv"));
        assert!(Encoding::String.equivalent_to_str("*"));
        assert!(Encoding::String.equivalent_to_str("r*"));
    }

    macro_rules! assert_enc {
        ($(
            fn $name:ident() {
                $encoding:expr;
                $(
                    ~$equivalent_encoding:expr;
                )*
                $(
                    !$not_encoding:expr;
                )*
                $string:literal;
                $(
                    ~$equivalent_string:expr;
                )*
                $(
                    !$not_string:literal;
                )*
            }
        )+) => {$(
            #[test]
            fn $name() {
                const E: Encoding = $encoding;

                // Check PartialEq
                assert_eq!(E, E, "equal");

                // Check Display
                assert_eq!(E.to_string(), $string, "equal to string");

                // Check encoding box
                let boxed = EncodingBox::from_str($string).expect("parse");
                assert_eq!(boxed.to_string(), $string, "parsed");

                // Check equivalence comparisons
                assert!(E.equivalent_to(&E), "equivalent self");
                assert!(E.equivalent_to_str($string), "equivalent self string {}", $string);
                assert!(E.equivalent_to_box(&boxed), "equivalent self boxed");
                $(
                    assert!(E.equivalent_to(&$equivalent_encoding), "equivalent encoding {}", $equivalent_encoding);
                    assert!(E.equivalent_to_str(&$equivalent_encoding.to_string()), "equivalent encoding string");
                    let boxed = EncodingBox::from_str(&$equivalent_encoding.to_string()).expect("parse equivalent encoding");
                    assert!(E.equivalent_to_box(&boxed), "equivalent encoding boxed");
                )*
                $(
                    assert!(E.equivalent_to_str($equivalent_string), "equivalent string {}", $equivalent_string);
                    let boxed = EncodingBox::from_str($equivalent_string).expect("parse equivalent string");
                    assert!(E.equivalent_to_box(&boxed), "equivalent string boxed");
                )*

                // Negative checks
                $(
                    assert_ne!(E, $not_encoding, "not equal");
                    assert!(!E.equivalent_to(&$not_encoding), "not equivalent encoding");
                    assert!(!E.equivalent_to_str(&$not_encoding.to_string()), "not equivalent encoding string");
                    let boxed = EncodingBox::from_str(&$not_encoding.to_string()).expect("parse not equivalent encoding");
                    assert!(!E.equivalent_to_box(&boxed), "not equivalent boxed");
                )*
                $(
                    assert!(!E.equivalent_to_str(&$not_string), "not equivalent string");
                )*

                // Check static str
                const STATIC_ENCODING_DATA: [u8; static_encoding_str_len(&E, NestingLevel::new())] = static_encoding_str_array(&E, NestingLevel::new());
                const STATIC_ENCODING_STR: &str = unsafe { core::str::from_utf8_unchecked(&STATIC_ENCODING_DATA) };
                assert_eq!(STATIC_ENCODING_STR, $string, "static");
            }
        )+};
    }

    assert_enc! {
        fn int() {
            Encoding::Int;
            !Encoding::Char;
            "i";
        }

        fn char() {
            Encoding::Char;
            !Encoding::Int;
            "c";
            // Qualifiers
            ~"rc";
            ~"nc";
            ~"Nc";
            ~"oc";
            ~"Oc";
            ~"Rc";
            ~"Vc";

            !"ri";
        }

        fn block() {
            Encoding::Block;
            ~Encoding::Class;
            ~Encoding::Object;
            !Encoding::Unknown;
            "@?";
        }

        fn object() {
            Encoding::Object;
            ~Encoding::Block;
            ~Encoding::Class;
            !Encoding::Sel;
            "@";
            ~"@\"AnyClassName\"";
            ~"@\"\""; // Empty class name
            ~"@?";
            ~"#";
            !"@\"MyClassName";
            !"@MyClassName\"";
        }

        fn unknown() {
            Encoding::Unknown;
            !Encoding::Block;
            "?";
        }

        fn double() {
            Encoding::Double;
            "d";
        }

        fn bitfield() {
            Encoding::BitField(4, None);
            !Encoding::Int;
            !Encoding::BitField(5, None);
            !Encoding::BitField(4, Some(&(0, Encoding::Bool)));
            "b4";
            !"b4a";
            !"b4c";
            !"b4B";
            !"b";
            !"b-4";
            !"b0B4";
        }

        fn bitfield_gnustep() {
            Encoding::BitField(4, Some(&(16, Encoding::Bool)));
            !Encoding::Int;
            !Encoding::BitField(4, None);
            !Encoding::BitField(5, Some(&(16, Encoding::Bool)));
            !Encoding::BitField(4, Some(&(20, Encoding::Bool)));
            !Encoding::BitField(4, Some(&(16, Encoding::Char)));
            "b16B4";
            !"b4";
            !"b16B";
            !"b20B4";
            !"b16B5";
            !"b16c4";
            !"b4a";
            !"b";
            !"b-4";
        }

        fn atomic() {
            Encoding::Atomic(&Encoding::Int);
            !Encoding::Pointer(&Encoding::Int);
            !Encoding::Atomic(&Encoding::Char);
            !Encoding::Atomic(&Encoding::Atomic(&Encoding::Int));
            "Ai";
        }

        fn atomic_string() {
            Encoding::Atomic(&Encoding::String);
            "A*";
        }

        fn pointer() {
            Encoding::Pointer(&Encoding::Int);
            !Encoding::Atomic(&Encoding::Int);
            !Encoding::Pointer(&Encoding::Char);
            !Encoding::Pointer(&Encoding::Pointer(&Encoding::Int));
            "^i";
        }

        fn array() {
            Encoding::Array(12, &Encoding::Int);
            !Encoding::Int;
            !Encoding::Array(11, &Encoding::Int);
            !Encoding::Array(12, &Encoding::Char);
            "[12i]";
            !"[12i";
        }

        fn struct_() {
            Encoding::Struct("SomeStruct", &[Encoding::Char, Encoding::Int]);
            ~Encoding::Struct("SomeStruct", &[]);
            !Encoding::Union("SomeStruct", &[Encoding::Char, Encoding::Int]);
            !Encoding::Int;
            !Encoding::Struct("SomeStruct", &[Encoding::Int]);
            !Encoding::Struct("SomeStruct", &[Encoding::Char, Encoding::Int, Encoding::Int]);
            !Encoding::Struct("SomeStruct", &[Encoding::Int, Encoding::Char]);
            !Encoding::Struct("AnotherName", &[Encoding::Char, Encoding::Int]);
            "{SomeStruct=ci}";
            ~"{SomeStruct=}";
            !"{SomeStruct}";
            !"{SomeStruct=ic}";
            !"{SomeStruct=malformed";
        }

        fn pointer_struct() {
            Encoding::Pointer(&Encoding::Struct("SomeStruct", &[Encoding::Char, Encoding::Int]));
            ~Encoding::Pointer(&Encoding::Struct("SomeStruct", &[]));
            !Encoding::Pointer(&Encoding::Struct("SomeStruct", &[Encoding::Int, Encoding::Char]));
            !Encoding::Pointer(&Encoding::Struct("AnotherName", &[Encoding::Char, Encoding::Int]));
            "^{SomeStruct=ci}";
            ~"^{SomeStruct=}";
            !"^{SomeStruct}";
            !"^{SomeStruct=ic}";
            !"^{SomeStruct=malformed";
        }

        fn pointer_pointer_struct() {
            Encoding::Pointer(&Encoding::Pointer(&Encoding::Struct("SomeStruct", &[Encoding::Char, Encoding::Int])));
            ~Encoding::Pointer(&Encoding::Pointer(&Encoding::Struct("SomeStruct", &[])));
            ~Encoding::Pointer(&Encoding::Pointer(&Encoding::Struct("SomeStruct", &[Encoding::Int, Encoding::Char])));
            !Encoding::Pointer(&Encoding::Pointer(&Encoding::Struct("AnotherName", &[Encoding::Char, Encoding::Int])));
            "^^{SomeStruct}";
            !"^^{SomeStruct=}";
            !"^^{SomeStruct=ci}";
            !"^^{SomeStruct=ic}";
            !"^^{AnotherName=ic}";
            !"^^{SomeStruct=malformed";
        }

        fn atomic_struct() {
            Encoding::Atomic(&Encoding::Struct("SomeStruct", &[Encoding::Char, Encoding::Int]));
            ~Encoding::Atomic(&Encoding::Struct("SomeStruct", &[]));
            ~Encoding::Atomic(&Encoding::Struct("SomeStruct", &[Encoding::Int, Encoding::Char]));
            !Encoding::Atomic(&Encoding::Struct("AnotherName", &[Encoding::Char, Encoding::Int]));
            "A{SomeStruct}";
            !"A{SomeStruct=}";
            !"A{SomeStruct=ci}";
            !"A{SomeStruct=ic}";
            !"A{SomeStruct=malformed";
        }

        fn empty_struct() {
            Encoding::Struct("SomeStruct", &[]);
            "{SomeStruct=}";
            ~"{SomeStruct=ci}";
            !"{SomeStruct}";
        }

        fn union_() {
            Encoding::Union("Onion", &[Encoding::Char, Encoding::Int]);
            !Encoding::Struct("Onion", &[Encoding::Char, Encoding::Int]);
            !Encoding::Int;
            !Encoding::Union("Onion", &[Encoding::Int, Encoding::Char]);
            !Encoding::Union("AnotherUnion", &[Encoding::Char, Encoding::Int]);
            "(Onion=ci)";
            !"(Onion=ci";
        }

        fn nested() {
            Encoding::Struct(
                "A",
                &[
                    Encoding::Struct("B", &[Encoding::Int]),
                    Encoding::Pointer(&Encoding::Struct("C", &[Encoding::Double])),
                    Encoding::Char,
                ],
            );
            ~Encoding::Struct(
                "A",
                &[
                    Encoding::Struct("B", &[Encoding::Int]),
                    Encoding::Pointer(&Encoding::Struct("C", &[])),
                    Encoding::Char,
                ],
            );
            "{A={B=i}^{C}c}";
            !"{A={B=i}^{C=d}c}";
            !"{A={B=i}^{C=i}c}";
            !"{A={B=i}^{C=d}c";
        }

        fn nested_pointer() {
            Encoding::Pointer(&Encoding::Struct(
                "A",
                &[
                    Encoding::Struct("B", &[Encoding::Int]),
                    Encoding::Pointer(&Encoding::Struct("C", &[Encoding::Double])),
                ],
            ));
            "^{A={B=i}^{C}}";
            !"^{A={B}^{C}}";
            !"^{A={B=i}^{C=d}}";
        }

        fn various() {
            Encoding::Struct(
                "abc",
                &[
                    Encoding::Pointer(&Encoding::Array(8, &Encoding::Bool)),
                    Encoding::Union("def", &[Encoding::Block]),
                    Encoding::Pointer(&Encoding::Pointer(&Encoding::BitField(255, None))),
                    Encoding::Char,
                    Encoding::Unknown,
                ]
            );
            "{abc=^[8B](def=@?)^^b255c?}";
            ~"{abc=}";
            !"{abc}";
        }

        fn identifier() {
            Encoding::Struct("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", &[]);
            "{_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789=}";
        }

        // Regression test. The encoding of the `CGLContextObj` object changed
        // between versions of macOS. As such, this is something that we must
        // be prepared to handle.
        fn cgl_context_obj() {
            Encoding::Pointer(&Encoding::Struct("_CGLContextObject", &[]));
            "^{_CGLContextObject=}";
            ~"^{_CGLContextObject=^{__GLIContextRec}{__GLIFunctionDispatchRec=^?^?^?^?^?}^{_CGLPrivateObject}^v}";
            !"^{_CGLContextObject}";
            !"^{SomeOtherStruct=}";
        }

        fn none() {
            Encoding::None;
            "";
            !"?";
        }

        fn none_in_array() {
            Encoding::Array(42, &Encoding::None);
            !Encoding::Array(42, &Encoding::Unknown);
            "[42]";
            !"[42i]";
        }

        fn none_in_pointer() {
            Encoding::Pointer(&Encoding::None);
            !Encoding::Pointer(&Encoding::Unknown);
            "^";
            !"";
            !"^i";
        }

        fn none_in_pointer_in_array() {
            Encoding::Array(42, &Encoding::Pointer(&Encoding::None));
            "[42^]";
        }

        fn class() {
            Encoding::Class;
            ~Encoding::Object;
            ~Encoding::Block;
            !Encoding::Sel;
            "#";
            ~"@?";
            ~"@";
            !"a";
        }
    }

    #[test]
    #[should_panic = "Struct name was not a valid identifier"]
    fn struct_empty() {
        let _ = Encoding::Struct("", &[]).to_string();
    }

    #[test]
    #[should_panic = "Struct name was not a valid identifier"]
    fn struct_unicode() {
        let _ = Encoding::Struct("☃", &[Encoding::Char]).to_string();
    }

    #[test]
    #[should_panic = "Union name was not a valid identifier"]
    fn union_invalid_identifier() {
        let _ = Encoding::Union("a-b", &[Encoding::Char]).equivalent_to_str("(☃=c)");
    }

    // Note: A raw `?` cannot happen in practice, since functions can only
    // be accessed through pointers, and that will yield `^?`
    #[test]
    fn object_unknown_in_struct() {
        let enc = Encoding::Struct("S", &[Encoding::Block, Encoding::Object, Encoding::Unknown]);
        let s = "{S=@?@?}";

        assert_eq!(&enc.to_string(), s);

        let parsed = EncodingBox::from_str(s).unwrap();
        let expected = EncodingBox::Struct(
            "S".to_string(),
            vec![EncodingBox::Block, EncodingBox::Block],
        );
        assert_eq!(parsed, expected);

        assert!(!enc.equivalent_to_box(&expected));
    }

    // Similar to `?`, `` cannot be accurately represented inside pointers
    // inside structs, and may be parsed incorrectly.
    #[test]
    fn none_in_struct() {
        let enc = Encoding::Struct("?", &[Encoding::Pointer(&Encoding::None), Encoding::Int]);
        let s = "{?=^i}";
        assert_eq!(&enc.to_string(), s);

        let parsed = EncodingBox::from_str(s).unwrap();
        let expected = EncodingBox::Struct(
            "?".to_string(),
            vec![EncodingBox::Pointer(Box::new(EncodingBox::Int))],
        );
        assert_eq!(parsed, expected);

        assert!(!enc.equivalent_to_box(&expected));
    }
}

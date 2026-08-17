//! This module tests for errors that happen in the expanded code,
//! errors detectable by the macro itself are tested in the proc macro crate.

#![allow(non_camel_case_types)]

///
/// ```rust
///
/// struct Foo<T>(T);
///
/// const_format::impl_fmt!{
///     impl[T,] Foo<T>
///     where[T: 'static,];
///
///     fn foo(){}
///
/// }
/// ```
///
/// ```compile_fail
///
/// struct Foo<T>(T);
///
/// const_format::impl_fmt!{
///     impl[T,] Foo<T>
///     where[asodkaspodaoskd,];
///
///     fn foo(){}
/// }
/// ```
///
/// ```compile_fail
///
/// struct Foo<T>(T);
///
/// const_format::impl_fmt!{
///     impl[T,] Foo<T>
///     where[T: T];
///
///     fn foo(){}
/// }
/// ```
///
#[cfg(feature = "fmt")]
pub struct ImplFmtWhereClause;

///
/// ```rust
///
/// #[derive(const_format::ConstDebug)]
/// struct Foo<T>(*const T)
/// where T: 'static;
///
/// fn main(){}
/// ```
///
/// ```compile_fail
///
/// #[derive(const_format::ConstDebug)]
/// struct Foo<T>(*const T)
/// where AAAA: AAAA;
///
/// fn main(){}
/// ```
///
#[cfg(feature = "derive")]
pub struct ConstDebugWhereClause;

/// ```rust
///
/// use const_format::StrWriterMut;
///
/// let mut len = 0;
/// let mut buffer = [0; 128];
///
/// let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
///
/// writer.write_str("hello").unwrap();
///
/// assert_eq!(writer.as_bytes(), b"hello")
///
/// ```
///
/// ```compile_fail
///
/// use const_format::StrWriterMut;
///
/// let mut len = 0;
/// let mut buffer = [0; 128];
///
/// let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
///
/// writer.write_str("hello").unwrap();
///
/// assert_eq!(writer.as_str(), "hello")
///
/// ```
///
#[cfg(feature = "fmt")]
pub struct AsStr_For_StrWriterMut_NoEncoding;

/// ```rust
///
/// const_format::assertc!(true, "foo");
///
/// ```
///
/// ```compile_fail
///
/// const_format::assertc!(false, "foo");
///
/// ```
///
/// # With a Formatting argument
///
/// ```rust
///
/// const_format::assertc!(
///     true,
///     "{foo}\n{foo:#?}\n{}",
///     |fmt| { const_format::call_debug_fmt!(array, [100u8], fmt ) },
///     foo = |fmt| { const_format::call_debug_fmt!(array, [(), ()], fmt ) },
/// );
///
/// const_format::assertc!(
///     true,
///     "{foo}\n{foo:#?}\n{}",
///     |fmt| const_format::call_debug_fmt!(array, [100u8], fmt ),
///     foo = |fmt| const_format::call_debug_fmt!(array, [(), ()], fmt ),
/// );
///
/// ```
///
/// ```compile_fail
///
/// const_format::assertc!(
///     false,
///     "{foo}\n{foo:#?}\n{}",
///     |fmt| { const_format::call_debug_fmt!(array, [100u8], fmt ) },
///     foo = |fmt| { const_format::call_debug_fmt!(array, [(), ()], fmt ) },
/// );
///
/// const_format::assertc!(
///     false,
///     "{foo}\n{foo:#?}\n{}",
///     |fmt| const_format::call_debug_fmt!(array, [100u8], fmt ),
///     foo = |fmt| const_format::call_debug_fmt!(array, [(), ()], fmt ),
/// );
///
/// ```
///
#[cfg(feature = "assertc")]
pub struct Assert;

/// # assert_eq
///
/// ```rust
///
/// const_format::assertc_eq!(0u8, 0u8, "foo");
///
/// ```
///
/// ```compile_fail
///
/// const_format::assertc_eq!(0u8, 10u8, "foo");
///
/// ```
///
/// # assert_ne
///
/// ```rust
///
/// const_format::assertc_ne!(0u8, 10u8, "foo");
///
/// ```
///
/// ```compile_fail
///
/// const_format::assertc_ne!(0u8, 0u8, "foo");
///
/// ```
///
#[cfg(feature = "assertc")]
pub struct AssertCmp;

/// ```rust
/// const_format::assertcp!(true, "foo");
/// ```
///
/// ```compile_fail
/// const_format::assertcp!(false, "foo");
/// ```
///
#[cfg(feature = "assertcp")]
pub struct AssertCP;

/// # assert_eq
///
/// ```rust
/// const_format::assertcp_eq!(0u8, 0u8, "foo");
/// ```
///
/// ```compile_fail
/// const_format::assertcp_eq!(0u8, 10u8, "foo");
/// ```
///
/// # assert_ne
///
/// ```rust
/// const_format::assertcp_ne!(0u8, 10u8, "foo");
/// ```
///
/// ```compile_fail
/// const_format::assertcp_ne!(0u8, 0u8, "foo");
/// ```
///
#[cfg(feature = "assertcp")]
pub struct AssertCPCmp;

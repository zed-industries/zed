use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// This macro expands to a set of `pub` Rust modules:
///
/// * The `types` module contains definitions for each `typename` declared in
///   the witx document. Type names are translated to the Rust-idiomatic
///   CamelCase.
///
/// * For each `module` defined in the witx document, a Rust module is defined
///   containing definitions for that module. Module names are translated to the
///   Rust-idiomatic snake\_case.
///
///     * For each `@interface func` defined in a witx module, an abi-level
///       function is generated which takes ABI-level arguments, along with
///       a ref that impls the module trait, and a `GuestMemory` implementation.
///       Users typically won't use these abi-level functions: Either the
///       `wasmtime_integration` macro or the `lucet-wiggle` crates adapt these
///       to work with a particular WebAssembly engine.
///
///     * A public "module trait" is defined (called the module name, in
///       SnakeCase) which has a `&self` method for each function in the
///       module. These methods takes idiomatic Rust types for each argument
///       and return `Result<($return_types),$error_type>`
///
///     * When the `wiggle` crate is built with the `wasmtime_integration`
///       feature, each module contains an `add_to_linker` function to add it to
///       a `wasmtime::Linker`.
///
/// Arguments are provided using Rust struct value syntax.
///
/// * `witx` takes a list of string literal paths. Paths are relative to the
///   CARGO_MANIFEST_DIR of the crate where the macro is invoked. Alternatively,
///   `witx_literal` takes a string containing a complete witx document.
/// * Optional: `errors` takes a mapping of witx identifiers to types, e.g
///   `{ errno => YourErrnoType }`. This allows you to use the `UserErrorConversion`
///   trait to map these rich errors into the flat witx type, or to terminate
///   WebAssembly execution by trapping.
///     * Instead of requiring the user to define an error type, wiggle can
///       generate an error type for the user which has conversions to/from
///       the base type, and permits trapping, using the syntax
///       `errno => trappable AnErrorType`.
/// * Optional: `async` takes a set of witx modules and functions which are
///   made Rust `async` functions in the module trait.
///
/// ## Example
///
/// ```
/// use wiggle::GuestPtr;
/// wiggle::from_witx!({
///     witx_literal: "
///         (typename $errno
///           (enum (@witx tag u32)
///             $ok
///             $invalid_arg
///             $io
///             $overflow))
///          (typename $alias_to_float f32)
///          (module $example
///            (@interface func (export \"int_float_args\")
///              (param $an_int u32)
///              (param $some_floats (list f32))
///              (result $r (expected (error $errno))))
///            (@interface func (export \"double_int_return_float\")
///              (param $an_int u32)
///              (result $r (expected $alias_to_float (error $errno)))))
///     ",
///     errors: { errno => YourRichError },
///     async: { example::double_int_return_float },
/// });
///
/// /// Witx generates a set of traits, which the user must impl on a
/// /// type they define. We call this the ctx type. It stores any context
/// /// these functions need to execute.
/// pub struct YourCtxType {}
///
/// /// Witx provides a hook to translate "rich" (arbitrary Rust type) errors
/// /// into the flat error enums used at the WebAssembly interface. You will
/// /// need to impl the `types::UserErrorConversion` trait to provide a translation
/// /// from this rich type.
/// #[derive(Debug)]
/// pub enum YourRichError {
///     InvalidArg(String),
///     Io(std::io::Error),
///     Overflow,
///     Trap(String),
/// }
///
/// /// The above witx text contains one module called `$example`. So, we must
/// /// implement this one method trait for our ctx type.
/// #[wiggle::async_trait]
/// /// We specified in the `async_` field that `example::double_int_return_float`
/// /// is an asynchronous method. Therefore, we use the `async_trait` proc macro
/// /// to define this trait, so that `double_int_return_float` can be an `async fn`.
/// /// `wiggle::async_trait` is defined as `#[async_trait::async_trait(?Send)]` -
/// /// in wiggle, async methods do not have the Send constraint.
/// impl example::Example for YourCtxType {
///     /// The arrays module has two methods, shown here.
///     /// Note that the `GuestPtr` type comes from `wiggle`,
///     /// whereas the witx-defined types like `Excuse` and `Errno` come
///     /// from the `pub mod types` emitted by the `wiggle::from_witx!`
///     /// invocation above.
///     fn int_float_args(&mut self, _int: u32, _floats: &GuestPtr<[f32]>)
///         -> Result<(), YourRichError> {
///         unimplemented!()
///     }
///     async fn double_int_return_float(&mut self, int: u32)
///         -> Result<f32, YourRichError> {
///         Ok(int.checked_mul(2).ok_or(YourRichError::Overflow)? as f32)
///     }
/// }
///
/// /// For all types used in the `error` an `expected` in the witx document,
/// /// you must implement `GuestErrorType` which tells wiggle-generated
/// /// code what value to return when the method returns Ok(...).
/// impl wiggle::GuestErrorType for types::Errno {
///     fn success() -> Self {
///         unimplemented!()
///     }
/// }
///
/// /// If you specify a `error` mapping to the macro, you must implement the
/// /// `types::UserErrorConversion` for your ctx type as well. This trait gives
/// /// you an opportunity to store or log your rich error type, while returning
/// /// a basic witx enum to the WebAssembly caller. It also gives you the ability
/// /// to terminate WebAssembly execution by trapping.
///
/// impl types::UserErrorConversion for YourCtxType {
///     fn errno_from_your_rich_error(&mut self, e: YourRichError)
///         -> Result<types::Errno, wiggle::wasmtime_crate::Error>
///     {
///         println!("Rich error: {:?}", e);
///         match e {
///             YourRichError::InvalidArg{..} => Ok(types::Errno::InvalidArg),
///             YourRichError::Io{..} => Ok(types::Errno::Io),
///             YourRichError::Overflow => Ok(types::Errno::Overflow),
///             YourRichError::Trap(s) => Err(wiggle::wasmtime_crate::Error::msg(s)),
///         }
///     }
/// }
///
/// # fn main() { println!("this fools doc tests into compiling the above outside a function body")
/// # }
/// ```
#[proc_macro]
pub fn from_witx(args: TokenStream) -> TokenStream {
    let config = parse_macro_input!(args as wiggle_generate::Config);

    let doc = config.load_document();

    let settings = wiggle_generate::CodegenSettings::new(
        &config.errors,
        &config.async_,
        &doc,
        config.wasmtime,
        &config.tracing,
        config.mutable,
    )
    .expect("validating codegen settings");

    let code = wiggle_generate::generate(&doc, &settings);
    let metadata = if cfg!(feature = "wiggle_metadata") {
        wiggle_generate::generate_metadata(&doc)
    } else {
        quote!()
    };

    let mut ret = quote! { #code #metadata };

    if std::env::var("WIGGLE_DEBUG_BINDGEN").is_ok() {
        use std::path::Path;
        use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static INVOCATION: AtomicUsize = AtomicUsize::new(0);
        let root = Path::new(env!("DEBUG_OUTPUT_DIR"));
        let n = INVOCATION.fetch_add(1, Relaxed);
        let path = root.join(format!("wiggle{n}.rs"));

        std::fs::write(&path, ret.to_string()).unwrap();

        // optimistically format the code but don't require success
        drop(
            std::process::Command::new("rustfmt")
                .arg(&path)
                .arg("--edition=2021")
                .output(),
        );

        let path = path.to_str().unwrap();
        ret = quote!(include!(#path););
    }
    TokenStream::from(ret)
}

#[proc_macro_attribute]
pub fn async_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as syn::parse::Nothing);
    let item = proc_macro2::TokenStream::from(item);
    TokenStream::from(quote! {
        #[wiggle::async_trait_crate::async_trait]
        #item
    })
}

/// Define the structs required to integrate a Wiggle implementation with Wasmtime.
///
/// ## Arguments
///
/// Arguments are provided using struct syntax e.g. `{ arg_name: value }`.
///
/// * `target`: The path of the module where the Wiggle implementation is defined.
#[proc_macro]
pub fn wasmtime_integration(args: TokenStream) -> TokenStream {
    let config = parse_macro_input!(args as wiggle_generate::WasmtimeConfig);
    let doc = config.c.load_document();

    let settings = wiggle_generate::CodegenSettings::new(
        &config.c.errors,
        &config.c.async_,
        &doc,
        true,
        &config.c.tracing,
        config.c.mutable,
    )
    .expect("validating codegen settings");

    let modules = doc.modules().map(|module| {
        wiggle_generate::wasmtime::link_module(&module, Some(&config.target), &settings)
    });
    quote!( #(#modules)* ).into()
}

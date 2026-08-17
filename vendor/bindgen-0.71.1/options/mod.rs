//! Declarations and setter methods for `bindgen` options.
//!
//! The main entry point of this module is the `options` macro.
#[macro_use]
mod helpers;
mod as_args;
#[cfg(feature = "__cli")]
pub(crate) mod cli;

use crate::callbacks::ParseCallbacks;
use crate::codegen::{
    AliasVariation, EnumVariation, MacroTypeVariation, NonCopyUnionStyle,
};
use crate::deps::DepfileSpec;
use crate::features::{RustEdition, RustFeatures, RustTarget};
use crate::regex_set::RegexSet;
use crate::Abi;
use crate::Builder;
use crate::CodegenConfig;
use crate::FieldVisibilityKind;
use crate::Formatter;
use crate::HashMap;
use crate::DEFAULT_ANON_FIELDS_PREFIX;

use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use as_args::AsArgs;
use helpers::ignore;

/// Macro used to generate the [`BindgenOptions`] type and the [`Builder`] setter methods for each
/// one of the fields of `BindgenOptions`.
///
/// The input format of this macro resembles a `struct` pattern. Each field of the `BindgenOptions`
/// type is declared by adding the name of the field and its type using the `name: type` syntax and
/// a block of code with the following items:
///
/// - `default`: The default value for the field. If this item is omitted, `Default::default()` is
///   used instead, meaning that the type of the field must implement `Default`.
/// - `methods`: A block of code containing methods for the `Builder` type. These methods should be
///   related to the field being declared.
/// - `as_args`: This item declares how the field should be converted into a valid CLI argument for
///   `bindgen` and is used in the [`Builder::command_line_flags`] method which is used to do a
///   roundtrip test of the CLI args in the `bindgen-test` crate. This item can take one of the
///   following:
///   - A string literal with the flag if the type of the field implements the [`AsArgs`] trait.
///   - A closure with the signature `|field, args: &mut Vec<String>| -> ()` that pushes arguments
///     into the `args` buffer based on the value of the field. This is used if the field does not
///     implement `AsArgs` or if the implementation of the trait is not logically correct for the
///     option and a custom behavior must be taken into account.
///   - The `ignore` literal, which does not emit any CLI arguments for this field. This is useful
///     if the field cannot be used from the `bindgen` CLI.
///
/// As an example, this would be the declaration of a `bool` field called `be_fun` whose default
/// value is `false` (the `Default` value for `bool`):
/// ```rust,ignore
/// be_fun: bool {
///    methods: {
///        /// Ask `bindgen` to be fun. This option is disabled by default.
///        fn be_fun(mut self) -> Self {
///            self.options.be_fun = true;
///            self
///        }
///    },
///    as_args: "--be-fun",
/// }
/// ```
///
/// However, we could also set the `be_fun` field to `true` by default and use a `--not-fun` flag
/// instead. This means that we have to add the `default` item and use a closure in the `as_args`
/// item:
/// ```rust,ignore
/// be_fun: bool {
///    default: true,
///    methods: {
///        /// Ask `bindgen` to not be fun. `bindgen` is fun by default.
///        fn not_fun(mut self) -> Self {
///            self.options.be_fun = false;
///            self
///        }
///    },
///    as_args: |be_fun, args| (!be_fun).as_args(args, "--not-fun"),
/// }
/// ```
/// More complex examples can be found in the sole invocation of this macro.
macro_rules! options {
    ($(
        $(#[doc = $docs:literal])+
        $field:ident: $ty:ty {
            $(default: $default:expr,)?
            methods: {$($methods_tokens:tt)*}$(,)?
            as_args: $as_args:expr$(,)?
        }$(,)?
    )*) => {
        #[derive(Debug, Clone)]
        pub(crate) struct BindgenOptions {
            $($(#[doc = $docs])* pub(crate) $field: $ty,)*
        }

        impl Default for BindgenOptions {
            fn default() -> Self {
                Self {
                    $($field: default!($($default)*),)*
                }
            }
        }

        impl Builder {
            /// Generates the command line flags used to create this [`Builder`].
            pub fn command_line_flags(&self) -> Vec<String> {
                let mut args = vec![];

                let headers = match self.options.input_headers.split_last() {
                    Some((header, headers)) => {
                        // The last input header is passed as an argument in the first position.
                        args.push(header.clone().into());
                        headers
                    },
                    None => &[]
                };

                $({
                    let func: fn(&$ty, &mut Vec<String>) = as_args!($as_args);
                    func(&self.options.$field, &mut args);
                })*

                // Add the `--experimental` flag if `bindgen` is built with the `experimental`
                // feature.
                if cfg!(feature = "experimental") {
                    args.push("--experimental".to_owned());
                }

                // Add all the clang arguments.
                args.push("--".to_owned());

                if !self.options.clang_args.is_empty() {
                    args.extend(self.options.clang_args.iter().map(|s| s.clone().into()));
                }

                // We need to pass all but the last header via the `-include` clang argument.
                for header in headers {
                    args.push("-include".to_owned());
                    args.push(header.clone().into());
                }

                args
            }

            $($($methods_tokens)*)*
        }
    };
}

options! {
    /// Types that have been blocklisted and should not appear anywhere in the generated code.
    blocklisted_types: RegexSet {
        methods: {
            regex_option! {
                /// Do not generate any bindings for the given type.
                ///
                /// This option is not recursive, meaning that it will only block types whose names
                /// explicitly match the argument of this method.
                pub fn blocklist_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_types.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-type",
    },
    /// Functions that have been blocklisted and should not appear in the generated code.
    blocklisted_functions: RegexSet {
        methods: {
            regex_option! {
                /// Do not generate any bindings for the given function.
                ///
                /// This option is not recursive, meaning that it will only block functions whose
                /// names explicitly match the argument of this method.
                pub fn blocklist_function<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_functions.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-function",
    },
    /// Items that have been blocklisted and should not appear in the generated code.
    blocklisted_items: RegexSet {
        methods: {
            regex_option! {
                /// Do not generate any bindings for the given item, regardless of whether it is a
                /// type, function, module, etc.
                ///
                /// This option is not recursive, meaning that it will only block items whose names
                /// explicitly match the argument of this method.
                pub fn blocklist_item<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_items.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-item",
    },
    /// Files whose contents should be blocklisted and should not appear in the generated code.
    blocklisted_files: RegexSet {
        methods: {
            regex_option! {
                /// Do not generate any bindings for the contents of the given file, regardless of
                /// whether the contents of the file are types, functions, modules, etc.
                ///
                /// This option is not recursive, meaning that it will only block files whose names
                /// explicitly match the argument of this method.
                ///
                /// This method will use the argument to match the complete path of the file
                /// instead of a section of it.
                pub fn blocklist_file<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_files.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-file",
    },
    /// Variables that have been blocklisted and should not appear in the generated code.
    blocklisted_vars: RegexSet {
        methods: {
            regex_option! {
                /// Do not generate any bindings for the given variable.
                ///
                /// This option is not recursive, meaning that it will only block variables whose
                /// names explicitly match the argument of this method.
                pub fn blocklist_var<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.blocklisted_vars.insert(arg);
                    self
                }
            }
        },
        as_args: "--blocklist-var",
    },
    /// Types that should be treated as opaque structures in the generated code.
    opaque_types: RegexSet {
        methods: {
            regex_option! {
                /// Treat the given type as opaque in the generated bindings.
                ///
                /// Opaque in this context means that none of the generated bindings will contain
                /// information about the inner representation of the type and the type itself will
                /// be represented as a chunk of bytes with the alignment and size of the type.
                pub fn opaque_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.opaque_types.insert(arg);
                    self
                }
            }
        },
        as_args: "--opaque-type",
    },
    /// The explicit `rustfmt` path.
    rustfmt_path: Option<PathBuf> {
        methods: {
            /// Set an explicit path to the `rustfmt` binary.
            ///
            /// This option only comes into effect if `rustfmt` is set to be the formatter used by
            /// `bindgen`. Check the documentation of the [`Builder::formatter`] method for more
            /// information.
            pub fn with_rustfmt<P: Into<PathBuf>>(mut self, path: P) -> Self {
                self.options.rustfmt_path = Some(path.into());
                self
            }
        },
        // This option cannot be set from the CLI.
        as_args: ignore,
    },
    /// The path to which we should write a Makefile-syntax depfile (if any).
    depfile: Option<DepfileSpec> {
        methods: {
            /// Add a depfile output which will be written alongside the generated bindings.
            pub fn depfile<H: Into<String>, D: Into<PathBuf>>(
                mut self,
                output_module: H,
                depfile: D,
            ) -> Builder {
                self.options.depfile = Some(DepfileSpec {
                    output_module: output_module.into(),
                    depfile_path: depfile.into(),
                });
                self
            }
        },
        as_args: |depfile, args| {
            if let Some(depfile) = depfile {
                args.push("--depfile".into());
                args.push(depfile.depfile_path.display().to_string());
            }
        },
    },
    /// Types that have been allowlisted and should appear in the generated code.
    allowlisted_types: RegexSet {
        methods: {
            regex_option! {
                /// Generate bindings for the given type.
                ///
                /// This option is transitive by default. Check the documentation of the
                /// [`Builder::allowlist_recursively`] method for further information.
                pub fn allowlist_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_types.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-type",
    },
    /// Functions that have been allowlisted and should appear in the generated code.
    allowlisted_functions: RegexSet {
        methods: {
            regex_option! {
                /// Generate bindings for the given function.
                ///
                /// This option is transitive by default. Check the documentation of the
                /// [`Builder::allowlist_recursively`] method for further information.
                pub fn allowlist_function<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_functions.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-function",
    },
    /// Variables that have been allowlisted and should appear in the generated code.
    allowlisted_vars: RegexSet {
        methods: {
            regex_option! {
                /// Generate bindings for the given variable.
                ///
                /// This option is transitive by default. Check the documentation of the
                /// [`Builder::allowlist_recursively`] method for further information.
                pub fn allowlist_var<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_vars.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-var",
    },
    /// Files whose contents have been allowlisted and should appear in the generated code.
    allowlisted_files: RegexSet {
        methods: {
            regex_option! {
                /// Generate bindings for the content of the given file.
                ///
                /// This option is transitive by default. Check the documentation of the
                /// [`Builder::allowlist_recursively`] method for further information.
                ///
                /// This method will use the argument to match the complete path of the file
                /// instead of a section of it.
                pub fn allowlist_file<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_files.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-file",
    },
    /// Items that have been allowlisted and should appear in the generated code.
    allowlisted_items: RegexSet {
        methods: {
            regex_option! {
                /// Generate bindings for the given item, regardless of whether it is a type,
                /// function, module, etc.
                ///
                /// This option is transitive by default. Check the documentation of the
                /// [`Builder::allowlist_recursively`] method for further information.
                pub fn allowlist_item<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.allowlisted_items.insert(arg);
                    self
                }
            }
        },
        as_args: "--allowlist-item",
    },
    /// The default style of for generated `enum`s.
    default_enum_style: EnumVariation {
        methods: {
            /// Set the default style for generated `enum`s.
            ///
            /// If this method is not called, the [`EnumVariation::Consts`] style will be used by
            /// default.
            ///
            /// To set the style for individual `enum`s, use [`Builder::bitfield_enum`],
            /// [`Builder::newtype_enum`], [`Builder::newtype_global_enum`],
            /// [`Builder::rustified_enum`], [`Builder::rustified_non_exhaustive_enum`],
            /// [`Builder::constified_enum_module`] or [`Builder::constified_enum`].
            pub fn default_enum_style(
                mut self,
                arg: EnumVariation,
            ) -> Builder {
                self.options.default_enum_style = arg;
                self
            }
        },
        as_args: |variation, args| {
            if *variation != Default::default() {
                args.push("--default-enum-style".to_owned());
                args.push(variation.to_string());
            }
        },
    },
    /// `enum`s marked as bitfield-like. This is, newtypes with bitwise operations.
    bitfield_enums: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `enum` as being bitfield-like.
                ///
                /// This is similar to the [`Builder::newtype_enum`] style, but with the bitwise
                /// operators implemented.
                pub fn bitfield_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.bitfield_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--bitfield-enum",
    },
    /// `enum`s marked as newtypes.
    newtype_enums: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `enum` as a newtype.
                ///
                /// This means that an integer newtype will be declared to represent the `enum`
                /// type and its variants will be represented as constants inside of this type's
                /// `impl` block.
                pub fn newtype_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.newtype_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--newtype-enum",
    },
    /// `enum`s marked as global newtypes .
    newtype_global_enums: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `enum` as a global newtype.
                ///
                /// This is similar to the [`Builder::newtype_enum`] style, but the constants for
                /// each variant are free constants instead of being declared inside an `impl`
                /// block for the newtype.
                pub fn newtype_global_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.newtype_global_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--newtype-global-enum",
    },
    /// `enum`s marked as Rust `enum`s.
    rustified_enums: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `enum` as a Rust `enum`.
                ///
                /// This means that each variant of the `enum` will be represented as a Rust `enum`
                /// variant.
                ///
                /// **Use this with caution**, creating an instance of a Rust `enum` with an
                /// invalid value will cause undefined behaviour. To avoid this, use the
                /// [`Builder::newtype_enum`] style instead.
                pub fn rustified_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.rustified_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--rustified-enum",
    },
    /// `enum`s marked as non-exhaustive Rust `enum`s.
    rustified_non_exhaustive_enums: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `enum` as a non-exhaustive Rust `enum`.
                ///
                /// This is similar to the [`Builder::rustified_enum`] style, but the `enum` is
                /// tagged with the `#[non_exhaustive]` attribute.
                pub fn rustified_non_exhaustive_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.rustified_non_exhaustive_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--rustified-non-exhaustive-enums",
    },
    /// `enum`s marked as modules of constants.
    constified_enum_modules: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `enum` as a module with a set of integer constants.
                pub fn constified_enum_module<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.constified_enum_modules.insert(arg);
                    self
                }
            }
        },
        as_args: "--constified-enum-module",
    },
    /// `enum`s marked as a set of constants.
    constified_enums: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `enum` as a set of integer constants.
                ///
                /// This is similar to the [`Builder::constified_enum_module`] style, but the
                /// constants are generated in the current module instead of in a new module.
                pub fn constified_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.constified_enums.insert(arg);
                    self
                }
            }
        },
        as_args: "--constified-enum",
    },
    /// The default type signedness for C macro constants.
    default_macro_constant_type: MacroTypeVariation {
        methods: {
            /// Set the default type signedness to be used for macro constants.
            ///
            /// If this method is not called, [`MacroTypeVariation::Unsigned`] is used by default.
            ///
            /// To set the type for individual macro constants, use the
            /// [`ParseCallbacks::int_macro`] method.
            pub fn default_macro_constant_type(mut self, arg: MacroTypeVariation) -> Builder {
                self.options.default_macro_constant_type = arg;
                self
            }

        },
        as_args: |variation, args| {
            if *variation != Default::default() {
                args.push("--default-macro-constant-type".to_owned());
                args.push(variation.to_string());
            }
        },
    },
    /// The default style of code generation for `typedef`s.
    default_alias_style: AliasVariation {
        methods: {
            /// Set the default style of code generation for `typedef`s.
            ///
            /// If this method is not called, the [`AliasVariation::TypeAlias`] style is used by
            /// default.
            ///
            /// To set the style for individual `typedefs`s, use [`Builder::type_alias`],
            /// [`Builder::new_type_alias`] or [`Builder::new_type_alias_deref`].
            pub fn default_alias_style(
                mut self,
                arg: AliasVariation,
            ) -> Builder {
                self.options.default_alias_style = arg;
                self
            }
        },
        as_args: |variation, args| {
            if *variation != Default::default() {
                args.push("--default-alias-style".to_owned());
                args.push(variation.to_string());
            }
        },
    },
    /// `typedef` patterns that will use regular type aliasing.
    type_alias: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `typedef` as a regular Rust `type` alias.
                ///
                /// This is the default behavior, meaning that this method only comes into effect
                /// if a style different from [`AliasVariation::TypeAlias`] was passed to the
                /// [`Builder::default_alias_style`] method.
                pub fn type_alias<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.type_alias.insert(arg);
                    self
                }
            }
        },
        as_args: "--type-alias",
    },
    /// `typedef` patterns that will be aliased by creating a newtype.
    new_type_alias: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `typedef` as a Rust newtype by having the aliased
                /// type be wrapped in a `struct` with `#[repr(transparent)]`.
                ///
                /// This method can be used to enforce stricter type checking.
                pub fn new_type_alias<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.new_type_alias.insert(arg);
                    self
                }
            }
        },
        as_args: "--new-type-alias",
    },
    /// `typedef` patterns that will be wrapped in a newtype implementing `Deref` and `DerefMut`.
    new_type_alias_deref: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `typedef` to be generated as a newtype that can be dereferenced.
                ///
                /// This is similar to the [`Builder::new_type_alias`] style, but the newtype
                /// implements `Deref` and `DerefMut` with the aliased type as a target.
                pub fn new_type_alias_deref<T: AsRef<str>>(mut self, arg: T) -> Builder {
                    self.options.new_type_alias_deref.insert(arg);
                    self
                }
            }
        },
        as_args: "--new-type-alias-deref",
    },
    /// The default style of code to generate for `union`s containing non-`Copy` members.
    default_non_copy_union_style: NonCopyUnionStyle {
        methods: {
            /// Set the default style of code to generate for `union`s with non-`Copy` members.
            ///
            /// If this method is not called, the [`NonCopyUnionStyle::BindgenWrapper`] style is
            /// used by default.
            ///
            /// To set the style for individual `union`s, use [`Builder::bindgen_wrapper_union`] or
            /// [`Builder::manually_drop_union`].
            pub fn default_non_copy_union_style(mut self, arg: NonCopyUnionStyle) -> Self {
                self.options.default_non_copy_union_style = arg;
                self
            }
        },
        as_args: |style, args| {
            if *style != Default::default() {
                args.push("--default-non-copy-union-style".to_owned());
                args.push(style.to_string());
            }
        },
    },
    /// The patterns marking non-`Copy` `union`s as using the `bindgen` generated wrapper.
    bindgen_wrapper_union: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `union` to use a `bindgen`-generated wrapper for its members if at
                /// least one them is not `Copy`.
                ///
                /// This is the default behavior, meaning that this method only comes into effect
                /// if a style different from [`NonCopyUnionStyle::BindgenWrapper`] was passed to
                /// the [`Builder::default_non_copy_union_style`] method.
                pub fn bindgen_wrapper_union<T: AsRef<str>>(mut self, arg: T) -> Self {
                    self.options.bindgen_wrapper_union.insert(arg);
                    self
                }
            }
        },
        as_args: "--bindgen-wrapper-union",
    },
    /// The patterns marking non-`Copy` `union`s as using the `ManuallyDrop` wrapper.
    manually_drop_union: RegexSet {
        methods: {
            regex_option! {
                /// Mark the given `union` to use [`::core::mem::ManuallyDrop`] for its members if
                /// at least one of them is not `Copy`.
                ///
                /// The `ManuallyDrop` type was stabilized in Rust 1.20.0, do not use this option
                /// if your target version is lower than this.
                pub fn manually_drop_union<T: AsRef<str>>(mut self, arg: T) -> Self {
                    self.options.manually_drop_union.insert(arg);
                    self
                }
            }

        },
        as_args: "--manually-drop-union",
    },


    /// Whether we should generate built-in definitions.
    builtins: bool {
        methods: {
            /// Generate Rust bindings for built-in definitions (for example `__builtin_va_list`).
            ///
            /// Bindings for built-in definitions are not emitted by default.
            pub fn emit_builtins(mut self) -> Builder {
                self.options.builtins = true;
                self
            }
        },
        as_args: "--builtins",
    },
    /// Whether we should dump the Clang AST for debugging purposes.
    emit_ast: bool {
        methods: {
            /// Emit the Clang AST to `stdout` for debugging purposes.
            ///
            /// The Clang AST is not emitted by default.
            pub fn emit_clang_ast(mut self) -> Builder {
                self.options.emit_ast = true;
                self
            }
        },
        as_args: "--emit-clang-ast",
    },
    /// Whether we should dump our IR for debugging purposes.
    emit_ir: bool {
        methods: {
            /// Emit the `bindgen` internal representation to `stdout` for debugging purposes.
            ///
            /// This internal representation is not emitted by default.
            pub fn emit_ir(mut self) -> Builder {
                self.options.emit_ir = true;
                self
            }
        },
        as_args: "--emit-ir",
    },
    /// Output path for the `graphviz` DOT file.
    emit_ir_graphviz: Option<String> {
        methods: {
            /// Set the path for the file where the`bindgen` internal representation will be
            /// emitted as a graph using the `graphviz` DOT language.
            ///
            /// This graph representation is not emitted by default.
            pub fn emit_ir_graphviz<T: Into<String>>(mut self, path: T) -> Builder {
                let path = path.into();
                self.options.emit_ir_graphviz = Some(path);
                self
            }
        },
        as_args: "--emit-ir-graphviz",
    },

    /// Whether we should emulate C++ namespaces with Rust modules.
    enable_cxx_namespaces: bool {
        methods: {
            /// Emulate C++ namespaces using Rust modules in the generated bindings.
            ///
            /// C++ namespaces are not emulated by default.
            pub fn enable_cxx_namespaces(mut self) -> Builder {
                self.options.enable_cxx_namespaces = true;
                self
            }
        },
        as_args: "--enable-cxx-namespaces",
    },
    /// Whether we should try to find unexposed attributes in functions.
    enable_function_attribute_detection: bool {
        methods: {
            /// Enable detecting function attributes on C functions.
            ///
            /// This enables the following features:
            /// - Add `#[must_use]` attributes to Rust items whose C counterparts are marked as so.
            /// This feature also requires that the Rust target version supports the attribute.
            /// - Set `!` as the return type for Rust functions whose C counterparts are marked as
            /// diverging.
            ///
            /// This option can be quite slow in some cases (check [#1465]), so it is disabled by
            /// default.
            ///
            /// [#1465]: https://github.com/rust-lang/rust-bindgen/issues/1465
            pub fn enable_function_attribute_detection(mut self) -> Self {
                self.options.enable_function_attribute_detection = true;
                self
            }

        },
        as_args: "--enable-function-attribute-detection",
    },
    /// Whether we should avoid mangling names with namespaces.
    disable_name_namespacing: bool {
        methods: {
            /// Disable name auto-namespacing.
            ///
            /// By default, `bindgen` mangles names like `foo::bar::Baz` to look like `foo_bar_Baz`
            /// instead of just `Baz`. This method disables that behavior.
            ///
            /// Note that this does not change the names used for allowlisting and blocklisting,
            /// which should still be mangled with the namespaces. Additionally, this option may
            /// cause `bindgen` to generate duplicate names.
            pub fn disable_name_namespacing(mut self) -> Builder {
                self.options.disable_name_namespacing = true;
                self
            }
        },
        as_args: "--disable-name-namespacing",
    },
    /// Whether we should avoid generating nested `struct` names.
    disable_nested_struct_naming: bool {
        methods: {
            /// Disable nested `struct` naming.
            ///
            /// The following `struct`s have different names for C and C++. In C, they are visible
            /// as `foo` and `bar`. In C++, they are visible as `foo` and `foo::bar`.
            ///
            /// ```c
            /// struct foo {
            ///     struct bar {
            ///     } b;
            /// };
            /// ```
            ///
            /// `bindgen` tries to avoid duplicate names by default, so it follows the C++ naming
            /// convention and it generates `foo` and `foo_bar` instead of just `foo` and `bar`.
            ///
            /// This method disables this behavior and it is indented to be used only for headers
            /// that were written in C.
            pub fn disable_nested_struct_naming(mut self) -> Builder {
                self.options.disable_nested_struct_naming = true;
                self
            }
        },
        as_args: "--disable-nested-struct-naming",
    },
    /// Whether we should avoid embedding version identifiers into source code.
    disable_header_comment: bool {
        methods: {
            /// Do not insert the `bindgen` version identifier into the generated bindings.
            ///
            /// This identifier is inserted by default.
            pub fn disable_header_comment(mut self) -> Self {
                self.options.disable_header_comment = true;
                self
            }

        },
        as_args: "--disable-header-comment",
    },
    /// Whether we should generate layout tests for generated `struct`s.
    layout_tests: bool {
        default: true,
        methods: {
            /// Set whether layout tests should be generated.
            ///
            /// Layout tests are generated by default.
            pub fn layout_tests(mut self, doit: bool) -> Self {
                self.options.layout_tests = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-layout-tests"),
    },
    /// Whether we should implement `Debug` for types that cannot derive it.
    impl_debug: bool {
        methods: {
            /// Set whether `Debug` should be implemented for types that cannot derive it.
            ///
            /// This option is disabled by default.
            pub fn impl_debug(mut self, doit: bool) -> Self {
                self.options.impl_debug = doit;
                self
            }

        },
        as_args: "--impl-debug",
    },
    /// Whether we should implement `PartialEq` types that cannot derive it.
    impl_partialeq: bool {
        methods: {
            /// Set whether `PartialEq` should be implemented for types that cannot derive it.
            ///
            /// This option is disabled by default.
            pub fn impl_partialeq(mut self, doit: bool) -> Self {
                self.options.impl_partialeq = doit;
                self
            }
        },
        as_args: "--impl-partialeq",
    },
    /// Whether we should derive `Copy` when possible.
    derive_copy: bool {
        default: true,
        methods: {
            /// Set whether the `Copy` trait should be derived when possible.
            ///
            /// `Copy` is derived by default.
            pub fn derive_copy(mut self, doit: bool) -> Self {
                self.options.derive_copy = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-derive-copy"),
    },

    /// Whether we should derive `Debug` when possible.
    derive_debug: bool {
        default: true,
        methods: {
            /// Set whether the `Debug` trait should be derived when possible.
            ///
            /// The [`Builder::impl_debug`] method can be used to implement `Debug` for types that
            /// cannot derive it.
            ///
            /// `Debug` is derived by default.
            pub fn derive_debug(mut self, doit: bool) -> Self {
                self.options.derive_debug = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-derive-debug"),
    },

    /// Whether we should derive `Default` when possible.
    derive_default: bool {
        methods: {
            /// Set whether the `Default` trait should be derived when possible.
            ///
            /// `Default` is not derived by default.
            pub fn derive_default(mut self, doit: bool) -> Self {
                self.options.derive_default = doit;
                self
            }
        },
        as_args: |&value, args| {
            let arg = if value {
                "--with-derive-default"
            } else {
                "--no-derive-default"
            };

            args.push(arg.to_owned());
        },
    },
    /// Whether we should derive `Hash` when possible.
    derive_hash: bool {
        methods: {
            /// Set whether the `Hash` trait should be derived when possible.
            ///
            /// `Hash` is not derived by default.
            pub fn derive_hash(mut self, doit: bool) -> Self {
                self.options.derive_hash = doit;
                self
            }
        },
        as_args: "--with-derive-hash",
    },
    /// Whether we should derive `PartialOrd` when possible.
    derive_partialord: bool {
        methods: {
            /// Set whether the `PartialOrd` trait should be derived when possible.
            ///
            /// Take into account that `Ord` cannot be derived for a type that does not implement
            /// `PartialOrd`. For this reason, setting this method to `false` also sets
            /// automatically [`Builder::derive_ord`] to `false`.
            ///
            /// `PartialOrd` is not derived by default.
            pub fn derive_partialord(mut self, doit: bool) -> Self {
                self.options.derive_partialord = doit;
                if !doit {
                    self.options.derive_ord = false;
                }
                self
            }
        },
        as_args: "--with-derive-partialord",
    },
    /// Whether we should derive `Ord` when possible.
    derive_ord: bool {
        methods: {
            /// Set whether the `Ord` trait should be derived when possible.
            ///
            /// Take into account that `Ord` cannot be derived for a type that does not implement
            /// `PartialOrd`. For this reason, the value set with this method will also be set
            /// automatically for [`Builder::derive_partialord`].
            ///
            /// `Ord` is not derived by default.
            pub fn derive_ord(mut self, doit: bool) -> Self {
                self.options.derive_ord = doit;
                self.options.derive_partialord = doit;
                self
            }
        },
        as_args: "--with-derive-ord",
    },
    /// Whether we should derive `PartialEq` when possible.
    derive_partialeq: bool {
        methods: {
            /// Set whether the `PartialEq` trait should be derived when possible.
            ///
            /// Take into account that `Eq` cannot be derived for a type that does not implement
            /// `PartialEq`. For this reason, setting this method to `false` also sets
            /// automatically [`Builder::derive_eq`] to `false`.
            ///
            /// The [`Builder::impl_partialeq`] method can be used to implement `PartialEq` for
            /// types that cannot derive it.
            ///
            /// `PartialEq` is not derived by default.
            pub fn derive_partialeq(mut self, doit: bool) -> Self {
                self.options.derive_partialeq = doit;
                if !doit {
                    self.options.derive_eq = false;
                }
                self
            }
        },
        as_args: "--with-derive-partialeq",
    },
    /// Whether we should derive `Eq` when possible.
    derive_eq: bool {
        methods: {
            /// Set whether the `Eq` trait should be derived when possible.
            ///
            /// Take into account that `Eq` cannot be derived for a type that does not implement
            /// `PartialEq`. For this reason, the value set with this method will also be set
            /// automatically for [`Builder::derive_partialeq`].
            ///
            /// `Eq` is not derived by default.
            pub fn derive_eq(mut self, doit: bool) -> Self {
                self.options.derive_eq = doit;
                if doit {
                    self.options.derive_partialeq = doit;
                }
                self
            }
        },
        as_args: "--with-derive-eq",
    },
    /// Whether we should use `core` instead of `std`.
    ///
    /// If this option is enabled and the Rust target version is greater than 1.64, the prefix for
    /// C platform-specific types will be `::core::ffi` instead of `::core::os::raw`.
    use_core: bool {
        methods: {
            /// Use `core` instead of `std` in the generated bindings.
            ///
            /// `std` is used by default.
            pub fn use_core(mut self) -> Builder {
                self.options.use_core = true;
                self
            }

        },
        as_args: "--use-core",
    },
    /// An optional prefix for the C platform-specific types.
    ctypes_prefix: Option<String> {
        methods: {
            /// Use the given prefix for the C platform-specific types instead of `::std::os::raw`.
            ///
            /// Alternatively, the [`Builder::use_core`] method can be used to set the prefix to
            /// `::core::ffi` or `::core::os::raw`.
            pub fn ctypes_prefix<T: Into<String>>(mut self, prefix: T) -> Builder {
                self.options.ctypes_prefix = Some(prefix.into());
                self
            }
        },
        as_args: "--ctypes-prefix",
    },
    /// The prefix for anonymous fields.
    anon_fields_prefix: String {
        default: DEFAULT_ANON_FIELDS_PREFIX.into(),
        methods: {
            /// Use the given prefix for the anonymous fields.
            ///
            /// An anonymous field, is a field of a C/C++ type that does not have a name. For
            /// example, in the following C code:
            /// ```c
            /// struct integer {
            ///   struct {
            ///     int inner;
            ///   };
            /// }
            /// ```
            ///
            /// The only field of the `integer` `struct` is an anonymous field and its Rust
            /// representation will be named using this prefix followed by an integer identifier.
            ///
            /// The default prefix is `__bindgen_anon_`.
            pub fn anon_fields_prefix<T: Into<String>>(mut self, prefix: T) -> Builder {
                self.options.anon_fields_prefix = prefix.into();
                self
            }
        },
        as_args: |prefix, args| {
            if prefix != DEFAULT_ANON_FIELDS_PREFIX {
                args.push("--anon-fields-prefix".to_owned());
                args.push(prefix.clone());
            }
        },
    },
    /// Whether to measure the time for each one of the `bindgen` phases.
    time_phases: bool {
        methods: {
            /// Set whether to measure the elapsed time for each one of the `bindgen` phases. This
            /// information is printed to `stderr`.
            ///
            /// The elapsed time is not measured by default.
            pub fn time_phases(mut self, doit: bool) -> Self {
                self.options.time_phases = doit;
                self
            }
        },
        as_args: "--time-phases",
    },
    /// Whether to convert C float types to `f32` and `f64`.
    convert_floats: bool {
        default: true,
        methods: {
            /// Avoid converting C float types to `f32` and `f64`.
            pub fn no_convert_floats(mut self) -> Self {
                self.options.convert_floats = false;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-convert-floats"),
    },
    /// The set of raw lines to be prepended to the top-level module of the generated Rust code.
    raw_lines: Vec<Box<str>> {
        methods: {
            /// Add a line of Rust code at the beginning of the generated bindings. The string is
            /// passed through without any modification.
            pub fn raw_line<T: Into<String>>(mut self, arg: T) -> Self {
                self.options.raw_lines.push(arg.into().into_boxed_str());
                self
            }
        },
        as_args: |raw_lines, args| {
            for line in raw_lines {
                args.push("--raw-line".to_owned());
                args.push(line.clone().into());
            }
        },
    },
    /// The set of raw lines to prepend to different modules.
    module_lines: HashMap<Box<str>, Vec<Box<str>>> {
        methods: {
            /// Add a given line to the beginning of a given module.
            ///
            /// This option only comes into effect if the [`Builder::enable_cxx_namespaces`] method
            /// is also being called.
            pub fn module_raw_line<T, U>(mut self, module: T, line: U) -> Self
            where
                T: Into<String>,
                U: Into<String>,
            {
                self.options
                    .module_lines
                    .entry(module.into().into_boxed_str())
                    .or_default()
                    .push(line.into().into_boxed_str());
                self
            }
        },
        as_args: |module_lines, args| {
            for (module, lines) in module_lines {
                for line in lines {
                    args.push("--module-raw-line".to_owned());
                    args.push(module.clone().into());
                    args.push(line.clone().into());
                }
            }
        },
    },
    /// The input header files.
    input_headers:  Vec<Box<str>> {
        methods: {
            /// Add an input C/C++ header to generate bindings for.
            ///
            /// This can be used to generate bindings for a single header:
            ///
            /// ```ignore
            /// let bindings = bindgen::Builder::default()
            ///     .header("input.h")
            ///     .generate()
            ///     .unwrap();
            /// ```
            ///
            /// Or for multiple headers:
            ///
            /// ```ignore
            /// let bindings = bindgen::Builder::default()
            ///     .header("first.h")
            ///     .header("second.h")
            ///     .header("third.h")
            ///     .generate()
            ///     .unwrap();
            /// ```
            pub fn header<T: Into<String>>(mut self, header: T) -> Builder {
                self.options.input_headers.push(header.into().into_boxed_str());
                self
            }

            /// Add input C/C++ header(s) to generate bindings for.
            ///
            /// This can be used to generate bindings for a single header:
            ///
            /// ```ignore
            /// let bindings = bindgen::Builder::default()
            ///     .headers(["input.h"])
            ///     .generate()
            ///     .unwrap();
            /// ```
            ///
            /// Or for multiple headers:
            ///
            /// ```ignore
            /// let bindings = bindgen::Builder::default()
            ///     .headers(["first.h", "second.h", "third.h"])
            ///     .generate()
            ///     .unwrap();
            /// ```
            pub fn headers<I: IntoIterator>(mut self, headers: I) -> Builder
            where
                I::Item: Into<String>,
            {
                self.options
                    .input_headers
                    .extend(headers.into_iter().map(Into::into).map(Into::into));
                self
            }
        },
        // This field is handled specially inside the macro.
        as_args: ignore,
    },
    /// The set of arguments to be passed straight through to Clang.
    clang_args: Vec<Box<str>> {
        methods: {
            /// Add an argument to be passed straight through to Clang.
            pub fn clang_arg<T: Into<String>>(self, arg: T) -> Builder {
                self.clang_args([arg.into().into_boxed_str()])
            }

            /// Add several arguments to be passed straight through to Clang.
            pub fn clang_args<I: IntoIterator>(mut self, args: I) -> Builder
            where
                I::Item: AsRef<str>,
            {
                for arg in args {
                    self.options.clang_args.push(arg.as_ref().to_owned().into_boxed_str());
                }
                self
            }
        },
        // This field is handled specially inside the macro.
        as_args: ignore,
    },
    /// Tuples of unsaved file contents of the form (name, contents).
    input_header_contents: Vec<(Box<str>, Box<str>)> {
        methods: {
            /// Add `contents` as an input C/C++ header named `name`.
            ///
            /// This can be used to inject additional C/C++ code as an input without having to
            /// create additional header files.
            pub fn header_contents(mut self, name: &str, contents: &str) -> Builder {
                // Apparently clang relies on having virtual FS correspondent to
                // the real one, so we need absolute paths here
                let absolute_path = env::current_dir()
                    .expect("Cannot retrieve current directory")
                    .join(name)
                    .to_str()
                    .expect("Cannot convert current directory name to string")
                    .into();
                self.options
                    .input_header_contents
                    .push((absolute_path, contents.into()));
                self
            }
        },
        // Header contents cannot be added from the CLI.
        as_args: ignore,
    },
    /// A user-provided visitor to allow customizing different kinds of situations.
    parse_callbacks: Vec<Rc<dyn ParseCallbacks>> {
        methods: {
            /// Add a new [`ParseCallbacks`] instance to configure types in different situations.
            ///
            /// This can also be used with [`CargoCallbacks`](struct@crate::CargoCallbacks) to emit
            /// `cargo:rerun-if-changed=...` for all `#include`d header files.
            pub fn parse_callbacks(mut self, cb: Box<dyn ParseCallbacks>) -> Self {
                self.options.parse_callbacks.push(Rc::from(cb));
                self
            }
        },
        as_args: |_callbacks, _args| {
            #[cfg(feature = "__cli")]
            for cb in _callbacks {
                _args.extend(cb.cli_args());
            }
        },
    },
    /// Which kind of items should we generate. We generate all of them by default.
    codegen_config: CodegenConfig {
        default: CodegenConfig::all(),
        methods: {
            /// Do not generate any functions.
            ///
            /// Functions are generated by default.
            pub fn ignore_functions(mut self) -> Builder {
                self.options.codegen_config.remove(CodegenConfig::FUNCTIONS);
                self
            }

            /// Do not generate any methods.
            ///
            /// Methods are generated by default.
            pub fn ignore_methods(mut self) -> Builder {
                self.options.codegen_config.remove(CodegenConfig::METHODS);
                self
            }

            /// Choose what to generate using a [`CodegenConfig`].
            ///
            /// This option overlaps with [`Builder::ignore_functions`] and
            /// [`Builder::ignore_methods`].
            ///
            /// All the items in `CodegenConfig` are generated by default.
            pub fn with_codegen_config(mut self, config: CodegenConfig) -> Self {
                self.options.codegen_config = config;
                self
            }
        },
        as_args: |codegen_config, args| {
            if !codegen_config.functions() {
                args.push("--ignore-functions".to_owned());
            }

            args.push("--generate".to_owned());

            //Temporary placeholder for the 4 options below.
            let mut options: Vec<String> = Vec::new();
            if codegen_config.functions() {
                options.push("functions".to_owned());
            }

            if codegen_config.types() {
                options.push("types".to_owned());
            }

            if codegen_config.vars() {
                options.push("vars".to_owned());
            }

            if codegen_config.methods() {
                options.push("methods".to_owned());
            }

            if codegen_config.constructors() {
                options.push("constructors".to_owned());
            }

            if codegen_config.destructors() {
                options.push("destructors".to_owned());
            }

            args.push(options.join(","));

            if !codegen_config.methods() {
                args.push("--ignore-methods".to_owned());
            }
        },
    },
    /// Whether to treat inline namespaces conservatively.
    conservative_inline_namespaces: bool {
        methods: {
            /// Treat inline namespaces conservatively.
            ///
            /// This is tricky, because in C++ is technically legal to override an item
            /// defined in an inline namespace:
            ///
            /// ```cpp
            /// inline namespace foo {
            ///     using Bar = int;
            /// }
            /// using Bar = long;
            /// ```
            ///
            /// Even though referencing `Bar` is a compiler error.
            ///
            /// We want to support this (arguably esoteric) use case, but we do not want to make
            /// the rest of `bindgen` users pay an usability penalty for that.
            ///
            /// To support this, we need to keep all the inline namespaces around, but then using
            /// `bindgen` becomes a bit more difficult, because you cannot reference paths like
            /// `std::string` (you'd need to use the proper inline namespace).
            ///
            /// We could complicate a lot of the logic to detect name collisions and, in the
            /// absence of collisions, generate a `pub use inline_ns::*` or something like that.
            ///
            /// That is probably something we can do to improve the usability of this option if we
            /// realize it is needed way more often. Our guess is that this extra logic is not
            /// going to be very useful.
            ///
            /// This option is disabled by default.
            pub fn conservative_inline_namespaces(mut self) -> Builder {
                self.options.conservative_inline_namespaces = true;
                self
            }
        },
        as_args: "--conservative-inline-namespaces",
    },
    /// Whether to keep documentation comments in the generated output.
    generate_comments: bool {
        default: true,
        methods: {
            /// Set whether the generated bindings should contain documentation comments.
            ///
            /// Documentation comments are included by default.
            ///
            /// Note that clang excludes comments from system headers by default, pass
            /// `"-fretain-comments-from-system-headers"` to the [`Builder::clang_arg`] method to
            /// include them.
            ///
            /// It is also possible to process all comments and not just documentation using the
            /// `"-fparse-all-comments"` flag. Check [these slides on clang comment parsing](
            /// https://llvm.org/devmtg/2012-11/Gribenko_CommentParsing.pdf) for more information
            /// and examples.
            pub fn generate_comments(mut self, doit: bool) -> Self {
                self.options.generate_comments = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-doc-comments"),
    },
    /// Whether to generate inline functions.
    generate_inline_functions: bool {
        methods: {
            /// Set whether to generate inline functions.
            ///
            /// This option is disabled by default.
            ///
            /// Note that they will usually not work. However you can use `-fkeep-inline-functions`
            /// or `-fno-inline-functions` if you are responsible of compiling the library to make
            /// them callable.
            ///
            /// Check the [`Builder::wrap_static_fns`] method for an alternative.
            pub fn generate_inline_functions(mut self, doit: bool) -> Self {
                self.options.generate_inline_functions = doit;
                self
            }
        },
        as_args: "--generate-inline-functions",
    },
    /// Whether to allowlist types recursively.
    allowlist_recursively: bool {
        default: true,
        methods: {
            /// Set whether to recursively allowlist items.
            ///
            /// Items are allowlisted recursively by default.
            ///
            /// Given that we have explicitly allowlisted the `initiate_dance_party` function in
            /// this C header:
            ///
            /// ```c
            /// typedef struct MoonBoots {
            ///     int bouncy_level;
            /// } MoonBoots;
            ///
            /// void initiate_dance_party(MoonBoots* boots);
            /// ```
            ///
            /// We would normally generate bindings to both the `initiate_dance_party` function and
            /// the `MoonBoots` type that it transitively references. If `false` is passed to this
            /// method, `bindgen` will not emit bindings for anything except the explicitly
            /// allowlisted items, meaning that the definition for `MoonBoots` would not be
            /// generated. However, the `initiate_dance_party` function would still reference
            /// `MoonBoots`!
            ///
            /// **Disabling this feature will almost certainly cause `bindgen` to emit bindings
            /// that will not compile!** If you disable this feature, then it is *your*
            /// responsibility to provide definitions for every type that is referenced from an
            /// explicitly allowlisted item. One way to provide the missing definitions is by using
            /// the [`Builder::raw_line`] method, another would be to define them in Rust and then
            /// `include!(...)` the bindings immediately afterwards.
            pub fn allowlist_recursively(mut self, doit: bool) -> Self {
                self.options.allowlist_recursively = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-recursive-allowlist"),
    },
    /// Whether to emit `#[macro_use] extern crate objc;` instead of `use objc;` in the prologue of
    /// the files generated from objective-c files.
    objc_extern_crate: bool {
        methods: {
            /// Emit `#[macro_use] extern crate objc;` instead of `use objc;` in the prologue of
            /// the files generated from objective-c files.
            ///
            /// `use objc;` is emitted by default.
            pub fn objc_extern_crate(mut self, doit: bool) -> Self {
                self.options.objc_extern_crate = doit;
                self
            }
        },
        as_args: "--objc-extern-crate",
    },
    /// Whether to generate proper block signatures instead of `void` pointers.
    generate_block: bool {
        methods: {
            /// Generate proper block signatures instead of `void` pointers.
            ///
            /// `void` pointers are used by default.
            pub fn generate_block(mut self, doit: bool) -> Self {
                self.options.generate_block = doit;
                self
            }
        },
        as_args: "--generate-block",
    },
    /// Whether to generate strings as `CStr`.
    generate_cstr: bool {
        methods: {
            /// Set whether string constants should be generated as `&CStr` instead of `&[u8]`.
            ///
            /// A minimum Rust target of 1.59 is required for this to have any effect as support
            /// for `CStr::from_bytes_with_nul_unchecked` in `const` contexts is needed.
            ///
            /// This option is disabled by default but will become enabled by default in a future
            /// release, so enabling this is recommended.
            pub fn generate_cstr(mut self, doit: bool) -> Self {
                self.options.generate_cstr = doit;
                self
            }
        },
        as_args: "--generate-cstr",
    },
    /// Whether to emit `#[macro_use] extern crate block;` instead of `use block;` in the prologue
    /// of the files generated from apple block files.
    block_extern_crate: bool {
        methods: {
            /// Emit `#[macro_use] extern crate block;` instead of `use block;` in the prologue of
            /// the files generated from apple block files.
            ///
            /// `use block;` is emitted by default.
            pub fn block_extern_crate(mut self, doit: bool) -> Self {
                self.options.block_extern_crate = doit;
                self
            }
        },
        as_args: "--block-extern-crate",
    },
    /// Whether to use the clang-provided name mangling.
    enable_mangling: bool {
        default: true,
        methods: {
            /// Set whether to use the clang-provided name mangling. This is probably needed for
            /// C++ features.
            ///
            /// The mangling provided by clang is used by default.
            ///
            /// We allow disabling this option because some old `libclang` versions seem to return
            /// incorrect results in some cases for non-mangled functions, check [#528] for more
            /// information.
            ///
            /// [#528]: https://github.com/rust-lang/rust-bindgen/issues/528
            pub fn trust_clang_mangling(mut self, doit: bool) -> Self {
                self.options.enable_mangling = doit;
                self
            }

        },
        as_args: |value, args| (!value).as_args(args, "--distrust-clang-mangling"),
    },
    /// Whether to detect include paths using `clang_sys`.
    detect_include_paths: bool {
        default: true,
        methods: {
            /// Set whether to detect include paths using `clang_sys`.
            ///
            /// `clang_sys` is used to detect include paths by default.
            pub fn detect_include_paths(mut self, doit: bool) -> Self {
                self.options.detect_include_paths = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-include-path-detection"),
    },
    /// Whether we should try to fit macro constants into types smaller than `u32` and `i32`.
    fit_macro_constants: bool {
        methods: {
            /// Set whether `bindgen` should try to fit macro constants into types smaller than `u32`
            /// and `i32`.
            ///
            /// This option is disabled by default.
            pub fn fit_macro_constants(mut self, doit: bool) -> Self {
                self.options.fit_macro_constants = doit;
                self
            }
        },
        as_args: "--fit-macro-constant-types",
    },
    /// Whether to prepend the `enum` name to constant or newtype variants.
    prepend_enum_name: bool {
        default: true,
        methods: {
            /// Set whether to prepend the `enum` name to constant or newtype variants.
            ///
            /// The `enum` name is prepended by default.
            pub fn prepend_enum_name(mut self, doit: bool) -> Self {
                self.options.prepend_enum_name = doit;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-prepend-enum-name"),
    },
    /// Version of the Rust compiler to target.
    rust_target: RustTarget {
        methods: {
            /// Specify the Rust target version.
            ///
            /// The default target is the latest stable Rust version.
            pub fn rust_target(mut self, rust_target: RustTarget) -> Self {
                self.options.set_rust_target(rust_target);
                self
            }
        },
        as_args: |rust_target, args| {
            args.push("--rust-target".to_owned());
            args.push(rust_target.to_string());
        },
    },
    /// The Rust edition to use for code generation.
    rust_edition: Option<RustEdition> {
        methods: {
            /// Specify the Rust target edition.
            ///
            /// The default edition is the latest edition supported by the chosen Rust target.
            pub fn rust_edition(mut self, rust_edition: RustEdition) -> Self {
                self.options.rust_edition = Some(rust_edition);
                self
            }
        }
        as_args: |edition, args| {
            if let Some(edition) =  edition {
                args.push("--rust-edition".to_owned());
                args.push(edition.to_string());
            }
        },
    },
    /// Features to be enabled. They are derived from `rust_target`.
    rust_features: RustFeatures {
        methods: {},
        // This field cannot be set from the CLI,
        as_args: ignore,
    },
    /// Enable support for native Rust unions if they are supported.
    untagged_union: bool {
        default: true,
        methods: {
            /// Disable support for native Rust unions, if supported.
            ///
            /// The default value of this option is set based on the value passed to
            /// [`Builder::rust_target`].
            pub fn disable_untagged_union(mut self) -> Self {
                self.options.untagged_union = false;
                self
            }
        }
        as_args: |value, args| (!value).as_args(args, "--disable-untagged-union"),
    },
    /// Whether we should record which items in the regex sets did match any C items.
    record_matches: bool {
        default: true,
        methods: {
            /// Set whether we should record which items in our regex sets did match any C items.
            ///
            /// Matches are recorded by default.
            pub fn record_matches(mut self, doit: bool) -> Self {
                self.options.record_matches = doit;
                self
            }

        },
        as_args: |value, args| (!value).as_args(args, "--no-record-matches"),
    },
    /// Whether `size_t` should be translated to `usize` automatically.
    size_t_is_usize: bool {
        default: true,
        methods: {
            /// Set whether `size_t` should be translated to `usize`.
            ///
            /// If `size_t` is translated to `usize`, type definitions for `size_t` will not be
            /// emitted.
            ///
            /// `size_t` is translated to `usize` by default.
            pub fn size_t_is_usize(mut self, is: bool) -> Self {
                self.options.size_t_is_usize = is;
                self
            }
        },
        as_args: |value, args| (!value).as_args(args, "--no-size_t-is-usize"),
    },
    /// The tool that should be used to format the generated bindings.
    formatter: Formatter {
        methods: {
            /// Set whether `rustfmt` should be used to format the generated bindings.
            ///
            /// `rustfmt` is used by default.
            ///
            /// This method overlaps in functionality with the more general [`Builder::formatter`].
            /// Thus, the latter should be preferred.
            #[deprecated]
            pub fn rustfmt_bindings(mut self, doit: bool) -> Self {
                self.options.formatter = if doit {
                    Formatter::Rustfmt
                } else {
                    Formatter::None
                };
                self
            }

            /// Set which tool should be used to format the generated bindings.
            ///
            /// The default formatter is [`Formatter::Rustfmt`].
            ///
            /// To be able to use `prettyplease` as a formatter, the `"prettyplease"` feature for
            /// `bindgen` must be enabled in the Cargo manifest.
            pub fn formatter(mut self, formatter: Formatter) -> Self {
                self.options.formatter = formatter;
                self
            }
        },
        as_args: |formatter, args| {
            if *formatter != Default::default() {
                args.push("--formatter".to_owned());
                args.push(formatter.to_string());
            }
        },
    },
    /// The absolute path to the `rustfmt` configuration file.
    rustfmt_configuration_file: Option<PathBuf> {
        methods: {
            /// Set the absolute path to the `rustfmt` configuration file.
            ///
            /// The default `rustfmt` options are used if `None` is passed to this method or if
            /// this method is not called at all.
            ///
            /// Calling this method will set the [`Builder::rustfmt_bindings`] option to `true`
            /// and the [`Builder::formatter`] option to [`Formatter::Rustfmt`].
            pub fn rustfmt_configuration_file(mut self, path: Option<PathBuf>) -> Self {
                self = self.formatter(Formatter::Rustfmt);
                self.options.rustfmt_configuration_file = path;
                self
            }
        },
        as_args: "--rustfmt-configuration-file",
    },
    /// Types that should not derive `PartialEq`.
    no_partialeq_types: RegexSet {
        methods: {
            regex_option! {
                /// Do not derive `PartialEq` for a given type.
                pub fn no_partialeq<T: Into<String>>(mut self, arg: T) -> Builder {
                    self.options.no_partialeq_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-partialeq",
    },
    /// Types that should not derive `Copy`.
    no_copy_types: RegexSet {
        methods: {
            regex_option! {
                /// Do not derive `Copy` and `Clone` for a given type.
                pub fn no_copy<T: Into<String>>(mut self, arg: T) -> Self {
                    self.options.no_copy_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-copy",
    },
    /// Types that should not derive `Debug`.
    no_debug_types: RegexSet {
        methods: {
            regex_option! {
                /// Do not derive `Debug` for a given type.
                pub fn no_debug<T: Into<String>>(mut self, arg: T) -> Self {
                    self.options.no_debug_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-debug",
    },
    /// Types that should not derive or implement `Default`.
    no_default_types: RegexSet {
        methods: {
            regex_option! {
                /// Do not derive or implement `Default` for a given type.
                pub fn no_default<T: Into<String>>(mut self, arg: T) -> Self {
                    self.options.no_default_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-default",
    },
    /// Types that should not derive `Hash`.
    no_hash_types: RegexSet {
        methods: {
            regex_option! {
                /// Do not derive `Hash` for a given type.
                pub fn no_hash<T: Into<String>>(mut self, arg: T) -> Builder {
                    self.options.no_hash_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--no-hash",
    },
    /// Types that should be annotated with `#[must_use]`.
    must_use_types: RegexSet {
        methods: {
            regex_option! {
                /// Annotate the given type with the `#[must_use]` attribute.
                pub fn must_use_type<T: Into<String>>(mut self, arg: T) -> Builder {
                    self.options.must_use_types.insert(arg.into());
                    self
                }
            }
        },
        as_args: "--must-use-type",
    },
    /// Whether C arrays should be regular pointers in rust or array pointers
    array_pointers_in_arguments: bool {
        methods: {
            /// Translate arrays `T arr[size]` into array pointers `*mut [T; size]` instead of
            /// translating them as `*mut T` which is the default.
            ///
            /// The same is done for `*const` pointers.
            pub fn array_pointers_in_arguments(mut self, doit: bool) -> Self {
                self.options.array_pointers_in_arguments = doit;
                self
            }

        },
        as_args: "--use-array-pointers-in-arguments",
    },
    /// The name of the `wasm_import_module`.
    wasm_import_module_name: Option<String> {
        methods: {
            /// Adds the `#[link(wasm_import_module = import_name)]` attribute to all the `extern`
            /// blocks generated by `bindgen`.
            ///
            /// This attribute is not added by default.
            pub fn wasm_import_module_name<T: Into<String>>(
                mut self,
                import_name: T,
            ) -> Self {
                self.options.wasm_import_module_name = Some(import_name.into());
                self
            }
        },
        as_args: "--wasm-import-module-name",
    },
    /// The name of the dynamic library (if we are generating bindings for a shared library).
    dynamic_library_name: Option<String> {
        methods: {
            /// Generate bindings for a shared library with the given name.
            ///
            /// This option is disabled by default.
            pub fn dynamic_library_name<T: Into<String>>(
                mut self,
                dynamic_library_name: T,
            ) -> Self {
                self.options.dynamic_library_name = Some(dynamic_library_name.into());
                self
            }
        },
        as_args: "--dynamic-loading",
    },
    /// Whether to require successful linkage for all routines in a shared library.
    dynamic_link_require_all: bool {
        methods: {
            /// Set whether to require successful linkage for all routines in a shared library.
            /// This allows us to optimize function calls by being able to safely assume function
            /// pointers are valid.
            ///
            /// This option only comes into effect if the [`Builder::dynamic_library_name`] option
            /// is set.
            ///
            /// This option is disabled by default.
            pub fn dynamic_link_require_all(mut self, req: bool) -> Self {
                self.options.dynamic_link_require_all = req;
                self
            }
        },
        as_args: "--dynamic-link-require-all",
    },
    /// Whether to only make generated bindings `pub` if the items would be publicly accessible by
    /// C++.
    respect_cxx_access_specs: bool {
        methods: {
            /// Set whether to respect the C++ access specifications.
            ///
            /// Passing `true` to this method will set the visibility of the generated Rust items
            /// as `pub` only if the corresponding C++ items are publicly accessible instead of
            /// marking all the items as public, which is the default.
            pub fn respect_cxx_access_specs(mut self, doit: bool) -> Self {
                self.options.respect_cxx_access_specs = doit;
                self
            }

        },
        as_args: "--respect-cxx-access-specs",
    },
    /// Whether to translate `enum` integer types to native Rust integer types.
    translate_enum_integer_types: bool {
        methods: {
            /// Set whether to always translate `enum` integer types to native Rust integer types.
            ///
            /// Passing `true` to this method will result in `enum`s having types such as `u32` and
            /// `i16` instead of `c_uint` and `c_short` which is the default. The `#[repr]` types
            /// of Rust `enum`s are always translated to Rust integer types.
            pub fn translate_enum_integer_types(mut self, doit: bool) -> Self {
                self.options.translate_enum_integer_types = doit;
                self
            }
        },
        as_args: "--translate-enum-integer-types",
    },
    /// Whether to generate types with C style naming.
    c_naming: bool {
        methods: {
            /// Set whether to generate types with C style naming.
            ///
            /// Passing `true` to this method will add prefixes to the generated type names. For
            /// example, instead of a `struct` with name `A` we will generate a `struct` with
            /// `struct_A`. Currently applies to `struct`s, `union`s, and `enum`s.
            pub fn c_naming(mut self, doit: bool) -> Self {
                self.options.c_naming = doit;
                self
            }
        },
        as_args: "--c-naming",
    },
    /// Whether to always emit explicit padding fields.
    force_explicit_padding: bool {
        methods: {
            /// Set whether to always emit explicit padding fields.
            ///
            /// This option should be enabled if a `struct` needs to be serialized in its native
            /// format (padding bytes and all). This could be required if such `struct` will be
            /// written to a file or sent over the network, as anything reading the padding bytes
            /// of a struct may cause undefined behavior.
            ///
            /// Padding fields are not emitted by default.
            pub fn explicit_padding(mut self, doit: bool) -> Self {
                self.options.force_explicit_padding = doit;
                self
            }
        },
        as_args: "--explicit-padding",
    },
    /// Whether to emit vtable functions.
    vtable_generation: bool {
        methods: {
            /// Set whether to enable experimental support to generate virtual table functions.
            ///
            /// This option should mostly work, though some edge cases are likely to be broken.
            ///
            /// Virtual table generation is disabled by default.
            pub fn vtable_generation(mut self, doit: bool) -> Self {
                self.options.vtable_generation = doit;
                self
            }
        },
        as_args: "--vtable-generation",
    },
    /// Whether to sort the generated Rust items.
    sort_semantically: bool {
        methods: {
            /// Set whether to sort the generated Rust items in a predefined manner.
            ///
            /// Items are not ordered by default.
            pub fn sort_semantically(mut self, doit: bool) -> Self {
                self.options.sort_semantically = doit;
                self
            }
        },
        as_args: "--sort-semantically",
    },
    /// Whether to deduplicate `extern` blocks.
    merge_extern_blocks: bool {
        methods: {
            /// Merge all extern blocks under the same module into a single one.
            ///
            /// Extern blocks are not merged by default.
            pub fn merge_extern_blocks(mut self, doit: bool) -> Self {
                self.options.merge_extern_blocks = doit;
                self
            }
        },
        as_args: "--merge-extern-blocks",
    },
    /// Whether to wrap unsafe operations in unsafe blocks.
    wrap_unsafe_ops: bool {
        methods: {
            /// Wrap all unsafe operations in unsafe blocks.
            ///
            /// Unsafe operations are not wrapped by default.
            pub fn wrap_unsafe_ops(mut self, doit: bool) -> Self {
                self.options.wrap_unsafe_ops = doit;
                self
            }
        },
        as_args: "--wrap-unsafe-ops",
    },
    /// Use DSTs to represent structures with flexible array members.
    flexarray_dst: bool {
        methods: {
            /// Use DSTs to represent structures with flexible array members.
            ///
            /// This option is disabled by default.
            pub fn flexarray_dst(mut self, doit: bool) -> Self {
                self.options.flexarray_dst = doit;
                self
            }
        },
        as_args: "--flexarray-dst",
    },
    /// Patterns for functions whose ABI should be overridden.
    abi_overrides: HashMap<Abi, RegexSet> {
        methods: {
            regex_option! {
                /// Override the ABI of a given function.
                pub fn override_abi<T: Into<String>>(mut self, abi: Abi, arg: T) -> Self {
                    self.options
                        .abi_overrides
                        .entry(abi)
                        .or_default()
                        .insert(arg.into());
                    self
                }
            }
        },
        as_args: |overrides, args| {
            for (abi, set) in overrides {
                for item in set.get_items() {
                    args.push("--override-abi".to_owned());
                    args.push(format!("{item}={abi}"));
                }
            }
        },
    },
    /// Whether to generate wrappers for `static` functions.
    wrap_static_fns: bool {
        methods: {
            /// Set whether to generate wrappers for `static`` functions.
            ///
            /// Passing `true` to this method will generate a C source file with non-`static`
            /// functions that call the `static` functions found in the input headers and can be
            /// called from Rust once the source file is compiled.
            ///
            /// The path of this source file can be set using the [`Builder::wrap_static_fns_path`]
            /// method.
            pub fn wrap_static_fns(mut self, doit: bool) -> Self {
                self.options.wrap_static_fns = doit;
                self
            }
        },
        as_args: "--wrap-static-fns",
    },
    /// The suffix to be added to the function wrappers for `static` functions.
    wrap_static_fns_suffix: Option<String> {
        methods: {
            /// Set the suffix added to the wrappers for `static` functions.
            ///
            /// This option only comes into effect if `true` is passed to the
            /// [`Builder::wrap_static_fns`] method.
            ///
            /// The default suffix is `__extern`.
            pub fn wrap_static_fns_suffix<T: AsRef<str>>(mut self, suffix: T) -> Self {
                self.options.wrap_static_fns_suffix = Some(suffix.as_ref().to_owned());
                self
            }
        },
        as_args: "--wrap-static-fns-suffix",
    },
    /// The path of the file where the wrappers for `static` functions will be emitted.
    wrap_static_fns_path: Option<PathBuf> {
        methods: {
            /// Set the path for the source code file that would be created if any wrapper
            /// functions must be generated due to the presence of `static` functions.
            ///
            /// `bindgen` will automatically add the right extension to the header and source code
            /// files.
            ///
            /// This option only comes into effect if `true` is passed to the
            /// [`Builder::wrap_static_fns`] method.
            ///
            /// The default path is `temp_dir/bindgen/extern`, where `temp_dir` is the path
            /// returned by [`std::env::temp_dir`] .
            pub fn wrap_static_fns_path<T: AsRef<Path>>(mut self, path: T) -> Self {
                self.options.wrap_static_fns_path = Some(path.as_ref().to_owned());
                self
            }
        },
        as_args: "--wrap-static-fns-path",
    },
    /// Default visibility of fields.
    default_visibility: FieldVisibilityKind {
        methods: {
            /// Set the default visibility of fields, including bitfields and accessor methods for
            /// bitfields.
            ///
            /// This option only comes into effect if the [`Builder::respect_cxx_access_specs`]
            /// option is disabled.
            pub fn default_visibility(
                mut self,
                visibility: FieldVisibilityKind,
            ) -> Self {
                self.options.default_visibility = visibility;
                self
            }
        },
        as_args: |visibility, args| {
            if *visibility != Default::default() {
                args.push("--default-visibility".to_owned());
                args.push(visibility.to_string());
            }
        },
    },
    /// Whether to emit diagnostics or not.
    emit_diagnostics: bool {
        methods: {
            #[cfg(feature = "experimental")]
            /// Emit diagnostics.
            ///
            /// These diagnostics are emitted to `stderr` if you are using `bindgen-cli` or printed
            /// using `cargo:warning=` if you are using `bindgen` as a `build-dependency`.
            ///
            /// Diagnostics are not emitted by default.
            ///
            /// The layout and contents of these diagnostic messages are not covered by versioning
            /// and can change without notice.
            pub fn emit_diagnostics(mut self) -> Self {
                self.options.emit_diagnostics = true;
                self
            }
        },
        as_args: "--emit-diagnostics",
    },
    /// Whether to use Clang evaluation on temporary files as a fallback for macros that fail to
    /// parse.
    clang_macro_fallback: bool {
        methods: {
            /// Use Clang as a fallback for macros that fail to parse using `CExpr`.
            ///
            /// This uses a workaround to evaluate each macro in a temporary file. Because this
            /// results in slower compilation, this option is opt-in.
            pub fn clang_macro_fallback(mut self) -> Self {
                self.options.clang_macro_fallback = true;
                self
            }
        },
        as_args: "--clang-macro-fallback",
    }
    /// Path to use for temporary files created by clang macro fallback code like precompiled
    /// headers.
    clang_macro_fallback_build_dir: Option<PathBuf> {
        methods: {
            /// Set a path to a directory to which `.c` and `.h.pch` files should be written for the
            /// purpose of using clang to evaluate macros that can't be easily parsed.
            ///
            /// The default location for `.h.pch` files is the directory that the corresponding
            /// `.h` file is located in. The default for the temporary `.c` file used for clang
            /// parsing is the current working directory. Both of these defaults are overridden
            /// by this option.
            pub fn clang_macro_fallback_build_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
                self.options.clang_macro_fallback_build_dir = Some(path.as_ref().to_owned());
                self
            }
        },
        as_args: "--clang-macro-fallback-build-dir",
    }
}

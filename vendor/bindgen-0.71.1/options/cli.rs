#![allow(unused_qualifications)] // Clap somehow generates a lot of these

use crate::{
    builder,
    callbacks::{
        AttributeInfo, DeriveInfo, ItemInfo, ParseCallbacks, TypeKind,
    },
    features::{RustEdition, EARLIEST_STABLE_RUST},
    regex_set::RegexSet,
    Abi, AliasVariation, Builder, CodegenConfig, EnumVariation,
    FieldVisibilityKind, Formatter, MacroTypeVariation, NonCopyUnionStyle,
    RustTarget,
};
use clap::{
    error::{Error, ErrorKind},
    CommandFactory, Parser,
};
use proc_macro2::TokenStream;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs::File, process::exit};

fn rust_target_help() -> String {
    format!(
        "Version of the Rust compiler to target. Any Rust version after {EARLIEST_STABLE_RUST} is supported. Defaults to {}.",
        RustTarget::default()
    )
}

fn rust_edition_help() -> String {
    format!("Rust edition to target. Defaults to the latest edition supported by the chosen Rust target. Possible values: ({}). ", RustEdition::ALL.map(|e| e.to_string()).join("|"))
}

fn parse_codegen_config(
    what_to_generate: &str,
) -> Result<CodegenConfig, Error> {
    let mut config = CodegenConfig::empty();
    for what in what_to_generate.split(',') {
        match what {
            "functions" => config.insert(CodegenConfig::FUNCTIONS),
            "types" => config.insert(CodegenConfig::TYPES),
            "vars" => config.insert(CodegenConfig::VARS),
            "methods" => config.insert(CodegenConfig::METHODS),
            "constructors" => config.insert(CodegenConfig::CONSTRUCTORS),
            "destructors" => config.insert(CodegenConfig::DESTRUCTORS),
            otherwise => {
                return Err(Error::raw(
                    ErrorKind::InvalidValue,
                    format!("Unknown codegen item kind: {otherwise}"),
                ));
            }
        }
    }

    Ok(config)
}

fn parse_rustfmt_config_path(path_str: &str) -> Result<PathBuf, Error> {
    let path = Path::new(path_str);

    if !path.is_absolute() {
        return Err(Error::raw(
            ErrorKind::InvalidValue,
            "--rustfmt-configuration-file needs to be an absolute path!",
        ));
    }

    if path.to_str().is_none() {
        return Err(Error::raw(
            ErrorKind::InvalidUtf8,
            "--rustfmt-configuration-file contains non-valid UTF8 characters.",
        ));
    }

    Ok(path.to_path_buf())
}

fn parse_abi_override(abi_override: &str) -> Result<(Abi, String), Error> {
    let (regex, abi_str) = abi_override
        .rsplit_once('=')
        .ok_or_else(|| Error::raw(ErrorKind::InvalidValue, "Missing `=`"))?;

    let abi = abi_str
        .parse()
        .map_err(|err| Error::raw(ErrorKind::InvalidValue, err))?;

    Ok((abi, regex.to_owned()))
}

fn parse_custom_derive(
    custom_derive: &str,
) -> Result<(Vec<String>, String), Error> {
    let (regex, derives) = custom_derive
        .rsplit_once('=')
        .ok_or_else(|| Error::raw(ErrorKind::InvalidValue, "Missing `=`"))?;

    let derives = derives.split(',').map(|s| s.to_owned()).collect();

    Ok((derives, regex.to_owned()))
}

fn parse_custom_attribute(
    custom_attribute: &str,
) -> Result<(Vec<String>, String), Error> {
    let mut brace_level = 0;
    let (regex, attributes) = custom_attribute
        .rsplit_once(|c| {
            match c {
                ']' => brace_level += 1,
                '[' => brace_level -= 1,
                _ => {}
            }
            c == '=' && brace_level == 0
        })
        .ok_or_else(|| Error::raw(ErrorKind::InvalidValue, "Missing `=`"))?;

    let mut brace_level = 0;
    let attributes = attributes
        .split(|c| {
            match c {
                ']' => brace_level += 1,
                '[' => brace_level -= 1,
                _ => {}
            }
            c == ',' && brace_level == 0
        })
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    for attribute in &attributes {
        if let Err(err) = TokenStream::from_str(attribute) {
            return Err(Error::raw(ErrorKind::InvalidValue, err));
        }
    }

    Ok((attributes, regex.to_owned()))
}

#[derive(Parser, Debug)]
#[clap(
    about = "Generates Rust bindings from C/C++ headers.",
    override_usage = "bindgen <FLAGS> <OPTIONS> <HEADER> -- <CLANG_ARGS>...",
    trailing_var_arg = true
)]
#[allow(clippy::doc_markdown)]
struct BindgenCommand {
    /// C or C++ header file.
    header: Option<String>,
    /// Path to write depfile to.
    #[arg(long)]
    depfile: Option<String>,
    /// The default STYLE of code used to generate enums.
    #[arg(long, value_name = "STYLE")]
    default_enum_style: Option<EnumVariation>,
    /// Mark any enum whose name matches REGEX as a set of bitfield flags.
    #[arg(long, value_name = "REGEX")]
    bitfield_enum: Vec<String>,
    /// Mark any enum whose name matches REGEX as a newtype.
    #[arg(long, value_name = "REGEX")]
    newtype_enum: Vec<String>,
    /// Mark any enum whose name matches REGEX as a global newtype.
    #[arg(long, value_name = "REGEX")]
    newtype_global_enum: Vec<String>,
    /// Mark any enum whose name matches REGEX as a Rust enum.
    #[arg(long, value_name = "REGEX")]
    rustified_enum: Vec<String>,
    /// Mark any enum whose name matches REGEX as a non-exhaustive Rust enum.
    #[arg(long, value_name = "REGEX")]
    rustified_non_exhaustive_enum: Vec<String>,
    /// Mark any enum whose name matches REGEX as a series of constants.
    #[arg(long, value_name = "REGEX")]
    constified_enum: Vec<String>,
    /// Mark any enum whose name matches REGEX as a module of constants.
    #[arg(long, value_name = "REGEX")]
    constified_enum_module: Vec<String>,
    /// The default signed/unsigned TYPE for C macro constants.
    #[arg(long, value_name = "TYPE")]
    default_macro_constant_type: Option<MacroTypeVariation>,
    /// The default STYLE of code used to generate typedefs.
    #[arg(long, value_name = "STYLE")]
    default_alias_style: Option<AliasVariation>,
    /// Mark any typedef alias whose name matches REGEX to use normal type aliasing.
    #[arg(long, value_name = "REGEX")]
    normal_alias: Vec<String>,
    /// Mark any typedef alias whose name matches REGEX to have a new type generated for it.
    #[arg(long, value_name = "REGEX")]
    new_type_alias: Vec<String>,
    /// Mark any typedef alias whose name matches REGEX to have a new type with Deref and DerefMut to the inner type.
    #[arg(long, value_name = "REGEX")]
    new_type_alias_deref: Vec<String>,
    /// The default STYLE of code used to generate unions with non-Copy members. Note that ManuallyDrop was first stabilized in Rust 1.20.0.
    #[arg(long, value_name = "STYLE")]
    default_non_copy_union_style: Option<NonCopyUnionStyle>,
    /// Mark any union whose name matches REGEX and who has a non-Copy member to use a bindgen-generated wrapper for fields.
    #[arg(long, value_name = "REGEX")]
    bindgen_wrapper_union: Vec<String>,
    /// Mark any union whose name matches REGEX and who has a non-Copy member to use ManuallyDrop (stabilized in Rust 1.20.0) for fields.
    #[arg(long, value_name = "REGEX")]
    manually_drop_union: Vec<String>,
    /// Mark TYPE as hidden.
    #[arg(long, value_name = "TYPE")]
    blocklist_type: Vec<String>,
    /// Mark FUNCTION as hidden.
    #[arg(long, value_name = "FUNCTION")]
    blocklist_function: Vec<String>,
    /// Mark ITEM as hidden.
    #[arg(long, value_name = "ITEM")]
    blocklist_item: Vec<String>,
    /// Mark FILE as hidden.
    #[arg(long, value_name = "FILE")]
    blocklist_file: Vec<String>,
    /// Mark VAR as hidden.
    #[arg(long, value_name = "VAR")]
    blocklist_var: Vec<String>,
    /// Avoid generating layout tests for any type.
    #[arg(long)]
    no_layout_tests: bool,
    /// Avoid deriving Copy on any type.
    #[arg(long)]
    no_derive_copy: bool,
    /// Avoid deriving Debug on any type.
    #[arg(long)]
    no_derive_debug: bool,
    /// Avoid deriving Default on any type.
    #[arg(long, hide = true)]
    no_derive_default: bool,
    /// Create a Debug implementation if it cannot be derived automatically.
    #[arg(long)]
    impl_debug: bool,
    /// Create a PartialEq implementation if it cannot be derived automatically.
    #[arg(long)]
    impl_partialeq: bool,
    /// Derive Default on any type.
    #[arg(long)]
    with_derive_default: bool,
    /// Derive Hash on any type.
    #[arg(long)]
    with_derive_hash: bool,
    /// Derive PartialEq on any type.
    #[arg(long)]
    with_derive_partialeq: bool,
    /// Derive PartialOrd on any type.
    #[arg(long)]
    with_derive_partialord: bool,
    /// Derive Eq on any type.
    #[arg(long)]
    with_derive_eq: bool,
    /// Derive Ord on any type.
    #[arg(long)]
    with_derive_ord: bool,
    /// Avoid including doc comments in the output, see: <https://github.com/rust-lang/rust-bindgen/issues/426>
    #[arg(long)]
    no_doc_comments: bool,
    /// Disable allowlisting types recursively. This will cause bindgen to emit Rust code that won't compile! See the `bindgen::Builder::allowlist_recursively` method's documentation for details.
    #[arg(long)]
    no_recursive_allowlist: bool,
    /// Use extern crate instead of use for objc.
    #[arg(long)]
    objc_extern_crate: bool,
    /// Generate block signatures instead of void pointers.
    #[arg(long)]
    generate_block: bool,
    /// Generate string constants as `&CStr` instead of `&[u8]`.
    #[arg(long)]
    generate_cstr: bool,
    /// Use extern crate instead of use for block.
    #[arg(long)]
    block_extern_crate: bool,
    /// Do not trust the libclang-provided mangling
    #[arg(long)]
    distrust_clang_mangling: bool,
    /// Output bindings for builtin definitions, e.g. __builtin_va_list.
    #[arg(long)]
    builtins: bool,
    /// Use the given PREFIX before raw types instead of ::std::os::raw.
    #[arg(long, value_name = "PREFIX")]
    ctypes_prefix: Option<String>,
    /// Use the given PREFIX for anonymous fields.
    #[arg(long, value_name = "PREFIX")]
    anon_fields_prefix: Option<String>,
    /// Time the different bindgen phases and print to stderr
    #[arg(long)]
    time_phases: bool,
    /// Output the Clang AST for debugging purposes.
    #[arg(long)]
    emit_clang_ast: bool,
    /// Output our internal IR for debugging purposes.
    #[arg(long)]
    emit_ir: bool,
    /// Dump a graphviz dot file to PATH.
    #[arg(long, value_name = "PATH")]
    emit_ir_graphviz: Option<String>,
    /// Enable support for C++ namespaces.
    #[arg(long)]
    enable_cxx_namespaces: bool,
    /// Disable namespacing via mangling, causing bindgen to generate names like `Baz` instead of `foo_bar_Baz` for an input name `foo::bar::Baz`.
    #[arg(long)]
    disable_name_namespacing: bool,
    /// Disable nested struct naming, causing bindgen to generate names like `bar` instead of `foo_bar` for a nested definition `struct foo { struct bar { } b; };`.
    #[arg(long)]
    disable_nested_struct_naming: bool,
    /// Disable support for native Rust unions.
    #[arg(long)]
    disable_untagged_union: bool,
    /// Suppress insertion of bindgen's version identifier into generated bindings.
    #[arg(long)]
    disable_header_comment: bool,
    /// Do not generate bindings for functions or methods. This is useful when you only care about struct layouts.
    #[arg(long)]
    ignore_functions: bool,
    /// Generate only given items, split by commas. Valid values are `functions`,`types`, `vars`, `methods`, `constructors` and `destructors`.
    #[arg(long, value_parser = parse_codegen_config)]
    generate: Option<CodegenConfig>,
    /// Do not generate bindings for methods.
    #[arg(long)]
    ignore_methods: bool,
    /// Do not automatically convert floats to f32/f64.
    #[arg(long)]
    no_convert_floats: bool,
    /// Do not prepend the enum name to constant or newtype variants.
    #[arg(long)]
    no_prepend_enum_name: bool,
    /// Do not try to detect default include paths
    #[arg(long)]
    no_include_path_detection: bool,
    /// Try to fit macro constants into types smaller than u32/i32
    #[arg(long)]
    fit_macro_constant_types: bool,
    /// Mark TYPE as opaque.
    #[arg(long, value_name = "TYPE")]
    opaque_type: Vec<String>,
    ///  Write Rust bindings to OUTPUT.
    #[arg(long, short, value_name = "OUTPUT")]
    output: Option<String>,
    /// Add a raw line of Rust code at the beginning of output.
    #[arg(long)]
    raw_line: Vec<String>,
    /// Add a RAW_LINE of Rust code to a given module with name MODULE_NAME.
    #[arg(long, number_of_values = 2, value_names = ["MODULE_NAME", "RAW_LINE"])]
    module_raw_line: Vec<String>,
    #[arg(long, help = rust_target_help())]
    rust_target: Option<RustTarget>,
    #[arg(long, value_name = "EDITION", help = rust_edition_help())]
    rust_edition: Option<RustEdition>,
    /// Use types from Rust core instead of std.
    #[arg(long)]
    use_core: bool,
    /// Conservatively generate inline namespaces to avoid name conflicts.
    #[arg(long)]
    conservative_inline_namespaces: bool,
    /// Allowlist all the free-standing functions matching REGEX. Other non-allowlisted functions will not be generated.
    #[arg(long, value_name = "REGEX")]
    allowlist_function: Vec<String>,
    /// Generate inline functions.
    #[arg(long)]
    generate_inline_functions: bool,
    /// Only generate types matching REGEX. Other non-allowlisted types will not be generated.
    #[arg(long, value_name = "REGEX")]
    allowlist_type: Vec<String>,
    /// Allowlist all the free-standing variables matching REGEX. Other non-allowlisted variables will not be generated.
    #[arg(long, value_name = "REGEX")]
    allowlist_var: Vec<String>,
    /// Allowlist all contents of PATH.
    #[arg(long, value_name = "PATH")]
    allowlist_file: Vec<String>,
    /// Allowlist all items matching REGEX. Other non-allowlisted items will not be generated.
    #[arg(long, value_name = "REGEX")]
    allowlist_item: Vec<String>,
    /// Print verbose error messages.
    #[arg(long)]
    verbose: bool,
    /// Preprocess and dump the input header files to disk. Useful when debugging bindgen, using C-Reduce, or when filing issues. The resulting file will be named something like `__bindgen.i` or `__bindgen.ii`.
    #[arg(long)]
    dump_preprocessed_input: bool,
    /// Do not record matching items in the regex sets. This disables reporting of unused items.
    #[arg(long)]
    no_record_matches: bool,
    /// Do not bind size_t as usize (useful on platforms where those types are incompatible).
    #[arg(long = "no-size_t-is-usize")]
    no_size_t_is_usize: bool,
    /// Do not format the generated bindings with rustfmt. This option is deprecated, please use
    /// `--formatter=none` instead.
    #[arg(long)]
    no_rustfmt_bindings: bool,
    /// Which FORMATTER should be used for the bindings
    #[arg(
        long,
        value_name = "FORMATTER",
        conflicts_with = "no_rustfmt_bindings"
    )]
    formatter: Option<Formatter>,
    /// The absolute PATH to the rustfmt configuration file. The configuration file will be used for formatting the bindings. This parameter sets `formatter` to `rustfmt`.
    #[arg(long, value_name = "PATH", conflicts_with = "no_rustfmt_bindings", value_parser=parse_rustfmt_config_path)]
    rustfmt_configuration_file: Option<PathBuf>,
    /// Avoid deriving PartialEq for types matching REGEX.
    #[arg(long, value_name = "REGEX")]
    no_partialeq: Vec<String>,
    /// Avoid deriving Copy and Clone for types matching REGEX.
    #[arg(long, value_name = "REGEX")]
    no_copy: Vec<String>,
    /// Avoid deriving Debug for types matching REGEX.
    #[arg(long, value_name = "REGEX")]
    no_debug: Vec<String>,
    /// Avoid deriving/implementing Default for types matching REGEX.
    #[arg(long, value_name = "REGEX")]
    no_default: Vec<String>,
    /// Avoid deriving Hash for types matching REGEX.
    #[arg(long, value_name = "REGEX")]
    no_hash: Vec<String>,
    /// Add `#[must_use]` annotation to types matching REGEX.
    #[arg(long, value_name = "REGEX")]
    must_use_type: Vec<String>,
    /// Enables detecting unexposed attributes in functions (slow). Used to generate `#[must_use]` annotations.
    #[arg(long)]
    enable_function_attribute_detection: bool,
    /// Use `*const [T; size]` instead of `*const T` for C arrays
    #[arg(long)]
    use_array_pointers_in_arguments: bool,
    /// The NAME to be used in a #[link(wasm_import_module = ...)] statement
    #[arg(long, value_name = "NAME")]
    wasm_import_module_name: Option<String>,
    /// Use dynamic loading mode with the given library NAME.
    #[arg(long, value_name = "NAME")]
    dynamic_loading: Option<String>,
    /// Require successful linkage to all functions in the library.
    #[arg(long)]
    dynamic_link_require_all: bool,
    /// Prefix the name of exported symbols.
    #[arg(long)]
    prefix_link_name: Option<String>,
    /// Makes generated bindings `pub` only for items if the items are publicly accessible in C++.
    #[arg(long)]
    respect_cxx_access_specs: bool,
    /// Always translate enum integer types to native Rust integer types.
    #[arg(long)]
    translate_enum_integer_types: bool,
    /// Generate types with C style naming.
    #[arg(long)]
    c_naming: bool,
    /// Always output explicit padding fields.
    #[arg(long)]
    explicit_padding: bool,
    /// Enables generation of vtable functions.
    #[arg(long)]
    vtable_generation: bool,
    /// Enables sorting of code generation in a predefined manner.
    #[arg(long)]
    sort_semantically: bool,
    /// Deduplicates extern blocks.
    #[arg(long)]
    merge_extern_blocks: bool,
    /// Overrides the ABI of functions matching REGEX. The OVERRIDE value must be of the shape REGEX=ABI where ABI can be one of C, stdcall, efiapi, fastcall, thiscall, aapcs, win64 or C-unwind<.>
    #[arg(long, value_name = "OVERRIDE", value_parser = parse_abi_override)]
    override_abi: Vec<(Abi, String)>,
    /// Wrap unsafe operations in unsafe blocks.
    #[arg(long)]
    wrap_unsafe_ops: bool,
    /// Enable fallback for clang macro parsing.
    #[arg(long)]
    clang_macro_fallback: bool,
    /// Set path for temporary files generated by fallback for clang macro parsing.
    #[arg(long)]
    clang_macro_fallback_build_dir: Option<PathBuf>,
    /// Use DSTs to represent structures with flexible array members.
    #[arg(long)]
    flexarray_dst: bool,
    /// Derive custom traits on any kind of type. The CUSTOM value must be of the shape REGEX=DERIVE where DERIVE is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_derive)]
    with_derive_custom: Vec<(Vec<String>, String)>,
    /// Derive custom traits on a `struct`. The CUSTOM value must be of the shape REGEX=DERIVE where DERIVE is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_derive)]
    with_derive_custom_struct: Vec<(Vec<String>, String)>,
    /// Derive custom traits on an `enum. The CUSTOM value must be of the shape REGEX=DERIVE where DERIVE is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_derive)]
    with_derive_custom_enum: Vec<(Vec<String>, String)>,
    /// Derive custom traits on a `union`. The CUSTOM value must be of the shape REGEX=DERIVE where DERIVE is a coma-separated list of derive macros.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_derive)]
    with_derive_custom_union: Vec<(Vec<String>, String)>,
    /// Add custom attributes on any kind of type. The CUSTOM value must be of the shape REGEX=ATTRIBUTE where ATTRIBUTE is a coma-separated list of attributes.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_attribute)]
    with_attribute_custom: Vec<(Vec<String>, String)>,
    /// Add custom attributes on a `struct`. The CUSTOM value must be of the shape REGEX=ATTRIBUTE where ATTRIBUTE is a coma-separated list of attributes.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_attribute)]
    with_attribute_custom_struct: Vec<(Vec<String>, String)>,
    /// Add custom attributes on an `enum. The CUSTOM value must be of the shape REGEX=ATTRIBUTE where ATTRIBUTE is a coma-separated list of attributes.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_attribute)]
    with_attribute_custom_enum: Vec<(Vec<String>, String)>,
    /// Add custom attributes on a `union`. The CUSTOM value must be of the shape REGEX=ATTRIBUTE where ATTRIBUTE is a coma-separated list of attributes.
    #[arg(long, value_name = "CUSTOM", value_parser = parse_custom_attribute)]
    with_attribute_custom_union: Vec<(Vec<String>, String)>,
    /// Generate wrappers for `static` and `static inline` functions.
    #[arg(long)]
    wrap_static_fns: bool,
    /// Sets the PATH for the source file that must be created due to the presence of `static` and
    /// `static inline` functions.
    #[arg(long, value_name = "PATH")]
    wrap_static_fns_path: Option<PathBuf>,
    /// Sets the SUFFIX added to the extern wrapper functions generated for `static` and `static
    /// inline` functions.
    #[arg(long, value_name = "SUFFIX")]
    wrap_static_fns_suffix: Option<String>,
    /// Set the default VISIBILITY of fields, including bitfields and accessor methods for
    /// bitfields. This flag is ignored if the `--respect-cxx-access-specs` flag is used.
    #[arg(long, value_name = "VISIBILITY")]
    default_visibility: Option<FieldVisibilityKind>,
    /// Whether to emit diagnostics or not.
    #[cfg(feature = "experimental")]
    #[arg(long, requires = "experimental")]
    emit_diagnostics: bool,
    /// Generates completions for the specified SHELL, sends them to `stdout` and exits.
    #[arg(long, value_name = "SHELL")]
    generate_shell_completions: Option<clap_complete::Shell>,
    /// Enables experimental features.
    #[arg(long)]
    experimental: bool,
    /// Prints the version, and exits
    #[arg(short = 'V', long)]
    version: bool,
    /// Arguments to be passed straight through to clang.
    clang_args: Vec<String>,
}

/// Construct a new [`Builder`](./struct.Builder.html) from command line flags.
pub fn builder_from_flags<I>(
    args: I,
) -> Result<(Builder, Box<dyn io::Write>, bool), io::Error>
where
    I: Iterator<Item = String>,
{
    let command = BindgenCommand::parse_from(args);

    let BindgenCommand {
        header,
        depfile,
        default_enum_style,
        bitfield_enum,
        newtype_enum,
        newtype_global_enum,
        rustified_enum,
        rustified_non_exhaustive_enum,
        constified_enum,
        constified_enum_module,
        default_macro_constant_type,
        default_alias_style,
        normal_alias,
        new_type_alias,
        new_type_alias_deref,
        default_non_copy_union_style,
        bindgen_wrapper_union,
        manually_drop_union,
        blocklist_type,
        blocklist_function,
        blocklist_item,
        blocklist_file,
        blocklist_var,
        no_layout_tests,
        no_derive_copy,
        no_derive_debug,
        no_derive_default,
        impl_debug,
        impl_partialeq,
        with_derive_default,
        with_derive_hash,
        with_derive_partialeq,
        with_derive_partialord,
        with_derive_eq,
        with_derive_ord,
        no_doc_comments,
        no_recursive_allowlist,
        objc_extern_crate,
        generate_block,
        generate_cstr,
        block_extern_crate,
        distrust_clang_mangling,
        builtins,
        ctypes_prefix,
        anon_fields_prefix,
        time_phases,
        emit_clang_ast,
        emit_ir,
        emit_ir_graphviz,
        enable_cxx_namespaces,
        disable_name_namespacing,
        disable_nested_struct_naming,
        disable_untagged_union,
        disable_header_comment,
        ignore_functions,
        generate,
        ignore_methods,
        no_convert_floats,
        no_prepend_enum_name,
        no_include_path_detection,
        fit_macro_constant_types,
        opaque_type,
        output,
        raw_line,
        module_raw_line,
        rust_target,
        rust_edition,
        use_core,
        conservative_inline_namespaces,
        allowlist_function,
        generate_inline_functions,
        allowlist_type,
        allowlist_var,
        allowlist_file,
        allowlist_item,
        verbose,
        dump_preprocessed_input,
        no_record_matches,
        no_size_t_is_usize,
        no_rustfmt_bindings,
        formatter,
        rustfmt_configuration_file,
        no_partialeq,
        no_copy,
        no_debug,
        no_default,
        no_hash,
        must_use_type,
        enable_function_attribute_detection,
        use_array_pointers_in_arguments,
        wasm_import_module_name,
        dynamic_loading,
        dynamic_link_require_all,
        prefix_link_name,
        respect_cxx_access_specs,
        translate_enum_integer_types,
        c_naming,
        explicit_padding,
        vtable_generation,
        sort_semantically,
        merge_extern_blocks,
        override_abi,
        wrap_unsafe_ops,
        clang_macro_fallback,
        clang_macro_fallback_build_dir,
        flexarray_dst,
        with_derive_custom,
        with_derive_custom_struct,
        with_derive_custom_enum,
        with_derive_custom_union,
        with_attribute_custom,
        with_attribute_custom_struct,
        with_attribute_custom_enum,
        with_attribute_custom_union,
        wrap_static_fns,
        wrap_static_fns_path,
        wrap_static_fns_suffix,
        default_visibility,
        #[cfg(feature = "experimental")]
        emit_diagnostics,
        generate_shell_completions,
        experimental: _,
        version,
        clang_args,
    } = command;

    if let Some(shell) = generate_shell_completions {
        clap_complete::generate(
            shell,
            &mut BindgenCommand::command(),
            "bindgen",
            &mut io::stdout(),
        );

        exit(0)
    }

    if version {
        println!(
            "bindgen {}",
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
        );
        if verbose {
            println!("Clang: {}", crate::clang_version().full);
        }

        exit(0)
    }

    if header.is_none() {
        return Err(io::Error::new(io::ErrorKind::Other, "Header not found"));
    }

    let mut builder = builder();

    #[derive(Debug)]
    struct PrefixLinkNameCallback {
        prefix: String,
    }

    impl ParseCallbacks for PrefixLinkNameCallback {
        fn generated_link_name_override(
            &self,
            item_info: ItemInfo<'_>,
        ) -> Option<String> {
            let mut prefix = self.prefix.clone();
            prefix.push_str(item_info.name);
            Some(prefix)
        }
    }

    #[derive(Debug)]
    struct CustomDeriveCallback {
        derives: Vec<String>,
        kind: Option<TypeKind>,
        regex_set: RegexSet,
    }

    impl ParseCallbacks for CustomDeriveCallback {
        fn cli_args(&self) -> Vec<String> {
            let mut args = vec![];

            let flag = match &self.kind {
                None => "--with-derive-custom",
                Some(TypeKind::Struct) => "--with-derive-custom-struct",
                Some(TypeKind::Enum) => "--with-derive-custom-enum",
                Some(TypeKind::Union) => "--with-derive-custom-union",
            };

            let derives = self.derives.join(",");

            for item in self.regex_set.get_items() {
                args.extend_from_slice(&[
                    flag.to_owned(),
                    format!("{item}={derives}"),
                ]);
            }

            args
        }

        fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
            if self.kind.map_or(true, |kind| kind == info.kind) &&
                self.regex_set.matches(info.name)
            {
                return self.derives.clone();
            }
            vec![]
        }
    }

    #[derive(Debug)]
    struct CustomAttributeCallback {
        attributes: Vec<String>,
        kind: Option<TypeKind>,
        regex_set: RegexSet,
    }

    impl ParseCallbacks for CustomAttributeCallback {
        fn cli_args(&self) -> Vec<String> {
            let mut args = vec![];

            let flag = match &self.kind {
                None => "--with-attribute-custom",
                Some(TypeKind::Struct) => "--with-attribute-custom-struct",
                Some(TypeKind::Enum) => "--with-attribute-custom-enum",
                Some(TypeKind::Union) => "--with-attribute-custom-union",
            };

            let attributes = self.attributes.join(",");

            for item in self.regex_set.get_items() {
                args.extend_from_slice(&[
                    flag.to_owned(),
                    format!("{item}={attributes}"),
                ]);
            }

            args
        }

        fn add_attributes(&self, info: &AttributeInfo<'_>) -> Vec<String> {
            if self.kind.map_or(true, |kind| kind == info.kind) &&
                self.regex_set.matches(info.name)
            {
                return self.attributes.clone();
            }
            vec![]
        }
    }

    /// Macro used to apply CLI arguments to a builder.
    ///
    /// This is done by passing an identifier for each argument and a function to be applied over
    /// the builder. For example:
    /// ```rust,ignore
    /// fn apply_arg(builder: Builder, arg_value: Value) -> Builder {
    ///     todo!()
    /// }
    ///
    /// apply_args!(
    ///     builder {
    ///         arg => apply_arg,
    ///     }
    /// );
    /// ```
    ///
    /// If the identifier of the argument is the same as an already existing builder method then
    /// you can omit the second part:
    /// ```rust,ignore
    /// apply_args!(
    ///     builder {
    ///         arg
    ///     }
    /// );
    /// ```
    /// Which expands to the same code as:
    /// ```rust,ignore
    /// apply_args!(
    ///     builder {
    ///         arg => Builder::arg,
    ///     }
    /// );
    /// ```
    macro_rules! apply_args {
        ($builder:ident {}) => { $builder };
        ($builder:ident {$arg:ident => $function:expr, $($token:tt)*}) => {
            {
                $builder = CliArg::apply($arg, $builder, $function);
                apply_args!($builder {$($token)*})
            }
        };
        ($builder:ident {$arg:ident, $($token:tt)*}) => {
            {
                $builder = CliArg::apply($arg, $builder, Builder::$arg);
                apply_args!($builder {$($token)*})
            }
        }
    }

    builder = apply_args!(
        builder {
            header,
            rust_target,
            rust_edition,
            default_enum_style,
            bitfield_enum,
            newtype_enum,
            newtype_global_enum,
            rustified_enum,
            rustified_non_exhaustive_enum,
            constified_enum,
            constified_enum_module,
            default_macro_constant_type,
            default_alias_style,
            normal_alias => Builder::type_alias,
            new_type_alias,
            new_type_alias_deref,
            default_non_copy_union_style,
            bindgen_wrapper_union,
            manually_drop_union,
            blocklist_type,
            blocklist_function,
            blocklist_item,
            blocklist_file,
            blocklist_var,
            builtins => |b, _| b.emit_builtins(),
            no_layout_tests => |b, _| b.layout_tests(false),
            no_derive_copy => |b, _| b.derive_copy(false),
            no_derive_debug => |b, _| b.derive_debug(false),
            impl_debug,
            impl_partialeq,
            with_derive_default => Builder::derive_default,
            with_derive_hash => Builder::derive_hash,
            with_derive_partialeq => Builder::derive_partialeq,
            with_derive_partialord => Builder::derive_partialord,
            with_derive_eq => Builder::derive_eq,
            with_derive_ord => Builder::derive_ord,
            no_derive_default => |b, _| b.derive_default(false),
            no_prepend_enum_name => |b, _| b.prepend_enum_name(false),
            no_include_path_detection => |b, _| b.detect_include_paths(false),
            fit_macro_constant_types => Builder::fit_macro_constants,
            time_phases,
            use_array_pointers_in_arguments => Builder::array_pointers_in_arguments,
            wasm_import_module_name,
            ctypes_prefix,
            anon_fields_prefix,
            generate => Builder::with_codegen_config,
            emit_clang_ast => |b, _| b.emit_clang_ast(),
            emit_ir => |b, _| b.emit_ir(),
            emit_ir_graphviz,
            enable_cxx_namespaces => |b, _| b.enable_cxx_namespaces(),
            enable_function_attribute_detection => |b, _| b.enable_function_attribute_detection(),
            disable_name_namespacing => |b, _| b.disable_name_namespacing(),
            disable_nested_struct_naming => |b, _| b.disable_nested_struct_naming(),
            disable_untagged_union => |b, _| b.disable_untagged_union(),
            disable_header_comment => |b, _| b.disable_header_comment(),
            ignore_functions => |b, _| b.ignore_functions(),
            ignore_methods => |b, _| b.ignore_methods(),
            no_convert_floats => |b, _| b.no_convert_floats(),
            no_doc_comments => |b, _| b.generate_comments(false),
            no_recursive_allowlist => |b, _| b.allowlist_recursively(false),
            objc_extern_crate,
            generate_block,
            generate_cstr,
            block_extern_crate,
            opaque_type,
            raw_line,
            use_core => |b, _| b.use_core(),
            distrust_clang_mangling => |b, _| b.trust_clang_mangling(false),
            conservative_inline_namespaces => |b, _| b.conservative_inline_namespaces(),
            generate_inline_functions,
            allowlist_function,
            allowlist_type,
            allowlist_var,
            allowlist_file,
            allowlist_item,
            clang_args => Builder::clang_arg,
            no_record_matches => |b, _| b.record_matches(false),
            no_size_t_is_usize => |b, _| b.size_t_is_usize(false),
            no_rustfmt_bindings => |b, _| b.formatter(Formatter::None),
            formatter,
            no_partialeq,
            no_copy,
            no_debug,
            no_default,
            no_hash,
            must_use_type,
            dynamic_loading => Builder::dynamic_library_name,
            dynamic_link_require_all,
            prefix_link_name => |b, prefix| b.parse_callbacks(Box::new(PrefixLinkNameCallback { prefix })),
            respect_cxx_access_specs,
            translate_enum_integer_types,
            c_naming,
            explicit_padding,
            vtable_generation,
            sort_semantically,
            merge_extern_blocks,
            override_abi => |b, (abi, regex)| b.override_abi(abi, regex),
            wrap_unsafe_ops,
            clang_macro_fallback => |b, _| b.clang_macro_fallback(),
            clang_macro_fallback_build_dir,
            flexarray_dst,
            wrap_static_fns,
            wrap_static_fns_path,
            wrap_static_fns_suffix,
            default_visibility,
        }
    );

    let mut values = module_raw_line.into_iter();
    while let Some(module) = values.next() {
        let line = values.next().unwrap();
        builder = builder.module_raw_line(module, line);
    }

    let output = if let Some(path) = &output {
        let file = File::create(path)?;
        if let Some(depfile) = depfile {
            builder = builder.depfile(path, depfile);
        }
        Box::new(io::BufWriter::new(file)) as Box<dyn io::Write>
    } else {
        if let Some(depfile) = depfile {
            builder = builder.depfile("-", depfile);
        }
        Box::new(io::BufWriter::new(io::stdout())) as Box<dyn io::Write>
    };

    if dump_preprocessed_input {
        builder.dump_preprocessed_input()?;
    }

    if let Some(path) = rustfmt_configuration_file {
        builder = builder.rustfmt_configuration_file(Some(path));
    }

    for (custom_derives, kind, _name) in [
        (with_derive_custom, None, "--with-derive-custom"),
        (
            with_derive_custom_struct,
            Some(TypeKind::Struct),
            "--with-derive-custom-struct",
        ),
        (
            with_derive_custom_enum,
            Some(TypeKind::Enum),
            "--with-derive-custom-enum",
        ),
        (
            with_derive_custom_union,
            Some(TypeKind::Union),
            "--with-derive-custom-union",
        ),
    ] {
        #[cfg(feature = "experimental")]
        let name = emit_diagnostics.then_some(_name);

        for (derives, regex) in custom_derives {
            let mut regex_set = RegexSet::default();
            regex_set.insert(regex);

            #[cfg(feature = "experimental")]
            regex_set.build_with_diagnostics(false, name);
            #[cfg(not(feature = "experimental"))]
            regex_set.build(false);

            builder = builder.parse_callbacks(Box::new(CustomDeriveCallback {
                derives,
                kind,
                regex_set,
            }));
        }
    }

    for (custom_attributes, kind, _name) in [
        (with_attribute_custom, None, "--with-attribute-custom"),
        (
            with_attribute_custom_struct,
            Some(TypeKind::Struct),
            "--with-attribute-custom-struct",
        ),
        (
            with_attribute_custom_enum,
            Some(TypeKind::Enum),
            "--with-attribute-custom-enum",
        ),
        (
            with_attribute_custom_union,
            Some(TypeKind::Union),
            "--with-attribute-custom-union",
        ),
    ] {
        #[cfg(feature = "experimental")]
        let name = emit_diagnostics.then_some(_name);

        for (attributes, regex) in custom_attributes {
            let mut regex_set = RegexSet::default();
            regex_set.insert(regex);

            #[cfg(feature = "experimental")]
            regex_set.build_with_diagnostics(false, name);
            #[cfg(not(feature = "experimental"))]
            regex_set.build(false);

            builder =
                builder.parse_callbacks(Box::new(CustomAttributeCallback {
                    attributes,
                    kind,
                    regex_set,
                }));
        }
    }

    #[cfg(feature = "experimental")]
    if emit_diagnostics {
        builder = builder.emit_diagnostics();
    }

    Ok((builder, output, verbose))
}

/// Trait for CLI arguments that can be applied to a [`Builder`].
trait CliArg {
    /// The value of this argument.
    type Value;

    /// Apply the current argument to the passed [`Builder`].
    fn apply(
        self,
        builder: Builder,
        f: impl Fn(Builder, Self::Value) -> Builder,
    ) -> Builder;
}

/// Boolean arguments are applied when they evaluate to `true`.
impl CliArg for bool {
    type Value = bool;

    fn apply(
        self,
        mut builder: Builder,
        f: impl Fn(Builder, Self::Value) -> Builder,
    ) -> Builder {
        if self {
            builder = f(builder, self);
        }

        builder
    }
}

/// Optional arguments are applied when they are `Some`.
impl<T> CliArg for Option<T> {
    type Value = T;

    fn apply(
        self,
        mut builder: Builder,
        f: impl Fn(Builder, Self::Value) -> Builder,
    ) -> Builder {
        if let Some(value) = self {
            builder = f(builder, value);
        }

        builder
    }
}

/// Multiple valued arguments are applied once for each value.
impl<T> CliArg for Vec<T> {
    type Value = T;

    fn apply(
        self,
        mut builder: Builder,
        f: impl Fn(Builder, Self::Value) -> Builder,
    ) -> Builder {
        for value in self {
            builder = f(builder, value);
        }

        builder
    }
}

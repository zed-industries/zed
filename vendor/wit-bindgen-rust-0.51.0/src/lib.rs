use crate::interface::InterfaceGenerator;
use anyhow::{Result, bail};
use core::panic;
use heck::*;
use indexmap::{IndexMap, IndexSet};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{self, Write as _};
use std::mem;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use wit_bindgen_core::abi::{Bitcast, WasmType};
use wit_bindgen_core::{
    AsyncFilterSet, Files, InterfaceGenerator as _, Source, Types, WorldGenerator, dealias,
    name_package_module, uwrite, uwriteln, wit_parser::*,
};

mod bindgen;
mod interface;

struct InterfaceName {
    /// True when this interface name has been remapped through the use of `with` in the `bindgen!`
    /// macro invocation.
    remapped: bool,

    /// The string name for this interface.
    path: String,
}

#[derive(Default)]
pub struct RustWasm {
    types: Types,
    src_preamble: Source,
    src: Source,
    opts: Opts,
    import_modules: Vec<(String, Vec<String>)>,
    export_modules: Vec<(String, Vec<String>)>,
    skip: HashSet<String>,
    interface_names: HashMap<InterfaceId, InterfaceName>,
    /// Each imported and exported interface is stored in this map. Value indicates if last use was import.
    interface_last_seen_as_import: HashMap<InterfaceId, bool>,
    import_funcs_called: bool,
    with_name_counter: usize,
    // Track which interfaces and types are generated. Remapped interfaces and types provided via `with`
    // are required to be used.
    generated_types: HashSet<String>,
    world: Option<WorldId>,

    rt_module: IndexSet<RuntimeItem>,
    export_macros: Vec<(String, String)>,

    /// Maps wit interface and type names to their Rust identifiers
    with: GenerationConfiguration,

    future_payloads: IndexMap<String, String>,
    stream_payloads: IndexMap<String, String>,
}

#[derive(Default)]
struct GenerationConfiguration {
    map: HashMap<String, TypeGeneration>,
    generate_by_default: bool,
}

impl GenerationConfiguration {
    fn get(&self, key: &str) -> Option<&TypeGeneration> {
        self.map.get(key).or_else(|| {
            self.generate_by_default
                .then_some(&TypeGeneration::Generate)
        })
    }

    fn insert(&mut self, name: String, generate: TypeGeneration) {
        self.map.insert(name, generate);
    }

    fn iter(&self) -> impl Iterator<Item = (&String, &TypeGeneration)> {
        self.map.iter()
    }
}

/// How a wit interface or type should be rendered in Rust
enum TypeGeneration {
    /// Uses a Rust identifier defined elsewhere
    Remap(String),
    /// Define the interface or type with this bindgen invocation
    Generate,
}

impl TypeGeneration {
    /// Returns true if the interface or type should be defined with this bindgen invocation
    fn generated(&self) -> bool {
        match self {
            TypeGeneration::Generate => true,
            TypeGeneration::Remap(_) => false,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum RuntimeItem {
    AllocCrate,
    StringType,
    StdAllocModule,
    VecType,
    StringLift,
    InvalidEnumDiscriminant,
    CharLift,
    BoolLift,
    CabiDealloc,
    RunCtorsOnce,
    AsI32,
    AsI64,
    AsF32,
    AsF64,
    ResourceType,
    BoxType,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum ExportKey {
    World,
    Name(String),
}

#[cfg(feature = "clap")]
fn parse_with(s: &str) -> Result<(String, WithOption), String> {
    let (k, v) = s.split_once('=').ok_or_else(|| {
        format!("expected string of form `<key>=<value>[,<key>=<value>...]`; got `{s}`")
    })?;
    let v = match v {
        "generate" => WithOption::Generate,
        other => WithOption::Path(other.to_string()),
    };
    Ok((k.to_string(), v))
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, rename_all = "kebab-case")
)]
pub struct Opts {
    /// Whether or not a formatter is executed to format generated code.
    #[cfg_attr(feature = "clap", arg(long))]
    pub format: bool,

    /// If true, code generation should qualify any features that depend on
    /// `std` with `cfg(feature = "std")`.
    #[cfg_attr(feature = "clap", arg(long))]
    pub std_feature: bool,

    /// If true, code generation should pass borrowed string arguments as
    /// `&[u8]` instead of `&str`. Strings are still required to be valid
    /// UTF-8, but this avoids the need for Rust code to do its own UTF-8
    /// validation if it doesn't already have a `&str`.
    #[cfg_attr(feature = "clap", arg(long))]
    pub raw_strings: bool,

    /// Names of functions to skip generating bindings for.
    #[cfg_attr(feature = "clap", arg(long, value_name = "NAME"))]
    pub skip: Vec<String>,

    /// If true, generate stub implementations for any exported functions,
    /// interfaces, and/or resources.
    #[cfg_attr(feature = "clap", arg(long))]
    pub stubs: bool,

    /// Optionally prefix any export names with the specified value.
    ///
    /// This is useful to avoid name conflicts when testing.
    #[cfg_attr(feature = "clap", arg(long, value_name = "STRING"))]
    pub export_prefix: Option<String>,

    /// Whether to generate owning or borrowing type definitions.
    ///
    /// Valid values include:
    ///
    /// - `owning`: Generated types will be composed entirely of owning fields,
    /// regardless of whether they are used as parameters to imports or not.
    ///
    /// - `borrowing`: Generated types used as parameters to imports will be
    /// "deeply borrowing", i.e. contain references rather than owned values
    /// when applicable.
    ///
    /// - `borrowing-duplicate-if-necessary`: As above, but generating distinct
    /// types for borrowing and owning, if necessary.
    #[cfg_attr(feature = "clap", arg(long, default_value_t = Ownership::Owning))]
    pub ownership: Ownership,

    /// The optional path to the wit-bindgen runtime module to use.
    ///
    /// This defaults to `wit_bindgen::rt`.
    #[cfg_attr(feature = "clap", arg(long, value_name = "PATH"))]
    pub runtime_path: Option<String>,

    /// The optional path to the bitflags crate to use.
    ///
    /// This defaults to `wit_bindgen::bitflags`.
    #[cfg_attr(feature = "clap", arg(long))]
    pub bitflags_path: Option<String>,

    /// Additional derive attributes to add to generated types. If using in a CLI, this flag can be
    /// specified multiple times to add multiple attributes.
    ///
    /// These derive attributes will be added to any generated structs or enums
    #[cfg_attr(feature = "clap", arg(long, short = 'd', value_name = "DERIVE"))]
    pub additional_derive_attributes: Vec<String>,

    /// Variants and records to ignore when applying additional derive attributes.
    ///
    /// These names are specified as they are listed in the wit file, i.e. in kebab case.
    /// This feature allows some variants and records to use types for which adding traits will cause
    /// compilation to fail, such as serde::Deserialize on wasi:io/streams.
    ///
    #[cfg_attr(feature = "clap", arg(long, value_name = "NAME"))]
    pub additional_derive_ignore: Vec<String>,

    /// Remapping of wit import interface and type names to Rust module names
    /// and types.
    ///
    /// Argument must be of the form `k=v` and this option can be passed
    /// multiple times or one option can be comma separated, for example
    /// `k1=v1,k2=v2`.
    #[cfg_attr(feature = "clap", arg(long, value_parser = parse_with, value_delimiter = ','))]
    pub with: Vec<(String, WithOption)>,

    /// Indicates that all interfaces not specified in `with` should be
    /// generated.
    #[cfg_attr(feature = "clap", arg(long))]
    pub generate_all: bool,

    /// Add the specified suffix to the name of the custome section containing
    /// the component type.
    #[cfg_attr(feature = "clap", arg(long, value_name = "STRING"))]
    pub type_section_suffix: Option<String>,

    /// Disable a workaround used to prevent libc ctors/dtors from being invoked
    /// too much.
    #[cfg_attr(feature = "clap", arg(long))]
    pub disable_run_ctors_once_workaround: bool,

    /// Changes the default module used in the generated `export!` macro to
    /// something other than `self`.
    #[cfg_attr(feature = "clap", arg(long, value_name = "NAME"))]
    pub default_bindings_module: Option<String>,

    /// Alternative name to use for the `export!` macro if one is generated.
    #[cfg_attr(feature = "clap", arg(long, value_name = "NAME"))]
    pub export_macro_name: Option<String>,

    /// Ensures that the `export!` macro will be defined as `pub` so it is a
    /// candidate for being exported outside of the crate.
    #[cfg_attr(feature = "clap", arg(long))]
    pub pub_export_macro: bool,

    /// Whether to generate unused structures, not generated by default (false)
    #[cfg_attr(feature = "clap", arg(long))]
    pub generate_unused_types: bool,

    /// Whether or not to generate helper function/constants to help link custom
    /// sections into the final output.
    ///
    /// Disabling this can shave a few bytes off a binary but makes
    /// library-based usage of `generate!` prone to breakage.
    #[cfg_attr(feature = "clap", arg(long))]
    pub disable_custom_section_link_helpers: bool,

    #[cfg_attr(feature = "clap", clap(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub async_: AsyncFilterSet,
}

impl Opts {
    pub fn build(self) -> RustWasm {
        let mut r = RustWasm::new();
        r.skip = self.skip.iter().cloned().collect();
        r.opts = self;
        r
    }
}

impl RustWasm {
    /// Generates Rust bindings from the `wit/` directory and writes
    /// the result into Cargo’s `OUT_DIR`. Intended for use in `build.rs`.
    ///
    /// The `world` parameter specifies the world name to select.
    /// It must be provided unless the main package contains exactly one world.
    ///
    /// Returns the full path to the generated bindings file.
    pub fn generate_to_out_dir(mut self, world: Option<&str>) -> Result<PathBuf> {
        let mut resolve = Resolve::default();
        println!("cargo:rerun-if-changed=wit/");
        let (pkg, _files) = resolve.push_path("wit")?;
        let main_packages = vec![pkg];
        let world = resolve.select_world(&main_packages, world)?;

        let mut files = Files::default();
        self.generate(&resolve, world, &mut files)?;
        let out_dir = std::env::var("OUT_DIR").expect("cargo sets OUT_DIR");
        let (name, contents) = files
            .iter()
            .next()
            .expect("exactly one file should be generated");
        let dst = Path::new(&out_dir).join(name);
        std::fs::write(&dst, contents)?;
        Ok(dst)
    }

    fn new() -> RustWasm {
        RustWasm::default()
    }

    fn interface<'a>(
        &'a mut self,
        identifier: Identifier<'a>,
        wasm_import_module: &'a str,
        resolve: &'a Resolve,
        in_import: bool,
    ) -> InterfaceGenerator<'a> {
        let mut sizes = SizeAlign::default();
        sizes.fill(resolve);

        InterfaceGenerator {
            identifier,
            wasm_import_module,
            src: Source::default(),
            in_import,
            r#gen: self,
            sizes,
            resolve,
            return_pointer_area_size: Default::default(),
            return_pointer_area_align: Default::default(),
            needs_runtime_module: false,
        }
    }

    fn emit_modules(&mut self, modules: Vec<(String, Vec<String>)>) {
        #[derive(Default)]
        struct Module {
            submodules: BTreeMap<String, Module>,
            contents: Vec<String>,
        }
        let mut map = Module::default();
        for (module, path) in modules {
            let mut cur = &mut map;
            for name in path[..path.len() - 1].iter() {
                cur = cur
                    .submodules
                    .entry(name.clone())
                    .or_insert(Module::default());
            }
            cur.contents.push(module);
        }

        emit(&mut self.src, map, &self.opts, true);
        fn emit(me: &mut Source, module: Module, opts: &Opts, toplevel: bool) {
            for (name, submodule) in module.submodules {
                if toplevel {
                    // Disable rustfmt. By default we already format the code
                    // using prettyplease, so we don't want `cargo fmt` to create
                    // extra diffs for users to deal with.
                    if opts.format {
                        uwriteln!(me, "#[rustfmt::skip]");
                    }

                    // Ignore dead-code and clippy warnings. If the bindings are
                    // only used within a crate, and not exported to a different
                    // crate, some parts may be unused, and that's ok.
                    uwriteln!(me, "#[allow(dead_code, clippy::all)]");
                }

                uwriteln!(me, "pub mod {name} {{");
                emit(me, submodule, opts, false);
                uwriteln!(me, "}}");
            }
            for submodule in module.contents {
                uwriteln!(me, "{submodule}");
            }
        }
    }

    fn runtime_path(&self) -> &str {
        self.opts
            .runtime_path
            .as_deref()
            .unwrap_or("wit_bindgen::rt")
    }

    fn bitflags_path(&self) -> String {
        self.opts
            .bitflags_path
            .to_owned()
            .unwrap_or(format!("{}::bitflags", self.runtime_path()))
    }

    fn async_support_path(&self) -> String {
        format!("{}::async_support", self.runtime_path())
    }

    fn name_interface(
        &mut self,
        resolve: &Resolve,
        id: InterfaceId,
        name: &WorldKey,
        is_export: bool,
    ) -> Result<bool> {
        let with_name = resolve.name_world_key(name);
        let remapping = if is_export {
            &TypeGeneration::Generate
        } else {
            match self.with.get(&with_name) {
                Some(remapping) => remapping,
                None => bail!(MissingWith(with_name)),
            }
        };
        self.generated_types.insert(with_name);
        let entry = match remapping {
            TypeGeneration::Remap(remapped_path) => {
                let name = format!("__with_name{}", self.with_name_counter);
                self.with_name_counter += 1;
                uwriteln!(
                    self.src,
                    "#[allow(unfulfilled_lint_expectations, unused_imports)]"
                );
                uwriteln!(self.src, "use {remapped_path} as {name};");
                InterfaceName {
                    remapped: true,
                    path: name,
                }
            }
            TypeGeneration::Generate => {
                let path = compute_module_path(name, resolve, is_export).join("::");

                InterfaceName {
                    remapped: false,
                    path,
                }
            }
        };

        let remapped = entry.remapped;
        self.interface_names.insert(id, entry);

        Ok(remapped)
    }

    fn finish_runtime_module(&mut self) {
        if !self.rt_module.is_empty() {
            // As above, disable rustfmt, as we use prettyplease.
            if self.opts.format {
                uwriteln!(self.src, "#[rustfmt::skip]");
            }

            self.src.push_str("mod _rt {\n");
            self.src
                .push_str("#![allow(dead_code, unused_imports, clippy::all)]\n");
            let mut emitted = IndexSet::new();
            while !self.rt_module.is_empty() {
                for item in mem::take(&mut self.rt_module) {
                    if emitted.insert(item) {
                        self.emit_runtime_item(item);
                    }
                }
            }
            self.src.push_str("}\n");
        }

        if !self.future_payloads.is_empty() {
            let async_support = self.async_support_path();
            self.src.push_str(&format!(
                "\
pub mod wit_future {{
    #![allow(dead_code, unused_variables, clippy::all)]

    #[doc(hidden)]
    pub trait FuturePayload: Unpin + Sized + 'static {{
        const VTABLE: &'static {async_support}::FutureVtable<Self>;
    }}"
            ));
            for code in self.future_payloads.values() {
                self.src.push_str(code);
            }
            self.src.push_str(&format!(
                "\
    /// Creates a new Component Model `future` with the specified payload type.
    ///
    /// The `default` function provided computes the default value to be sent in
    /// this future if no other value was otherwise sent.
    pub fn new<T: FuturePayload>(default: fn() -> T) -> ({async_support}::FutureWriter<T>, {async_support}::FutureReader<T>) {{
        unsafe {{ {async_support}::future_new::<T>(default, T::VTABLE) }}
    }}
}}
                ",
            ));
        }

        if !self.stream_payloads.is_empty() {
            let async_support = self.async_support_path();
            self.src.push_str(&format!(
                "\
pub mod wit_stream {{
    #![allow(dead_code, unused_variables, clippy::all)]

    pub trait StreamPayload: Unpin + Sized + 'static {{
        const VTABLE: &'static {async_support}::StreamVtable<Self>;
    }}"
            ));
            for code in self.stream_payloads.values() {
                self.src.push_str(code);
            }
            self.src.push_str(
                &format!("\
    /// Creates a new Component Model `stream` with the specified payload type.
    pub fn new<T: StreamPayload>() -> ({async_support}::StreamWriter<T>, {async_support}::StreamReader<T>) {{
        unsafe {{ {async_support}::stream_new::<T>(T::VTABLE) }}
    }}
}}
                "),
            );
        }
    }

    fn emit_runtime_item(&mut self, item: RuntimeItem) {
        match item {
            RuntimeItem::AllocCrate => {
                uwriteln!(self.src, "extern crate alloc as alloc_crate;");
            }
            RuntimeItem::StdAllocModule => {
                self.rt_module.insert(RuntimeItem::AllocCrate);
                uwriteln!(self.src, "pub use alloc_crate::alloc;");
            }
            RuntimeItem::StringType => {
                self.rt_module.insert(RuntimeItem::AllocCrate);
                uwriteln!(self.src, "pub use alloc_crate::string::String;");
            }
            RuntimeItem::BoxType => {
                self.rt_module.insert(RuntimeItem::AllocCrate);
                uwriteln!(self.src, "pub use alloc_crate::boxed::Box;");
            }
            RuntimeItem::VecType => {
                self.rt_module.insert(RuntimeItem::AllocCrate);
                uwriteln!(self.src, "pub use alloc_crate::vec::Vec;");
            }
            RuntimeItem::CabiDealloc => {
                self.rt_module.insert(RuntimeItem::StdAllocModule);
                self.src.push_str(
                    "\
pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
    if size == 0 {
        return;
    }
    unsafe {
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
}
                    ",
                );
            }

            RuntimeItem::StringLift => {
                self.rt_module.insert(RuntimeItem::StringType);
                self.src.push_str(
                    "\
pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
    if cfg!(debug_assertions) {
        String::from_utf8(bytes).unwrap()
    } else {
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}
                    ",
                );
            }

            RuntimeItem::InvalidEnumDiscriminant => {
                self.src.push_str(
                    "\
pub unsafe fn invalid_enum_discriminant<T>() -> T {
    if cfg!(debug_assertions) {
        panic!(\"invalid enum discriminant\")
    } else {
        unsafe { core::hint::unreachable_unchecked() }
    }
}
                    ",
                );
            }

            RuntimeItem::CharLift => {
                self.src.push_str(
                    "\
pub unsafe fn char_lift(val: u32) -> char {
    if cfg!(debug_assertions) {
        core::char::from_u32(val).unwrap()
    } else {
        unsafe { core::char::from_u32_unchecked(val) }
    }
}
                    ",
                );
            }

            RuntimeItem::BoolLift => {
                self.src.push_str(
                    "\
pub unsafe fn bool_lift(val: u8) -> bool {
    if cfg!(debug_assertions) {
        match val {
            0 => false,
            1 => true,
            _ => panic!(\"invalid bool discriminant\"),
        }
    } else {
        val != 0
    }
}
                    ",
                );
            }

            RuntimeItem::RunCtorsOnce => {
                let rt = self.runtime_path();
                self.src.push_str(&format!(
                    r#"
#[cfg(target_arch = "wasm32")]
pub fn run_ctors_once() {{
    {rt}::run_ctors_once();
}}
                    "#,
                ));
            }

            RuntimeItem::AsI32 => {
                self.emit_runtime_as_trait(
                    "i32",
                    &["i32", "u32", "i16", "u16", "i8", "u8", "char", "usize"],
                );
            }

            RuntimeItem::AsI64 => {
                self.emit_runtime_as_trait("i64", &["i64", "u64"]);
            }

            RuntimeItem::AsF32 => {
                self.emit_runtime_as_trait("f32", &["f32"]);
            }

            RuntimeItem::AsF64 => {
                self.emit_runtime_as_trait("f64", &["f64"]);
            }

            RuntimeItem::ResourceType => {
                self.src.push_str(
                    r#"

use core::fmt;
use core::marker;
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

/// A type which represents a component model resource, either imported or
/// exported into this component.
///
/// This is a low-level wrapper which handles the lifetime of the resource
/// (namely this has a destructor). The `T` provided defines the component model
/// intrinsics that this wrapper uses.
///
/// One of the chief purposes of this type is to provide `Deref` implementations
/// to access the underlying data when it is owned.
///
/// This type is primarily used in generated code for exported and imported
/// resources.
#[repr(transparent)]
pub struct Resource<T: WasmResource> {
    // NB: This would ideally be `u32` but it is not. The fact that this has
    // interior mutability is not exposed in the API of this type except for the
    // `take_handle` method which is supposed to in theory be private.
    //
    // This represents, almost all the time, a valid handle value. When it's
    // invalid it's stored as `u32::MAX`.
    handle: AtomicU32,
    _marker: marker::PhantomData<T>,
}

/// A trait which all wasm resources implement, namely providing the ability to
/// drop a resource.
///
/// This generally is implemented by generated code, not user-facing code.
#[allow(clippy::missing_safety_doc)]
pub unsafe trait WasmResource {
    /// Invokes the `[resource-drop]...` intrinsic.
    unsafe fn drop(handle: u32);
}

impl<T: WasmResource> Resource<T> {
    #[doc(hidden)]
    pub unsafe fn from_handle(handle: u32) -> Self {
        debug_assert!(handle != 0 && handle != u32::MAX);
        Self {
            handle: AtomicU32::new(handle),
            _marker: marker::PhantomData,
        }
    }

    /// Takes ownership of the handle owned by `resource`.
    ///
    /// Note that this ideally would be `into_handle` taking `Resource<T>` by
    /// ownership. The code generator does not enable that in all situations,
    /// unfortunately, so this is provided instead.
    ///
    /// Also note that `take_handle` is in theory only ever called on values
    /// owned by a generated function. For example a generated function might
    /// take `Resource<T>` as an argument but then call `take_handle` on a
    /// reference to that argument. In that sense the dynamic nature of
    /// `take_handle` should only be exposed internally to generated code, not
    /// to user code.
    #[doc(hidden)]
    pub fn take_handle(resource: &Resource<T>) -> u32 {
        resource.handle.swap(u32::MAX, Relaxed)
    }

    #[doc(hidden)]
    pub fn handle(resource: &Resource<T>) -> u32 {
        resource.handle.load(Relaxed)
    }
}

impl<T: WasmResource> fmt::Debug for Resource<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resource")
            .field("handle", &self.handle)
            .finish()
    }
}

impl<T: WasmResource> Drop for Resource<T> {
    fn drop(&mut self) {
        unsafe {
            match self.handle.load(Relaxed) {
                // If this handle was "taken" then don't do anything in the
                // destructor.
                u32::MAX => {}

                // ... but otherwise do actually destroy it with the imported
                // component model intrinsic as defined through `T`.
                other => T::drop(other),
            }
        }
    }
}
                    "#,
                );
            }
        }
    }

    // This is a workaround for in the bindings sometimes we've got `&i32` and
    // sometimes we've got `i32` but that should all be able to be worked with
    // as `i32`, so these helper functions are used to boil away the
    // indirection.
    fn emit_runtime_as_trait(&mut self, ty: &str, to_convert: &[&str]) {
        let upcase = ty.to_uppercase();
        self.src.push_str(&format!(
            r#"
pub fn as_{ty}<T: As{upcase}>(t: T) -> {ty} {{
    t.as_{ty}()
}}

pub trait As{upcase} {{
    fn as_{ty}(self) -> {ty};
}}

impl<'a, T: Copy + As{upcase}> As{upcase} for &'a T {{
    fn as_{ty}(self) -> {ty} {{
        (*self).as_{ty}()
    }}
}}
            "#
        ));

        for to_convert in to_convert {
            self.src.push_str(&format!(
                r#"
impl As{upcase} for {to_convert} {{
    #[inline]
    fn as_{ty}(self) -> {ty} {{
        self as {ty}
    }}
}}
                "#
            ));
        }
    }

    /// Generates an `export!` macro for the `world_id` specified.
    ///
    /// This will generate a macro which will then itself invoke all the
    /// other macros collected in `self.export_macros` prior. All these macros
    /// are woven together in this single invocation.
    fn finish_export_macro(&mut self, resolve: &Resolve, world_id: WorldId) {
        if self.export_macros.is_empty() {
            return;
        }
        let world = &resolve.worlds[world_id];
        let world_name = world.name.to_snake_case();

        let default_bindings_module = self
            .opts
            .default_bindings_module
            .clone()
            .unwrap_or("self".to_string());
        let (macro_export, use_vis) = if self.opts.pub_export_macro {
            ("#[macro_export]", "pub")
        } else {
            ("", "pub(crate)")
        };
        let export_macro_name = self
            .opts
            .export_macro_name
            .as_deref()
            .unwrap_or("export")
            .to_string();
        uwriteln!(
            self.src,
            r#"
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! {export_macro_name} {{ ($($t:tt)*) => (); }}
/// # trait Guest {{}}
/// struct MyType;
///
/// impl Guest for MyType {{
///     // ...
/// }}
///
/// {export_macro_name}!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
{macro_export}
macro_rules! __export_{world_name}_impl {{
    ($ty:ident) => ({default_bindings_module}::{export_macro_name}!($ty with_types_in {default_bindings_module}););
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => ("#
        );
        for (name, path_to_types) in self.export_macros.iter() {
            let mut path = "$($path_to_types_root)*".to_string();
            if !path_to_types.is_empty() {
                path.push_str("::");
                path.push_str(path_to_types)
            }
            uwriteln!(self.src, "{path}::{name}!($ty with_types_in {path});");
        }

        // See comments in `finish` for why this conditionally happens here.
        if self.opts.pub_export_macro {
            uwriteln!(self.src, "const _: () = {{");
            self.emit_custom_section(resolve, world_id, "imports and exports", None);
            uwriteln!(self.src, "}};");
        }

        uwriteln!(self.src, ")\n}}");

        uwriteln!(
            self.src,
            "#[doc(inline)]\n\
            {use_vis} use __export_{world_name}_impl as {export_macro_name};"
        );

        if self.opts.stubs {
            uwriteln!(self.src, "export!(Stub);");
        }
    }

    /// Generates a `#[link_section]` custom section to get smuggled through
    /// `wasm-ld`.
    ///
    /// This custom section is an encoding of the component metadata and will be
    /// used as part of the `wit-component`-based componentization process.
    ///
    /// The `section_suffix` here is used to distinguish the multiple sections
    /// that this generator emits, and `func_name` is an optional function to
    /// generate next to this which is used to force rustc to at least visit
    /// this `static` and codegen it.
    fn emit_custom_section(
        &mut self,
        resolve: &Resolve,
        world_id: WorldId,
        section_suffix: &str,
        func_name: Option<&str>,
    ) {
        // As above, disable rustfmt, as we use prettyplease.
        if self.opts.format {
            uwriteln!(self.src, "#[rustfmt::skip]");
        }
        self.src.push_str("\n#[cfg(target_arch = \"wasm32\")]\n");

        // The custom section name here must start with "component-type" but
        // otherwise is attempted to be unique here to ensure that this doesn't get
        // concatenated to other custom sections by LLD by accident since LLD will
        // concatenate custom sections of the same name.
        let opts_suffix = self.opts.type_section_suffix.as_deref().unwrap_or("");
        let world = &resolve.worlds[world_id];
        let world_name = &world.name;
        let pkg = &resolve.packages[world.package.unwrap()].name;
        let version = env!("CARGO_PKG_VERSION");
        self.src.push_str(&format!(
            "#[unsafe(link_section = \"component-type:wit-bindgen:{version}:\
             {pkg}:{world_name}:{section_suffix}{opts_suffix}\")]\n"
        ));

        let mut producers = wasm_metadata::Producers::empty();
        producers.add(
            "processed-by",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        );

        let component_type = wit_component::metadata::encode(
            resolve,
            world_id,
            wit_component::StringEncoding::UTF8,
            Some(&producers),
        )
        .unwrap();

        self.src.push_str("#[doc(hidden)]\n");
        self.src.push_str("#[allow(clippy::octal_escapes)]\n");
        self.src.push_str(&format!(
            "pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; {}] = *b\"\\\n",
            component_type.len()
        ));
        let old_indent = self.src.set_indent(0);
        let mut line_length = 0;
        let s = self.src.as_mut_string();
        for byte in component_type.iter() {
            if line_length >= 80 {
                s.push_str("\\\n");
                line_length = 0;
            }
            match byte {
                b'\\' => {
                    s.push_str("\\\\");
                    line_length += 2;
                }
                b'"' => {
                    s.push_str("\\\"");
                    line_length += 2;
                }
                b if b.is_ascii_alphanumeric() || b.is_ascii_punctuation() => {
                    s.push(char::from(*byte));
                    line_length += 1;
                }
                0 => {
                    s.push_str("\\0");
                    line_length += 2;
                }
                _ => {
                    uwrite!(s, "\\x{:02x}", byte);
                    line_length += 4;
                }
            }
        }

        self.src.push_str("\";\n");
        self.src.set_indent(old_indent);

        if let Some(func_name) = func_name {
            let rt = self.runtime_path().to_string();
            uwriteln!(
                self.src,
                "
                #[inline(never)]
                #[doc(hidden)]
                pub fn {func_name}() {{
                    {rt}::maybe_link_cabi_realloc();
                }}
            ",
            );
        }
    }

    fn is_async(
        &mut self,
        resolve: &Resolve,
        interface: Option<&WorldKey>,
        func: &Function,
        is_import: bool,
    ) -> bool {
        self.opts
            .async_
            .is_async(resolve, interface, func, is_import)
    }
}

impl WorldGenerator for RustWasm {
    fn preprocess(&mut self, resolve: &Resolve, world: WorldId) {
        wit_bindgen_core::generated_preamble(&mut self.src_preamble, env!("CARGO_PKG_VERSION"));

        // Render some generator options to assist with debugging and/or to help
        // recreate it if the original generation command is lost.
        uwriteln!(self.src_preamble, "// Options used:");
        if self.opts.std_feature {
            uwriteln!(self.src_preamble, "//   * std_feature");
        }
        if self.opts.raw_strings {
            uwriteln!(self.src_preamble, "//   * raw_strings");
        }
        if !self.opts.skip.is_empty() {
            uwriteln!(self.src_preamble, "//   * skip: {:?}", self.opts.skip);
        }
        if self.opts.stubs {
            uwriteln!(self.src_preamble, "//   * stubs");
        }
        if let Some(export_prefix) = &self.opts.export_prefix {
            uwriteln!(
                self.src_preamble,
                "//   * export_prefix: {:?}",
                export_prefix
            );
        }
        if let Some(runtime_path) = &self.opts.runtime_path {
            uwriteln!(self.src_preamble, "//   * runtime_path: {:?}", runtime_path);
        }
        if let Some(bitflags_path) = &self.opts.bitflags_path {
            uwriteln!(
                self.src_preamble,
                "//   * bitflags_path: {:?}",
                bitflags_path
            );
        }
        if !matches!(self.opts.ownership, Ownership::Owning) {
            uwriteln!(
                self.src_preamble,
                "//   * ownership: {:?}",
                self.opts.ownership
            );
        }
        if !self.opts.additional_derive_attributes.is_empty() {
            uwriteln!(
                self.src_preamble,
                "//   * additional derives {:?}",
                self.opts.additional_derive_attributes
            );
        }
        if !self.opts.additional_derive_ignore.is_empty() {
            uwriteln!(
                self.src_preamble,
                "//   * additional derives ignored {:?}",
                self.opts.additional_derive_ignore
            );
        }
        for (k, v) in self.opts.with.iter() {
            uwriteln!(self.src_preamble, "//   * with {k:?} = {v}");
        }
        if let Some(type_section_suffix) = &self.opts.type_section_suffix {
            uwriteln!(
                self.src_preamble,
                "//   * type_section_suffix: {:?}",
                type_section_suffix
            );
        }
        if let Some(default) = &self.opts.default_bindings_module {
            uwriteln!(
                self.src_preamble,
                "//   * default-bindings-module: {default:?}"
            );
        }
        if self.opts.disable_run_ctors_once_workaround {
            uwriteln!(
                self.src_preamble,
                "//   * disable-run-ctors-once-workaround"
            );
        }
        if let Some(s) = &self.opts.export_macro_name {
            uwriteln!(self.src_preamble, "//   * export-macro-name: {s}");
        }
        if self.opts.pub_export_macro {
            uwriteln!(self.src_preamble, "//   * pub-export-macro");
        }
        if self.opts.generate_unused_types {
            uwriteln!(self.src_preamble, "//   * generate_unused_types");
        }
        if self.opts.disable_custom_section_link_helpers {
            uwriteln!(
                self.src_preamble,
                "//   * disable_custom_section_link_helpers"
            );
        }
        for opt in self.opts.async_.debug_opts() {
            uwriteln!(self.src_preamble, "//   * async: {opt}");
        }
        self.types.analyze(resolve);
        self.world = Some(world);

        let world = &resolve.worlds[world];
        // Specify that all imports local to the world's package should be
        // generated
        for (key, item) in world.imports.iter() {
            if let WorldItem::Interface { id, .. } = item {
                if resolve.interfaces[*id].package == world.package {
                    let name = resolve.name_world_key(key);
                    if self.with.get(&name).is_none() {
                        self.with.insert(name, TypeGeneration::Generate);
                    }
                }
            }
        }

        for (k, v) in self.opts.with.iter() {
            self.with.insert(k.clone(), v.clone().into());
        }
        self.with.generate_by_default = self.opts.generate_all;
    }

    fn import_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        id: InterfaceId,
        _files: &mut Files,
    ) -> Result<()> {
        let mut to_define = Vec::new();
        for (name, ty_id) in resolve.interfaces[id].types.iter() {
            let full_name = full_wit_type_name(resolve, *ty_id);
            if let Some(type_gen) = self.with.get(&full_name) {
                // skip type definition generation for remapped types
                if type_gen.generated() {
                    to_define.push((name, ty_id));
                }
            } else {
                to_define.push((name, ty_id));
            }
            self.generated_types.insert(full_name);
        }

        self.interface_last_seen_as_import.insert(id, true);
        let wasm_import_module = resolve.name_world_key(name);
        let mut r#gen = self.interface(
            Identifier::Interface(id, name),
            &wasm_import_module,
            resolve,
            true,
        );
        let (snake, module_path) = r#gen.start_append_submodule(name);
        if r#gen.r#gen.name_interface(resolve, id, name, false)? {
            return Ok(());
        }

        for (name, ty_id) in to_define {
            r#gen.define_type(&name, *ty_id);
        }

        r#gen.generate_imports(resolve.interfaces[id].functions.values(), Some(name));

        let docs = &resolve.interfaces[id].docs;

        r#gen.finish_append_submodule(&snake, module_path, docs);

        Ok(())
    }

    fn import_funcs(
        &mut self,
        resolve: &Resolve,
        world: WorldId,
        funcs: &[(&str, &Function)],
        _files: &mut Files,
    ) {
        self.import_funcs_called = true;

        let mut r#gen = self.interface(Identifier::World(world), "$root", resolve, true);

        r#gen.generate_imports(funcs.iter().map(|(_, func)| *func), None);

        let src = r#gen.finish();
        self.src.push_str(&src);
    }

    fn export_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        id: InterfaceId,
        _files: &mut Files,
    ) -> Result<()> {
        let mut to_define = Vec::new();
        for (name, ty_id) in resolve.interfaces[id].types.iter() {
            let full_name = full_wit_type_name(resolve, *ty_id);
            to_define.push((name, ty_id));
            self.generated_types.insert(full_name);
        }

        self.interface_last_seen_as_import.insert(id, false);
        let wasm_import_module = format!("[export]{}", resolve.name_world_key(name));
        let mut r#gen = self.interface(
            Identifier::Interface(id, name),
            &wasm_import_module,
            resolve,
            false,
        );
        let (snake, module_path) = r#gen.start_append_submodule(name);
        if r#gen.r#gen.name_interface(resolve, id, name, true)? {
            return Ok(());
        }

        for (name, ty_id) in to_define {
            r#gen.define_type(&name, *ty_id);
        }

        let macro_name =
            r#gen.generate_exports(Some((id, name)), resolve.interfaces[id].functions.values())?;

        let docs = &resolve.interfaces[id].docs;

        r#gen.finish_append_submodule(&snake, module_path, docs);
        self.export_macros
            .push((macro_name, self.interface_names[&id].path.clone()));

        if self.opts.stubs {
            let world_id = self.world.unwrap();
            let mut r#gen = self.interface(
                Identifier::World(world_id),
                &wasm_import_module,
                resolve,
                false,
            );
            r#gen.generate_stub(Some((id, name)), resolve.interfaces[id].functions.values());
            let stub = r#gen.finish();
            self.src.push_str(&stub);
        }
        Ok(())
    }

    fn export_funcs(
        &mut self,
        resolve: &Resolve,
        world: WorldId,
        funcs: &[(&str, &Function)],
        _files: &mut Files,
    ) -> Result<()> {
        let mut r#gen = self.interface(Identifier::World(world), "[export]$root", resolve, false);
        let macro_name = r#gen.generate_exports(None, funcs.iter().map(|f| f.1))?;
        let src = r#gen.finish();
        self.src.push_str(&src);
        self.export_macros.push((macro_name, String::new()));

        if self.opts.stubs {
            let mut r#gen =
                self.interface(Identifier::World(world), "[export]$root", resolve, false);
            r#gen.generate_stub(None, funcs.iter().map(|f| f.1));
            let stub = r#gen.finish();
            self.src.push_str(&stub);
        }
        Ok(())
    }

    fn import_types(
        &mut self,
        resolve: &Resolve,
        world: WorldId,
        types: &[(&str, TypeId)],
        _files: &mut Files,
    ) {
        let mut to_define = Vec::new();
        for (name, ty_id) in types {
            let full_name = full_wit_type_name(resolve, *ty_id);
            if let Some(type_gen) = self.with.get(&full_name) {
                // skip type definition generation for remapped types
                if type_gen.generated() {
                    to_define.push((name, ty_id));
                }
            } else {
                to_define.push((name, ty_id));
            }
            self.generated_types.insert(full_name);
        }
        let mut r#gen = self.interface(Identifier::World(world), "$root", resolve, true);
        for (name, ty) in to_define {
            r#gen.define_type(name, *ty);
        }
        let src = r#gen.finish();
        self.src.push_str(&src);
    }

    fn finish_imports(&mut self, resolve: &Resolve, world: WorldId, files: &mut Files) {
        if !self.import_funcs_called {
            // We call `import_funcs` even if the world doesn't import any
            // functions since one of the side effects of that method is to
            // generate `struct`s for any imported resources.
            self.import_funcs(resolve, world, &[], files);
        }
    }

    fn finish(&mut self, resolve: &Resolve, world: WorldId, files: &mut Files) -> Result<()> {
        let name = &resolve.worlds[world].name;

        let imports = mem::take(&mut self.import_modules);
        self.emit_modules(imports);
        let exports = mem::take(&mut self.export_modules);
        self.emit_modules(exports);

        self.finish_runtime_module();
        self.finish_export_macro(resolve, world);

        // This is a bit tricky, but we sometimes want to "split" the `world` in
        // two and only encode the imports here.
        //
        // First, a primer. Each invocation of `generate!` has a WIT world as
        // input. This is one of the first steps in the build process as wasm
        // hasn't even been produced yet. One of the later stages of the build
        // process will be to emit a component, currently through the
        // `wit-component` crate. That crate relies on custom sections being
        // present to describe what WIT worlds were present in the wasm binary.
        //
        // Additionally a `generate!` macro is not the only thing in a binary.
        // There might be multiple `generate!` macros, perhaps even across
        // different languages. To handle all this `wit-component` will decode
        // each custom section and "union" everything together. Unioning in
        // general should work so long as everything has the same structure and
        // came from the same source.
        //
        // The problem here is that if `pub_export_macros` is turned on, meaning
        // that the macros are supposed to be used across crates, then neither
        // the imports nor the exports of this world are guaranteed to be used.
        // For imports that's ok because `wit-component` will drop any unused
        // imports automatically. For exports that's a problem because
        // `wit-component` unconditionally looks for a definition for all
        // exports.
        //
        // When `pub_export_macros` is turned on, and cross-crate usage of the
        // macro is expected, this is solved by emitting two custom sections:
        //
        // 1. The first section emitted here only has the imports of the world.
        //    This slimmed down world should be able to be unioned with the
        //    first world trivially and will be GC'd by `wit-component` if not
        //    used.
        // 2. The second section is emitted as part of the generated `export!`
        //    macro invocation. That world has all the export information as
        //    well as all the import information.
        //
        // In the end this is hoped to ensure that usage of crates like `wasi`
        // don't accidentally try to export things, for example.
        let mut resolve_copy;
        let (resolve_to_encode, world_to_encode) = if self.opts.pub_export_macro {
            resolve_copy = resolve.clone();
            let world_copy = resolve_copy.worlds.alloc(World {
                exports: Default::default(),
                name: format!("{name}-with-all-of-its-exports-removed"),
                ..resolve.worlds[world].clone()
            });
            (&resolve_copy, world_copy)
        } else {
            (resolve, world)
        };
        self.emit_custom_section(
            resolve_to_encode,
            world_to_encode,
            "encoded world",
            if self.opts.disable_custom_section_link_helpers {
                None
            } else {
                Some("__link_custom_section_describing_imports")
            },
        );

        if self.opts.stubs {
            self.src.push_str("\n#[derive(Debug)]\npub struct Stub;\n");
        }

        let mut src = mem::take(&mut self.src);
        if self.opts.format {
            let syntax_tree = syn::parse_file(src.as_str()).unwrap();
            *src.as_mut_string() = prettyplease::unparse(&syntax_tree);
        }

        // Prepend the preamble. We do this after formatting because
        // `syn::parse_file` + `prettyplease::unparse` does not preserve comments.
        let src_preamble = mem::take(&mut self.src_preamble);
        *src.as_mut_string() = format!("{}{}", src_preamble.as_str(), src.as_str());

        let module_name = name.to_snake_case();
        files.push(&format!("{module_name}.rs"), src.as_bytes());

        let remapped_keys = self
            .with
            .iter()
            .map(|(k, _)| k)
            .cloned()
            .collect::<HashSet<String>>();

        let mut unused_keys = remapped_keys
            .difference(&self.generated_types)
            .collect::<Vec<&String>>();

        unused_keys.sort();

        if !unused_keys.is_empty() {
            bail!("unused remappings provided via `with`: {unused_keys:?}");
        }

        // Error about unused async configuration to help catch configuration
        // errors.
        self.opts.async_.ensure_all_used()?;

        Ok(())
    }
}

fn compute_module_path(name: &WorldKey, resolve: &Resolve, is_export: bool) -> Vec<String> {
    let mut path = Vec::new();
    if is_export {
        path.push("exports".to_string());
    }
    match name {
        WorldKey::Name(name) => {
            path.push(to_rust_ident(name));
        }
        WorldKey::Interface(id) => {
            let iface = &resolve.interfaces[*id];
            let pkg = iface.package.unwrap();
            let pkgname = resolve.packages[pkg].name.clone();
            path.push(to_rust_ident(&pkgname.namespace));
            path.push(name_package_module(resolve, pkg));
            path.push(to_rust_ident(iface.name.as_ref().unwrap()));
        }
    }
    path
}

enum Identifier<'a> {
    World(WorldId),
    Interface(InterfaceId, &'a WorldKey),
    StreamOrFuturePayload,
}

fn group_by_resource<'a>(
    funcs: impl Iterator<Item = &'a Function>,
) -> BTreeMap<Option<TypeId>, Vec<&'a Function>> {
    let mut by_resource = BTreeMap::<_, Vec<_>>::new();
    for func in funcs {
        by_resource
            .entry(func.kind.resource())
            .or_default()
            .push(func);
    }
    by_resource
}

#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum Ownership {
    /// Generated types will be composed entirely of owning fields, regardless
    /// of whether they are used as parameters to imports or not.
    #[default]
    Owning,

    /// Generated types used as parameters to imports will be "deeply
    /// borrowing", i.e. contain references rather than owned values when
    /// applicable.
    Borrowing {
        /// Whether or not to generate "duplicate" type definitions for a single
        /// WIT type if necessary, for example if it's used as both an import
        /// and an export, or if it's used both as a parameter to an import and
        /// a return value from an import.
        duplicate_if_necessary: bool,
    },
}

impl FromStr for Ownership {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owning" => Ok(Self::Owning),
            "borrowing" => Ok(Self::Borrowing {
                duplicate_if_necessary: false,
            }),
            "borrowing-duplicate-if-necessary" => Ok(Self::Borrowing {
                duplicate_if_necessary: true,
            }),
            _ => Err(format!(
                "unrecognized ownership: `{s}`; \
                 expected `owning`, `borrowing`, or `borrowing-duplicate-if-necessary`"
            )),
        }
    }
}

impl fmt::Display for Ownership {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Ownership::Owning => "owning",
            Ownership::Borrowing {
                duplicate_if_necessary: false,
            } => "borrowing",
            Ownership::Borrowing {
                duplicate_if_necessary: true,
            } => "borrowing-duplicate-if-necessary",
        })
    }
}

/// Options for with "with" remappings.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum WithOption {
    Path(String),
    Generate,
}

impl std::fmt::Display for WithOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WithOption::Path(p) => f.write_fmt(format_args!("\"{p}\"")),
            WithOption::Generate => f.write_str("generate"),
        }
    }
}

impl From<WithOption> for TypeGeneration {
    fn from(opt: WithOption) -> Self {
        match opt {
            WithOption::Path(p) => TypeGeneration::Remap(p),
            WithOption::Generate => TypeGeneration::Generate,
        }
    }
}

#[derive(Default)]
struct FnSig {
    async_: bool,
    unsafe_: bool,
    private: bool,
    use_item_name: bool,
    generics: Option<String>,
    self_arg: Option<String>,
    self_is_first_param: bool,
}

impl FnSig {
    fn update_for_func(&mut self, func: &Function) {
        if let FunctionKind::Method(_) | FunctionKind::AsyncMethod(_) = &func.kind {
            self.self_arg = Some("&self".into());
            self.self_is_first_param = true;
        }
    }
}

pub fn to_rust_ident(name: &str) -> String {
    match name {
        // Escape Rust keywords.
        // Source: https://doc.rust-lang.org/reference/keywords.html
        "as" => "as_".into(),
        "break" => "break_".into(),
        "const" => "const_".into(),
        "continue" => "continue_".into(),
        "crate" => "crate_".into(),
        "else" => "else_".into(),
        "enum" => "enum_".into(),
        "extern" => "extern_".into(),
        "false" => "false_".into(),
        "fn" => "fn_".into(),
        "for" => "for_".into(),
        "if" => "if_".into(),
        "impl" => "impl_".into(),
        "in" => "in_".into(),
        "let" => "let_".into(),
        "loop" => "loop_".into(),
        "match" => "match_".into(),
        "mod" => "mod_".into(),
        "move" => "move_".into(),
        "mut" => "mut_".into(),
        "pub" => "pub_".into(),
        "ref" => "ref_".into(),
        "return" => "return_".into(),
        "self" => "self_".into(),
        "static" => "static_".into(),
        "struct" => "struct_".into(),
        "super" => "super_".into(),
        "trait" => "trait_".into(),
        "true" => "true_".into(),
        "type" => "type_".into(),
        "unsafe" => "unsafe_".into(),
        "use" => "use_".into(),
        "where" => "where_".into(),
        "while" => "while_".into(),
        "async" => "async_".into(),
        "await" => "await_".into(),
        "dyn" => "dyn_".into(),
        "abstract" => "abstract_".into(),
        "become" => "become_".into(),
        "box" => "box_".into(),
        "do" => "do_".into(),
        "final" => "final_".into(),
        "macro" => "macro_".into(),
        "override" => "override_".into(),
        "priv" => "priv_".into(),
        "typeof" => "typeof_".into(),
        "unsized" => "unsized_".into(),
        "virtual" => "virtual_".into(),
        "yield" => "yield_".into(),
        "try" => "try_".into(),
        s => s.to_snake_case(),
    }
}

fn to_upper_camel_case(name: &str) -> String {
    match name {
        // The name "Guest" is reserved for traits generated by exported
        // interfaces, so remap types defined in wit to something else.
        "guest" => "Guest_".to_string(),
        s => s.to_upper_camel_case(),
    }
}

fn wasm_type(ty: WasmType) -> &'static str {
    match ty {
        WasmType::I32 => "i32",
        WasmType::I64 => "i64",
        WasmType::F32 => "f32",
        WasmType::F64 => "f64",
        WasmType::Pointer => "*mut u8",
        WasmType::Length => "usize",

        // `PointerOrI64` can hold either a `u64` or a pointer with provenance.
        // Neither a `u64` nor a pointer type can portably do both, so we use
        // `MaybeUninit<u64>`, since `MaybeUninit` is [documented] to preserve
        // provenance.
        // [documented]: https://github.com/rust-lang/rfcs/blob/master/text/3559-rust-has-provenance.md#reference-level-explanation
        WasmType::PointerOrI64 => "::core::mem::MaybeUninit::<u64>",
    }
}

fn declare_import(
    wasm_import_module: &str,
    wasm_import_name: &str,
    rust_name: &str,
    params: &[WasmType],
    results: &[WasmType],
) -> String {
    let mut sig = "(".to_owned();
    for param in params.iter() {
        sig.push_str("_: ");
        sig.push_str(wasm_type(*param));
        sig.push_str(", ");
    }
    sig.push(')');
    assert!(results.len() < 2);
    for result in results.iter() {
        sig.push_str(" -> ");
        sig.push_str(wasm_type(*result));
    }
    format!(
        "
            #[cfg(target_arch = \"wasm32\")]
            #[link(wasm_import_module = \"{wasm_import_module}\")]
            unsafe extern \"C\" {{
                #[link_name = \"{wasm_import_name}\"]
                fn {rust_name}{sig};
            }}

            #[cfg(not(target_arch = \"wasm32\"))]
            unsafe extern \"C\" fn {rust_name}{sig} {{ unreachable!() }}
        "
    )
}

fn int_repr(repr: Int) -> &'static str {
    match repr {
        Int::U8 => "u8",
        Int::U16 => "u16",
        Int::U32 => "u32",
        Int::U64 => "u64",
    }
}

fn bitcast(casts: &[Bitcast], operands: &[String], results: &mut Vec<String>) {
    for (cast, operand) in casts.iter().zip(operands) {
        results.push(perform_cast(operand, cast));
    }
}

fn perform_cast(operand: &str, cast: &Bitcast) -> String {
    match cast {
        Bitcast::None => operand.to_owned(),
        Bitcast::I32ToI64 => format!("i64::from({operand})"),
        Bitcast::F32ToI32 => format!("({operand}).to_bits() as i32"),
        Bitcast::F64ToI64 => format!("({operand}).to_bits() as i64"),
        Bitcast::I64ToI32 => format!("{operand} as i32"),
        Bitcast::I32ToF32 => format!("f32::from_bits({operand} as u32)"),
        Bitcast::I64ToF64 => format!("f64::from_bits({operand} as u64)"),
        Bitcast::F32ToI64 => format!("i64::from(({operand}).to_bits())"),
        Bitcast::I64ToF32 => format!("f32::from_bits({operand} as u32)"),

        // Convert an `i64` into a `MaybeUninit<u64>`.
        Bitcast::I64ToP64 => format!("::core::mem::MaybeUninit::new({operand} as u64)"),
        // Convert a `MaybeUninit<u64>` holding an `i64` value back into
        // the `i64` value.
        Bitcast::P64ToI64 => format!("{operand}.assume_init() as i64"),

        // Convert a pointer value into a `MaybeUninit<u64>`.
        Bitcast::PToP64 => {
            format!(
                "{{
                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                        t.as_mut_ptr().cast::<*mut u8>().write({operand});
                        t
                    }}"
            )
        }
        // Convert a `MaybeUninit<u64>` holding a pointer value back into
        // the pointer value.
        Bitcast::P64ToP => {
            format!("{operand}.as_ptr().cast::<*mut u8>().read()")
        }
        // Convert an `i32` or a `usize` into a pointer.
        Bitcast::I32ToP | Bitcast::LToP => {
            format!("{operand} as *mut u8")
        }
        // Convert a pointer or length holding an `i32` value back into the `i32`.
        Bitcast::PToI32 | Bitcast::LToI32 => {
            format!("{operand} as i32")
        }
        // Convert an `i32`, `i64`, or pointer holding a `usize` value back into the `usize`.
        Bitcast::I32ToL | Bitcast::I64ToL | Bitcast::PToL => {
            format!("{operand} as usize")
        }
        // Convert a `usize` into an `i64`.
        Bitcast::LToI64 => {
            format!("{operand} as i64")
        }
        Bitcast::Sequence(sequence) => {
            let [first, second] = &**sequence;
            perform_cast(&perform_cast(operand, first), second)
        }
    }
}

enum RustFlagsRepr {
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl RustFlagsRepr {
    fn new(f: &Flags) -> RustFlagsRepr {
        match f.repr() {
            FlagsRepr::U8 => RustFlagsRepr::U8,
            FlagsRepr::U16 => RustFlagsRepr::U16,
            FlagsRepr::U32(1) => RustFlagsRepr::U32,
            FlagsRepr::U32(2) => RustFlagsRepr::U64,
            FlagsRepr::U32(3 | 4) => RustFlagsRepr::U128,
            FlagsRepr::U32(n) => panic!("unsupported number of flags: {}", n * 32),
        }
    }
}

impl fmt::Display for RustFlagsRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustFlagsRepr::U8 => "u8".fmt(f),
            RustFlagsRepr::U16 => "u16".fmt(f),
            RustFlagsRepr::U32 => "u32".fmt(f),
            RustFlagsRepr::U64 => "u64".fmt(f),
            RustFlagsRepr::U128 => "u128".fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MissingWith(pub String);

impl fmt::Display for MissingWith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "missing `with` mapping for the key `{}`", self.0)
    }
}

impl std::error::Error for MissingWith {}

// bail!("no remapping found for {with_name:?} - use the `generate!` macro's `with` option to force the interface to be generated or specify where it is already defined:
// ```
// with: {{\n\t{with_name:?}: generate\n}}
// ```")

/// Returns the full WIT type name with fully qualified interface name
fn full_wit_type_name(resolve: &Resolve, id: TypeId) -> String {
    let id = dealias(resolve, id);
    let type_def = &resolve.types[id];
    let interface_name = match type_def.owner {
        TypeOwner::World(w) => Some(resolve.worlds[w].name.clone()),
        TypeOwner::Interface(id) => resolve.id_of(id),
        TypeOwner::None => None,
    };
    match interface_name {
        Some(interface_name) => format!("{}/{}", interface_name, type_def.name.clone().unwrap()),
        None => type_def.name.clone().unwrap(),
    }
}

enum ConstructorReturnType {
    /// Resource constructor is infallible. E.g.:
    /// ```wit
    /// resource R {
    ///    constructor(..);
    /// }
    /// ```
    Self_,

    /// Resource constructor is fallible. E.g.:
    /// ```wit
    /// resource R {
    ///    constructor(..) -> result<R, err>;
    /// }
    /// ```
    Result { err: Option<Type> },
}

fn classify_constructor_return_type(
    resolve: &Resolve,
    resource_id: TypeId,
    result: &Option<Type>,
) -> ConstructorReturnType {
    fn classify(
        resolve: &Resolve,
        resource_id: TypeId,
        result: &Option<Type>,
    ) -> Option<ConstructorReturnType> {
        let resource_id = dealias(resolve, resource_id);
        let typedef = match result.as_ref()? {
            Type::Id(id) => &resolve.types[dealias(resolve, *id)],
            _ => return None,
        };

        match &typedef.kind {
            TypeDefKind::Handle(Handle::Own(id)) if dealias(resolve, *id) == resource_id => {
                Some(ConstructorReturnType::Self_)
            }
            TypeDefKind::Result(Result_ { ok, err }) => {
                let ok_typedef = match ok.as_ref()? {
                    Type::Id(id) => &resolve.types[dealias(resolve, *id)],
                    _ => return None,
                };

                match &ok_typedef.kind {
                    TypeDefKind::Handle(Handle::Own(id))
                        if dealias(resolve, *id) == resource_id =>
                    {
                        Some(ConstructorReturnType::Result { err: *err })
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    classify(resolve, resource_id, result).expect("invalid constructor")
}

use crate::bindgen::{FunctionBindgen, POINTER_SIZE_EXPRESSION};
use crate::{
    ConstructorReturnType, FnSig, Identifier, InterfaceName, Ownership, RuntimeItem, RustFlagsRepr,
    RustWasm, TypeGeneration, classify_constructor_return_type, full_wit_type_name, int_repr,
    to_rust_ident, to_upper_camel_case, wasm_type,
};
use anyhow::Result;
use heck::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::mem;
use wit_bindgen_core::abi::{self, AbiVariant, LiftLower};
use wit_bindgen_core::{
    AnonymousTypeGenerator, Source, TypeInfo, dealias, uwrite, uwriteln, wit_parser::*,
};

pub struct InterfaceGenerator<'a> {
    pub src: Source,
    pub(super) identifier: Identifier<'a>,
    pub in_import: bool,
    pub sizes: SizeAlign,
    pub(super) r#gen: &'a mut RustWasm,
    pub wasm_import_module: &'a str,
    pub resolve: &'a Resolve,
    pub return_pointer_area_size: ArchitectureSize,
    pub return_pointer_area_align: Alignment,
    pub(super) needs_runtime_module: bool,
}

/// A description of the "mode" in which a type is printed.
///
/// Rust types can either be "borrowed" or "owned". This primarily has to do
/// with lists and imports where arguments to imports can be borrowed lists in
/// theory as ownership is not taken from the caller. This structure is used to
/// help with this fact in addition to the various codegen options of this
/// generator. Namely types in WIT can be reflected into Rust as two separate
/// types, one "owned" and one "borrowed" (aka one with `Vec` and one with
/// `&[T]`).
///
/// This structure is used in conjunction with `modes_of` and `type_mode_for*`
/// primarily. That enables creating a programmatic description of what a type
/// is rendered as along with various options.
///
/// Note that a `TypeMode` is a description of a single "level" of a type. This
/// means that there's one mode for `Vec<T>` and one mode for `T` internally.
/// This is mostly used for things like records where some fields have lifetime
/// parameters, for example, and others don't.
///
/// This type is intended to simplify generation of types and encapsulate all
/// the knowledge about whether lifetime parameters are used and how lists are
/// rendered.
///
/// There are currently two users of lifetime parameters:
///
/// * Lists - when borrowed these are rendered as either `&[T]` or `&str`.
/// * Borrowed resources - for resources owned by the current module they're
///   represented as `&T` and for borrows of imported resources they're
///   represented, more-or-less, as `&Resource<T>`.
///
/// Lists have a choice of being rendered as borrowed or not but resources are
/// required to be borrowed.
#[derive(Debug, Copy, Clone, PartialEq)]
struct TypeMode {
    /// The lifetime parameter, if any, for this type. If present this type is
    /// required to have a lifetime parameter.
    lifetime: Option<&'static str>,

    /// Whether or not lists are borrowed in this type.
    ///
    /// If this field is `true` then lists are rendered as `&[T]` and `&str`
    /// rather than their owned equivalent. If this field is `false` than the
    /// owned equivalents are used instead.
    lists_borrowed: bool,

    /// The "style" of ownership that this mode was created with.
    ///
    /// This information is used to determine what mode the next layer deep int
    /// he type tree is rendered with. For example if this layer is owned so is
    /// the next layer. This is primarily used for the "OnlyTopBorrowed"
    /// ownership style where all further layers beneath that are `Owned`.
    style: TypeOwnershipStyle,
}

/// The style of ownership of a type, used to initially create a `TypeMode` and
/// stored internally within it as well.
#[derive(Debug, Copy, Clone, PartialEq)]
enum TypeOwnershipStyle {
    /// This style means owned things are printed such as `Vec<T>` and `String`.
    ///
    /// Note that this primarily applies to lists.
    Owned,

    /// This style means that lists/strings are `&[T]` and `&str`.
    ///
    /// Note that this primarily applies to lists.
    Borrowed,

    /// This style means that the top-level of a type is borrowed but all other
    /// layers are `Owned`.
    ///
    /// This is used for parameters in the "owning" mode of generation to
    /// imports. It's easy enough to create a `&T` at the root layer but it's
    /// more difficult to create `&T` stored within a `U`, for example.
    OnlyTopBorrowed,
}

impl TypeMode {
    /// Returns a mode where everything is indicated that it's supposed to be
    /// rendered as an "owned" type.
    fn owned() -> TypeMode {
        TypeMode {
            lifetime: None,
            lists_borrowed: false,
            style: TypeOwnershipStyle::Owned,
        }
    }
}

impl TypeOwnershipStyle {
    /// Preserves this mode except for `OnlyTopBorrowed` where it switches it to
    /// `Owned`.
    fn next(&self) -> TypeOwnershipStyle {
        match self {
            TypeOwnershipStyle::Owned => TypeOwnershipStyle::Owned,
            TypeOwnershipStyle::Borrowed => TypeOwnershipStyle::Borrowed,
            TypeOwnershipStyle::OnlyTopBorrowed => TypeOwnershipStyle::Owned,
        }
    }
}

enum PayloadFor {
    Future,
    Stream,
}

impl<'i> InterfaceGenerator<'i> {
    pub(super) fn generate_exports<'a>(
        &mut self,
        interface: Option<(InterfaceId, &WorldKey)>,
        funcs: impl Iterator<Item = &'a Function> + Clone,
    ) -> Result<String> {
        let mut traits = BTreeMap::new();
        let mut funcs_to_export = Vec::new();
        let mut resources_to_drop = Vec::new();

        traits.insert(None, ("Guest".to_string(), Vec::new()));

        if let Some((id, _)) = interface {
            for (name, id) in self.resolve.interfaces[id].types.iter() {
                match self.resolve.types[*id].kind {
                    TypeDefKind::Resource => {}
                    _ => continue,
                }
                resources_to_drop.push(name);
                let camel = name.to_upper_camel_case();
                traits.insert(Some(*id), (format!("Guest{camel}"), Vec::new()));
            }
        }

        for func in funcs {
            if self.r#gen.skip.contains(&func.name) {
                continue;
            }

            let async_ = self
                .r#gen
                .is_async(self.resolve, interface.map(|p| p.1), func, false);
            let resource = func.kind.resource();

            funcs_to_export.push((func, resource, async_));
            let (trait_name, methods) = traits.get_mut(&resource).unwrap();
            self.generate_guest_export(func, interface.map(|(_, k)| k), &trait_name, async_);

            let prev = mem::take(&mut self.src);
            let mut sig = FnSig {
                async_,
                use_item_name: true,
                private: true,
                ..Default::default()
            };
            sig.update_for_func(&func);
            self.print_signature(func, true, &sig);
            self.src.push_str(";\n");
            let trait_method = mem::replace(&mut self.src, prev);
            methods.push(trait_method);
        }

        let (name, methods) = traits.remove(&None).unwrap();
        if !methods.is_empty() || !traits.is_empty() {
            self.generate_interface_trait(
                &name,
                &methods,
                traits
                    .iter()
                    .map(|(resource, (trait_name, ..))| (resource.unwrap(), trait_name.as_str())),
            )
        }

        for (resource, (trait_name, methods)) in traits.iter() {
            uwriteln!(self.src, "pub trait {trait_name}: 'static {{");
            let resource = resource.unwrap();
            let resource_name = self.resolve.types[resource].name.as_ref().unwrap();
            let (_, interface_name) = interface.unwrap();
            let module = self.resolve.name_world_key(interface_name);
            let wasm_import_module = format!("[export]{module}");
            let import_new = crate::declare_import(
                &wasm_import_module,
                &format!("[resource-new]{resource_name}"),
                "new",
                &[abi::WasmType::Pointer],
                &[abi::WasmType::I32],
            );
            let import_rep = crate::declare_import(
                &wasm_import_module,
                &format!("[resource-rep]{resource_name}"),
                "rep",
                &[abi::WasmType::I32],
                &[abi::WasmType::Pointer],
            );
            uwriteln!(
                self.src,
                r#"
#[doc(hidden)]
unsafe fn _resource_new(val: *mut u8) -> u32
    where Self: Sized
{{
    {import_new}
    unsafe {{ new(val) as u32 }}
}}

#[doc(hidden)]
fn _resource_rep(handle: u32) -> *mut u8
    where Self: Sized
{{
    {import_rep}
    unsafe {{ rep(handle as i32) }}
}}

                    "#
            );
            for method in methods {
                self.src.push_str(method);
            }
            uwriteln!(self.src, "}}");
        }

        let macro_name = match interface {
            None => {
                let world = match self.identifier {
                    Identifier::World(w) => w,
                    Identifier::Interface(..) | Identifier::StreamOrFuturePayload => unreachable!(),
                };
                let world = self.resolve.worlds[world].name.to_snake_case();
                format!("__export_world_{world}_cabi")
            }
            Some((_, name)) => {
                format!(
                    "__export_{}_cabi",
                    self.resolve
                        .name_world_key(name)
                        .to_snake_case()
                        .chars()
                        .map(|c| if !c.is_alphanumeric() { '_' } else { c })
                        .collect::<String>()
                )
            }
        };
        let (macro_export, use_vis) = if self.r#gen.opts.pub_export_macro {
            ("#[macro_export]", "pub")
        } else {
            ("", "pub(crate)")
        };

        uwriteln!(
            self.src,
            "\
#[doc(hidden)]
{macro_export}
macro_rules! {macro_name} {{
    ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {{
"
        );

        for (func, resource, async_) in funcs_to_export {
            let ty = match resource {
                None => "$ty".to_string(),
                Some(id) => {
                    let name = self.resolve.types[id]
                        .name
                        .as_ref()
                        .unwrap()
                        .to_upper_camel_case();
                    format!("<$ty as $($path_to_types)*::Guest>::{name}")
                }
            };
            self.generate_raw_cabi_export(func, &ty, "$($path_to_types)*", async_);
        }
        let export_prefix = self.r#gen.opts.export_prefix.as_deref().unwrap_or("");
        for name in resources_to_drop {
            let module = match self.identifier {
                Identifier::Interface(_, key) => self.resolve.name_world_key(key),
                Identifier::World(_) | Identifier::StreamOrFuturePayload => {
                    unreachable!()
                }
            };
            let camel = name.to_upper_camel_case();
            uwriteln!(
                self.src,
                r#"
                const _: () = {{
                    #[doc(hidden)]
                    #[unsafe(export_name = "{export_prefix}{module}#[dtor]{name}")]
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn dtor(rep: *mut u8) {{
                        unsafe {{
                            $($path_to_types)*::{camel}::dtor::<
                                <$ty as $($path_to_types)*::Guest>::{camel}
                            >(rep)
                        }}
                    }}
                }};
                "#
            );
        }
        uwriteln!(self.src, "}};);");
        uwriteln!(self.src, "}}");
        uwriteln!(self.src, "#[doc(hidden)]");
        uwriteln!(self.src, "{use_vis} use {macro_name};");
        Ok(macro_name)
    }

    fn generate_interface_trait<'a>(
        &mut self,
        trait_name: &str,
        methods: &[Source],
        resource_traits: impl Iterator<Item = (TypeId, &'a str)>,
    ) {
        uwriteln!(self.src, "pub trait {trait_name} {{");
        for (id, trait_name) in resource_traits {
            let name = self.resolve.types[id]
                .name
                .as_ref()
                .unwrap()
                .to_upper_camel_case();
            uwriteln!(self.src, "type {name}: {trait_name};");
        }
        for method in methods {
            self.src.push_str(method);
        }
        uwriteln!(self.src, "}}");
    }

    pub fn generate_imports<'a>(
        &mut self,
        funcs: impl Iterator<Item = &'a Function>,
        interface: Option<&WorldKey>,
    ) {
        for func in funcs {
            self.generate_guest_import(func, interface);
        }
    }

    pub fn align_area(&mut self, alignment: Alignment) {
        match alignment {
            Alignment::Pointer => uwriteln!(
                self.src,
                "#[cfg_attr(target_pointer_width=\"64\", repr(align(8)))]
                    #[cfg_attr(target_pointer_width=\"32\", repr(align(4)))]"
            ),
            Alignment::Bytes(bytes) => {
                uwriteln!(self.src, "#[repr(align({align}))]", align = bytes.get())
            }
        }
    }

    pub fn finish(&mut self) -> String {
        if !self.return_pointer_area_size.is_empty() {
            uwriteln!(self.src,);
            self.align_area(self.return_pointer_area_align);
            uwrite!(
                self.src,
                    "struct _RetArea([::core::mem::MaybeUninit::<u8>; {size}]);
                    static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); {size}]);
",
                size = self.return_pointer_area_size.format_term(POINTER_SIZE_EXPRESSION, true),
            );
        }

        let src = mem::take(&mut self.src).into();

        if self.needs_runtime_module {
            let root = self.path_to_root();
            if !root.is_empty() {
                return format!("use {root}_rt;\n{src}");
            }
        }
        src
    }

    pub(crate) fn path_to_root(&self) -> String {
        let mut path_to_root = String::new();

        match self.identifier {
            Identifier::Interface(_, key) => {
                // Escape the submodule for this interface
                path_to_root.push_str("super::");

                // Escape the `exports` top-level submodule
                if !self.in_import {
                    path_to_root.push_str("super::");
                }

                // Escape the namespace/package submodules for interface-based ids
                match key {
                    WorldKey::Name(_) => {}
                    WorldKey::Interface(_) => {
                        path_to_root.push_str("super::super::");
                    }
                }
            }
            Identifier::StreamOrFuturePayload => path_to_root.push_str("super::super::"),
            _ => {}
        }
        path_to_root
    }

    pub fn start_append_submodule(&mut self, name: &WorldKey) -> (String, Vec<String>) {
        let snake = match name {
            WorldKey::Name(name) => to_rust_ident(name),
            WorldKey::Interface(id) => {
                to_rust_ident(self.resolve.interfaces[*id].name.as_ref().unwrap())
            }
        };
        let module_path = crate::compute_module_path(name, &self.resolve, !self.in_import);
        (snake, module_path)
    }

    pub fn finish_append_submodule(mut self, snake: &str, module_path: Vec<String>, docs: &Docs) {
        let module = self.finish();

        self.rustdoc(docs);
        let docs = mem::take(&mut self.src).to_string();
        let docs = docs.trim_end();

        let path_to_root = self.path_to_root();
        let used_static = if self.r#gen.opts.disable_custom_section_link_helpers {
            String::new()
        } else {
            format!(
                "\
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() =
                        {path_to_root}__link_custom_section_describing_imports;
                "
            )
        };
        let module = format!(
            "\
                {docs}
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod {snake} {{
                    {used_static}
                    {module}
                }}
",
        );
        let map = if self.in_import {
            &mut self.r#gen.import_modules
        } else {
            &mut self.r#gen.export_modules
        };
        map.push((module, module_path))
    }

    fn generate_payloads(&mut self, prefix: &str, func: &Function, interface: Option<&WorldKey>) {
        let old_identifier = mem::replace(&mut self.identifier, Identifier::StreamOrFuturePayload);

        for (index, ty) in func
            .find_futures_and_streams(self.resolve)
            .into_iter()
            .enumerate()
        {
            let module = format!(
                "{prefix}{}",
                interface
                    .map(|name| self.resolve.name_world_key(name))
                    .unwrap_or_else(|| "$root".into())
            );
            let func_name = &func.name;

            match &self.resolve.types[ty].kind {
                TypeDefKind::Future(payload_type) => {
                    self.generate_payload(
                        PayloadFor::Future,
                        &module,
                        index,
                        func_name,
                        payload_type.as_ref(),
                    );
                }
                TypeDefKind::Stream(payload_type) => {
                    self.generate_payload(
                        PayloadFor::Stream,
                        &module,
                        index,
                        func_name,
                        payload_type.as_ref(),
                    );
                }
                _ => unreachable!(),
            }
        }

        self.identifier = old_identifier;
    }

    fn generate_payload(
        &mut self,
        payload_for: PayloadFor,
        module: &str,
        index: usize,
        func_name: &str,
        payload_type: Option<&Type>,
    ) {
        let name = if let Some(payload_type) = payload_type {
            self.type_name_owned(payload_type)
        } else {
            "()".into()
        };
        let map = match payload_for {
            PayloadFor::Future => &mut self.r#gen.future_payloads,
            PayloadFor::Stream => &mut self.r#gen.stream_payloads,
        };

        if map.contains_key(&name) {
            return;
        }
        let ordinal = map.len();
        let async_support = self.r#gen.async_support_path();
        let (size, align) = if let Some(payload_type) = payload_type {
            (
                self.sizes.size(payload_type),
                self.sizes.align(payload_type),
            )
        } else {
            (
                ArchitectureSize {
                    bytes: 0,
                    pointers: 0,
                },
                Alignment::default(),
            )
        };
        let size = size.size_wasm32();
        let align = align.align_wasm32();
        let lift;
        let lower;
        let dealloc_lists;
        if let Some(payload_type) = payload_type {
            lift = self.lift_from_memory("ptr", &payload_type, &module);
            dealloc_lists = self.deallocate_lists(
                std::slice::from_ref(payload_type),
                &["ptr".to_string()],
                true,
                &module,
            );
            lower = self.lower_to_memory("ptr", "value", &payload_type, &module);
        } else {
            lift = format!("let _ = ptr;");
            lower = format!("let _ = (ptr, value);");
            dealloc_lists = format!("let _ = ptr;");
        }
        let import_prefix = match payload_for {
            PayloadFor::Future => "future",
            PayloadFor::Stream => "stream",
        };
        let camel = match payload_for {
            PayloadFor::Future => "Future",
            PayloadFor::Stream => "Stream",
        };
        let start_extra = match payload_for {
            PayloadFor::Future => "",
            PayloadFor::Stream => ", _: usize",
        };
        let mut lift_fn = format!("unsafe fn lift(ptr: *mut u8) -> {name} {{ {lift} }}");
        let mut lower_fn = format!("unsafe fn lower(value: {name}, ptr: *mut u8) {{ {lower} }}");
        let mut dealloc_lists_fn =
            format!("unsafe fn dealloc_lists(ptr: *mut u8) {{ {dealloc_lists} }}");
        let mut lift_arg = "lift";
        let mut lower_arg = "lower";
        let mut dealloc_lists_arg = "dealloc_lists";

        if let PayloadFor::Stream = payload_for {
            lift_arg = "lift: Some(lift)";
            lower_arg = "lower: Some(lower)";
            dealloc_lists_arg = "dealloc_lists: Some(dealloc_lists)";

            let is_list_canonical = match payload_type {
                Some(ty) => self.is_list_canonical(ty),
                None => true,
            };

            if is_list_canonical {
                lift_arg = "lift: None";
                lower_arg = "lower: None";
                dealloc_lists_arg = "dealloc_lists: None";
                lift_fn = String::new();
                lower_fn = String::new();
                dealloc_lists_fn = String::new();
            }
        }

        let code = format!(
            r#"
#[doc(hidden)]
#[allow(unused_unsafe)]
pub mod vtable{ordinal} {{

    #[cfg(not(target_arch = "wasm32"))]
    unsafe extern "C" fn cancel_write(_: u32) -> u32 {{ unreachable!() }}
    #[cfg(not(target_arch = "wasm32"))]
    unsafe extern "C" fn cancel_read(_: u32) -> u32 {{ unreachable!() }}
    #[cfg(not(target_arch = "wasm32"))]
    unsafe extern "C" fn drop_writable(_: u32) {{ unreachable!() }}
    #[cfg(not(target_arch = "wasm32"))]
    unsafe extern "C" fn drop_readable(_: u32) {{ unreachable!() }}
    #[cfg(not(target_arch = "wasm32"))]
    unsafe extern "C" fn new() -> u64 {{ unreachable!() }}
    #[cfg(not(target_arch = "wasm32"))]
    unsafe extern "C" fn start_read(_: u32, _: *mut u8{start_extra}) -> u32 {{ unreachable!() }}
    #[cfg(not(target_arch = "wasm32"))]
    unsafe extern "C" fn start_write(_: u32, _: *const u8{start_extra}) -> u32 {{ unreachable!() }}

    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "{module}")]
    unsafe extern "C" {{
        #[link_name = "[{import_prefix}-new-{index}]{func_name}"]
        fn new() -> u64;
        #[link_name = "[{import_prefix}-cancel-write-{index}]{func_name}"]
        fn cancel_write(_: u32) -> u32;
        #[link_name = "[{import_prefix}-cancel-read-{index}]{func_name}"]
        fn cancel_read(_: u32) -> u32;
        #[link_name = "[{import_prefix}-drop-writable-{index}]{func_name}"]
        fn drop_writable(_: u32);
        #[link_name = "[{import_prefix}-drop-readable-{index}]{func_name}"]
        fn drop_readable(_: u32);
        #[link_name = "[async-lower][{import_prefix}-read-{index}]{func_name}"]
        fn start_read(_: u32, _: *mut u8{start_extra}) -> u32;
        #[link_name = "[async-lower][{import_prefix}-write-{index}]{func_name}"]
        fn start_write(_: u32, _: *const u8{start_extra}) -> u32;
    }}

    {lift_fn}
    {lower_fn}
    {dealloc_lists_fn}

    pub static VTABLE: {async_support}::{camel}Vtable<{name}> = {async_support}::{camel}Vtable::<{name}> {{
        cancel_write,
        cancel_read,
        drop_writable,
        drop_readable,
        {dealloc_lists_arg},
        layout: unsafe {{
            ::std::alloc::Layout::from_size_align_unchecked({size}, {align})
        }},
        {lift_arg},
        {lower_arg},
        new,
        start_read,
        start_write,
    }};

    impl super::{camel}Payload for {name} {{
        const VTABLE: &'static {async_support}::{camel}Vtable<Self> = &VTABLE;
    }}
}}
                        "#,
        );

        let map = match payload_for {
            PayloadFor::Future => &mut self.r#gen.future_payloads,
            PayloadFor::Stream => &mut self.r#gen.stream_payloads,
        };
        map.insert(name, code);
    }

    fn generate_guest_import(&mut self, func: &Function, interface: Option<&WorldKey>) {
        if self.r#gen.skip.contains(&func.name) {
            return;
        }

        self.generate_payloads("", func, interface);

        let async_ = self.r#gen.is_async(self.resolve, interface, func, true);
        let mut sig = FnSig {
            async_,
            ..Default::default()
        };
        if let Some(id) = func.kind.resource() {
            let name = self.resolve.types[id].name.as_ref().unwrap();
            let name = to_upper_camel_case(name);
            uwriteln!(self.src, "impl {name} {{");
            sig.use_item_name = true;
            sig.update_for_func(&func);
        }
        self.src.push_str("#[allow(unused_unsafe, clippy::all)]\n");
        let params = self.print_signature(func, async_, &sig);
        self.src.push_str("{\n");
        self.src.push_str("unsafe {\n");

        if async_ {
            self.generate_guest_import_body_async(&self.wasm_import_module, func, params);
        } else {
            self.generate_guest_import_body_sync(&self.wasm_import_module, func, params);
        }

        self.src.push_str("}\n");
        self.src.push_str("}\n");

        if func.kind.resource().is_some() {
            self.src.push_str("}\n");
        }
    }

    fn lower_to_memory(&mut self, address: &str, value: &str, ty: &Type, module: &str) -> String {
        let mut f = FunctionBindgen::new(self, Vec::new(), module, true);
        abi::lower_to_memory(f.r#gen.resolve, &mut f, address.into(), value.into(), ty);
        format!("unsafe {{ {} }}", String::from(f.src))
    }

    fn deallocate_lists(
        &mut self,
        types: &[Type],
        operands: &[String],
        indirect: bool,
        module: &str,
    ) -> String {
        let mut f = FunctionBindgen::new(self, Vec::new(), module, true);
        abi::deallocate_lists_in_types(f.r#gen.resolve, types, operands, indirect, &mut f);
        format!("unsafe {{ {} }}", String::from(f.src))
    }

    fn deallocate_lists_and_own(
        &mut self,
        types: &[Type],
        operands: &[String],
        indirect: bool,
        module: &str,
    ) -> String {
        let mut f = FunctionBindgen::new(self, Vec::new(), module, true);
        abi::deallocate_lists_and_own_in_types(f.r#gen.resolve, types, operands, indirect, &mut f);
        format!("unsafe {{ {} }}", String::from(f.src))
    }

    fn lift_from_memory(&mut self, address: &str, ty: &Type, module: &str) -> String {
        let mut f = FunctionBindgen::new(self, Vec::new(), module, true);
        let result = abi::lift_from_memory(f.r#gen.resolve, &mut f, address.into(), ty);
        format!("unsafe {{ {}\n{result} }}", String::from(f.src))
    }

    fn generate_guest_import_body_sync(
        &mut self,
        module: &str,
        func: &Function,
        params: Vec<String>,
    ) {
        let mut f = FunctionBindgen::new(self, params, module, false);
        abi::call(
            f.r#gen.resolve,
            AbiVariant::GuestImport,
            LiftLower::LowerArgsLiftResults,
            func,
            &mut f,
            false,
        );
        let FunctionBindgen {
            needs_cleanup_list,
            src,
            import_return_pointer_area_size,
            import_return_pointer_area_align,
            handle_decls,
            ..
        } = f;

        if needs_cleanup_list {
            let vec = self.path_to_vec();
            uwriteln!(self.src, "let mut cleanup_list = {vec}::new();");
        }
        assert!(handle_decls.is_empty());
        if !import_return_pointer_area_size.is_empty() {
            uwriteln!(self.src,);
            self.align_area(import_return_pointer_area_align);
            uwrite!(
                self.src,
                "struct RetArea([::core::mem::MaybeUninit::<u8>; {import_return_pointer_area_size}]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); {import_return_pointer_area_size}]);
", import_return_pointer_area_size = import_return_pointer_area_size.format_term(POINTER_SIZE_EXPRESSION, true)
            );
        }
        self.src.push_str(&String::from(src));
    }

    fn generate_guest_import_body_async(
        &mut self,
        module: &str,
        func: &Function,
        mut params: Vec<String>,
    ) {
        let param_tys = func.params.iter().map(|(_, ty)| *ty).collect::<Vec<_>>();
        let async_support = self.r#gen.async_support_path();
        let sig = self
            .resolve
            .wasm_signature(AbiVariant::GuestImportAsync, func);

        // Generate `type ParamsLower`
        //
        uwrite!(
            self.src,
            "
#[derive(Copy, Clone)]
struct ParamsLower(
            "
        );
        let mut params_lower = sig.params.as_slice();
        if sig.retptr {
            params_lower = &params_lower[..params_lower.len() - 1];
        }
        for ty in params_lower {
            self.src.push_str(wasm_type(*ty));
            self.src.push_str(", ");
        }
        uwriteln!(
            self.src,
            "
);
unsafe impl Send for ParamsLower {{}}
            "
        );

        uwriteln!(
            self.src,
            "
use {async_support}::Subtask as _Subtask;
struct _MySubtask<'a> {{ _unused: core::marker::PhantomData<&'a ()> }}
#[allow(unused_parens)]
unsafe impl<'a> _Subtask for _MySubtask<'a> {{
            "
        );

        // Generate `type Params`
        uwrite!(self.src, "type Params = (");
        for (_, ty) in func.params.iter() {
            let mode = self.type_mode_for(ty, TypeOwnershipStyle::Owned, "'a");
            self.print_ty(ty, mode);
            self.src.push_str(", ");
        }
        uwriteln!(self.src, ");");

        // Generate `type Results`
        match func.result {
            Some(ty) => {
                uwrite!(self.src, "type Results = ");
                self.print_ty(&ty, TypeMode::owned());
                uwriteln!(self.src, ";");
            }
            None => {
                uwriteln!(self.src, "type Results = ();");
            }
        }

        // Generate `type ParamsLower`
        uwrite!(self.src, "type ParamsLower = ParamsLower;");

        // Generate `const ABI_LAYOUT`
        let mut heap_types = Vec::new();
        if sig.indirect_params {
            heap_types.extend(param_tys.iter().cloned());
        }
        heap_types.extend(func.result);

        let layout = self.sizes.record(&heap_types);
        uwriteln!(
            self.src,
            r#"
fn abi_layout(&mut self) -> ::core::alloc::Layout {{
    unsafe {{
        ::core::alloc::Layout::from_size_align_unchecked({}, {})
    }}
}}
            "#,
            layout.size.format(POINTER_SIZE_EXPRESSION),
            layout.align.format(POINTER_SIZE_EXPRESSION),
        );

        // Generate `const RESULTS_OFFSET`
        let offset = match func.result {
            Some(_) => {
                let offsets = self.sizes.field_offsets(&heap_types);
                offsets.last().unwrap().0.format(POINTER_SIZE_EXPRESSION)
            }
            None => "0".to_string(),
        };
        uwriteln!(
            self.src,
            "fn results_offset(&mut self) -> usize {{ {offset} }}"
        );

        // Generate `fn call_import`
        let import_name = &func.name;
        let intrinsic = crate::declare_import(
            &module,
            &format!("[async-lower]{import_name}"),
            "call",
            &sig.params,
            &sig.results,
        );
        let mut args = String::new();
        for i in 0..params_lower.len() {
            args.push_str(&format!("_params.{i},"));
        }
        if func.result.is_some() {
            args.push_str("_results");
        }
        uwriteln!(
            self.src,
            r#"
unsafe fn call_import(&mut self, _params: Self::ParamsLower, _results: *mut u8) -> u32 {{
    {intrinsic}
    unsafe {{ call({args}) as u32 }}
}}
            "#
        );

        // Generate `fn params_dealloc_lists`
        let mut dealloc_lists_params = Vec::new();
        for i in 0..params_lower.len() {
            dealloc_lists_params.push(format!("_params.{i}"));
        }
        let dealloc_lists = self.deallocate_lists(
            &param_tys,
            &dealloc_lists_params,
            sig.indirect_params,
            module,
        );
        uwriteln!(
            self.src,
            "unsafe fn params_dealloc_lists(&mut self, _params: Self::ParamsLower) {{"
        );
        uwriteln!(self.src, "{dealloc_lists}");
        uwriteln!(self.src, "}}");

        // Generate `fn params_dealloc_lists_and_own`
        let dealloc_lists_and_own = self.deallocate_lists_and_own(
            &param_tys,
            &dealloc_lists_params,
            sig.indirect_params,
            module,
        );
        uwriteln!(
            self.src,
            "unsafe fn params_dealloc_lists_and_own(&mut self, _params: Self::ParamsLower) {{"
        );
        uwriteln!(self.src, "{dealloc_lists_and_own}");
        uwriteln!(self.src, "}}");

        // Generate `fn params_lower`
        let mut lowers = Vec::new();
        let mut param_lowers = Vec::new();
        if sig.indirect_params {
            let offsets = self
                .sizes
                .field_offsets(func.params.iter().map(|(_, ty)| ty));
            for (i, (offset, ty)) in offsets.into_iter().enumerate() {
                let name = format!("_lower{i}");
                let mut start = format!(
                    "let _param_ptr = unsafe {{ _ptr.add({}) }};\n",
                    offset.format(POINTER_SIZE_EXPRESSION)
                );
                let lower = self.lower_to_memory("_param_ptr", &name, ty, module);
                start.push_str(&lower);
                lowers.push(start);
                param_lowers.push(name);
            }
            lowers.push("ParamsLower(_ptr,)".to_string());
        } else {
            let mut f = FunctionBindgen::new(self, Vec::new(), module, true);
            let mut results = Vec::new();
            for (i, (_, ty)) in func.params.iter().enumerate() {
                let name = format!("_lower{i}");
                results.extend(abi::lower_flat(f.r#gen.resolve, &mut f, name.clone(), ty));
                param_lowers.push(name);
            }
            for result in results.iter_mut() {
                result.push_str(",");
            }
            let result = format!("ParamsLower({})", results.join(" "));
            lowers.push(format!("unsafe {{ {} {result} }}", String::from(f.src)));
        }

        for param in param_lowers.iter_mut() {
            param.push_str(",");
        }
        uwriteln!(
            self.src,
            "unsafe fn params_lower(&mut self, ({}): Self::Params, _ptr: *mut u8) -> Self::ParamsLower {{",
            param_lowers.join(" "),
        );
        for lower in lowers.iter() {
            uwriteln!(self.src, "{lower}");
        }
        uwriteln!(self.src, "}}");

        // Generate `fn results_lift`
        let lift = match func.result {
            Some(result) => self.lift_from_memory("_ptr", &result, module),
            None => String::new(),
        };
        uwriteln!(
            self.src,
            "unsafe fn results_lift(&mut self, _ptr: *mut u8) -> Self::Results {{"
        );
        uwriteln!(self.src, "{lift}");
        uwriteln!(self.src, "}}");

        uwriteln!(self.src, "}}");

        for param in params.iter_mut() {
            param.push_str(",");
        }
        uwriteln!(
            self.src,
            "_MySubtask {{ _unused: core::marker::PhantomData }}.call(({})).await",
            params.join(" ")
        );
    }

    fn generate_guest_export(
        &mut self,
        func: &Function,
        interface: Option<&WorldKey>,
        trait_name: &str,
        async_: bool,
    ) {
        let name_snake = func.name.to_snake_case().replace('.', "_");

        self.generate_payloads("[export]", func, interface);

        uwrite!(
            self.src,
            "\
                #[doc(hidden)]
                #[allow(non_snake_case, unused_unsafe)]
                pub unsafe fn _export_{name_snake}_cabi<T: {trait_name}>\
            ",
        );
        let params = self.print_export_sig(func, async_);
        self.push_str(" { unsafe {");

        if !self.r#gen.opts.disable_run_ctors_once_workaround {
            let run_ctors_once = self.path_to_run_ctors_once();
            // Before executing any other code, use this function to run all
            // static constructors, if they have not yet been run. This is a
            // hack required to work around wasi-libc ctors calling import
            // functions to initialize the environment.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            uwrite!(
                self.src,
                "#[cfg(target_arch=\"wasm32\")]\n{run_ctors_once}();",
            );
        }

        let mut f = FunctionBindgen::new(self, params, self.wasm_import_module, false);
        let variant = if async_ {
            AbiVariant::GuestExportAsync
        } else {
            AbiVariant::GuestExport
        };
        abi::call(
            f.r#gen.resolve,
            variant,
            LiftLower::LiftArgsLowerResults,
            func,
            &mut f,
            async_,
        );
        let FunctionBindgen {
            needs_cleanup_list,
            src,
            handle_decls,
            ..
        } = f;
        if async_ {
            let async_support = self.r#gen.async_support_path();
            uwriteln!(self.src, "{async_support}::start_task(async move {{");
            uwriteln!(
                self.src,
                "let _task_cancel = {async_support}::TaskCancelOnDrop::new();"
            );
            if needs_cleanup_list {
                let vec = self.path_to_vec();
                uwriteln!(self.src, "let mut cleanup_list = {vec}::new();");
            }
        } else {
            assert!(!needs_cleanup_list);
        }
        for decl in handle_decls {
            self.src.push_str(&decl);
            self.src.push_str("\n");
        }
        self.src.push_str(&String::from(src));
        if async_ {
            uwriteln!(self.src, "}})")
        }
        self.src.push_str("} }\n");

        if async_ {
            let async_support = self.r#gen.async_support_path();
            uwrite!(
                self.src,
                "\
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    pub unsafe fn __callback_{name_snake}(event0: u32, event1: u32, event2: u32) -> u32 {{
                        unsafe {{
                            {async_support}::callback(event0, event1, event2)
                        }}
                    }}
                "
            );
        } else if abi::guest_export_needs_post_return(self.resolve, func) {
            uwrite!(
                self.src,
                "\
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    pub unsafe fn __post_return_{name_snake}<T: {trait_name}>\
                "
            );
            let params = self.print_post_return_sig(func);
            self.src.push_str("{ unsafe {\n");

            let mut f = FunctionBindgen::new(self, params, self.wasm_import_module, false);
            abi::post_return(f.r#gen.resolve, func, &mut f);
            let FunctionBindgen {
                needs_cleanup_list,
                src,
                handle_decls,
                ..
            } = f;
            assert!(!needs_cleanup_list);
            assert!(handle_decls.is_empty());
            self.src.push_str(&String::from(src));
            self.src.push_str("} }\n");
        }
    }

    fn generate_raw_cabi_export(
        &mut self,
        func: &Function,
        ty: &str,
        path_to_self: &str,
        async_: bool,
    ) {
        let name_snake = func.name.to_snake_case().replace('.', "_");
        let wasm_module_export_name = match self.identifier {
            Identifier::Interface(_, key) => Some(self.resolve.name_world_key(key)),
            Identifier::World(_) => None,
            Identifier::StreamOrFuturePayload => unreachable!(),
        };
        let export_prefix = self.r#gen.opts.export_prefix.as_deref().unwrap_or("");
        let export_name = func.legacy_core_export_name(wasm_module_export_name.as_deref());
        let export_name = if async_ {
            format!("[async-lift]{export_name}")
        } else {
            export_name.to_string()
        };
        uwrite!(
            self.src,
            "\
                #[unsafe(export_name = \"{export_prefix}{export_name}\")]
                unsafe extern \"C\" fn export_{name_snake}\
",
        );

        let params = self.print_export_sig(func, async_);
        self.push_str(" {\n");
        uwriteln!(
            self.src,
            "unsafe {{ {path_to_self}::_export_{name_snake}_cabi::<{ty}>({}) }}",
            params.join(", ")
        );
        self.push_str("}\n");

        let export_prefix = self.r#gen.opts.export_prefix.as_deref().unwrap_or("");
        if async_ {
            uwrite!(
                self.src,
                "\
                    #[unsafe(export_name = \"{export_prefix}[callback]{export_name}\")]
                    unsafe extern \"C\" fn _callback_{name_snake}(event0: u32, event1: u32, event2: u32) -> u32 {{
                        unsafe {{
                            {path_to_self}::__callback_{name_snake}(event0, event1, event2)
                        }}
                    }}
                "
            );
        } else if abi::guest_export_needs_post_return(self.resolve, func) {
            uwrite!(
                self.src,
                "\
                    #[unsafe(export_name = \"{export_prefix}cabi_post_{export_name}\")]
                    unsafe extern \"C\" fn _post_return_{name_snake}\
"
            );
            let params = self.print_post_return_sig(func);
            self.src.push_str("{\n");
            uwriteln!(
                self.src,
                "unsafe {{ {path_to_self}::__post_return_{name_snake}::<{ty}>({}) }}",
                params.join(", ")
            );
            self.src.push_str("}\n");
        }
    }

    fn print_export_sig(&mut self, func: &Function, async_: bool) -> Vec<String> {
        self.src.push_str("(");
        let variant = if async_ {
            AbiVariant::GuestExportAsync
        } else {
            AbiVariant::GuestExport
        };
        let sig = self.resolve.wasm_signature(variant, func);
        let mut params = Vec::new();
        for (i, param) in sig.params.iter().enumerate() {
            let name = format!("arg{i}");
            uwrite!(self.src, "{name}: {},", wasm_type(*param));
            params.push(name);
        }
        self.src.push_str(")");

        match sig.results.len() {
            0 => {}
            1 => {
                uwrite!(self.src, " -> {}", wasm_type(sig.results[0]));
            }
            _ => unimplemented!(),
        }

        params
    }

    fn print_post_return_sig(&mut self, func: &Function) -> Vec<String> {
        self.src.push_str("(");
        let mut params = Vec::new();
        let sig = self.resolve.wasm_signature(AbiVariant::GuestExport, func);
        for (i, result) in sig.results.iter().enumerate() {
            let name = format!("arg{i}");
            uwrite!(self.src, "{name}: {},", wasm_type(*result));
            params.push(name);
        }
        self.src.push_str(") ");
        params
    }

    pub fn generate_stub<'a>(
        &mut self,
        interface: Option<(InterfaceId, &WorldKey)>,
        funcs: impl Iterator<Item = &'a Function> + Clone,
    ) {
        let mut funcs = super::group_by_resource(funcs.clone());

        let root_methods = funcs.remove(&None).unwrap_or(Vec::new());

        let mut extra_trait_items = String::new();
        let guest_trait = match interface {
            Some((id, _)) => {
                let path = self.path_to_interface(id).unwrap();
                for (name, id) in self.resolve.interfaces[id].types.iter() {
                    match self.resolve.types[*id].kind {
                        TypeDefKind::Resource => {}
                        _ => continue,
                    }
                    let camel = name.to_upper_camel_case();
                    uwriteln!(extra_trait_items, "type {camel} = Stub;");

                    let resource_methods = funcs.remove(&Some(*id)).unwrap_or(Vec::new());
                    let trait_name = format!("{path}::Guest{camel}");
                    self.generate_stub_impl(&trait_name, "", &resource_methods, interface);
                }
                format!("{path}::Guest")
            }
            None => {
                assert!(funcs.is_empty());
                "Guest".to_string()
            }
        };

        if !root_methods.is_empty() || !extra_trait_items.is_empty() {
            self.generate_stub_impl(&guest_trait, &extra_trait_items, &root_methods, interface);
        }
    }

    fn generate_stub_impl(
        &mut self,
        trait_name: &str,
        extra_trait_items: &str,
        funcs: &[&Function],
        interface: Option<(InterfaceId, &WorldKey)>,
    ) {
        uwriteln!(self.src, "impl {trait_name} for Stub {{");
        self.src.push_str(extra_trait_items);

        for func in funcs {
            if self.r#gen.skip.contains(&func.name) {
                continue;
            }
            let async_ = self
                .r#gen
                .is_async(self.resolve, interface.map(|p| p.1), func, false);
            let mut sig = FnSig {
                async_,
                use_item_name: true,
                private: true,
                ..Default::default()
            };
            sig.update_for_func(&func);
            self.src.push_str("#[allow(unused_variables)]\n");
            self.print_signature(func, true, &sig);
            self.src.push_str("{ unreachable!() }\n");
        }

        self.src.push_str("}\n");
    }

    fn rustdoc(&mut self, docs: &Docs) {
        let docs = match &docs.contents {
            Some(docs) => docs,
            None => return,
        };
        for line in docs.trim().lines() {
            self.push_str("///");
            if !line.is_empty() {
                self.push_str(" ");
                self.push_str(line);
            }
            self.push_str("\n");
        }
    }

    fn rustdoc_params(&mut self, docs: &[(String, Type)], header: &str) {
        let _ = (docs, header);
        // let docs = docs
        //     .iter()
        //     .filter(|param| param.docs.trim().len() > 0)
        //     .collect::<Vec<_>>();
        // if docs.len() == 0 {
        //     return;
        // }

        // self.push_str("///\n");
        // self.push_str("/// ## ");
        // self.push_str(header);
        // self.push_str("\n");
        // self.push_str("///\n");

        // for param in docs {
        //     for (i, line) in param.docs.lines().enumerate() {
        //         self.push_str("/// ");
        //         // Currently wasi only has at most one return value, so there's no
        //         // need to indent it or name it.
        //         if header != "Return" {
        //             if i == 0 {
        //                 self.push_str("* `");
        //                 self.push_str(to_rust_ident(param.name.as_str()));
        //                 self.push_str("` - ");
        //             } else {
        //                 self.push_str("  ");
        //             }
        //         }
        //         self.push_str(line);
        //         self.push_str("\n");
        //     }
        // }
    }

    fn print_signature(&mut self, func: &Function, params_owned: bool, sig: &FnSig) -> Vec<String> {
        let params = self.print_docs_and_params(func, params_owned, sig);
        self.push_str(" -> ");
        if let FunctionKind::Constructor(resource_id) = &func.kind {
            match classify_constructor_return_type(&self.resolve, *resource_id, &func.result) {
                ConstructorReturnType::Self_ => {
                    self.push_str("Self");
                }
                ConstructorReturnType::Result { err } => {
                    self.push_str("Result<Self, ");
                    self.print_result_type(&err);
                    self.push_str("> where Self: Sized");
                }
            }
        } else {
            self.print_result_type(&func.result);
        }
        params
    }

    fn print_docs_and_params(
        &mut self,
        func: &Function,
        params_owned: bool,
        sig: &FnSig,
    ) -> Vec<String> {
        self.rustdoc(&func.docs);
        self.rustdoc_params(&func.params, "Parameters");
        // TODO: re-add this when docs are back
        // self.rustdoc_params(&func.results, "Return");

        // TODO: should probably return `impl Future` in traits instead of
        // having an `async fn` but that'll require more plumbing here.
        self.push_str("#[allow(async_fn_in_trait)]\n");

        if !sig.private {
            self.push_str("pub ");
        }
        if sig.unsafe_ {
            self.push_str("unsafe ");
        }
        if sig.async_ {
            self.push_str("async ");
        }
        self.push_str("fn ");
        let func_name = if sig.use_item_name {
            if let FunctionKind::Constructor(_) = &func.kind {
                "new"
            } else {
                func.item_name()
            }
        } else {
            func.item_name()
        };
        self.push_str(&to_rust_ident(func_name));
        if let Some(generics) = &sig.generics {
            self.push_str(generics);
        }
        self.push_str("(");
        if let Some(arg) = &sig.self_arg {
            self.push_str(arg);
            self.push_str(",");
        }
        let mut params = Vec::new();
        for (i, (name, param)) in func.params.iter().enumerate() {
            if i == 0 && sig.self_is_first_param {
                params.push("self".to_string());
                continue;
            }
            let name = to_rust_ident(name);
            self.push_str(&name);
            self.push_str(": ");

            // Select the "style" of mode that the parameter's type will be
            // rendered as. Owned parameters are always owned, that's the easy
            // case. Otherwise it means that we're rendering the arguments to an
            // imported function which technically don't need ownership. In this
            // case the `ownership` configuration is consulted.
            //
            // If `Owning` is specified then that means that the top-level
            // argument will be `&T` but everything under that will be `T`. For
            // example a record-of-lists would be passed as `&RecordOfLists` as
            // opposed to `RecordOfLists<'a>`.
            //
            // In the `Borrowing` mode however a different tradeoff is made. The
            // types are generated differently meaning that a borrowed version
            // is used.
            let style = if params_owned {
                TypeOwnershipStyle::Owned
            } else {
                match self.r#gen.opts.ownership {
                    Ownership::Owning => TypeOwnershipStyle::OnlyTopBorrowed,
                    Ownership::Borrowing { .. } => TypeOwnershipStyle::Borrowed,
                }
            };
            let mode = self.type_mode_for(param, style, "'_");
            self.print_ty(param, mode);
            self.push_str(",");

            // Depending on the style of this request vs what we got perhaps
            // change how this argument is used.
            //
            // If the `mode` that was selected matches the requested style, then
            // everything is as expected and the argument should be used as-is.
            // If it differs though then that means that we requested a borrowed
            // mode but a different mode ended up being selected. This situation
            // indicates for example that an argument to an import should be
            // borrowed but the argument's type means that it can't be borrowed.
            // For example all arguments to imports are borrowed by default but
            // owned resources cannot ever be borrowed, so they pop out here as
            // owned instead.
            //
            // In such a situation the lower code still expects to be operating
            // over borrows. For example raw pointers from lists are passed to
            // the canonical ABI layer assuming that the lists are "rooted" by
            // the caller. To uphold this invariant a borrow of the argument is
            // recorded as the name of this parameter. That ensures that all
            // access to the parameter is done indirectly which pretends, at
            // least internally, that the argument was borrowed. The original
            // motivation for this was #817.
            if mode.style == style {
                params.push(name);
            } else {
                assert!(style != TypeOwnershipStyle::Owned);
                params.push(format!("&{name}"));
            }
        }
        self.push_str(")");
        params
    }

    fn print_result_type(&mut self, result: &Option<Type>) {
        match result {
            None => {
                self.push_str("()");
            }
            Some(ty) => {
                let mode = self.type_mode_for(ty, TypeOwnershipStyle::Owned, "'INVALID");
                assert!(mode.lifetime.is_none());
                self.print_ty(ty, mode);
            }
        }
    }

    /// Calculates the `TypeMode` to be used for the `ty` specified.
    ///
    /// This takes a `style` argument which is the requested style of ownership
    /// for this type. Note that the returned `TypeMode` may have a different
    /// `style`.
    ///
    /// This additionally takes a `lt` parameter which, if needed, is what will
    /// be used to render lifetimes.
    fn type_mode_for(&self, ty: &Type, style: TypeOwnershipStyle, lt: &'static str) -> TypeMode {
        match ty {
            Type::Id(id) => self.type_mode_for_id(*id, style, lt),

            // Borrowed strings are handled specially here since they're the
            // only list-like primitive.
            Type::String if style != TypeOwnershipStyle::Owned => TypeMode {
                lifetime: Some(lt),
                lists_borrowed: true,
                style,
            },

            _ => TypeMode::owned(),
        }
    }

    /// Same as `type_mode_for`, but specifically for `TypeId` which refers to a
    /// type.
    fn type_mode_for_id(
        &self,
        ty: TypeId,
        style: TypeOwnershipStyle,
        lt: &'static str,
    ) -> TypeMode {
        // NB: This method is the heart of determining how to render types.
        // There's a lot of permutations and corner cases to handle, especially
        // with being able to configure at the generator level how types are
        // generated. Long story short this is a subtle and complicated method.
        //
        // The hope is that most of the complexity around type generation in
        // Rust is largely centered here where everything else can lean on this.
        // This has gone through so many refactors I've lost count at this
        // point, but maybe this one is the one that'll stick!
        //
        // The general idea is that there's some clear-and-fast rules for how
        // `TypeMode` must be returned here. For example borrowed handles are
        // required to have a lifetime parameter. Everything else though is here
        // to handle the various levels of configuration and semantics for each
        // level of types.
        //
        // As a reminder a `TypeMode` is generated for each "level" of a type
        // hierarchy, for example there's one mode for `Vec<T>` and another mode
        // for `T`. This enables, for example, rendering the outer layer as
        // either `Vec<T>` or `&[T]` but the inner `T` may or may not have a
        // type parameter.

        let info = self.info(ty);
        let lifetime = if info.has_borrow_handle {
            // Borrowed handles always have a lifetime associated with them so
            // thread it through.
            Some(lt)
        } else if style == TypeOwnershipStyle::Owned {
            // If this type is being rendered as an "owned" type, and it
            // doesn't have any borrowed handles, then no lifetimes are needed
            // since any internal lists will be their owned version.
            None
        } else if info.has_own_handle || !info.has_list {
            // At this point there are no borrowed handles and a borrowed style
            // of type is requested. In this situation there's two cases where a
            // lifetime is never used:
            //
            // * Owned handles are present - in this situation ownership is used
            //   to statically reflect how a value is lost when passed to an
            //   import. This means that no lifetime is used for internal lists
            //   since they must be rendered in an owned mode.
            //
            // * There are no lists present - here the lifetime parameter won't
            //   be used for anything because there's no borrows or lists, so
            //   it's skipped.
            None
        } else if !info.owned || self.uses_two_names(&info) {
            // This next layer things get a little more interesting. To recap,
            // so far we know that there's no borrowed handles, a borrowed mode
            // is requested, there's no own handles, and there's a list. In that
            // situation if `info` shows that this type is never used in an
            // owned position, or if two types are explicitly requested for
            // owned/borrowed values, then a lifetime is used.
            Some(lt)
        } else {
            // ... and finally, here at the end we know:
            //
            // * No borrowed handles
            // * Borrowing mode is requested
            // * No owned handles
            // * A list is somewhere
            // * This type is used somewhere in an owned position
            // * This type does not used "two names" meaning that we must use
            //   the owned version of the type.
            //
            // If the configured ownership mode for generating types of this
            // generator is "owning" then that's the only type that can be used.
            // If borrowing is requested then this means that `&T` is going to
            // be rendered, so thread it through.
            //
            // If the configured ownership mode uses borrowing by default, then
            // things get a little weird. This means that a lifetime is going to
            // be used an any lists should be borrowed, but we specifically
            // switch to only borrowing the top layer of the type rather than
            // the entire hierarchy. This situation can happen in
            // `duplicate_if_necessary: false` mode for example where we're
            // borrowing a type which is used in an owned position elsewhere.
            // The only possibility at that point is to borrow it at the root
            // but everything else internally is required to be owned from then
            // on.
            match self.r#gen.opts.ownership {
                Ownership::Owning => Some(lt),
                Ownership::Borrowing { .. } => {
                    return TypeMode {
                        lifetime: Some(lt),
                        lists_borrowed: true,
                        style: TypeOwnershipStyle::OnlyTopBorrowed,
                    };
                }
            }
        };
        TypeMode {
            lifetime,

            // If a lifetime is present and ownership isn't requested, then make
            // sure any lists show up as `&str` or `&[T]`.
            lists_borrowed: lifetime.is_some() && style != TypeOwnershipStyle::Owned,

            // Switch the style to `Owned` if an `own<T>` handle is present
            // because there's no option but to take interior types by ownership
            // as that statically shows that the ownership of the value is being
            // lost.
            style: if info.has_own_handle {
                TypeOwnershipStyle::Owned
            } else {
                style
            },
        }
    }

    /// Generates the "next" mode for a type.
    ///
    /// The `ty` specified is the type that a mode is being generated for, and
    /// the `mode` argument is the "parent" mode that the previous outer layer
    /// of type was rendered with. The returned mode should be used to render
    /// `ty`.
    fn filter_mode(&self, ty: &Type, mode: TypeMode) -> TypeMode {
        match mode.lifetime {
            Some(lt) => self.type_mode_for(ty, mode.style.next(), lt),
            None => TypeMode::owned(),
        }
    }

    /// Same as `filder_mode` except if `mode` has the type `OnlyTopBorrowed`
    /// the `mode` is specifically preserved as-is.
    ///
    /// This is used for types like `Option<T>` to render as `Option<&T>`
    /// instead of `&Option<T>` for example.
    fn filter_mode_preserve_top(&self, ty: &Type, mode: TypeMode) -> TypeMode {
        if mode.style == TypeOwnershipStyle::OnlyTopBorrowed {
            mode
        } else {
            self.filter_mode(ty, mode)
        }
    }

    pub(crate) fn type_name_owned_with_id(&mut self, ty: &Type, id: Identifier<'i>) -> String {
        let old_identifier = mem::replace(&mut self.identifier, id);
        let name = self.type_name_owned(ty);
        self.identifier = old_identifier;
        name
    }

    fn type_name_owned(&mut self, ty: &Type) -> String {
        self.type_name(
            ty,
            TypeMode {
                lifetime: None,
                lists_borrowed: false,
                style: TypeOwnershipStyle::Owned,
            },
        )
    }

    fn type_name(&mut self, ty: &Type, mode: TypeMode) -> String {
        let old = mem::take(&mut self.src);
        self.print_ty(ty, mode);
        String::from(mem::replace(&mut self.src, old))
    }

    fn print_ty(&mut self, ty: &Type, mode: TypeMode) {
        // If we have a typedef of a string or a list, the typedef is an alias
        // for `String` or `Vec<T>`. If this is a borrow, instead of borrowing
        // them as `&String` or `&Vec<T>`, use `&str` or `&[T]` so that callers
        // don't need to create owned copies.
        if let Type::Id(id) = ty {
            let id = dealias(self.resolve, *id);
            let typedef = &self.resolve.types[id];
            match &typedef.kind {
                TypeDefKind::Type(Type::String) => {
                    if let Some(lt) = mode.lifetime {
                        self.print_borrowed_str(lt);
                        return;
                    }
                }
                TypeDefKind::List(element) => {
                    if mode.lifetime.is_some() {
                        self.print_list(element, mode);
                        return;
                    }
                }
                _ => {}
            }
        }

        match ty {
            Type::Id(t) => self.print_tyid(*t, mode),
            Type::Bool => self.push_str("bool"),
            Type::U8 => self.push_str("u8"),
            Type::U16 => self.push_str("u16"),
            Type::U32 => self.push_str("u32"),
            Type::U64 => self.push_str("u64"),
            Type::S8 => self.push_str("i8"),
            Type::S16 => self.push_str("i16"),
            Type::S32 => self.push_str("i32"),
            Type::S64 => self.push_str("i64"),
            Type::F32 => self.push_str("f32"),
            Type::F64 => self.push_str("f64"),
            Type::Char => self.push_str("char"),
            Type::String => {
                assert_eq!(mode.lists_borrowed, mode.lifetime.is_some());
                match mode.lifetime {
                    Some(lt) => self.print_borrowed_str(lt),
                    None => {
                        if self.r#gen.opts.raw_strings {
                            self.push_vec_name();
                            self.push_str("::<u8>");
                        } else {
                            self.push_string_name();
                        }
                    }
                }
            }

            Type::ErrorContext => {
                let async_support = self.r#gen.async_support_path();
                self.push_str(&format!("{async_support}::ErrorContext"));
            }
        }
    }

    fn print_optional_ty(&mut self, ty: Option<&Type>, mode: TypeMode) {
        match ty {
            Some(ty) => {
                let mode = self.filter_mode_preserve_top(ty, mode);
                self.print_ty(ty, mode)
            }
            None => self.push_str("()"),
        }
    }

    pub fn type_path(&self, id: TypeId, owned: bool) -> String {
        let full_wit_type_name = full_wit_type_name(self.resolve, id);
        if let Some(TypeGeneration::Remap(remapped_path)) = self.r#gen.with.get(&full_wit_type_name)
        {
            remapped_path.clone()
        } else {
            self.type_path_with_name(
                id,
                if owned {
                    self.result_name(id)
                } else {
                    self.param_name(id)
                },
            )
        }
    }

    fn type_path_with_name(&self, id: TypeId, name: String) -> String {
        if let TypeOwner::Interface(id) = self.resolve.types[id].owner {
            if let Some(path) = self.path_to_interface(id) {
                return format!("{path}::{name}");
            }
        }
        name
    }

    fn print_tyid(&mut self, id: TypeId, mode: TypeMode) {
        let ty = &self.resolve.types[id];
        if ty.name.is_some() {
            // NB: Most of the heavy lifting of `TypeMode` and what to do here
            // has already happened in `type_mode_for*`. Here though a little
            // more happens because this is where `OnlyTopBorrowed` is
            // processed.
            //
            // Specifically what should happen is that in the case of an
            // argument to an imported function if only the top value is
            // borrowed then we want to render it as `&T`. If this all is
            // applicable then the lifetime is rendered here before the type.
            // The `mode` is then switched to `Owned` and recalculated for the
            // type we're rendering here to avoid accidentally giving it a
            // lifetime type parameter when it otherwise doesn't have it.
            let mode = if mode.style == TypeOwnershipStyle::OnlyTopBorrowed {
                if let Some(lt) = mode.lifetime {
                    self.push_str("&");
                    if lt != "'_" {
                        self.push_str(lt);
                        self.push_str(" ");
                    }
                    self.type_mode_for_id(id, TypeOwnershipStyle::Owned, lt)
                } else {
                    mode
                }
            } else {
                mode
            };
            let name = self.type_path(
                id,
                match mode.style {
                    TypeOwnershipStyle::Owned => true,
                    TypeOwnershipStyle::OnlyTopBorrowed | TypeOwnershipStyle::Borrowed => false,
                },
            );
            self.push_str(&name);
            self.print_generics(mode.lifetime);
            return;
        }

        let mut anonymous_type_gen = AnonTypeGenerator {
            mode,
            resolve: self.resolve,
            interface: self,
        };
        anonymous_type_gen.define_anonymous_type(id);
    }

    fn print_list(&mut self, ty: &Type, mode: TypeMode) {
        let next_mode = self.filter_mode(ty, mode);
        if mode.lists_borrowed {
            let lifetime = mode.lifetime.unwrap();
            self.push_str("&");
            if lifetime != "'_" {
                self.push_str(lifetime);
                self.push_str(" ");
            }
            self.push_str("[");
            self.print_ty(ty, next_mode);
            self.push_str("]");
        } else {
            self.push_vec_name();
            self.push_str("::<");
            self.print_ty(ty, next_mode);
            self.push_str(">");
        }
    }

    fn print_generics(&mut self, lifetime: Option<&str>) {
        if lifetime.is_none() {
            return;
        }
        self.push_str("<");
        if let Some(lt) = lifetime {
            self.push_str(lt);
            self.push_str(",");
        }
        self.push_str(">");
    }

    fn int_repr(&mut self, repr: Int) {
        self.push_str(int_repr(repr));
    }

    fn modes_of(&self, ty: TypeId) -> Vec<(String, TypeMode)> {
        let info = self.info(ty);
        let mut result = Vec::new();
        if !self.r#gen.opts.generate_unused_types {
            // If this type isn't actually used, no need to generate it.
            if !info.owned && !info.borrowed {
                return result;
            }
        }
        // Generate one mode for when the type is owned and another for when
        // it's borrowed.
        let a = self.type_mode_for_id(ty, TypeOwnershipStyle::Owned, "'a");
        let b = self.type_mode_for_id(ty, TypeOwnershipStyle::Borrowed, "'a");

        if self.uses_two_names(&info) {
            // If this type uses two names then, well, it uses two names. In
            // this situation both modes are returned.
            assert!(a != b);
            result.push((self.result_name(ty), a));
            result.push((self.param_name(ty), b));
        } else if a == b {
            // If the modes are the same then there's only one result.
            result.push((self.result_name(ty), a));
        } else if info.owned || matches!(self.r#gen.opts.ownership, Ownership::Owning) {
            // If this type is owned or if ownership is preferred then the owned
            // variant is used as a priority. This is where the generator's
            // configuration comes into play.
            result.push((self.result_name(ty), a));
        } else {
            // And finally, failing all that, the borrowed variant is used.
            assert!(!info.owned);
            result.push((self.param_name(ty), b));
        }
        result
    }

    fn print_typedef_record(&mut self, id: TypeId, record: &Record, docs: &Docs) {
        let info = self.info(id);
        // We use a BTree set to make sure we don't have any duplicates and we have a stable order
        let additional_derives: BTreeSet<String> = self
            .r#gen
            .opts
            .additional_derive_attributes
            .iter()
            .cloned()
            .collect();
        for (name, mode) in self.modes_of(id) {
            self.rustdoc(docs);
            let mut derives = BTreeSet::new();
            if !self
                .r#gen
                .opts
                .additional_derive_ignore
                .contains(&name.to_kebab_case())
            {
                derives.extend(additional_derives.clone());
            }
            if info.is_copy() {
                self.push_str("#[repr(C)]\n");
                derives.extend(["Copy", "Clone"].into_iter().map(|s| s.to_string()));
            } else if info.is_clone() {
                derives.insert("Clone".to_string());
            }
            if !derives.is_empty() {
                self.push_str("#[derive(");
                self.push_str(&derives.into_iter().collect::<Vec<_>>().join(", "));
                self.push_str(")]\n")
            }
            self.push_str(&format!("pub struct {name}"));
            self.print_generics(mode.lifetime);
            self.push_str(" {\n");
            for field in record.fields.iter() {
                self.rustdoc(&field.docs);
                self.push_str("pub ");
                self.push_str(&to_rust_ident(&field.name));
                self.push_str(": ");
                let mode = self.filter_mode(&field.ty, mode);
                self.print_ty(&field.ty, mode);
                self.push_str(",\n");
            }
            self.push_str("}\n");

            self.push_str("impl");
            self.print_generics(mode.lifetime);
            self.push_str(" ::core::fmt::Debug for ");
            self.push_str(&name);
            self.print_generics(mode.lifetime);
            self.push_str(" {\n");
            self.push_str(
                "fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {\n",
            );
            self.push_str(&format!("f.debug_struct(\"{name}\")"));
            for field in record.fields.iter() {
                self.push_str(&format!(
                    ".field(\"{}\", &self.{})",
                    field.name,
                    to_rust_ident(&field.name)
                ));
            }
            self.push_str(".finish()\n");
            self.push_str("}\n");
            self.push_str("}\n");

            if info.error {
                self.push_str("impl");
                self.print_generics(mode.lifetime);
                self.push_str(" ::core::fmt::Display for ");
                self.push_str(&name);
                self.print_generics(mode.lifetime);
                self.push_str(" {\n");
                self.push_str(
                    "fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {\n",
                );
                self.push_str("write!(f, \"{:?}\", self)\n");
                self.push_str("}\n");
                self.push_str("}\n");
                if self.r#gen.opts.std_feature {
                    self.push_str("#[cfg(feature = \"std\")]\n");
                }
                self.push_str("impl ::core::error::Error for ");
                self.push_str(&name);
                self.push_str(" {}\n");
            }
        }
    }

    fn print_typedef_variant(&mut self, id: TypeId, variant: &Variant, docs: &Docs)
    where
        Self: Sized,
    {
        self.print_rust_enum(
            id,
            variant
                .cases
                .iter()
                .map(|c| (c.name.to_upper_camel_case(), &c.docs, c.ty.as_ref())),
            docs,
        );
    }

    fn print_rust_enum<'b>(
        &mut self,
        id: TypeId,
        cases: impl IntoIterator<Item = (String, &'b Docs, Option<&'b Type>)> + Clone,
        docs: &Docs,
    ) where
        Self: Sized,
    {
        let info = self.info(id);
        // We use a BTree set to make sure we don't have any duplicates and have a stable order
        let additional_derives: BTreeSet<String> = self
            .r#gen
            .opts
            .additional_derive_attributes
            .iter()
            .cloned()
            .collect();
        for (name, mode) in self.modes_of(id) {
            self.rustdoc(docs);
            let mut derives = BTreeSet::new();
            if !self
                .r#gen
                .opts
                .additional_derive_ignore
                .contains(&name.to_kebab_case())
            {
                derives.extend(additional_derives.clone());
            }
            if info.is_copy() {
                derives.extend(["Copy", "Clone"].into_iter().map(|s| s.to_string()));
            } else if info.is_clone() {
                derives.insert("Clone".to_string());
            }
            if !derives.is_empty() {
                self.push_str("#[derive(");
                self.push_str(&derives.into_iter().collect::<Vec<_>>().join(", "));
                self.push_str(")]\n")
            }
            self.push_str(&format!("pub enum {name}"));
            self.print_generics(mode.lifetime);
            self.push_str(" {\n");
            for (case_name, docs, payload) in cases.clone() {
                self.rustdoc(docs);
                self.push_str(&case_name);
                if let Some(ty) = payload {
                    self.push_str("(");
                    let mode = self.filter_mode(ty, mode);
                    self.print_ty(ty, mode);
                    self.push_str(")")
                }
                self.push_str(",\n");
            }
            self.push_str("}\n");

            self.print_rust_enum_debug(
                mode,
                &name,
                cases
                    .clone()
                    .into_iter()
                    .map(|(name, _docs, ty)| (name, ty)),
            );

            if info.error {
                self.push_str("impl");
                self.print_generics(mode.lifetime);
                self.push_str(" ::core::fmt::Display for ");
                self.push_str(&name);
                self.print_generics(mode.lifetime);
                self.push_str(" {\n");
                self.push_str(
                    "fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {\n",
                );
                self.push_str("write!(f, \"{:?}\", self)\n");
                self.push_str("}\n");
                self.push_str("}\n");
                self.push_str("\n");

                if self.r#gen.opts.std_feature {
                    self.push_str("#[cfg(feature = \"std\")]\n");
                }
                self.push_str("impl");
                self.print_generics(mode.lifetime);
                self.push_str(" ::core::error::Error for ");
                self.push_str(&name);
                self.print_generics(mode.lifetime);
                self.push_str(" {}\n");
            }
        }
    }

    fn print_rust_enum_debug<'b>(
        &mut self,
        mode: TypeMode,
        name: &str,
        cases: impl IntoIterator<Item = (String, Option<&'b Type>)>,
    ) where
        Self: Sized,
    {
        self.push_str("impl");
        self.print_generics(mode.lifetime);
        self.push_str(" ::core::fmt::Debug for ");
        self.push_str(name);
        self.print_generics(mode.lifetime);
        self.push_str(" {\n");
        self.push_str(
            "fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {\n",
        );
        self.push_str("match self {\n");
        for (case_name, payload) in cases {
            self.push_str(name);
            self.push_str("::");
            self.push_str(&case_name);
            if payload.is_some() {
                self.push_str("(e)");
            }
            self.push_str(" => {\n");
            self.push_str(&format!("f.debug_tuple(\"{name}::{case_name}\")"));
            if payload.is_some() {
                self.push_str(".field(e)");
            }
            self.push_str(".finish()\n");
            self.push_str("}\n");
        }
        self.push_str("}\n");
        self.push_str("}\n");
        self.push_str("}\n");
    }

    fn print_typedef_option(&mut self, id: TypeId, payload: &Type, docs: &Docs) {
        for (name, mode) in self.modes_of(id) {
            self.rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(mode.lifetime);
            self.push_str("= Option<");
            self.print_ty(payload, mode);
            self.push_str(">;\n");
        }
    }

    fn print_typedef_result(&mut self, id: TypeId, result: &Result_, docs: &Docs) {
        for (name, mode) in self.modes_of(id) {
            self.rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(mode.lifetime);
            self.push_str("= Result<");
            self.print_optional_ty(result.ok.as_ref(), mode);
            self.push_str(",");
            self.print_optional_ty(result.err.as_ref(), mode);
            self.push_str(">;\n");
        }
    }

    fn print_typedef_enum(
        &mut self,
        id: TypeId,
        name: &str,
        enum_: &Enum,
        docs: &Docs,
        attrs: &[String],
        case_attr: Box<dyn Fn(&EnumCase) -> String>,
    ) where
        Self: Sized,
    {
        let info = self.info(id);

        let name = to_upper_camel_case(name);
        self.rustdoc(docs);
        for attr in attrs {
            self.push_str(&format!("{attr}\n"));
        }
        self.push_str("#[repr(");
        self.int_repr(enum_.tag());
        self.push_str(")]\n");
        // We use a BTree set to make sure we don't have any duplicates and a stable order
        let mut derives: BTreeSet<String> = BTreeSet::new();
        if !self
            .r#gen
            .opts
            .additional_derive_ignore
            .contains(&name.to_kebab_case())
        {
            derives.extend(self.r#gen.opts.additional_derive_attributes.to_vec());
        }
        derives.extend(
            ["Clone", "Copy", "PartialEq", "Eq", "PartialOrd", "Ord"]
                .into_iter()
                .map(|s| s.to_string()),
        );
        self.push_str("#[derive(");
        self.push_str(&derives.into_iter().collect::<Vec<_>>().join(", "));
        self.push_str(")]\n");
        self.push_str(&format!("pub enum {name} {{\n"));
        for case in enum_.cases.iter() {
            self.rustdoc(&case.docs);
            self.push_str(&case_attr(case));
            self.push_str(&case.name.to_upper_camel_case());
            self.push_str(",\n");
        }
        self.push_str("}\n");

        // Auto-synthesize an implementation of the standard `Error` trait for
        // error-looking types based on their name.
        if info.error {
            self.push_str("impl ");
            self.push_str(&name);
            self.push_str("{\n");

            self.push_str("pub fn name(&self) -> &'static str {\n");
            self.push_str("match self {\n");
            for case in enum_.cases.iter() {
                self.push_str(&name);
                self.push_str("::");
                self.push_str(&case.name.to_upper_camel_case());
                self.push_str(" => \"");
                self.push_str(case.name.as_str());
                self.push_str("\",\n");
            }
            self.push_str("}\n");
            self.push_str("}\n");

            self.push_str("pub fn message(&self) -> &'static str {\n");
            self.push_str("match self {\n");
            for case in enum_.cases.iter() {
                self.push_str(&name);
                self.push_str("::");
                self.push_str(&case.name.to_upper_camel_case());
                self.push_str(" => \"");
                if let Some(contents) = &case.docs.contents {
                    self.push_str(contents.trim());
                }
                self.push_str("\",\n");
            }
            self.push_str("}\n");
            self.push_str("}\n");

            self.push_str("}\n");

            self.push_str("impl ::core::fmt::Debug for ");
            self.push_str(&name);
            self.push_str(
                "{\nfn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {\n",
            );
            self.push_str("f.debug_struct(\"");
            self.push_str(&name);
            self.push_str("\")\n");
            self.push_str(".field(\"code\", &(*self as i32))\n");
            self.push_str(".field(\"name\", &self.name())\n");
            self.push_str(".field(\"message\", &self.message())\n");
            self.push_str(".finish()\n");
            self.push_str("}\n");
            self.push_str("}\n");

            self.push_str("impl ::core::fmt::Display for ");
            self.push_str(&name);
            self.push_str(
                "{\nfn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {\n",
            );
            self.push_str("write!(f, \"{} (error {})\", self.name(), *self as i32)\n");
            self.push_str("}\n");
            self.push_str("}\n");
            self.push_str("\n");
            if self.r#gen.opts.std_feature {
                self.push_str("#[cfg(feature = \"std\")]\n");
            }
            self.push_str("impl ::core::error::Error for ");
            self.push_str(&name);
            self.push_str(" {}\n");
        } else {
            self.print_rust_enum_debug(
                TypeMode::owned(),
                &name,
                enum_
                    .cases
                    .iter()
                    .map(|c| (c.name.to_upper_camel_case(), None)),
            )
        }
    }

    fn print_typedef_alias(&mut self, id: TypeId, ty: &Type, docs: &Docs) {
        for (name, mode) in self.modes_of(id) {
            self.rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(mode.lifetime);
            self.push_str(" = ");
            self.print_ty(ty, mode);
            self.push_str(";\n");
        }

        if self.is_exported_resource(id) {
            self.rustdoc(docs);
            let name = self.resolve.types[id].name.as_ref().unwrap();
            let name = name.to_upper_camel_case();
            self.push_str(&format!("pub type {name}Borrow<'a>"));
            self.push_str(" = ");
            self.print_ty(ty, TypeMode::owned());
            self.push_str("Borrow<'a>");
            self.push_str(";\n");
        }
    }

    fn param_name(&self, ty: TypeId) -> String {
        let info = self.info(ty);
        let name = to_upper_camel_case(self.resolve.types[ty].name.as_ref().unwrap());
        if self.uses_two_names(&info) {
            format!("{name}Param")
        } else {
            name
        }
    }

    fn result_name(&self, ty: TypeId) -> String {
        let info = self.info(ty);
        let name = to_upper_camel_case(self.resolve.types[ty].name.as_ref().unwrap());
        if self.uses_two_names(&info) {
            format!("{name}Result")
        } else {
            name
        }
    }

    fn uses_two_names(&self, info: &TypeInfo) -> bool {
        // Types are only duplicated if explicitly requested ...
        matches!(
            self.r#gen.opts.ownership,
            Ownership::Borrowing {
                duplicate_if_necessary: true
            }
        )
            // ... and if they're both used in a borrowed/owned context
            && info.borrowed
            && info.owned
            // ... and they have a list ...
            && info.has_list
            // ... and if there's NOT an `own` handle since those are always
            // done by ownership.
            && !info.has_own_handle
    }

    fn path_to_interface(&self, interface: InterfaceId) -> Option<String> {
        let InterfaceName { path, remapped } = &self.r#gen.interface_names[&interface];
        if *remapped {
            let mut path_to_root = self.path_to_root();
            path_to_root.push_str(path);
            return Some(path_to_root);
        } else {
            let mut full_path = String::new();
            match self.identifier {
                Identifier::Interface(cur, name) => {
                    if cur == interface {
                        return None;
                    }
                    if !self.in_import {
                        full_path.push_str("super::");
                    }
                    match name {
                        WorldKey::Name(_) => {
                            full_path.push_str("super::");
                        }
                        WorldKey::Interface(_) => {
                            full_path.push_str("super::super::super::");
                        }
                    }
                }
                Identifier::StreamOrFuturePayload => {
                    full_path.push_str("super::super::");
                }
                _ => {}
            }
            full_path.push_str(&path);
            Some(full_path)
        }
    }

    fn push_vec_name(&mut self) {
        let path = self.path_to_vec();
        self.push_str(&path);
    }

    pub fn is_exported_resource(&self, ty: TypeId) -> bool {
        let ty = dealias(self.resolve, ty);
        let ty = &self.resolve.types[ty];
        match &ty.kind {
            TypeDefKind::Resource => {}
            _ => return false,
        }

        match ty.owner {
            // Worlds cannot export types of any kind as of this writing.
            TypeOwner::World(_) => false,

            // Interfaces are "stateful" currently where whatever we last saw
            // them as dictates whether it's exported or not.
            TypeOwner::Interface(i) => !self.r#gen.interface_last_seen_as_import[&i],

            // Shouldn't be the case for resources
            TypeOwner::None => unreachable!(),
        }
    }

    fn push_string_name(&mut self) {
        let path = self.path_to_string();
        self.push_str(&path);
    }

    fn push_str(&mut self, s: &str) {
        self.src.push_str(s);
    }

    fn info(&self, ty: TypeId) -> TypeInfo {
        self.r#gen.types.get(ty)
    }

    fn print_borrowed_str(&mut self, lifetime: &'static str) {
        self.push_str("&");
        if lifetime != "'_" {
            self.push_str(lifetime);
            self.push_str(" ");
        }
        if self.r#gen.opts.raw_strings {
            self.push_str("[u8]");
        } else {
            self.push_str("str");
        }
    }

    pub fn path_to_resource(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::ResourceType, "Resource")
    }

    fn path_to_wasm_resource(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::ResourceType, "WasmResource")
    }

    pub fn path_to_invalid_enum_discriminant(&mut self) -> String {
        self.path_from_runtime_module(
            RuntimeItem::InvalidEnumDiscriminant,
            "invalid_enum_discriminant",
        )
    }

    pub fn path_to_string_lift(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::StringLift, "string_lift")
    }

    pub fn path_to_cabi_dealloc(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::CabiDealloc, "cabi_dealloc")
    }

    pub fn path_to_as_i32(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::AsI32, "as_i32")
    }

    pub fn path_to_as_i64(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::AsI64, "as_i64")
    }

    pub fn path_to_as_f32(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::AsF32, "as_f32")
    }

    pub fn path_to_as_f64(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::AsF64, "as_f64")
    }

    pub fn path_to_char_lift(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::CharLift, "char_lift")
    }

    pub fn path_to_bool_lift(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::BoolLift, "bool_lift")
    }

    fn path_to_run_ctors_once(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::RunCtorsOnce, "run_ctors_once")
    }

    pub fn path_to_vec(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::VecType, "Vec")
    }

    pub fn path_to_string(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::StringType, "String")
    }

    pub fn path_to_box(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::BoxType, "Box")
    }

    pub fn path_to_std_alloc_module(&mut self) -> String {
        self.path_from_runtime_module(RuntimeItem::StdAllocModule, "alloc")
    }

    fn path_from_runtime_module(
        &mut self,
        item: RuntimeItem,
        name_in_runtime_module: &str,
    ) -> String {
        self.needs_runtime_module = true;
        self.r#gen.rt_module.insert(item);
        let prefix = if let Identifier::StreamOrFuturePayload = &self.identifier {
            "super::super::"
        } else {
            ""
        };
        format!("{prefix}_rt::{name_in_runtime_module}")
    }

    pub fn is_list_canonical(&self, ty: &Type) -> bool {
        if !self.resolve.all_bits_valid(ty) {
            return false;
        }
        match ty {
            // Note that tuples in Rust are not ABI-compatible with component
            // model tuples, so those are exempted here from canonical lists.
            Type::Id(id) => {
                let info = self.r#gen.types.get(*id);
                !info.has_resource && !info.has_tuple
            }
            _ => true,
        }
    }
}

impl<'a> wit_bindgen_core::InterfaceGenerator<'a> for InterfaceGenerator<'a> {
    fn resolve(&self) -> &'a Resolve {
        self.resolve
    }

    fn type_record(&mut self, id: TypeId, _name: &str, record: &Record, docs: &Docs) {
        self.print_typedef_record(id, record, docs);
    }

    fn type_resource(&mut self, _id: TypeId, name: &str, docs: &Docs) {
        self.rustdoc(docs);
        let camel = to_upper_camel_case(name);
        let resource = self.path_to_resource();

        let wasm_import_module = if self.in_import {
            // Imported resources are a simple wrapper around `Resource<T>` in
            // the `wit-bindgen` crate.
            uwriteln!(
                self.src,
                r#"
                    #[derive(Debug)]
                    #[repr(transparent)]
                    pub struct {camel} {{
                        handle: {resource}<{camel}>,
                    }}

                    impl {camel} {{
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {{
                            Self {{
                                handle: unsafe {{ {resource}::from_handle(handle) }},
                            }}
                        }}

                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {{
                            {resource}::take_handle(&self.handle)
                        }}

                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {{
                            {resource}::handle(&self.handle)
                        }}
                    }}
                "#
            );
            self.wasm_import_module.to_string()
        } else {
            let module = match self.identifier {
                Identifier::Interface(_, key) => self.resolve.name_world_key(key),
                Identifier::World(_) => unimplemented!("resource exports from worlds"),
                Identifier::StreamOrFuturePayload => unreachable!(),
            };
            let box_path = self.path_to_box();
            uwriteln!(
                self.src,
                r#"
#[derive(Debug)]
#[repr(transparent)]
pub struct {camel} {{
    handle: {resource}<{camel}>,
}}

type _{camel}Rep<T> = Option<T>;

impl {camel} {{
    /// Creates a new resource from the specified representation.
    ///
    /// This function will create a new resource handle by moving `val` onto
    /// the heap and then passing that heap pointer to the component model to
    /// create a handle. The owned handle is then returned as `{camel}`.
    pub fn new<T: Guest{camel}>(val: T) -> Self {{
        Self::type_guard::<T>();
        let val: _{camel}Rep<T> = Some(val);
        let ptr: *mut _{camel}Rep<T> =
            {box_path}::into_raw({box_path}::new(val));
        unsafe {{
            Self::from_handle(T::_resource_new(ptr.cast()))
        }}
    }}

    /// Gets access to the underlying `T` which represents this resource.
    pub fn get<T: Guest{camel}>(&self) -> &T {{
        let ptr = unsafe {{ &*self.as_ptr::<T>() }};
        ptr.as_ref().unwrap()
    }}

    /// Gets mutable access to the underlying `T` which represents this
    /// resource.
    pub fn get_mut<T: Guest{camel}>(&mut self) -> &mut T {{
        let ptr = unsafe {{ &mut *self.as_ptr::<T>() }};
        ptr.as_mut().unwrap()
    }}

    /// Consumes this resource and returns the underlying `T`.
    pub fn into_inner<T: Guest{camel}>(self) -> T {{
        let ptr = unsafe {{ &mut *self.as_ptr::<T>() }};
        ptr.take().unwrap()
    }}

    #[doc(hidden)]
    pub unsafe fn from_handle(handle: u32) -> Self {{
        Self {{
            handle: unsafe {{ {resource}::from_handle(handle) }},
        }}
    }}

    #[doc(hidden)]
    pub fn take_handle(&self) -> u32 {{
        {resource}::take_handle(&self.handle)
    }}

    #[doc(hidden)]
    pub fn handle(&self) -> u32 {{
        {resource}::handle(&self.handle)
    }}

    // It's theoretically possible to implement the `Guest{camel}` trait twice
    // so guard against using it with two different types here.
    #[doc(hidden)]
    fn type_guard<T: 'static>() {{
        use core::any::TypeId;
        static mut LAST_TYPE: Option<TypeId> = None;
        unsafe {{
            assert!(!cfg!(target_feature = "atomics"));
            let id = TypeId::of::<T>();
            match LAST_TYPE {{
                Some(ty) => assert!(ty == id, "cannot use two types with this resource type"),
                None => LAST_TYPE = Some(id),
            }}
        }}
    }}

    #[doc(hidden)]
    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {{
        Self::type_guard::<T>();
        let _ = unsafe {{ {box_path}::from_raw(handle as *mut _{camel}Rep<T>) }};
    }}

    fn as_ptr<T: Guest{camel}>(&self) -> *mut _{camel}Rep<T> {{
       {camel}::type_guard::<T>();
       T::_resource_rep(self.handle()).cast()
    }}
}}

/// A borrowed version of [`{camel}`] which represents a borrowed value
/// with the lifetime `'a`.
#[derive(Debug)]
#[repr(transparent)]
pub struct {camel}Borrow<'a> {{
    rep: *mut u8,
    _marker: core::marker::PhantomData<&'a {camel}>,
}}

impl<'a> {camel}Borrow<'a>{{
    #[doc(hidden)]
    pub unsafe fn lift(rep: usize) -> Self {{
        Self {{
            rep: rep as *mut u8,
            _marker: core::marker::PhantomData,
        }}
    }}

    /// Gets access to the underlying `T` in this resource.
    pub fn get<T: Guest{camel}>(&self) -> &'a T {{
       let ptr = unsafe {{ &mut *self.as_ptr::<T>() }};
       ptr.as_ref().unwrap()
    }}

    // NB: mutable access is not allowed due to the component model allowing
    // multiple borrows of the same resource.

    fn as_ptr<T: 'static>(&self) -> *mut _{camel}Rep<T> {{
       {camel}::type_guard::<T>();
       self.rep.cast()
    }}
}}
                "#
            );
            format!("[export]{module}")
        };

        let wasm_resource = self.path_to_wasm_resource();
        let intrinsic = crate::declare_import(
            &wasm_import_module,
            &format!("[resource-drop]{name}"),
            "drop",
            &[abi::WasmType::I32],
            &[],
        );
        uwriteln!(
            self.src,
            r#"
                unsafe impl {wasm_resource} for {camel} {{
                     #[inline]
                     unsafe fn drop(_handle: u32) {{
                         {intrinsic}
                         unsafe {{ drop(_handle as i32); }}
                     }}
                }}
            "#
        );
    }

    fn type_tuple(&mut self, id: TypeId, _name: &str, tuple: &Tuple, docs: &Docs) {
        for (name, mode) in self.modes_of(id) {
            self.rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(mode.lifetime);
            self.push_str(" = (");
            for ty in tuple.types.iter() {
                let mode = self.filter_mode(ty, mode);
                self.print_ty(ty, mode);
                self.push_str(",");
            }
            self.push_str(");\n");
        }
    }

    fn type_flags(&mut self, _id: TypeId, name: &str, flags: &Flags, docs: &Docs) {
        self.src.push_str(&format!(
            "{bitflags}::bitflags! {{\n",
            bitflags = self.r#gen.bitflags_path()
        ));
        self.rustdoc(docs);
        let repr = RustFlagsRepr::new(flags);
        self.src.push_str(&format!(
            "#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]\npub struct {}: {repr} {{\n",
            name.to_upper_camel_case(),
        ));
        for (i, flag) in flags.flags.iter().enumerate() {
            self.rustdoc(&flag.docs);
            self.src.push_str(&format!(
                "const {} = 1 << {};\n",
                flag.name.to_shouty_snake_case(),
                i,
            ));
        }
        self.src.push_str("}\n");
        self.src.push_str("}\n");
    }

    fn type_variant(&mut self, id: TypeId, _name: &str, variant: &Variant, docs: &Docs) {
        self.print_typedef_variant(id, variant, docs);
    }

    fn type_option(&mut self, id: TypeId, _name: &str, payload: &Type, docs: &Docs) {
        self.print_typedef_option(id, payload, docs);
    }

    fn type_result(&mut self, id: TypeId, _name: &str, result: &Result_, docs: &Docs) {
        self.print_typedef_result(id, result, docs);
    }

    fn type_enum(&mut self, id: TypeId, name: &str, enum_: &Enum, docs: &Docs) {
        self.print_typedef_enum(id, name, enum_, docs, &[], Box::new(|_| String::new()));

        let name = to_upper_camel_case(name);
        let mut cases = String::new();
        let repr = int_repr(enum_.tag());
        for (i, case) in enum_.cases.iter().enumerate() {
            let case = case.name.to_upper_camel_case();
            cases.push_str(&format!("{i} => {name}::{case},\n"));
        }
        uwriteln!(
            self.src,
            r#"
                impl {name} {{
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: {repr}) -> {name} {{
                        if !cfg!(debug_assertions) {{
                            return unsafe {{ ::core::mem::transmute(val) }};
                        }}

                        match val {{
                            {cases}
                            _ => panic!("invalid enum discriminant"),
                        }}
                    }}
                }}
            "#
        );
    }

    fn type_alias(&mut self, id: TypeId, _name: &str, ty: &Type, docs: &Docs) {
        self.print_typedef_alias(id, ty, docs);
    }

    fn type_list(&mut self, id: TypeId, _name: &str, ty: &Type, docs: &Docs) {
        for (name, mode) in self.modes_of(id) {
            self.rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(mode.lifetime);
            self.push_str(" = ");
            self.print_list(ty, mode);
            self.push_str(";\n");
        }
    }

    fn type_future(&mut self, _id: TypeId, name: &str, ty: &Option<Type>, docs: &Docs) {
        let async_support = self.r#gen.async_support_path();
        let mode = TypeMode {
            style: TypeOwnershipStyle::Owned,
            lists_borrowed: false,
            lifetime: None,
        };
        self.rustdoc(docs);
        self.push_str(&format!("pub type {}", name.to_upper_camel_case()));
        self.print_generics(mode.lifetime);
        self.push_str(" = ");
        self.push_str(&format!("{async_support}::FutureReader<"));
        self.print_optional_ty(ty.as_ref(), mode);
        self.push_str(">");
        self.push_str(";\n");
    }

    fn type_stream(&mut self, _id: TypeId, name: &str, ty: &Option<Type>, docs: &Docs) {
        let async_support = self.r#gen.async_support_path();
        let mode = TypeMode {
            style: TypeOwnershipStyle::Owned,
            lists_borrowed: false,
            lifetime: None,
        };
        self.rustdoc(docs);
        self.push_str(&format!("pub type {}", name.to_upper_camel_case()));
        self.print_generics(mode.lifetime);
        self.push_str(" = ");
        self.push_str(&format!("{async_support}::StreamReader<"));
        self.print_optional_ty(ty.as_ref(), mode);
        self.push_str(">");
        self.push_str(";\n");
    }

    fn type_builtin(&mut self, _id: TypeId, name: &str, ty: &Type, docs: &Docs) {
        self.rustdoc(docs);
        self.src
            .push_str(&format!("pub type {}", name.to_upper_camel_case()));
        self.src.push_str(" = ");
        self.print_ty(ty, TypeMode::owned());
        self.src.push_str(";\n");
    }
}

struct AnonTypeGenerator<'a, 'b> {
    mode: TypeMode,
    resolve: &'a Resolve,
    interface: &'a mut InterfaceGenerator<'b>,
}

impl<'a, 'b> wit_bindgen_core::AnonymousTypeGenerator<'a> for AnonTypeGenerator<'a, 'b> {
    fn resolve(&self) -> &'a Resolve {
        self.resolve
    }

    fn anonymous_type_type(&mut self, _id: TypeId, ty: &Type, _docs: &Docs) {
        self.interface.print_ty(ty, self.mode);
    }

    fn anonymous_type_handle(&mut self, _id: TypeId, handle: &Handle, _docs: &Docs) {
        match handle {
            Handle::Own(ty) => {
                self.interface.print_ty(&Type::Id(*ty), self.mode);
            }
            Handle::Borrow(ty) => {
                assert!(self.mode.lifetime.is_some());
                let lt = self.mode.lifetime.unwrap();
                if self.interface.is_exported_resource(*ty) {
                    let camel = self.resolve.types[*ty]
                        .name
                        .as_deref()
                        .unwrap()
                        .to_upper_camel_case();
                    let name = format!("{camel}Borrow");
                    self.interface
                        .push_str(&self.interface.type_path_with_name(*ty, name));
                    self.interface.push_str("<");
                    self.interface.push_str(lt);
                    self.interface.push_str(">");
                } else {
                    self.interface.push_str("&");
                    if lt != "'_" {
                        self.interface.push_str(lt);
                        self.interface.push_str(" ");
                    }
                    let ty = &Type::Id(*ty);
                    let mode = self.interface.filter_mode(ty, self.mode);
                    self.interface.print_ty(ty, mode);
                }
            }
        }
    }

    fn anonymous_type_tuple(&mut self, _id: TypeId, ty: &Tuple, _docs: &Docs) {
        self.interface.push_str("(");
        for ty in ty.types.iter() {
            let mode = self.interface.filter_mode_preserve_top(ty, self.mode);
            self.interface.print_ty(ty, mode);
            self.interface.push_str(",");
        }
        self.interface.push_str(")");
    }

    fn anonymous_type_option(&mut self, _id: TypeId, t: &Type, _docs: &Docs) {
        self.interface.push_str("Option<");
        let mode = self.interface.filter_mode_preserve_top(t, self.mode);
        self.interface.print_ty(t, mode);
        self.interface.push_str(">");
    }

    fn anonymous_type_result(&mut self, _id: TypeId, r: &Result_, _docs: &Docs) {
        self.interface.push_str("Result<");
        self.interface.print_optional_ty(r.ok.as_ref(), self.mode);
        self.interface.push_str(",");
        self.interface.print_optional_ty(r.err.as_ref(), self.mode);
        self.interface.push_str(">");
    }

    fn anonymous_type_list(&mut self, _id: TypeId, ty: &Type, _docs: &Docs) {
        self.interface.print_list(ty, self.mode)
    }

    fn anonymous_type_future(&mut self, _id: TypeId, ty: &Option<Type>, _docs: &Docs) {
        let async_support = self.interface.r#gen.async_support_path();
        let mode = TypeMode {
            style: TypeOwnershipStyle::Owned,
            lists_borrowed: false,
            lifetime: None,
        };
        self.interface
            .push_str(&format!("{async_support}::FutureReader<"));
        self.interface.print_optional_ty(ty.as_ref(), mode);
        self.interface.push_str(">");
    }

    fn anonymous_type_stream(&mut self, _id: TypeId, ty: &Option<Type>, _docs: &Docs) {
        let async_support = self.interface.r#gen.async_support_path();
        let mode = TypeMode {
            style: TypeOwnershipStyle::Owned,
            lists_borrowed: false,
            lifetime: None,
        };
        self.interface
            .push_str(&format!("{async_support}::StreamReader<"));
        self.interface.print_optional_ty(ty.as_ref(), mode);
        self.interface.push_str(">");
    }
}

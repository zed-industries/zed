use anyhow::{Result, anyhow, bail};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;
use std::mem;
use std::ops::Deref;
use wit_parser::*;

// NB: keep in sync with `crates/wit-parser/src/ast/lex.rs`
const PRINT_F32_F64_DEFAULT: bool = true;

/// A utility for printing WebAssembly interface definitions to a string.
pub struct WitPrinter<O: Output = OutputToString> {
    /// Visitor that holds the WIT document being printed.
    pub output: O,

    // Count of how many items in this current block have been printed to print
    // a blank line between each item, but not the first item.
    any_items: bool,

    // Whether to print doc comments.
    emit_docs: bool,

    print_f32_f64: bool,
}

impl Default for WitPrinter {
    fn default() -> Self {
        Self::new(OutputToString::default())
    }
}

impl<O: Output> WitPrinter<O> {
    /// Craete new instance.
    pub fn new(output: O) -> Self {
        Self {
            output,
            any_items: false,
            emit_docs: true,
            print_f32_f64: match std::env::var("WIT_REQUIRE_F32_F64") {
                Ok(s) => s == "1",
                Err(_) => PRINT_F32_F64_DEFAULT,
            },
        }
    }

    /// Prints the specified `pkg` which is located in `resolve` to `O`.
    ///
    /// The `nested` list of packages are other packages to include at the end
    /// of the output in `package ... { ... }` syntax.
    pub fn print(&mut self, resolve: &Resolve, pkg: PackageId, nested: &[PackageId]) -> Result<()> {
        self.print_package(resolve, pkg, true)?;
        for (i, pkg_id) in nested.iter().enumerate() {
            if i > 0 {
                self.output.newline();
                self.output.newline();
            }
            self.print_package(resolve, *pkg_id, false)?;
        }
        Ok(())
    }

    /// Configure whether doc comments will be printed.
    ///
    /// Defaults to true.
    pub fn emit_docs(&mut self, enabled: bool) -> &mut Self {
        self.emit_docs = enabled;
        self
    }

    /// Prints the specified `pkg`.
    ///
    /// If `is_main` is not set, nested package notation is used.
    pub fn print_package(
        &mut self,
        resolve: &Resolve,
        pkg: PackageId,
        is_main: bool,
    ) -> Result<()> {
        let pkg = &resolve.packages[pkg];
        self.print_package_outer(pkg)?;

        if is_main {
            self.output.semicolon();
            self.output.newline();
        } else {
            self.output.indent_start();
        }

        for (name, id) in pkg.interfaces.iter() {
            self.print_interface_outer(resolve, *id, name)?;
            self.output.indent_start();
            self.print_interface(resolve, *id)?;
            self.output.indent_end();
            if is_main {
                self.output.newline();
            }
        }

        for (name, id) in pkg.worlds.iter() {
            self.print_docs(&resolve.worlds[*id].docs);
            self.print_stability(&resolve.worlds[*id].stability);
            self.output.keyword("world");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::WorldDeclaration);
            self.output.indent_start();
            self.print_world(resolve, *id)?;
            self.output.indent_end();
        }
        if !is_main {
            self.output.indent_end();
        }
        Ok(())
    }

    /// Print the specified package without its content.
    /// Does not print the semicolon nor starts the indentation.
    pub fn print_package_outer(&mut self, pkg: &Package) -> Result<()> {
        self.print_docs(&pkg.docs);
        self.output.keyword("package");
        self.output.str(" ");
        self.print_name_type(&pkg.name.namespace, TypeKind::NamespaceDeclaration);
        self.output.str(":");
        self.print_name_type(&pkg.name.name, TypeKind::PackageNameDeclaration);
        if let Some(version) = &pkg.name.version {
            self.print_name_type(&format!("@{version}"), TypeKind::VersionDeclaration);
        }
        Ok(())
    }

    fn new_item(&mut self) {
        if self.any_items {
            self.output.newline();
        }
        self.any_items = true;
    }

    /// Print the given WebAssembly interface without its content.
    /// Does not print the semicolon nor starts the indentation.
    pub fn print_interface_outer(
        &mut self,
        resolve: &Resolve,
        id: InterfaceId,
        name: &str,
    ) -> Result<()> {
        self.print_docs(&resolve.interfaces[id].docs);
        self.print_stability(&resolve.interfaces[id].stability);
        self.output.keyword("interface");
        self.output.str(" ");
        self.print_name_type(name, TypeKind::InterfaceDeclaration);
        Ok(())
    }

    /// Print the inner content of a given WebAssembly interface.
    pub fn print_interface(&mut self, resolve: &Resolve, id: InterfaceId) -> Result<()> {
        let prev_items = mem::replace(&mut self.any_items, false);
        let interface = &resolve.interfaces[id];

        let mut resource_funcs = HashMap::new();
        let mut freestanding = Vec::new();
        for (_, func) in interface.functions.iter() {
            if let Some(id) = func.kind.resource() {
                resource_funcs.entry(id).or_insert(Vec::new()).push(func);
            } else {
                freestanding.push(func);
            }
        }

        self.print_types(
            resolve,
            TypeOwner::Interface(id),
            interface
                .types
                .iter()
                .map(|(name, id)| (name.as_str(), *id)),
            &resource_funcs,
        )?;

        for func in freestanding {
            self.new_item();
            self.print_docs(&func.docs);
            self.print_stability(&func.stability);
            self.print_name_type(func.item_name(), TypeKind::FunctionFreestanding);
            self.output.str(": ");
            self.print_function(resolve, func)?;
            self.output.semicolon();
        }

        self.any_items = prev_items;

        Ok(())
    }

    /// Print types of an interface.
    pub fn print_types<'a>(
        &mut self,
        resolve: &Resolve,
        owner: TypeOwner,
        types: impl Iterator<Item = (&'a str, TypeId)>,
        resource_funcs: &HashMap<TypeId, Vec<&Function>>,
    ) -> Result<()> {
        // Partition types defined in this interface into either those imported
        // from foreign interfaces or those defined locally.
        let mut types_to_declare = Vec::new();
        let mut types_to_import: Vec<(_, &_, Vec<_>)> = Vec::new();
        for (name, ty_id) in types {
            let ty = &resolve.types[ty_id];
            if let TypeDefKind::Type(Type::Id(other)) = ty.kind {
                let other = &resolve.types[other];
                match other.owner {
                    TypeOwner::None => {}
                    other_owner if owner != other_owner => {
                        let other_name = other
                            .name
                            .as_ref()
                            .ok_or_else(|| anyhow!("cannot import unnamed type"))?;
                        if let Some((owner, stability, list)) = types_to_import.last_mut() {
                            if *owner == other_owner && ty.stability == **stability {
                                list.push((name, other_name));
                                continue;
                            }
                        }
                        types_to_import.push((
                            other_owner,
                            &ty.stability,
                            vec![(name, other_name)],
                        ));
                        continue;
                    }
                    _ => {}
                }
            }

            types_to_declare.push(ty_id);
        }

        // Generate a `use` statement for all imported types.
        let my_pkg = match owner {
            TypeOwner::Interface(id) => resolve.interfaces[id].package.unwrap(),
            TypeOwner::World(id) => resolve.worlds[id].package.unwrap(),
            TypeOwner::None => unreachable!(),
        };
        for (owner, stability, tys) in types_to_import {
            self.any_items = true;
            self.print_stability(stability);
            self.output.keyword("use");
            self.output.str(" ");
            let id = match owner {
                TypeOwner::Interface(id) => id,
                // it's only possible to import types from interfaces at
                // this time.
                _ => unreachable!(),
            };
            self.print_path_to_interface(resolve, id, my_pkg)?;
            self.output.str(".{"); // Note: not changing the indentation.
            for (i, (my_name, other_name)) in tys.into_iter().enumerate() {
                if i > 0 {
                    self.output.str(", ");
                }
                if my_name == other_name {
                    self.print_name_type(my_name, TypeKind::TypeImport);
                } else {
                    self.print_name_type(other_name, TypeKind::TypeImport);
                    self.output.str(" ");
                    self.output.keyword("as");
                    self.output.str(" ");
                    self.print_name_type(my_name, TypeKind::TypeAlias);
                }
            }
            self.output.str("}"); // Note: not changing the indentation.
            self.output.semicolon();
        }

        for id in types_to_declare {
            self.new_item();
            self.print_docs(&resolve.types[id].docs);
            self.print_stability(&resolve.types[id].stability);
            match resolve.types[id].kind {
                TypeDefKind::Resource => self.print_resource(
                    resolve,
                    id,
                    resource_funcs.get(&id).unwrap_or(&Vec::new()),
                )?,
                _ => self.declare_type(resolve, &Type::Id(id))?,
            }
        }

        Ok(())
    }

    fn print_resource(&mut self, resolve: &Resolve, id: TypeId, funcs: &[&Function]) -> Result<()> {
        let ty = &resolve.types[id];
        self.output.ty("resource", TypeKind::BuiltIn);
        self.output.str(" ");
        self.print_name_type(
            ty.name.as_ref().expect("resources must be named"),
            TypeKind::Resource,
        );
        if funcs.is_empty() {
            self.output.semicolon();
            return Ok(());
        }
        self.output.indent_start();
        for func in funcs {
            self.print_docs(&func.docs);
            self.print_stability(&func.stability);

            match &func.kind {
                FunctionKind::Constructor(_) => {}
                FunctionKind::Method(_) | FunctionKind::AsyncMethod(_) => {
                    self.print_name_type(func.item_name(), TypeKind::FunctionMethod);
                    self.output.str(": ");
                }
                FunctionKind::Static(_) | FunctionKind::AsyncStatic(_) => {
                    self.print_name_type(func.item_name(), TypeKind::FunctionStatic);
                    self.output.str(": ");
                    self.output.keyword("static");
                    self.output.str(" ");
                }
                FunctionKind::Freestanding | FunctionKind::AsyncFreestanding => unreachable!(),
            }
            self.print_function(resolve, func)?;
            self.output.semicolon();
        }
        self.output.indent_end();

        Ok(())
    }

    fn print_function(&mut self, resolve: &Resolve, func: &Function) -> Result<()> {
        // Handle the `async` prefix if necessary
        match &func.kind {
            FunctionKind::AsyncFreestanding
            | FunctionKind::AsyncMethod(_)
            | FunctionKind::AsyncStatic(_) => {
                self.output.keyword("async");
                self.output.str(" ");
            }
            _ => {}
        }

        // Constructors are named slightly differently.
        match &func.kind {
            FunctionKind::Constructor(_) => {
                self.output.keyword("constructor");
                self.output.str("(");
            }
            FunctionKind::Freestanding
            | FunctionKind::AsyncFreestanding
            | FunctionKind::Method(_)
            | FunctionKind::AsyncMethod(_)
            | FunctionKind::Static(_)
            | FunctionKind::AsyncStatic(_) => {
                self.output.keyword("func");
                self.output.str("(");
            }
        }

        // Methods don't print their `self` argument
        let params_to_skip = match &func.kind {
            FunctionKind::Method(_) | FunctionKind::AsyncMethod(_) => 1,
            _ => 0,
        };
        for (i, (name, ty)) in func.params.iter().skip(params_to_skip).enumerate() {
            if i > 0 {
                self.output.str(", ");
            }
            self.print_name_param(name);
            self.output.str(": ");
            self.print_type_name(resolve, ty)?;
        }
        self.output.str(")");

        // shorthand constructors don't have their results printed
        if func.is_constructor_shorthand(resolve) {
            return Ok(());
        }

        if let Some(ty) = &func.result {
            self.output.str(" -> ");
            self.print_type_name(resolve, ty)?;
        }
        Ok(())
    }

    /// Prints the world `id` within `resolve`.
    ///
    /// This is a little tricky to preserve round-tripping that WIT wants. This
    /// function inherently can't preserve ordering of imports because resource
    /// functions aren't guaranteed to be all adjacent to the resource itself
    /// they're attached to. That means that at the very least, when printing
    /// resource functions, items may be printed out-of-order.
    ///
    /// To help solve this the printing here is kept in sync with WIT encoding
    /// of worlds which is to print items in the order of:
    ///
    /// * Any imported interface. Ordering between interfaces is preserved.
    /// * Any types, including resource functions on those types. Ordering
    ///   between types is preserved.
    /// * Any functions, which may refer to those types. Ordering between
    ///   functions is preserved.
    ///
    /// This keeps things printed in a roughly topological fashion and makes
    /// round-tripping a bit more reliable.
    fn print_world(&mut self, resolve: &Resolve, id: WorldId) -> Result<()> {
        let prev_items = mem::replace(&mut self.any_items, false);
        let world = &resolve.worlds[id];
        let pkgid = world.package.unwrap();
        let mut types = Vec::new();
        let mut resource_funcs = HashMap::new();
        let mut function_imports_to_print = Vec::new();
        for (name, import) in world.imports.iter() {
            match import {
                WorldItem::Type(t) => match name {
                    WorldKey::Name(s) => types.push((s.as_str(), *t)),
                    WorldKey::Interface(_) => unreachable!(),
                },
                _ => {
                    if let WorldItem::Function(f) = import {
                        if let Some(id) = f.kind.resource() {
                            resource_funcs.entry(id).or_insert(Vec::new()).push(f);
                            continue;
                        }
                        function_imports_to_print.push((name, import));
                        continue;
                    }
                    self.print_world_item(resolve, name, import, pkgid, "import")?;
                    // Don't put a blank line between imports, but count
                    // imports as having printed something so if anything comes
                    // after them then a blank line is printed after imports.
                    self.any_items = true;
                }
            }
        }
        self.print_types(
            resolve,
            TypeOwner::World(id),
            types.into_iter(),
            &resource_funcs,
        )?;

        for (name, import) in function_imports_to_print {
            self.print_world_item(resolve, name, import, pkgid, "import")?;
            self.any_items = true;
        }
        if !world.exports.is_empty() {
            self.new_item();
        }
        for (name, export) in world.exports.iter() {
            self.print_world_item(resolve, name, export, pkgid, "export")?;
        }
        self.any_items = prev_items;
        Ok(())
    }

    fn print_world_item(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        item: &WorldItem,
        cur_pkg: PackageId,
        import_or_export_keyword: &str,
    ) -> Result<()> {
        // Print inline item docs
        if matches!(name, WorldKey::Name(_)) {
            self.print_docs(match item {
                WorldItem::Interface { id, .. } => &resolve.interfaces[*id].docs,
                WorldItem::Function(f) => &f.docs,
                // Types are handled separately
                WorldItem::Type(_) => unreachable!(),
            });
        }

        self.print_stability(item.stability(resolve));
        self.output.keyword(import_or_export_keyword);
        self.output.str(" ");
        match name {
            WorldKey::Name(name) => {
                match item {
                    WorldItem::Interface { id, .. } => {
                        self.print_name_type(name, TypeKind::Other);
                        self.output.str(": ");
                        assert!(resolve.interfaces[*id].name.is_none());
                        self.output.keyword("interface");
                        self.output.indent_start();
                        self.print_interface(resolve, *id)?;
                        self.output.indent_end();
                    }
                    WorldItem::Function(f) => {
                        self.print_name_type(&f.name, TypeKind::Other);
                        self.output.str(": ");
                        self.print_function(resolve, f)?;
                        self.output.semicolon();
                    }
                    // Types are handled separately
                    WorldItem::Type(_) => unreachable!(),
                }
            }
            WorldKey::Interface(id) => {
                match item {
                    WorldItem::Interface { id: id2, .. } => assert_eq!(id, id2),
                    _ => unreachable!(),
                }
                self.print_path_to_interface(resolve, *id, cur_pkg)?;
                self.output.semicolon();
            }
        }
        Ok(())
    }

    fn print_path_to_interface(
        &mut self,
        resolve: &Resolve,
        interface: InterfaceId,
        cur_pkg: PackageId,
    ) -> Result<()> {
        let iface = &resolve.interfaces[interface];
        if iface.package == Some(cur_pkg) {
            self.print_name_type(iface.name.as_ref().unwrap(), TypeKind::InterfacePath);
        } else {
            let pkg = &resolve.packages[iface.package.unwrap()].name;
            self.print_name_type(&pkg.namespace, TypeKind::NamespacePath);
            self.output.str(":");
            self.print_name_type(&pkg.name, TypeKind::PackageNamePath);
            self.output.str("/");
            self.print_name_type(iface.name.as_ref().unwrap(), TypeKind::InterfacePath);
            if let Some(version) = &pkg.version {
                self.print_name_type(&format!("@{version}"), TypeKind::VersionPath);
            }
        }
        Ok(())
    }

    /// Print the name of type `ty`.
    pub fn print_type_name(&mut self, resolve: &Resolve, ty: &Type) -> Result<()> {
        match ty {
            Type::Bool => self.output.ty("bool", TypeKind::BuiltIn),
            Type::U8 => self.output.ty("u8", TypeKind::BuiltIn),
            Type::U16 => self.output.ty("u16", TypeKind::BuiltIn),
            Type::U32 => self.output.ty("u32", TypeKind::BuiltIn),
            Type::U64 => self.output.ty("u64", TypeKind::BuiltIn),
            Type::S8 => self.output.ty("s8", TypeKind::BuiltIn),
            Type::S16 => self.output.ty("s16", TypeKind::BuiltIn),
            Type::S32 => self.output.ty("s32", TypeKind::BuiltIn),
            Type::S64 => self.output.ty("s64", TypeKind::BuiltIn),
            Type::F32 => {
                if self.print_f32_f64 {
                    self.output.ty("f32", TypeKind::BuiltIn)
                } else {
                    self.output.ty("f32", TypeKind::BuiltIn)
                }
            }
            Type::F64 => {
                if self.print_f32_f64 {
                    self.output.ty("f64", TypeKind::BuiltIn)
                } else {
                    self.output.ty("f64", TypeKind::BuiltIn)
                }
            }
            Type::Char => self.output.ty("char", TypeKind::BuiltIn),
            Type::String => self.output.ty("string", TypeKind::BuiltIn),
            Type::ErrorContext => self.output.ty("error-context", TypeKind::BuiltIn),

            Type::Id(id) => {
                let ty = &resolve.types[*id];
                if let Some(name) = &ty.name {
                    self.print_name_type(name, TypeKind::Other);
                    return Ok(());
                }

                match &ty.kind {
                    TypeDefKind::Handle(h) => {
                        self.print_handle_type(resolve, h, false)?;
                    }
                    TypeDefKind::Resource => {
                        bail!("resolve has an unnamed resource type");
                    }
                    TypeDefKind::Tuple(t) => {
                        self.print_tuple_type(resolve, t)?;
                    }
                    TypeDefKind::Option(t) => {
                        self.print_option_type(resolve, t)?;
                    }
                    TypeDefKind::Result(t) => {
                        self.print_result_type(resolve, t)?;
                    }
                    TypeDefKind::Record(_) => {
                        bail!("resolve has an unnamed record type");
                    }
                    TypeDefKind::Flags(_) => {
                        bail!("resolve has unnamed flags type")
                    }
                    TypeDefKind::Enum(_) => {
                        bail!("resolve has unnamed enum type")
                    }
                    TypeDefKind::Variant(_) => {
                        bail!("resolve has unnamed variant type")
                    }
                    TypeDefKind::List(ty) => {
                        self.output.ty("list", TypeKind::BuiltIn);
                        self.output.generic_args_start();
                        self.print_type_name(resolve, ty)?;
                        self.output.generic_args_end();
                    }
                    TypeDefKind::Map(key_ty, value_ty) => {
                        self.output.ty("map", TypeKind::BuiltIn);
                        self.output.generic_args_start();
                        self.print_type_name(resolve, key_ty)?;
                        self.output.str(", ");
                        self.print_type_name(resolve, value_ty)?;
                        self.output.generic_args_end();
                    }
                    TypeDefKind::FixedSizeList(ty, size) => {
                        self.output.ty("list", TypeKind::BuiltIn);
                        self.output.generic_args_start();
                        self.print_type_name(resolve, ty)?;
                        self.output.push_str(&format!(", {}", *size));
                        self.output.generic_args_end();
                    }
                    TypeDefKind::Type(ty) => self.print_type_name(resolve, ty)?,
                    TypeDefKind::Future(ty) => {
                        if let Some(ty) = ty {
                            self.output.push_str("future<");
                            self.print_type_name(resolve, ty)?;
                            self.output.push_str(">");
                        } else {
                            self.output.push_str("future");
                        }
                    }
                    TypeDefKind::Stream(ty) => {
                        if let Some(ty) = ty {
                            self.output.push_str("stream<");
                            self.print_type_name(resolve, ty)?;
                            self.output.push_str(">");
                        } else {
                            self.output.push_str("stream");
                        }
                    }
                    TypeDefKind::Unknown => unreachable!(),
                }
            }
        }

        Ok(())
    }

    fn print_handle_type(
        &mut self,
        resolve: &Resolve,
        handle: &Handle,
        force_handle_type_printed: bool,
    ) -> Result<()> {
        match handle {
            Handle::Own(ty) => {
                let ty = &resolve.types[*ty];
                if force_handle_type_printed {
                    self.output.ty("own", TypeKind::BuiltIn);
                    self.output.generic_args_start();
                }
                self.print_name_type(
                    ty.name
                        .as_ref()
                        .ok_or_else(|| anyhow!("unnamed resource type"))?,
                    TypeKind::Resource,
                );
                if force_handle_type_printed {
                    self.output.generic_args_end();
                }
            }

            Handle::Borrow(ty) => {
                self.output.ty("borrow", TypeKind::BuiltIn);
                self.output.generic_args_start();
                let ty = &resolve.types[*ty];
                self.print_name_type(
                    ty.name
                        .as_ref()
                        .ok_or_else(|| anyhow!("unnamed resource type"))?,
                    TypeKind::Resource,
                );
                self.output.generic_args_end();
            }
        }

        Ok(())
    }

    fn print_tuple_type(&mut self, resolve: &Resolve, tuple: &Tuple) -> Result<()> {
        self.output.ty("tuple", TypeKind::BuiltIn);
        self.output.generic_args_start();
        for (i, ty) in tuple.types.iter().enumerate() {
            if i > 0 {
                self.output.str(", ");
            }
            self.print_type_name(resolve, ty)?;
        }
        self.output.generic_args_end();

        Ok(())
    }

    fn print_option_type(&mut self, resolve: &Resolve, payload: &Type) -> Result<()> {
        self.output.ty("option", TypeKind::BuiltIn);
        self.output.generic_args_start();
        self.print_type_name(resolve, payload)?;
        self.output.generic_args_end();
        Ok(())
    }

    fn print_result_type(&mut self, resolve: &Resolve, result: &Result_) -> Result<()> {
        match result {
            Result_ {
                ok: Some(ok),
                err: Some(err),
            } => {
                self.output.ty("result", TypeKind::BuiltIn);
                self.output.generic_args_start();
                self.print_type_name(resolve, ok)?;
                self.output.str(", ");
                self.print_type_name(resolve, err)?;
                self.output.generic_args_end();
            }
            Result_ {
                ok: None,
                err: Some(err),
            } => {
                self.output.ty("result", TypeKind::BuiltIn);
                self.output.generic_args_start();
                self.output.str("_, ");
                self.print_type_name(resolve, err)?;
                self.output.generic_args_end();
            }
            Result_ {
                ok: Some(ok),
                err: None,
            } => {
                self.output.ty("result", TypeKind::BuiltIn);
                self.output.generic_args_start();
                self.print_type_name(resolve, ok)?;
                self.output.generic_args_end();
            }
            Result_ {
                ok: None,
                err: None,
            } => {
                self.output.ty("result", TypeKind::BuiltIn);
            }
        }
        Ok(())
    }

    fn declare_type(&mut self, resolve: &Resolve, ty: &Type) -> Result<()> {
        match ty {
            Type::Bool
            | Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::S8
            | Type::S16
            | Type::S32
            | Type::S64
            | Type::F32
            | Type::F64
            | Type::Char
            | Type::String
            | Type::ErrorContext => return Ok(()),

            Type::Id(id) => {
                let ty = &resolve.types[*id];
                match &ty.kind {
                    TypeDefKind::Handle(h) => {
                        self.declare_handle(resolve, ty.name.as_deref(), h)?
                    }
                    TypeDefKind::Resource => panic!("resources should be processed separately"),
                    TypeDefKind::Record(r) => {
                        self.declare_record(resolve, ty.name.as_deref(), r)?
                    }
                    TypeDefKind::Tuple(t) => self.declare_tuple(resolve, ty.name.as_deref(), t)?,
                    TypeDefKind::Flags(f) => self.declare_flags(ty.name.as_deref(), f)?,
                    TypeDefKind::Variant(v) => {
                        self.declare_variant(resolve, ty.name.as_deref(), v)?
                    }
                    TypeDefKind::Option(t) => {
                        self.declare_option(resolve, ty.name.as_deref(), t)?
                    }
                    TypeDefKind::Result(r) => {
                        self.declare_result(resolve, ty.name.as_deref(), r)?
                    }
                    TypeDefKind::Enum(e) => self.declare_enum(ty.name.as_deref(), e)?,
                    TypeDefKind::List(inner) => {
                        self.declare_list(resolve, ty.name.as_deref(), inner)?
                    }
                    TypeDefKind::Map(key, value) => {
                        self.declare_map(resolve, ty.name.as_deref(), key, value)?
                    }
                    TypeDefKind::FixedSizeList(inner, size) => {
                        self.declare_fixed_size_list(resolve, ty.name.as_deref(), inner, *size)?
                    }
                    TypeDefKind::Type(inner) => match ty.name.as_deref() {
                        Some(name) => {
                            self.output.keyword("type");
                            self.output.str(" ");
                            self.print_name_type(name, TypeKind::TypeName);
                            self.output.str(" = ");
                            self.print_type_name(resolve, inner)?;
                            self.output.semicolon();
                        }
                        None => bail!("unnamed type in document"),
                    },
                    TypeDefKind::Future(inner) => {
                        self.declare_future(resolve, ty.name.as_deref(), inner.as_ref())?
                    }
                    TypeDefKind::Stream(inner) => {
                        self.declare_stream(resolve, ty.name.as_deref(), inner.as_ref())?
                    }
                    TypeDefKind::Unknown => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn declare_handle(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        handle: &Handle,
    ) -> Result<()> {
        match name {
            Some(name) => {
                self.output.keyword("type");
                self.output.str(" ");
                self.print_name_type(name, TypeKind::Resource);
                self.output.str(" = ");
                // Note that the `true` here forces owned handles to be printed
                // as `own<T>`. The purpose of this is because `type a = b`, if
                // `b` is a resource, is encoded differently as `type a =
                // own<b>`. By forcing a handle to be printed here it's staying
                // true to what's in the WIT document.
                self.print_handle_type(resolve, handle, true)?;
                self.output.semicolon();

                Ok(())
            }
            None => bail!("document has unnamed handle type"),
        }
    }

    fn declare_record(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        record: &Record,
    ) -> Result<()> {
        match name {
            Some(name) => {
                self.output.keyword("record");
                self.output.str(" ");
                self.print_name_type(name, TypeKind::Record);
                self.output.indent_start();
                for field in &record.fields {
                    self.print_docs(&field.docs);
                    self.print_name_param(&field.name);
                    self.output.str(": ");
                    self.print_type_name(resolve, &field.ty)?;
                    self.output.str(",");
                    self.output.newline();
                }
                self.output.indent_end();
                Ok(())
            }
            None => bail!("document has unnamed record type"),
        }
    }

    fn declare_tuple(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        tuple: &Tuple,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::Tuple);
            self.output.str(" = ");
            self.print_tuple_type(resolve, tuple)?;
            self.output.semicolon();
        }
        Ok(())
    }

    fn declare_flags(&mut self, name: Option<&str>, flags: &Flags) -> Result<()> {
        match name {
            Some(name) => {
                self.output.keyword("flags");
                self.output.str(" ");
                self.print_name_type(name, TypeKind::Flags);
                self.output.indent_start();
                for flag in &flags.flags {
                    self.print_docs(&flag.docs);
                    self.print_name_case(&flag.name);
                    self.output.str(",");
                    self.output.newline();
                }
                self.output.indent_end();
            }
            None => bail!("document has unnamed flags type"),
        }
        Ok(())
    }

    fn declare_variant(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        variant: &Variant,
    ) -> Result<()> {
        let name = match name {
            Some(name) => name,
            None => bail!("document has unnamed variant type"),
        };
        self.output.keyword("variant");
        self.output.str(" ");
        self.print_name_type(name, TypeKind::Variant);
        self.output.indent_start();
        for case in &variant.cases {
            self.print_docs(&case.docs);
            self.print_name_case(&case.name);
            if let Some(ty) = case.ty {
                self.output.str("(");
                self.print_type_name(resolve, &ty)?;
                self.output.str(")");
            }
            self.output.str(",");
            self.output.newline();
        }
        self.output.indent_end();
        Ok(())
    }

    fn declare_option(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        payload: &Type,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::Option);
            self.output.str(" = ");
            self.print_option_type(resolve, payload)?;
            self.output.semicolon();
        }
        Ok(())
    }

    fn declare_result(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        result: &Result_,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::Result);
            self.output.str(" = ");
            self.print_result_type(resolve, result)?;
            self.output.semicolon();
        }
        Ok(())
    }

    fn declare_enum(&mut self, name: Option<&str>, enum_: &Enum) -> Result<()> {
        let name = match name {
            Some(name) => name,
            None => bail!("document has unnamed enum type"),
        };
        self.output.keyword("enum");
        self.output.str(" ");
        self.print_name_type(name, TypeKind::Enum);
        self.output.indent_start();
        for case in &enum_.cases {
            self.print_docs(&case.docs);
            self.print_name_case(&case.name);
            self.output.str(",");
            self.output.newline();
        }
        self.output.indent_end();
        Ok(())
    }

    fn declare_list(&mut self, resolve: &Resolve, name: Option<&str>, ty: &Type) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::List);
            self.output.str(" = ");
            self.output.ty("list", TypeKind::BuiltIn);
            self.output.str("<");
            self.print_type_name(resolve, ty)?;
            self.output.str(">");
            self.output.semicolon();
            return Ok(());
        }

        Ok(())
    }

    fn declare_map(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        key_ty: &Type,
        value_ty: &Type,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::Map);
            self.output.str(" = ");
            self.output.ty("map", TypeKind::BuiltIn);
            self.output.str("<");
            self.print_type_name(resolve, key_ty)?;
            self.output.str(", ");
            self.print_type_name(resolve, value_ty)?;
            self.output.str(">");
            self.output.semicolon();
            return Ok(());
        }

        Ok(())
    }

    fn declare_fixed_size_list(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        ty: &Type,
        elements: u32,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::List);
            self.output.str(" = ");
            self.output.ty("list", TypeKind::BuiltIn);
            self.output.str("<");
            self.print_type_name(resolve, ty)?;
            self.output.str(&format!(", {elements}"));
            self.output.str(">");
            self.output.semicolon();
            return Ok(());
        }

        Ok(())
    }

    fn declare_stream(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        ty: Option<&Type>,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::Stream);
            self.output.str(" = ");
            self.output.ty("stream", TypeKind::BuiltIn);
            if let Some(ty) = ty {
                self.output.str("<");
                self.print_type_name(resolve, ty)?;
                self.output.str(">");
            }
            self.output.semicolon();
        }

        Ok(())
    }

    fn declare_future(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        ty: Option<&Type>,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.keyword("type");
            self.output.str(" ");
            self.print_name_type(name, TypeKind::Future);
            self.output.str(" = ");
            self.output.ty("future", TypeKind::BuiltIn);
            if let Some(ty) = ty {
                self.output.str("<");
                self.print_type_name(resolve, ty)?;
                self.output.str(">");
            }
            self.output.semicolon();
        }

        Ok(())
    }

    fn escape_name(name: &str) -> Cow<'_, str> {
        if is_keyword(name) {
            Cow::Owned(format!("%{name}"))
        } else {
            Cow::Borrowed(name)
        }
    }

    fn print_name_type(&mut self, name: &str, kind: TypeKind) {
        self.output.ty(Self::escape_name(name).deref(), kind);
    }

    fn print_name_param(&mut self, name: &str) {
        self.output.param(Self::escape_name(name).deref());
    }

    fn print_name_case(&mut self, name: &str) {
        self.output.case(Self::escape_name(name).deref());
    }

    fn print_docs(&mut self, docs: &Docs) {
        if self.emit_docs {
            if let Some(contents) = &docs.contents {
                for line in contents.lines() {
                    self.output.doc(line);
                }
            }
        }
    }

    fn print_stability(&mut self, stability: &Stability) {
        match stability {
            Stability::Unknown => {}
            Stability::Stable { since, deprecated } => {
                self.output.keyword("@since");
                self.output.str("(");
                self.output.keyword("version");
                self.output.str(" = ");
                self.print_name_type(&since.to_string(), TypeKind::VersionAnnotation);
                self.output.str(")");
                self.output.newline();
                if let Some(version) = deprecated {
                    self.output.keyword("@deprecated");
                    self.output.str("(");
                    self.output.keyword("version");
                    self.output.str(" = ");
                    self.print_name_type(&version.to_string(), TypeKind::VersionAnnotation);
                    self.output.str(")");
                    self.output.newline();
                }
            }
            Stability::Unstable {
                feature,
                deprecated,
            } => {
                self.output.keyword("@unstable");
                self.output.str("(");
                self.output.keyword("feature");
                self.output.str(" = ");
                self.output.str(feature);
                self.output.str(")");
                self.output.newline();
                if let Some(version) = deprecated {
                    self.output.keyword("@deprecated");
                    self.output.str("(");
                    self.output.keyword("version");
                    self.output.str(" = ");
                    self.print_name_type(&version.to_string(), TypeKind::VersionAnnotation);
                    self.output.str(")");
                    self.output.newline();
                }
            }
        }
    }
}

fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "use"
            | "type"
            | "func"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "s8"
            | "s16"
            | "s32"
            | "s64"
            | "f32"
            | "f64"
            | "float32"
            | "float64"
            | "char"
            | "resource"
            | "record"
            | "flags"
            | "variant"
            | "enum"
            | "bool"
            | "string"
            | "option"
            | "result"
            | "future"
            | "stream"
            | "list"
            | "own"
            | "borrow"
            | "_"
            | "as"
            | "from"
            | "static"
            | "interface"
            | "tuple"
            | "world"
            | "import"
            | "export"
            | "package"
            | "with"
            | "include"
            | "constructor"
            | "error-context"
            | "async"
    )
}

/// Trait defining visitor methods driven by [`WitPrinter`](WitPrinter).
///
/// Some methods in this trait have default implementations. These default
/// implementations may rely on helper functions that are not
/// invoked directly by `WitPrinter`.
pub trait Output {
    /// Push a string slice into a buffer or an output.
    ///
    /// Parameter `src` can contain punctation characters, and must be escaped
    /// when outputing to languages like HTML.
    /// Helper function used exclusively by the default implementations of trait methods.
    /// This function is not called directly by `WitPrinter`.
    /// When overriding all the trait methods, users do not need to handle this function.
    fn push_str(&mut self, src: &str);

    /// Set the appropriate indentation.
    ///
    /// Helper function used exclusively by the default implementations of trait methods.
    /// This function is not called directly by `WitPrinter`.
    /// When overriding all the trait methods, users do not need to handle this function.
    fn indent_if_needed(&mut self) -> bool;

    /// Start of indentation. In WIT this represents ` {\n`.
    fn indent_start(&mut self);

    /// End of indentation. In WIT this represents `}\n`.
    fn indent_end(&mut self);

    /// This method is designed to be used only by the default methods of this trait.
    /// Called only from the default implementation functions of this trait.
    fn indent_and_print(&mut self, src: &str) {
        assert!(!src.contains('\n'));
        let idented = self.indent_if_needed();
        if idented && src.starts_with(' ') {
            panic!("cannot add a space at the beginning of a line");
        }
        self.push_str(src);
    }

    /// A newline is added.
    fn newline(&mut self);

    /// A keyword is added. Keywords are hardcoded strings from `[a-z]`, but can start with `@`
    /// when printing a [Feature Gate](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#feature-gates)
    fn keyword(&mut self, src: &str) {
        self.indent_and_print(src);
    }

    /// A type is added.
    fn ty(&mut self, src: &str, _kind: TypeKind) {
        self.indent_and_print(src);
    }

    /// A parameter name of a function, record or a named return is added.
    fn param(&mut self, src: &str) {
        self.indent_and_print(src);
    }

    /// A case belonging to a variant, enum or flags is added.
    fn case(&mut self, src: &str) {
        self.indent_and_print(src);
    }

    /// Generic argument section starts. In WIT this represents the `<` character.
    fn generic_args_start(&mut self) {
        assert!(
            !self.indent_if_needed(),
            "`generic_args_start` is never called after newline"
        );
        self.push_str("<");
    }

    /// Generic argument section ends. In WIT this represents the '>' character.
    fn generic_args_end(&mut self) {
        assert!(
            !self.indent_if_needed(),
            "`generic_args_end` is never called after newline"
        );
        self.push_str(">");
    }

    /// Called when a single documentation line is added.
    /// The `doc` parameter starts with `///` omitted, and can be an empty string.
    fn doc(&mut self, doc: &str) {
        assert!(!doc.contains('\n'));
        self.indent_if_needed();
        self.push_str("///");
        if !doc.is_empty() {
            self.push_str(" ");
            self.push_str(doc);
        }
        self.newline();
    }

    /// A semicolon is added.
    fn semicolon(&mut self) {
        assert!(
            !self.indent_if_needed(),
            "`semicolon` is never called after newline"
        );
        self.push_str(";");
        self.newline();
    }

    /// Any string that does not have a specialized function is added.
    /// Parameter `src` can contain punctation characters, and must be escaped
    /// when outputing to languages like HTML.
    fn str(&mut self, src: &str) {
        self.indent_and_print(src);
    }
}

/// Represents the different kinds of types that can be encountered while
/// visiting a WIT file.
///
/// Each variant refers to the name of the respective element (e.g., function, type, or namespace),
/// not the entire declaration.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TypeKind {
    /// A built-in type, such as "list" or "option".
    BuiltIn,
    /// An enumeration type name.
    Enum,
    /// An error-context type name.
    ErrorContext,
    /// A flags type name.
    Flags,
    /// A freestanding function name, not associated with any specific type or namespace.
    /// For example, "myfunc" in `myfunc: func() -> string;`.
    FunctionFreestanding,
    /// A method, associated with a resource.
    FunctionMethod,
    /// A static function, associated with a resource.
    FunctionStatic,
    /// A future type name.
    Future,
    /// An interface declaration name.
    InterfaceDeclaration,
    /// An interface name when printing a path, for example in `use`.
    InterfacePath,
    /// A list type name.
    List,
    /// A map type name.
    Map,
    /// A namespace declaration.
    NamespaceDeclaration,
    /// A namespace when printing a path, for example in `use`.
    NamespacePath,
    /// An option type name.
    Option,
    /// A package name declaration.
    PackageNameDeclaration,
    /// A package name when printing a path, for example in `use`.
    PackageNamePath,
    /// A record type name.
    Record,
    /// A resource type name.
    Resource,
    /// A result type name.
    Result,
    /// A stream type name.
    Stream,
    /// A tuple type name.
    Tuple,
    /// A type alias.
    TypeAlias,
    /// An imported type name.
    TypeImport,
    /// A user-defined type name.
    TypeName,
    /// A variant type name.
    Variant,
    /// A version declaration.
    VersionDeclaration,
    /// A version when printing a path, for example in `use`.
    VersionPath,
    /// A version when printing stability annotations, for example in `@since`
    VersionAnnotation,
    /// A world declaration name.
    WorldDeclaration,
    /// A fallback for types that do not fit into any other category.
    Other,
}

/// Helper structure to help maintain an indentation level when printing source,
/// modeled after the support in `wit-bindgen-core`. Indentation is set to two spaces.
#[derive(Default)]
pub struct OutputToString {
    indent: usize,
    output: String,
    // set to true after newline, then to false after first item is indented.
    needs_indent: bool,
}

impl Output for OutputToString {
    fn push_str(&mut self, src: &str) {
        self.output.push_str(src);
    }

    fn indent_if_needed(&mut self) -> bool {
        if self.needs_indent {
            for _ in 0..self.indent {
                // Indenting by two spaces.
                self.output.push_str("  ");
            }
            self.needs_indent = false;
            true
        } else {
            false
        }
    }

    fn indent_start(&mut self) {
        assert!(
            !self.needs_indent,
            "`indent_start` is never called after newline"
        );
        self.output.push_str(" {");
        self.indent += 1;
        self.newline();
    }

    fn indent_end(&mut self) {
        // Note that a `saturating_sub` is used here to prevent a panic
        // here in the case of invalid code being generated in debug
        // mode. It's typically easier to debug those issues through
        // looking at the source code rather than getting a panic.
        self.indent = self.indent.saturating_sub(1);
        self.indent_if_needed();
        self.output.push('}');
        self.newline();
    }

    fn newline(&mut self) {
        self.output.push('\n');
        self.needs_indent = true;
    }
}

impl From<OutputToString> for String {
    fn from(output: OutputToString) -> String {
        output.output
    }
}

impl Display for OutputToString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.output.fmt(f)
    }
}

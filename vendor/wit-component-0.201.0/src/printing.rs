use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::mem;
use wit_parser::*;

// NB: keep in sync with `crates/wit-parser/src/ast/lex.rs`
const PRINT_SEMICOLONS_DEFAULT: bool = true;
const PRINT_F32_F64_DEFAULT: bool = false;

/// A utility for printing WebAssembly interface definitions to a string.
pub struct WitPrinter {
    output: Output,

    // Count of how many items in this current block have been printed to print
    // a blank line between each item, but not the first item.
    any_items: bool,

    // Whether to print doc comments.
    emit_docs: bool,

    print_semicolons: bool,
    print_f32_f64: bool,
}

impl Default for WitPrinter {
    fn default() -> Self {
        Self {
            output: Default::default(),
            any_items: false,
            emit_docs: true,
            print_semicolons: match std::env::var("WIT_REQUIRE_SEMICOLONS") {
                Ok(s) => s == "1",
                Err(_) => PRINT_SEMICOLONS_DEFAULT,
            },
            print_f32_f64: match std::env::var("WIT_REQUIRE_F32_F64") {
                Ok(s) => s == "1",
                Err(_) => PRINT_F32_F64_DEFAULT,
            },
        }
    }
}

impl WitPrinter {
    /// Configure whether doc comments will be printed.
    ///
    /// Defaults to true.
    pub fn emit_docs(&mut self, enabled: bool) -> &mut Self {
        self.emit_docs = enabled;
        self
    }

    /// Print the given WIT package to a string.
    pub fn print(&mut self, resolve: &Resolve, pkgid: PackageId) -> Result<String> {
        let pkg = &resolve.packages[pkgid];
        self.print_docs(&pkg.docs);
        self.output.push_str("package ");
        self.print_name(&pkg.name.namespace);
        self.output.push_str(":");
        self.print_name(&pkg.name.name);
        if let Some(version) = &pkg.name.version {
            self.output.push_str(&format!("@{version}"));
        }
        self.print_semicolon();
        self.output.push_str("\n\n");
        for (name, id) in pkg.interfaces.iter() {
            self.print_docs(&resolve.interfaces[*id].docs);
            self.output.push_str("interface ");
            self.print_name(name);
            self.output.push_str(" {\n");
            self.print_interface(resolve, *id)?;
            writeln!(&mut self.output, "}}\n")?;
        }

        for (name, id) in pkg.worlds.iter() {
            self.print_docs(&resolve.worlds[*id].docs);
            self.output.push_str("world ");
            self.print_name(name);
            self.output.push_str(" {\n");
            self.print_world(resolve, *id)?;
            writeln!(&mut self.output, "}}")?;
        }

        Ok(std::mem::take(&mut self.output).into())
    }

    fn print_semicolon(&mut self) {
        if self.print_semicolons {
            self.output.push_str(";");
        }
    }

    fn new_item(&mut self) {
        if self.any_items {
            self.output.push_str("\n");
        }
        self.any_items = true;
    }

    /// Print the given WebAssembly interface to a string.
    fn print_interface(&mut self, resolve: &Resolve, id: InterfaceId) -> Result<()> {
        let prev_items = mem::replace(&mut self.any_items, false);
        let interface = &resolve.interfaces[id];

        let mut resource_funcs = HashMap::new();
        let mut freestanding = Vec::new();
        for (name, func) in interface.functions.iter() {
            if let Some(id) = resource_func(func) {
                resource_funcs.entry(id).or_insert(Vec::new()).push(func);
            } else {
                freestanding.push((name, func));
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

        for (name, func) in freestanding {
            self.new_item();
            self.print_docs(&func.docs);
            self.print_name(name);
            self.output.push_str(": ");
            self.print_function(resolve, func)?;
            self.print_semicolon();
            self.output.push_str("\n");
        }

        self.any_items = prev_items;

        Ok(())
    }

    fn print_types<'a>(
        &mut self,
        resolve: &Resolve,
        owner: TypeOwner,
        types: impl Iterator<Item = (&'a str, TypeId)>,
        resource_funcs: &HashMap<TypeId, Vec<&Function>>,
    ) -> Result<()> {
        // Partition types defined in this interface into either those imported
        // from foreign interfaces or those defined locally.
        let mut types_to_declare = Vec::new();
        let mut types_to_import: Vec<(_, Vec<_>)> = Vec::new();
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
                        if let Some((owner, list)) = types_to_import.last_mut() {
                            if *owner == other_owner {
                                list.push((name, other_name));
                                continue;
                            }
                        }
                        types_to_import.push((other_owner, vec![(name, other_name)]));
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
        for (owner, tys) in types_to_import {
            self.any_items = true;
            write!(&mut self.output, "use ")?;
            let id = match owner {
                TypeOwner::Interface(id) => id,
                // it's only possible to import types from interfaces at
                // this time.
                _ => unreachable!(),
            };
            self.print_path_to_interface(resolve, id, my_pkg)?;
            write!(&mut self.output, ".{{")?;
            for (i, (my_name, other_name)) in tys.into_iter().enumerate() {
                if i > 0 {
                    write!(&mut self.output, ", ")?;
                }
                if my_name == other_name {
                    self.print_name(my_name);
                } else {
                    self.print_name(other_name);
                    self.output.push_str(" as ");
                    self.print_name(my_name);
                }
            }
            write!(&mut self.output, "}}")?;
            self.print_semicolon();
            self.output.push_str("\n");
        }

        for id in types_to_declare {
            self.new_item();
            self.print_docs(&resolve.types[id].docs);
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
        self.output.push_str("resource ");
        self.print_name(ty.name.as_ref().expect("resources must be named"));
        if funcs.is_empty() {
            self.print_semicolon();
            self.output.push_str("\n");
            return Ok(());
        }
        self.output.push_str(" {\n");
        for func in funcs {
            match &func.kind {
                FunctionKind::Constructor(_) => {
                    self.print_docs(&func.docs);
                }
                FunctionKind::Method(_) => {
                    self.print_docs(&func.docs);
                    self.print_name(func.item_name());
                    self.output.push_str(": ");
                }
                FunctionKind::Static(_) => {
                    self.print_docs(&func.docs);
                    self.print_name(func.item_name());
                    self.output.push_str(": ");
                    self.output.push_str("static ");
                }
                FunctionKind::Freestanding => unreachable!(),
            }
            self.print_function(resolve, func)?;
            self.print_semicolon();
            self.output.push_str("\n");
        }
        self.output.push_str("}\n");

        Ok(())
    }

    fn print_function(&mut self, resolve: &Resolve, func: &Function) -> Result<()> {
        // Constructors are named slightly differently.
        match &func.kind {
            FunctionKind::Constructor(_) => self.output.push_str("constructor("),
            _ => self.output.push_str("func("),
        }

        // Methods don't print their `self` argument
        let params_to_skip = match &func.kind {
            FunctionKind::Method(_) => 1,
            _ => 0,
        };
        for (i, (name, ty)) in func.params.iter().skip(params_to_skip).enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            self.print_name(name);
            self.output.push_str(": ");
            self.print_type_name(resolve, ty)?;
        }
        self.output.push_str(")");

        // constructors don't have their results printed
        if let FunctionKind::Constructor(_) = func.kind {
            return Ok(());
        }

        match &func.results {
            Results::Named(rs) => match rs.len() {
                0 => (),
                _ => {
                    self.output.push_str(" -> (");
                    for (i, (name, ty)) in rs.iter().enumerate() {
                        if i > 0 {
                            self.output.push_str(", ");
                        }
                        self.print_name(name);
                        self.output.push_str(": ");
                        self.print_type_name(resolve, ty)?;
                    }
                    self.output.push_str(")");
                }
            },
            Results::Anon(ty) => {
                self.output.push_str(" -> ");
                self.print_type_name(resolve, ty)?;
            }
        }
        Ok(())
    }

    fn print_world(&mut self, resolve: &Resolve, id: WorldId) -> Result<()> {
        let prev_items = mem::replace(&mut self.any_items, false);
        let world = &resolve.worlds[id];
        let pkgid = world.package.unwrap();
        let mut types = Vec::new();
        let mut resource_funcs = HashMap::new();
        for (name, import) in world.imports.iter() {
            match import {
                WorldItem::Type(t) => match name {
                    WorldKey::Name(s) => types.push((s.as_str(), *t)),
                    WorldKey::Interface(_) => unreachable!(),
                },
                _ => {
                    if let WorldItem::Function(f) = import {
                        if let Some(id) = resource_func(f) {
                            resource_funcs.entry(id).or_insert(Vec::new()).push(f);
                            continue;
                        }
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
        desc: &str,
    ) -> Result<()> {
        // Print inline item docs
        if matches!(name, WorldKey::Name(_)) {
            self.print_docs(match item {
                WorldItem::Interface(id) => &resolve.interfaces[*id].docs,
                WorldItem::Function(f) => &f.docs,
                // Types are handled separately
                WorldItem::Type(_) => unreachable!(),
            });
        }

        self.output.push_str(desc);
        self.output.push_str(" ");
        match name {
            WorldKey::Name(name) => {
                self.print_name(name);
                self.output.push_str(": ");
                match item {
                    WorldItem::Interface(id) => {
                        assert!(resolve.interfaces[*id].name.is_none());
                        writeln!(self.output, "interface {{")?;
                        self.print_interface(resolve, *id)?;
                        writeln!(self.output, "}}")?;
                    }
                    WorldItem::Function(f) => {
                        self.print_function(resolve, f)?;
                        self.print_semicolon();
                        self.output.push_str("\n");
                    }
                    // Types are handled separately
                    WorldItem::Type(_) => unreachable!(),
                }
            }
            WorldKey::Interface(id) => {
                match item {
                    WorldItem::Interface(id2) => assert_eq!(id, id2),
                    _ => unreachable!(),
                }
                self.print_path_to_interface(resolve, *id, cur_pkg)?;
                self.print_semicolon();
                self.output.push_str("\n");
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
            self.print_name(iface.name.as_ref().unwrap());
        } else {
            let pkg = &resolve.packages[iface.package.unwrap()].name;
            self.print_name(&pkg.namespace);
            self.output.push_str(":");
            self.print_name(&pkg.name);
            self.output.push_str("/");
            self.print_name(iface.name.as_ref().unwrap());
            if let Some(version) = &pkg.version {
                self.output.push_str(&format!("@{version}"));
            }
        }
        Ok(())
    }

    fn print_type_name(&mut self, resolve: &Resolve, ty: &Type) -> Result<()> {
        match ty {
            Type::Bool => self.output.push_str("bool"),
            Type::U8 => self.output.push_str("u8"),
            Type::U16 => self.output.push_str("u16"),
            Type::U32 => self.output.push_str("u32"),
            Type::U64 => self.output.push_str("u64"),
            Type::S8 => self.output.push_str("s8"),
            Type::S16 => self.output.push_str("s16"),
            Type::S32 => self.output.push_str("s32"),
            Type::S64 => self.output.push_str("s64"),
            Type::Float32 => {
                if self.print_f32_f64 {
                    self.output.push_str("f32")
                } else {
                    self.output.push_str("float32")
                }
            }
            Type::Float64 => {
                if self.print_f32_f64 {
                    self.output.push_str("f64")
                } else {
                    self.output.push_str("float64")
                }
            }
            Type::Char => self.output.push_str("char"),
            Type::String => self.output.push_str("string"),

            Type::Id(id) => {
                let ty = &resolve.types[*id];
                if let Some(name) = &ty.name {
                    self.print_name(name);
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
                        self.output.push_str("list<");
                        self.print_type_name(resolve, ty)?;
                        self.output.push_str(">");
                    }
                    TypeDefKind::Type(ty) => self.print_type_name(resolve, ty)?,
                    TypeDefKind::Future(_) => {
                        todo!("document has an unnamed future type")
                    }
                    TypeDefKind::Stream(_) => {
                        todo!("document has an unnamed stream type")
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
                    self.output.push_str("own<");
                }
                self.print_name(
                    ty.name
                        .as_ref()
                        .ok_or_else(|| anyhow!("unnamed resource type"))?,
                );
                if force_handle_type_printed {
                    self.output.push_str(">");
                }
            }

            Handle::Borrow(ty) => {
                self.output.push_str("borrow<");
                let ty = &resolve.types[*ty];
                self.print_name(
                    ty.name
                        .as_ref()
                        .ok_or_else(|| anyhow!("unnamed resource type"))?,
                );
                self.output.push_str(">");
            }
        }

        Ok(())
    }

    fn print_tuple_type(&mut self, resolve: &Resolve, tuple: &Tuple) -> Result<()> {
        self.output.push_str("tuple<");
        for (i, ty) in tuple.types.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            self.print_type_name(resolve, ty)?;
        }
        self.output.push_str(">");

        Ok(())
    }

    fn print_option_type(&mut self, resolve: &Resolve, payload: &Type) -> Result<()> {
        self.output.push_str("option<");
        self.print_type_name(resolve, payload)?;
        self.output.push_str(">");
        Ok(())
    }

    fn print_result_type(&mut self, resolve: &Resolve, result: &Result_) -> Result<()> {
        match result {
            Result_ {
                ok: Some(ok),
                err: Some(err),
            } => {
                self.output.push_str("result<");
                self.print_type_name(resolve, ok)?;
                self.output.push_str(", ");
                self.print_type_name(resolve, err)?;
                self.output.push_str(">");
            }
            Result_ {
                ok: None,
                err: Some(err),
            } => {
                self.output.push_str("result<_, ");
                self.print_type_name(resolve, err)?;
                self.output.push_str(">");
            }
            Result_ {
                ok: Some(ok),
                err: None,
            } => {
                self.output.push_str("result<");
                self.print_type_name(resolve, ok)?;
                self.output.push_str(">");
            }
            Result_ {
                ok: None,
                err: None,
            } => {
                self.output.push_str("result");
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
            | Type::Float32
            | Type::Float64
            | Type::Char
            | Type::String => return Ok(()),

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
                    TypeDefKind::Type(inner) => match ty.name.as_deref() {
                        Some(name) => {
                            self.output.push_str("type ");
                            self.print_name(name);
                            self.output.push_str(" = ");
                            self.print_type_name(resolve, inner)?;
                            self.print_semicolon();
                            self.output.push_str("\n");
                        }
                        None => bail!("unnamed type in document"),
                    },
                    TypeDefKind::Future(_) => todo!("declare future"),
                    TypeDefKind::Stream(_) => todo!("declare stream"),
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
                self.output.push_str("type ");
                self.print_name(name);
                self.output.push_str(" = ");
                // Note that the `true` here forces owned handles to be printed
                // as `own<T>`. The purpose of this is because `type a = b`, if
                // `b` is a resource, is encoded differently as `type a =
                // own<b>`. By forcing a handle to be printed here it's staying
                // true to what's in the WIT document.
                self.print_handle_type(resolve, handle, true)?;
                self.print_semicolon();
                self.output.push_str("\n");

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
                self.output.push_str("record ");
                self.print_name(name);
                self.output.push_str(" {\n");
                for field in &record.fields {
                    self.print_docs(&field.docs);
                    self.print_name(&field.name);
                    self.output.push_str(": ");
                    self.print_type_name(resolve, &field.ty)?;
                    self.output.push_str(",\n");
                }
                self.output.push_str("}\n");
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
            self.output.push_str("type ");
            self.print_name(name);
            self.output.push_str(" = ");
            self.print_tuple_type(resolve, tuple)?;
            self.print_semicolon();
            self.output.push_str("\n");
        }
        Ok(())
    }

    fn declare_flags(&mut self, name: Option<&str>, flags: &Flags) -> Result<()> {
        match name {
            Some(name) => {
                self.output.push_str("flags ");
                self.print_name(name);
                self.output.push_str(" {\n");
                for flag in &flags.flags {
                    self.print_docs(&flag.docs);
                    self.print_name(&flag.name);
                    self.output.push_str(",\n");
                }
                self.output.push_str("}\n");
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
        self.output.push_str("variant ");
        self.print_name(name);
        self.output.push_str(" {\n");
        for case in &variant.cases {
            self.print_docs(&case.docs);
            self.print_name(&case.name);
            if let Some(ty) = case.ty {
                self.output.push_str("(");
                self.print_type_name(resolve, &ty)?;
                self.output.push_str(")");
            }
            self.output.push_str(",\n");
        }
        self.output.push_str("}\n");
        Ok(())
    }

    fn declare_option(
        &mut self,
        resolve: &Resolve,
        name: Option<&str>,
        payload: &Type,
    ) -> Result<()> {
        if let Some(name) = name {
            self.output.push_str("type ");
            self.print_name(name);
            self.output.push_str(" = ");
            self.print_option_type(resolve, payload)?;
            self.print_semicolon();
            self.output.push_str("\n");
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
            self.output.push_str("type ");
            self.print_name(name);
            self.output.push_str(" = ");
            self.print_result_type(resolve, result)?;
            self.print_semicolon();
            self.output.push_str("\n");
        }
        Ok(())
    }

    fn declare_enum(&mut self, name: Option<&str>, enum_: &Enum) -> Result<()> {
        let name = match name {
            Some(name) => name,
            None => bail!("document has unnamed enum type"),
        };
        self.output.push_str("enum ");
        self.print_name(name);
        self.output.push_str(" {\n");
        for case in &enum_.cases {
            self.print_docs(&case.docs);
            self.print_name(&case.name);
            self.output.push_str(",\n");
        }
        self.output.push_str("}\n");
        Ok(())
    }

    fn declare_list(&mut self, resolve: &Resolve, name: Option<&str>, ty: &Type) -> Result<()> {
        if let Some(name) = name {
            self.output.push_str("type ");
            self.print_name(name);
            self.output.push_str(" = list<");
            self.print_type_name(resolve, ty)?;
            self.output.push_str(">");
            self.print_semicolon();
            self.output.push_str("\n");
            return Ok(());
        }

        Ok(())
    }

    fn print_name(&mut self, name: &str) {
        if is_keyword(name) {
            self.output.push_str("%");
        }
        self.output.push_str(name);
    }

    fn print_docs(&mut self, docs: &Docs) {
        if self.emit_docs {
            if let Some(contents) = &docs.contents {
                for line in contents.lines() {
                    self.output.push_str("/// ");
                    self.output.push_str(line);
                    self.output.push_str("\n");
                }
            }
        }
    }
}

fn resource_func(f: &Function) -> Option<TypeId> {
    match f.kind {
        FunctionKind::Freestanding => None,
        FunctionKind::Method(id) | FunctionKind::Constructor(id) | FunctionKind::Static(id) => {
            Some(id)
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
    )
}

/// Helper structure to help maintain an indentation level when printing source,
/// modeled after the support in `wit-bindgen-core`.
#[derive(Default)]
struct Output {
    indent: usize,
    output: String,
}

impl Output {
    fn push_str(&mut self, src: &str) {
        let lines = src.lines().collect::<Vec<_>>();
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with('}') && self.output.ends_with("  ") {
                self.output.pop();
                self.output.pop();
            }
            self.output.push_str(if lines.len() == 1 {
                line
            } else {
                line.trim_start()
            });
            if trimmed.ends_with('{') {
                self.indent += 1;
            }
            if trimmed.starts_with('}') {
                // Note that a `saturating_sub` is used here to prevent a panic
                // here in the case of invalid code being generated in debug
                // mode. It's typically easier to debug those issues through
                // looking at the source code rather than getting a panic.
                self.indent = self.indent.saturating_sub(1);
            }
            if i != lines.len() - 1 || src.ends_with('\n') {
                // Trim trailing whitespace, if any, then push an indented
                // newline
                while let Some(c) = self.output.chars().next_back() {
                    if c.is_whitespace() && c != '\n' {
                        self.output.pop();
                    } else {
                        break;
                    }
                }
                self.output.push('\n');
                for _ in 0..self.indent {
                    self.output.push_str("  ");
                }
            }
        }
    }
}

impl Write for Output {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl From<Output> for String {
    fn from(output: Output) -> String {
        output.output
    }
}

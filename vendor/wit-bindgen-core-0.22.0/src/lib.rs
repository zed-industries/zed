use std::collections::HashMap;
use std::fmt::Write;

use anyhow::Result;
pub use wit_parser;
use wit_parser::*;
pub mod abi;
mod ns;
pub use ns::Ns;
pub mod source;
pub use source::{Files, Source};

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    #[default]
    Import,
    Export,
}

#[derive(Default)]
pub struct Types {
    type_info: HashMap<TypeId, TypeInfo>,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct TypeInfo {
    /// Whether or not this type is ever used (transitively) within the
    /// parameter of an imported function.
    ///
    /// This means that it's used in a context where ownership isn't
    /// relinquished.
    pub borrowed: bool,

    /// Whether or not this type is ever used (transitively) within the
    /// parameter or result of an export, or the result of an import.
    ///
    /// This means that it's used in a context where ownership is required and
    /// memory management is necessary.
    pub owned: bool,

    /// Whether or not this type is ever used (transitively) within the
    /// error case in the result of a function.
    pub error: bool,

    /// Whether or not this type (transitively) has a list (or string).
    pub has_list: bool,

    /// Whether or not this type (transitively) has a resource (or handle).
    pub has_resource: bool,

    /// Whether or not this type (transitively) has a borrow handle.
    pub has_borrow_handle: bool,

    /// Whether or not this type (transitively) has an own handle.
    pub has_own_handle: bool,
}

impl std::ops::BitOrAssign for TypeInfo {
    fn bitor_assign(&mut self, rhs: Self) {
        self.borrowed |= rhs.borrowed;
        self.owned |= rhs.owned;
        self.error |= rhs.error;
        self.has_list |= rhs.has_list;
        self.has_resource |= rhs.has_resource;
        self.has_borrow_handle |= rhs.has_borrow_handle;
        self.has_own_handle |= rhs.has_own_handle;
    }
}

impl TypeInfo {
    pub fn is_clone(&self) -> bool {
        !self.has_resource
    }
    pub fn is_copy(&self) -> bool {
        !self.has_list && !self.has_resource
    }
}

impl Types {
    pub fn analyze(&mut self, resolve: &Resolve) {
        for (t, _) in resolve.types.iter() {
            self.type_id_info(resolve, t);
        }
        for (_, world) in resolve.worlds.iter() {
            for (import, (_, item)) in world
                .imports
                .iter()
                .map(|i| (true, i))
                .chain(world.exports.iter().map(|i| (false, i)))
            {
                match item {
                    WorldItem::Function(f) => {
                        self.type_info_func(resolve, f, import);
                    }
                    WorldItem::Interface(id) => {
                        for (_, f) in resolve.interfaces[*id].functions.iter() {
                            self.type_info_func(resolve, f, import);
                        }
                    }
                    WorldItem::Type(_) => {}
                }
            }
        }
    }

    fn type_info_func(&mut self, resolve: &Resolve, func: &Function, import: bool) {
        let mut live = LiveTypes::default();
        for (_, ty) in func.params.iter() {
            self.type_info(resolve, ty);
            live.add_type(resolve, ty);
        }
        for id in live.iter() {
            if resolve.types[id].name.is_some() {
                let info = self.type_info.get_mut(&id).unwrap();
                if import {
                    info.borrowed = true;
                } else {
                    info.owned = true;
                }
            }
        }
        let mut live = LiveTypes::default();
        for ty in func.results.iter_types() {
            self.type_info(resolve, ty);
            live.add_type(resolve, ty);
        }
        for id in live.iter() {
            if resolve.types[id].name.is_some() {
                self.type_info.get_mut(&id).unwrap().owned = true;
            }
        }

        for ty in func.results.iter_types() {
            let id = match ty {
                Type::Id(id) => *id,
                _ => continue,
            };
            let err = match &resolve.types[id].kind {
                TypeDefKind::Result(Result_ { err, .. }) => err,
                _ => continue,
            };
            if let Some(Type::Id(id)) = err {
                // When an interface `use`s a type from another interface, it creates a new typeid
                // referring to the definition typeid. Chase any chain of references down to the
                // typeid of the definition.
                fn resolve_type_definition_id(resolve: &Resolve, mut id: TypeId) -> TypeId {
                    loop {
                        match resolve.types[id].kind {
                            TypeDefKind::Type(Type::Id(def_id)) => id = def_id,
                            _ => return id,
                        }
                    }
                }
                let id = resolve_type_definition_id(resolve, *id);
                self.type_info.get_mut(&id).unwrap().error = true;
            }
        }
    }

    pub fn get(&self, id: TypeId) -> TypeInfo {
        self.type_info[&id]
    }

    pub fn type_id_info(&mut self, resolve: &Resolve, ty: TypeId) -> TypeInfo {
        if let Some(info) = self.type_info.get(&ty) {
            return *info;
        }
        let mut info = TypeInfo::default();
        match &resolve.types[ty].kind {
            TypeDefKind::Record(r) => {
                for field in r.fields.iter() {
                    info |= self.type_info(resolve, &field.ty);
                }
            }
            TypeDefKind::Resource => {
                info.has_resource = true;
            }
            TypeDefKind::Handle(handle) => {
                match handle {
                    Handle::Borrow(_) => info.has_borrow_handle = true,
                    Handle::Own(_) => info.has_own_handle = true,
                }
                info.has_resource = true;
            }
            TypeDefKind::Tuple(t) => {
                for ty in t.types.iter() {
                    info |= self.type_info(resolve, ty);
                }
            }
            TypeDefKind::Flags(_) => {}
            TypeDefKind::Enum(_) => {}
            TypeDefKind::Variant(v) => {
                for case in v.cases.iter() {
                    info |= self.optional_type_info(resolve, case.ty.as_ref());
                }
            }
            TypeDefKind::List(ty) => {
                info = self.type_info(resolve, ty);
                info.has_list = true;
            }
            TypeDefKind::Type(ty) => {
                info = self.type_info(resolve, ty);
            }
            TypeDefKind::Option(ty) => {
                info = self.type_info(resolve, ty);
            }
            TypeDefKind::Result(r) => {
                info = self.optional_type_info(resolve, r.ok.as_ref());
                info |= self.optional_type_info(resolve, r.err.as_ref());
            }
            TypeDefKind::Future(ty) => {
                info = self.optional_type_info(resolve, ty.as_ref());
            }
            TypeDefKind::Stream(stream) => {
                info = self.optional_type_info(resolve, stream.element.as_ref());
                info |= self.optional_type_info(resolve, stream.end.as_ref());
            }
            TypeDefKind::Unknown => unreachable!(),
        }
        let prev = self.type_info.insert(ty, info);
        assert!(prev.is_none());
        info
    }

    pub fn type_info(&mut self, resolve: &Resolve, ty: &Type) -> TypeInfo {
        let mut info = TypeInfo::default();
        match ty {
            Type::String => info.has_list = true,
            Type::Id(id) => return self.type_id_info(resolve, *id),
            _ => {}
        }
        info
    }

    fn optional_type_info(&mut self, resolve: &Resolve, ty: Option<&Type>) -> TypeInfo {
        match ty {
            Some(ty) => self.type_info(resolve, ty),
            None => TypeInfo::default(),
        }
    }
}

pub trait WorldGenerator {
    fn generate(&mut self, resolve: &Resolve, id: WorldId, files: &mut Files) -> Result<()> {
        let world = &resolve.worlds[id];
        self.preprocess(resolve, id);

        fn unwrap_name(key: &WorldKey) -> &str {
            match key {
                WorldKey::Name(name) => name,
                WorldKey::Interface(_) => panic!("unexpected interface key"),
            }
        }

        let mut funcs = Vec::new();
        let mut types = Vec::new();
        for (name, import) in world.imports.iter() {
            match import {
                WorldItem::Function(f) => funcs.push((unwrap_name(name), f)),
                WorldItem::Interface(id) => self.import_interface(resolve, name, *id, files),
                WorldItem::Type(id) => types.push((unwrap_name(name), *id)),
            }
        }
        if !types.is_empty() {
            self.import_types(resolve, id, &types, files);
        }
        if !funcs.is_empty() {
            self.import_funcs(resolve, id, &funcs, files);
        }
        funcs.clear();

        self.finish_imports(resolve, id, files);

        // First generate bindings for any freestanding functions, if any. If
        // these refer to types defined in the world they need to refer to the
        // imported types generated above.
        //
        // Interfaces are then generated afterwards so if the same interface is
        // both imported and exported the right types are all used everywhere.
        let mut interfaces = Vec::new();
        for (name, export) in world.exports.iter() {
            match export {
                WorldItem::Function(f) => funcs.push((unwrap_name(name), f)),
                WorldItem::Interface(id) => interfaces.push((name, id)),
                WorldItem::Type(_) => unreachable!(),
            }
        }
        if !funcs.is_empty() {
            self.export_funcs(resolve, id, &funcs, files)?;
        }

        self.pre_export_interface(resolve, files)?;

        for (name, id) in interfaces {
            self.export_interface(resolve, name, *id, files)?;
        }
        self.finish(resolve, id, files)
    }

    fn finish_imports(&mut self, resolve: &Resolve, world: WorldId, files: &mut Files) {
        let _ = (resolve, world, files);
    }

    fn preprocess(&mut self, resolve: &Resolve, world: WorldId) {
        let _ = (resolve, world);
    }

    fn import_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        iface: InterfaceId,
        files: &mut Files,
    );

    /// Called before any exported interfaces are generated.
    fn pre_export_interface(&mut self, resolve: &Resolve, files: &mut Files) -> Result<()> {
        let _ = (resolve, files);
        Ok(())
    }

    fn export_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        iface: InterfaceId,
        files: &mut Files,
    ) -> Result<()>;
    fn import_funcs(
        &mut self,
        resolve: &Resolve,
        world: WorldId,
        funcs: &[(&str, &Function)],
        files: &mut Files,
    );
    fn export_funcs(
        &mut self,
        resolve: &Resolve,
        world: WorldId,
        funcs: &[(&str, &Function)],
        files: &mut Files,
    ) -> Result<()>;
    fn import_types(
        &mut self,
        resolve: &Resolve,
        world: WorldId,
        types: &[(&str, TypeId)],
        files: &mut Files,
    );
    fn finish(&mut self, resolve: &Resolve, world: WorldId, files: &mut Files) -> Result<()>;
}

/// This is a possible replacement for the `Generator` trait above, currently
/// only used by the JS bindings for generating bindings for a component.
///
/// The current plan is to see how things shake out with worlds and various
/// other generators to see if everything can be updated to a less
/// per-`*.wit`-file centric interface in the future. Even this will probably
/// change for JS though. In any case it's something that was useful for JS and
/// is suitable to replace otherwise at any time.
pub trait InterfaceGenerator<'a> {
    fn resolve(&self) -> &'a Resolve;

    fn type_record(&mut self, id: TypeId, name: &str, record: &Record, docs: &Docs);
    fn type_resource(&mut self, id: TypeId, name: &str, docs: &Docs);
    fn type_flags(&mut self, id: TypeId, name: &str, flags: &Flags, docs: &Docs);
    fn type_tuple(&mut self, id: TypeId, name: &str, flags: &Tuple, docs: &Docs);
    fn type_variant(&mut self, id: TypeId, name: &str, variant: &Variant, docs: &Docs);
    fn type_option(&mut self, id: TypeId, name: &str, payload: &Type, docs: &Docs);
    fn type_result(&mut self, id: TypeId, name: &str, result: &Result_, docs: &Docs);
    fn type_enum(&mut self, id: TypeId, name: &str, enum_: &Enum, docs: &Docs);
    fn type_alias(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs);
    fn type_list(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs);
    fn type_builtin(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs);

    fn types(&mut self, iface: InterfaceId) {
        let iface = &self.resolve().interfaces[iface];
        for (name, id) in iface.types.iter() {
            self.define_type(name, *id);
        }
    }

    fn define_type(&mut self, name: &str, id: TypeId) {
        let ty = &self.resolve().types[id];
        match &ty.kind {
            TypeDefKind::Record(record) => self.type_record(id, name, record, &ty.docs),
            TypeDefKind::Resource => self.type_resource(id, name, &ty.docs),
            TypeDefKind::Flags(flags) => self.type_flags(id, name, flags, &ty.docs),
            TypeDefKind::Tuple(tuple) => self.type_tuple(id, name, tuple, &ty.docs),
            TypeDefKind::Enum(enum_) => self.type_enum(id, name, enum_, &ty.docs),
            TypeDefKind::Variant(variant) => self.type_variant(id, name, variant, &ty.docs),
            TypeDefKind::Option(t) => self.type_option(id, name, t, &ty.docs),
            TypeDefKind::Result(r) => self.type_result(id, name, r, &ty.docs),
            TypeDefKind::List(t) => self.type_list(id, name, t, &ty.docs),
            TypeDefKind::Type(t) => self.type_alias(id, name, t, &ty.docs),
            TypeDefKind::Future(_) => todo!("generate for future"),
            TypeDefKind::Stream(_) => todo!("generate for stream"),
            TypeDefKind::Handle(_) => todo!("generate for handle"),
            TypeDefKind::Unknown => unreachable!(),
        }
    }
}

pub fn generated_preamble(src: &mut Source, version: &str) {
    uwriteln!(src, "// Generated by `wit-bindgen` {version}. DO NOT EDIT!")
}

pub fn dealias(resolve: &Resolve, mut id: TypeId) -> TypeId {
    loop {
        match &resolve.types[id].kind {
            TypeDefKind::Type(Type::Id(that_id)) => id = *that_id,
            _ => break id,
        }
    }
}

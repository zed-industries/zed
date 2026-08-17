use std::fmt::Write;

use anyhow::Result;
pub use wit_parser;
use wit_parser::*;
pub mod abi;
mod ns;
pub use ns::Ns;
pub mod source;
pub use source::{Files, Source};
mod types;
pub use types::{TypeInfo, Types};
mod path;
pub use path::name_package_module;
mod async_;
pub use async_::AsyncFilterSet;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    #[default]
    Import,
    Export,
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
                WorldItem::Interface { id, .. } => {
                    self.import_interface(resolve, name, *id, files)?
                }
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
                WorldItem::Interface { id, .. } => interfaces.push((name, id)),
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
    ) -> Result<()>;

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
    fn type_tuple(&mut self, id: TypeId, name: &str, tuple: &Tuple, docs: &Docs);
    fn type_variant(&mut self, id: TypeId, name: &str, variant: &Variant, docs: &Docs);
    fn type_option(&mut self, id: TypeId, name: &str, payload: &Type, docs: &Docs);
    fn type_result(&mut self, id: TypeId, name: &str, result: &Result_, docs: &Docs);
    fn type_enum(&mut self, id: TypeId, name: &str, enum_: &Enum, docs: &Docs);
    fn type_alias(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs);
    fn type_list(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs);
    fn type_builtin(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs);
    fn type_future(&mut self, id: TypeId, name: &str, ty: &Option<Type>, docs: &Docs);
    fn type_stream(&mut self, id: TypeId, name: &str, ty: &Option<Type>, docs: &Docs);
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
            TypeDefKind::Future(t) => self.type_future(id, name, t, &ty.docs),
            TypeDefKind::Stream(t) => self.type_stream(id, name, t, &ty.docs),
            TypeDefKind::Handle(_) => panic!("handle types do not require definition"),
            TypeDefKind::FixedSizeList(..) => todo!(),
            TypeDefKind::Map(..) => todo!(),
            TypeDefKind::Unknown => unreachable!(),
        }
    }
}

pub trait AnonymousTypeGenerator<'a> {
    fn resolve(&self) -> &'a Resolve;

    fn anonymous_type_handle(&mut self, id: TypeId, handle: &Handle, docs: &Docs);
    fn anonymous_type_tuple(&mut self, id: TypeId, ty: &Tuple, docs: &Docs);
    fn anonymous_type_option(&mut self, id: TypeId, ty: &Type, docs: &Docs);
    fn anonymous_type_result(&mut self, id: TypeId, ty: &Result_, docs: &Docs);
    fn anonymous_type_list(&mut self, id: TypeId, ty: &Type, docs: &Docs);
    fn anonymous_type_future(&mut self, id: TypeId, ty: &Option<Type>, docs: &Docs);
    fn anonymous_type_stream(&mut self, id: TypeId, ty: &Option<Type>, docs: &Docs);
    fn anonymous_type_type(&mut self, id: TypeId, ty: &Type, docs: &Docs);

    fn define_anonymous_type(&mut self, id: TypeId) {
        let ty = &self.resolve().types[id];
        match &ty.kind {
            TypeDefKind::Flags(_)
            | TypeDefKind::Record(_)
            | TypeDefKind::Resource
            | TypeDefKind::Enum(_)
            | TypeDefKind::Variant(_) => {
                unreachable!()
            }
            TypeDefKind::Type(t) => self.anonymous_type_type(id, t, &ty.docs),
            TypeDefKind::Tuple(tuple) => self.anonymous_type_tuple(id, tuple, &ty.docs),
            TypeDefKind::Option(t) => self.anonymous_type_option(id, t, &ty.docs),
            TypeDefKind::Result(r) => self.anonymous_type_result(id, r, &ty.docs),
            TypeDefKind::List(t) => self.anonymous_type_list(id, t, &ty.docs),
            TypeDefKind::Future(f) => self.anonymous_type_future(id, f, &ty.docs),
            TypeDefKind::Stream(s) => self.anonymous_type_stream(id, s, &ty.docs),
            TypeDefKind::Handle(handle) => self.anonymous_type_handle(id, handle, &ty.docs),
            TypeDefKind::FixedSizeList(..) => todo!(),
            TypeDefKind::Map(..) => todo!(),
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

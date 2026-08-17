//! A helper, non public, module to assist with cloning items within a
//! `Resolve`.
//!
//! Cloning items is not as simple as calling `Clone` due to the nature of how
//! `TypeId` tracks relationships between interfaces and types. A full deep
//! clone requires walking the full structure and allocating new id links. This
//! is akin to, for example, creating a deep copy of an `Rc<T>` by calling
//! `Clone for T`.
//!
//! This is currently used when merging worlds together to help copy anonymously
//! named items from one world to another.
//!
//! The general structure of this module is that each method takes a mutable
//! reference to an AST item and updates it as necessary internally, delegating
//! to other methods for internal AST items.
//!
//! This module does not at the time of this writing have full support for
//! cloning everything within a `Resolve`.

use crate::*;
use std::collections::HashMap;

/// Represents the results of cloning types and/or interfaces as part of a
/// `Resolve::merge_worlds` operation.
#[derive(Default)]
pub struct CloneMaps {
    pub(super) types: HashMap<TypeId, TypeId>,
    pub(super) interfaces: HashMap<InterfaceId, InterfaceId>,
}

impl CloneMaps {
    /// The types cloned during a `Resolve::merge_worlds` operation.
    ///
    /// The key is the original type, and the value is the clone.
    pub fn types(&self) -> &HashMap<TypeId, TypeId> {
        &self.types
    }

    /// The interfaces cloned during a `Resolve::merge_worlds` operation.
    ///
    /// The key is the original interface, and the value is the clone.
    pub fn interfaces(&self) -> &HashMap<InterfaceId, InterfaceId> {
        &self.interfaces
    }
}

pub struct Cloner<'a> {
    pub resolve: &'a mut Resolve,
    prev_owner: TypeOwner,
    new_owner: TypeOwner,

    /// This map keeps track, in the current scope of types, of all copied
    /// types. This deduplicates copying types to ensure that they're only
    /// copied at most once.
    pub types: HashMap<TypeId, TypeId>,

    /// If `None` then it's inferred from `self.new_owner`.
    pub new_package: Option<PackageId>,
}

impl<'a> Cloner<'a> {
    pub fn new(
        resolve: &'a mut Resolve,
        prev_owner: TypeOwner,
        new_owner: TypeOwner,
    ) -> Cloner<'a> {
        Cloner {
            prev_owner,
            new_owner,
            resolve,
            types: Default::default(),
            new_package: None,
        }
    }

    pub fn register_world_type_overlap(&mut self, from: WorldId, into: WorldId) {
        let into = &self.resolve.worlds[into];
        let from = &self.resolve.worlds[from];
        for (name, into_import) in into.imports.iter() {
            let WorldKey::Name(_) = name else { continue };
            let WorldItem::Type(into_id) = into_import else {
                continue;
            };
            let Some(WorldItem::Type(from_id)) = from.imports.get(name) else {
                continue;
            };
            self.types.insert(*from_id, *into_id);
        }
    }

    pub fn world_item(&mut self, key: &WorldKey, item: &mut WorldItem, clone_maps: &mut CloneMaps) {
        match key {
            WorldKey::Name(_) => {}
            WorldKey::Interface(_) => return,
        }

        match item {
            WorldItem::Type(t) => {
                self.type_id(t);
            }
            WorldItem::Function(f) => {
                self.function(f);
            }
            WorldItem::Interface { id, .. } => {
                let old = *id;
                self.interface(id, &mut clone_maps.types);
                clone_maps.interfaces.insert(old, *id);
            }
        }
    }

    fn type_id(&mut self, ty: &mut TypeId) {
        if !self.types.contains_key(ty) {
            let mut new = self.resolve.types[*ty].clone();
            self.type_def(&mut new);
            let id = self.resolve.types.alloc(new);
            self.types.insert(*ty, id);
        }
        *ty = self.types[ty];
    }

    fn type_def(&mut self, def: &mut TypeDef) {
        if def.owner != TypeOwner::None {
            assert_eq!(def.owner, self.prev_owner);
            def.owner = self.new_owner;
        }
        match &mut def.kind {
            TypeDefKind::Type(Type::Id(id)) => {
                if self.resolve.types[*id].owner == self.prev_owner {
                    self.type_id(id);
                } else {
                    // ..
                }
            }
            TypeDefKind::Type(_)
            | TypeDefKind::Resource
            | TypeDefKind::Flags(_)
            | TypeDefKind::Enum(_) => {}
            TypeDefKind::Handle(Handle::Own(ty) | Handle::Borrow(ty)) => {
                self.type_id(ty);
            }
            TypeDefKind::Option(ty)
            | TypeDefKind::List(ty)
            | TypeDefKind::FixedSizeList(ty, ..) => {
                self.ty(ty);
            }
            TypeDefKind::Map(k, v) => {
                self.ty(k);
                self.ty(v);
            }
            TypeDefKind::Tuple(list) => {
                for ty in list.types.iter_mut() {
                    self.ty(ty);
                }
            }
            TypeDefKind::Record(r) => {
                for field in r.fields.iter_mut() {
                    self.ty(&mut field.ty);
                }
            }
            TypeDefKind::Variant(r) => {
                for case in r.cases.iter_mut() {
                    if let Some(ty) = &mut case.ty {
                        self.ty(ty);
                    }
                }
            }
            TypeDefKind::Result(r) => {
                if let Some(ok) = &mut r.ok {
                    self.ty(ok);
                }
                if let Some(err) = &mut r.err {
                    self.ty(err);
                }
            }
            TypeDefKind::Future(ty) | TypeDefKind::Stream(ty) => {
                if let Some(ty) = ty {
                    self.ty(ty);
                }
            }
            TypeDefKind::Unknown => {}
        }
    }

    fn ty(&mut self, ty: &mut Type) {
        match ty {
            Type::Id(id) => self.type_id(id),
            _ => {}
        }
    }

    fn function(&mut self, func: &mut Function) {
        if let Some(id) = func.kind.resource_mut() {
            self.type_id(id);
        }
        for (_, ty) in func.params.iter_mut() {
            self.ty(ty);
        }
        if let Some(ty) = &mut func.result {
            self.ty(ty);
        }
    }

    fn interface(&mut self, id: &mut InterfaceId, cloned_types: &mut HashMap<TypeId, TypeId>) {
        let mut new = self.resolve.interfaces[*id].clone();
        let next_id = self.resolve.interfaces.next_id();
        let mut clone = Cloner::new(
            self.resolve,
            TypeOwner::Interface(*id),
            TypeOwner::Interface(next_id),
        );
        for id in new.types.values_mut() {
            clone.type_id(id);
        }
        for func in new.functions.values_mut() {
            clone.function(func);
        }
        cloned_types.extend(clone.types);
        new.package = Some(self.new_package.unwrap_or_else(|| match self.new_owner {
            TypeOwner::Interface(id) => self.resolve.interfaces[id].package.unwrap(),
            TypeOwner::World(id) => self.resolve.worlds[id].package.unwrap(),
            TypeOwner::None => unreachable!(),
        }));
        *id = self.resolve.interfaces.alloc(new);
        assert_eq!(*id, next_id);
    }
}

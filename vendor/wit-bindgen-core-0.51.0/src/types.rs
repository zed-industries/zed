use std::collections::HashMap;

use wit_parser::*;

#[derive(Default)]
pub struct Types {
    type_info: HashMap<TypeId, TypeInfo>,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct TypeInfo {
    /// Whether this type is ever used (transitively) within the
    /// parameter of an imported function.
    ///
    /// This means that it's used in a context where ownership isn't
    /// relinquished.
    pub borrowed: bool,

    /// Whether this type is ever used (transitively) within the
    /// parameter or result of an export, or the result of an import.
    ///
    /// This means that it's used in a context where ownership is required and
    /// memory management is necessary.
    pub owned: bool,

    /// Whether this type is ever used (transitively) within the
    /// error case in the result of a function.
    pub error: bool,

    /// Whether this type (transitively) has a list (or string).
    pub has_list: bool,

    /// Whether this type (transitively) has a tuple.
    pub has_tuple: bool,

    /// Whether this type (transitively) has a resource (or handle).
    pub has_resource: bool,

    /// Whether this type (transitively) has a borrow handle.
    pub has_borrow_handle: bool,

    /// Whether this type (transitively) has an own handle.
    pub has_own_handle: bool,
}

impl std::ops::BitOrAssign for TypeInfo {
    fn bitor_assign(&mut self, rhs: Self) {
        self.borrowed |= rhs.borrowed;
        self.owned |= rhs.owned;
        self.error |= rhs.error;
        self.has_list |= rhs.has_list;
        self.has_tuple |= rhs.has_tuple;
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
                    WorldItem::Interface { id, stability: _ } => {
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
        if let Some(ty) = &func.result {
            self.type_info(resolve, ty);
            live.add_type(resolve, ty);
        }
        for id in live.iter() {
            if resolve.types[id].name.is_some() {
                self.type_info.get_mut(&id).unwrap().owned = true;
            }
        }

        if let Some(Type::Id(id)) = func.result {
            let err = match &resolve.types[id].kind {
                TypeDefKind::Result(Result_ { err, .. }) => err,
                _ => return,
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
                info.has_tuple = true;
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
            TypeDefKind::Future(_) | TypeDefKind::Stream(_) => {
                // These are all u32 handles regardless of payload type, so no
                // need to recurse.
                info.has_resource = true;

                // Flag these as being onwed handles as lowering these values
                // should use the same ownership semantics as `own<T>`
                info.has_own_handle = true;
            }
            TypeDefKind::FixedSizeList(..) => todo!(),
            TypeDefKind::Map(..) => todo!(),
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
            Type::ErrorContext => info.has_resource = true,
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

use crate::{
    Function, FunctionKind, InterfaceId, Resolve, Type, TypeDef, TypeDefKind, TypeId, WorldId,
    WorldItem,
};
use indexmap::IndexSet;

#[derive(Default)]
pub struct LiveTypes {
    set: IndexSet<TypeId>,
}

impl LiveTypes {
    pub fn iter(&self) -> impl Iterator<Item = TypeId> + '_ {
        self.set.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn contains(&self, id: TypeId) -> bool {
        self.set.contains(&id)
    }

    pub fn add_interface(&mut self, resolve: &Resolve, iface: InterfaceId) {
        self.visit_interface(resolve, iface);
    }

    pub fn add_world(&mut self, resolve: &Resolve, world: WorldId) {
        self.visit_world(resolve, world);
    }

    pub fn add_world_item(&mut self, resolve: &Resolve, item: &WorldItem) {
        self.visit_world_item(resolve, item);
    }

    pub fn add_func(&mut self, resolve: &Resolve, func: &Function) {
        self.visit_func(resolve, func);
    }

    pub fn add_type_id(&mut self, resolve: &Resolve, ty: TypeId) {
        self.visit_type_id(resolve, ty);
    }

    pub fn add_type(&mut self, resolve: &Resolve, ty: &Type) {
        self.visit_type(resolve, ty);
    }
}

impl TypeIdVisitor for LiveTypes {
    fn before_visit_type_id(&mut self, id: TypeId) -> bool {
        !self.set.contains(&id)
    }

    fn after_visit_type_id(&mut self, id: TypeId) {
        assert!(self.set.insert(id));
    }
}

/// Helper trait to walk the structure of a type and visit all `TypeId`s that
/// it refers to, possibly transitively.
pub trait TypeIdVisitor {
    /// Callback invoked just before a type is visited.
    ///
    /// If this function returns `false` the type is not visited, otherwise it's
    /// recursed into.
    fn before_visit_type_id(&mut self, id: TypeId) -> bool {
        let _ = id;
        true
    }

    /// Callback invoked once a type is finished being visited.
    fn after_visit_type_id(&mut self, id: TypeId) {
        let _ = id;
    }

    fn visit_interface(&mut self, resolve: &Resolve, iface: InterfaceId) {
        let iface = &resolve.interfaces[iface];
        for (_, id) in iface.types.iter() {
            self.visit_type_id(resolve, *id);
        }
        for (_, func) in iface.functions.iter() {
            self.visit_func(resolve, func);
        }
    }

    fn visit_world(&mut self, resolve: &Resolve, world: WorldId) {
        let world = &resolve.worlds[world];
        for (_, item) in world.imports.iter().chain(world.exports.iter()) {
            self.visit_world_item(resolve, item);
        }
    }

    fn visit_world_item(&mut self, resolve: &Resolve, item: &WorldItem) {
        match item {
            WorldItem::Interface { id, .. } => self.visit_interface(resolve, *id),
            WorldItem::Function(f) => self.visit_func(resolve, f),
            WorldItem::Type(t) => self.visit_type_id(resolve, *t),
        }
    }

    fn visit_func(&mut self, resolve: &Resolve, func: &Function) {
        match func.kind {
            // This resource is live as it's attached to a static method but
            // it's not guaranteed to be present in either params or results, so
            // be sure to attach it here.
            FunctionKind::Static(id) | FunctionKind::AsyncStatic(id) => {
                self.visit_type_id(resolve, id)
            }

            // The resource these are attached to is in the params/results, so
            // no need to re-add it here.
            FunctionKind::Method(_)
            | FunctionKind::AsyncMethod(_)
            | FunctionKind::Constructor(_) => {}

            FunctionKind::Freestanding | FunctionKind::AsyncFreestanding => {}
        }

        for (_, ty) in func.params.iter() {
            self.visit_type(resolve, ty);
        }
        if let Some(ty) = &func.result {
            self.visit_type(resolve, ty);
        }
    }

    fn visit_type_id(&mut self, resolve: &Resolve, ty: TypeId) {
        if self.before_visit_type_id(ty) {
            self.visit_type_def(resolve, &resolve.types[ty]);
            self.after_visit_type_id(ty);
        }
    }

    fn visit_type_def(&mut self, resolve: &Resolve, ty: &TypeDef) {
        match &ty.kind {
            TypeDefKind::Type(t)
            | TypeDefKind::List(t)
            | TypeDefKind::FixedSizeList(t, ..)
            | TypeDefKind::Option(t)
            | TypeDefKind::Future(Some(t))
            | TypeDefKind::Stream(Some(t)) => self.visit_type(resolve, t),
            TypeDefKind::Map(k, v) => {
                self.visit_type(resolve, k);
                self.visit_type(resolve, v);
            }
            TypeDefKind::Handle(handle) => match handle {
                crate::Handle::Own(ty) => self.visit_type_id(resolve, *ty),
                crate::Handle::Borrow(ty) => self.visit_type_id(resolve, *ty),
            },
            TypeDefKind::Resource => {}
            TypeDefKind::Record(r) => {
                for field in r.fields.iter() {
                    self.visit_type(resolve, &field.ty);
                }
            }
            TypeDefKind::Tuple(r) => {
                for ty in r.types.iter() {
                    self.visit_type(resolve, ty);
                }
            }
            TypeDefKind::Variant(v) => {
                for case in v.cases.iter() {
                    if let Some(ty) = &case.ty {
                        self.visit_type(resolve, ty);
                    }
                }
            }
            TypeDefKind::Result(r) => {
                if let Some(ty) = &r.ok {
                    self.visit_type(resolve, ty);
                }
                if let Some(ty) = &r.err {
                    self.visit_type(resolve, ty);
                }
            }
            TypeDefKind::Flags(_)
            | TypeDefKind::Enum(_)
            | TypeDefKind::Future(None)
            | TypeDefKind::Stream(None) => {}
            TypeDefKind::Unknown => unreachable!(),
        }
    }

    fn visit_type(&mut self, resolve: &Resolve, ty: &Type) {
        match ty {
            Type::Id(id) => self.visit_type_id(resolve, *id),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LiveTypes, Resolve};

    fn live(wit: &str, ty: &str) -> Vec<String> {
        let mut resolve = Resolve::default();
        resolve.push_str("test.wit", wit).unwrap();
        let (_, interface) = resolve.interfaces.iter().next_back().unwrap();
        let ty = interface.types[ty];
        let mut live = LiveTypes::default();
        live.add_type_id(&resolve, ty);

        live.iter()
            .filter_map(|ty| resolve.types[ty].name.clone())
            .collect()
    }

    #[test]
    fn no_deps() {
        let types = live(
            "
                package foo:bar;

                interface foo {
                    type t = u32;
                }
            ",
            "t",
        );
        assert_eq!(types, ["t"]);
    }

    #[test]
    fn one_dep() {
        let types = live(
            "
                package foo:bar;

                interface foo {
                    type t = u32;
                    type u = t;
                }
            ",
            "u",
        );
        assert_eq!(types, ["t", "u"]);
    }

    #[test]
    fn chain() {
        let types = live(
            "
                package foo:bar;

                interface foo {
                    resource t1;
                    record t2 {
                        x: t1,
                    }
                    variant t3 {
                        x(t2),
                    }
                    flags t4 { a }
                    enum t5 { a }
                    type t6 = tuple<t5, t4, t3>;
                }
            ",
            "t6",
        );
        assert_eq!(types, ["t5", "t4", "t1", "t2", "t3", "t6"]);
    }
}

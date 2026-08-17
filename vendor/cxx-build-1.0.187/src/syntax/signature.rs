use crate::syntax::set::{OrderedSet, UnorderedSet};
use crate::syntax::{FnKind, Receiver, Signature, Type};
use proc_macro2::Ident;
use syn::Lifetime;

impl Signature {
    pub fn receiver(&self) -> Option<&Receiver> {
        match &self.kind {
            FnKind::Method(receiver) => Some(receiver),
            FnKind::Assoc(_) | FnKind::Free => None,
        }
    }

    pub fn receiver_mut(&mut self) -> Option<&mut Receiver> {
        match &mut self.kind {
            FnKind::Method(receiver) => Some(receiver),
            FnKind::Assoc(_) | FnKind::Free => None,
        }
    }

    pub fn self_type(&self) -> Option<&Ident> {
        match &self.kind {
            FnKind::Method(receiver) => Some(&receiver.ty.rust),
            FnKind::Assoc(self_type) => Some(self_type),
            FnKind::Free => None,
        }
    }

    #[cfg_attr(not(proc_macro), allow(dead_code))]
    pub fn undeclared_lifetimes<'a>(&'a self) -> OrderedSet<&'a Lifetime> {
        let mut declared_lifetimes = UnorderedSet::new();
        for param in self.generics.lifetimes() {
            declared_lifetimes.insert(&param.lifetime);
        }

        let mut undeclared_lifetimes = OrderedSet::new();
        let mut collect_lifetime = |lifetime: &'a Lifetime| {
            if lifetime.ident != "_"
                && lifetime.ident != "static"
                && !declared_lifetimes.contains(lifetime)
            {
                undeclared_lifetimes.insert(lifetime);
            }
        };

        match &self.kind {
            FnKind::Method(receiver) => {
                if let Some(lifetime) = &receiver.lifetime {
                    collect_lifetime(lifetime);
                }
                for lifetime in &receiver.ty.generics.lifetimes {
                    collect_lifetime(lifetime);
                }
            }
            FnKind::Assoc(self_type) => {
                // If support is added for explicit lifetimes in the Self type
                // of static member functions, that needs to be handled here.
                let _: &Ident = self_type;
            }
            FnKind::Free => {}
        }

        fn collect_type<'a>(collect_lifetime: &mut impl FnMut(&'a Lifetime), ty: &'a Type) {
            match ty {
                Type::Ident(named_type) => {
                    for lifetime in &named_type.generics.lifetimes {
                        collect_lifetime(lifetime);
                    }
                }
                Type::RustBox(ty1)
                | Type::RustVec(ty1)
                | Type::UniquePtr(ty1)
                | Type::SharedPtr(ty1)
                | Type::WeakPtr(ty1)
                | Type::CxxVector(ty1) => collect_type(collect_lifetime, &ty1.inner),
                Type::Ref(ty) | Type::Str(ty) => {
                    if let Some(lifetime) = &ty.lifetime {
                        collect_lifetime(lifetime);
                    }
                    collect_type(collect_lifetime, &ty.inner);
                }
                Type::Ptr(ty) => collect_type(collect_lifetime, &ty.inner),
                Type::Fn(signature) => {
                    for lifetime in signature.undeclared_lifetimes() {
                        collect_lifetime(lifetime);
                    }
                }
                Type::Void(_) => {}
                Type::SliceRef(ty) => {
                    if let Some(lifetime) = &ty.lifetime {
                        collect_lifetime(lifetime);
                    }
                    collect_type(collect_lifetime, &ty.inner);
                }
                Type::Array(ty) => collect_type(collect_lifetime, &ty.inner),
            }
        }

        for arg in &self.args {
            collect_type(&mut collect_lifetime, &arg.ty);
        }

        if let Some(ret) = &self.ret {
            collect_type(&mut collect_lifetime, ret);
        }

        undeclared_lifetimes
    }
}

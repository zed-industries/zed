use crate::syntax::{
    Array, ExternFn, Include, Lifetimes, Ptr, Receiver, Ref, Signature, SliceRef, Ty1, Type, Var,
};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{Deref, DerefMut};

impl PartialEq for Include {
    fn eq(&self, other: &Self) -> bool {
        let Include {
            cfg: _,
            path,
            kind,
            begin_span: _,
            end_span: _,
        } = self;
        let Include {
            cfg: _,
            path: path2,
            kind: kind2,
            begin_span: _,
            end_span: _,
        } = other;
        path == path2 && kind == kind2
    }
}

impl Deref for ExternFn {
    type Target = Signature;

    fn deref(&self) -> &Self::Target {
        &self.sig
    }
}

impl DerefMut for ExternFn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sig
    }
}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        match self {
            Type::Ident(t) => t.hash(state),
            Type::RustBox(t) => t.hash(state),
            Type::UniquePtr(t) => t.hash(state),
            Type::SharedPtr(t) => t.hash(state),
            Type::WeakPtr(t) => t.hash(state),
            Type::Ref(t) => t.hash(state),
            Type::Ptr(t) => t.hash(state),
            Type::Str(t) => t.hash(state),
            Type::RustVec(t) => t.hash(state),
            Type::CxxVector(t) => t.hash(state),
            Type::Fn(t) => t.hash(state),
            Type::SliceRef(t) => t.hash(state),
            Type::Array(t) => t.hash(state),
            Type::Void(_) => {}
        }
    }
}

impl Eq for Type {}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Ident(lhs), Type::Ident(rhs)) => lhs == rhs,
            (Type::RustBox(lhs), Type::RustBox(rhs)) => lhs == rhs,
            (Type::UniquePtr(lhs), Type::UniquePtr(rhs)) => lhs == rhs,
            (Type::SharedPtr(lhs), Type::SharedPtr(rhs)) => lhs == rhs,
            (Type::WeakPtr(lhs), Type::WeakPtr(rhs)) => lhs == rhs,
            (Type::Ref(lhs), Type::Ref(rhs)) => lhs == rhs,
            (Type::Str(lhs), Type::Str(rhs)) => lhs == rhs,
            (Type::RustVec(lhs), Type::RustVec(rhs)) => lhs == rhs,
            (Type::CxxVector(lhs), Type::CxxVector(rhs)) => lhs == rhs,
            (Type::Fn(lhs), Type::Fn(rhs)) => lhs == rhs,
            (Type::SliceRef(lhs), Type::SliceRef(rhs)) => lhs == rhs,
            (Type::Void(_), Type::Void(_)) => true,
            (_, _) => false,
        }
    }
}

impl Eq for Lifetimes {}

impl PartialEq for Lifetimes {
    fn eq(&self, other: &Self) -> bool {
        let Lifetimes {
            lt_token: _,
            lifetimes,
            gt_token: _,
        } = self;
        let Lifetimes {
            lt_token: _,
            lifetimes: lifetimes2,
            gt_token: _,
        } = other;
        lifetimes.iter().eq(lifetimes2)
    }
}

impl Hash for Lifetimes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Lifetimes {
            lt_token: _,
            lifetimes,
            gt_token: _,
        } = self;
        lifetimes.len().hash(state);
        for lifetime in lifetimes {
            lifetime.hash(state);
        }
    }
}

impl Eq for Ty1 {}

impl PartialEq for Ty1 {
    fn eq(&self, other: &Self) -> bool {
        let Ty1 {
            name,
            langle: _,
            inner,
            rangle: _,
        } = self;
        let Ty1 {
            name: name2,
            langle: _,
            inner: inner2,
            rangle: _,
        } = other;
        name == name2 && inner == inner2
    }
}

impl Hash for Ty1 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Ty1 {
            name,
            langle: _,
            inner,
            rangle: _,
        } = self;
        name.hash(state);
        inner.hash(state);
    }
}

impl Eq for Ref {}

impl PartialEq for Ref {
    fn eq(&self, other: &Self) -> bool {
        let Ref {
            pinned,
            ampersand: _,
            lifetime,
            mutable,
            inner,
            pin_tokens: _,
            mutability: _,
        } = self;
        let Ref {
            pinned: pinned2,
            ampersand: _,
            lifetime: lifetime2,
            mutable: mutable2,
            inner: inner2,
            pin_tokens: _,
            mutability: _,
        } = other;
        pinned == pinned2 && lifetime == lifetime2 && mutable == mutable2 && inner == inner2
    }
}

impl Hash for Ref {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Ref {
            pinned,
            ampersand: _,
            lifetime,
            mutable,
            inner,
            pin_tokens: _,
            mutability: _,
        } = self;
        pinned.hash(state);
        lifetime.hash(state);
        mutable.hash(state);
        inner.hash(state);
    }
}

impl Eq for Ptr {}

impl PartialEq for Ptr {
    fn eq(&self, other: &Ptr) -> bool {
        let Ptr {
            star: _,
            mutable,
            inner,
            mutability: _,
            constness: _,
        } = self;
        let Ptr {
            star: _,
            mutable: mutable2,
            inner: inner2,
            mutability: _,
            constness: _,
        } = other;
        mutable == mutable2 && inner == inner2
    }
}

impl Hash for Ptr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Ptr {
            star: _,
            mutable,
            inner,
            mutability: _,
            constness: _,
        } = self;
        mutable.hash(state);
        inner.hash(state);
    }
}

impl Eq for SliceRef {}

impl PartialEq for SliceRef {
    fn eq(&self, other: &Self) -> bool {
        let SliceRef {
            ampersand: _,
            lifetime,
            mutable,
            bracket: _,
            inner,
            mutability: _,
        } = self;
        let SliceRef {
            ampersand: _,
            lifetime: lifetime2,
            mutable: mutable2,
            bracket: _,
            inner: inner2,
            mutability: _,
        } = other;
        lifetime == lifetime2 && mutable == mutable2 && inner == inner2
    }
}

impl Hash for SliceRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let SliceRef {
            ampersand: _,
            lifetime,
            mutable,
            bracket: _,
            inner,
            mutability: _,
        } = self;
        lifetime.hash(state);
        mutable.hash(state);
        inner.hash(state);
    }
}

impl Eq for Array {}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        let Array {
            bracket: _,
            inner,
            semi_token: _,
            len,
            len_token: _,
        } = self;
        let Array {
            bracket: _,
            inner: inner2,
            semi_token: _,
            len: len2,
            len_token: _,
        } = other;
        inner == inner2 && len == len2
    }
}

impl Hash for Array {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Array {
            bracket: _,
            inner,
            semi_token: _,
            len,
            len_token: _,
        } = self;
        inner.hash(state);
        len.hash(state);
    }
}

impl Eq for Signature {}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        let Signature {
            asyncness,
            unsafety,
            fn_token: _,
            generics: _,
            kind,
            args,
            ret,
            throws,
            paren_token: _,
            throws_tokens: _,
        } = self;
        let Signature {
            asyncness: asyncness2,
            unsafety: unsafety2,
            fn_token: _,
            generics: _,
            kind: kind2,
            args: args2,
            ret: ret2,
            throws: throws2,
            paren_token: _,
            throws_tokens: _,
        } = other;
        asyncness.is_some() == asyncness2.is_some()
            && unsafety.is_some() == unsafety2.is_some()
            && kind == kind2
            && ret == ret2
            && throws == throws2
            && args.len() == args2.len()
            && args.iter().zip(args2).all(|(arg, arg2)| {
                let Var {
                    cfg: _,
                    doc: _,
                    attrs: _,
                    visibility: _,
                    name: _,
                    colon_token: _,
                    ty,
                } = arg;
                let Var {
                    cfg: _,
                    doc: _,
                    attrs: _,
                    visibility: _,
                    name: _,
                    colon_token: _,
                    ty: ty2,
                } = arg2;
                ty == ty2
            })
    }
}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Signature {
            asyncness,
            unsafety,
            fn_token: _,
            generics: _,
            kind,
            args,
            ret,
            throws,
            paren_token: _,
            throws_tokens: _,
        } = self;
        asyncness.is_some().hash(state);
        unsafety.is_some().hash(state);
        kind.hash(state);
        for arg in args {
            let Var {
                cfg: _,
                doc: _,
                attrs: _,
                visibility: _,
                name: _,
                colon_token: _,
                ty,
            } = arg;
            ty.hash(state);
        }
        ret.hash(state);
        throws.hash(state);
    }
}

impl Eq for Receiver {}

impl PartialEq for Receiver {
    fn eq(&self, other: &Self) -> bool {
        let Receiver {
            pinned,
            ampersand: _,
            lifetime,
            mutable,
            var: _,
            colon_token: _,
            ty,
            shorthand: _,
            pin_tokens: _,
            mutability: _,
        } = self;
        let Receiver {
            pinned: pinned2,
            ampersand: _,
            lifetime: lifetime2,
            mutable: mutable2,
            var: _,
            colon_token: _,
            ty: ty2,
            shorthand: _,
            pin_tokens: _,
            mutability: _,
        } = other;
        pinned == pinned2 && lifetime == lifetime2 && mutable == mutable2 && ty == ty2
    }
}

impl Hash for Receiver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Receiver {
            pinned,
            ampersand: _,
            lifetime,
            mutable,
            var: _,
            colon_token: _,
            ty,
            shorthand: _,
            pin_tokens: _,
            mutability: _,
        } = self;
        pinned.hash(state);
        lifetime.hash(state);
        mutable.hash(state);
        ty.hash(state);
    }
}

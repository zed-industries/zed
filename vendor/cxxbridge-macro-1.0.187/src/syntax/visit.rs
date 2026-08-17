use crate::syntax::Type;

pub(crate) trait Visit<'a> {
    fn visit_type(&mut self, ty: &'a Type) {
        visit_type(self, ty);
    }
}

pub(crate) fn visit_type<'a, V>(visitor: &mut V, ty: &'a Type)
where
    V: Visit<'a> + ?Sized,
{
    match ty {
        Type::Ident(_) | Type::Str(_) | Type::Void(_) => {}
        Type::RustBox(ty)
        | Type::UniquePtr(ty)
        | Type::SharedPtr(ty)
        | Type::WeakPtr(ty)
        | Type::CxxVector(ty)
        | Type::RustVec(ty) => visitor.visit_type(&ty.inner),
        Type::Ref(r) => visitor.visit_type(&r.inner),
        Type::Ptr(p) => visitor.visit_type(&p.inner),
        Type::Array(a) => visitor.visit_type(&a.inner),
        Type::SliceRef(s) => visitor.visit_type(&s.inner),
        Type::Fn(fun) => {
            if let Some(ret) = &fun.ret {
                visitor.visit_type(ret);
            }
            for arg in &fun.args {
                visitor.visit_type(&arg.ty);
            }
        }
    }
}

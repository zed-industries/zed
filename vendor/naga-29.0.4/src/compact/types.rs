use super::{HandleSet, ModuleMap};
use crate::Handle;

pub struct TypeTracer<'a> {
    pub overrides: &'a crate::Arena<crate::Override>,
    pub types_used: &'a mut HandleSet<crate::Type>,
    pub expressions_used: &'a mut HandleSet<crate::Expression>,
    pub overrides_used: &'a mut HandleSet<crate::Override>,
}

impl TypeTracer<'_> {
    pub fn trace_type(&mut self, ty: &crate::Type) {
        use crate::TypeInner as Ti;
        match ty.inner {
            // Types that do not contain handles.
            Ti::Scalar { .. }
            | Ti::Vector { .. }
            | Ti::Matrix { .. }
            | Ti::CooperativeMatrix { .. }
            | Ti::Atomic { .. }
            | Ti::ValuePointer { .. }
            | Ti::Image { .. }
            | Ti::Sampler { .. }
            | Ti::AccelerationStructure { .. }
            | Ti::RayQuery { .. } => {}

            // Types that do contain handles.
            Ti::Array {
                base,
                size,
                stride: _,
            }
            | Ti::BindingArray { base, size } => {
                self.types_used.insert(base);
                match size {
                    crate::ArraySize::Pending(handle) => {
                        self.overrides_used.insert(handle);
                        let r#override = &self.overrides[handle];
                        self.types_used.insert(r#override.ty);
                        if let Some(expr) = r#override.init {
                            self.expressions_used.insert(expr);
                        }
                    }
                    crate::ArraySize::Constant(_) | crate::ArraySize::Dynamic => {}
                }
            }
            Ti::Pointer { base, space: _ } => {
                self.types_used.insert(base);
            }
            Ti::Struct {
                ref members,
                span: _,
            } => {
                self.types_used.insert_iter(members.iter().map(|m| m.ty));
            }
        }
    }
}

impl ModuleMap {
    pub fn adjust_type(&self, ty: &mut crate::Type) {
        let adjust = |ty: &mut Handle<crate::Type>| self.types.adjust(ty);

        use crate::TypeInner as Ti;
        match ty.inner {
            // Types that do not contain handles.
            Ti::Scalar(_)
            | Ti::Vector { .. }
            | Ti::Matrix { .. }
            | Ti::CooperativeMatrix { .. }
            | Ti::Atomic(_)
            | Ti::ValuePointer { .. }
            | Ti::Image { .. }
            | Ti::Sampler { .. }
            | Ti::AccelerationStructure { .. }
            | Ti::RayQuery { .. } => {}

            // Types that do contain handles.
            Ti::Pointer {
                ref mut base,
                space: _,
            } => adjust(base),
            Ti::Array {
                ref mut base,
                ref mut size,
                stride: _,
            }
            | Ti::BindingArray {
                ref mut base,
                ref mut size,
            } => {
                adjust(base);
                match *size {
                    crate::ArraySize::Pending(ref mut r#override) => {
                        self.overrides.adjust(r#override);
                    }
                    crate::ArraySize::Constant(_) | crate::ArraySize::Dynamic => {}
                }
            }
            Ti::Struct {
                ref mut members,
                span: _,
            } => {
                for member in members {
                    self.types.adjust(&mut member.ty);
                }
            }
        };
    }
}

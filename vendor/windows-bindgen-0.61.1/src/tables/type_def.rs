use super::*;

impl std::fmt::Debug for TypeDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeDef({})", self.type_name())
    }
}

impl TypeDef {
    pub fn flags(&self) -> TypeAttributes {
        TypeAttributes(self.usize(0) as u32)
    }

    pub fn type_name(&self) -> TypeName {
        TypeName(self.namespace(), self.name())
    }

    pub fn name(&self) -> &'static str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &'static str {
        self.str(2)
    }

    pub fn extends(&self) -> Option<TypeName> {
        let extends = self.usize(3);

        if extends == 0 {
            return None;
        }

        Some(TypeDefOrRef::decode(self.file(), extends).type_name())
    }

    pub fn methods(&self) -> RowIterator<MethodDef> {
        self.list(5)
    }

    pub fn fields(&self) -> RowIterator<Field> {
        self.list(4)
    }

    pub fn generics(&self) -> Vec<Type> {
        self.file()
            .equal_range(2, TypeOrMethodDef::TypeDef(*self).encode())
            .map(|generic: GenericParam| Type::Generic(generic))
            .collect()
    }

    pub fn interface_impls(&self) -> RowIterator<InterfaceImpl> {
        self.file().equal_range(0, self.index() + 1)
    }

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.file().equal_range(2, self.index() + 1).next()
    }

    pub fn nested(&self) -> Option<NestedClass> {
        self.file().equal_range(0, self.index() + 1).next()
    }

    pub fn underlying_type(&self) -> Type {
        let field = self.fields().next().unwrap();
        if let Some(constant) = field.constant() {
            constant.ty()
        } else {
            field.ty(None)
        }
    }

    pub fn invalid_values(&self) -> Vec<i64> {
        let mut values = Vec::new();
        for attribute in self.attributes() {
            if attribute.name() == "InvalidHandleValueAttribute" {
                if let Some((_, Value::I64(value))) = attribute.args().first() {
                    values.push(*value);
                }
            }
        }
        values
    }

    pub fn free_function(&self) -> Option<CppFn> {
        if let Some(attribute) = self.find_attribute("RAIIFreeAttribute") {
            if let Some((_, Value::Str(name))) = attribute.args().first() {
                if let Some(Type::CppFn(ty)) =
                    self.reader().with_full_name(self.namespace(), name).next()
                {
                    return Some(ty);
                }
            }
        }

        None
    }

    pub fn is_agile(&self) -> bool {
        for attribute in self.attributes() {
            match attribute.name() {
                "AgileAttribute" => return true,
                "MarshalingBehaviorAttribute" => {
                    if let Some((_, Value::I32(2))) = attribute.args().first() {
                        return true;
                    }
                }
                _ => {}
            }
        }

        self.is_async()
    }

    pub fn is_async(&self) -> bool {
        matches!(
            TypeName(self.namespace(), self.name()),
            TypeName::IAsyncAction
                | TypeName::IAsyncActionWithProgress
                | TypeName::IAsyncOperation
                | TypeName::IAsyncOperationWithProgress
        )
    }
}

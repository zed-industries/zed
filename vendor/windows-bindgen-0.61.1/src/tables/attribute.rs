use super::*;

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Attribute").field(&self.name()).finish()
    }
}

impl Attribute {
    pub fn ty(&self) -> AttributeType {
        self.decode(1)
    }

    pub fn name(&self) -> &'static str {
        self.ty().parent().name()
    }

    pub fn args(&self) -> Vec<(&'static str, Value)> {
        let mut sig = self.ty().signature();
        let mut values = self.blob(2);
        let prolog = values.read_u16();
        debug_assert_eq!(prolog, 1);
        let flags = sig.read_u8();
        debug_assert_eq!(flags, MethodCallAttributes::HASTHIS.0);
        let fixed_arg_count = sig.read_usize();
        let ret_type = sig.read_usize();
        debug_assert_eq!(ret_type, 1);
        let mut args = Vec::with_capacity(fixed_arg_count);
        let reader = self.reader();

        for _ in 0..fixed_arg_count {
            let arg = match Type::from_blob(&mut sig, None, &[]) {
                Type::Bool => Value::Bool(values.read_bool()),
                Type::I8 => Value::I8(values.read_i8()),
                Type::U8 => Value::U8(values.read_u8()),
                Type::I16 => Value::I16(values.read_i16()),
                Type::U16 => Value::U16(values.read_u16()),
                Type::I32 => Value::I32(values.read_i32()),
                Type::U32 => Value::U32(values.read_u32()),
                Type::I64 => Value::I64(values.read_i64()),
                Type::U64 => Value::U64(values.read_u64()),
                Type::String => Value::Str(values.read_str()),
                Type::Type => Value::TypeName(TypeName::parse(values.read_str())),
                Type::CppEnum(ty) => {
                    let underlying_type = ty.def.underlying_type();
                    values.read_integer(underlying_type)
                }
                Type::Enum(ty) => {
                    let underlying_type = ty.def.underlying_type();
                    values.read_integer(underlying_type)
                }
                rest => panic!("{rest:?}"),
            };

            args.push(("", arg));
        }

        let named_arg_count = values.read_u16();
        args.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = values.read_u8();
            let arg_type = values.read_u8();
            let mut name = values.read_str();
            let arg = match arg_type {
                ELEMENT_TYPE_BOOLEAN => Value::Bool(values.read_bool()),
                ELEMENT_TYPE_I2 => Value::I16(values.read_i16()),
                ELEMENT_TYPE_I4 => Value::I32(values.read_i32()),
                ELEMENT_TYPE_U4 => Value::U32(values.read_u32()),
                ELEMENT_TYPE_STRING => Value::Str(values.read_str()),
                0x50 => Value::TypeName(TypeName::parse(values.read_str())),
                0x55 => {
                    let tn = TypeName::parse(name);
                    let def = reader.unwrap_full_name(tn.namespace(), tn.name());
                    name = values.read_str();
                    let underlying_type = def.underlying_type();
                    values.read_integer(underlying_type)
                }
                rest => panic!("{rest:?}"),
            };
            args.push((name, arg));
        }

        debug_assert_eq!(sig.len(), 0);
        debug_assert_eq!(values.len(), 0);
        args
    }
}

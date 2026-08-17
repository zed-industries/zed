use super::*;

impl std::fmt::Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Constant").field(&self.value()).finish()
    }
}

impl Constant {
    pub fn ty(&self) -> Type {
        Type::from_element_type(self.usize(0)).unwrap()
    }

    pub fn value(&self) -> Value {
        let mut blob = self.blob(2);

        match self.ty() {
            Type::I8 => Value::I8(blob.read_i8()),
            Type::U8 => Value::U8(blob.read_u8()),
            Type::I16 => Value::I16(blob.read_i16()),
            Type::U16 => Value::U16(blob.read_u16()),
            Type::I32 => Value::I32(blob.read_i32()),
            Type::U32 => Value::U32(blob.read_u32()),
            Type::I64 => Value::I64(blob.read_i64()),
            Type::U64 => Value::U64(blob.read_u64()),
            Type::F32 => Value::F32(blob.read_f32()),
            Type::F64 => Value::F64(blob.read_f64()),
            Type::String => Value::String(blob.read_utf16()),
            rest => panic!("{rest:?}"),
        }
    }
}

use crate::reflect::repeated::transmute::transmute_mut_if_eq;
use crate::reflect::ProtobufValue;

pub(crate) enum VecMutVariant<'a> {
    U32(&'a mut Vec<u32>),
    U64(&'a mut Vec<u64>),
    I32(&'a mut Vec<i32>),
    I64(&'a mut Vec<i64>),
    F32(&'a mut Vec<f32>),
    F64(&'a mut Vec<f64>),
    Bool(&'a mut Vec<bool>),
}

impl<'a> VecMutVariant<'a> {
    pub(crate) fn downcast<V: ProtobufValue>(vec: &'a mut Vec<V>) -> Option<VecMutVariant<'a>> {
        let vec = match transmute_mut_if_eq(vec) {
            Ok(vec) => return Some(VecMutVariant::U32(vec)),
            Err(vec) => vec,
        };
        let vec = match transmute_mut_if_eq(vec) {
            Ok(vec) => return Some(VecMutVariant::U64(vec)),
            Err(vec) => vec,
        };
        let vec = match transmute_mut_if_eq(vec) {
            Ok(vec) => return Some(VecMutVariant::I32(vec)),
            Err(vec) => vec,
        };
        let vec = match transmute_mut_if_eq(vec) {
            Ok(vec) => return Some(VecMutVariant::I64(vec)),
            Err(vec) => vec,
        };
        let vec = match transmute_mut_if_eq(vec) {
            Ok(vec) => return Some(VecMutVariant::F32(vec)),
            Err(vec) => vec,
        };
        let vec = match transmute_mut_if_eq(vec) {
            Ok(vec) => return Some(VecMutVariant::F64(vec)),
            Err(vec) => vec,
        };
        let vec = match transmute_mut_if_eq(vec) {
            Ok(vec) => return Some(VecMutVariant::Bool(vec)),
            Err(vec) => vec,
        };
        let _ = vec;
        None
    }
}

use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueRef;

pub(crate) struct ReflectRepeatedIter<'a> {
    imp: Box<dyn Iterator<Item = ReflectValueRef<'a>> + 'a>,
}

impl<'a> ReflectRepeatedIter<'a> {
    pub(crate) fn new(
        iter: impl Iterator<Item = ReflectValueRef<'a>> + 'a,
    ) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(iter),
        }
    }

    pub(crate) fn new_slice<V: ProtobufValue>(slice: &'a [V]) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter::new(slice.into_iter().map(V::RuntimeType::as_ref))
    }
}

impl<'a> Iterator for ReflectRepeatedIter<'a> {
    type Item = ReflectValueRef<'a>;

    fn next(&mut self) -> Option<ReflectValueRef<'a>> {
        self.imp.next()
    }
}

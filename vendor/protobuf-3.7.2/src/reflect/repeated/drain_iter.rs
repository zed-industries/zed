use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;

pub(crate) struct ReflectRepeatedDrainIter<'a> {
    imp: Box<dyn Iterator<Item = ReflectValueBox> + 'a>,
}

impl<'a> ReflectRepeatedDrainIter<'a> {
    pub(crate) fn new(
        imp: impl Iterator<Item = ReflectValueBox> + 'a,
    ) -> ReflectRepeatedDrainIter<'a> {
        ReflectRepeatedDrainIter { imp: Box::new(imp) }
    }

    pub(crate) fn new_vec<V: ProtobufValue>(v: &'a mut Vec<V>) -> ReflectRepeatedDrainIter<'a> {
        ReflectRepeatedDrainIter::new(v.drain(..).map(V::RuntimeType::into_value_box))
    }
}

impl<'a> Iterator for ReflectRepeatedDrainIter<'a> {
    type Item = ReflectValueBox;

    fn next(&mut self) -> Option<Self::Item> {
        self.imp.next()
    }
}

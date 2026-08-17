use crate::message_dyn::MessageDyn;
use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::FieldDescriptor;
use crate::reflect::ReflectFieldRef;
use crate::reflect::ReflectMapMut;
use crate::reflect::ReflectRepeatedMut;
use crate::reflect::ReflectValueBox;

pub(crate) struct DynamicFieldDescriptorRef<'a> {
    pub(crate) field: &'a FieldDescriptor,
}

impl<'a> DynamicFieldDescriptorRef<'a> {
    pub(crate) fn get_reflect<'b>(&self, message: &'b dyn MessageDyn) -> ReflectFieldRef<'b> {
        DynamicMessage::downcast_ref(message).get_reflect(&self.field)
    }

    pub(crate) fn mut_repeated<'b>(
        &self,
        message: &'b mut dyn MessageDyn,
    ) -> ReflectRepeatedMut<'b> {
        DynamicMessage::downcast_mut(message).mut_repeated(&self.field)
    }

    pub(crate) fn mut_map<'b>(&self, message: &'b mut dyn MessageDyn) -> ReflectMapMut<'b> {
        DynamicMessage::downcast_mut(message).mut_map(&self.field)
    }

    pub(crate) fn set_field(&self, message: &mut dyn MessageDyn, value: ReflectValueBox) {
        DynamicMessage::downcast_mut(message).set_field(&self.field, value)
    }

    pub(crate) fn clear_field(&self, message: &mut dyn MessageDyn) {
        DynamicMessage::downcast_mut(message).clear_field(&self.field)
    }
}

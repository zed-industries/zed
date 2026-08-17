use std::fmt;

use crate::descriptor::field_descriptor_proto;
use crate::descriptor::FieldDescriptorProto;
use crate::message_dyn::MessageDyn;
use crate::reflect::acc::v2::map::MapFieldAccessorHolder;
use crate::reflect::acc::v2::repeated::RepeatedFieldAccessorHolder;
use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::GeneratedFieldAccessor;
use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::field::dynamic::DynamicFieldDescriptorRef;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::field::index::FieldKind;
use crate::reflect::field::protobuf_field_type::ProtobufFieldType;
use crate::reflect::field::runtime_field_type::RuntimeFieldType;
use crate::reflect::map::ReflectMapMut;
use crate::reflect::map::ReflectMapRef;
use crate::reflect::message::message_ref::MessageRef;
use crate::reflect::message::MessageDescriptorImplRef;
use crate::reflect::oneof::OneofDescriptor;
use crate::reflect::protobuf_type_box::ProtobufType;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::repeated::ReflectRepeatedMut;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::FileDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectOptionalRef;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeType;

pub(crate) mod dynamic;
pub(crate) mod index;
pub(crate) mod protobuf_field_type;
pub(crate) mod runtime_field_type;

/// Reference to a value stored in a field, optional, repeated or map.
#[derive(PartialEq)]
pub enum ReflectFieldRef<'a> {
    /// Singular field, optional or required in proto3 and just plain field in proto3
    Optional(ReflectOptionalRef<'a>),
    /// Repeated field
    Repeated(ReflectRepeatedRef<'a>),
    /// Map field
    Map(ReflectMapRef<'a>),
}

impl<'a> ReflectFieldRef<'a> {
    pub(crate) fn default_for_field(field: &FieldDescriptor) -> ReflectFieldRef<'a> {
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(elem) => {
                ReflectFieldRef::Optional(ReflectOptionalRef::none(elem))
            }
            RuntimeFieldType::Repeated(elem) => {
                ReflectFieldRef::Repeated(ReflectRepeatedRef::new_empty(elem))
            }
            RuntimeFieldType::Map(k, v) => ReflectFieldRef::Map(ReflectMapRef::new_empty(k, v)),
        }
    }
}

impl<'a> ReflectEq for ReflectFieldRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        match (self, that) {
            (ReflectFieldRef::Optional(a), ReflectFieldRef::Optional(b)) => {
                match (a.value(), b.value()) {
                    (Some(av), Some(bv)) => av.reflect_eq(&bv, mode),
                    (None, None) => true,
                    _ => false,
                }
            }
            (ReflectFieldRef::Repeated(a), ReflectFieldRef::Repeated(b)) => a.reflect_eq(b, mode),
            (ReflectFieldRef::Map(a), ReflectFieldRef::Map(b)) => a.reflect_eq(b, mode),
            _ => false,
        }
    }
}

fn _assert_sync<'a>() {
    fn _assert_send_sync<T: Sync>() {}
    _assert_send_sync::<ReflectFieldRef<'a>>();
}

/// Field descriptor.
///
/// Can be used for runtime reflection.
#[derive(Eq, PartialEq, Clone)]
pub struct FieldDescriptor {
    pub(crate) file_descriptor: FileDescriptor,
    pub(crate) index: usize,
}

impl fmt::Display for FieldDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.index().kind {
            FieldKind::MessageField(m) => write!(
                f,
                "{}.{}",
                self.file_descriptor.message_by_index(*m),
                self.name()
            ),
            FieldKind::Extension(Some(m), _) => write!(
                f,
                "{}.{}",
                self.file_descriptor.message_by_index(*m),
                self.name()
            ),
            FieldKind::Extension(None, _) => {
                if self.file_descriptor.proto().package().is_empty() {
                    write!(f, "{}", self.name())
                } else {
                    write!(
                        f,
                        "{}.{}",
                        self.file_descriptor.proto().package(),
                        self.name()
                    )
                }
            }
        }
    }
}

impl FieldDescriptor {
    pub(crate) fn regular(&self) -> (MessageDescriptor, usize) {
        match self.index().kind {
            FieldKind::MessageField(_) => {
                let m = self.containing_message();
                (
                    m.clone(),
                    self.index - m.index().message_index.first_field_index,
                )
            }
            // TODO: implement and remove.
            _ => panic!("regular field"),
        }
    }

    pub(crate) fn file_descriptor(&self) -> &FileDescriptor {
        &self.file_descriptor
    }

    /// Get `.proto` description of field
    pub fn proto(&self) -> &FieldDescriptorProto {
        &self.index().proto
    }

    /// Field name as specified in `.proto` file.
    pub fn name(&self) -> &str {
        self.proto().name()
    }

    /// Field number as specified in `.proto` file.
    pub fn number(&self) -> i32 {
        self.proto().number()
    }

    /// Fully qualified name of the field: fully qualified name of the declaring type
    /// followed by the field name.
    ///
    /// Declaring type is a message (for regular field or extensions) or a package
    /// (for top-level extensions).
    pub fn full_name(&self) -> String {
        self.to_string()
    }

    /// Oneof descriptor containing this field. Do not skip synthetic oneofs.
    pub fn containing_oneof_including_synthetic(&self) -> Option<OneofDescriptor> {
        if let FieldKind::MessageField(..) = self.index().kind {
            let proto = self.proto();
            if proto.has_oneof_index() {
                Some(OneofDescriptor {
                    file_descriptor: self.file_descriptor().clone(),
                    index: self.containing_message().index().oneofs.start
                        + proto.oneof_index() as usize,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Oneof containing this field.
    ///
    /// Return `None` if this field is not part of oneof or if it is synthetic oneof.
    pub fn containing_oneof(&self) -> Option<OneofDescriptor> {
        self.containing_oneof_including_synthetic()
            .filter(|o| !o.is_synthetic())
    }

    /// Message which declares this field (for extension, **not** the message we extend).
    fn _declaring_message(&self) -> Option<MessageDescriptor> {
        match &self.index().kind {
            FieldKind::MessageField(m) => Some(self.file_descriptor.message_by_index(*m)),
            FieldKind::Extension(m, _) => Some(self.file_descriptor.message_by_index(*m.as_ref()?)),
        }
    }

    /// Message which contains this field.
    ///
    /// For extension fields, this is the message being extended.
    pub fn containing_message(&self) -> MessageDescriptor {
        match &self.index().kind {
            FieldKind::MessageField(m) => self.file_descriptor().message_by_index(*m),
            FieldKind::Extension(_, extendee) => extendee.resolve_message(self.file_descriptor()),
        }
    }

    fn index(&self) -> &FieldIndex {
        &self.file_descriptor.common().fields[self.index]
    }

    fn index_with_message_lifetime<'a>(&self, m: &'a dyn MessageDyn) -> &'a FieldIndex {
        let (descriptor, index) = self.regular();
        let file_fields = match self.singular() {
            SingularFieldAccessorRef::Generated(..) => {
                &descriptor
                    .file_descriptor
                    .common_for_generated_descriptor()
                    .fields
            }
            SingularFieldAccessorRef::Dynamic(..) => {
                &DynamicMessage::downcast_ref(m)
                    .descriptor()
                    .file_descriptor
                    .common()
                    .fields
            }
        };
        &descriptor.index().message_index.slice_fields(file_fields)[index]
    }

    /// JSON field name.
    ///
    /// Can be different from `.proto` field name.
    ///
    /// See [JSON mapping][json] for details.
    ///
    /// [json]: https://developers.google.com/protocol-buffers/docs/proto3#json
    pub fn json_name(&self) -> &str {
        &self.index().json_name
    }

    /// If this field is optional or required.
    pub fn is_singular(&self) -> bool {
        match self.proto().label() {
            field_descriptor_proto::Label::LABEL_REQUIRED => true,
            field_descriptor_proto::Label::LABEL_OPTIONAL => true,
            field_descriptor_proto::Label::LABEL_REPEATED => false,
        }
    }

    /// Is this field required.
    pub fn is_required(&self) -> bool {
        self.proto().label() == field_descriptor_proto::Label::LABEL_REQUIRED
    }

    /// If this field repeated or map?
    pub fn is_repeated_or_map(&self) -> bool {
        self.proto().label() == field_descriptor_proto::Label::LABEL_REPEATED
    }

    /// Is this field repeated, but not map field?
    pub fn is_repeated(&self) -> bool {
        match self.runtime_field_type() {
            RuntimeFieldType::Repeated(..) => true,
            _ => false,
        }
    }

    fn get_impl(&self) -> FieldDescriptorImplRef {
        let (descriptor, index) = self.regular();
        match descriptor.get_impl() {
            MessageDescriptorImplRef::Generated(g) => {
                FieldDescriptorImplRef::Generated(&g.non_map().fields[index].accessor)
            }
            MessageDescriptorImplRef::Dynamic => {
                FieldDescriptorImplRef::Dynamic(DynamicFieldDescriptorRef { field: self })
            }
        }
    }

    /// If this field a map field?
    pub fn is_map(&self) -> bool {
        match self.runtime_field_type() {
            RuntimeFieldType::Map(..) => true,
            _ => false,
        }
    }

    /// Check if field is set in given message.
    ///
    /// For repeated field or map field return `true` if
    /// collection is not empty.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn has_field(&self, m: &dyn MessageDyn) -> bool {
        match self.get_reflect(m) {
            ReflectFieldRef::Optional(v) => v.value().is_some(),
            ReflectFieldRef::Repeated(r) => !r.is_empty(),
            ReflectFieldRef::Map(m) => !m.is_empty(),
        }
    }

    // accessors

    fn singular(&self) -> SingularFieldAccessorRef {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(
                AccessorV2::Singular(ref a),
            )) => SingularFieldAccessorRef::Generated(a),
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(..)) => {
                panic!("not a singular field: {}", self)
            }
            FieldDescriptorImplRef::Dynamic(d) => SingularFieldAccessorRef::Dynamic(d),
        }
    }

    fn repeated(&self) -> RepeatedFieldAccessorRef {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(
                AccessorV2::Repeated(ref a),
            )) => RepeatedFieldAccessorRef::Generated(a),
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(..)) => {
                panic!("not a repeated field: {}", self)
            }
            FieldDescriptorImplRef::Dynamic(d) => RepeatedFieldAccessorRef::Dynamic(d),
        }
    }

    fn map(&self) -> MapFieldAccessorRef {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(AccessorV2::Map(
                ref a,
            ))) => MapFieldAccessorRef::Generated(a),
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(..)) => {
                panic!("not a map field: {}", self)
            }
            FieldDescriptorImplRef::Dynamic(d) => MapFieldAccessorRef::Dynamic(d),
        }
    }

    /// Obtain type of map key and value.
    pub(crate) fn map_proto_type(&self) -> (ProtobufType, ProtobufType) {
        match self.protobuf_field_type() {
            ProtobufFieldType::Map(k, v) => (k, v),
            _ => panic!("not a map field: {}", self),
        }
    }

    /// Get message field or default instance if field is unset.
    ///
    /// # Panics
    /// If this field belongs to a different message type or
    /// field type is not message.
    pub fn get_message<'a>(&self, m: &'a dyn MessageDyn) -> MessageRef<'a> {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Message(m) => m,
            _ => panic!("not message field: {}", self),
        }
    }

    /// Get a mutable reference to a message field.
    /// Initialize field with default message if unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field type is not singular message.
    pub fn mut_message<'a>(&self, m: &'a mut dyn MessageDyn) -> &'a mut dyn MessageDyn {
        match self.mut_singular_field_or_default(m) {
            ReflectValueMut::Message(m) => m,
        }
    }

    /// Default value.
    ///
    /// # Panics
    ///
    /// If field is not singular.
    pub fn singular_default_value(&self) -> ReflectValueRef {
        self.index().default_value(self)
    }

    /// Get singular field value.
    ///
    /// Return field default value if field is unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or fields is not singular.
    pub fn get_singular_field_or_default<'a>(&self, m: &'a dyn MessageDyn) -> ReflectValueRef<'a> {
        match self.get_singular(m) {
            Some(m) => m,
            None => self.index_with_message_lifetime(m).default_value(self),
        }
    }

    // Not public because it is not implemented for all types
    fn mut_singular_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
        match self.singular() {
            SingularFieldAccessorRef::Generated(g) => g.accessor.mut_field_or_default(m),
            SingularFieldAccessorRef::Dynamic(..) => {
                DynamicMessage::downcast_mut(m).mut_singular_field_or_default(self)
            }
        }
    }

    /// Runtime representation of singular field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not singular.
    pub fn singular_runtime_type(&self) -> RuntimeType {
        match self.runtime_field_type() {
            RuntimeFieldType::Singular(s) => s,
            _ => panic!("Not a singular field: {}", self),
        }
    }

    /// Set singular field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field is not singular or value is of different type.
    pub fn set_singular_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
        match self.singular() {
            SingularFieldAccessorRef::Generated(g) => g.accessor.set_field(m, value),
            SingularFieldAccessorRef::Dynamic(d) => d.set_field(m, value),
        }
    }

    /// Clear a field.
    pub fn clear_field(&self, m: &mut dyn MessageDyn) {
        if self.is_singular() {
            match self.singular() {
                SingularFieldAccessorRef::Generated(g) => g.accessor.clear_field(m),
                SingularFieldAccessorRef::Dynamic(d) => d.clear_field(m),
            }
        } else if self.is_repeated() {
            self.mut_repeated(m).clear();
        } else if self.is_map() {
            self.mut_map(m).clear();
        }
    }

    /// Dynamic representation of field type with wire type.
    pub(crate) fn protobuf_field_type(&self) -> ProtobufFieldType {
        self.index().field_type.resolve(self.file_descriptor())
    }

    /// Dynamic representation of field type.
    pub fn runtime_field_type(&self) -> RuntimeFieldType {
        self.protobuf_field_type().runtime()
    }

    /// Get field of any type.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn get_reflect<'a>(&self, m: &'a dyn MessageDyn) -> ReflectFieldRef<'a> {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(g) => g.get_reflect(m),
            FieldDescriptorImplRef::Dynamic(d) => d.get_reflect(m),
        }
    }

    /// Get singular field value.
    ///
    /// Return `None` if field is unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or fields is not singular.
    pub fn get_singular<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
        match self.get_reflect(m) {
            ReflectFieldRef::Optional(o) => o.value(),
            _ => panic!("not a singular field"),
        }
    }

    // repeated

    /// Get repeated field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not repeated.
    pub fn get_repeated<'a>(&self, m: &'a dyn MessageDyn) -> ReflectRepeatedRef<'a> {
        match self.get_reflect(m) {
            ReflectFieldRef::Repeated(r) => r,
            _ => panic!("not a repeated field"),
        }
    }

    /// Get a mutable reference to `repeated` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not `repeated`.
    pub fn mut_repeated<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectRepeatedMut<'a> {
        match self.repeated() {
            RepeatedFieldAccessorRef::Generated(g) => g.accessor.mut_repeated(m),
            RepeatedFieldAccessorRef::Dynamic(d) => d.mut_repeated(m),
        }
    }

    // map

    /// Get `map` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not `map`.
    pub fn get_map<'a>(&self, m: &'a dyn MessageDyn) -> ReflectMapRef<'a> {
        match self.get_reflect(m) {
            ReflectFieldRef::Map(m) => m,
            _ => panic!("not a map field"),
        }
    }

    /// Get a mutable reference to `map` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not `map`.
    pub fn mut_map<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectMapMut<'a> {
        match self.map() {
            MapFieldAccessorRef::Generated(g) => g.accessor.mut_reflect(m),
            MapFieldAccessorRef::Dynamic(d) => d.mut_map(m),
        }
    }
}

enum SingularFieldAccessorRef<'a> {
    Generated(&'a SingularFieldAccessorHolder),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

enum RepeatedFieldAccessorRef<'a> {
    Generated(&'a RepeatedFieldAccessorHolder),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

enum MapFieldAccessorRef<'a> {
    Generated(&'a MapFieldAccessorHolder),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

pub(crate) enum FieldDescriptorImplRef<'a> {
    Generated(&'static GeneratedFieldAccessor),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

#[cfg(test)]
mod test {
    use crate::descriptor::DescriptorProto;
    use crate::MessageFull;

    #[test]
    #[cfg_attr(miri, ignore)]
    fn display() {
        let field = DescriptorProto::descriptor()
            .field_by_name("enum_type")
            .unwrap();
        assert_eq!(
            "google.protobuf.DescriptorProto.enum_type",
            field.to_string()
        );
    }
}

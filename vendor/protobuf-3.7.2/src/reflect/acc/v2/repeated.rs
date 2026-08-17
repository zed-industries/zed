use std::fmt;
use std::marker;

use crate::message_dyn::MessageDyn;
use crate::message_full::MessageFull;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::repeated::ReflectRepeated;
use crate::reflect::repeated::ReflectRepeatedMut;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::ProtobufValue;
use crate::reflect::RuntimeType;

pub(crate) trait RepeatedFieldAccessor: Send + Sync + 'static {
    fn get_repeated<'a>(&self, m: &'a dyn MessageDyn) -> ReflectRepeatedRef<'a>;
    fn mut_repeated<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectRepeatedMut<'a>;
    fn _element_type(&self) -> RuntimeType;
}

pub(crate) struct RepeatedFieldAccessorHolder {
    pub accessor: Box<dyn RepeatedFieldAccessor>,
}

impl<'a> fmt::Debug for RepeatedFieldAccessorHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepeatedFieldAccessorHolder").finish()
    }
}

trait RepeatedFieldGetMut<M, R: ?Sized>: Send + Sync + 'static
where
    M: MessageFull + 'static,
{
    fn get_field<'a>(&self, message: &'a M) -> &'a R;
    fn mut_field<'a>(&self, message: &'a mut M) -> &'a mut R;
}

struct RepeatedFieldGetMutImpl<M, L>
where
    M: MessageFull + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a L,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut L,
}

impl<M, V> RepeatedFieldGetMut<M, dyn ReflectRepeated> for RepeatedFieldGetMutImpl<M, Vec<V>>
where
    M: MessageFull + 'static,
    V: ProtobufValue,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a dyn ReflectRepeated {
        (self.get_field)(m) as &dyn ReflectRepeated
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut dyn ReflectRepeated {
        (self.mut_field)(m) as &mut dyn ReflectRepeated
    }
}

struct RepeatedFieldAccessorImpl<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    fns: Box<dyn RepeatedFieldGetMut<M, dyn ReflectRepeated>>,
    _marker: marker::PhantomData<V>,
}

impl<M, V> RepeatedFieldAccessor for RepeatedFieldAccessorImpl<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    fn get_repeated<'a>(&self, m: &'a dyn MessageDyn) -> ReflectRepeatedRef<'a> {
        let m = m.downcast_ref().unwrap();
        let repeated = self.fns.get_field(m);
        ReflectRepeatedRef::new(repeated)
    }

    fn mut_repeated<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectRepeatedMut<'a> {
        let m = m.downcast_mut().unwrap();
        let repeated = self.fns.mut_field(m);
        ReflectRepeatedMut::new(repeated)
    }

    fn _element_type(&self) -> RuntimeType {
        V::RuntimeType::runtime_type_box()
    }
}

/// Make accessor for `Vec` field
pub fn make_vec_simpler_accessor<M, V>(
    name: &'static str,
    get_vec: for<'a> fn(&'a M) -> &'a Vec<V>,
    mut_vec: for<'a> fn(&'a mut M) -> &'a mut Vec<V>,
) -> FieldAccessor
where
    M: MessageFull + 'static,
    V: ProtobufValue,
{
    FieldAccessor::new(
        name,
        AccessorV2::Repeated(RepeatedFieldAccessorHolder {
            accessor: Box::new(RepeatedFieldAccessorImpl::<M, V> {
                fns: Box::new(RepeatedFieldGetMutImpl::<M, Vec<V>> {
                    get_field: get_vec,
                    mut_field: mut_vec,
                }),
                _marker: marker::PhantomData::<V>,
            }),
        }),
    )
}

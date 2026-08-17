use std::fmt;

use crate::reflect::map::ReflectMapIter;
use crate::reflect::map::ReflectMapIterTrait;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeType;

#[derive(Clone)]
pub(crate) struct DynamicEmptyMap {
    key_type: RuntimeType,
    value_type: RuntimeType,
}

impl DynamicEmptyMap {
    pub(crate) fn new(key_type: RuntimeType, value_type: RuntimeType) -> DynamicEmptyMap {
        Self {
            key_type,
            value_type,
        }
    }

    pub(crate) fn len(&self) -> usize {
        0
    }

    pub(crate) fn is_empty(&self) -> bool {
        true
    }

    pub(crate) fn get<'a>(&'a self, _key: ReflectValueRef) -> Option<ReflectValueRef<'a>> {
        None
    }

    pub(crate) fn key_type(&self) -> RuntimeType {
        self.key_type.clone()
    }

    pub(crate) fn value_type(&self) -> RuntimeType {
        self.value_type.clone()
    }

    pub(crate) fn reflect_iter(&self) -> ReflectMapIter {
        ReflectMapIter::new(DynamicEmptyMapIter {
            _key_type: self.key_type.clone(),
            _value_type: self.value_type.clone(),
        })
    }
}

impl fmt::Debug for DynamicEmptyMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().finish()
    }
}

struct DynamicEmptyMapIter {
    _key_type: RuntimeType,
    _value_type: RuntimeType,
}

impl<'a> ReflectMapIterTrait<'a> for DynamicEmptyMapIter {
    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        None
    }

    fn _key_type(&self) -> RuntimeType {
        self._key_type.clone()
    }

    fn _value_type(&self) -> RuntimeType {
        self._value_type.clone()
    }
}

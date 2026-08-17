use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::ReflectOptionalRef;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeType;

#[derive(Debug, Clone)]
pub(crate) struct DynamicOptional {
    elem: RuntimeType,
    value: Option<ReflectValueBox>,
}

impl DynamicOptional {
    pub(crate) fn none(elem: RuntimeType) -> DynamicOptional {
        DynamicOptional { elem, value: None }
    }

    pub(crate) fn mut_or_default(&mut self) -> ReflectValueMut {
        if let None = self.value {
            self.value = Some(self.elem.default_value_ref().to_box());
        }
        self.value.as_mut().unwrap().as_value_mut()
    }

    pub(crate) fn clear(&mut self) {
        self.value = None;
    }

    pub(crate) fn set(&mut self, value: ReflectValueBox) {
        assert_eq!(value.get_type(), self.elem);
        self.value = Some(value);
    }

    pub(crate) fn reflect_singlar_ref(&self) -> ReflectOptionalRef {
        match &self.value {
            Some(value) => ReflectOptionalRef::some(value.as_value_ref()),
            None => ReflectOptionalRef::none(self.elem.clone()),
        }
    }
}

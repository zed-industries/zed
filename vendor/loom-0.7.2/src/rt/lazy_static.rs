use crate::rt::synchronize::Synchronize;
use std::{any::Any, collections::HashMap};

pub(crate) struct Set {
    /// Registered statics.
    statics: Option<HashMap<StaticKeyId, StaticValue>>,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub(crate) struct StaticKeyId(usize);

pub(crate) struct StaticValue {
    pub(crate) sync: Synchronize,
    v: Box<dyn Any>,
}

impl Set {
    /// Create an empty statics set.
    pub(crate) fn new() -> Set {
        Set {
            statics: Some(HashMap::new()),
        }
    }

    pub(crate) fn reset(&mut self) {
        assert!(
            self.statics.is_none(),
            "lazy_static was not dropped during execution"
        );
        self.statics = Some(HashMap::new());
    }

    pub(crate) fn drop(&mut self) -> HashMap<StaticKeyId, StaticValue> {
        self.statics
            .take()
            .expect("lazy_statics were dropped twice in one execution")
    }

    pub(crate) fn get_static<T: 'static>(
        &mut self,
        key: &'static crate::lazy_static::Lazy<T>,
    ) -> Option<&mut StaticValue> {
        self.statics
            .as_mut()
            .expect("attempted to access lazy_static during shutdown")
            .get_mut(&StaticKeyId::new(key))
    }

    pub(crate) fn init_static<T: 'static>(
        &mut self,
        key: &'static crate::lazy_static::Lazy<T>,
        value: StaticValue,
    ) -> &mut StaticValue {
        let v = self
            .statics
            .as_mut()
            .expect("attempted to access lazy_static during shutdown")
            .entry(StaticKeyId::new(key));

        if let std::collections::hash_map::Entry::Occupied(_) = v {
            unreachable!("told to init static, but it was already init'd");
        }

        v.or_insert(value)
    }
}

impl StaticKeyId {
    fn new<T>(key: &'static crate::lazy_static::Lazy<T>) -> Self {
        Self(key as *const _ as usize)
    }
}

impl StaticValue {
    pub(crate) fn new<T: 'static>(value: T) -> Self {
        Self {
            sync: Synchronize::new(),
            v: Box::new(value),
        }
    }

    pub(crate) fn get<T: 'static>(&self) -> &T {
        self.v
            .downcast_ref::<T>()
            .expect("lazy value must downcast to expected type")
    }
}

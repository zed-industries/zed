use crate::AHashMap;
use alloc::vec::Vec;
use xim_parser::{Attribute, AttributeName, XimWrite};

pub struct NestedListBuilder<'a> {
    id_map: &'a AHashMap<AttributeName, u16>,
    out: &'a mut Vec<u8>,
}

impl<'a> NestedListBuilder<'a> {
    pub fn push<V: XimWrite>(self, name: AttributeName, value: V) -> Self {
        if let Some(id) = self.id_map.get(&name).copied() {
            let attr = Attribute {
                id,
                value: xim_parser::write_to_vec(value),
            };
            xim_parser::write_extend_vec(attr, self.out);
        }

        self
    }
}

pub struct AttributeBuilder<'a> {
    id_map: &'a AHashMap<AttributeName, u16>,
    out: Vec<Attribute>,
}

impl<'a> AttributeBuilder<'a> {
    pub(crate) fn new(id_map: &'a AHashMap<AttributeName, u16>) -> Self {
        Self {
            id_map,
            out: Vec::new(),
        }
    }

    pub fn push<V: XimWrite>(mut self, name: AttributeName, value: V) -> Self {
        if let Some(id) = self.id_map.get(&name).copied() {
            self.out.push(Attribute {
                id,
                value: xim_parser::write_to_vec(value),
            });
        }

        self
    }

    pub fn nested_list(mut self, name: AttributeName, f: impl FnOnce(NestedListBuilder)) -> Self {
        if let Some(id) = self.id_map.get(&name).copied() {
            let mut value = Vec::new();
            f(NestedListBuilder {
                id_map: self.id_map,
                out: &mut value,
            });
            self.out.push(Attribute { id, value });
        }

        self
    }

    pub fn build(self) -> Vec<Attribute> {
        self.out
    }
}

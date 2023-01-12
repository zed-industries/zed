use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use super::Binding;

#[derive(Default)]
pub struct Keymap {
    bindings: Vec<Binding>,
    binding_indices_by_action_type: HashMap<TypeId, SmallVec<[usize; 3]>>,
}

impl Keymap {
    pub fn new(bindings: Vec<Binding>) -> Self {
        let mut binding_indices_by_action_type = HashMap::new();
        for (ix, binding) in bindings.iter().enumerate() {
            binding_indices_by_action_type
                .entry(binding.action().type_id())
                .or_insert_with(SmallVec::new)
                .push(ix);
        }

        Self {
            binding_indices_by_action_type,
            bindings,
        }
    }

    pub(crate) fn bindings_for_action_type(
        &self,
        action_type: TypeId,
    ) -> impl Iterator<Item = &'_ Binding> {
        self.binding_indices_by_action_type
            .get(&action_type)
            .map(SmallVec::as_slice)
            .unwrap_or(&[])
            .iter()
            .map(|ix| &self.bindings[*ix])
    }

    pub(crate) fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        for binding in bindings {
            self.binding_indices_by_action_type
                .entry(binding.action().as_any().type_id())
                .or_default()
                .push(self.bindings.len());
            self.bindings.push(binding);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_type.clear();
    }

    pub fn bindings(&self) -> &Vec<Binding> {
        &self.bindings
    }
}

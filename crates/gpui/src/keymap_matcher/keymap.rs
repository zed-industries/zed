use smallvec::SmallVec;
use std::{any::TypeId, collections::HashMap};

use super::Binding;

#[derive(Default)]
pub struct Keymap {
    bindings: Vec<Binding>,
    binding_indices_by_action_id: HashMap<TypeId, SmallVec<[usize; 3]>>,
}

impl Keymap {
    pub fn new(bindings: Vec<Binding>) -> Self {
        let mut binding_indices_by_action_id = HashMap::new();
        for (ix, binding) in bindings.iter().enumerate() {
            binding_indices_by_action_id
                .entry(binding.action().id())
                .or_insert_with(SmallVec::new)
                .push(ix);
        }

        Self {
            binding_indices_by_action_id,
            bindings,
        }
    }

    pub(crate) fn bindings_for_action(
        &self,
        action_id: TypeId,
    ) -> impl Iterator<Item = &'_ Binding> {
        self.binding_indices_by_action_id
            .get(&action_id)
            .map(SmallVec::as_slice)
            .unwrap_or(&[])
            .iter()
            .map(|ix| &self.bindings[*ix])
    }

    pub(crate) fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        for binding in bindings {
            self.binding_indices_by_action_id
                .entry(binding.action().id())
                .or_default()
                .push(self.bindings.len());
            self.bindings.push(binding);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_id.clear();
    }

    pub fn bindings(&self) -> &Vec<Binding> {
        &self.bindings
    }
}

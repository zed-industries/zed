// FIXME: This file was copied from vm/component/resources.rs and should be
// deduplicated as part of
// https://github.com/bytecodealliance/wasmtime/issues/11189.

use alloc::vec::Vec;
use anyhow::{Result, bail};
use core::mem;

/// The maximum handle value is specified in
/// <https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md>
/// currently and keeps the upper bit free for use in the component.
const MAX_HANDLE: u32 = 1 << 30;

enum Slot<T> {
    Free { next: u32 },
    Occupied { rep: u32, state: T },
}

pub struct StateTable<T> {
    next: u32,
    slots: Vec<Slot<T>>,
    // TODO: This is a sparse table (where zero means "no entry"); it might make
    // more sense to use a `HashMap` here, but we'd need one that's
    // no_std-compatible.  A `BTreeMap` might also be appropriate if we restrict
    // ourselves to `alloc::collections`.
    reps_to_indexes: Vec<u32>,
}

impl<T> Default for StateTable<T> {
    fn default() -> Self {
        Self {
            next: 0,
            slots: Vec::new(),
            reps_to_indexes: Vec::new(),
        }
    }
}

impl<T> StateTable<T> {
    /// Returns whether or not this table is empty.
    pub fn is_empty(&self) -> bool {
        self.slots
            .iter()
            .all(|slot| matches!(slot, Slot::Free { .. }))
    }

    pub fn insert(&mut self, rep: u32, state: T) -> Result<u32> {
        if matches!(self
            .reps_to_indexes
            .get(usize::try_from(rep).unwrap()), Some(idx) if *idx != 0)
        {
            bail!("rep {rep} already exists in this table");
        }

        let next = self.next as usize;
        if next == self.slots.len() {
            self.slots.push(Slot::Free {
                next: self.next.checked_add(1).unwrap(),
            });
        }
        let ret = self.next;
        self.next = match mem::replace(&mut self.slots[next], Slot::Occupied { rep, state }) {
            Slot::Free { next } => next,
            _ => unreachable!(),
        };
        // The component model reserves index 0 as never allocatable so add one
        // to the table index to start the numbering at 1 instead. Also note
        // that the component model places an upper-limit per-table on the
        // maximum allowed index.
        let ret = ret + 1;
        if ret >= MAX_HANDLE {
            bail!("cannot allocate another handle: index overflow");
        }

        let rep = usize::try_from(rep).unwrap();
        if self.reps_to_indexes.len() <= rep {
            self.reps_to_indexes.resize(rep.checked_add(1).unwrap(), 0);
        }

        self.reps_to_indexes[rep] = ret;

        Ok(ret)
    }

    fn handle_index_to_table_index(&self, idx: u32) -> Option<usize> {
        // NB: `idx` is decremented by one to account for the `+1` above during
        // allocation.
        let idx = idx.checked_sub(1)?;
        usize::try_from(idx).ok()
    }

    fn get_mut(&mut self, idx: u32) -> Result<&mut Slot<T>> {
        let slot = self
            .handle_index_to_table_index(idx)
            .and_then(|i| self.slots.get_mut(i));
        match slot {
            None | Some(Slot::Free { .. }) => bail!("unknown handle index {idx}"),
            Some(slot) => Ok(slot),
        }
    }

    pub fn has_handle(&self, idx: u32) -> bool {
        matches!(
            self.handle_index_to_table_index(idx)
                .and_then(|i| self.slots.get(i)),
            Some(Slot::Occupied { .. })
        )
    }

    pub fn get_mut_by_index(&mut self, idx: u32) -> Result<(u32, &mut T)> {
        let slot = self
            .handle_index_to_table_index(idx)
            .and_then(|i| self.slots.get_mut(i));
        match slot {
            None | Some(Slot::Free { .. }) => bail!("unknown handle index {idx}"),
            Some(Slot::Occupied { rep, state }) => Ok((*rep, state)),
        }
    }

    pub fn get_mut_by_rep(&mut self, rep: u32) -> Option<(u32, &mut T)> {
        let index = *self.reps_to_indexes.get(usize::try_from(rep).unwrap())?;
        if index > 0 {
            let (_, state) = self.get_mut_by_index(index).unwrap();
            Some((index, state))
        } else {
            None
        }
    }

    pub fn remove_by_index(&mut self, idx: u32) -> Result<(u32, T)> {
        let to_fill = Slot::Free { next: self.next };
        let Slot::Occupied { rep, state } = mem::replace(self.get_mut(idx)?, to_fill) else {
            unreachable!()
        };
        self.next = idx - 1;
        {
            let rep = usize::try_from(rep).unwrap();
            assert_eq!(idx, self.reps_to_indexes[rep]);
            self.reps_to_indexes[rep] = 0;
        }
        Ok((rep, state))
    }
}

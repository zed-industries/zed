use crate::{MachineEnv, PReg, RegClass};

/// This iterator represents a traversal through all allocatable
/// registers of a given class, in a certain order designed to
/// minimize allocation contention.
///
/// The order in which we try registers is somewhat complex:
/// - First, if there is a hint, we try that.
/// - Then, we try registers in a traversal order that is based on an
///   "offset" (usually the bundle index) spreading pressure evenly
///   among registers to reduce commitment-map contention.
/// - Within that scan, we try registers in two groups: first,
///   prferred registers; then, non-preferred registers. (In normal
///   usage, these consist of caller-save and callee-save registers
///   respectively, to minimize clobber-saves; but they need not.)

pub struct RegTraversalIter<'a> {
    env: &'a MachineEnv,
    class: usize,
    hints: [Option<PReg>; 2],
    hint_idx: usize,
    pref_idx: usize,
    non_pref_idx: usize,
    offset_pref: usize,
    offset_non_pref: usize,
    is_fixed: bool,
    fixed: Option<PReg>,
}

impl<'a> RegTraversalIter<'a> {
    pub fn new(
        env: &'a MachineEnv,
        class: RegClass,
        hint_reg: PReg,
        hint2_reg: PReg,
        offset: usize,
        fixed: Option<PReg>,
    ) -> Self {
        let mut hint_reg = if hint_reg != PReg::invalid() {
            Some(hint_reg)
        } else {
            None
        };
        let mut hint2_reg = if hint2_reg != PReg::invalid() {
            Some(hint2_reg)
        } else {
            None
        };

        if hint_reg.is_none() {
            hint_reg = hint2_reg;
            hint2_reg = None;
        }
        let hints = [hint_reg, hint2_reg];
        let class = class as u8 as usize;
        let offset_pref = if env.preferred_regs_by_class[class].len() > 0 {
            offset % env.preferred_regs_by_class[class].len()
        } else {
            0
        };
        let offset_non_pref = if env.non_preferred_regs_by_class[class].len() > 0 {
            offset % env.non_preferred_regs_by_class[class].len()
        } else {
            0
        };
        Self {
            env,
            class,
            hints,
            hint_idx: 0,
            pref_idx: 0,
            non_pref_idx: 0,
            offset_pref,
            offset_non_pref,
            is_fixed: fixed.is_some(),
            fixed,
        }
    }
}

impl<'a> core::iter::Iterator for RegTraversalIter<'a> {
    type Item = PReg;

    fn next(&mut self) -> Option<PReg> {
        if self.is_fixed {
            let ret = self.fixed;
            self.fixed = None;
            return ret;
        }

        fn wrap(idx: usize, limit: usize) -> usize {
            if idx >= limit {
                idx - limit
            } else {
                idx
            }
        }
        if self.hint_idx < 2 && self.hints[self.hint_idx].is_some() {
            let h = self.hints[self.hint_idx];
            self.hint_idx += 1;
            return h;
        }
        while self.pref_idx < self.env.preferred_regs_by_class[self.class].len() {
            let arr = &self.env.preferred_regs_by_class[self.class][..];
            let r = arr[wrap(self.pref_idx + self.offset_pref, arr.len())];
            self.pref_idx += 1;
            if Some(r) == self.hints[0] || Some(r) == self.hints[1] {
                continue;
            }
            return Some(r);
        }
        while self.non_pref_idx < self.env.non_preferred_regs_by_class[self.class].len() {
            let arr = &self.env.non_preferred_regs_by_class[self.class][..];
            let r = arr[wrap(self.non_pref_idx + self.offset_non_pref, arr.len())];
            self.non_pref_idx += 1;
            if Some(r) == self.hints[0] || Some(r) == self.hints[1] {
                continue;
            }
            return Some(r);
        }
        None
    }
}

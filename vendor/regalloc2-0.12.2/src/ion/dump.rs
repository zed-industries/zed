//! Debugging output.

use alloc::string::ToString;
use alloc::{format, vec};
use alloc::{string::String, vec::Vec};

use super::Env;
use crate::{Block, Function, ProgPoint};

impl<'a, F: Function> Env<'a, F> {
    pub fn dump_state(&self) {
        trace!("Bundles:");
        for (i, b) in self.bundles.iter().enumerate() {
            trace!(
                "bundle{}: spillset={:?} alloc={:?}",
                i,
                b.spillset,
                b.allocation
            );
            for entry in &b.ranges {
                trace!(
                    " * range {:?} -- {:?}: range{}",
                    entry.range.from,
                    entry.range.to,
                    entry.index.index()
                );
            }
        }
        trace!("VRegs:");
        for (i, v) in self.vregs.iter().enumerate() {
            trace!("vreg{}:", i);
            for entry in &v.ranges {
                trace!(
                    " * range {:?} -- {:?}: range{}",
                    entry.range.from,
                    entry.range.to,
                    entry.index.index()
                );
            }
        }
        trace!("Ranges:");
        for (i, r) in self.ranges.iter().enumerate() {
            trace!(
                "range{}: range={:?} vreg={:?} bundle={:?} weight={:?}",
                i,
                r.range,
                r.vreg,
                r.bundle,
                r.uses_spill_weight(),
            );
            for u in &r.uses {
                trace!(" * use at {:?} (slot {}): {:?}", u.pos, u.slot, u.operand);
            }
        }
    }

    pub fn annotate(&mut self, progpoint: ProgPoint, s: String) {
        if self.annotations_enabled {
            self.debug_annotations
                .entry(progpoint)
                .or_insert_with(|| vec![])
                .push(s);
        }
    }

    pub fn dump_results(&self) {
        log::info!("=== REGALLOC RESULTS ===");
        for block in 0..self.func.num_blocks() {
            let block = Block::new(block);
            log::info!(
                "block{}: [succs {:?} preds {:?}]",
                block.index(),
                self.func
                    .block_succs(block)
                    .iter()
                    .map(|b| b.index())
                    .collect::<Vec<_>>(),
                self.func
                    .block_preds(block)
                    .iter()
                    .map(|b| b.index())
                    .collect::<Vec<_>>()
            );
            for inst in self.func.block_insns(block).iter() {
                for annotation in self
                    .debug_annotations
                    .get(&ProgPoint::before(inst))
                    .map(|v| &v[..])
                    .unwrap_or(&[])
                {
                    log::info!("  inst{}-pre: {}", inst.index(), annotation);
                }
                let ops = self
                    .func
                    .inst_operands(inst)
                    .iter()
                    .map(|op| format!("{}", op))
                    .collect::<Vec<_>>();
                let clobbers = self
                    .func
                    .inst_clobbers(inst)
                    .into_iter()
                    .map(|preg| format!("{}", preg))
                    .collect::<Vec<_>>();
                let allocs = (0..ops.len())
                    .map(|i| format!("{}", self.get_alloc(inst, i)))
                    .collect::<Vec<_>>();
                let opname = if self.func.is_branch(inst) {
                    "br"
                } else if self.func.is_ret(inst) {
                    "ret"
                } else {
                    "op"
                };
                let args = ops
                    .iter()
                    .zip(allocs.iter())
                    .map(|(op, alloc)| format!("{} [{}]", op, alloc))
                    .collect::<Vec<_>>();
                let clobbers = if clobbers.is_empty() {
                    "".to_string()
                } else {
                    format!(" [clobber: {}]", clobbers.join(", "))
                };
                log::info!(
                    "  inst{}: {} {}{}",
                    inst.index(),
                    opname,
                    args.join(", "),
                    clobbers
                );
                for annotation in self
                    .debug_annotations
                    .get(&ProgPoint::after(inst))
                    .map(|v| &v[..])
                    .unwrap_or(&[])
                {
                    log::info!("  inst{}-post: {}", inst.index(), annotation);
                }
            }
        }
    }
}

//! The `CFGPrinter` utility.

use alloc::vec::Vec;
use core::fmt::{Display, Formatter, Result, Write};

use crate::entity::SecondaryMap;
use crate::flowgraph::{BlockPredecessor, ControlFlowGraph};
use crate::ir::Function;
use crate::write::{FuncWriter, PlainWriter};

/// A utility for pretty-printing the CFG of a `Function`.
pub struct CFGPrinter<'a> {
    func: &'a Function,
    cfg: ControlFlowGraph,
}

/// A utility for pretty-printing the CFG of a `Function`.
impl<'a> CFGPrinter<'a> {
    /// Create a new CFGPrinter.
    pub fn new(func: &'a Function) -> Self {
        Self {
            func,
            cfg: ControlFlowGraph::with_function(func),
        }
    }

    /// Write the CFG for this function to `w`.
    pub fn write(&self, w: &mut dyn Write) -> Result {
        self.header(w)?;
        self.block_nodes(w)?;
        self.cfg_connections(w)?;
        writeln!(w, "}}")
    }

    fn header(&self, w: &mut dyn Write) -> Result {
        writeln!(w, "digraph \"{}\" {{", self.func.name)?;
        if let Some(entry) = self.func.layout.entry_block() {
            writeln!(w, "    {{rank=min; {entry}}}")?;
        }
        Ok(())
    }

    fn block_nodes(&self, w: &mut dyn Write) -> Result {
        let mut aliases = SecondaryMap::<_, Vec<_>>::new();
        for v in self.func.dfg.values() {
            // VADFS returns the immediate target of an alias
            if let Some(k) = self.func.dfg.value_alias_dest_for_serialization(v) {
                aliases[k].push(v);
            }
        }

        for block in &self.func.layout {
            write!(w, "    {block} [shape=record, label=\"{{")?;
            crate::write::write_block_header(w, self.func, block, 4)?;
            // Add all outgoing branch instructions to the label.
            if let Some(inst) = self.func.layout.last_inst(block) {
                write!(w, " | <{inst}>")?;
                PlainWriter.write_instruction(w, self.func, &aliases, inst, 0)?;
            }
            writeln!(w, "}}\"]")?
        }
        Ok(())
    }

    fn cfg_connections(&self, w: &mut dyn Write) -> Result {
        for block in &self.func.layout {
            for BlockPredecessor {
                block: parent,
                inst,
            } in self.cfg.pred_iter(block)
            {
                writeln!(w, "    {parent}:{inst} -> {block}")?;
            }
        }
        Ok(())
    }
}

impl<'a> Display for CFGPrinter<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write(f)
    }
}

//! Cranelift compilation context and main entry point.
//!
//! When compiling many small functions, it is important to avoid repeatedly allocating and
//! deallocating the data structures needed for compilation. The `Context` struct is used to hold
//! on to memory allocations between function compilations.
//!
//! The context does not hold a `TargetIsa` instance which has to be provided as an argument
//! instead. This is because an ISA instance is immutable and can be used by multiple compilation
//! contexts concurrently. Typically, you would have one context per compilation thread and only a
//! single ISA instance.

use crate::alias_analysis::AliasAnalysis;
use crate::dominator_tree::DominatorTree;
use crate::dominator_tree::DominatorTreePreorder;
use crate::egraph::EgraphPass;
use crate::flowgraph::ControlFlowGraph;
use crate::inline::{Inline, do_inlining};
use crate::ir::Function;
use crate::isa::TargetIsa;
use crate::legalizer::simple_legalize;
use crate::loop_analysis::LoopAnalysis;
use crate::machinst::{CompiledCode, CompiledCodeStencil};
use crate::nan_canonicalization::do_nan_canonicalization;
use crate::remove_constant_phis::do_remove_constant_phis;
use crate::result::{CodegenResult, CompileResult};
use crate::settings::{FlagsOrIsa, OptLevel};
use crate::trace;
use crate::unreachable_code::eliminate_unreachable_code;
use crate::verifier::{VerifierErrors, VerifierResult, verify_context};
use crate::{CompileError, timing};
#[cfg(feature = "souper-harvest")]
use alloc::string::String;
use alloc::vec::Vec;
use cranelift_control::ControlPlane;
use target_lexicon::Architecture;

#[cfg(feature = "souper-harvest")]
use crate::souper_harvest::do_souper_harvest;

/// Persistent data structures and compilation pipeline.
pub struct Context {
    /// The function we're compiling.
    pub func: Function,

    /// The control flow graph of `func`.
    pub cfg: ControlFlowGraph,

    /// Dominator tree for `func`.
    pub domtree: DominatorTree,

    /// Dominator tree with dominance stored implicitly via visit-order indices for `func`
    domtree_preorder: DominatorTreePreorder,

    /// Loop analysis of `func`.
    pub loop_analysis: LoopAnalysis,

    /// Result of MachBackend compilation, if computed.
    pub(crate) compiled_code: Option<CompiledCode>,

    /// Flag: do we want a disassembly with the CompiledCode?
    pub want_disasm: bool,
}

impl Context {
    /// Allocate a new compilation context.
    ///
    /// The returned instance should be reused for compiling multiple functions in order to avoid
    /// needless allocator thrashing.
    pub fn new() -> Self {
        Self::for_function(Function::new())
    }

    /// Allocate a new compilation context with an existing Function.
    ///
    /// The returned instance should be reused for compiling multiple functions in order to avoid
    /// needless allocator thrashing.
    pub fn for_function(func: Function) -> Self {
        Self {
            func,
            cfg: ControlFlowGraph::new(),
            domtree: DominatorTree::new(),
            domtree_preorder: DominatorTreePreorder::new(),
            loop_analysis: LoopAnalysis::new(),
            compiled_code: None,
            want_disasm: false,
        }
    }

    /// Clear all data structures in this context.
    pub fn clear(&mut self) {
        self.func.clear();
        self.cfg.clear();
        self.domtree.clear();
        self.loop_analysis.clear();
        self.compiled_code = None;
        self.want_disasm = false;
    }

    /// Returns the compilation result for this function, available after any `compile` function
    /// has been called.
    pub fn compiled_code(&self) -> Option<&CompiledCode> {
        self.compiled_code.as_ref()
    }

    /// Returns the compilation result for this function, available after any `compile` function
    /// has been called.
    pub fn take_compiled_code(&mut self) -> Option<CompiledCode> {
        self.compiled_code.take()
    }

    /// Set the flag to request a disassembly when compiling with a
    /// `MachBackend` backend.
    pub fn set_disasm(&mut self, val: bool) {
        self.want_disasm = val;
    }

    /// Compile the function, and emit machine code into a `Vec<u8>`.
    #[deprecated = "use Context::compile"]
    pub fn compile_and_emit(
        &mut self,
        isa: &dyn TargetIsa,
        mem: &mut Vec<u8>,
        ctrl_plane: &mut ControlPlane,
    ) -> CompileResult<'_, &CompiledCode> {
        let compiled_code = self.compile(isa, ctrl_plane)?;
        mem.extend_from_slice(compiled_code.code_buffer());
        Ok(compiled_code)
    }

    /// Internally compiles the function into a stencil.
    ///
    /// Public only for testing and fuzzing purposes.
    pub fn compile_stencil(
        &mut self,
        isa: &dyn TargetIsa,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        let result;
        trace!("****** START compiling {}", self.func.display_spec());
        {
            let _tt = timing::compile();

            self.verify_if(isa)?;
            self.optimize(isa, ctrl_plane)?;
            result = isa.compile_function(&self.func, &self.domtree, self.want_disasm, ctrl_plane);
        }
        trace!("****** DONE compiling {}\n", self.func.display_spec());
        result
    }

    /// Optimize the function, performing all compilation steps up to
    /// but not including machine-code lowering and register
    /// allocation.
    ///
    /// Public only for testing purposes.
    pub fn optimize(
        &mut self,
        isa: &dyn TargetIsa,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<()> {
        log::debug!(
            "Number of CLIF instructions to optimize: {}",
            self.func.dfg.num_insts()
        );
        log::debug!(
            "Number of CLIF blocks to optimize: {}",
            self.func.dfg.num_blocks()
        );

        let opt_level = isa.flags().opt_level();
        crate::trace!(
            "Optimizing (opt level {:?}):\n{}",
            opt_level,
            self.func.display()
        );

        if isa.flags().enable_nan_canonicalization() {
            self.canonicalize_nans(isa)?;
        }

        self.legalize(isa)?;

        self.compute_cfg();
        self.compute_domtree();
        self.eliminate_unreachable_code(isa)?;
        self.remove_constant_phis(isa)?;

        self.func.dfg.resolve_all_aliases();

        if opt_level != OptLevel::None {
            self.egraph_pass(isa, ctrl_plane)?;
        }

        Ok(())
    }

    /// Perform function call inlining.
    ///
    /// Returns `true` if any function call was inlined, `false` otherwise.
    pub fn inline(&mut self, inliner: impl Inline) -> CodegenResult<bool> {
        do_inlining(&mut self.func, inliner)
    }

    /// Compile the function,
    ///
    /// Run the function through all the passes necessary to generate
    /// code for the target ISA represented by `isa`. The generated
    /// machine code is not relocated. Instead, any relocations can be
    /// obtained from `compiled_code.buffer.relocs()`.
    ///
    /// Performs any optimizations that are enabled, unless
    /// `optimize()` was already invoked.
    ///
    /// Returns the generated machine code as well as information about
    /// the function's code and read-only data.
    pub fn compile(
        &mut self,
        isa: &dyn TargetIsa,
        ctrl_plane: &mut ControlPlane,
    ) -> CompileResult<'_, &CompiledCode> {
        let stencil = self
            .compile_stencil(isa, ctrl_plane)
            .map_err(|error| CompileError {
                inner: error,
                func: &self.func,
            })?;
        Ok(self
            .compiled_code
            .insert(stencil.apply_params(&self.func.params)))
    }

    /// If available, return information about the code layout in the
    /// final machine code: the offsets (in bytes) of each basic-block
    /// start, and all basic-block edges.
    #[deprecated = "use CompiledCode::get_code_bb_layout"]
    pub fn get_code_bb_layout(&self) -> Option<(Vec<usize>, Vec<(usize, usize)>)> {
        self.compiled_code().map(CompiledCode::get_code_bb_layout)
    }

    /// Creates unwind information for the function.
    ///
    /// Returns `None` if the function has no unwind information.
    #[cfg(feature = "unwind")]
    #[deprecated = "use CompiledCode::create_unwind_info"]
    pub fn create_unwind_info(
        &self,
        isa: &dyn TargetIsa,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        self.compiled_code().unwrap().create_unwind_info(isa)
    }

    /// Run the verifier on the function.
    ///
    /// Also check that the dominator tree and control flow graph are consistent with the function.
    ///
    /// TODO: rename to "CLIF validate" or similar.
    pub fn verify<'a, FOI: Into<FlagsOrIsa<'a>>>(&self, fisa: FOI) -> VerifierResult<()> {
        let mut errors = VerifierErrors::default();
        let _ = verify_context(&self.func, &self.cfg, &self.domtree, fisa, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Run the verifier only if the `enable_verifier` setting is true.
    pub fn verify_if<'a, FOI: Into<FlagsOrIsa<'a>>>(&self, fisa: FOI) -> CodegenResult<()> {
        let fisa = fisa.into();
        if fisa.flags.enable_verifier() {
            self.verify(fisa)?;
        }
        Ok(())
    }

    /// Perform constant-phi removal on the function.
    pub fn remove_constant_phis<'a, FOI: Into<FlagsOrIsa<'a>>>(
        &mut self,
        fisa: FOI,
    ) -> CodegenResult<()> {
        do_remove_constant_phis(&mut self.func, &mut self.domtree);
        self.verify_if(fisa)?;
        Ok(())
    }

    /// Perform NaN canonicalizing rewrites on the function.
    pub fn canonicalize_nans(&mut self, isa: &dyn TargetIsa) -> CodegenResult<()> {
        // Currently only RiscV64 is the only arch that may not have vector support.
        let has_vector_support = match isa.triple().architecture {
            Architecture::Riscv64(_) => match isa.isa_flags().iter().find(|f| f.name == "has_v") {
                Some(value) => value.as_bool().unwrap_or(false),
                None => false,
            },
            _ => true,
        };
        do_nan_canonicalization(&mut self.func, has_vector_support);
        self.verify_if(isa)
    }

    /// Run the legalizer for `isa` on the function.
    pub fn legalize(&mut self, isa: &dyn TargetIsa) -> CodegenResult<()> {
        // Legalization invalidates the domtree and loop_analysis by mutating the CFG.
        // TODO: Avoid doing this when legalization doesn't actually mutate the CFG.
        self.domtree.clear();
        self.loop_analysis.clear();
        self.cfg.clear();

        // Run some specific legalizations only.
        simple_legalize(&mut self.func, isa);
        self.verify_if(isa)
    }

    /// Compute the control flow graph.
    pub fn compute_cfg(&mut self) {
        self.cfg.compute(&self.func)
    }

    /// Compute dominator tree.
    pub fn compute_domtree(&mut self) {
        self.domtree.compute(&self.func, &self.cfg);
        self.domtree_preorder.compute(&self.domtree);
    }

    /// Compute the loop analysis.
    pub fn compute_loop_analysis(&mut self) {
        self.loop_analysis
            .compute(&self.func, &self.cfg, &self.domtree)
    }

    /// Compute the control flow graph and dominator tree.
    pub fn flowgraph(&mut self) {
        self.compute_cfg();
        self.compute_domtree()
    }

    /// Perform unreachable code elimination.
    pub fn eliminate_unreachable_code<'a, FOI>(&mut self, fisa: FOI) -> CodegenResult<()>
    where
        FOI: Into<FlagsOrIsa<'a>>,
    {
        eliminate_unreachable_code(&mut self.func, &mut self.cfg, &self.domtree);
        self.verify_if(fisa)
    }

    /// Replace all redundant loads with the known values in
    /// memory. These are loads whose values were already loaded by
    /// other loads earlier, as well as loads whose values were stored
    /// by a store instruction to the same instruction (so-called
    /// "store-to-load forwarding").
    pub fn replace_redundant_loads(&mut self) -> CodegenResult<()> {
        let mut analysis = AliasAnalysis::new(&self.func, &self.domtree_preorder);
        analysis.compute_and_update_aliases(&mut self.func);
        Ok(())
    }

    /// Harvest candidate left-hand sides for superoptimization with Souper.
    #[cfg(feature = "souper-harvest")]
    pub fn souper_harvest(
        &mut self,
        out: &mut std::sync::mpsc::Sender<String>,
    ) -> CodegenResult<()> {
        do_souper_harvest(&self.func, out);
        Ok(())
    }

    /// Run optimizations via the egraph infrastructure.
    pub fn egraph_pass<'a, FOI>(
        &mut self,
        fisa: FOI,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<()>
    where
        FOI: Into<FlagsOrIsa<'a>>,
    {
        let _tt = timing::egraph();

        trace!(
            "About to optimize with egraph phase:\n{}",
            self.func.display()
        );
        let fisa = fisa.into();
        self.compute_loop_analysis();
        let mut alias_analysis = AliasAnalysis::new(&self.func, &self.domtree_preorder);
        let mut pass = EgraphPass::new(
            &mut self.func,
            &self.domtree,
            &self.loop_analysis,
            &mut alias_analysis,
            &fisa.flags,
            ctrl_plane,
        );
        pass.run();
        log::debug!("egraph stats: {:?}", pass.stats);
        trace!("After egraph optimization:\n{}", self.func.display());

        self.verify_if(fisa)
    }
}

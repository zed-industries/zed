//! ISLE integration glue code for Pulley lowering.

// Pull in the ISLE generated code.
pub mod generated_code;
use generated_code::MInst;
use inst::InstAndKind;

// Types that the generated ISLE code uses via `use super::*`.
use crate::ir::{condcodes::*, immediates::*, types::*, *};
use crate::isa::pulley_shared::{
    inst::{
        FReg, OperandSize, PulleyCall, ReturnCallInfo, VReg, WritableFReg, WritableVReg,
        WritableXReg, XReg,
    },
    lower::{Cond, regs},
    *,
};
use crate::machinst::{
    CallArgList, CallInfo, CallRetList, MachInst, Reg, VCodeConstant, VCodeConstantData,
    abi::{ArgPair, RetPair, StackAMode},
    isle::*,
};
use alloc::boxed::Box;
use pulley_interpreter::U6;
use regalloc2::PReg;
use smallvec::SmallVec;

type Unit = ();
type VecArgPair = Vec<ArgPair>;
type VecRetPair = Vec<RetPair>;
type BoxCallInfo = Box<CallInfo<PulleyCall>>;
type BoxCallIndInfo = Box<CallInfo<XReg>>;
type BoxCallIndirectHostInfo = Box<CallInfo<ExternalName>>;
type BoxReturnCallInfo = Box<ReturnCallInfo<ExternalName>>;
type BoxReturnCallIndInfo = Box<ReturnCallInfo<XReg>>;
type BoxExternalName = Box<ExternalName>;
type UpperXRegSet = pulley_interpreter::UpperRegSet<pulley_interpreter::XReg>;

#[expect(
    unused_imports,
    reason = "used on other backends, used here to suppress warning elsewhere"
)]
use crate::machinst::isle::UnwindInst as _;

pub(crate) struct PulleyIsleContext<'a, 'b, I, B>
where
    I: VCodeInst,
    B: LowerBackend,
{
    pub lower_ctx: &'a mut Lower<'b, I>,
    pub backend: &'a B,
}

impl<'a, 'b, P> PulleyIsleContext<'a, 'b, InstAndKind<P>, PulleyBackend<P>>
where
    P: PulleyTargetKind,
{
    fn new(lower_ctx: &'a mut Lower<'b, InstAndKind<P>>, backend: &'a PulleyBackend<P>) -> Self {
        Self { lower_ctx, backend }
    }

    pub(crate) fn dfg(&self) -> &crate::ir::DataFlowGraph {
        &self.lower_ctx.f.dfg
    }
}

impl<P> generated_code::Context for PulleyIsleContext<'_, '_, InstAndKind<P>, PulleyBackend<P>>
where
    P: PulleyTargetKind,
{
    crate::isle_lower_prelude_methods!(InstAndKind<P>);

    fn gen_call_info(
        &mut self,
        sig: Sig,
        name: ExternalName,
        mut uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
    ) -> BoxCallInfo {
        let stack_ret_space = self.lower_ctx.sigs()[sig].sized_stack_ret_space();
        let stack_arg_space = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

        // The first four integer arguments to a call can be handled via
        // special pulley call instructions. Assert here that
        // `uses` is sorted in order and then take out x0-x3 if
        // they're present and move them from `uses` to
        // `dest.args` to be handled differently during register
        // allocation.
        let mut args = SmallVec::new();
        uses.sort_by_key(|arg| arg.preg);
        uses.retain(|arg| {
            if arg.preg != regs::x0()
                && arg.preg != regs::x1()
                && arg.preg != regs::x2()
                && arg.preg != regs::x3()
            {
                return true;
            }
            args.push(XReg::new(arg.vreg).unwrap());
            false
        });
        let dest = PulleyCall { name, args };
        Box::new(
            self.lower_ctx
                .gen_call_info(sig, dest, uses, defs, try_call_info),
        )
    }

    fn gen_call_ind_info(
        &mut self,
        sig: Sig,
        dest: Reg,
        uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
    ) -> BoxCallIndInfo {
        let stack_ret_space = self.lower_ctx.sigs()[sig].sized_stack_ret_space();
        let stack_arg_space = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

        let dest = XReg::new(dest).unwrap();
        Box::new(
            self.lower_ctx
                .gen_call_info(sig, dest, uses, defs, try_call_info),
        )
    }

    fn gen_call_host_info(
        &mut self,
        sig: Sig,
        dest: ExternalName,
        uses: CallArgList,
        defs: CallRetList,
        try_call_info: Option<TryCallInfo>,
    ) -> BoxCallIndirectHostInfo {
        let stack_ret_space = self.lower_ctx.sigs()[sig].sized_stack_ret_space();
        let stack_arg_space = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

        Box::new(
            self.lower_ctx
                .gen_call_info(sig, dest, uses, defs, try_call_info),
        )
    }

    fn gen_return_call_info(
        &mut self,
        sig: Sig,
        dest: ExternalName,
        uses: CallArgList,
    ) -> BoxReturnCallInfo {
        let new_stack_arg_size = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_tail_args_size(new_stack_arg_size);

        Box::new(ReturnCallInfo {
            dest,
            uses,
            new_stack_arg_size,
        })
    }

    fn gen_return_call_ind_info(
        &mut self,
        sig: Sig,
        dest: Reg,
        uses: CallArgList,
    ) -> BoxReturnCallIndInfo {
        let new_stack_arg_size = self.lower_ctx.sigs()[sig].sized_stack_arg_space();
        self.lower_ctx
            .abi_mut()
            .accumulate_tail_args_size(new_stack_arg_size);

        Box::new(ReturnCallInfo {
            dest: XReg::new(dest).unwrap(),
            uses,
            new_stack_arg_size,
        })
    }

    fn vreg_new(&mut self, r: Reg) -> VReg {
        VReg::new(r).unwrap()
    }
    fn writable_vreg_new(&mut self, r: WritableReg) -> WritableVReg {
        r.map(|wr| VReg::new(wr).unwrap())
    }
    fn writable_vreg_to_vreg(&mut self, arg0: WritableVReg) -> VReg {
        arg0.to_reg()
    }
    fn writable_vreg_to_writable_reg(&mut self, arg0: WritableVReg) -> WritableReg {
        arg0.map(|vr| vr.to_reg())
    }
    fn vreg_to_reg(&mut self, arg0: VReg) -> Reg {
        *arg0
    }
    fn xreg_new(&mut self, r: Reg) -> XReg {
        XReg::new(r).unwrap()
    }
    fn writable_xreg_new(&mut self, r: WritableReg) -> WritableXReg {
        r.map(|wr| XReg::new(wr).unwrap())
    }
    fn writable_xreg_to_xreg(&mut self, arg0: WritableXReg) -> XReg {
        arg0.to_reg()
    }
    fn writable_xreg_to_writable_reg(&mut self, arg0: WritableXReg) -> WritableReg {
        arg0.map(|xr| xr.to_reg())
    }
    fn xreg_to_reg(&mut self, arg0: XReg) -> Reg {
        *arg0
    }
    fn freg_new(&mut self, r: Reg) -> FReg {
        FReg::new(r).unwrap()
    }
    fn writable_freg_new(&mut self, r: WritableReg) -> WritableFReg {
        r.map(|wr| FReg::new(wr).unwrap())
    }
    fn writable_freg_to_freg(&mut self, arg0: WritableFReg) -> FReg {
        arg0.to_reg()
    }
    fn writable_freg_to_writable_reg(&mut self, arg0: WritableFReg) -> WritableReg {
        arg0.map(|fr| fr.to_reg())
    }
    fn freg_to_reg(&mut self, arg0: FReg) -> Reg {
        *arg0
    }

    #[inline]
    fn emit(&mut self, arg0: &MInst) -> Unit {
        self.lower_ctx.emit(arg0.clone().into());
    }

    fn sp_reg(&mut self) -> XReg {
        XReg::new(regs::stack_reg()).unwrap()
    }

    fn cond_invert(&mut self, cond: &Cond) -> Cond {
        cond.invert()
    }

    fn u6_from_u8(&mut self, imm: u8) -> Option<U6> {
        U6::new(imm)
    }

    fn endianness(&mut self, flags: MemFlags) -> Endianness {
        flags.endianness(self.backend.isa_flags.endianness())
    }

    fn is_native_endianness(&mut self, endianness: &Endianness) -> bool {
        *endianness == self.backend.isa_flags.endianness()
    }

    fn pointer_width(&mut self) -> PointerWidth {
        P::pointer_width()
    }

    fn memflags_nontrapping(&mut self, flags: MemFlags) -> bool {
        flags.trap_code().is_none()
    }

    fn memflags_is_wasm(&mut self, flags: MemFlags) -> bool {
        flags.trap_code() == Some(TrapCode::HEAP_OUT_OF_BOUNDS)
            && self.endianness(flags) == Endianness::Little
    }

    fn g32_offset(
        &mut self,
        load_offset: i32,
        load_ty: Type,
        bound_check_offset: u64,
    ) -> Option<u16> {
        // NB: for more docs on this see the ISLE definition.
        let load_offset = u64::try_from(load_offset).ok()?;
        let load_bytes = u64::from(load_ty.bytes());
        if bound_check_offset != load_offset + load_bytes {
            return None;
        }
        u16::try_from(load_offset).ok()
    }
}

/// The main entry point for lowering with ISLE.
pub(crate) fn lower<P>(
    lower_ctx: &mut Lower<InstAndKind<P>>,
    backend: &PulleyBackend<P>,
    inst: Inst,
) -> Option<InstOutput>
where
    P: PulleyTargetKind,
{
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = PulleyIsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower(&mut isle_ctx, inst)
}

/// The main entry point for branch lowering with ISLE.
pub(crate) fn lower_branch<P>(
    lower_ctx: &mut Lower<InstAndKind<P>>,
    backend: &PulleyBackend<P>,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()>
where
    P: PulleyTargetKind,
{
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = PulleyIsleContext::new(lower_ctx, backend);
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}

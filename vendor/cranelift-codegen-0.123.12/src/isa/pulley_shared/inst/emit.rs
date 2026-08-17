//! Pulley binary code emission.

use super::*;
use crate::ir::{self, Endianness};
use crate::isa;
use crate::isa::pulley_shared::PointerWidth;
use crate::isa::pulley_shared::abi::PulleyMachineDeps;
use core::marker::PhantomData;
use cranelift_control::ControlPlane;
use pulley_interpreter::encode as enc;
use pulley_interpreter::regs::BinaryOperands;

pub struct EmitInfo {
    call_conv: isa::CallConv,
    shared_flags: settings::Flags,
    isa_flags: crate::isa::pulley_shared::settings::Flags,
}

impl EmitInfo {
    pub(crate) fn new(
        call_conv: isa::CallConv,
        shared_flags: settings::Flags,
        isa_flags: crate::isa::pulley_shared::settings::Flags,
    ) -> Self {
        Self {
            call_conv,
            shared_flags,
            isa_flags,
        }
    }

    fn endianness(&self, flags: MemFlags) -> Endianness {
        flags.endianness(self.isa_flags.endianness())
    }
}

/// State carried between emissions of a sequence of instructions.
#[derive(Default, Clone, Debug)]
pub struct EmitState<P>
where
    P: PulleyTargetKind,
{
    _phantom: PhantomData<P>,
    ctrl_plane: ControlPlane,
    user_stack_map: Option<ir::UserStackMap>,
    frame_layout: FrameLayout,
}

impl<P> EmitState<P>
where
    P: PulleyTargetKind,
{
    fn take_stack_map(&mut self) -> Option<ir::UserStackMap> {
        self.user_stack_map.take()
    }
}

impl<P> MachInstEmitState<InstAndKind<P>> for EmitState<P>
where
    P: PulleyTargetKind,
{
    fn new(abi: &Callee<PulleyMachineDeps<P>>, ctrl_plane: ControlPlane) -> Self {
        EmitState {
            _phantom: PhantomData,
            ctrl_plane,
            user_stack_map: None,
            frame_layout: abi.frame_layout().clone(),
        }
    }

    fn pre_safepoint(&mut self, user_stack_map: Option<ir::UserStackMap>) {
        self.user_stack_map = user_stack_map;
    }

    fn ctrl_plane_mut(&mut self) -> &mut ControlPlane {
        &mut self.ctrl_plane
    }

    fn take_ctrl_plane(self) -> ControlPlane {
        self.ctrl_plane
    }

    fn frame_layout(&self) -> &FrameLayout {
        &self.frame_layout
    }
}

impl<P> MachInstEmit for InstAndKind<P>
where
    P: PulleyTargetKind,
{
    type State = EmitState<P>;
    type Info = EmitInfo;

    fn emit(&self, sink: &mut MachBuffer<Self>, emit_info: &Self::Info, state: &mut Self::State) {
        // N.B.: we *must* not exceed the "worst-case size" used to compute
        // where to insert islands, except when islands are explicitly triggered
        // (with an `EmitIsland`). We check this in debug builds. This is `mut`
        // to allow disabling the check for `JTSequence`, which is always
        // emitted following an `EmitIsland`.
        let mut start = sink.cur_offset();
        pulley_emit(self, sink, emit_info, state, &mut start);

        let end = sink.cur_offset();
        assert!(
            (end - start) <= InstAndKind::<P>::worst_case_size(),
            "encoded inst {self:?} longer than worst-case size: length: {}, Inst::worst_case_size() = {}",
            end - start,
            InstAndKind::<P>::worst_case_size()
        );
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> String {
        self.print_with_state(state)
    }
}

fn pulley_emit<P>(
    inst: &Inst,
    sink: &mut MachBuffer<InstAndKind<P>>,
    emit_info: &EmitInfo,
    state: &mut EmitState<P>,
    start_offset: &mut u32,
) where
    P: PulleyTargetKind,
{
    match inst {
        // Pseduo-instructions that don't actually encode to anything.
        Inst::Args { .. } | Inst::Rets { .. } | Inst::DummyUse { .. } => {}

        Inst::TrapIf { cond, code } => {
            let trap = sink.defer_trap(*code);
            let not_trap = sink.get_label();

            <InstAndKind<P>>::from(Inst::BrIf {
                cond: cond.clone(),
                taken: trap,
                not_taken: not_trap,
            })
            .emit(sink, emit_info, state);
            sink.bind_label(not_trap, &mut state.ctrl_plane);
        }

        Inst::Nop => todo!(),

        Inst::GetSpecial { dst, reg } => enc::xmov(sink, dst, reg),

        Inst::LoadExtName { .. } => todo!(),

        Inst::Call { info } => {
            let offset = sink.cur_offset();

            // If arguments happen to already be in the right register for the
            // ABI then remove them from this list. Otherwise emit the
            // appropriate `Call` instruction depending on how many arguments we
            // have that aren't already in their correct register according to
            // ABI conventions.
            let mut args = &info.dest.args[..];
            while !args.is_empty() && args.last().copied() == XReg::new(x_reg(args.len() - 1)) {
                args = &args[..args.len() - 1];
            }
            match args {
                [] => enc::call(sink, 0),
                [x0] => enc::call1(sink, x0, 0),
                [x0, x1] => enc::call2(sink, x0, x1, 0),
                [x0, x1, x2] => enc::call3(sink, x0, x1, x2, 0),
                [x0, x1, x2, x3] => enc::call4(sink, x0, x1, x2, x3, 0),
                _ => unreachable!(),
            }
            let end = sink.cur_offset();
            sink.add_reloc_at_offset(
                end - 4,
                // TODO: is it actually okay to reuse this reloc here?
                Reloc::X86CallPCRel4,
                &info.dest.name,
                // This addend adjusts for the difference between the start of
                // the instruction and the beginning of the immediate offset
                // field which is always the final 4 bytes of the instruction.
                -i64::from(end - offset - 4),
            );
            if let Some(s) = state.take_stack_map() {
                let offset = sink.cur_offset();
                sink.push_user_stack_map(state, offset, s);
            }

            if let Some(try_call) = info.try_call_info.as_ref() {
                sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
            } else {
                sink.add_call_site();
            }

            let adjust = -i32::try_from(info.callee_pop_size).unwrap();
            for i in PulleyMachineDeps::<P>::gen_sp_reg_adjust(adjust) {
                i.emit(sink, emit_info, state);
            }

            // Load any stack-carried return values.
            info.emit_retval_loads::<PulleyMachineDeps<P>, _, _>(
                state.frame_layout().stackslots_size,
                |inst| inst.emit(sink, emit_info, state),
                |space_needed| Some(<InstAndKind<P>>::from(Inst::EmitIsland { space_needed })),
            );

            // If this is a try-call, jump to the continuation
            // (normal-return) block.
            if let Some(try_call) = info.try_call_info.as_ref() {
                let jmp = InstAndKind::<P>::from(Inst::Jump {
                    label: try_call.continuation,
                });
                jmp.emit(sink, emit_info, state);
            }

            // We produce an island above if needed, so disable
            // the worst-case-size check in this case.
            *start_offset = sink.cur_offset();
        }

        Inst::IndirectCall { info } => {
            enc::call_indirect(sink, info.dest);

            if let Some(s) = state.take_stack_map() {
                let offset = sink.cur_offset();
                sink.push_user_stack_map(state, offset, s);
            }

            if let Some(try_call) = info.try_call_info.as_ref() {
                sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
            } else {
                sink.add_call_site();
            }

            let adjust = -i32::try_from(info.callee_pop_size).unwrap();
            for i in PulleyMachineDeps::<P>::gen_sp_reg_adjust(adjust) {
                i.emit(sink, emit_info, state);
            }

            // Load any stack-carried return values.
            info.emit_retval_loads::<PulleyMachineDeps<P>, _, _>(
                state.frame_layout().stackslots_size,
                |inst| inst.emit(sink, emit_info, state),
                |space_needed| Some(<InstAndKind<P>>::from(Inst::EmitIsland { space_needed })),
            );

            // If this is a try-call, jump to the continuation
            // (normal-return) block.
            if let Some(try_call) = info.try_call_info.as_ref() {
                let jmp = InstAndKind::<P>::from(Inst::Jump {
                    label: try_call.continuation,
                });
                jmp.emit(sink, emit_info, state);
            }

            // We produce an island above if needed, so disable
            // the worst-case-size check in this case.
            *start_offset = sink.cur_offset();
        }

        Inst::ReturnCall { info } => {
            emit_return_call_common_sequence(sink, emit_info, state, &info);

            // Emit an unconditional jump which is quite similar to `Inst::Call`
            // except that a `jump` opcode is used instead of a `call` opcode.
            sink.put1(pulley_interpreter::Opcode::Jump as u8);
            sink.add_reloc(Reloc::X86CallPCRel4, &info.dest, -1);
            sink.put4(0);

            // Islands were manually handled in
            // `emit_return_call_common_sequence`.
            *start_offset = sink.cur_offset();
        }

        Inst::ReturnIndirectCall { info } => {
            emit_return_call_common_sequence(sink, emit_info, state, &info);
            enc::xjump(sink, info.dest);

            // Islands were manually handled in
            // `emit_return_call_common_sequence`.
            *start_offset = sink.cur_offset();
        }

        Inst::IndirectCallHost { info } => {
            // Emit a relocation to fill in the actual immediate argument here
            // in `call_indirect_host`.
            sink.add_reloc(Reloc::PulleyCallIndirectHost, &info.dest, 0);
            enc::call_indirect_host(sink, 0_u8);

            if let Some(s) = state.take_stack_map() {
                let offset = sink.cur_offset();
                sink.push_user_stack_map(state, offset, s);
            }

            if let Some(try_call) = info.try_call_info.as_ref() {
                sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
            } else {
                sink.add_call_site();
            }

            // If a callee pop is happening here that means that something has
            // messed up, these are expected to be "very simple" signatures.
            assert!(info.callee_pop_size == 0);
        }

        Inst::Jump { label } => {
            sink.use_label_at_offset(*start_offset + 1, *label, LabelUse::Jump(1));
            sink.add_uncond_branch(*start_offset, *start_offset + 5, *label);
            enc::jump(sink, 0x00000000);
        }

        Inst::BrIf {
            cond,
            taken,
            not_taken,
        } => {
            // Encode the inverted form of the branch. Branches always have
            // their trailing 4 bytes as the relative offset which is what we're
            // going to target here within the `MachBuffer`.
            let mut inverted = SmallVec::<[u8; 16]>::new();
            cond.invert().encode(&mut inverted);
            let len = inverted.len() as u32;
            debug_assert!(len > 4);

            // Use the `taken` label 4 bytes before the end of the instruction
            // we're about to emit as that's the base of `PcRelOffset`. Note
            // that the `Jump` here factors in the offset from the start of the
            // instruction to the start of the relative offset, hence `len - 4`
            // as the factor to adjust by.
            let taken_end = *start_offset + len;
            sink.use_label_at_offset(taken_end - 4, *taken, LabelUse::Jump(len - 4));
            sink.add_cond_branch(*start_offset, taken_end, *taken, &inverted);
            cond.encode(sink);
            debug_assert_eq!(sink.cur_offset(), taken_end);

            // For the not-taken branch use an unconditional jump to the
            // relevant label, and we know that the jump instruction is 5 bytes
            // long where the final 4 bytes are the offset to jump by.
            let not_taken_start = taken_end + 1;
            let not_taken_end = not_taken_start + 4;
            sink.use_label_at_offset(not_taken_start, *not_taken, LabelUse::Jump(1));
            sink.add_uncond_branch(taken_end, not_taken_end, *not_taken);
            enc::jump(sink, 0x00000000);
            assert_eq!(sink.cur_offset(), not_taken_end);
        }

        Inst::LoadAddr { dst, mem } => {
            let base = mem.get_base_register();
            let offset = mem.get_offset_with_state(state);

            if let Some(base) = base {
                if offset == 0 {
                    enc::xmov(sink, dst, base);
                } else {
                    if let Ok(offset) = i8::try_from(offset) {
                        enc::xconst8(sink, dst, offset);
                    } else if let Ok(offset) = i16::try_from(offset) {
                        enc::xconst16(sink, dst, offset);
                    } else {
                        enc::xconst32(sink, dst, offset);
                    }

                    match P::pointer_width() {
                        PointerWidth::PointerWidth32 => {
                            enc::xadd32(sink, BinaryOperands::new(dst, base, dst))
                        }
                        PointerWidth::PointerWidth64 => {
                            enc::xadd64(sink, BinaryOperands::new(dst, base, dst))
                        }
                    }
                }
            } else {
                unreachable!("all pulley amodes have a base register right now")
            }
        }

        Inst::XLoad {
            dst,
            mem,
            ty,
            flags,
        } => {
            use Endianness as E;
            assert!(flags.trap_code().is_none());
            let addr = AddrO32::Base {
                addr: mem.get_base_register().unwrap(),
                offset: mem.get_offset_with_state(state),
            };
            let endian = emit_info.endianness(*flags);
            match *ty {
                I8 => enc::xload8_u32_o32(sink, dst, addr),
                I16 => match endian {
                    E::Little => enc::xload16le_s32_o32(sink, dst, addr),
                    E::Big => enc::xload16be_s32_o32(sink, dst, addr),
                },
                I32 => match endian {
                    E::Little => enc::xload32le_o32(sink, dst, addr),
                    E::Big => enc::xload32be_o32(sink, dst, addr),
                },
                I64 => match endian {
                    E::Little => enc::xload64le_o32(sink, dst, addr),
                    E::Big => enc::xload64be_o32(sink, dst, addr),
                },
                _ => unimplemented!("xload ty={ty:?}"),
            }
        }

        Inst::FLoad {
            dst,
            mem,
            ty,
            flags,
        } => {
            use Endianness as E;
            assert!(flags.trap_code().is_none());
            let addr = AddrO32::Base {
                addr: mem.get_base_register().unwrap(),
                offset: mem.get_offset_with_state(state),
            };
            let endian = emit_info.endianness(*flags);
            match *ty {
                F32 => match endian {
                    E::Little => enc::fload32le_o32(sink, dst, addr),
                    E::Big => enc::fload32be_o32(sink, dst, addr),
                },
                F64 => match endian {
                    E::Little => enc::fload64le_o32(sink, dst, addr),
                    E::Big => enc::fload64be_o32(sink, dst, addr),
                },
                _ => unimplemented!("fload ty={ty:?}"),
            }
        }

        Inst::VLoad {
            dst,
            mem,
            ty,
            flags,
        } => {
            assert!(flags.trap_code().is_none());
            let addr = AddrO32::Base {
                addr: mem.get_base_register().unwrap(),
                offset: mem.get_offset_with_state(state),
            };
            let endian = emit_info.endianness(*flags);
            assert_eq!(endian, Endianness::Little);
            assert_eq!(ty.bytes(), 16);
            enc::vload128le_o32(sink, dst, addr);
        }

        Inst::XStore {
            mem,
            src,
            ty,
            flags,
        } => {
            use Endianness as E;
            assert!(flags.trap_code().is_none());
            let addr = AddrO32::Base {
                addr: mem.get_base_register().unwrap(),
                offset: mem.get_offset_with_state(state),
            };
            let endian = emit_info.endianness(*flags);
            match *ty {
                I8 => enc::xstore8_o32(sink, addr, src),
                I16 => match endian {
                    E::Little => enc::xstore16le_o32(sink, addr, src),
                    E::Big => enc::xstore16be_o32(sink, addr, src),
                },
                I32 => match endian {
                    E::Little => enc::xstore32le_o32(sink, addr, src),
                    E::Big => enc::xstore32be_o32(sink, addr, src),
                },
                I64 => match endian {
                    E::Little => enc::xstore64le_o32(sink, addr, src),
                    E::Big => enc::xstore64be_o32(sink, addr, src),
                },
                _ => unimplemented!("xstore ty={ty:?}"),
            }
        }

        Inst::FStore {
            mem,
            src,
            ty,
            flags,
        } => {
            use Endianness as E;
            assert!(flags.trap_code().is_none());
            let addr = AddrO32::Base {
                addr: mem.get_base_register().unwrap(),
                offset: mem.get_offset_with_state(state),
            };
            let endian = emit_info.endianness(*flags);
            match *ty {
                F32 => match endian {
                    E::Little => enc::fstore32le_o32(sink, addr, src),
                    E::Big => enc::fstore32be_o32(sink, addr, src),
                },
                F64 => match endian {
                    E::Little => enc::fstore64le_o32(sink, addr, src),
                    E::Big => enc::fstore64be_o32(sink, addr, src),
                },
                _ => unimplemented!("fstore ty={ty:?}"),
            }
        }

        Inst::VStore {
            mem,
            src,
            ty,
            flags,
        } => {
            assert!(flags.trap_code().is_none());
            let addr = AddrO32::Base {
                addr: mem.get_base_register().unwrap(),
                offset: mem.get_offset_with_state(state),
            };
            let endian = emit_info.endianness(*flags);
            assert_eq!(endian, Endianness::Little);
            assert_eq!(ty.bytes(), 16);
            enc::vstore128le_o32(sink, addr, src);
        }

        Inst::BrTable {
            idx,
            default,
            targets,
        } => {
            // Encode the `br_table32` instruction directly which expects the
            // next `amt` 4-byte integers to all be relative offsets. Each
            // offset is the pc-relative offset of the branch destination.
            //
            // Pulley clamps the branch targets to the `amt` specified so the
            // final branch target is the default jump target.
            //
            // Note that this instruction may have many branch targets so it
            // manually checks to see if an island is needed. If so we emit a
            // jump around the island before the `br_table32` itself gets
            // emitted.
            let amt = u32::try_from(targets.len() + 1).expect("too many branch targets");
            let br_table_size = amt * 4 + 6;
            if sink.island_needed(br_table_size) {
                let label = sink.get_label();
                <InstAndKind<P>>::from(Inst::Jump { label }).emit(sink, emit_info, state);
                sink.emit_island(br_table_size, &mut state.ctrl_plane);
                sink.bind_label(label, &mut state.ctrl_plane);
            }
            enc::br_table32(sink, *idx, amt);
            for target in targets.iter() {
                let offset = sink.cur_offset();
                sink.use_label_at_offset(offset, *target, LabelUse::Jump(0));
                sink.put4(0);
            }
            let offset = sink.cur_offset();
            sink.use_label_at_offset(offset, *default, LabelUse::Jump(0));
            sink.put4(0);

            // We manually handled `emit_island` above when dealing with
            // `island_needed` so update the starting offset to the current
            // offset so this instruction doesn't accidentally trigger
            // the assertion that we're always under worst-case-size.
            *start_offset = sink.cur_offset();
        }

        Inst::Raw { raw } => super::generated::emit(raw, sink),

        Inst::EmitIsland { space_needed } => {
            if sink.island_needed(*space_needed) {
                let label = sink.get_label();
                <InstAndKind<P>>::from(Inst::Jump { label }).emit(sink, emit_info, state);
                sink.emit_island(space_needed + 8, &mut state.ctrl_plane);
                sink.bind_label(label, &mut state.ctrl_plane);
            }
        }
    }
}

fn emit_return_call_common_sequence<T, P>(
    sink: &mut MachBuffer<InstAndKind<P>>,
    emit_info: &EmitInfo,
    state: &mut EmitState<P>,
    info: &ReturnCallInfo<T>,
) where
    P: PulleyTargetKind,
{
    // The return call sequence can potentially emit a lot of instructions, so
    // lets emit an island here if we need it.
    //
    // It is difficult to calculate exactly how many instructions are going to
    // be emitted, so we calculate it by emitting it into a disposable buffer,
    // and then checking how many instructions were actually emitted.
    let mut buffer = MachBuffer::new();
    let mut fake_emit_state = state.clone();

    return_call_emit_impl(&mut buffer, emit_info, &mut fake_emit_state, info);

    // Finalize the buffer and get the number of bytes emitted.
    let buffer = buffer.finish(&Default::default(), &mut Default::default());
    let length = buffer.data().len() as u32;

    // And now emit the island inline with this instruction.
    if sink.island_needed(length) {
        let jump_around_label = sink.get_label();
        <InstAndKind<P>>::gen_jump(jump_around_label).emit(sink, emit_info, state);
        sink.emit_island(length + 4, &mut state.ctrl_plane);
        sink.bind_label(jump_around_label, &mut state.ctrl_plane);
    }

    // Now that we're done, emit the *actual* return sequence.
    return_call_emit_impl(sink, emit_info, state, info);
}

/// This should not be called directly, Instead prefer to call [emit_return_call_common_sequence].
fn return_call_emit_impl<T, P>(
    sink: &mut MachBuffer<InstAndKind<P>>,
    emit_info: &EmitInfo,
    state: &mut EmitState<P>,
    info: &ReturnCallInfo<T>,
) where
    P: PulleyTargetKind,
{
    let epilogue = <PulleyMachineDeps<P>>::gen_epilogue_frame_restore(
        emit_info.call_conv,
        &emit_info.shared_flags,
        &emit_info.isa_flags,
        &state.frame_layout,
    );

    for inst in epilogue {
        inst.emit(sink, emit_info, state);
    }

    // Now that `sp` is restored to what it was on function entry it may need to
    // be adjusted if the stack arguments of our own function differ from the
    // stack arguments of the callee. Perform any necessary adjustment here.
    //
    // Note that this means that there's a brief window where stack arguments
    // might be below `sp` in the case that the callee has more stack arguments
    // than ourselves. That's in theory ok though as we're inventing the pulley
    // ABI and nothing like async signals are happening that we have to worry
    // about.
    let incoming_args_diff =
        i64::from(state.frame_layout().tail_args_size - info.new_stack_arg_size);

    if incoming_args_diff != 0 {
        let amt = i32::try_from(incoming_args_diff).unwrap();
        for inst in PulleyMachineDeps::<P>::gen_sp_reg_adjust(amt) {
            inst.emit(sink, emit_info, state);
        }
    }
}

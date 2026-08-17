use crate::runtime::vm::vmcontext::VMArrayCallNative;
use crate::runtime::vm::{
    StoreBox, TrapRegisters, TrapTest, VMContext, VMOpaqueContext, f32x4, f64x2, i8x16, tls,
};
use crate::{Engine, ValRaw};
use core::marker;
use core::ptr::NonNull;
use pulley_interpreter::interp::{DoneReason, RegType, TrapKind, Val, Vm, XRegVal};
use pulley_interpreter::{FReg, Reg, XReg};
use wasmtime_environ::{BuiltinFunctionIndex, HostCall, Trap};
use wasmtime_unwinder::Unwind;

/// Interpreter state stored within a `Store<T>`.
#[repr(transparent)]
pub struct Interpreter {
    /// Pulley VM state, stored in a `StoreBox<T>`.
    ///
    /// This representation has a dual purpose of (a) having a low overhead if
    /// pulley is disabled (just a null pointer) and (b) enabling safe access to
    /// the `Vm` in the face of recursive calls.
    ///
    /// For (b) that's the most tricky part of this, but the basic problem looks
    /// like:
    ///
    /// * The host initially executes some WebAssembly.
    /// * This acquires a `&mut Vm` and does some execution.
    /// * The WebAssembly then invokes the host.
    /// * This bottoms out in `CallIndirectHost` which means that we'll do a
    ///   dynamic dispatch to a function pointer in pulley registers.
    /// * The function we call gets unfettered access to `StoreContextMut<T>`
    /// * When the function returns our original `&mut Vm` pointer is
    ///   invalidated, so it has to be re-acquired.
    ///
    /// The usage of `StoreBox` here solves this conundrum by storing the
    /// `InterpreterRef` at-rest as a `NonNull<Vm>` as opposed to a `&mut Vm`.
    /// This is required to model how after a host call the VM state must be
    /// re-acquire from store state to re-assert that it has an exclusive
    /// borrow.
    ///
    /// This in turn models how VM state could be modified as part of the
    /// recursive function call, for example with another VM execution itself.
    ///
    /// Note that the safety of this all relies not only on correctly managing
    /// this pointer but it also requires that this pointer is never
    /// deallocated while an `InterpreterRef` is live. The `InterpreterRef` type
    /// carries a borrow of this type to ensure this isn't dropped
    /// independently, and then this file never overwrites this private field to
    /// otherwise guarantee this.
    pulley: StoreBox<Vm>,
}

impl Interpreter {
    /// Creates a new interpreter ready to interpret code.
    pub fn new(engine: &Engine) -> Interpreter {
        let ret = Interpreter {
            pulley: StoreBox::new(Vm::with_stack(engine.config().max_wasm_stack)),
        };
        engine.profiler().register_interpreter(&ret);
        ret
    }

    /// Returns the `InterpreterRef` structure which can be used to actually
    /// execute interpreted code.
    pub fn as_interpreter_ref(&mut self) -> InterpreterRef<'_> {
        InterpreterRef {
            vm: self.pulley.get(),
            _phantom: marker::PhantomData,
        }
    }

    pub fn pulley(&self) -> &Vm {
        unsafe { self.pulley.get().as_ref() }
    }

    /// Get an implementation of `Unwind` used to walk the Pulley stack.
    pub fn unwinder(&self) -> &'static dyn Unwind {
        &UnwindPulley
    }
}

/// Wrapper around `&mut pulley_interpreter::Vm` to enable compiling this to a
/// zero-sized structure when pulley is disabled at compile time.
#[repr(transparent)]
pub struct InterpreterRef<'a> {
    vm: NonNull<Vm>,
    _phantom: marker::PhantomData<&'a mut Vm>,
}

/// An implementation of stack-walking details specifically designed
/// for unwinding Pulley's runtime stack.
pub struct UnwindPulley;

unsafe impl Unwind for UnwindPulley {
    fn next_older_fp_from_fp_offset(&self) -> usize {
        0
    }
    fn next_older_sp_from_fp_offset(&self) -> usize {
        if cfg!(target_pointer_width = "32") {
            8
        } else {
            16
        }
    }
    unsafe fn get_next_older_pc_from_fp(&self, fp: usize) -> usize {
        // The calling convention always pushes the return pointer (aka the PC
        // of the next older frame) just before this frame.
        unsafe { *(fp as *mut usize).offset(1) }
    }
    fn assert_fp_is_aligned(&self, fp: usize) {
        let expected = if cfg!(target_pointer_width = "32") {
            8
        } else {
            16
        };
        assert_eq!(fp % expected, 0, "stack should always be aligned");
    }
}

/// Equivalent of a native platform's `jmp_buf` (sort of).
///
/// This structure ensures that all callee-save state in Pulley is saved at wasm
/// function boundaries. This handles the case for example where a function is
/// executed but it traps halfway through. The trap will unwind the Pulley stack
/// and reset it back to what it was when the function started. This means that
/// Pulley function prologues don't execute and callee-saved registers aren't
/// restored. This structure is used to restore all that state to as it was
/// when the function started.
///
/// Note that this is a blind copy of all callee-saved state which is kept in
/// sync with `pulley_shared/abi.rs` in Cranelift. This includes the upper 16
/// x-regs, the upper 16 f-regs, the frame pointer, and the link register. The
/// stack pointer is included in the upper 16 x-regs. This representation is
/// explicitly chosen over an alternative such as only saving a bare minimum
/// amount of state and using function ABIs to auto-save registers. For example
/// we could, in Cranelift, indicate that the Pulley-to-host function call
/// clobbered all registers forcing the function prologue to save all
/// xregs/fregs. This means though that every wasm->host call would save/restore
/// all this state, even when a trap didn't happen. Alternatively this structure
/// being large means that the state is only saved once per host->wasm call
/// instead which is currently what's being optimized for.
///
/// If saving this structure is a performance hot spot in the future it might be
/// worth reevaluating this decision or perhaps shrinking the register file of
/// Pulley so less state need be saved.
#[derive(Clone, Copy)]
struct Setjmp {
    xregs: [u64; 16],
    fregs: [f64; 16],
    fp: *mut u8,
    lr: *mut u8,
}

impl InterpreterRef<'_> {
    fn vm(&mut self) -> &mut Vm {
        // SAFETY: This is a bit of a tricky code. The safety here is isolated
        // to this file, but not isolated to just this function call.
        //
        // An `InterpreterRef` guarantees that we have a pointer to a `Vm`, and
        // that pointer originates from a `StoreBox<VM>` in the store itself.
        // One level of safety here relies on that never being deallocated or
        // overwritten, which this file upholds as it's a private field only
        // this module can access.
        //
        // Another aspect upheld by `InterpreterRef` is that it transfers, to
        // the compiler, a mutable borrow of the store (e.g `struct Interpreter`
        // above) to this reference. While this doesn't actually hold such a
        // lifetime-bound pointer it guarantees that only one of these can be
        // active at a time per interpreter.
        //
        // Finally the lifetime of the returned `Vm` is bound to `self` which
        // ensures that there is at most one per `InterpreterRef`.
        //
        // All put together this should allow at most one `&mut Vm` per-store,
        // which is one guarantee we need for this to be safe.
        //
        // Otherwise this is then done to represent how across host function
        // calls the interpreter needs to be re-borrowed as the state may have
        // changed as part of the dynamic host call.
        unsafe { self.vm.as_mut() }
    }

    /// Invokes interpreted code.
    ///
    /// The `bytecode` pointer should previously have been produced by Cranelift
    /// and `callee` / `caller` / `args_and_results` are normal array-call
    /// arguments being passed around.
    pub unsafe fn call(
        mut self,
        mut bytecode: NonNull<u8>,
        callee: NonNull<VMOpaqueContext>,
        caller: NonNull<VMContext>,
        args_and_results: NonNull<[ValRaw]>,
    ) -> bool {
        // Initialize argument registers with the ABI arguments.
        let args = [
            XRegVal::new_ptr(callee.as_ptr()).into(),
            XRegVal::new_ptr(caller.as_ptr()).into(),
            XRegVal::new_ptr(args_and_results.cast::<u8>().as_ptr()).into(),
            XRegVal::new_u64(args_and_results.len() as u64).into(),
        ];

        let mut vm = self.vm();

        // Fake a "poor man's setjmp" for now by saving some critical context to
        // get restored when a trap happens. This pseudo-implements the stack
        // unwinding necessary for a trap.
        //
        // See more comments in `trap` below about how this isn't actually
        // correct as it's not saving all callee-save state.
        let setjmp = setjmp(vm);

        let old_lr = unsafe { vm.call_start(&args) };

        // Run the interpreter as much as possible until it finishes, and then
        // handle each finish condition differently.
        let ret = loop {
            match unsafe { vm.call_run(bytecode) } {
                // If the VM returned entirely then read the return value and
                // return that (it indicates whether a trap happened or not.
                DoneReason::ReturnToHost(()) => {
                    match unsafe { vm.call_end(old_lr, [RegType::XReg]).next().unwrap() } {
                        #[allow(
                            clippy::cast_possible_truncation,
                            reason = "intentionally reading the lower bits only"
                        )]
                        Val::XReg(xreg) => break (xreg.get_u32() as u8) != 0,
                        _ => unreachable!(),
                    }
                }
                // If the VM wants to call out to the host then dispatch that
                // here based on `sig`. Once that returns we can resume
                // execution at `resume`.
                //
                // Note that the `raise` libcall is handled specially here since
                // longjmp/setjmp is handled differently than on the host.
                DoneReason::CallIndirectHost { id, resume } => {
                    if u32::from(id) == HostCall::Builtin(BuiltinFunctionIndex::raise()).index() {
                        longjmp(vm, setjmp);
                        break false;
                    } else {
                        vm = unsafe { self.call_indirect_host(id) };
                        bytecode = resume;
                    }
                }
                // If the VM trapped then process that here and return `false`.
                DoneReason::Trap { pc, kind } => {
                    trap(vm, pc, kind, setjmp);
                    break false;
                }
            }
        };

        if cfg!(debug_assertions) {
            for (i, reg) in callee_save_xregs() {
                assert!(vm[reg].get_u64() == setjmp.xregs[i]);
            }
            for (i, reg) in callee_save_fregs() {
                assert!(vm[reg].get_f64().to_bits() == setjmp.fregs[i].to_bits());
            }
            assert!(vm.fp() == setjmp.fp);
            assert!(vm.lr() == setjmp.lr);
        }
        ret
    }

    /// Handles the `call_indirect_host` instruction, dispatching the `sig`
    /// number here which corresponds to `wasmtime_environ::HostCall`.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        unused,
        reason = "macro-generated code"
    )]
    #[cfg_attr(
        not(feature = "component-model"),
        expect(unused_macro_rules, reason = "macro-code")
    )]
    unsafe fn call_indirect_host(&mut self, id: u8) -> &mut Vm {
        let id = u32::from(id);
        let fnptr = self.vm()[XReg::x0].get_ptr();
        let mut arg_reg = 1;

        /// Helper macro to invoke a builtin.
        ///
        /// Used as:
        ///
        /// `call(@builtin(ty1, ty2, ...) -> retty)` - invoke a core or
        /// component builtin with the macro-defined signature.
        ///
        /// `call(@host Ty(ty1, ty2, ...) -> retty)` - invoke a host function
        /// with the type `Ty`. The other types in the macro are checked by
        /// rustc to match the actual `Ty` definition in Rust.
        macro_rules! call {
            (@builtin($($param:ident),*) $(-> $result:ident)?) => {{
                #[allow(improper_ctypes_definitions, reason = "__m128i known not FFI-safe")]
                type T = unsafe extern "C" fn($(call!(@ty $param)),*) $(-> call!(@ty $result))?;
                call!(@host T($($param),*) $(-> $result)?);
            }};
            (@host $ty:ident($($param:ident),*) $(-> $result:ident)?) => {{

                // Decode each argument according to this macro, pulling
                // arguments from successive registers.
                let ret = unsafe {
                    let mut vm = self.vm();
                    // Convert the pointer from pulley to a native function pointer.
                    union GetNative {
                        fnptr: *mut u8,
                        host: $ty,
                    }
                    let host = GetNative { fnptr }.host;
                    host($({
                        let reg = XReg::new(arg_reg).unwrap();
                        arg_reg += 1;
                        call!(@get $param vm[reg])
                    }),*)
                };
                let _ = arg_reg; // silence last dead arg_reg increment warning

                let vm = self.vm();

                // Store the return value, if one is here, in x0.
                $(
                    call!(@set $result ret => vm[XReg::x0]);
                )?
                let _ = ret; // silence warning if no return value

                // Return from the outer `call_indirect_host` host function as
                // it's been processed.
                return vm;
            }};

            // Conversion from macro-defined types to Rust host types.
            (@ty bool) => (bool);
            (@ty u8) => (u8);
            (@ty u32) => (u32);
            (@ty i32) => (i32);
            (@ty u64) => (u64);
            (@ty i64) => (i64);
            (@ty f32) => (f32);
            (@ty f64) => (f64);
            (@ty i8x16) => (i8x16);
            (@ty f32x4) => (f32x4);
            (@ty f64x2) => (f64x2);
            (@ty vmctx) => (*mut VMContext);
            (@ty pointer) => (*mut u8);
            (@ty ptr_u8) => (*mut u8);
            (@ty ptr_u16) => (*mut u16);
            (@ty ptr_size) => (*mut usize);
            (@ty size) => (usize);

            // Conversion from a pulley register value to the macro-defined
            // type.
            (@get u8 $reg:expr) => ($reg.get_i32() as u8);
            (@get u32 $reg:expr) => ($reg.get_u32());
            (@get u64 $reg:expr) => ($reg.get_u64());
            (@get f32 $reg:expr) => (unreachable::<f32, _>($reg));
            (@get f64 $reg:expr) => (unreachable::<f64, _>($reg));
            (@get i8x16 $reg:expr) => (unreachable::<i8x16, _>($reg));
            (@get f32x4 $reg:expr) => (unreachable::<f32x4, _>($reg));
            (@get f64x2 $reg:expr) => (unreachable::<f64x2, _>($reg));
            (@get vmctx $reg:expr) => ($reg.get_ptr());
            (@get pointer $reg:expr) => ($reg.get_ptr());
            (@get ptr $reg:expr) => ($reg.get_ptr());
            (@get nonnull $reg:expr) => (NonNull::new($reg.get_ptr()).unwrap());
            (@get ptr_u8 $reg:expr) => ($reg.get_ptr());
            (@get ptr_u16 $reg:expr) => ($reg.get_ptr());
            (@get ptr_size $reg:expr) => ($reg.get_ptr());
            (@get size $reg:expr) => ($reg.get_ptr::<u8>() as usize);

            // Conversion from a Rust value back into a macro-defined type,
            // stored in a pulley register.
            (@set bool $src:expr => $dst:expr) => ($dst.set_i32(i32::from($src)));
            (@set u32 $src:expr => $dst:expr) => ($dst.set_u32($src));
            (@set u64 $src:expr => $dst:expr) => ($dst.set_u64($src));
            (@set f32 $src:expr => $dst:expr) => (unreachable::<f32, _>(($dst, $src)));
            (@set f64 $src:expr => $dst:expr) => (unreachable::<f64, _>(($dst, $src)));
            (@set i8x16 $src:expr => $dst:expr) => (unreachable::<i8x16, _>(($dst, $src)));
            (@set f32x4 $src:expr => $dst:expr) => (unreachable::<f32x4, _>(($dst, $src)));
            (@set f64x2 $src:expr => $dst:expr) => (unreachable::<f64x2, _>(($dst, $src)));
            (@set pointer $src:expr => $dst:expr) => ($dst.set_ptr($src));
            (@set size $src:expr => $dst:expr) => ($dst.set_ptr($src as *mut u8));
        }

        // With the helper macro above structure this into:
        //
        // foreach [core, component]
        //   * dispatch the call-the-host function pointer type
        //   * dispatch all builtins by their index.
        //
        // The hope is that this is relatively easy for LLVM to optimize since
        // it's a bunch of:
        //
        //  if id == 0 { ...;  return; }
        //  if id == 1 { ...;  return; }
        //  if id == 2 { ...;  return; }
        //  ...
        //

        if id == const { HostCall::ArrayCall.index() } {
            call!(@host VMArrayCallNative(nonnull, nonnull, nonnull, size) -> bool);
        }

        macro_rules! core {
            (
                $(
                    $( #[cfg($attr:meta)] )?
                    $name:ident($($pname:ident: $param:ident ),* ) $(-> $result:ident)?;
                )*
            ) => {
                $(
                    $( #[cfg($attr)] )?
                    if id == const { HostCall::Builtin(BuiltinFunctionIndex::$name()).index() } {
                        call!(@builtin($($param),*) $(-> $result)?);
                    }
                )*
            }
        }
        wasmtime_environ::foreach_builtin_function!(core);

        #[cfg(feature = "component-model")]
        {
            use crate::runtime::vm::component::VMLoweringCallee;
            use wasmtime_environ::component::ComponentBuiltinFunctionIndex;

            if id == const { HostCall::ComponentLowerImport.index() } {
                call!(@host VMLoweringCallee(nonnull, nonnull, u32, u32, nonnull, size) -> bool);
            }

            macro_rules! component {
                (
                    $(
                        $( #[cfg($attr:meta)] )?
                        $name:ident($($pname:ident: $param:ident ),* ) $(-> $result:ident)?;
                    )*
                ) => {
                    $(
                        $( #[cfg($attr)] )?
                        if id == const { HostCall::ComponentBuiltin(ComponentBuiltinFunctionIndex::$name()).index() } {
                            call!(@builtin($($param),*) $(-> $result)?);
                        }
                    )*
                }
            }
            wasmtime_environ::foreach_builtin_component_function!(component);
        }

        // if we got this far then something has gone seriously wrong.
        return unreachable(());

        fn unreachable<T, U>(_: U) -> T {
            unreachable!()
        }
    }
}

/// Handles an interpreter trap. This will initialize the trap state stored
/// in TLS via the `test_if_trap` helper below by reading the pc/fp of the
/// interpreter and seeing if that's a valid opcode to trap at.
fn trap(vm: &mut Vm, pc: NonNull<u8>, kind: Option<TrapKind>, setjmp: Setjmp) {
    let regs = TrapRegisters {
        pc: pc.as_ptr() as usize,
        fp: vm.fp() as usize,
    };
    tls::with(|s| {
        let s = s.unwrap();
        match kind {
            Some(kind) => {
                let trap = match kind {
                    TrapKind::IntegerOverflow => Trap::IntegerOverflow,
                    TrapKind::DivideByZero => Trap::IntegerDivisionByZero,
                    TrapKind::BadConversionToInteger => Trap::BadConversionToInteger,
                    TrapKind::MemoryOutOfBounds => Trap::MemoryOutOfBounds,
                    TrapKind::DisabledOpcode => Trap::DisabledOpcode,
                    TrapKind::StackOverflow => Trap::StackOverflow,
                };
                s.set_jit_trap(regs, None, trap);
            }
            None => {
                match s.test_if_trap(regs, None, |_| false) {
                    // This shouldn't be possible, so this is a fatal error
                    // if it happens.
                    TrapTest::NotWasm => {
                        panic!("pulley trap at {pc:?} without trap code registered")
                    }

                    // Not possible with our closure above returning `false`.
                    #[cfg(has_host_compiler_backend)]
                    TrapTest::HandledByEmbedder => unreachable!(),

                    // Trap was handled, yay! We don't use `jmp_buf`.
                    TrapTest::Trap { .. } => {}
                }
            }
        }
    });

    longjmp(vm, setjmp);
}

fn setjmp(vm: &Vm) -> Setjmp {
    let mut xregs = [0; 16];
    let mut fregs = [0.0; 16];
    for (i, reg) in callee_save_xregs() {
        xregs[i] = vm[reg].get_u64();
    }
    for (i, reg) in callee_save_fregs() {
        fregs[i] = vm[reg].get_f64();
    }
    Setjmp {
        xregs,
        fregs,
        fp: vm.fp(),
        lr: vm.lr(),
    }
}

/// Perform a "longjmp" by restoring the "setjmp" context saved when this
/// started.
fn longjmp(vm: &mut Vm, setjmp: Setjmp) {
    let Setjmp {
        xregs,
        fregs,
        fp,
        lr,
    } = setjmp;
    unsafe {
        for (i, reg) in callee_save_xregs() {
            vm[reg].set_u64(xregs[i]);
        }
        for (i, reg) in callee_save_fregs() {
            vm[reg].set_f64(fregs[i]);
        }
        vm.set_fp(fp);
        vm.set_lr(lr);
    }
}

fn callee_save_xregs() -> impl Iterator<Item = (usize, XReg)> {
    (0..16).map(|i| (i.into(), XReg::new(i + 16).unwrap()))
}

fn callee_save_fregs() -> impl Iterator<Item = (usize, FReg)> {
    (0..16).map(|i| (i.into(), FReg::new(i + 16).unwrap()))
}

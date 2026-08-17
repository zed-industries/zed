//! Stubs for when pulley is disabled at compile time.
//!
//! Note that this is structured so that these structures are all zero-sized and
//! `Option<Thing>` is also zero-sized so there should be no runtime cost for
//! having these structures plumbed around.

use crate::runtime::Uninhabited;
use crate::runtime::vm::{VMContext, VMOpaqueContext};
use crate::{Engine, ValRaw};
use core::marker;
use core::mem;
use core::ptr::NonNull;
use wasmtime_unwinder::Unwind;

pub struct Interpreter {
    empty: Uninhabited,
}

const _: () = assert!(mem::size_of::<Interpreter>() == 0);
const _: () = assert!(mem::size_of::<Option<Interpreter>>() == 0);

impl Interpreter {
    pub fn new(_engine: &Engine) -> Interpreter {
        unreachable!()
    }

    pub fn as_interpreter_ref(&mut self) -> InterpreterRef<'_> {
        match self.empty {}
    }

    pub fn unwinder(&self) -> &'static dyn Unwind {
        match self.empty {}
    }
}

pub struct InterpreterRef<'a> {
    empty: Uninhabited,
    _marker: marker::PhantomData<&'a mut Interpreter>,
}

const _: () = assert!(mem::size_of::<InterpreterRef<'_>>() == 0);
const _: () = assert!(mem::size_of::<Option<InterpreterRef<'_>>>() == 0);

impl InterpreterRef<'_> {
    pub unsafe fn call(
        self,
        _bytecode: NonNull<u8>,
        _callee: NonNull<VMOpaqueContext>,
        _caller: NonNull<VMContext>,
        _args_and_results: NonNull<[ValRaw]>,
    ) -> bool {
        match self.empty {}
    }
}

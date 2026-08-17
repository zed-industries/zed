use crate::{codegen::CodeGenError, isa::reg::Reg, masm::StackSlot};
use anyhow::{Result, anyhow};
use smallvec::SmallVec;
use wasmparser::{Ieee32, Ieee64};
use wasmtime_environ::WasmValType;

/// A typed register value used to track register values in the value
/// stack.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct TypedReg {
    /// The physical register.
    pub reg: Reg,
    /// The type associated to the physical register.
    pub ty: WasmValType,
}

impl TypedReg {
    /// Create a new [`TypedReg`].
    pub fn new(ty: WasmValType, reg: Reg) -> Self {
        Self { ty, reg }
    }

    /// Create an i64 [`TypedReg`].
    pub fn i64(reg: Reg) -> Self {
        Self {
            ty: WasmValType::I64,
            reg,
        }
    }

    /// Create an i32 [`TypedReg`].
    pub fn i32(reg: Reg) -> Self {
        Self {
            ty: WasmValType::I32,
            reg,
        }
    }

    /// Create an f64 [`TypedReg`].
    pub fn f64(reg: Reg) -> Self {
        Self {
            ty: WasmValType::F64,
            reg,
        }
    }

    /// Create an f32 [`TypedReg`].
    pub fn f32(reg: Reg) -> Self {
        Self {
            ty: WasmValType::F32,
            reg,
        }
    }

    /// Create a v128 [`TypedReg`].
    pub fn v128(reg: Reg) -> Self {
        Self {
            ty: WasmValType::V128,
            reg,
        }
    }
}

impl From<TypedReg> for Reg {
    fn from(tr: TypedReg) -> Self {
        tr.reg
    }
}

/// A local value.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Local {
    /// The index of the local.
    pub index: u32,
    /// The type of the local.
    pub ty: WasmValType,
}

/// A memory value.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Memory {
    /// The type associated with the memory offset.
    pub ty: WasmValType,
    /// The stack slot corresponding to the memory value.
    pub slot: StackSlot,
}

/// Value definition to be used within the shadow stack.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Val {
    /// I32 Constant.
    I32(i32),
    /// I64 Constant.
    I64(i64),
    /// F32 Constant.
    F32(Ieee32),
    /// F64 Constant.
    F64(Ieee64),
    /// V128 Constant.
    V128(i128),
    /// A register value.
    Reg(TypedReg),
    /// A local slot.
    Local(Local),
    /// Offset to a memory location.
    Memory(Memory),
}

impl From<TypedReg> for Val {
    fn from(tr: TypedReg) -> Self {
        Val::Reg(tr)
    }
}

impl From<Local> for Val {
    fn from(local: Local) -> Self {
        Val::Local(local)
    }
}

impl From<Memory> for Val {
    fn from(mem: Memory) -> Self {
        Val::Memory(mem)
    }
}

impl TryFrom<u32> for Val {
    type Error = anyhow::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        i32::try_from(value).map(Val::i32).map_err(Into::into)
    }
}

impl Val {
    /// Create a new I32 constant value.
    pub fn i32(v: i32) -> Self {
        Self::I32(v)
    }

    /// Create a new I64 constant value.
    pub fn i64(v: i64) -> Self {
        Self::I64(v)
    }

    /// Create a new F32 constant value.
    pub fn f32(v: Ieee32) -> Self {
        Self::F32(v)
    }

    pub fn f64(v: Ieee64) -> Self {
        Self::F64(v)
    }

    /// Create a new V128 constant value.
    pub fn v128(v: i128) -> Self {
        Self::V128(v)
    }

    /// Create a new Reg value.
    pub fn reg(reg: Reg, ty: WasmValType) -> Self {
        Self::Reg(TypedReg { reg, ty })
    }

    /// Create a new Local value.
    pub fn local(index: u32, ty: WasmValType) -> Self {
        Self::Local(Local { index, ty })
    }

    /// Create a Memory value.
    pub fn mem(ty: WasmValType, slot: StackSlot) -> Self {
        Self::Memory(Memory { ty, slot })
    }

    /// Check whether the value is a register.
    pub fn is_reg(&self) -> bool {
        match *self {
            Self::Reg(_) => true,
            _ => false,
        }
    }

    /// Check whether the value is a memory offset.
    pub fn is_mem(&self) -> bool {
        match *self {
            Self::Memory(_) => true,
            _ => false,
        }
    }

    /// Check whether the value is a constant.
    pub fn is_const(&self) -> bool {
        match *self {
            Val::I32(_) | Val::I64(_) | Val::F32(_) | Val::F64(_) | Val::V128(_) => true,
            _ => false,
        }
    }

    /// Check whether the value is local with a particular index.
    pub fn is_local_at_index(&self, index: u32) -> bool {
        match *self {
            Self::Local(Local { index: i, .. }) if i == index => true,
            _ => false,
        }
    }

    /// Get the register representation of the value.
    ///
    /// # Panics
    /// This method will panic if the value is not a register.
    pub fn unwrap_reg(&self) -> TypedReg {
        match self {
            Self::Reg(tr) => *tr,
            v => panic!("expected value {v:?} to be a register"),
        }
    }

    /// Get the integer representation of the value.
    ///
    /// # Panics
    /// This method will panic if the value is not an i32.
    pub fn unwrap_i32(&self) -> i32 {
        match self {
            Self::I32(v) => *v,
            v => panic!("expected value {v:?} to be i32"),
        }
    }

    /// Get the integer representation of the value.
    ///
    /// # Panics
    /// This method will panic if the value is not an i64.
    pub fn unwrap_i64(&self) -> i64 {
        match self {
            Self::I64(v) => *v,
            v => panic!("expected value {v:?} to be i64"),
        }
    }

    /// Get the float representation of the value.
    ///
    /// # Panics
    /// This method will panic if the value is not an f32.
    pub fn unwrap_f32(&self) -> Ieee32 {
        match self {
            Self::F32(v) => *v,
            v => panic!("expected value {v:?} to be f32"),
        }
    }

    /// Get the float representation of the value.
    ///
    /// # Panics
    /// This method will panic if the value is not an f64.
    pub fn unwrap_f64(&self) -> Ieee64 {
        match self {
            Self::F64(v) => *v,
            v => panic!("expected value {v:?} to be f64"),
        }
    }

    /// Returns the underlying memory value if it is one, panics otherwise.
    pub fn unwrap_mem(&self) -> Memory {
        match self {
            Self::Memory(m) => *m,
            v => panic!("expected value {v:?} to be a Memory"),
        }
    }

    /// Check whether the value is an i32 constant.
    pub fn is_i32_const(&self) -> bool {
        match *self {
            Self::I32(_) => true,
            _ => false,
        }
    }

    /// Check whether the value is an i64 constant.
    pub fn is_i64_const(&self) -> bool {
        match *self {
            Self::I64(_) => true,
            _ => false,
        }
    }

    /// Check whether the value is an f32 constant.
    pub fn is_f32_const(&self) -> bool {
        match *self {
            Self::F32(_) => true,
            _ => false,
        }
    }

    /// Check whether the value is an f64 constant.
    pub fn is_f64_const(&self) -> bool {
        match *self {
            Self::F64(_) => true,
            _ => false,
        }
    }

    /// Get the type of the value.
    pub fn ty(&self) -> WasmValType {
        match self {
            Val::I32(_) => WasmValType::I32,
            Val::I64(_) => WasmValType::I64,
            Val::F32(_) => WasmValType::F32,
            Val::F64(_) => WasmValType::F64,
            Val::V128(_) => WasmValType::V128,
            Val::Reg(r) => r.ty,
            Val::Memory(m) => m.ty,
            Val::Local(l) => l.ty,
        }
    }
}

/// The shadow stack used for compilation.
#[derive(Default, Debug)]
pub(crate) struct Stack {
    // NB: The 64 is chosen arbitrarily. We can adjust as we see fit.
    inner: SmallVec<[Val; 64]>,
}

impl Stack {
    /// Allocate a new stack.
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    /// Ensures that there are at least `n` elements in the value stack,
    /// and returns the index calculated by: stack length minus `n`.
    pub fn ensure_index_at(&self, n: usize) -> Result<usize> {
        if self.len() >= n {
            Ok(self.len() - n)
        } else {
            Err(anyhow!(CodeGenError::missing_values_in_stack()))
        }
    }

    /// Returns true if the stack contains a local with the provided index
    /// except if the only time the local appears is the top element.
    pub fn contains_latent_local(&self, index: u32) -> bool {
        self.inner
            .iter()
            // Iterate top-to-bottom so we can skip the top element and stop
            // when we see a memory element.
            .rev()
            // The local is not latent if it's the top element because the top
            // element will be popped next which materializes the local.
            .skip(1)
            // Stop when we see a memory element because that marks where we
            // spilled up to so there will not be any locals past this point.
            .take_while(|v| !v.is_mem())
            .any(|v| v.is_local_at_index(index))
    }

    /// Extend the stack with the given elements.
    pub fn extend(&mut self, values: impl IntoIterator<Item = Val>) {
        self.inner.extend(values);
    }

    /// Inserts many values at the given index.
    pub fn insert_many(&mut self, at: usize, values: &[Val]) {
        debug_assert!(at <= self.len());

        if at == self.len() {
            self.inner.extend_from_slice(values);
        } else {
            self.inner.insert_from_slice(at, values);
        }
    }

    /// Get the length of the stack.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Push a value to the stack.
    pub fn push(&mut self, val: Val) {
        self.inner.push(val);
    }

    /// Peek into the top in the stack.
    pub fn peek(&self) -> Option<&Val> {
        self.inner.last()
    }

    /// Returns an iterator referencing the last n items of the stack,
    /// in bottom-most to top-most order.
    pub fn peekn(&self, n: usize) -> impl Iterator<Item = &Val> + '_ {
        let len = self.len();
        assert!(n <= len);

        let partition = len - n;
        self.inner[partition..].into_iter()
    }

    /// Pops the top element of the stack, if any.
    pub fn pop(&mut self) -> Option<Val> {
        self.inner.pop()
    }

    /// Pops the element at the top of the stack if it is an i32 const;
    /// returns `None` otherwise.
    pub fn pop_i32_const(&mut self) -> Option<i32> {
        match self.peek() {
            Some(v) => v.is_i32_const().then(|| self.pop().unwrap().unwrap_i32()),
            _ => None,
        }
    }

    /// Pops the element at the top of the stack if it is an i64 const;
    /// returns `None` otherwise.
    pub fn pop_i64_const(&mut self) -> Option<i64> {
        match self.peek() {
            Some(v) => v.is_i64_const().then(|| self.pop().unwrap().unwrap_i64()),
            _ => None,
        }
    }

    /// Pops the element at the top of the stack if it is an f32 const;
    /// returns `None` otherwise.
    pub fn pop_f32_const(&mut self) -> Option<Ieee32> {
        match self.peek() {
            Some(v) => v.is_f32_const().then(|| self.pop().unwrap().unwrap_f32()),
            _ => None,
        }
    }

    /// Pops the element at the top of the stack if it is an f64 const;
    /// returns `None` otherwise.
    pub fn pop_f64_const(&mut self) -> Option<Ieee64> {
        match self.peek() {
            Some(v) => v.is_f64_const().then(|| self.pop().unwrap().unwrap_f64()),
            _ => None,
        }
    }

    /// Pops the element at the top of the stack if it is a register;
    /// returns `None` otherwise.
    pub fn pop_reg(&mut self) -> Option<TypedReg> {
        match self.peek() {
            Some(v) => v.is_reg().then(|| self.pop().unwrap().unwrap_reg()),
            _ => None,
        }
    }

    /// Pops the given register if it is at the top of the stack;
    /// returns `None` otherwise.
    pub fn pop_named_reg(&mut self, reg: Reg) -> Option<TypedReg> {
        match self.peek() {
            Some(v) => {
                (v.is_reg() && v.unwrap_reg().reg == reg).then(|| self.pop().unwrap().unwrap_reg())
            }
            _ => None,
        }
    }

    /// Get a mutable reference to the inner stack representation.
    pub fn inner_mut(&mut self) -> &mut SmallVec<[Val; 64]> {
        &mut self.inner
    }

    /// Get a reference to the inner stack representation.
    pub fn inner(&self) -> &SmallVec<[Val; 64]> {
        &self.inner
    }

    /// Calculates the size of, in bytes, of the top n [Memory] entries
    /// in the value stack.
    pub fn sizeof(&self, top: usize) -> u32 {
        self.peekn(top).fold(0, |acc, v| {
            if v.is_mem() {
                acc + v.unwrap_mem().slot.size
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Stack, Val};
    use crate::isa::reg::Reg;
    use wasmtime_environ::WasmValType;

    #[test]
    fn test_pop_i32_const() {
        let mut stack = Stack::new();
        stack.push(Val::i32(33i32));
        assert_eq!(33, stack.pop_i32_const().unwrap());

        stack.push(Val::local(10, WasmValType::I32));
        assert!(stack.pop_i32_const().is_none());
    }

    #[test]
    fn test_pop_reg() {
        let mut stack = Stack::new();
        let reg = Reg::int(2usize);
        stack.push(Val::reg(reg, WasmValType::I32));
        stack.push(Val::i32(4));

        assert_eq!(None, stack.pop_reg());
        let _ = stack.pop().unwrap();
        assert_eq!(reg, stack.pop_reg().unwrap().reg);
    }

    #[test]
    fn test_pop_named_reg() {
        let mut stack = Stack::new();
        let reg = Reg::int(2usize);
        stack.push(Val::reg(reg, WasmValType::I32));
        stack.push(Val::reg(Reg::int(4), WasmValType::I32));

        assert_eq!(None, stack.pop_named_reg(reg));
        let _ = stack.pop().unwrap();
        assert_eq!(reg, stack.pop_named_reg(reg).unwrap().reg);
    }
}

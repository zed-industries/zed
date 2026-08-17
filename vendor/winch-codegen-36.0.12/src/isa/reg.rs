use regalloc2::PReg;
pub use regalloc2::RegClass;

/// A newtype abstraction on top of a physical register.
//
// NOTE
// This is temporary; the intention behind this newtype
// is to keep the usage of PReg contained to this module
// so that the rest of Winch should only need to operate
// on top of the concept of `Reg`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reg(PReg);

pub(crate) type WritableReg = cranelift_codegen::Writable<Reg>;

/// Mark a given register as writable. This macro constructs
/// a [`cranelift_codegen::Writable`].
macro_rules! writable {
    ($e:expr) => {
        cranelift_codegen::Writable::from_reg($e)
    };
}

pub(crate) use writable;

impl Reg {
    /// Create a register from its encoding and class.
    pub fn from(class: RegClass, enc: usize) -> Self {
        Self::new(PReg::new(enc, class))
    }

    /// Create a new register from a physical register.
    pub const fn new(raw: PReg) -> Self {
        Reg(raw)
    }

    /// Create a new general purpose register from encoding.
    pub fn int(enc: usize) -> Self {
        Self::new(PReg::new(enc, RegClass::Int))
    }

    /// Create a new floating point register from encoding.
    pub fn float(enc: usize) -> Self {
        Self::new(PReg::new(enc, RegClass::Float))
    }

    /// Get the encoding of the underlying register.
    pub const fn hw_enc(self) -> usize {
        self.0.hw_enc()
    }

    /// Get the physical register representation.
    pub(super) fn inner(&self) -> PReg {
        self.0
    }

    /// Get the register class.
    pub fn class(&self) -> RegClass {
        self.0.class()
    }

    /// Returns true if the registers is a general purpose
    /// integer register.
    pub fn is_int(&self) -> bool {
        self.class() == RegClass::Int
    }

    /// Returns true if the registers is a float register.
    pub fn is_float(&self) -> bool {
        self.class() == RegClass::Float
    }
}

impl From<Reg> for cranelift_codegen::Reg {
    fn from(reg: Reg) -> Self {
        reg.inner().into()
    }
}

impl std::fmt::Debug for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

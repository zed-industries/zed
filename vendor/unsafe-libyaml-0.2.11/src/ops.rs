pub(crate) trait ForceAdd: Sized {
    fn force_add(self, rhs: Self) -> Self;
}

impl ForceAdd for u8 {
    fn force_add(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_or_else(die)
    }
}

impl ForceAdd for i32 {
    fn force_add(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_or_else(die)
    }
}

impl ForceAdd for u32 {
    fn force_add(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_or_else(die)
    }
}

impl ForceAdd for u64 {
    fn force_add(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_or_else(die)
    }
}

impl ForceAdd for usize {
    fn force_add(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_or_else(die)
    }
}

pub(crate) trait ForceMul: Sized {
    fn force_mul(self, rhs: Self) -> Self;
}

impl ForceMul for i32 {
    fn force_mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs).unwrap_or_else(die)
    }
}

impl ForceMul for i64 {
    fn force_mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs).unwrap_or_else(die)
    }
}

impl ForceMul for u64 {
    fn force_mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs).unwrap_or_else(die)
    }
}

pub(crate) trait ForceInto {
    fn force_into<U>(self) -> U
    where
        Self: TryInto<U>;
}

impl<T> ForceInto for T {
    fn force_into<U>(self) -> U
    where
        Self: TryInto<U>,
    {
        <Self as TryInto<U>>::try_into(self)
            .ok()
            .unwrap_or_else(die)
    }
}

// Deterministically abort on arithmetic overflow, instead of wrapping and
// continuing with invalid behavior.
//
// This is impossible or nearly impossible to hit as the arithmetic computations
// in libyaml are all related to either:
//
//  - small integer processing (ascii, hex digits)
//  - allocation sizing
//
// and the only allocations in libyaml are for fixed-sized objects and
// geometrically growing buffers with a growth factor of 2. So in order for an
// allocation computation to overflow usize, the previous allocation for that
// container must have been filled to a size of usize::MAX/2, which is an
// allocation that would have failed in the allocator. But we check for this to
// be pedantic and to find out if it ever does happen.
//
// No-std abort is implemented using a double panic. On most platforms the
// current mechanism for this is for core::intrinsics::abort to invoke an
// invalid instruction. On Unix, the process will probably terminate with a
// signal like SIGABRT, SIGILL, SIGTRAP, SIGSEGV or SIGBUS. The precise
// behaviour is not guaranteed and not stable, but is safe.
#[cold]
pub(crate) fn die<T>() -> T {
    struct PanicAgain;

    impl Drop for PanicAgain {
        fn drop(&mut self) {
            panic!("arithmetic overflow");
        }
    }

    fn do_die() -> ! {
        let _panic_again = PanicAgain;
        panic!("arithmetic overflow");
    }

    do_die();
}

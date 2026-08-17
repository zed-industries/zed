use core::{
    cmp::PartialEq,
    ops::{BitAnd, BitOr, Not},
};

pub trait Io {
    type Value: Copy
        + PartialEq
        + BitAnd<Output = Self::Value>
        + BitOr<Output = Self::Value>
        + Not<Output = Self::Value>;

    fn read(&self) -> Self::Value;
    fn write(&mut self, value: Self::Value);

    #[inline(always)]
    fn readf(&self, flags: Self::Value) -> bool {
        (self.read() & flags) as Self::Value == flags
    }

    #[inline(always)]
    fn writef(&mut self, flags: Self::Value, value: bool) {
        let tmp: Self::Value = match value {
            true => self.read() | flags,
            false => self.read() & !flags,
        };
        self.write(tmp);
    }
}

pub struct ReadOnly<I> {
    inner: I,
}

impl<I> ReadOnly<I> {
    pub const fn new(inner: I) -> ReadOnly<I> {
        ReadOnly { inner: inner }
    }
}

impl<I: Io> ReadOnly<I> {
    #[inline(always)]
    pub fn read(&self) -> I::Value {
        self.inner.read()
    }

    #[inline(always)]
    pub fn readf(&self, flags: I::Value) -> bool {
        self.inner.readf(flags)
    }
}

pub struct WriteOnly<I> {
    inner: I,
}

impl<I> WriteOnly<I> {
    pub const fn new(inner: I) -> WriteOnly<I> {
        WriteOnly { inner: inner }
    }
}

impl<I: Io> WriteOnly<I> {
    #[inline(always)]
    pub fn write(&mut self, value: I::Value) {
        self.inner.write(value)
    }

    #[inline(always)]
    pub fn writef(&mut self, flags: I::Value, value: bool) {
        self.inner.writef(flags, value)
    }
}

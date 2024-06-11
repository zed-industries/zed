use core::array;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Register(u8);
const N_REGISTERS: usize = 38;

impl Register {
    pub const DEFAULT: Self = Self(10);
    pub const SYSTEM: Self = Self(11);
    pub const A: Self = Self(12);
}

#[derive(Clone)]
pub(crate) struct Registers {
    registers: [Option<String>; N_REGISTERS],
    top: usize,
}

impl Registers {
    pub(crate) fn push(&mut self, content: String) {
        if self.top == 0 {
            self.top = 9;
        } else {
            self.top -= 1;
        }
        self.registers[self.top] = Some(content);
    }
}

fn get_true_index(index: Register, top: usize) -> usize {
    if index.0 > 9 {
        return index.0 as usize;
    }
    return (top + index.0 as usize) % 10;
}

impl Index<Register> for Registers {
    type Output = Option<String>;

    fn index(&self, index: Register) -> &Self::Output {
        return &self.registers[get_true_index(index, self.top)];
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        return &mut self.registers[get_true_index(index, self.top)];
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            registers: array::from_fn(|_| None),
            top: 0,
        }
    }
}

impl std::convert::TryFrom<Arc<str>> for Register {
    type Error = ();

    fn try_from(value: Arc<str>) -> Result<Self, Self::Error> {
        let chars = value.as_bytes();
        if chars.len() != 1 {
            return Err(());
        }
        let ch = chars[0];
        if ch >= 0x30 && ch <= 0x39 {
            return Ok(Register(ch & 0xf));
        } else if ch >= 0x61 && ch <= 0x7a {
            return Ok(Register(ch - 85));
        }
        Err(())
    }
}

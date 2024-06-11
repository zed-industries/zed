use core::array;
use std::ops::{Index, IndexMut};

#[allow(dead_code)]
pub(crate) enum Register {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Default, // "
    System,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

const N_REGISTERS: usize = Register::Z as usize + 1;

#[derive(Clone)]
pub(crate) struct Registers {
    registers: [Option<String>; N_REGISTERS],
    top: usize,
}

fn get_true_index(index: Register, top: usize) -> usize {
    let index = index as usize;
    if index > Register::Nine as usize {
        return index;
    }
    return (top + index) % 10;
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
            top: 9,
        }
    }
}

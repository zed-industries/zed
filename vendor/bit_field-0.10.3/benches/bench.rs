#![feature(test)]

extern crate bit_field;

use bit_field::*;

pub trait BitOper {
    const BIT_LEN: usize;
    fn get_b(&self, idx: usize) -> bool;
    fn set_b(&mut self, idx: usize, val: bool);
    fn toggle(&mut self, idx: usize);
}

pub trait BitArrayOper<T: BitOper> {
    fn get_blen(&self) -> usize;
    fn get_b(&self, idx: usize) -> bool;
    fn set_b(&mut self, idx: usize, val: bool);
    fn toggle(&mut self, idx: usize);
}

impl BitOper for u8 {
    const BIT_LEN: usize = std::mem::size_of::<Self>() as usize * 8;

    fn set_b(&mut self, idx: usize, val: bool) {
        assert!(idx < Self::BIT_LEN);
        if val {
            *self |= 1 << idx;
        } else {
            *self &= !(1 << idx);
        }
    }

    fn get_b(&self, idx: usize) -> bool {
        assert!(idx < Self::BIT_LEN);
        (self & 1 << idx) != 0
    }

    fn toggle(&mut self, idx: usize) {
        assert!(idx < Self::BIT_LEN);
        *self ^= 1 << idx;
    }
}

impl BitOper for u32 {
    const BIT_LEN: usize = std::mem::size_of::<Self>() as usize * 8;
    fn set_b(&mut self, idx: usize, val: bool) {
        assert!(idx < Self::BIT_LEN);
        if val {
            *self |= 1 << idx;
        } else {
            *self &= !(1 << idx);
        }
    }

    fn get_b(&self, idx: usize) -> bool {
        assert!(idx < Self::BIT_LEN);
        (self & 1 << idx) != 0
    }

    fn toggle(&mut self, idx: usize) {
        assert!(idx < Self::BIT_LEN);
        *self ^= 1 << idx;
    }
}

impl BitOper for u64 {
    const BIT_LEN: usize = std::mem::size_of::<Self>() as usize * 8;
    fn set_b(&mut self, idx: usize, val: bool) {
        assert!(idx < Self::BIT_LEN);
        if val {
            *self |= 1 << idx;
        } else {
            *self &= !(1 << idx);
        }
    }

    fn get_b(&self, idx: usize) -> bool {
        assert!(idx < Self::BIT_LEN);
        (self & 1 << idx) != 0
    }

    fn toggle(&mut self, idx: usize) {
        assert!(idx < Self::BIT_LEN);
        *self ^= 1 << idx;
    }
}

impl<T: BitOper> BitArrayOper<T> for [T] {
    fn get_blen(&self) -> usize {
        self.len() * T::BIT_LEN
    }

    fn get_b(&self, idx: usize) -> bool {
        self[idx / T::BIT_LEN].get_b(idx % T::BIT_LEN)
    }

    fn set_b(&mut self, idx: usize, val: bool) {
        self[idx / T::BIT_LEN].set_b(idx % T::BIT_LEN, val);
    }

    fn toggle(&mut self, idx: usize) {
        self[idx / T::BIT_LEN].toggle(idx % T::BIT_LEN);
    }
}

extern crate test;

use test::Bencher;

const LEN: usize = 256;

fn set_bitfield<T: BitField>(v: &mut Vec<T>) {
    for i in 0..v.len() * T::BIT_LENGTH {
        v.as_mut_slice().set_bit(i, true);
    }
}

fn get_bitfield<T: BitField>(v: &Vec<T>) {
    for i in 0..v.len() * T::BIT_LENGTH {
        let _b = v.as_slice().get_bit(i);
    }
}

fn set_trivial<T: BitOper>(v: &mut Vec<T>) {
    for i in 0..v.len() * T::BIT_LEN {
        v.set_b(i, true);
    }
}

fn get_trivial<T: BitOper>(v: &Vec<T>) {
    for i in 0..v.len() * T::BIT_LEN {
        let _b = v.get_b(i);
    }
}

#[bench]
fn u8_set_bitfield(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];
    b.iter(|| {
        set_bitfield(&mut v);
    });
}

#[bench]
fn u8_set_trivial(b: &mut Bencher) {
    let mut v = vec![0u8; LEN];

    b.iter(|| {
        set_trivial(&mut v);
    });
}

#[bench]
fn u8_get_bitfield(b: &mut Bencher) {
    let v = vec![1u8; LEN];
    b.iter(|| {
        get_bitfield(&v);
    });
}

#[bench]
fn u8_get_trivial(b: &mut Bencher) {
    let v = vec![1u8; LEN];
    b.iter(|| {
        get_trivial(&v);
    });
}

#[bench]
fn u32_set_bitfield(b: &mut Bencher) {
    let mut v = vec![0u32; LEN];
    b.iter(|| {
        set_bitfield(&mut v);
    });
}

#[bench]
fn u32_set_trivial(b: &mut Bencher) {
    let mut v = vec![0u32; LEN];

    b.iter(|| {
        set_trivial(&mut v);
    });
}

#[bench]
fn u32_get_bitfield(b: &mut Bencher) {
    let v = vec![1u32; LEN];
    b.iter(|| {
        get_bitfield(&v);
    });
}

#[bench]
fn u32_get_trivial(b: &mut Bencher) {
    let v = vec![1u32; LEN];
    b.iter(|| {
        get_trivial(&v);
    });
}

#[bench]
fn u64_set_bitfield(b: &mut Bencher) {
    let mut v = vec![0u64; LEN];
    b.iter(|| {
        set_bitfield(&mut v);
    });
}

#[bench]
fn u64_set_trivial(b: &mut Bencher) {
    let mut v = vec![0u64; LEN];

    b.iter(|| {
        set_trivial(&mut v);
    });
}

#[bench]
fn u64_get_bitfield(b: &mut Bencher) {
    let v = vec![1u64; LEN];
    b.iter(|| {
        get_bitfield(&v);
    });
}

#[bench]
fn u64_get_trivial(b: &mut Bencher) {
    let v = vec![1u64; LEN];
    b.iter(|| {
        get_trivial(&v);
    });
}

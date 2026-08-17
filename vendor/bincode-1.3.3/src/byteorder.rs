// Copyright (c) 2015 Andrew Gallant

use std::io;
use std::io::Result;
use std::ptr::copy_nonoverlapping;

#[derive(Copy, Clone)]
pub struct LittleEndian;

#[derive(Copy, Clone)]
pub struct BigEndian;

#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => {{
        assert!($size == ::std::mem::size_of::<$ty>());
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping($src.as_ptr(), &mut data as *mut $ty as *mut u8, $size);
        }
        data.$which()
    }};
}

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => {{
        assert!($size <= $dst.len());
        unsafe {
            // N.B. https://github.com/rust-lang/rust/issues/22776
            let bytes = *(&$n.$which() as *const _ as *const [u8; $size]);
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    }};
}

impl ByteOrder for LittleEndian {
    #[inline]
    fn read_u16(buf: &[u8]) -> u16 {
        read_num_bytes!(u16, 2, buf, to_le)
    }

    #[inline]
    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_le)
    }

    #[inline]
    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_le)
    }

    #[inline]
    fn write_u16(buf: &mut [u8], n: u16) {
        write_num_bytes!(u16, 2, n, buf, to_le);
    }

    #[inline]
    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_le);
    }

    #[inline]
    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_le);
    }

    serde_if_integer128! {
        #[inline]
        fn write_u128(buf: &mut [u8], n: u128) {
            write_num_bytes!(u128, 16, n, buf, to_le);
        }

        #[inline]
        fn read_u128(buf: &[u8]) -> u128 {
            read_num_bytes!(u128, 16, buf, to_le)
        }
    }
}

impl ByteOrder for BigEndian {
    #[inline]
    fn read_u16(buf: &[u8]) -> u16 {
        read_num_bytes!(u16, 2, buf, to_be)
    }

    #[inline]
    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_be)
    }

    #[inline]
    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_be)
    }

    #[inline]
    fn write_u16(buf: &mut [u8], n: u16) {
        write_num_bytes!(u16, 2, n, buf, to_be);
    }

    #[inline]
    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_be);
    }

    #[inline]
    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_be);
    }

    serde_if_integer128! {
        #[inline]
        fn write_u128(buf: &mut [u8], n: u128) {
            write_num_bytes!(u128, 16, n, buf, to_be);
        }

        #[inline]
        fn read_u128(buf: &[u8]) -> u128 {
            read_num_bytes!(u128, 16, buf, to_be)
        }
    }
}

pub trait ByteOrder: Clone + Copy {
    fn read_u16(buf: &[u8]) -> u16;

    fn read_u32(buf: &[u8]) -> u32;

    fn read_u64(buf: &[u8]) -> u64;

    fn write_u16(buf: &mut [u8], n: u16);

    fn write_u32(buf: &mut [u8], n: u32);

    fn write_u64(buf: &mut [u8], n: u64);

    #[inline]
    fn read_i16(buf: &[u8]) -> i16 {
        Self::read_u16(buf) as i16
    }

    #[inline]
    fn read_i32(buf: &[u8]) -> i32 {
        Self::read_u32(buf) as i32
    }

    #[inline]
    fn read_i64(buf: &[u8]) -> i64 {
        Self::read_u64(buf) as i64
    }

    #[inline]
    fn read_f32(buf: &[u8]) -> f32 {
        unsafe { *(&Self::read_u32(buf) as *const u32 as *const f32) }
    }

    #[inline]
    fn read_f64(buf: &[u8]) -> f64 {
        unsafe { *(&Self::read_u64(buf) as *const u64 as *const f64) }
    }

    #[inline]
    fn write_i16(buf: &mut [u8], n: i16) {
        Self::write_u16(buf, n as u16)
    }

    #[inline]
    fn write_i32(buf: &mut [u8], n: i32) {
        Self::write_u32(buf, n as u32)
    }

    #[inline]
    fn write_i64(buf: &mut [u8], n: i64) {
        Self::write_u64(buf, n as u64)
    }

    #[inline]
    fn write_f32(buf: &mut [u8], n: f32) {
        let n = unsafe { *(&n as *const f32 as *const u32) };
        Self::write_u32(buf, n)
    }

    #[inline]
    fn write_f64(buf: &mut [u8], n: f64) {
        let n = unsafe { *(&n as *const f64 as *const u64) };
        Self::write_u64(buf, n)
    }

    serde_if_integer128! {
        fn read_u128(buf: &[u8]) -> u128;
        fn write_u128(buf: &mut [u8], n: u128);

        #[inline]
        fn read_i128(buf: &[u8]) -> i128 {
            Self::read_u128(buf) as i128
        }

        #[inline]
        fn write_i128(buf: &mut [u8], n: i128) {
            Self::write_u128(buf, n as u128)
        }
    }
}

pub trait ReadBytesExt: io::Read {
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        try!(self.read_exact(&mut buf));
        Ok(buf[0])
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        try!(self.read_exact(&mut buf));
        Ok(buf[0] as i8)
    }

    #[inline]
    fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        try!(self.read_exact(&mut buf));
        Ok(T::read_u16(&buf))
    }

    #[inline]
    fn read_i16<T: ByteOrder>(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        try!(self.read_exact(&mut buf));
        Ok(T::read_i16(&buf))
    }

    #[inline]
    fn read_u32<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        try!(self.read_exact(&mut buf));
        Ok(T::read_u32(&buf))
    }

    #[inline]
    fn read_i32<T: ByteOrder>(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        try!(self.read_exact(&mut buf));
        Ok(T::read_i32(&buf))
    }

    #[inline]
    fn read_u64<T: ByteOrder>(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf));
        Ok(T::read_u64(&buf))
    }

    #[inline]
    fn read_i64<T: ByteOrder>(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf));
        Ok(T::read_i64(&buf))
    }

    #[inline]
    fn read_f32<T: ByteOrder>(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        try!(self.read_exact(&mut buf));
        Ok(T::read_f32(&buf))
    }

    #[inline]
    fn read_f64<T: ByteOrder>(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf));
        Ok(T::read_f64(&buf))
    }

    serde_if_integer128! {
        #[inline]
        fn read_u128<T: ByteOrder>(&mut self) -> Result<u128> {
            let mut buf = [0; 16];
            try!(self.read_exact(&mut buf));
            Ok(T::read_u128(&buf))
        }

        #[inline]
        fn read_i128<T: ByteOrder>(&mut self) -> Result<i128> {
            let mut buf = [0; 16];
            try!(self.read_exact(&mut buf));
            Ok(T::read_i128(&buf))
        }
    }
}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}

pub trait WriteBytesExt: io::Write {
    #[inline]
    fn write_u8(&mut self, n: u8) -> Result<()> {
        self.write_all(&[n])
    }

    #[inline]
    fn write_i8(&mut self, n: i8) -> Result<()> {
        self.write_all(&[n as u8])
    }

    #[inline]
    fn write_u16<T: ByteOrder>(&mut self, n: u16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i16<T: ByteOrder>(&mut self, n: i16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_i16(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_u32<T: ByteOrder>(&mut self, n: u32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i32<T: ByteOrder>(&mut self, n: i32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_u64<T: ByteOrder>(&mut self, n: u64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i64<T: ByteOrder>(&mut self, n: i64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_f32<T: ByteOrder>(&mut self, n: f32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_f32(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_f64<T: ByteOrder>(&mut self, n: f64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all(&buf)
    }

    serde_if_integer128! {
        #[inline]
        fn write_u128<T: ByteOrder>(&mut self, n: u128) -> Result<()> {
            let mut buf = [0; 16];
            T::write_u128(&mut buf, n);
            self.write_all(&buf)
        }

        #[inline]
        fn write_i128<T: ByteOrder>(&mut self, n: i128) -> Result<()> {
            let mut buf = [0; 16];
            T::write_i128(&mut buf, n);
            self.write_all(&buf)
        }
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt for W {}

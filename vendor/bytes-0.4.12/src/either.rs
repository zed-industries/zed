extern crate either;

use {Buf, BufMut};

use self::either::Either;
use self::either::Either::*;
use iovec::IoVec;

impl<L, R> Buf for Either<L, R>
where
    L: Buf,
    R: Buf,
{
    fn remaining(&self) -> usize {
        match *self {
            Left(ref b) => b.remaining(),
            Right(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        match *self {
            Left(ref b) => b.bytes(),
            Right(ref b) => b.bytes(),
        }
    }

    fn bytes_vec<'a>(&'a self, dst: &mut [&'a IoVec]) -> usize {
        match *self {
            Left(ref b) => b.bytes_vec(dst),
            Right(ref b) => b.bytes_vec(dst),
        }
    }

    fn advance(&mut self, cnt: usize) {
        match *self {
            Left(ref mut b) => b.advance(cnt),
            Right(ref mut b) => b.advance(cnt),
        }
    }

    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        match *self {
            Left(ref mut b) => b.copy_to_slice(dst),
            Right(ref mut b) => b.copy_to_slice(dst),
        }
    }
}

impl<L, R> BufMut for Either<L, R>
where
    L: BufMut,
    R: BufMut,
{
    fn remaining_mut(&self) -> usize {
        match *self {
            Left(ref b) => b.remaining_mut(),
            Right(ref b) => b.remaining_mut(),
        }
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        match *self {
            Left(ref mut b) => b.bytes_mut(),
            Right(ref mut b) => b.bytes_mut(),
        }
    }

    unsafe fn bytes_vec_mut<'a>(&'a mut self, dst: &mut [&'a mut IoVec]) -> usize {
        match *self {
            Left(ref mut b) => b.bytes_vec_mut(dst),
            Right(ref mut b) => b.bytes_vec_mut(dst),
        }
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        match *self {
            Left(ref mut b) => b.advance_mut(cnt),
            Right(ref mut b) => b.advance_mut(cnt),
        }
    }

    fn put_slice(&mut self, src: &[u8]) {
        match *self {
            Left(ref mut b) => b.put_slice(src),
            Right(ref mut b) => b.put_slice(src),
        }
    }
}

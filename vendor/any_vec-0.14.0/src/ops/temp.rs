use core::any::TypeId;
use core::{mem, ptr};
use crate::any_value::{AnyValue, AnyValueCloneable, AnyValueMut, AnyValueSizeless, AnyValueSizelessMut, AnyValueTypeless, AnyValueTypelessMut, Unknown};
use crate::any_vec_raw::AnyVecRaw;
use crate::any_vec_ptr::{IAnyVecPtr, IAnyVecRawPtr};
use crate::{AnyVec, copy_nonoverlapping_value};
use crate::traits::Cloneable;

pub trait Operation {
    type AnyVecPtr: IAnyVecRawPtr;

    fn any_vec_ptr(&self) -> Self::AnyVecPtr;

    fn bytes(&self) -> *const u8;

    /// Called after bytes move.
    fn consume(&mut self);
}

/// Temporary existing value in memory.
/// Data will be erased with TempValue destruction.
///
/// Have internal `&mut AnyVec`.
///
/// May do some postponed actions on consumption/destruction.
///
pub struct TempValue<Op: Operation>{
    op: Op,
}
impl<Op: Operation> TempValue<Op>{
    #[inline]
    pub(crate) fn new(op: Op) -> Self {
        Self{op}
    }

    #[inline]
    fn any_vec_raw(&self) -> &AnyVecRaw<<Op::AnyVecPtr as IAnyVecRawPtr>::M>{
        unsafe{ self.op.any_vec_ptr().any_vec_raw() }
    }

    #[inline]
    fn bytes_len(&self) -> usize{
        if Unknown::is::<<Op::AnyVecPtr as IAnyVecRawPtr>::Element>() {
            self.any_vec_raw().element_layout().size()
        } else{
            mem::size_of::<<Op::AnyVecPtr as IAnyVecRawPtr>::Element>()
        }
    }
}

impl<Op: Operation> AnyValueSizeless for TempValue<Op> {
    type Type = <Op::AnyVecPtr as IAnyVecRawPtr>::Element;

    #[inline]
    fn as_bytes_ptr(&self) -> *const u8 {
        self.op.bytes()
    }

    #[inline]
    unsafe fn move_into<KnownType:'static /*= Unknown*/>(mut self, out: *mut u8, bytes_size: usize) {
        copy_nonoverlapping_value::<KnownType>(self.as_bytes_ptr(), out, bytes_size);
        self.op.consume();
        mem::forget(self);
    }
}
impl<Op: Operation> AnyValueSizelessMut for TempValue<Op> {
    #[inline]
    fn as_bytes_mut_ptr(&mut self) -> *mut u8 {
        // Somehow this is OK with MIRI.
        self.op.bytes() as *mut u8
    }
}
impl<Op: Operation> AnyValueTypeless for TempValue<Op>{
    #[inline]
    fn size(&self) -> usize {
        self.bytes_len()
    }
}
impl<Op: Operation> AnyValue for TempValue<Op>{
    #[inline]
    fn value_typeid(&self) -> TypeId {
        let typeid = TypeId::of::<Self::Type>();
        if typeid == TypeId::of::<Unknown>(){
            self.any_vec_raw().type_id
        } else {
            typeid
        }
    }
}

impl<Op: Operation> AnyValueTypelessMut for TempValue<Op> {}
impl<Op: Operation> AnyValueMut for TempValue<Op> {}

impl<Op: Operation> AnyValueCloneable for TempValue<Op>
where
    Op::AnyVecPtr: IAnyVecPtr,
    <Op::AnyVecPtr as IAnyVecPtr>::Traits: Cloneable
{
    #[inline]
    unsafe fn clone_into(&self, out: *mut u8) {
        let clone_fn = self.op.any_vec_ptr().any_vec().clone_fn();
        (clone_fn)(self.as_bytes().as_ptr(), out, 1);
    }
}

impl<Op: Operation> Drop for TempValue<Op>{
    #[inline]
    fn drop(&mut self) {
        unsafe{
            let drop_fn = self.any_vec_raw().drop_fn;
            let element = self.op.bytes() as *mut u8;

            // compile-time check
            if Unknown::is::<<Self as AnyValueSizeless>::Type>() {
                if let Some(drop_fn) = drop_fn{
                    (drop_fn)(element, 1);
                }
            } else {
                ptr::drop_in_place(element as *mut <Self as AnyValueSizeless>::Type);
            }
        }
        self.op.consume();
    }
}

unsafe impl<Op: Operation> Send for TempValue<Op>
where
    Op::AnyVecPtr: IAnyVecPtr,
    AnyVec<
        <Op::AnyVecPtr as IAnyVecPtr>::Traits,
        <Op::AnyVecPtr as IAnyVecRawPtr>::M
    >: Send
{}

unsafe impl<Op: Operation> Sync for TempValue<Op>
where
    Op::AnyVecPtr: IAnyVecPtr,
    AnyVec<
        <Op::AnyVecPtr as IAnyVecPtr>::Traits,
        <Op::AnyVecPtr as IAnyVecRawPtr>::M
    >: Sync
{}

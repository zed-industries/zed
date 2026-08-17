#![no_std]
#![cfg_attr(miri, feature(alloc_layout_extra) )]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Type erased vector [`AnyVec`]. Allow to store elements of the same type.
//! Have same performance and *operations* as `std::vec::Vec`.
//!
//! You can downcast type erased [`AnyVec`] to concrete [`AnyVecTyped`] with `downcast`-family.
//! Or use [`AnyVec`] type erased operations, which works with [`any_value`].
//!
//! ```rust
//!     use any_vec::AnyVec;
//!     let mut vec: AnyVec = AnyVec::new::<String>();
//!     {
//!         // Typed operations.
//!         let mut vec = vec.downcast_mut::<String>().unwrap();
//!         vec.push(String::from("0"));
//!         vec.push(String::from("1"));
//!         vec.push(String::from("2"));
//!     }
//!
//!     let mut other_vec: AnyVec = AnyVec::new::<String>();
//!     // Fully type erased element move from one vec to another
//!     // without intermediate mem-copies.
//!     let element = vec.swap_remove(0);
//!     other_vec.push(element);
//!
//!     // Output 2 1
//!     for s in vec.downcast_ref::<String>().unwrap(){
//!         println!("{}", s);
//!     }
//!
//!```
//!
//! N.B. [`AnyVecTyped`] operations may be somewhat faster, due to the fact that
//! compiler able to do better optimisation with full type knowledge.
//!
//! # Send, Sync, Clone
//!
//! You can make [`AnyVec`] [`Send`]able, [`Sync`]able, [`Cloneable`]:
//!
//! [`Cloneable`]: traits::Cloneable
//!
//!```rust
//! use any_vec::AnyVec;
//! use any_vec::traits::*;
//! let v1: AnyVec<dyn Cloneable + Sync + Send> = AnyVec::new::<String>();
//! let v2 = v1.clone();
//! ```
//! This constraints will be applied compiletime to element type:
//!```compile_fail
//! # use any_vec::AnyVec;
//! # use std::rc::Rc;
//! let v1: AnyVec<dyn Sync + Send> = AnyVec::new::<Rc<usize>>();
//!```
//!
//! # LazyClone
//!
//! Whenever possible, [`any_vec`] types implement [`AnyValueCloneable`], which
//! can work with [`LazyClone`]:
//!
//! [`any_vec`]: crate
//! [`AnyValueCloneable`]: any_value::AnyValueCloneable
//! [`LazyClone`]: any_value::LazyClone
//!
//!```rust
//! # use any_vec::any_value::{AnyValueCloneable, AnyValueWrapper};
//! # use any_vec::AnyVec;
//! # use any_vec::traits::*;
//! let mut v1: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
//! v1.push(AnyValueWrapper::new(String::from("0")));
//!
//! let mut v2: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
//! let e = v1.swap_remove(0);
//! v2.push(e.lazy_clone());
//! v2.push(e.lazy_clone());
//! ```
//!
//! # MemBuilder
//!
//! [`MemBuilder`] + [`Mem`] works like [`Allocator`] for [`AnyVec`]. But unlike allocator,
//! [`Mem`] container-specialized design allows to perform more optimizations. For example,
//! it is possible to make stack-allocated `FixedAnyVec` and small-buffer-optimized(SBO) `SmallAnyVec`
//! from `AnyVec` by just changing [`MemBuilder`]:
//!
//!```rust
//! # use any_vec::any_value::AnyValueWrapper;
//! # use any_vec::AnyVec;
//! # use any_vec::mem::Stack;
//! # use any_vec::traits::None;
//!
//! type FixedAnyVec<Traits = dyn None> = AnyVec<Traits, Stack<512>>;
//! let mut any_vec: FixedAnyVec = AnyVec::new::<String>();
//!
//! // This will be on stack, without any allocations.
//! any_vec.push(AnyValueWrapper::new(String::from("0")))
//!```
//!
//! With help of [`clone_empty_in`] you can use stack allocated, or SBO [`AnyVec`]
//! as fast intermediate storage for values of unknown type:
//!
//!```rust
//! # use any_vec::any_value::{AnyValueCloneable, AnyValueWrapper};
//! # use any_vec::AnyVec;
//! # use any_vec::mem::StackN;
//! # use any_vec::traits::*;
//!
//! fn self_push_first_element<T: Trait + Cloneable>(any_vec: &mut AnyVec<T>){
//!    let mut tmp = any_vec.clone_empty_in(StackN::<1, 256>);
//!    tmp.push(any_vec.at(0).lazy_clone());
//!    any_vec.push(tmp.pop().unwrap());
//! }
//!```
//!
//! [`MemBuilder`] interface, being stateful, allow to make [`Mem`],
//! which can work with complex custom allocators.
//!
//! [`MemBuilder`]: mem::MemBuilder
//! [`Mem`]: mem::Mem
//! [`Allocator`]: core::alloc::Allocator
//! [`clone_empty_in`]: AnyVec::clone_empty_in
//!
//! # AnyValue
//!
//! Being type erased, [AnyVec] needs a way to operate on untyped values safely.
//! Instead of working with plain `*mut u8`, [AnyVec] operates with [any_value].
//!
//! [AnyValue] is a trait, that provide operations to work with type erased values.
//! Any type that implements [AnyValue] can be used with [AnyVec].
//! [AnyValue] interface allows to perform postponed operations on consumption.
//! This trick used heavily by [AnyVec] destructive operations, which instead of concrete
//! type return [AnyValue], which perform actual operation on value drop.
//!
//! Implementing [AnyValueMut] and [AnyValueCloneable] makes type mutable and
//! cloneable respectively.
//!
//! [AnyValue]: any_value::AnyValue
//! [AnyValueMut]: any_value::AnyValueMut
//! [AnyValueCloneable]: any_value::AnyValueCloneable
//! 
//! # No `alloc`
//! 
//! This library is `no_std` and can work without `alloc`.
//! For this - disable default `alloc` feature. [mem::Heap] will become unavailable
//! after that, and you'll have to specify [MemBuilder] for [AnyVec]. You can use
//! [mem::Stack], or specify your own [Mem].
//! 
//! [MemBuilder]: mem::MemBuilder
//! [Mem]: mem::Mem

mod any_vec;
mod clone_type;
mod any_vec_ptr;
mod any_vec_raw;
mod any_vec_typed;
mod iter;

use core::any::TypeId;
pub use crate::any_vec::{AnyVec, AnyVecMut, AnyVecRef, RawParts, SatisfyTraits, traits};
pub use any_vec_typed::AnyVecTyped;
pub use iter::{ElementIterator, Iter, IterMut, IterRef};

pub mod mem;
pub mod any_value;
pub mod ops;
pub mod element;

use core::ptr;
use core::ops::{Bound, Range, RangeBounds};
use crate::any_value::Unknown;

/// This is faster then ptr::copy,
/// when count is runtime value, and count is small.
///
/// Last time benchmarked on nightly 1.80
#[inline(always)]
unsafe fn copy_bytes(src: *const u8, dst: *mut u8, count: usize){
    // MIRI hack
    if cfg!(miri)
        || count >= 128
    {
        ptr::copy(src, dst, count);
        return;
    }

    for i in 0..count{
        *dst.add(i) = *src.add(i);
    }
}

/// One element copy_nonoverlapping, that utilize type knowledge.
#[inline(always)]
pub(crate) unsafe fn copy_nonoverlapping_value<KnownType: 'static>(
    input: *const u8, out: *mut u8, value_size: usize
) {
    if !Unknown::is::<KnownType>() {
        ptr::copy_nonoverlapping(
            input as *const KnownType,
            out as *mut KnownType,
            1
        );
    } else {
        ptr::copy_nonoverlapping(
            input,
            out,
            value_size
        );
    }
}

#[inline]
fn into_range(
    len: usize,
    range: impl RangeBounds<usize>
) -> Range<usize> {
    let start = match range.start_bound() {
        Bound::Included(i) => *i,
        Bound::Excluded(i) => *i + 1,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(i) => *i + 1,
        Bound::Excluded(i) => *i,
        Bound::Unbounded => len,
    };
    assert!(start <= end);
    assert!(end <= len);
    start..end
}

#[inline]
fn assert_types_equal(t1: TypeId, t2: TypeId){
    assert_eq!(t1, t2, "Type mismatch!");
}
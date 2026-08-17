use core::alloc::Layout;
use core::mem::MaybeUninit;
use crate::mem::{Mem, MemBuilder};

/// Fixed `SIZE` capacity on-stack memory for `N` elements.
///
/// Can contain `N` elements, with total size at most `SIZE` bytes.
/// Unlike [`Stack`] does not involve heavy operations for building.
///
/// N.B. It should be `ELEMENT_SIZE` instead of total `SIZE`, but Rust
/// still can't do `N * ELEMENT_SIZE` in generic context.
///
/// [`Stack`]: super::Stack
#[derive(Default, Clone, Copy)]
pub struct StackN<const N:usize, const SIZE: usize>;
impl<const N:usize, const SIZE: usize> MemBuilder for StackN<N, SIZE>{
    type Mem = StackNMem<N, SIZE>;

    #[inline]
    fn build(&mut self, element_layout: Layout) -> Self::Mem {
        assert!(N*element_layout.size() <= SIZE, "Insufficient storage!");
        StackNMem{
            mem: MaybeUninit::uninit(),
            element_layout
        }
    }
}

pub struct StackNMem<const N:usize, const SIZE: usize>{
    mem: MaybeUninit<[u8; SIZE]>,
    element_layout: Layout
}

impl<const N:usize, const SIZE: usize> Mem for StackNMem<N, SIZE>{
    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.mem.as_ptr() as *const u8
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.mem.as_mut_ptr() as *mut u8
    }

    #[inline]
    fn element_layout(&self) -> Layout {
        self.element_layout
    }

    #[inline]
    fn size(&self) -> usize {
        N
    }
}
use core::any::TypeId;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use crate::any_value::{AnyValue, AnyValueCloneable, AnyValueMut, AnyValueTypelessMut, AnyValueTypeless, AnyValueSizeless, AnyValueSizelessMut};
use crate::any_vec_raw::AnyVecRaw;
use crate::any_vec_ptr::{AnyVecPtr, IAnyVecPtr, IAnyVecRawPtr};
use crate::{AnyVec, mem};
use crate::mem::MemBuilder;
use crate::traits::{Cloneable, None, Trait};

// Typed operations will never use type-erased ElementPointer, so there is no
// need in type-known-based optimizations.

/// Owning pointer to type-erased [`AnyVec`] element.
///
/// Obtained by dereferencing [`ElementRef`], [`ElementMut`] or from some
/// destructive [`AnyVec`] operations, like [`drain`] or [`splice`].
///
/// # Consuming
///
/// Whenever you have `ElementPointer` as a value (from destructive [`AnyVec`] operations),
/// you can safely take pointed value with [`AnyValue::downcast`], or unsafely
/// take its content with [`AnyValueSizeless::move_into`].
/// Otherwise, it will be destructed with destruction of [`Element`].
///
/// # Notes
///
/// `ElementPointer` have it's own implementation of `downcast_` family (which return `&'a T`, instead of `&T`).
/// This is done, so you don't have to keep [`ElementRef`]/[`ElementMut`] alive, while casting to concrete type.
///
/// [`AnyVec`]: crate::AnyVec
/// [`AnyVec::get`]: crate::AnyVec::get
/// [`drain`]: crate::AnyVec::drain
/// [`splice`]: crate::AnyVec::splice
/// [`any_value`]: crate::any_value
pub struct ElementPointer<'a, AnyVecPtr: IAnyVecRawPtr>{
    any_vec_ptr: AnyVecPtr,
    element: NonNull<u8>,
    phantom: PhantomData<&'a mut AnyVecRaw<AnyVecPtr::M>>
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> ElementPointer<'a, AnyVecPtr>{
    #[inline]
    pub(crate) fn new(any_vec_ptr: AnyVecPtr, element: NonNull<u8>) -> Self {
        Self{any_vec_ptr, element, phantom: PhantomData}
    }

    /// ElementPointer should not be `Clone`, because it can be both & and &mut.
    #[inline]
    pub(crate) fn clone(&self) -> Self {
        Self::new(self.any_vec_ptr, self.element)
    }

    #[inline]
    fn any_vec_raw(&self) -> &'a AnyVecRaw<AnyVecPtr::M>{
        unsafe { self.any_vec_ptr.any_vec_raw() }
    }

    /// Same as [`AnyValue::downcast_ref`], but return `&'a T`, instead of `&T`.
    #[inline]
    pub fn downcast_ref<T: 'static>(&self) -> Option<&'a T>{
        if self.value_typeid() != TypeId::of::<T>(){
            None
        } else {
            Some(unsafe{ self.downcast_ref_unchecked::<T>() })
        }
    }

    /// Same as [`AnyValueSizeless::downcast_ref_unchecked`], but return `&'a T`, instead of `&T`.
    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &'a T{
        &*(self.as_bytes().as_ptr() as *const T)
    }

    /// Same as [`AnyValueMut::downcast_mut`], but return `&'a mut T`, instead of `&mut T`.
    #[inline]
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&'a mut T>{
        if self.value_typeid() != TypeId::of::<T>(){
            None
        } else {
            Some(unsafe{ self.downcast_mut_unchecked::<T>() })
        }
    }

    /// Same as [`AnyValueSizelessMut::downcast_mut_unchecked`], but return `&'a mut T`, instead of `&mut T`.
    #[inline]
    pub unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &'a mut T{
        &mut *(self.as_bytes_mut().as_mut_ptr() as *mut T)
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> Drop for ElementPointer<'a, AnyVecPtr>{
    #[inline]
    fn drop(&mut self) {
        if let Some(drop_fn) = self.any_vec_raw().drop_fn{
            unsafe{
                (drop_fn)(self.element.as_ptr(), 1);
            }
        }
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> AnyValueSizeless for ElementPointer<'a, AnyVecPtr> {
    type Type = AnyVecPtr::Element;

    #[inline]
    fn as_bytes_ptr(&self) -> *const u8 {
        self.element.as_ptr()
    }
}
impl<'a, AnyVecPtr: IAnyVecRawPtr> AnyValueTypeless for ElementPointer<'a, AnyVecPtr>{
    #[inline]
    fn size(&self) -> usize {
        self.any_vec_raw().element_layout().size()
    }
}
impl<'a, AnyVecPtr: IAnyVecRawPtr> AnyValue for ElementPointer<'a, AnyVecPtr>{
    #[inline]
    fn value_typeid(&self) -> TypeId {
        self.any_vec_raw().type_id
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> AnyValueSizelessMut   for ElementPointer<'a, AnyVecPtr>{
    #[inline]
    fn as_bytes_mut_ptr(&mut self) -> *mut u8 {
        self.element.as_ptr()
    }
}
impl<'a, AnyVecPtr: IAnyVecRawPtr> AnyValueTypelessMut for ElementPointer<'a, AnyVecPtr>{}
impl<'a, AnyVecPtr: IAnyVecRawPtr> AnyValueMut for ElementPointer<'a, AnyVecPtr>{}

impl<'a, Traits: ?Sized + Cloneable + Trait, M: MemBuilder>
    AnyValueCloneable for ElementPointer<'a, AnyVecPtr<Traits, M>>
{
    #[inline]
    unsafe fn clone_into(&self, out: *mut u8) {
        let clone_fn = self.any_vec_ptr.any_vec().clone_fn();
        (clone_fn)(self.as_bytes().as_ptr(), out, 1);
    }
}

unsafe impl<'a, Traits: ?Sized + Trait, M: MemBuilder> Send
for
    ElementPointer<'a, AnyVecPtr<Traits, M>>
where
    AnyVec<Traits, M>: Send
{}

unsafe impl<'a, Traits: ?Sized + Trait, M: MemBuilder> Sync
for
    ElementPointer<'a, AnyVecPtr<Traits, M>>
where
    AnyVec<Traits, M>: Sync
{}

impl<'a, Traits: ?Sized + Trait, M: MemBuilder> ElementReference<'a, Traits, M>
    for &'a ElementPointer<'a, AnyVecPtr<Traits, M>>
{}


// Do not implement Send/Sync for AnyVecPtrRaw, since it will be casted to concrete type anyway.


/// [`AnyVec`] element.
///
/// See [`ElementPointer`].
///
/// [`AnyVec`]: crate::AnyVec
pub type Element<'a, Traits = dyn None, M = mem::Default> = ElementPointer<'a, AnyVecPtr<Traits, M>>;

/// Reference to [`AnyVec`] element.
///
/// Implemented by [`ElementRef`], [`ElementMut`] and &[`Element`].
pub trait ElementReference<'a, Traits: ?Sized + Trait = dyn None, M: MemBuilder + 'a = mem::Default>
    : Deref<Target = Element<'a, Traits, M> >
{}

/// Reference to [`AnyVec`] element.
///
/// Created by  [`AnyVec::get`].
///
/// [`AnyVec::get`]: crate::AnyVec::get
pub struct ElementRef<'a, Traits: ?Sized + Trait = dyn None, M: MemBuilder = mem::Default>(
    pub(crate) ManuallyDrop<Element<'a, Traits, M>>
);
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> ElementReference<'a, Traits, M> for ElementRef<'a, Traits, M>{}
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> Deref for ElementRef<'a, Traits, M>{
    type Target = Element<'a, Traits, M>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> Clone for ElementRef<'a, Traits, M>{
    #[inline]
    fn clone(&self) -> Self {
        Self(ManuallyDrop::new(self.0.clone()))
    }
}

/// Mutable reference to [`AnyVec`] element.
///
/// Created by  [`AnyVec::get_mut`].
///
/// [`AnyVec::get_mut`]: crate::AnyVec::get_mut
pub struct ElementMut<'a, Traits: ?Sized + Trait = dyn None, M: MemBuilder = mem::Default>(
    pub(crate) ManuallyDrop<Element<'a, Traits, M>>
);
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> ElementReference<'a, Traits, M> for ElementMut<'a, Traits, M>{}
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> Deref for ElementMut<'a, Traits, M>{
    type Target = Element<'a, Traits, M>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> DerefMut for ElementMut<'a, Traits, M>{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
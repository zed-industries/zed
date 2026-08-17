use std::any::TypeId;

pub(crate) fn transmute_mut_if_eq<A: 'static, B: 'static>(a: &mut A) -> Result<&mut B, &mut A> {
    if TypeId::of::<A>() == TypeId::of::<B>() {
        // SAFETY: we check type before transmuting.
        Ok(unsafe { &mut *(a as *mut A as *mut B) })
    } else {
        Err(a)
    }
}

pub(crate) fn transmute_ref_if_eq<A: 'static, B: 'static>(a: &A) -> Result<&B, &A> {
    if TypeId::of::<A>() == TypeId::of::<B>() {
        // SAFETY: we check type before transmuting.
        Ok(unsafe { &*(a as *const A as *const B) })
    } else {
        Err(a)
    }
}

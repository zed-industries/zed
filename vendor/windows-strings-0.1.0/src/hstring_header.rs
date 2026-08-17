use super::*;

pub const HSTRING_REFERENCE_FLAG: u32 = 1;

#[repr(C)]
pub struct HStringHeader {
    pub flags: u32,
    pub len: u32,
    pub _0: u32,
    pub _1: u32,
    pub data: *mut u16,
    pub count: RefCount,
    pub buffer_start: u16,
}

impl HStringHeader {
    pub fn alloc(len: u32) -> Result<*mut Self> {
        if len == 0 {
            return Ok(core::ptr::null_mut());
        }

        // Allocate enough space for header and two bytes per character.
        // The space for the terminating null character is already accounted for inside of `HStringHeader`.
        let bytes = core::mem::size_of::<Self>() + 2 * len as usize;

        let header =
            unsafe { bindings::HeapAlloc(bindings::GetProcessHeap(), 0, bytes) } as *mut Self;

        if header.is_null() {
            return Err(Error::from_hresult(HRESULT(bindings::E_OUTOFMEMORY)));
        }

        unsafe {
            // Use `ptr::write` (since `header` is unintialized). `HStringHeader` is safe to be all zeros.
            header.write(core::mem::MaybeUninit::<Self>::zeroed().assume_init());
            (*header).len = len;
            (*header).count = RefCount::new(1);
            (*header).data = &mut (*header).buffer_start;
        }

        Ok(header)
    }

    pub unsafe fn free(header: *mut Self) {
        if header.is_null() {
            return;
        }

        bindings::HeapFree(bindings::GetProcessHeap(), 0, header as *mut _);
    }

    pub fn duplicate(&self) -> Result<*mut Self> {
        if self.flags & HSTRING_REFERENCE_FLAG == 0 {
            // If this is not a "fast pass" string then simply increment the reference count.
            self.count.add_ref();
            Ok(self as *const Self as *mut Self)
        } else {
            // Otherwise, allocate a new string and copy the value into the new string.
            let copy = Self::alloc(self.len)?;
            // SAFETY: since we are duplicating the string it is safe to copy all data from self to the initialized `copy`.
            // We copy `len + 1` characters since `len` does not account for the terminating null character.
            unsafe {
                core::ptr::copy_nonoverlapping(self.data, (*copy).data, self.len as usize + 1);
            }
            Ok(copy)
        }
    }
}

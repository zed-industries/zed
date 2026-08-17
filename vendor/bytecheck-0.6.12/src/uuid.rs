//! [`CheckBytes`](crate::CheckBytes) implementations for uuid.

use crate::CheckBytes;
use uuid::{Bytes, Uuid};

impl<C: ?Sized> CheckBytes<C> for Uuid {
    type Error = <Bytes as CheckBytes<C>>::Error;

    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        // Safety: cast is OK because Uuid is repr(transparent)
        Bytes::check_bytes(value.cast(), context)?;
        Ok(&*value)
    }
}

#[cfg(test)]
mod bytecheck_tests {
    use crate::CheckBytes;
    use uuid::Uuid;

    #[test]
    fn test_check_bytes() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let u = Uuid::parse_str(uuid_str).unwrap();

        // Safety: the pointer is aligned and points to enough bytes to represent a Uuid
        unsafe {
            Uuid::check_bytes(&u as *const Uuid, &mut ()).expect("failed to check uuid");
        }
    }
}

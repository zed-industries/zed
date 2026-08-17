#[cfg(feature = "nightly")]
use core::mem::MaybeUninit;

#[cfg(feature = "nightly")]
use crate::io::{ErrorKind, Read, Write};

#[cfg(feature = "nightly")]
pub fn copy<R: ?Sized, W: ?Sized, const S: usize>(
    reader: &mut R,
    writer: &mut W,
) -> crate::io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = MaybeUninit::<[u8; S]>::uninit();
    // FIXME: #42788
    //
    //   - This creates a (mut) reference to a slice of
    //     _uninitialized_ integers, which is **undefined behavior**
    //
    //   - Only the standard library gets to soundly "ignore" this,
    //     based on its privileged knowledge of unstable rustc
    //     internals;
    unsafe {
        reader.initializer().initialize(buf.assume_init_mut());
    }

    let mut written = 0;
    loop {
        let len = match reader.read(unsafe { buf.assume_init_mut() }) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(unsafe { &buf.assume_init_ref()[..len] })?;
        written += len as u64;
    }
}

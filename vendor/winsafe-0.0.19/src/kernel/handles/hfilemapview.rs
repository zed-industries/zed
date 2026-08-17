#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;

impl_handle! { HFILEMAPVIEW;
	/// Address of a
	/// [mapped view](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile).
	/// Originally just an `LPVOID`.
	///
	/// Unless you need something specific, consider using the
	/// [`FileMapped`](crate::FileMapped) high-level abstraction.
}

impl kernel_Hfilemapview for HFILEMAPVIEW {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HFILEMAPVIEW`](crate::HFILEMAPVIEW).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hfilemapview: Handle {
	/// Returns a slice representing the mapped memory. You can modify the
	/// contents. You should call this method only if the file has write access.
	///
	/// **Note**: If the file is resized to a smaller size, the slice will still
	/// map the bytes beyond the file. This may cause serious errors. So, if the
	/// file is resized, re-generate the slice by calling `as_slice` again.
	#[must_use]
	fn as_mut_slice(&self, len: usize) -> &mut [u8] {
		unsafe { std::slice::from_raw_parts_mut(self.ptr() as _, len) }
	}

	/// Returns a slice representing the mapped memory.
	///
	/// **Note**: If the file is resized to a smaller size, the slice will still
	/// map the bytes beyond the file. This may cause serious errors. So, if the
	/// file is resized, re-generate the slice by calling `as_slice` again.
	///
	/// # Examples
	///
	/// Reading the contents of a file into a string:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let (hfile, _) = w::HFILE::CreateFile(
	///     "C:\\Temp\\test.txt",
	///     co::GENERIC::READ,
	///     Some(co::FILE_SHARE::READ),
	///     None,
	///     co::DISPOSITION::OPEN_EXISTING,
	///     co::FILE_ATTRIBUTE::NORMAL,
	///     None,
	///     None,
	///     None,
	/// )?;
	///
	/// let hmap = hfile.CreateFileMapping(
	///     None,
	///     co::PAGE::READONLY,
	///     None,
	///     None,
	/// )?;
	///
	/// let view = hmap.MapViewOfFile(co::FILE_MAP::READ, 0, None)?;
	///
	/// let slice = view.as_slice(hfile.GetFileSizeEx()? as _);
	/// let text = std::str::from_utf8(slice)?;
	///
	/// println!("{}", text);
	/// # Ok::<_, Box<dyn std::error::Error>>(())
	/// ```
	#[must_use]
	fn as_slice(&self, len: usize) -> &[u8] {
		unsafe { std::slice::from_raw_parts(self.ptr() as _, len) }
	}
}

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;

/// Manages an [`HFILEMAP`](crate::HFILEMAP) handle, which provides
/// memory-mapped file operations, including read/write through slices. It is
/// closed automatically when the object goes out of scope.
///
/// # Examples
///
/// [Parsing](crate::WString::parse) a file as string by memory-mapping the file
/// (usually the fastest method):
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let f = w::FileMapped::open(
///     "C:\\Temp\\foo.txt",
///     w::FileAccess::ExistingReadOnly,
/// )?;
/// let raw_bytes = f.as_slice();
/// let text = w::WString::parse(raw_bytes)?.to_string();
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub struct FileMapped {
	access: FileAccess,
	file: File,
	hmap: CloseHandleGuard<HFILEMAP>,
	hview: UnmapViewOfFileGuard,
	size: u64,
}

impl FileMapped {
	/// Opens a file with the desired access, then map its contents in memory.
	#[must_use]
	pub fn open(file_path: &str, access: FileAccess) -> SysResult<Self> {
		let file = File::open(file_path, access)?;
		let (hmap, hview) = Self::map_in_memory(&file, access)?;
		let size = file.hfile().GetFileSizeEx()?; // cache
		Ok(Self { access, file, hmap, hview, size })
	}

	#[must_use]
	fn map_in_memory(
		file: &File,
		access: FileAccess,
	) -> SysResult<(CloseHandleGuard<HFILEMAP>, UnmapViewOfFileGuard)>
	{
		let hmap = file.hfile().CreateFileMapping(
			None,
			match access {
				FileAccess::ExistingReadOnly => co::PAGE::READONLY,
				FileAccess::ExistingRW
					| FileAccess::OpenOrCreateRW
					| FileAccess::CreateRW => co::PAGE::READWRITE,
			},
			None,
			None,
		)?;

		let hview = hmap.MapViewOfFile(
			match access {
				FileAccess::ExistingReadOnly => co::FILE_MAP::READ,
				FileAccess::ExistingRW
					| FileAccess::OpenOrCreateRW
					| FileAccess::CreateRW => co::FILE_MAP::READ | co::FILE_MAP::WRITE,
			},
			0,
			None,
		)?;

		Ok((hmap, hview))
	}

	/// Returns a mutable slice to the mapped memory.
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		self.hview.as_mut_slice(self.size as _)
	}

	/// Returns a slice to the mapped memory.
	#[must_use]
	pub fn as_slice(&self) -> &[u8] {
		self.hview.as_slice(self.size as _)
	}

	/// Returns the underlying file handle.
	#[must_use]
	pub fn hfile(&self) -> &HFILE {
		self.file.hfile()
	}

	/// Resizes the file, which will be remapped in memory.
	///
	/// **Note:** Since the mapping pointers will change, any existing slices
	/// must be recreated. The following functions must be called again:
	/// * [`as_mut_slice`](crate::FileMapped::as_mut_slice);
	/// * [`as_slice`](crate::FileMapped::as_slice).
	pub fn resize(&mut self, num_bytes: u64) -> SysResult<()> {
		unsafe {
			self.hview = UnmapViewOfFileGuard::new(HFILEMAPVIEW::NULL); // close mapping handles
			self.hmap = CloseHandleGuard::new(HFILEMAP::NULL);
		}

		self.file.resize(num_bytes)?;
		let (hmap, hview) = Self::map_in_memory(&self.file, self.access)?;

		self.hmap = hmap;
		self.hview = hview;
		self.size = num_bytes;
		Ok(())
	}

	/// Returns the size of the file.
	///
	/// This value is cached.
	#[must_use]
	pub const fn size(&self) -> u64 {
		self.size
	}

	/// Returns the creation and last write times of the file, in the current
	/// time zone.
	#[must_use]
	pub fn times(&self) -> SysResult<(SYSTEMTIME, SYSTEMTIME)> {
		self.file.times()
	}
}

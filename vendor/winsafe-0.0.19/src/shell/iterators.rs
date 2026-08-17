use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::shell::ffi;

pub(in crate::shell) struct HdropIter<'a, H>
	where H: shell_Hdrop,
{
	hdrop: &'a mut H,
	buffer: WString,
	count: u32,
	current: u32,
}

impl<'a, H> Drop for HdropIter<'a, H>
	where H: shell_Hdrop,
{
	fn drop(&mut self) {
		unsafe { ffi::DragFinish(self.hdrop.ptr()); }
	}
}

impl<'a, H> Iterator for HdropIter<'a, H>
	where H: shell_Hdrop,
{
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match unsafe {
			ffi::DragQueryFileW(
				self.hdrop.ptr(),
				self.current,
				self.buffer.as_mut_ptr(),
				self.buffer.buf_len() as _,
			)
		} {
			0 => {
				self.current = self.count; // no further iterations will be made
				Some(Err(GetLastError()))
			},
			_ => {
				self.current += 1;
				Some(Ok(self.buffer.to_string()))
			},
		}
	}
}

impl<'a, H> HdropIter<'a, H>
	where H: shell_Hdrop,
{
	pub(in crate::shell) fn new(hdrop: &'a mut H) -> SysResult<Self> {
		let count = unsafe {
			ffi::DragQueryFileW( // preliminar call to retrieve the file count
				hdrop.ptr(),
				0xffff_ffff,
				std::ptr::null_mut(),
				0,
			)
		};

		Ok(Self {
			hdrop,
			buffer: WString::new_alloc_buf(MAX_PATH + 1), // so we alloc just once
			count,
			current: 0,
		})
	}
}

//------------------------------------------------------------------------------

pub(in crate::shell) struct IenumshellitemsIter<'a, I>
	where I: shell_IEnumShellItems,
{
	enum_shi: &'a I,
}

impl<'a, I> Iterator for IenumshellitemsIter<'a, I>
	where I: shell_IEnumShellItems,
{
	type Item = HrResult<IShellItem>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_shi.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a, I> IenumshellitemsIter<'a, I>
	where I: shell_IEnumShellItems,
{
	pub(in crate::shell) fn new(enum_shi: &'a I) -> Self {
		Self { enum_shi }
	}
}

//------------------------------------------------------------------------------

pub(in crate::shell) struct IshellitemarrayIter<'a, I>
	where I: shell_IShellItemArray,
{
	shi_arr: &'a I,
	count: u32,
	current: u32,
}

impl<'a, I> Iterator for IshellitemarrayIter<'a, I>
	where I: shell_IShellItemArray,
{
	type Item = HrResult<IShellItem>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.shi_arr.GetItemAt(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(shell_item) => {
				self.current += 1;
				Some(Ok(shell_item))
			},
		}
	}
}

impl<'a, I> IshellitemarrayIter<'a, I>
	where I: shell_IShellItemArray,
{
	pub(in crate::shell) fn new(shi_arr: &'a I) -> HrResult<Self> {
		let count = shi_arr.GetCount()?;
		Ok(Self { shi_arr, count, current: 0 })
	}
}

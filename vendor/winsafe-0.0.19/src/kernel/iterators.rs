use crate::co;
use crate::decl::*;
use crate::kernel::ffi;
use crate::prelude::*;

pub(in crate::kernel) struct HheapHeapwalkIter<'a, H>
	where H: kernel_Hheap,
{
	hheap: &'a H,
	entry: PROCESS_HEAP_ENTRY,
	has_more: bool,
}

impl<'a, H> Iterator for HheapHeapwalkIter<'a, H>
	where H: kernel_Hheap,
{
	type Item = SysResult<&'a PROCESS_HEAP_ENTRY>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		match unsafe {
			ffi::HeapWalk(self.hheap.ptr(), &mut self.entry as *mut _ as _)
		} {
			0 => {
				self.has_more = false; // no further iterations
				match GetLastError() {
					co::ERROR::NO_MORE_ITEMS => None, // search completed successfully
					err => Some(Err(err)), // actual error
				}
			},
			_ => {
				// Returning a reference cannot be done until GATs
				// stabilization, so we simply cheat the borrow checker.
				let ptr = &self.entry as *const PROCESS_HEAP_ENTRY;
				Some(Ok(unsafe { &*ptr }))
			},
		}
	}
}

impl<'a, H> HheapHeapwalkIter<'a, H>
	where H: kernel_Hheap,
{
	pub(in crate::kernel) fn new(hheap: &'a H) -> Self {
		Self {
			hheap,
			entry: PROCESS_HEAP_ENTRY::default(),
			has_more: true,
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::kernel) struct HkeyKeyIter<'a, H>
	where H: kernel_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for HkeyKeyIter<'a, H>
	where H: kernel_Hkey,
{
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match unsafe {
			co::ERROR::from_raw(
				ffi::RegEnumKeyExW(
					self.hkey.ptr(),
					self.current,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				) as _,
			)
		} {
			co::ERROR::SUCCESS => {
				self.current += 1;
				Some(Ok(self.name_buffer.to_string()))
			},
			e => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
		}
	}
}

impl<'a, H> HkeyKeyIter<'a, H>
	where H: kernel_Hkey,
{
	pub(in crate::kernel) fn new(hkey: &'a H) -> SysResult<Self> {
		let mut num_keys = u32::default();
		let mut max_key_name_len = u32::default();
		hkey.RegQueryInfoKey(
			None, Some(&mut num_keys), Some(&mut max_key_name_len),
			None, None, None, None, None, None)?;

		Ok(Self {
			hkey,
			count: num_keys,
			current: 0,
			name_buffer: WString::new_alloc_buf(max_key_name_len as usize + 1),
		})
	}
}

//------------------------------------------------------------------------------

pub(in crate::kernel) struct HkeyValueIter<'a, H>
	where H: kernel_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for HkeyValueIter<'a, H>
	where H: kernel_Hkey,
{
	type Item = SysResult<(String, co::REG)>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut raw_data_type = u32::default();
		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match unsafe {
			co::ERROR::from_raw(
				ffi::RegEnumValueW(
					self.hkey.ptr(),
					self.current,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					&mut raw_data_type,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				) as _,
			)
		} {
			co::ERROR::SUCCESS => {
				self.current += 1;
				Some(Ok((self.name_buffer.to_string(), unsafe { co::REG::from_raw(raw_data_type) })))
			},
			e => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
		}
	}
}

impl<'a, H> HkeyValueIter<'a, H>
	where H: kernel_Hkey,
{
	pub(in crate::kernel) fn new(hkey: &'a H) -> SysResult<Self> {
		let mut num_vals = u32::default();
		let mut max_val_name_len = u32::default();
		hkey.RegQueryInfoKey(
			None, None, None, None, Some(&mut num_vals), Some(&mut max_val_name_len),
			None, None, None)?;

		Ok(Self {
			hkey,
			count: num_vals,
			current: 0,
			name_buffer: WString::new_alloc_buf(max_val_name_len as usize + 1),
		})
	}
}

//------------------------------------------------------------------------------

pub(in crate::kernel) struct HprocesslistHeapIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a mut H,
	hl32: HEAPLIST32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for HprocesslistHeapIter<'a, H>
	where H: kernel_Hprocesslist,
{
	type Item = SysResult<&'a HEAPLIST32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Heap32ListFirst(&mut self.hl32)
		} else {
			self.hpl.Heap32ListNext(&mut self.hl32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.hl32 as *const HEAPLIST32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no heap found
				}
			},
		}
	}
}

impl<'a, H> HprocesslistHeapIter<'a, H>
	where H: kernel_Hprocesslist,
{
	pub(in crate::kernel) fn new(hpl: &'a mut H) -> Self {
		Self {
			hpl,
			hl32: HEAPLIST32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::kernel) struct HprocesslistModuleIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a mut H,
	me32: MODULEENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for HprocesslistModuleIter<'a, H>
	where H: kernel_Hprocesslist,
{
	type Item = SysResult<&'a MODULEENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Module32First(&mut self.me32)
		} else {
			self.hpl.Module32Next(&mut self.me32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.me32 as *const MODULEENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no module found
				}
			},
		}
	}
}

impl<'a, H> HprocesslistModuleIter<'a, H>
	where H: kernel_Hprocesslist,
{
	pub(in crate::kernel) fn new(hpl: &'a mut H) -> Self {
		Self {
			hpl,
			me32: MODULEENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::kernel) struct HprocesslistProcessIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a mut H,
	pe32: PROCESSENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for HprocesslistProcessIter<'a, H>
	where H: kernel_Hprocesslist,
{
	type Item = SysResult<&'a PROCESSENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Process32First(&mut self.pe32)
		} else {
			self.hpl.Process32Next(&mut self.pe32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.pe32 as *const PROCESSENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no process found
				}
			},
		}
	}
}

impl<'a, H> HprocesslistProcessIter<'a, H>
	where H: kernel_Hprocesslist,
{
	pub(in crate::kernel) fn new(hpl: &'a mut H) -> Self {
		Self {
			hpl,
			pe32: PROCESSENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

//------------------------------------------------------------------------------

pub(in crate::kernel) struct HprocesslistThreadIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a mut H,
	te32: THREADENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for HprocesslistThreadIter<'a, H>
	where H: kernel_Hprocesslist,
{
	type Item = SysResult<&'a THREADENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Thread32First(&mut self.te32)
		} else {
			self.hpl.Thread32Next(&mut self.te32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.te32 as *const THREADENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no thread found
				}
			},
		}
	}
}

impl<'a, H> HprocesslistThreadIter<'a, H>
	where H: kernel_Hprocesslist,
{
	pub(in crate::kernel) fn new(hpl: &'a mut H) -> Self {
		Self {
			hpl,
			te32: THREADENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::co;
use crate::decl::*;
use crate::kernel::ffi;
use crate::prelude::*;

/// RAII implementation for a [`Handle`](crate::prelude::Handle) which
/// automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// when the object goes out of scope.
pub struct CloseHandleGuard<T>
	where T: Handle,
{
	handle: T,
}

impl<T> Drop for CloseHandleGuard<T>
	where T: Handle,
{
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			unsafe { ffi::CloseHandle(h.ptr()); } // ignore errors
		}
	}
}

impl<T> Deref for CloseHandleGuard<T>
	where T: Handle,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl<T> DerefMut for CloseHandleGuard<T>
	where T: Handle,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.handle
	}
}

impl<T> CloseHandleGuard<T>
	where T: Handle,
{
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(handle: T) -> Self {
		Self { handle }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> T {
		std::mem::replace(&mut self.handle, T::INVALID)
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION)
/// which automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// on `hProcess` and `hThread` fields when the object goes out of scope.
pub struct CloseHandlePiGuard {
	pi: PROCESS_INFORMATION,
}

impl Drop for CloseHandlePiGuard {
	fn drop(&mut self) {
		if let Some(h) = self.pi.hProcess.as_opt() {
			let _ = unsafe { CloseHandleGuard::new(h.raw_copy()) };
		}
		if let Some(h) = self.pi.hThread.as_opt() {
			let _ = unsafe { CloseHandleGuard::new(h.raw_copy()) };
		}
	}
}

impl Deref for CloseHandlePiGuard {
	type Target = PROCESS_INFORMATION;

	fn deref(&self) -> &Self::Target {
		&self.pi
	}
}

impl DerefMut for CloseHandlePiGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.pi
	}
}

impl CloseHandlePiGuard {
	/// Constructs the guard by taking ownership of the struct.
	///
	/// # Safety
	///
	/// Be sure the handles must be freed with
	/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// at the end of the scope.
	#[must_use]
	pub const unsafe fn new(pi: PROCESS_INFORMATION) -> Self {
		Self { pi }
	}

	/// Ejects the underlying struct, leaving
	/// [`PROCESS_INFORMATION::default`](crate::PROCESS_INFORMATION::default) in
	/// its place.
	///
	/// Since the internal handles will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> PROCESS_INFORMATION {
		std::mem::take(&mut self.pi)
	}
}

//------------------------------------------------------------------------------

handle_guard! { CloseServiceHandleGuard: HSC;
	ffi::CloseServiceHandle;
	/// RAII implementation for [`HSC`](crate::HSC) which automatically calls
	/// [`CloseServiceHandle`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-closeservicehandle)
	/// when the object goes out of scope.
}

handle_guard! { CloseServiceHandleSvcGuard: HSERVICE;
	ffi::CloseServiceHandle;
	/// RAII implementation for [`HSERVICE`](crate::HSERVICE) which
	/// automatically calls
	/// [`CloseServiceHandle`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-closeservicehandle)
	/// when the object goes out of scope.
}

handle_guard! { DeregisterEventSourceGuard: HEVENTLOG;
	ffi::DeregisterEventSource;
	/// RAII implementation for [`HEVENTLOG`](crate::HEVENTLOG) which
	/// automatically calls
	/// [`DeregisterEventSource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-deregistereventsource)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

/// RAII implementation [`HUPDATERSRC`](crate::HUPDATERSRC) which automatically
/// calls
/// [`EndUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
/// when the object goes out of scope.
pub struct EndUpdateResourceGuard {
	hupsrc: HUPDATERSRC,
}

impl Drop for EndUpdateResourceGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hupsrc.as_opt() {
			unsafe { ffi::EndUpdateResourceW(h.ptr(), false as _); } // ignore errors
		}
	}
}

impl Deref for EndUpdateResourceGuard {
	type Target = HUPDATERSRC;

	fn deref(&self) -> &Self::Target {
		&self.hupsrc
	}
}

impl DerefMut for EndUpdateResourceGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hupsrc
	}
}

impl EndUpdateResourceGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`EndUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hupsrc: HUPDATERSRC) -> Self {
		Self { hupsrc }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> HUPDATERSRC {
		std::mem::replace(&mut self.hupsrc, HUPDATERSRC::INVALID)
	}
}

//------------------------------------------------------------------------------

handle_guard! { FindCloseGuard: HFINDFILE;
	ffi::FindClose;
	/// RAII implementation for [`HFINDFILE`](crate::HFINDFILE) which
	/// automatically calls
	/// [`FindClose`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// when the object goes out of scope.
}

handle_guard! { FreeLibraryGuard: HINSTANCE;
	ffi::FreeLibrary;
	/// RAII implementation for [`HINSTANCE`](crate::HINSTANCE) which
	/// automatically calls
	/// [`FreeLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

/// RAII implementation for [`SID`](crate::SID), returned by
/// [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid), which
/// automatically calls
/// [`FreeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
/// when the object goes out of scope.
pub struct FreeSidGuard {
	psid: *mut SID,
}

impl Drop for FreeSidGuard {
	fn drop(&mut self) {
		if !self.psid.is_null() {
			unsafe { ffi::FreeSid(self.psid as *mut _ as _); } // ignore errors
		}
	}
}

impl Deref for FreeSidGuard {
	type Target = SID;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.psid }
	}
}

impl std::fmt::Display for FreeSidGuard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.deref(), f) // delegate to the underlying SID
	}
}

impl FreeSidGuard {
	/// Constructs the guard by taking ownership of the pointer.
	///
	/// # Safety
	///
	/// Be sure the pointer must be freed with
	/// [`FreeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid).
	#[must_use]
	pub const unsafe fn new(psid: *mut SID) -> Self {
		Self { psid }
	}

	/// Ejects the underlying pointer, leaving a null pointer in its place.
	///
	/// Since the internal pointer will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> *mut SID {
		std::mem::replace(&mut self.psid, std::ptr::null_mut())
	}
}

//------------------------------------------------------------------------------

handle_guard! { GlobalFreeGuard: HGLOBAL;
	ffi::GlobalFree;
	/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) which automatically
	/// calls
	/// [`GlobalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalfree)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) lock which automatically
/// calls
/// [`GlobalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
/// when the object goes out of scope.
pub struct GlobalUnlockGuard<'a, H>
	where H: kernel_Hglobal,
{
	hglobal: &'a H,
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl<'a, H> Drop for GlobalUnlockGuard<'a, H>
	where H: kernel_Hglobal,
{
	fn drop(&mut self) {
		if let Some(h) = self.hglobal.as_opt() {
			unsafe { ffi::GlobalUnlock(h.ptr()); } // ignore errors
		}
	}
}

impl<'a, H> GlobalUnlockGuard<'a, H>
	where H: kernel_Hglobal,
{
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`GlobalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
	/// at the end of scope, the pointer is valid, and the size is correct.
	#[must_use]
	pub const unsafe fn new(
		hglobal: &'a H,
		pmem: *mut std::ffi::c_void,
		sz: usize,
	) -> Self
	{
		Self { hglobal, pmem, sz }
	}

	pub_fn_mem_block!();
}

//------------------------------------------------------------------------------

handle_guard! { HeapDestroyGuard: HHEAP;
	ffi::HeapDestroy;
	/// RAII implementation for [`HHEAP`](crate::HHEAP) which automatically
	/// calls
	/// [`HeapDestroy`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapdestroy)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

/// RAII implementation for the memory allocated by
/// [`HHEAP::HeapAlloc`](crate::prelude::kernel_Hheap::HeapAlloc) which
/// automatically calls
/// [`HeapFree`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree)
/// when the object goes out of scope.
pub struct HeapFreeGuard<'a, H>
	where H: kernel_Hheap,
{
	hheap: &'a H,
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl<'a, H> Drop for HeapFreeGuard<'a, H>
	where H: kernel_Hheap,
{
	fn drop(&mut self) {
		if let Some(h) = self.hheap.as_opt() {
			if !self.pmem.is_null() {
				unsafe { ffi::HeapFree(h.ptr(), 0, self.pmem); } // ignore errors
			}
		}
	}
}

impl<'a, H> HeapFreeGuard<'a, H>
	where H: kernel_Hheap,
{
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`HeapFree`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree)
	/// at the end of scope, the pointer is valid, and the size is correct.
	#[must_use]
	pub const unsafe fn new(
		hheap: &'a H,
		pmem: *mut std::ffi::c_void,
		sz: usize,
	) -> Self
	{
		Self { hheap, pmem, sz }
	}

	/// Ejects the underlying memory pointer and size, leaving null and zero in
	/// their places.
	///
	/// Since the internal memory pointer will be invalidated, the destructor
	/// will not run. It's your responsibility to run it, otherwise you'll cause
	/// a memory leak.
	#[must_use]
	pub fn leak(&mut self) -> (*mut std::ffi::c_void, usize) {
		(
			std::mem::replace(&mut self.pmem, std::ptr::null_mut()),
			std::mem::replace(&mut self.sz, 0),
		)
	}

	pub_fn_mem_block!();
}

//------------------------------------------------------------------------------

/// RAII implementation for [`HHEAP`](crate::HHEAP) which automatically calls
/// [`HeapUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapunlock)
/// when the object goes out of scope.
pub struct HeapUnlockGuard<'a, H>
	where H: kernel_Hheap,
{
	hheap: &'a H,
}

impl<'a, H> Drop for HeapUnlockGuard<'a, H>
	where H: kernel_Hheap,
{
	fn drop(&mut self) {
		if let Some(h) = self.hheap.as_opt() {
			unsafe { ffi::HeapUnlock(h.ptr()); } // ignore errors
		}
	}
}

impl<'a, H> HeapUnlockGuard<'a, H>
	where H: kernel_Hheap,
{
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`HeapUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapunlock)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hheap: &'a H) -> Self {
		Self { hheap }
	}
}

//------------------------------------------------------------------------------

handle_guard! { LocalFreeGuard: HLOCAL;
	ffi::LocalFree;
	/// RAII implementation for [`HLOCAL`](crate::HLOCAL) which automatically
	/// calls
	/// [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// when the object goes out of scope.
}

//------------------------------------------------------------------------------

/// RAII implementation for [`SID`](crate::SID), returned by
/// [`ConvertStringSidToSid`](crate::ConvertStringSidToSid), which automatically
/// calls
/// [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
/// when the object goes out of scope.
pub struct LocalFreeSidGuard {
	pmem: LocalFreeGuard,
}

impl Deref for LocalFreeSidGuard {
	type Target = SID;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.pmem.ptr() as *mut _) }
	}
}

impl std::fmt::Display for LocalFreeSidGuard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.deref(), f) // delegate to the underlying SID
	}
}

impl LocalFreeSidGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the pointer is an [`HLOCAL`](crate::HLOCAL) handle pointing to a
	/// [`SID`](crate::SID) memory block.
	#[must_use]
	pub const unsafe fn new(pmem: HLOCAL) -> Self {
		Self { pmem: LocalFreeGuard::new(pmem) }
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for [`HLOCAL`](crate::HLOCAL) lock which automatically
/// calls
/// [`LocalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localunlock)
/// when the object goes out of scope.
pub struct LocalUnlockGuard<'a, H>
	where H: kernel_Hlocal,
{
	hlocal: &'a H,
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl<'a, H> Drop for LocalUnlockGuard<'a, H>
	where H: kernel_Hlocal,
{
	fn drop(&mut self) {
		if let Some(h) = self.hlocal.as_opt() {
			unsafe { ffi::LocalUnlock(h.ptr()); } // ignore errors
		}
	}
}

impl<'a, H> LocalUnlockGuard<'a, H>
	where H: kernel_Hlocal,
{
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`LocalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localunlock)
	/// at the end of scope, the pointer is valid, and the size is correct.
	#[must_use]
	pub const unsafe fn new(
		hlocal: &'a H,
		pmem: *mut std::ffi::c_void,
		sz: usize,
	) -> Self
	{
		Self { hlocal, pmem, sz }
	}

	pub_fn_mem_block!();
}

//------------------------------------------------------------------------------

/// RAII implementation for [`HKEY`](crate::HKEY) which automatically calls
/// [`RegCloseKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
/// when the object goes out of scope.
pub struct RegCloseKeyGuard {
	hkey: HKEY,
}

impl Drop for RegCloseKeyGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hkey.as_opt() {
			if !self.is_predef_key() { // guard predefined keys
				unsafe { ffi::RegCloseKey(h.ptr()); } // ignore errors
			}
		}
	}
}

impl Deref for RegCloseKeyGuard {
	type Target = HKEY;

	fn deref(&self) -> &Self::Target {
		&self.hkey
	}
}

impl DerefMut for RegCloseKeyGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hkey
	}
}

impl RegCloseKeyGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`RegCloseKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hkey: HKEY) -> Self {
		Self { hkey }
	}

	/// Ejects the underlying handle, leaving
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> HKEY {
		std::mem::replace(&mut self.hkey, HKEY::INVALID)
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for [`SID`](crate::SID), returned by
/// [`CopySid`](crate::CopySid),
/// [`CreateWellKnownSid`](crate::CreateWellKnownSid),
/// [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid) and
/// [`LookupAccountName`](crate::LookupAccountName), which automatically frees
/// the underlying memory block when the object goes out of scope.
pub struct SidGuard {
	ptr: GlobalFreeGuard,
}

impl Deref for SidGuard {
	type Target = SID;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.ptr.ptr() as *const _) }
	}
}

impl std::fmt::Display for SidGuard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.deref(), f) // delegate to the underlying SID
	}
}

impl SidGuard {
	/// Constructs a new guard by taking ownership of the data.
	///
	/// # Safety
	///
	/// Be sure the data is an allocated [`SID`](crate::SID) structure.
	#[must_use]
	pub const unsafe fn new(ptr: GlobalFreeGuard) -> Self {
		Self { ptr }
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for [`TOKEN_GROUPS`](crate::TOKEN_GROUPS) which manages
/// the allocated memory.
pub struct TokenGroupsGuard<'a> {
	ptr: GlobalFreeGuard,
	_groups: PhantomData<&'a ()>,
}

impl<'a> Deref for TokenGroupsGuard<'a> {
	type Target = TOKEN_GROUPS<'a>;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.ptr.ptr() as *const _) }
	}
}

impl<'a> DerefMut for TokenGroupsGuard<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(self.ptr.ptr() as *mut _) }
	}
}

impl<'a> TokenGroupsGuard<'a> {
	#[must_use]
	pub(in crate::kernel) fn new(
		groups: &'a [SID_AND_ATTRIBUTES<'a>],
	) -> SysResult<Self>
	{
		let sz = std::mem::size_of::<TOKEN_GROUPS>() // size in bytes of the allocated struct
			- std::mem::size_of::<SID_AND_ATTRIBUTES>()
			+ (groups.len() * std::mem::size_of::<SID_AND_ATTRIBUTES>());
		let mut new_self = Self {
			ptr: HGLOBAL::GlobalAlloc(
				Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
				sz,
			)?,
			_groups: PhantomData,
		};
		new_self.GroupCount = groups.len() as _;
		groups.iter()
			.zip(new_self.Groups_mut())
			.for_each(|(src, dest)| *dest = src.clone()); // copy all SID_AND_ATTRIBUTES into struct room
		Ok(new_self)
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for [`TOKEN_PRIVILEGES`](crate::TOKEN_PRIVILEGES) which
/// manages the allocated memory.
pub struct TokenPrivilegesGuard {
	ptr: GlobalFreeGuard,
}

impl Deref for TokenPrivilegesGuard {
	type Target = TOKEN_PRIVILEGES;

	fn deref(&self) -> &Self::Target {
		unsafe { &*(self.ptr.ptr() as *const _) }
	}
}

impl DerefMut for TokenPrivilegesGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(self.ptr.ptr() as *mut _) }
	}
}

impl TokenPrivilegesGuard {
	pub(in crate::kernel) fn new(
		privileges: &[LUID_AND_ATTRIBUTES],
	) -> SysResult<Self>
	{
		let sz = std::mem::size_of::<TOKEN_PRIVILEGES>() // size in bytes of the allocated struct
			- std::mem::size_of::<LUID_AND_ATTRIBUTES>()
			+ (privileges.len() * std::mem::size_of::<LUID_AND_ATTRIBUTES>());
		let mut new_self = Self {
			ptr: HGLOBAL::GlobalAlloc(
				Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
				sz,
			)?,
		};
		new_self.PrivilegeCount = privileges.len() as _;
		privileges.iter()
			.zip(new_self.Privileges_mut())
			.for_each(|(src, dest)| *dest = *src); // copy all LUID_AND_ATTRIBUTES into struct room
		Ok(new_self)
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for the [`HFILE`](crate::HFILE) lock which automatically
/// calls
/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
/// when the object goes out of scope.
pub struct UnlockFileGuard<'a, H>
	where H: kernel_Hfile,
{
	hfile: &'a H,
	offset: u64,
	num_bytes_to_lock: u64,
}

impl<'a, H> Drop for UnlockFileGuard<'a, H>
	where H: kernel_Hfile,
{
	fn drop(&mut self) {
		if let Some(h) = self.hfile.as_opt() {
			unsafe {
				ffi::UnlockFile( // ignore errors
					h.ptr(),
					LODWORD(self.offset),
					HIDWORD(self.offset),
					LODWORD(self.num_bytes_to_lock),
					HIDWORD(self.num_bytes_to_lock),
				);
			}
		}
	}
}

impl<'a, H> UnlockFileGuard<'a, H>
	where H: kernel_Hfile,
{
	/// Constructs the guard by taking ownership of the objects.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(
		hfile: &'a H,
		offset: u64,
		num_bytes_to_lock: u64,
	) -> Self
	{
		Self { hfile, offset, num_bytes_to_lock }
	}

	/// Returns the memory offset of the lock.
	#[must_use]
	pub const fn offset(&self) -> u64 {
		self.offset
	}

	/// Returns the number of locked bytes.
	#[must_use]
	pub const fn num_bytes_to_lock(&self) -> u64 {
		self.num_bytes_to_lock
	}
}

//------------------------------------------------------------------------------

handle_guard! { UnmapViewOfFileGuard: HFILEMAPVIEW;
	ffi::UnmapViewOfFile;
	/// RAII implementation for [`HFILEMAPVIEW`](crate::HFILEMAPVIEW) which
	/// automatically calls
	/// [`UnmapViewOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile)
	/// when the object goes out of scope.
}

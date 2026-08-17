#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;

/// [`COAUTHIDENTITY`](https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ns-wtypesbase-coauthidentity)
/// struct.
#[repr(C)]
pub struct COAUTHIDENTITY<'a, 'b, 'c> {
	User: *mut u16,
	UserLength: u32,
	Domain: *mut u16,
	DomainLength: u32,
	Password: *mut u16,
	PasswordLength: u32,
	pub Flags: co::SEC_WINNT_AUTH_IDENTITY,

	_User: PhantomData<&'a mut u16>,
	_Domain: PhantomData<&'b mut u16>,
	_Password: PhantomData<&'c mut u16>,
}

impl_default!(COAUTHIDENTITY, 'a, 'b, 'c);

impl<'a, 'b, 'c> COAUTHIDENTITY<'a, 'b, 'c> {
	pub_fn_string_ptrlen_get_set!('a, User, set_User, UserLength);
	pub_fn_string_ptrlen_get_set!('b, Domain, set_Domain, DomainLength);
	pub_fn_string_ptrlen_get_set!('c, Password, set_Password, PasswordLength);
}

/// [`COAUTHINFO`](https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ns-wtypesbase-coauthinfo)
/// struct.
#[repr(C)]
pub struct COAUTHINFO<'a, 'b, 'c, 'd, 'e> {
	pub dwAuthnSvc: co::RPC_C_AUTHN,
	pub dwAuthzSvc: co::RPC_C_AUTHZ,
	pwszServerPrincName: *mut u16,
	pub dwAuthnLevel: co::RPC_C_AUTHN,
	pub dwImpersonationLevel: co::RPC_C_IMP_LEVEL,
	pAuthIdentityData: *mut COAUTHIDENTITY<'c, 'd, 'e>,
	pub dwCapabilities: co::RPC_C_QOS_CAPABILITIES,

	_pwszServerPrincName: PhantomData<&'a mut u16>,
	_pAuthIdentityData: PhantomData<&'b mut COAUTHIDENTITY<'c, 'd, 'e>>,
}

impl_default!(COAUTHINFO, 'a, 'b, 'c, 'd, 'e);

impl<'a, 'b, 'c, 'd, 'e> COAUTHINFO<'a, 'b, 'c, 'd, 'e> {
	pub_fn_string_ptr_get_set!('a, pwszServerPrincName, set_pwszServerPrincName);
	pub_fn_ptr_get_set!('b, pAuthIdentityData, set_pAuthIdentityData, COAUTHIDENTITY<'c, 'd, 'e>);
}

/// [`COSERVERINFO`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-coserverinfo)
/// struct.
#[repr(C)]
pub struct COSERVERINFO<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	dwReserved1: u32,
	pwszName: *mut u16,
	pAuthInfo: *mut COAUTHINFO<'c, 'd, 'e, 'f, 'g>,
	dwReserved2: u32,

	_pwszName: PhantomData<&'a mut u16>,
	_pAuthInfo: PhantomData<&'b COAUTHINFO<'c, 'd, 'e, 'f, 'g>>,
}

impl_default!(COSERVERINFO, 'a, 'b, 'c, 'd, 'e, 'f, 'g);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> COSERVERINFO<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	pub_fn_string_ptr_get_set!('a, pwszName, set_pwszName);
	pub_fn_ptr_get_set!('b, pAuthInfo, set_pAuthInfo, COAUTHINFO<'c, 'd, 'e, 'f, 'g>);
}

/// [`FORMATETC`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-formatetc)
/// struct.
///
/// The [`Default`] trait automatically initializes `lindex` to `-1`.
#[repr(C)]
pub struct FORMATETC<'a> {
	cfFormat: u16,
	ptd: *mut DVTARGETDEVICE,
	pub dwAspect: u32,
	pub lindex: i32,
	pub tymed: co::TYMED,

	_ptd: PhantomData<&'a mut DVTARGETDEVICE>,
}

impl<'a> Default for FORMATETC<'a> {
	fn default() -> Self {
		Self {
			cfFormat: 0,
			ptd: std::ptr::null_mut(),
			dwAspect: 0,
			lindex: -1,
			tymed: co::TYMED::default(),
			_ptd: PhantomData,
		}
	}
}

impl<'a> FORMATETC<'a> {
	/// Returns the `cfFormat` field.
	#[must_use]
	pub fn cfFormat(&self) -> co::CF {
		unsafe { co::CF::from_raw(self.cfFormat as _) }
	}

	/// Sets the `cfFormat` field.
	pub fn set_cfFormat(&mut self, val: co::CF) {
		self.cfFormat = val.raw() as _;
	}

	pub_fn_ptr_get_set!('a, ptd, set_ptd, DVTARGETDEVICE);
}

/// [`DVTARGETDEVICE`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-dvtargetdevice)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DVTARGETDEVICE {
	pub tdSize: u32,
	pub tdDriverNameOffset: u16,
	pub tdDeviceNameOffset: u16,
	pub tdPortNameOffset: u16,
	pub tdExtDevmodeOffset: u16,
	pub tdData: [u8; 1],
}

impl VariableSized for DVTARGETDEVICE {}

/// [`MULTI_QI`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-multi_qi)
/// struct.
///
/// Used to query multiple interfaces with
/// [`CoCreateInstanceEx`](crate::CoCreateInstanceEx).
#[repr(C)]
pub struct MULTI_QI<'a> {
	pIID: *mut co::IID,
	pItf: COMPTR,
	pub hr: co::HRESULT,

	_pIID: PhantomData<&'a mut co::IID>,
}

impl_default!(MULTI_QI, 'a);
impl_drop_comptr!(pItf, MULTI_QI, 'a);

impl<'a> MULTI_QI<'a> {
	pub_fn_ptr_get_set!('a, pIID, set_pIID, co::IID);
	pub_fn_comptr_get_set!(pItf, set_pItf, ole_IUnknown);
}

/// [`SNB`](https://learn.microsoft.com/en-us/windows/win32/stg/snb)
/// struct.
#[repr(transparent)]
pub struct SNB(*mut *mut u16);

impl_default!(SNB);

impl Drop for SNB {
	fn drop(&mut self) {
		let _ = unsafe { CoTaskMemFreeGuard::new(self.0 as _, 0) }; // size is irrelevant
	}
}

impl SNB {
	/// Creates a new `SNB` by allocating a contiguous memory block for pointers
	/// and strings.
	pub fn new(strs: &[impl AsRef<str>]) -> HrResult<Self> {
		if strs.is_empty() {
			return Ok(Self::default());
		}

		let tot_bytes_ptrs = (strs.len() + 1) * std::mem::size_of::<*mut u16>(); // plus null

		let wstrs = WString::from_str_vec(strs);
		let num_valid_chars = wstrs.as_slice() // count just the chars; important because stack alloc has zero fills
			.iter()
			.filter(|ch| **ch != 0x0000)
			.count();
		let tot_bytes_wstrs = ( num_valid_chars + strs.len() ) // plus one null for each string
			* std::mem::size_of::<u16>();

		let mut block = CoTaskMemAlloc(tot_bytes_ptrs + tot_bytes_wstrs)?; // alloc block for pointers and strings
		let ptr_block = block.as_mut_ptr() as *mut u8;
		let ptr_block_wstrs = unsafe { ptr_block.add(tot_bytes_ptrs) } as *mut u16; // start of strings block

		let mut idx_cur_wstr = 0; // current string to be copied to block
		*unsafe { block.as_mut_slice_aligned::<*mut u16>() }
			.get_mut(idx_cur_wstr).unwrap() = ptr_block_wstrs; // copy pointer to 1st string
		idx_cur_wstr += 1;

		wstrs.as_slice()
			.iter()
			.enumerate()
			.for_each(|(idx, ch)| {
				if *ch == 0x0000 && idx_cur_wstr < strs.len() {
					unsafe {
						*block.as_mut_slice_aligned::<*mut u16>() // copy pointer to subsequent strings in the block
							.get_mut(idx_cur_wstr).unwrap() = ptr_block_wstrs.add(idx + 1);
					}
					idx_cur_wstr += 1;
				}
			});

		*unsafe { block.as_mut_slice_aligned::<*mut u16>() }
			.get_mut(idx_cur_wstr).unwrap() = std::ptr::null_mut(); // null pointer after string pointers

		wstrs.copy_to_slice( // beyond pointers, copy each null-terminated string sequentially
			unsafe {
				std::slice::from_raw_parts_mut(
					ptr_block_wstrs,
					tot_bytes_wstrs / std::mem::size_of::<u16>(),
				)
			},
		);

		let (ptr, _) = block.leak();
		Ok(Self(ptr as _))
	}

	/// Returns the underlying pointer.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut *mut u16 {
		self.0
	}

	/// Converts the internal UTF-16 blocks into strings.
	#[must_use]
	pub fn to_strings(&self) -> Vec<String> {
		let mut vec = Vec::<String>::default();
		if !self.0.is_null() {
			let mut idx_ptr = 0;
			loop {
				let ptr_ws = unsafe {
					let sli_ptrs = std::slice::from_raw_parts(self.0, idx_ptr + 1);
					*sli_ptrs.get_unchecked(idx_ptr) // get nth pointer to string
				};
				if ptr_ws.is_null() { // a null pointer means the end of pointers block
					break;
				}
				let ws = unsafe { WString::from_wchars_nullt(ptr_ws) };
				vec.push(ws.to_string());
				idx_ptr += 1;
			}
		}
		vec
	}
}

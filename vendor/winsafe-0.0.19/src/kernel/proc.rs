use crate::decl::*;
use crate::kernel::ffi_types::*;

pub(in crate::kernel) extern "system" fn hinstance_enum_resource_languages<F>(
	_: HINSTANCE,
	_: *const u16,
	_: *const u16,
	language_id: u16,
	lparam: isize,
) -> BOOL
	where F: FnMut(LANGID) -> bool,
{
	let func = unsafe { &mut *(lparam as *mut F) };
	func(unsafe { LANGID::from_raw(language_id) }) as _
}

pub(in crate::kernel) extern "system" fn hinstance_enum_resource_names<F>(
	_: HINSTANCE,
	_: *const u16,
	resource_id: *mut u16,
	lparam: isize,
) -> BOOL
	where F: FnMut(IdStr) -> bool,
{
	let func = unsafe { &mut *(lparam as *mut F) };
	func(unsafe { IdStr::from_ptr(resource_id) }) as _
}

pub(in crate::kernel) extern "system" fn hinstance_enum_resource_types<F>(
	_: HINSTANCE,
	resource_type: *const u16,
	lparam: isize,
) -> BOOL
	where F: FnMut(RtStr) -> bool,
{
	let func = unsafe { &mut *(lparam as *mut F) };
	func(unsafe { RtStr::from_ptr(resource_type) }) as _
}

pub(in crate::kernel) extern "system" fn hservicestatus_register_service_ctrl_handler_ex<F>(
	control: u32,
	event_type: u32,
	event_data: PVOID,
	context: PVOID,
) -> u32
	where F: FnMut(SvcCtl) -> u32,
{
	let func = unsafe { &mut *(context as *mut F) };
	func(unsafe { SvcCtl::from_raw(control, event_type, event_data) })
}

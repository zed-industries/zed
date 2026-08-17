const_bitflag! { VT: u16;
	/// [`VARENUM`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ne-wtypes-varenum)
	/// enumeration (`u16`).
	=>
	=>
	/// Nothing.
	EMPTY 0
	/// SQL style NULL.
	NULL 1
	/// 2 byte signed int.
	I2 2
	/// 4 byte signed int.
	I4 3
	/// 4 byte real.
	R4 4
	/// 8 byte real.
	R8 5
	/// Currency.
	CY 6
	/// Date.
	DATE 7
	/// OLE Automation string.
	BSTR 8
	/// [`IDispatch`](crate::IDispatch) pointer.
	DISPATCH 9
	/// SCODE.
	ERROR 10
	/// True = -1, False = 0.
	BOOL 11
	/// VARIANT pointer.
	VARIANT 12
	/// [`IUnknown`](crate::IUnknown) pointer.
	UNKNOWN 13
	/// 16 byte fixed point.
	DECIMAL 14
	/// Signed char.
	I1 16
	/// Unsigned char.
	UI1 17
	/// Unsigned short.
	UI2 18
	/// ULONG.
	UI4 19
	/// Signed 64-bit int.
	I8 20
	/// Unsigned 64-bit int.
	UI8 21
	/// Signed machine int.
	INT 22
	/// Unsigned machine int.
	UINT 23
	/// C style void.
	VOID 24
	/// Standard return type.
	HRESULT 25
	/// Pointer type.
	PTR 26
	/// Use `VT::ARRAY` in `VARIANT`.
	SAFEARRAY 27
	/// C style array.
	CARRAY 28
	/// User defined type.
	USERDEFINED 29
	/// Null terminated string.
	LPSTR 30
	/// Wide null terminated string.
	LPWSTR 31
	/// User defined type.
	RECORD 36
	/// Signed machine register size width.
	INT_PTR 37
	/// Unsigned machine register size width.
	UINT_PTR 38
	/// [`FILETIME`](crate::FILETIME).
	FILETIME 64
	/// Length of prefixed bytes.
	BLOB 65
	/// Name of the stream follows.
	STREAM 66
	/// Name of the storage follows.
	STORAGE 67
	/// Stream contains an object.
	STREAMED_OBJECT 68
	/// Storage contains an object.
	STORED_OBJECT 69
	/// Blob contains an object.
	BLOB_OBJECT 70
	/// Clipboard format.
	CF 71
	/// A class ID.
	CLSID 72
	/// Stream with a GUID version.
	VERSIONED_STREAM 73
	/// Reserved for system use.
	BSTR_BLOB 0xfff
	/// Simple counted array.
	VECTOR 0x1000
	/// SAFEARRAY pointer.
	ARRAY 0x2000
	/// Void pointer for local use.
	BYREF 0x4000
	RESERVED 0x8000
	ILLEGAL 0xffff
	ILLEGALMASKED 0xfff
	TYPEMASK 0xfff
}

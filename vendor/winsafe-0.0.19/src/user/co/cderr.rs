#![allow(non_upper_case_globals)]

const_no_debug_display! { CDERR: u32;
	/// Common dialog box
	/// [error codes](https://learn.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-commdlgextendederror).
	///
	/// Also includes `PDERR`, `CFERR`, `FNERR` and `FRERR` prefixes.
	///
	/// Implements the standard [`Error`](std::error::Error) trait.
	///
	/// Does not implement [`FormattedError`](crate::prelude::FormattedError)
	/// because [`FormatMessage`](crate::FormatMessage) function does not offer
	/// support for it, so there is no way to obtain a textual description of
	/// the error codes.
}

impl std::error::Error for CDERR {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}

impl std::fmt::Display for CDERR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}] Common dialog error.", self.0)
	}
}
impl std::fmt::Debug for CDERR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.0 > 0xffff {
			write!(f, "CDERR({:#010x} {}) Common dialog error.", self.0, self.0)
		} else {
			write!(f, "CDERR({:#06x} {}) Common dialog error.", self.0, self.0)
		}
	}
}

const_values! { CDERR;
	=>
	/// None of the actual values (zero).
	NoValue 0
	DIALOGFAILURE 0xffff
	FINDRESFAILURE 0x0006
	INITIALIZATION 0x0002
	LOADRESFAILURE 0x0007
	LOADSTRFAILURE 0x0005
	LOCKRESFAILURE 0x0008
	MEMALLOCFAILURE 0x0009
	MEMLOCKFAILURE 0x000a
	NOHINSTANCE 0x0004
	NOHOOK 0x000b
	NOTEMPLATE 0x0003
	REGISTERMSGFAIL 0x000c
	STRUCTSIZE 0x0001
	PD_CREATEICFAILURE 0x100a
	PD_DEFAULTDIFFERENT 0x100c
	PD_DNDMMISMATCH 0x1009
	PD_GETDEVMODEFAIL 0x1005
	PD_INITFAILURE 0x1006
	PD_LOADDRVFAILURE 0x1004
	PD_NODEFAULTPRN 0x1008
	PD_NODEVICES 0x1007
	PD_PARSEFAILURE 0x1002
	PD_PRINTERNOTFOUND 0x100b
	PD_RETDEFFAILURE 0x1003
	PD_SETUPFAILURE 0x1001
	CF_MAXLESSTHANMIN 0x2002
	CF_NOFONTS 0x2001
	FN_BUFFERTOOSMALL 0x3003
	FN_INVALIDFILENAME 0x3002
	FN_SUBCLASSFAILURE 0x3001
	FR_BUFFERLENGTHZERO 0x4001
}

#![allow(non_camel_case_types, non_upper_case_globals)]

const_bitflag! { ACCESS_RIGHTS: u32;
	/// Standard access rights
	/// [flags](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)
	/// (`u32`).
	///
	/// Originally has no prefix.
	=>
	=>
	DELETE 0x0001_0000
	READ_CONTROL 0x0002_0000
	WRITE_DAC 0x0004_0000
	WRITE_OWNER 0x0008_0000
	SYNCHRONIZE 0x0010_0000
}

const_bitflag! { CLAIM_SECURITY_ATTRIBUTE: u16;
	/// [`CLAIM_SECURITY_ATTRIBUTE_V1`](crate::CLAIM_SECURITY_ATTRIBUTE_V1)
	/// `Flags` (`u16`);
	=>
	=>
	NON_INHERITABLE 0x0001
	VALUE_CASE_SENSITIVE 0x0002
	USE_FOR_DENY_ONLY 0x0004
	DISABLED_BY_DEFAULT 0x0008
	DISABLED 0x0010
	MANDATORY 0x0020
}

const_ordinary! { CLAIM_SECURITY_ATTRIBUTE_TYPE: u16;
	/// [`CLAIM_SECURITY_ATTRIBUTE_V1`](crate::CLAIM_SECURITY_ATTRIBUTE_V1)
	/// `ValueType` (`u16`);
	=>
	=>
	INT64 0x0001
	UINT64 0x0002
	STRING 0x0003
	FQBN 0x0004
	SID 0x0005
	BOOLEAN 0x0006
	OCTET_STRING 0x0010
}

const_bitflag! { CONSOLE: u32;
	/// [`HSTD::SetConsoleMode`](crate::prelude::kernel_Hstd::SetConsoleMode)
	/// `mode` (`u32`).
	///
	/// Originally has no prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ENABLE_PROCESSED_INPUT 0x0001
	ENABLE_LINE_INPUT 0x0002
	ENABLE_ECHO_INPUT 0x0004
	ENABLE_WINDOW_INPUT 0x0008
	ENABLE_MOUSE_INPUT 0x0010
	ENABLE_INSERT_MODE 0x0020
	ENABLE_QUICK_EDIT_MODE 0x0040
	ENABLE_EXTENDED_FLAGS 0x0080
	ENABLE_AUTO_POSITION 0x0100
	ENABLE_VIRTUAL_TERMINAL_INPUT 0x0200

	ENABLE_PROCESSED_OUTPUT 0x0001
	ENABLE_WRAP_AT_EOL_OUTPUT 0x0002
	ENABLE_VIRTUAL_TERMINAL_PROCESSING 0x0004
	DISABLE_NEWLINE_AUTO_RETURN 0x0008
	ENABLE_LVB_GRID_WORLDWIDE 0x0010
}

const_ordinary! { CP: u16;
	/// [`WideCharToMultiByte`](crate::WideCharToMultiByte) and
	/// [`MultiByteToWideChar`](crate::MultiByteToWideChar) `code_page`
	/// [identifiers](https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)
	/// (`u16`).
	///
	/// Originally these functions receive an `u32` parameter, but these are in
	/// fact `u16` constants.
	=>
	=>
	/// The system default Windows ANSI code page.
	ACP 0
	/// The current system OEM code page.
	OEMCP 1
	/// The current system Macintosh code page.
	MACCP 2
	/// The Windows ANSI code page for the current thread.
	THREAD_ACP 3
	/// Symbol code page (42).
	SYMBOL 42

	/// Unicode UTF-16 little endian byte order (BMP of ISO 10646); available
	/// only to managed applications.
	UTF16 1200
	/// Unicode UTF-16 big endian byte order; available only to managed
	/// applications.
	UNICODE_FFFE 1201
	/// ANSI Central European; Central European (Windows).
	WINDOWS_1250 1250
	/// ANSI Cyrillic; Cyrillic (Windows).
	WINDOWS_1251 1251
	/// ANSI Latin 1; Western European (Windows).
	WINDOWS_1252 1252
	/// ANSI Greek; Greek (Windows).
	WINDOWS_1253 1253
	/// ANSI Turkish; Turkish (Windows).
	WINDOWS_1254 1254
	/// ANSI Hebrew; Hebrew (Windows).
	WINDOWS_1255 1255
	/// ANSI Arabic; Arabic (Windows).
	WINDOWS_1256 1256
	/// ANSI Baltic; Baltic (Windows).
	WINDOWS_1257 1257
	/// ANSI/OEM Vietnamese; Vietnamese (Windows).
	WINDOWS_1258 1258
	/// Korean (Johab).
	JOHAB 1361
	/// MAC Roman; Western European (Mac).
	MACINTOSH 10000

	/// Unicode (UTF-7).
	///
	/// Conversion functions: use this value only when forced by a 7-bit
	/// transport mechanism. Use of UTF-8 is preferred. With this value set,
	/// `lpDefaultChar` and `lpUsedDefaultChar` must be set to null.
	UTF7 65000
	/// Unicode (UTF-8).
	///
	/// Conversion functions: with this value set `lpDefaultChar` and
	/// `lpUsedDefaultChar` must be set to null.
	UTF8 65001
}

const_bitflag! { CREATE: u32;
	/// Process creation
	/// [flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	BREAKAWAY_FROM_JOB 0x0100_0000
	DEFAULT_ERROR_MODE 0x0400_0000
	NEW_CONSOLE 0x0000_0010
	NEW_PROCESS_GROUP 0x0000_0200
	NO_WINDOW 0x0800_0000
	PROTECTED_PROCESS 0x0004_0000
	PRESERVE_CODE_AUTHZ_LEVEL 0x0200_0000
	SECURE_PROCESS 0x0040_0000
	SEPARATE_WOW_VDM 0x0000_0800
	SHARED_WOW_VDM 0x0000_1000
	SUSPENDED 0x0000_0004
	UNICODE_ENVIRONMENT 0x0000_0400
	/// Originally has no `CREATE` prefix.
	DEBUG_ONLY_THIS_PROCESS 0x0000_0002
	/// Originally has no `CREATE` prefix.
	DEBUG_PROCESS 0x0000_0001
	/// Originally has no `CREATE` prefix.
	DETACHED_PROCESS 0x0000_0008
	/// Originally has no `CREATE` prefix.
	EXTENDED_STARTUPINFO_PRESENT 0x0008_0000
	/// Originally has no `CREATE` prefix.
	INHERIT_PARENT_AFFINITY 0x0001_0000
}

const_bitflag! { CREATE_EVENT: u32;
	/// [`HEVENT::CreateEventEx`](crate::prelude::kernel_Hevent::CreateEventEx)
	/// `flags` (`u32`).
	=>
	=>
	INITIAL_SET 0x0000_0002
	MANUAL_RESET 0x0000_0001
}

const_ordinary! { DBT: u16;
	/// [`wm::DeviceChange`](crate::msg::wm::DeviceChange) event (`u16`).
	=>
	=>
	DEVNODES_CHANGED 0x0007
	QUERYCHANGECONFIG 0x0017
	CONFIGCHANGED 0x0018
	CONFIGCHANGECANCELED 0x0019
	DEVICEARRIVAL 0x8000
	DEVICEQUERYREMOVE 0x8001
	DEVICEQUERYREMOVEFAILED 0x8002
	DEVICEREMOVEPENDING 0x8003
	DEVICEREMOVECOMPLETE 0x8004
	DEVICETYPESPECIFIC 0x8005
	CUSTOMEVENT 0x8006
	USERDEFINED 0xffff
}

const_ordinary! { DBT_DEVTYP: u32;
	/// [`DEV_BROADCAST_HDR`](crate::DEV_BROADCAST_HDR) `dbch_devicetype`
	/// (`u32`).
	=>
	=>
	DEVICEINTERFACE 0x0000_0005
	HANDLE 0x0000_0006
	OEM 0x0000_0000
	PORT 0x0000_0003
	VOLUME 0x0000_0002
}

const_bitflag! { DBTF: u16;
	/// [`DEV_BROADCAST_VOLUME`](crate::DEV_BROADCAST_VOLUME) `dbcv_flags`
	/// (`u16`).
	=>
	=>
	MEDIA 0x0001
	NET 0x0002
}

const_ordinary! { DISPOSITION: u32;
	/// [`HFILE::CreateFile`](crate::prelude::kernel_Hfile::CreateFile)
	/// `creation_disposition` (`u32`).
	///
	/// Originally has no prefix.
	=>
	=>
	/// Creates a new file only if it does not already exist.
	///
	/// If the specified file exists the function fails and the last-error code
	/// is set to [`ERROR::FILE_EXISTS`](crate::co::ERROR::FILE_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location a new file is created.
	CREATE_NEW 1
	/// Creates a new file always.
	///
	/// If the specified file exists and is writable the function overwrites
	/// the file the function succeeds and last-error code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path a new file is
	/// created the function succeeds and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	CREATE_ALWAYS 2
	/// Opens a file or device only if it exists.
	///
	/// If the specified file or device does not exist the function fails and
	/// the last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	OPEN_EXISTING 3
	/// Opens a file always.
	///
	/// If the specified file exists the function succeeds and the last-error
	/// code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location the function creates a file and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	OPEN_ALWAYS 4
	/// Opens a file and truncates it so that its size is zero bytes only if it
	/// exists.
	///
	/// If the specified file does not exist the function fails and the
	/// last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	///
	/// The calling process must open the file with the
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) bit set as part of the
	/// `dwDesiredAccess` parameter.
	TRUNCATE_EXISTING 5
}

const_ordinary! { DRIVE: u32;
	/// [`GetDriveType`](crate::GetDriveType) return type (`u32`).
	=>
	=>
	UNKNOWN 0
	NO_ROOT_DIR 1
	REMOVABLE 2
	FIXED 3
	REMOTE 4
	CDROM 5
	RAMDISK 6
}

const_ordinary! { EVENTLOG: u16;
	/// [`HEVENTLOG::ReportEvent`](crate::prelude::kernel_Heventlog::ReportEvent)
	/// `event_type` [`u16`].
	=>
	=>
	SUCCESS 0x0000
	AUDIT_FAILURE 0x0010
	AUDIT_SUCCESS 0x0008
	ERROR_TYPE 0x0001
	INFORMATION_TYPE 0x0004
	WARNING_TYPE 0x0002
}

const_bitflag! { EVENT_RIGHTS: u32;
	/// Event
	/// [access rights](https://learn.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
	/// (`u32`).
	=>
	=>
	DELETE ACCESS_RIGHTS::DELETE.0
	READ_CONTROL ACCESS_RIGHTS::READ_CONTROL.0
	SYNCHRONIZE ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE_DAC ACCESS_RIGHTS::WRITE_DAC.0
	WRITE_OWNER ACCESS_RIGHTS::WRITE_OWNER.0

	ALL_ACCESS 0x1f_0003
	MODIFY_STATE 0x0002
}

const_bitflag! { FILE_ATTRIBUTE: u32;
	/// File
	/// [attributes](https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants)
	/// (`u32`).
	=>
	=>
	READONLY 0x0000_0001
	HIDDEN 0x0000_0002
	SYSTEM 0x0000_0004
	DIRECTORY 0x0000_0010
	ARCHIVE 0x0000_0020
	DEVICE 0x0000_0040
	NORMAL 0x0000_0080
	TEMPORARY 0x0000_0100
	SPARSE_FILE 0x0000_0200
	REPARSE_POINT 0x0000_0400
	COMPRESSED 0x0000_0800
	OFFLINE 0x0000_1000
	NOT_CONTENT_INDEXED 0x0000_2000
	ENCRYPTED 0x0000_4000
	INTEGRITY_STREAM 0x0000_8000
	VIRTUAL 0x0001_0000
	NO_SCRUB_DATA 0x0002_0000
	EA 0x0004_0000
	PINNED 0x0008_0000
	UNPINNED 0x0010_0000
	RECALL_ON_OPEN 0x0004_0000
	RECALL_ON_DATA_ACCESS 0x0040_0000
}

const_bitflag! { FILE_CACHE: u32;
	/// [`GetSystemFileCacheSize`](crate::GetSystemFileCacheSize) returned flags
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	MAX_HARD_ENABLE 0x1
	MIN_HARD_ENABLE 0x4
}

const_bitflag! { FILE_FLAG: u32;
	/// [`HFILE::CreateFile`](crate::prelude::kernel_Hfile::CreateFile) `flags`
	/// (`u32`).
	=>
	=>
	BACKUP_SEMANTICS 0x0200_0000
	DELETE_ON_CLOSE 0x0400_0000
	NO_BUFFERING 0x2000_0000
	OPEN_NO_RECALL 0x0010_0000
	OPEN_REPARSE_POINT 0x0020_0000
	OVERLAPPED 0x4000_0000
	POSIX_SEMANTICS 0x0100_0000
	RANDOM_ACCESS 0x1000_0000
	SESSION_AWARE 0x0080_0000
	SEQUENTIAL_SCAN 0x0800_0000
	WRITE_THROUGH 0x8000_0000
}

const_bitflag! { FILE_MAP: u32;
	/// [`HFILEMAP::MapViewOfFile`](crate::prelude::kernel_Hfilemap::MapViewOfFile)
	/// `desired_access` (`u32`).
	=>
	=>
	ALL_ACCESS SECTION::ALL_ACCESS.0
	READ SECTION::MAP_READ.0
	WRITE SECTION::MAP_WRITE.0

	COPY 0x0000_0001
	EXECUTE SECTION::MAP_EXECUTE_EXPLICIT.0
	LARGE_PAGES 0x2000_0000
	TARGETS_INVALID 0x4000_0000
}

const_bitflag! { FILE_SHARE: u32;
	/// [`HFILE::CreateFile`](crate::prelude::kernel_Hfile::CreateFile)
	/// `share_mode` (`u32`).
	=>
	=>
	READ 0x0000_0001
	WRITE 0x0000_0002
	DELETE 0x0000_0004
}

const_bitflag! { FILE_SECURITY: u32;
	/// [`HFILE::CreateFile`](crate::prelude::kernel_Hfile::CreateFile)
	/// `security` (`u32`).
	///
	/// Originally has `SECURITY` prefix.
	=>
	=>
	ANONYMOUS 0 << 16
	IDENTIFICATION 1 << 16
	IMPERSONATION 2 << 16
	DELEGATION 3 << 16
	CONTEXT_TRACKING 0x0004_0000
	EFFECTIVE_ONLY 0x0008_0000
}

const_ordinary! { FILE_STARTING_POINT: u32;
	/// [`HFILE::SetFilePointerEx`](crate::prelude::kernel_Hfile::SetFilePointerEx)
	/// `move_method` (`u32`).
	///
	/// Originally has `FILE` prefix.
	=>
	=>
	/// The starting point is zero or the beginning of the file. If this flag is
	/// specified then the `liDistanceToMove` parameter is interpreted as an
	/// unsigned value.
	BEGIN 0
	/// The start point is the current value of the file pointer.
	CURRENT 1
	/// The starting point is the current end-of-file position.
	END 2
}

const_ordinary! { FILE_TYPE: u32;
	/// [`HFILE::GetFileType`](crate::prelude::kernel_Hfile::GetFileType) return
	/// value (`u32`).
	=>
	=>
	/// The specified file is a character file typically an LPT device or a
	/// console.
	CHAR 0x0002
	/// The specified file is a disk file.
	DISK 0x0001
	/// The specified file is a socket a named pipe or an anonymous pipe.
	PIPE 0x0003
	/// Unused.
	REMOTE 0x8000
	/// Either the type of the specified file is unknown or the function
	/// failed.
	UNKNOWN 0x0000
}

const_bitflag! { FILE_VOL: u32;
	/// [`GetVolumeInformation`](crate::GetVolumeInformation) `flags` (`u32`).
	///
	/// Originally has `FILE` prefix.
	=>
	=>
	CASE_PRESERVED_NAMES 0x0000_0002
	CASE_SENSITIVE_SEARCH 0x0000_0001
	DAX_VOLUME 0x2000_0000
	FILE_COMPRESSION 0x0000_0010
	NAMED_STREAMS 0x0004_0000
	PERSISTENT_ACLS 0x0000_0008
	READ_ONLY_VOLUME 0x0008_0000
	SEQUENTIAL_WRITE_ONCE 0x0010_0000
	SUPPORTS_ENCRYPTION 0x0002_0000
	SUPPORTS_EXTENDED_ATTRIBUTES 0x0080_0000
	SUPPORTS_HARD_LINKS 0x0040_0000
	SUPPORTS_OBJECT_IDS 0x0001_0000
	SUPPORTS_OPEN_BY_FILE_ID 0x0100_0000
	SUPPORTS_REPARSE_POINTS 0x0000_0080
	SUPPORTS_SPARSE_FILES 0x0000_0040
	SUPPORTS_TRANSACTIONS 0x0020_0000
	SUPPORTS_USN_JOURNAL 0x0200_0000
	UNICODE_ON_DISK 0x0000_0004
	VOLUME_IS_COMPRESSED 0x0000_8000
	VOLUME_QUOTAS 0x0000_0020
	SUPPORTS_BLOCK_REFCOUNTING 0x0800_0000
}

const_ordinary! { FIRMWARE_TYPE: u32;
	/// [`FIRMWARE_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-firmware_type)
	/// enumeration (`u32`).
	=>
	=>
	Unknown 0
	Bios 1
	Uefi 2
}

const_bitflag! { FORMAT_MESSAGE: u32;
	/// [`FormatMessage`](crate::prelude::FormattedError::FormatMessage)
	/// `dwFlags` (`u32`).
	=>
	=>
	ALLOCATE_BUFFER 0x0000_0100
	ARGUMENT_ARRAY 0x0000_2000
	FROM_HMODULE 0x0000_0800
	FROM_STRING 0x0000_0400
	FROM_SYSTEM 0x0000_1000
	IGNORE_INSERTS 0x0000_0200
	MAX_WIDTH_MASK 0x0000_00ff
}

const_bitflag! { GENERIC: u32;
	/// Generic access rights
	/// [flags](https://learn.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights)
	/// (`u32`).
	=>
	=>
	/// Read access.
	READ 0x8000_0000
	/// Write access.
	WRITE 0x4000_0000
	/// Execute access.
	EXECUTE 0x2000_0000
	/// All possible access rights.
	ALL 0x1000_0000
}

const_bitflag! { GMEM: u32;
	/// [`HGLOBAL::GlobalAlloc`](crate::prelude::kernel_Hglobal::GlobalAlloc)
	/// and
	/// [`HGLOBAL::GlobalReAlloc`](crate::prelude::kernel_Hglobal::GlobalReAlloc)
	/// `flags` (`u32`).
	=>
	=>
	/// Allocates fixed memory. The return value is a pointer to the memory
	/// object.
	FIXED 0x0000
	/// Allocates movable memory. Memory blocks are never moved in physical
	/// memory, but they can be moved within the default heap.
	///
	/// The return value is a handle to the memory object. To translate the
	/// handle to a pointer, use the
	/// [`LocalLock`](crate::prelude::kernel_Hlocal::LocalLock) function.
	MOVEABLE 0x0002
	/// Initializes memory contents to zero.
	ZEROINIT 0x0040
	/// Combines `MOVEABLE` and `ZEROINIT`.
	GHND Self::MOVEABLE.0 | Self::ZEROINIT.0
	/// Combines `FIXED` and `ZEROINIT`.
	GPTR Self::FIXED.0 | Self::ZEROINIT.0
}

const_bitflag! { GR: u32;
	/// [`HPROCESS::GetGuiResources`](crate::prelude::kernel_Hprocess::GetGuiResources)
	/// `flags` (`u32`).
	=>
	=>
	GDIOBJECTS 0
	GDIOBJECTS_PEAK 2
	USEROBJECTS 1
	USEROBJECTS_PEAK 4
}

const_bitflag! { HEAP_ALLOC: u32;
	/// [`HHEAP::HeapAlloc`](crate::prelude::kernel_Hheap::HeapAlloc) `flags`
	/// (`u32`).
	///
	/// Originally has `HEAP` prefix.
	=>
	=>
	GENERATE_EXCEPTIONS 0x0000_0004
	NO_SERIALIZE 0x0000_0001
	ZERO_MEMORY 0x0000_0008
}

const_bitflag! { HEAP_CREATE: u32;
	/// [`HHEAP::HeapCreate`](crate::prelude::kernel_Hheap::HeapCreate)
	/// `options` (`u32`).
	///
	/// Originally has `HEAP` prefix.
	=>
	=>
	ENABLE_EXECUTE 0x0004_0000
	GENERATE_EXCEPTIONS 0x0000_0004
	NO_SERIALIZE 0x0000_0001
}

const_ordinary! { HEAP_INFORMATION: u32;
	/// [`HHEAP::HeapSetInformation`](crate::prelude::kernel_Hheap::HeapSetInformation)
	/// information class (`u32`).
	=>
	=>
	CompatibilityInformation 0
	EnableTerminationOnCorruption 1
	OptimizeResources 3
}

const_bitflag! { HEAP_REALLOC: u32;
	/// [`HHEAP::HeapReAlloc`](crate::prelude::kernel_Hheap::HeapReAlloc)
	/// `options` (`u32`).
	///
	/// Originally has `HEAP` prefix.
	=>
	=>
	GENERATE_EXCEPTIONS 0x0000_0004
	NO_SERIALIZE 0x0000_0001
	REALLOC_IN_PLACE_ONLY 0x0000_0010
	ZERO_MEMORY 0x0000_0008
}

const_bitflag! { HEAP_SIZE: u32;
	/// [`HHEAP::HeapSize`](crate::prelude::kernel_Hheap::HeapSize) and
	/// [`HHEAP::HeapCompact`](crate::prelude::kernel_Hheap::HeapCompact)
	/// `options` (`u32`).
	///
	/// Originally has `HEAP` prefix.
	=>
	=>
	NO_SERIALIZE 0x0000_0001
}

const_ordinary! { HF32: u32;
	/// [`HEAPLIST32`](crate::HEAPLIST32) `dwFlags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Default heap of the process.
	DEFAULT 1
}

const_bitflag! { KEY: u32;
	/// [Registry access rights](https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-key-security-and-access-rights)
	/// (`u32`).
	=>
	=>
	QUERY_VALUE 0x0001
	SET_VALUE 0x0002
	CREATE_SUB_KEY 0x0004
	ENUMERATE_SUB_KEYS 0x0008
	NOTIFY 0x0010
	CREATE_LINK 0x0020
	WOW64_32KEY 0x0200
	WOW64_64KEY 0x0100
	WOW64_RES 0x0300
	READ (STANDARD_RIGHTS::READ.0 | Self::QUERY_VALUE.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE (STANDARD_RIGHTS::WRITE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	EXECUTE Self::READ.0 & !ACCESS_RIGHTS::SYNCHRONIZE.0
	ALL_ACCESS (STANDARD_RIGHTS::ALL.0 | Self::QUERY_VALUE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0 | Self::CREATE_LINK.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
}

const_ordinary! { LANG: u16;
	/// Language
	/// [identifier](https://learn.microsoft.com/en-us/windows/win32/intl/language-identifier-constants-and-strings)
	/// (`u16`).
	=>
	=>
	NEUTRAL 0x00
	INVARIANT 0x7f
	AFRIKAANS 0x36
	ALBANIAN 0x1c
	ALSATIAN 0x84
	AMHARIC 0x5e
	ARABIC 0x01
	ARMENIAN 0x2b
	ASSAMESE 0x4d
	AZERI 0x2c
	AZERBAIJANI 0x2c
	BANGLA 0x45
	BASHKIR 0x6d
	BASQUE 0x2d
	BELARUSIAN 0x23
	BENGALI 0x45
	BRETON 0x7e
	BOSNIAN 0x1a
	BOSNIAN_NEUTRAL 0x781a
	BULGARIAN 0x02
	CATALAN 0x03
	CENTRAL_KURDISH 0x92
	CHEROKEE 0x5c
	CHINESE 0x04
	CHINESE_SIMPLIFIED 0x04
	CHINESE_TRADITIONAL 0x7c04
	CORSICAN 0x83
	CROATIAN 0x1a
	CZECH 0x05
	DANISH 0x06
	DARI 0x8c
	DIVEHI 0x65
	DUTCH 0x13
	ENGLISH 0x09
	ESTONIAN 0x25
	FAEROESE 0x38
	FARSI 0x29
	FILIPINO 0x64
	FINNISH 0x0b
	FRENCH 0x0c
	FRISIAN 0x62
	FULAH 0x67
	GALICIAN 0x56
	GEORGIAN 0x37
	GERMAN 0x07
	GREEK 0x08
	GREENLANDIC 0x6f
	GUJARATI 0x47
	HAUSA 0x68
	HAWAIIAN 0x75
	HEBREW 0x0d
	HINDI 0x39
	HUNGARIAN 0x0e
	ICELANDIC 0x0f
	IGBO 0x70
	INDONESIAN 0x21
	INUKTITUT 0x5d
	IRISH 0x3c
	ITALIAN 0x10
	JAPANESE 0x11
	KANNADA 0x4b
	KASHMIRI 0x60
	KAZAK 0x3f
	KHMER 0x53
	KICHE 0x86
	KINYARWANDA 0x87
	KONKANI 0x57
	KOREAN 0x12
	KYRGYZ 0x40
	LAO 0x54
	LATVIAN 0x26
	LITHUANIAN 0x27
	LOWER_SORBIAN 0x2e
	LUXEMBOURGISH 0x6e
	MACEDONIAN 0x2f
	MALAY 0x3e
	MALAYALAM 0x4c
	MALTESE 0x3a
	MANIPURI 0x58
	MAORI 0x81
	MAPUDUNGUN 0x7a
	MARATHI 0x4e
	MOHAWK 0x7c
	MONGOLIAN 0x50
	NEPALI 0x61
	NORWEGIAN 0x14
	OCCITAN 0x82
	ODIA 0x48
	ORIYA 0x48
	PASHTO 0x63
	PERSIAN 0x29
	POLISH 0x15
	PORTUGUESE 0x16
	PULAR 0x67
	PUNJABI 0x46
	QUECHUA 0x6b
	ROMANIAN 0x18
	ROMANSH 0x17
	RUSSIAN 0x19
	SAKHA 0x85
	SAMI 0x3b
	SANSKRIT 0x4f
	SCOTTISH_GAELIC 0x91
	SERBIAN 0x1a
	SERBIAN_NEUTRAL 0x7c1a
	SINDHI 0x59
	SINHALESE 0x5b
	SLOVAK 0x1b
	SLOVENIAN 0x24
	SOTHO 0x6c
	SPANISH 0x0a
	SWAHILI 0x41
	SWEDISH 0x1d
	SYRIAC 0x5a
	TAJIK 0x28
	TAMAZIGHT 0x5f
	TAMIL 0x49
	TATAR 0x44
	TELUGU 0x4a
	THAI 0x1e
	TIBETAN 0x51
	TIGRIGNA 0x73
	TIGRINYA 0x73
	TSWANA 0x32
	TURKISH 0x1f
	TURKMEN 0x42
	UIGHUR 0x80
	UKRAINIAN 0x22
	UPPER_SORBIAN 0x2e
	URDU 0x20
	UZBEK 0x43
	VALENCIAN 0x03
	VIETNAMESE 0x2a
	WELSH 0x52
	WOLOF 0x88
	XHOSA 0x34
	YAKUT 0x85
	YI 0x78
	YORUBA 0x6a
	ZULU 0x35
}

const_bitflag! { LMEM: u32;
	/// [`HLOCAL::LocalAlloc`](crate::prelude::kernel_Hlocal::LocalAlloc) and
	/// [`HLOCAL::LocalReAlloc`](crate::prelude::kernel_Hlocal::LocalReAlloc)
	/// `flags` (`u32`).
	=>
	=>
	/// Allocates fixed memory. The return value is a pointer to the memory
	/// object.
	FIXED 0x0000
	/// Allocates movable memory. Memory blocks are never moved in physical
	/// memory, but they can be moved within the default heap.
	///
	/// The return value is a handle to the memory object. To translate the
	/// handle to a pointer, use the
	/// [`LocalLock`](crate::prelude::kernel_Hlocal::LocalLock) function.
	MOVEABLE 0x0002
	/// Initializes memory contents to zero.
	ZEROINIT 0x0040
	/// Combines `MOVEABLE` and `ZEROINIT`.
	LHND Self::MOVEABLE.0 | Self::ZEROINIT.0
	/// Combines `FIXED` and `ZEROINIT`.
	LPTR Self::FIXED.0 | Self::ZEROINIT.0
}

const_bitflag! { MBC: u32;
	/// [`MultiByteToWideChar`](crate::MultiByteToWideChar) `flags` (`u32`).
	///
	/// Originally has `MB` prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	COMPOSITE 0x0000_0002
	ERR_INVALID_CHARS 0x0000_0008
	PRECOMPOSED 0x0000_0001
	USEGLYPHCHARS 0x0000_0004
}

const_ordinary! { MONITOR_DISPLAY_STATE: u32;
	/// [`MONITOR_DISPLAY_STATE`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ne-wdm-_monitor_display_state)
	/// enumeration (`u32`).
	=>
	=>
	Off 0
	On 1
	Dim 2
}

const_ordinary! { PAGE: u32;
	/// [`HFILE::CreateFileMapping`](crate::prelude::kernel_Hfile::CreateFileMapping)
	/// `protect` (`u32`).
	=>
	=>
	/// Allows views to be mapped for read-only copy-on-write or execute
	/// access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_READ 0x20
	/// Allows views to be mapped for read-only copy-on-write read/write or
	/// execute access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ),
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_READWRITE 0x40
	/// Allows views to be mapped for read-only copy-on-write or execute
	/// access. This value is equivalent to PAGE_EXECUTE_READ.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_WRITECOPY 0x80
	/// Allows views to be mapped for read-only or copy-on-write access. An
	/// attempt to write to a specific region results in an access violation.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) access right.
	READONLY 0x02
	/// Allows views to be mapped for read-only copy-on-write or read/write
	/// access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) access rights.
	READWRITE 0x04
	/// Allows views to be mapped for read-only or copy-on-write access. This
	/// value is equivalent to `PAGE::READONLY`.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) access right.
	WRITECOPY 0x08

	SEC_COMMIT 0x800_0000
	SEC_IMAGE 0x100_0000
	SEC_IMAGE_NO_EXECUTE 0x1100_0000
	SEC_LARGE_PAGES 0x8000_0000
	SEC_NOCACHE 0x1000_0000
	SEC_RESERVE 0x400_0000
	SEC_WRITECOMBINE 0x4000_0000
}

const_ordinary! { PBT: u32;
	/// [`wm::PowerBroadcast`](crate::msg::wm::PowerBroadcast) power-management
	/// event (`u32`).
	=>
	=>
	APMPOWERSTATUSCHANGE 0xa
	APMRESUMEAUTOMATIC 0x12
	APMRESUMESUSPEND 0x7
	APMSUSPEND 0x4
	POWERSETTINGCHANGE 0x8013
}

const_bitflag! { PRIORITY_CLASS: u32;
	/// [`GetPriorityClass`](crate::prelude::kernel_Hprocess::GetPriorityClass)
	/// and
	/// [`SetPriorityClass`](crate::prelude::kernel_Hprocess::SetPriorityClass)
	/// `priority_class` (`u32`).
	///
	/// Originally has `PRIORITY_CLASS` suffix.
	=>
	=>
	ABOVE_NORMAL 0x0000_8000
	BELOW_NORMAL 0x0000_4000
	HIGH 0x0000_0080
	IDLE 0x0000_0040
	NORMAL 0x0000_0020
	REALTIME 0x0000_0100
}

const_bitflag! { PROCESS: u32;
	/// Process
	/// [security and access rights](https://learn.microsoft.com/en-us/windows/win32/procthread/process-security-and-access-rights)
	/// (`u32`).
	=>
	=>
	DELETE ACCESS_RIGHTS::DELETE.0
	READ_CONTROL ACCESS_RIGHTS::READ_CONTROL.0
	SYNCHRONIZE ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE_DAC ACCESS_RIGHTS::WRITE_DAC.0
	WRITE_OWNER ACCESS_RIGHTS::WRITE_OWNER.0

	ALL_ACCESS STANDARD_RIGHTS::REQUIRED.0 | ACCESS_RIGHTS::SYNCHRONIZE.0 | 0xffff
	CREATE_PROCESS 0x0080
	CREATE_THREAD 0x0002
	DUP_HANDLE 0x0040
	QUERY_INFORMATION 0x0400
	QUERY_LIMITED_INFORMATION 0x1000
	SET_INFORMATION 0x0200
	SET_QUOTA 0x0100
	SUSPEND_RESUME 0x0800
	TERMINATE 0x0001
	WM_OPERATION 0x0008
	VM_READ 0x0010
	VM_WRITE 0x0020
}

const_bitflag! { PROCESS_AFFINITY: u32;
	/// [`QueryProcessAffinityUpdateMode`](crate::prelude::kernel_Hprocess::QueryProcessAffinityUpdateMode)
	/// and
	/// [`SetProcessAffinityUpdateMode`](crate::prelude::kernel_Hprocess::SetProcessAffinityUpdateMode)
	/// `flags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ENABLE_AUTO_UPDATE 0x0000_0001
}

const_bitflag! { PROCESS_HEAP: u16;
	/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) `wFlags` (`u16`).
	=>
	=>
	ENTRY_BUSY 0x0004
	ENTRY_DDESHARE 0x0020
	ENTRY_MOVEABLE 0x0010
	REGION 0x0001
	UNCOMMITTED_RANGE 0x0002
}

const_ordinary! { PROCESS_NAME: u32;
	/// [`QueryFullProcessImageName`](crate::prelude::kernel_Hprocess::QueryFullProcessImageName)
	/// flags (`u32`).
	=>
	=>
	/// The name should use the Win32 path format.
	WIN32 0
	/// The name should use the native system path format.
	NATIVE 0x0000_0001
}

const_ordinary! { PROCESSOR: u32;
	/// [`SYSTEM_INFO`](crate::SYSTEM_INFO) `dwProcessorType` (`u32`).
	=>
	=>
	INTEL_386 386
	INTEL_486 486
	INTEL_PENTIUM 586
	INTEL_IA64 2200
	AMD_X8664 8664
	MIPS_R4000 4000
	ALPHA_21064 21064
	PPC_601 601
	PPC_603 603
	PPC_604 604
	PPC_620 620
	HITACHI_SH3 10003
	HITACHI_SH3E 10004
	HITACHI_SH4 10005
	MOTOROLA_821 821
	SHx_SH3 103
	SHx_SH4 104
	STRONGARM 2577
	ARM720 1824
	ARM820 2080
	ARM920 2336
	ARM_7TDMI 70001
	OPTIL 0x494f
}

const_ordinary! { PROCESSOR_ARCHITECTURE: u16;
	/// [`SYSTEM_INFO`](crate::SYSTEM_INFO) `wProcessorArchitecture` (`u16`).
	=>
	=>
	INTEL 0
	MIPS 1
	ALPHA 2
	PPC 3
	SHX 4
	ARM 5
	IA64 6
	ALPHA64 7
	MSIL 8
	AMD64 9
	IA32_ON_WIN64 10
	NEUTRAL 11
	ARM64 12
	ARM32_ON_WIN64 13
	IA32_ON_ARM64 14
	UNKNOWN 0xffff
}

const_ordinary! { REG: u32;
	/// Registry
	/// [value types](https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types)
	/// (`u32`).
	=>
	=>
	NONE 0
	SZ 1
	EXPAND_SZ 2
	BINARY 3
	DWORD 4
	DWORD_LITTLE_ENDIAN 4
	DWORD_BIG_ENDIAN 5
	LINK 6
	MULTI_SZ 7
	RESOURCE_LIST 8
	FULL_RESOURCE_DESCRIPTOR 9
	RESOURCE_REQUIREMENTS_LIST 10
	QWORD 11
	QWORD_LITTLE_ENDIAN 11
}

const_ordinary! { REG_DISPOSITION: u32;
	/// [`HKEY::RegCreateKeyEx`](crate::prelude::kernel_Hkey::RegCreateKeyEx)
	/// creation disposition (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// The key did not exist and was created.
	CREATED_NEW_KEY 0x0000_0001
	/// The key existed and was simply opened without being changed.
	OPENED_EXISTING_KEY 0x0000_0002
}

const_bitflag! { REG_OPTION: u32;
	/// [`HKEY::RegOpenKeyEx`](crate::prelude::kernel_Hkey::RegOpenKeyEx)
	/// `options` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	RESERVED 0x0000_0000
	NON_VOLATILE 0x0000_0000
	VOLATILE 0x0000_0001
	CREATE_LINK 0x0000_0002
	BACKUP_RESTORE 0x0000_0004
	OPEN_LINK 0x0000_0008
}

const_ordinary! { REG_RESTORE: u32;
	/// Registry restore
	/// [`flags`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrestorekeyw)
	/// (`u32`).
	///
	/// Originally has `REG` prefix.
	=>
	=>
	FORCE_RESTORE 0x0000_0008
	WHOLE_HIVE_VOLATILE 0x0000_0001
}

const_ordinary! { REG_SAVE: u32;
	/// Registry save
	/// [`flags`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyexw)
	/// (`u32`).
	///
	/// Originally has `REG` prefix.
	=>
	=>
	STANDARD_FORMAT 1
	LATEST_FORMAT 2
	NO_COMPRESSION 4
}

const_bitflag! { REPLACEFILE: u32;
	/// [`ReplaceFile`](crate::ReplaceFile) `flags` (`u32`).
	=>
	=>
	WRITE_THROUGH 0x0000_0001
	IGNORE_MERGE_ERRORS 0x0000_0002
	IGNORE_ACL_ERRORS 0x0000_0004
}

const_ordinary! { RID: u32;
	/// The
	/// [portion](https://learn.microsoft.com/en-us/windows/win32/secgloss/r-gly)
	/// of a [`SID`](crate::SID) that identifies a user or group in relation to
	/// the authority that issued the `SID`.
	///
	/// Originally has `RID` suffix.
	=>
	=>
	SECURITY_NULL 0x0000_0000
	SECURITY_WORLD 0x0000_0000
	SECURITY_LOCAL 0x0000_0000
	SECURITY_LOCAL_LOGON 0x0000_0001
	SECURITY_CREATOR_OWNER 0x0000_0000
	SECURITY_CREATOR_GROUP 0x0000_0001
	SECURITY_CREATOR_OWNER_SERVER 0x0000_0002
	SECURITY_CREATOR_GROUP_SERVER 0x0000_0003
	SECURITY_CREATOR_OWNER_RIGHTS 0x0000_0004

	SECURITY_DIALUP 0x0000_0001
	SECURITY_NETWORK 0x0000_0002
	SECURITY_BATCH 0x0000_0003
	SECURITY_INTERACTIVE 0x0000_0004
	SECURITY_LOGON_IDS 0x0000_0005
	SECURITY_SERVICE 0x0000_0006
	SECURITY_ANONYMOUS_LOGON 0x0000_0007
	SECURITY_PROXY 0x0000_0008
	SECURITY_ENTERPRISE_CONTROLLERS 0x00000_009
	SECURITY_SERVER_LOGON Self::SECURITY_ENTERPRISE_CONTROLLERS.0
	SECURITY_PRINCIPAL_SELF 0x0000_000a
	SECURITY_AUTHENTICATED_USER 0x0000_000b
	SECURITY_RESTRICTED_CODE 0x0000_000c
	SECURITY_TERMINAL_SERVER 0x0000_000d
	SECURITY_REMOTE_LOGON 0x0000_000e
	SECURITY_THIS_ORGANIZATION 0x0000_000f
	SECURITY_IUSER 0x0000_0011
	SECURITY_LOCAL_SYSTEM 0x0000_0012
	SECURITY_LOCAL_SERVICE 0x0000_0013
	SECURITY_NETWORK_SERVICE 0x0000_0014
	SECURITY_NT_NON_UNIQUE 0x0000_0015
	SECURITY_ENTERPRISE_READONLY_CONTROLLERS 0x0000_0016
	SECURITY_BUILTIN_DOMAIN 0x0000_0020
	SECURITY_WRITE_RESTRICTED_CODE 0x0000_0021
	SECURITY_PACKAGE_BASE 0x0000_0040
	SECURITY_PACKAGE_NTLM 0x0000_000a
	SECURITY_PACKAGE_SCHANNEL 0x0000_000e
	SECURITY_PACKAGE_DIGEST 0x0000_0015
	SECURITY_CRED_TYPE_BASE 0x0000_0041
	SECURITY_CRED_TYPE_THIS_ORG_CERT 0x0000_0001
	SECURITY_MIN_BASE 0x0000_0050
	SECURITY_SERVICE_ID_BASE 0x0000_0050
	SECURITY_RESERVED_ID_BASE 0x0000_0051
	SECURITY_APPPOOL_ID_BASE 0x0000_0052
	SECURITY_VIRTUALSERVER_ID_BASE 0x0000_0053
	SECURITY_USERMODEDRIVERHOST_ID_BASE 0x0000_0054
	SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_BASE 0x0000_0055
	SECURITY_WMIHOST_ID_BASE 0x0000_0056
	SECURITY_TASK_ID_BASE 0x0000_0057
	SECURITY_NFS_ID_BASE 0x0000_0058
	SECURITY_COM_ID_BASE 0x0000_0059
	SECURITY_WINDOW_MANAGER_BASE 0x0000_005a
	SECURITY_RDV_GFX_BASE 0x0000_005b
	SECURITY_DASHOST_ID_BASE 0x0000_005c
	SECURITY_USERMANAGER_ID_BASE 0x0000_005d
	SECURITY_WINRM_ID_BASE 0x0000_005e
	SECURITY_CCG_ID_BASE 0x0000_005f
	SECURITY_UMFD_BASE 0x0000_0060
	SECURITY_MAX_BASE 0x0000_006f
	SECURITY_MAX_ALWAYS_FILTERED 0x0000_03e7
	SECURITY_MIN_NEVER_FILTERED 0x0000_03e8
	SECURITY_OTHER_ORGANIZATION 0x0000_03e8
	SECURITY_WINDOWSMOBILE_ID_BASE 0x0000_0070
	SECURITY_LOCAL_ACCOUNT 0x0000_0071
	SECURITY_LOCAL_ACCOUNT_AND_ADMIN 0x0000_0072

	DOMAIN_GROUP_AUTHORIZATION_DATA_IS_COMPOUNDED 0x0000_01f0
	DOMAIN_GROUP_AUTHORIZATION_DATA_CONTAINS_CLAIMS 0x0000_01f1
	DOMAIN_GROUP_ENTERPRISE_READONLY_DOMAIN_CONTROLLERS 0x0000_01f2
	DOMAIN_USER_ADMIn 0x0000_01f4
	DOMAIN_USER_GUEST 0x0000_01f5
	DOMAIN_USER_KRBTGT 0x0000_01f6
	DOMAIN_USER_DEFAULT_ACCOUNT 0x0000_01f7
	DOMAIN_USER_WDAG_ACCOUNT 0x0000_01f8
	DOMAIN_GROUP_ADMINS 0x0000_0200
	DOMAIN_GROUP_USERS 0x0000_0201
	DOMAIN_GROUP_GUESTS 0x0000_0202
	DOMAIN_GROUP_COMPUTERS 0x0000_0203
	DOMAIN_GROUP_CONTROLLERS 0x0000_0204
	DOMAIN_GROUP_CERT_ADMINS 0x0000_0205
	DOMAIN_GROUP_SCHEMA_ADMINS 0x0000_0206
	DOMAIN_GROUP_ENTERPRISE_ADMINS 0x0000_0207
	DOMAIN_GROUP_POLICY_ADMINS 0x0000_0208
	DOMAIN_GROUP_READONLY_CONTROLLERS 0x0000_0209
	DOMAIN_GROUP_CLONEABLE_CONTROLLERS 0x0000_020a
	DOMAIN_GROUP_CDC_RESERVED 0x0000_020c
	DOMAIN_GROUP_PROTECTED_USERS 0x0000_020d
	DOMAIN_GROUP_KEY_ADMINS 0x0000_020e
	DOMAIN_GROUP_ENTERPRISE_KEY_ADMINS 0x0000_020f
	DOMAIN_ALIAS_ADMINS 0x0000_0220
	DOMAIN_ALIAS_USERS 0x0000_0221
	DOMAIN_ALIAS_GUESTS 0x0000_0222
	DOMAIN_ALIAS_POWER_USERS 0x0000_0223
	DOMAIN_ALIAS_ACCOUNT_OPS 0x0000_0224
	DOMAIN_ALIAS_SYSTEM_OPS 0x0000_0225
	DOMAIN_ALIAS_PRINT_OPS 0x0000_0226
	DOMAIN_ALIAS_BACKUP_OPS 0x0000_0227
	DOMAIN_ALIAS_REPLICATOR 0x0000_0228
	DOMAIN_ALIAS_RAS_SERVERS 0x0000_0229
	DOMAIN_ALIAS_PREW2KCOMPACCESS 0x0000_022a
	DOMAIN_ALIAS_REMOTE_DESKTOP_USERS 0x0000_022b
	DOMAIN_ALIAS_NETWORK_CONFIGURATION_OPS 0x0000_022c
	DOMAIN_ALIAS_INCOMING_FOREST_TRUST_BUILDERS 0x0000_022d
	DOMAIN_ALIAS_MONITORING_USERS 0x0000_022e
	DOMAIN_ALIAS_LOGGING_USERS 0x0000_022f
	DOMAIN_ALIAS_AUTHORIZATIONACCESS 0x0000_0230
	DOMAIN_ALIAS_TS_LICENSE_SERVERS 0x0000_0231
	DOMAIN_ALIAS_DCOM_USERS 0x0000_0232
	DOMAIN_ALIAS_IUSERS 0x0000_0238
	DOMAIN_ALIAS_CRYPTO_OPERATORS 0x0000_0239
	DOMAIN_ALIAS_CACHEABLE_PRINCIPALS_GROUP 0x0000_023b
	DOMAIN_ALIAS_NON_CACHEABLE_PRINCIPALS_GROUP 0x0000_023c
	DOMAIN_ALIAS_EVENT_LOG_READERS_GROUP 0x0000_023d
	DOMAIN_ALIAS_CERTSVC_DCOM_ACCESS_GROUP 0x0000_023e
	DOMAIN_ALIAS_RDS_REMOTE_ACCESS_SERVERS 0x0000_023f
	DOMAIN_ALIAS_RDS_ENDPOINT_SERVERS 0x0000_0240
	DOMAIN_ALIAS_RDS_MANAGEMENT_SERVERS 0x0000_0241
	DOMAIN_ALIAS_HYPER_V_ADMINS 0x0000_0242
	DOMAIN_ALIAS_ACCESS_CONTROL_ASSISTANCE_OPS 0x0000_0243
	DOMAIN_ALIAS_REMOTE_MANAGEMENT_USERS 0x0000_0244
	DOMAIN_ALIAS_DEFAULT_ACCOUNT 0x0000_0245
	DOMAIN_ALIAS_STORAGE_REPLICA_ADMINS 0x0000_0246
	DOMAIN_ALIAS_DEVICE_OWNERS 0x0000_0247
	SECURITY_APP_PACKAGE_BASE 0x0000_0002
	SECURITY_CAPABILITY_BASE 0x0000_0003
	SECURITY_CAPABILITY_APP 0x00000_0400
	SECURITY_BUILTIN_PACKAGE_ANY_PACKAGE 0x0000_0001
	SECURITY_BUILTIN_PACKAGE_ANY_RESTRICTED_PACKAGE 0x0000_0002
	SECURITY_CAPABILITY_INTERNET_CLIENT 0x0000_0001
	SECURITY_CAPABILITY_INTERNET_CLIENT_SERVER 0x0000_0002
	SECURITY_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER 0x0000_0003
	SECURITY_CAPABILITY_PICTURES_LIBRARY 0x0000_0004
	SECURITY_CAPABILITY_VIDEOS_LIBRARY 0x0000_0005
	SECURITY_CAPABILITY_MUSIC_LIBRARY 0x0000_0006
	SECURITY_CAPABILITY_DOCUMENTS_LIBRARY 0x0000_0007
	SECURITY_CAPABILITY_ENTERPRISE_AUTHENTICATION 0x0000_0008
	SECURITY_CAPABILITY_SHARED_USER_CERTIFICATES 0x0000_0009
	SECURITY_CAPABILITY_REMOVABLE_STORAGE 0x0000_000a
	SECURITY_CAPABILITY_APPOINTMENTS 0x0000_000b
	SECURITY_CAPABILITY_CONTACTS 0x0000_000c
	SECURITY_CAPABILITY_INTERNET_EXPLORER 0x0000_1000
	SECURITY_AUTHENTICATION_AUTHORITY_ASSERTED 0x0000_0001
	SECURITY_AUTHENTICATION_SERVICE_ASSERTED 0x0000_0002
	SECURITY_AUTHENTICATION_FRESH_KEY_AUTH 0x0000_0003
	SECURITY_AUTHENTICATION_KEY_TRUST 0x0000_0004
	SECURITY_AUTHENTICATION_KEY_PROPERTY_MFA 0x0000_0005
	SECURITY_AUTHENTICATION_KEY_PROPERTY_ATTESTATION 0x0000_0006
	SECURITY_PROCESS_PROTECTION_TYPE_FULL 0x0000_0400
	SECURITY_PROCESS_PROTECTION_TYPE_LITE 0x0000_0200
	SECURITY_PROCESS_PROTECTION_TYPE_NONE 0x0000_0000
	SECURITY_PROCESS_PROTECTION_LEVEL_WINTCB 0x0000_2000
	SECURITY_PROCESS_PROTECTION_LEVEL_WINDOWS 0x0000_1000
	SECURITY_PROCESS_PROTECTION_LEVEL_APP 0x0000_0800
	SECURITY_PROCESS_PROTECTION_LEVEL_ANTIMALWARE 0x0000_0600
	SECURITY_PROCESS_PROTECTION_LEVEL_AUTHENTICODE 0x0000_0400
	SECURITY_PROCESS_PROTECTION_LEVEL_NONE 0x0000_0000
	SECURITY_TRUSTED_INSTALLER_1 9_5600_8885
	SECURITY_TRUSTED_INSTALLER_2 34_1852_2649
	SECURITY_TRUSTED_INSTALLER_3 18_3103_8044
	SECURITY_TRUSTED_INSTALLER_4 18_5329_2631
	SECURITY_TRUSTED_INSTALLER_5 22_7147_8464
}

const_bitflag! { RRF: u32;
	/// [`HKEY::GetValue`](crate::prelude::kernel_Hkey::RegGetValue) `dwFlags`
	/// (`u32`).
	=>
	=>
	RT_REG_NONE 0x0000_0001
	RT_REG_SZ 0x0000_0002
	RT_REG_EXPAND_SZ 0x0000_0004
	RT_REG_BINARY 0x0000_0008
	RT_REG_DWORD 0x0000_0010
	RT_REG_MULTI_SZ 0x0000_0020
	RT_REG_QWORD 0x0000_0040
	RT_DWORD Self::RT_REG_BINARY.0 | Self::RT_REG_DWORD.0
	RT_QWORD Self::RT_REG_BINARY.0 | Self::RT_REG_QWORD.0
	RT_ANY 0x0000_ffff

	SUBKEY_WOW6464KEY 0x0001_0000
	SUBKEY_WOW6432KEY 0x0002_0000
	WOW64_MASK 0x0003_0000

	NOEXPAND 0x1000_0000
	ZEROONFAILURE 0x2000_0000
}

const_ordinary! { RT: u16;
	/// Predefined resource
	/// [types](https://learn.microsoft.com/en-us/windows/win32/menurc/resource-types)
	/// (`u16`).
	=>
	=>
	ACCELERATOR 9
	ANICURSOR 21
	ANIICON 22
	BITMAP 2
	CURSOR 1
	DIALOG 5
	DLGINCLUDE 17
	FONT 8
	FONTDIR 7
	GROUP_CURSOR 12
	GROUP_ICON 14
	HTML 23
	ICON 3
	MANIFEST 24
	MENU 4
	MESSAGETABLE 11
	PLUGPLAY 19
	RCDATA 10
	STRING 6
	VERSION 16
	VXD 20
}

const_bitflag! { SC_MANAGER: u32;
	/// Service Control Manager access rights
	/// [`flags`](https://learn.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights)
	/// (`u32`).
	=>
	=>
	ALL_ACCESS 0xf003f
	CREATE_SERVICE 0x0002
	CONNECT 0x0001
	ENUMERATE_SERVICE 0x0004
	LOCK 0x0008
	MODIFY_BOOT_CONFIG 0x0020
	QUERY_LOCK_STATUS 0x0010

	GENERIC_READ STANDARD_RIGHTS::READ.0 | Self::ENUMERATE_SERVICE.0 | Self::QUERY_LOCK_STATUS.0
	GENERIC_WRITE STANDARD_RIGHTS::WRITE.0 | Self::CREATE_SERVICE.0 | Self::MODIFY_BOOT_CONFIG.0
	GENERIC_EXECUTE STANDARD_RIGHTS::EXECUTE.0 | Self::CONNECT.0 | Self::LOCK.0
	GENERIC_ALL Self::ALL_ACCESS.0
}

const_ordinary! { SCS: u32;
	/// [`GetBinaryType`](crate::GetBinaryType) return value (`u32`).
	=>
	=>
	W_32BIT_BINARY 0
	DOS_BINARY 1
	WOW_BINARY 2
	PIF_BINARY 3
	POSIX_BINARY 4
	OS216_BINARY 5
	W_64BIT_BINARY 6
}

const_bitflag! { SE: u16;
	/// [`SECURITY_DESCRIPTOR_CONTROL`](https://learn.microsoft.com/en-us/windows/win32/secauthz/security-descriptor-control)
	/// bit flags (`u16`).
	=>
	=>
	OWNER_DEFAULTED 0x0001
	GROUP_DEFAULTED 0x0002
	DACL_PRESENT 0x0004
	DACL_DEFAULTED 0x0008
	SACL_PRESENT 0x0010
	SACL_DEFAULTED 0x0020
	DACL_AUTO_INHERIT_REQ 0x0100
	SACL_AUTO_INHERIT_REQ 0x0200
	DACL_AUTO_INHERITED 0x0400
	SACL_AUTO_INHERITED 0x0800
	DACL_PROTECTED 0x1000
	SACL_PROTECTED 0x2000
	RM_CONTROL_VALID 0x4000
	SELF_RELATIVE 0x8000
}

const_bitflag! { SE_PRIV_ATTR: u32;
	/// [Privilege attributes](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)
	/// (`u32`).
	///
	/// Originally has `SE_PRIVILEGE` prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ENABLED_BY_DEFAULT 0x0000_0001
	ENABLED 0x0000_0002
	REMOVED 0x0000_0004
	USED_FOR_ACCESS 0x8000_0000
	VALID_ATTRIBUTES Self::ENABLED_BY_DEFAULT.0 | Self::ENABLED.0 | Self::REMOVED.0 | Self::USED_FOR_ACCESS.0
}

const_str! { SE_PRIV;
	/// [Privilege constants](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)
	/// (`&'static str`).
	///
	/// Originally has `SE` prefix.
	=>
	CREATE_TOKEN_NAME "SeCreateTokenPrivilege"
	ASSIGNPRIMARYTOKEN_NAME "SeAssignPrimaryTokenPrivilege"
	LOCK_MEMORY_NAME "SeLockMemoryPrivilege"
	INCREASE_QUOTA_NAME "SeIncreaseQuotaPrivilege"
	UNSOLICITED_INPUT_NAME "SeUnsolicitedInputPrivilege"
	MACHINE_ACCOUNT_NAME "SeMachineAccountPrivilege"
	TCB_NAME "SeTcbPrivilege"
	SECURITY_NAME "SeSecurityPrivilege"
	TAKE_OWNERSHIP_NAME "SeTakeOwnershipPrivilege"
	LOAD_DRIVER_NAME "SeLoadDriverPrivilege"
	SYSTEM_PROFILE_NAME "SeSystemProfilePrivilege"
	SYSTEMTIME_NAME "SeSystemtimePrivilege"
	PROF_SINGLE_PROCESS_NAME "SeProfileSingleProcessPrivilege"
	INC_BASE_PRIORITY_NAME "SeIncreaseBasePriorityPrivilege"
	CREATE_PAGEFILE_NAME "SeCreatePagefilePrivilege"
	CREATE_PERMANENT_NAME "SeCreatePermanentPrivilege"
	BACKUP_NAME "SeBackupPrivilege"
	RESTORE_NAME "SeRestorePrivilege"
	SHUTDOWN_NAME "SeShutdownPrivilege"
	DEBUG_NAME "SeDebugPrivilege"
	AUDIT_NAME "SeAuditPrivilege"
	SYSTEM_ENVIRONMENT_NAME "SeSystemEnvironmentPrivilege"
	CHANGE_NOTIFY_NAME "SeChangeNotifyPrivilege"
	REMOTE_SHUTDOWN_NAME "SeRemoteShutdownPrivilege"
	UNDOCK_NAME "SeUndockPrivilege"
	SYNC_AGENT_NAME "SeSyncAgentPrivilege"
	ENABLE_DELEGATION_NAME "SeEnableDelegationPrivilege"
	MANAGE_VOLUME_NAME "SeManageVolumePrivilege"
	IMPERSONATE_NAME "SeImpersonatePrivilege"
	CREATE_GLOBAL_NAME "SeCreateGlobalPrivilege"
	TRUSTED_CREDMAN_ACCESS_NAME "SeTrustedCredManAccessPrivilege"
	RELABEL_NAME "SeRelabelPrivilege"
	INC_WORKING_SET_NAME "SeIncreaseWorkingSetPrivilege"
	TIME_ZONE_NAME "SeTimeZonePrivilege"
	CREATE_SYMBOLIC_LINK_NAME "SeCreateSymbolicLinkPrivilege"
	DELEGATE_SESSION_USER_IMPERSONATE_NAME "SeDelegateSessionUserImpersonatePrivilege"
}

const_bitflag! { SECTION: u32;
	/// Composes [`FILE_MAP`](crate::co::FILE_MAP) (`u32`).
	=>
	=>
	QUERY 0x0001
	MAP_WRITE 0x0002
	MAP_READ 0x0004
	MAP_EXECUTE 0x0008
	EXTEND_SIZE 0x0010
	MAP_EXECUTE_EXPLICIT 0x0020
	ALL_ACCESS STANDARD_RIGHTS::REQUIRED.0 | Self::QUERY.0 | Self::MAP_WRITE.0 | Self::MAP_EXECUTE.0 | Self::EXTEND_SIZE.0
}

const_bitflag! { SECURITY_IMPERSONATION: u32;
	/// [`SECURITY_IMPERSONATION_LEVEL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-security_impersonation_level)
	/// enumeration (`u32`).
	=>
	=>
	Anonymous 0
	Identification 1
	Impersonation 2
	Delegation 3
}

const_bitflag! { SECURITY_INFORMATION: u32;
	/// [`SECURITY_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/secauthz/security-information)
	/// flags (`u32`).
	///
	/// Originally has `SECURITY_INFORMATION` suffix.
	=>
	=>
	OWNER 0x0000_0001
	GROUP 0x0000_0002
	DACL 0x0000_0004
	SACL 0x0000_0008
	LABEL 0x0000_0010
	ATTRIBUTE 0x0000_0020
	SCOPE 0x0000_0040
	PROCESS_TRUST_LABEL 0x0000_0080
	ACCESS_FILTER 0x0000_0100
	BACKUP 0x0001_0000
	PROTECTED_DACL 0x8000_0000
	PROTECTED_SACL 0x4000_0000
	UNPROTECTED_DACL 0x2000_0000
	UNPROTECTED_SACL 0x1000_0000
}

const_bitflag! { SERVICE: u32;
	/// Service access rights
	/// [`flags`](https://learn.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights)
	/// (`u32`).
	=>
	=>
	ALL_ACCESS 0xf01ff
	CHANGE_CONFIG 0x0002
	ENUMERATE_DEPENDENTS 0x0008
	INTERROGATE 0x0080
	PAUSE_CONTINUE 0x0040
	QUERY_CONFIG 0x0001
	QUERY_STATUS 0x0004
	START 0x0010
	STOP 0x0020
	USER_DEFINED_CONTROL 0x0100

	ACCESS_SYSTEM_SECURITY 0x0100_0000
	DELETE 0x1_0000
	READ_CONTROL 0x2_0000
	WRITE_DAC 0x4_0000
	WRITE_OWNER 0x8_0000

	GENERIC_READ STANDARD_RIGHTS::READ.0 | Self::QUERY_CONFIG.0 | Self::QUERY_STATUS.0 | Self::INTERROGATE.0 | Self::ENUMERATE_DEPENDENTS.0
	GENERIC_WRITE STANDARD_RIGHTS::WRITE.0 | Self::CHANGE_CONFIG.0
	GENERIC_EXECUTE STANDARD_RIGHTS::EXECUTE.0 | Self::START.0 | Self::STOP.0 | Self::PAUSE_CONTINUE.0 | Self::USER_DEFINED_CONTROL.0
}

const_bitflag! { SERVICE_ACCEPT: u32;
	/// [`SERVICE_STATUS`](crate::SERVICE_STATUS) `dwControlsAccepted` (`u32`).
	=>
	=>
	NETBINDCHANGE 0x0000_0010
	PARAMCHANGE 0x0000_0008
	PAUSE_CONTINUE 0x0000_0002
	PRESHUTDOWN 0x0000_0100
	SHUTDOWN 0x0000_0004
	STOP 0x0000_0001
	HARDWAREPROFILECHANGE 0x0000_0020
	POWEREVENT 0x0000_0040
	SESSIONCHANGE 0x0000_0080
	TIMECHANGE 0x0000_0200
	TRIGGEREVENT 0x0000_0400
	USERMODEREBOOT 0x0000_0800
}

const_bitflag! { SERVICE_CONTROL: u32;
	/// [`LPHANDLER_FUNCTION_EX`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function_ex)
	/// `dwControl` (`u32`).
	///
	/// Used within [`SvcCtl`](crate::SvcCtl) parsing.
	=>
	=>
	CONTINUE 0x0000_0003
	INTERROGATE 0x0000_0004
	NETBINDADD 0x0000_0007
	NETBINDDISABLE 0x0000_000a
	NETBINDENABLE 0x0000_0009
	NETBINDREMOVE 0x0000_0008
	PARAMCHANGE 0x0000_0006
	PAUSE 0x0000_0002
	PRESHUTDOWN 0x0000_000f
	SHUTDOWN 0x0000_0005
	STOP 0x0000_0001

	DEVICEEVENT 0x0000_000b
	HARDWAREPROFILECHANGE 0x0000_000c
	POWEREVENT 0x0000_000d
	SESSIONCHANGE 0x0000_000e
	TIMECHANGE 0x0000_0010
	TRIGGEREVENT 0x0000_0020
	USERMODEREBOOT 0x0000_0040
}

const_ordinary! { SERVICE_ERROR: u32;
	/// [`HSC::CreateService`](crate::prelude::kernel_Hsc::CreateService)
	/// `error_control` (`u32`).
	=>
	=>
	CRITICAL 0x0000_0003
	IGNORE 0x0000_0000
	NORMAL 0x0000_0001
	SEVERE 0x0000_0002
}

const_ordinary! { SERVICE_START: u32;
	/// [`HSC::CreateService`](crate::prelude::kernel_Hsc::CreateService)
	/// `start_type` (`u32`).
	///
	/// Originally has `SERVICE` prefix.
	=>
	=>
	AUTO_START 0x0000_0002
	BOOT_START 0x0000_0000
	DEMAND_START 0x0000_0003
	DISABLED 0x0000_0004
	SYSTEM_START 0x0000_0001
}

const_ordinary! { SERVICE_STATE: u32;
	/// [`HSERVICESTATUS::SetServiceStatus`](crate::prelude::kernel_Hservicestatus::SetServiceStatus)
	/// `current_state` (u32)
	=>
	=>
	CONTINUE_PENDING 0x0000_0005
	PAUSE_PENDING 0x0000_0006
	PAUSED 0x0000_0007
	RUNNING 0x0000_0004
	START_PENDING 0x0000_0002
	STOP_PENDING 0x0000_0003
	STOPPED 0x0000_0001
}

const_ordinary! { SERVICE_TYPE: u32;
	/// [`HSC::CreateService`](crate::prelude::kernel_Hsc::CreateService)
	/// `service_type` (`u32`).
	///
	/// Originally has `SERVICE` prefix.
	=>
	=>
	ADAPTER 0x0000_0004
	FILE_SYSTEM_DRIVER 0x0000_0002
	KERNEL_DRIVER 0x0000_0001
	RECOGNIZER_DRIVER 0x0000_0008
	WIN32_OWN_PROCESS 0x0000_0010
	WIN32_SHARE_PROCESS 0x0000_0020

	WIN32_OWN_PROCESS_INTERACTIVE Self::WIN32_OWN_PROCESS.0 | 0x0000_0100
	WIN32_SHARE_PROCESS_INTERACTIVE Self::WIN32_SHARE_PROCESS.0 | 0x0000_0100
}

const_bitflag! { SHTDN_REASON: u32;
	/// Shutdown reason
	/// [`codes`](https://learn.microsoft.com/en-us/windows/win32/shutdown/system-shutdown-reason-codes)
	/// (`u32`).
	=>
	=>
	MAJOR_APPLICATION 0x0004_0000
	MAJOR_HARDWARE 0x0001_0000
	MAJOR_LEGACY_API 0x0007_0000
	MAJOR_OPERATINGSYSTEM 0x0002_0000
	MAJOR_OTHER 0x0000_0000
	MAJOR_POWER 0x0006_0000
	MAJOR_SOFTWARE 0x0003_0000
	MAJOR_SYSTEM 0x0005_0000

	MINOR_BLUESCREEN 0x0000_000f
	MINOR_CORDUNPLUGGED 0x0000_000b
	MINOR_DISK 0x0000_0007
	MINOR_ENVIRONMENT 0x0000_000c
	MINOR_HARDWARE_DRIVER 0x0000_000d
	MINOR_HOTFIX 0x0000_0011
	MINOR_HOTFIX_UNINSTALL 0x0000_0017
	MINOR_HUNG 0x0000_0005
	MINOR_INSTALLATION 0x0000_0002
	MINOR_MAINTENANCE 0x0000_0001
	MINOR_MMC 0x0000_0019
	MINOR_NETWORK_CONNECTIVITY 0x0000_0014
	MINOR_NETWORKCARD 0x0000_0009
	MINOR_OTHER 0x0000_0000
	MINOR_OTHERDRIVER 0x0000_000e
	MINOR_POWER_SUPPLY 0x0000_000a
	MINOR_PROCESSOR 0x0000_0008
	MINOR_RECONFIG 0x0000_0004
	MINOR_SECURITY 0x0000_0013
	MINOR_SECURITYFIX 0x0000_0012
	MINOR_SECURITYFIX_UNINSTALL 0x0000_0018
	MINOR_SERVICEPACK 0x0000_0010
	MINOR_SERVICEPACK_UNINSTALL 0x0000_0016
	MINOR_TERMSRV 0x0000_0020
	MINOR_UNSTABLE 0x0000_0006
	MINOR_UPGRADE 0x0000_0003
	MINOR_WMI 0x0000_0015

	FLAG_USER_DEFINED 0x4000_0000
	FLAG_PLANNED 0x8000_0000
}

const_ordinary! { SID_NAME_USE: u32;
	/// [`SID_NAME_USE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-sid_name_use)
	/// enumeration (`u32`).
	///
	/// Originally has `Sid` prefix.
	=>
	=>
	User 1
	Group 2
	Domain 3
	Alias 4
	WellKnownGroup 5
	DeletedAccount 6
	Invalid 7
	Unknown 8
	Computer 9
	Label 10
	LogonSession 11
}

const_ordinary! { SORT: u16;
	/// Sort order
	/// [identifiers](https://learn.microsoft.com/en-us/windows/win32/intl/sort-order-identifiers)
	/// (`u16`).
	=>
	=>
	DEFAULT 0x0
	INVARIANT_MATH 0x1
	JAPANESE_XJIS 0x0
	JAPANESE_UNICODE 0x1
	JAPANESE_RADICALSTROKE 0x4
	CHINESE_BIG5 0x0
	CHINESE_PRCP 0x0
	CHINESE_UNICODE 0x1
	CHINESE_PRC 0x2
	CHINESE_BOPOMOFO 0x3
	CHINESE_RADICALSTROKE 0x4
	KOREAN_KSC 0x0
	KOREAN_UNICODE 0x1
	GERMAN_PHONE_BOOK 0x1
	HUNGARIAN_DEFAULT 0x0
	HUNGARIAN_TECHNICAL 0x1
	GEORGIAN_TRADITIONAL 0x0
	GEORGIAN_MODERN 0x1
}

const_bitflag! { STANDARD_RIGHTS: u32;
	/// Standard access rights
	/// [flags](https://learn.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)
	/// (`u32`).
	=>
	=>
	REQUIRED 0x000_f0000
	READ ACCESS_RIGHTS::READ_CONTROL.0
	WRITE ACCESS_RIGHTS::READ_CONTROL.0
	EXECUTE ACCESS_RIGHTS::READ_CONTROL.0
	ALL 0x001_f0000
}

const_bitflag! { STARTF: u32;
	/// [`HPROCESS::CreateProcess`](crate::prelude::kernel_Hprocess::CreateProcess)
	/// `dwFlags` (`u32`).
	=>
	=>
	FORCEONFEEDBACK 0x0000_0040
	FORCEOFFFEEDBACK 0x0000_0080
	PREVENTPINNING 0x0000_2000
	RUNFULLSCREEN 0x0000_0020
	TITLEISAPPID 0x0000_1000
	TITLEISLINKNAME 0x0000_0800
	UNTRUSTEDSOURCE 0x0000_8000
	USECOUNTCHARS 0x0000_0008
	USEFILLATTRIBUTE 0x0000_0010
	USEHOTKEY 0x0000_0200
	USEPOSITION 0x0000_0004
	USESHOWWINDOW 0x0000_0001
	USESIZE 0x0000_0002
	USESTDHANDLES 0x0000_0100
}

const_ordinary! { STD_HANDLE: u32;
	/// [`GetStdHandle`](crate::prelude::kernel_Hstd::GetStdHandle) `std_handle`
	/// (`u32`).
	///
	/// Originally has `STD` prefix and `HANDLE` suffix.
	=>
	=>
	INPUT -10i32 as u32
	OUTPUT -11i32 as u32
	ERROR -12i32 as u32
}

const_ordinary! { SUBLANG: u16;
	/// Sublanguage
	/// [identifier](https://learn.microsoft.com/en-us/windows/win32/intl/language-identifier-constants-and-strings)
	/// (`u16`).
	=>
	=>
	NEUTRAL 0x00
	DEFAULT 0x01
	SYS_DEFAULT 0x02
	CUSTOM_DEFAULT 0x03
	CUSTOM_UNSPECIFIED 0x04
	UI_CUSTOM_DEFAULT 0x05
	AFRIKAANS_SOUTH_AFRICA 0x01
	ALBANIAN_ALBANIA 0x01
	ALSATIAN_FRANCE 0x01
	AMHARIC_ETHIOPIA 0x01
	ARABIC_SAUDI_ARABIA 0x01
	ARABIC_IRAQ 0x02
	ARABIC_EGYPT 0x03
	ARABIC_LIBYA 0x04
	ARABIC_ALGERIA 0x05
	ARABIC_MOROCCO 0x06
	ARABIC_TUNISIA 0x07
	ARABIC_OMAN 0x08
	ARABIC_YEMEN 0x09
	ARABIC_SYRIA 0x0a
	ARABIC_JORDAN 0x0b
	ARABIC_LEBANON 0x0c
	ARABIC_KUWAIT 0x0d
	ARABIC_UAE 0x0e
	ARABIC_BAHRAIN 0x0f
	ARABIC_QATAR 0x10
	ARMENIAN_ARMENIA 0x01
	ASSAMESE_INDIA 0x01
	AZERI_LATIN 0x01
	AZERI_CYRILLIC 0x02
	AZERBAIJANI_AZERBAIJAN_LATIN 0x01
	AZERBAIJANI_AZERBAIJAN_CYRILLIC 0x02
	BANGLA_INDIA 0x01
	BANGLA_BANGLADESH 0x02
	BASHKIR_RUSSIA 0x01
	BASQUE_BASQUE 0x01
	BELARUSIAN_BELARUS 0x01
	BENGALI_INDIA 0x01
	BENGALI_BANGLADESH 0x02
	BOSNIAN_BOSNIA_HERZEGOVINA_LATIN 0x05
	BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC 0x08
	BRETON_FRANCE 0x01
	BULGARIAN_BULGARIA 0x01
	CATALAN_CATALAN 0x01
	CENTRAL_KURDISH_IRAQ 0x01
	CHEROKEE_CHEROKEE 0x01
	CHINESE_TRADITIONAL 0x01
	CHINESE_SIMPLIFIED 0x02
	CHINESE_HONGKONG 0x03
	CHINESE_SINGAPORE 0x04
	CHINESE_MACAU 0x05
	CORSICAN_FRANCE 0x01
	CZECH_CZECH_REPUBLIC 0x01
	CROATIAN_CROATIA 0x01
	CROATIAN_BOSNIA_HERZEGOVINA_LATIN 0x04
	DANISH_DENMARK 0x01
	DARI_AFGHANISTAN 0x01
	DIVEHI_MALDIVES 0x01
	DUTCH 0x01
	DUTCH_BELGIAN 0x02
	ENGLISH_US 0x01
	ENGLISH_UK 0x02
	ENGLISH_AUS 0x03
	ENGLISH_CAN 0x04
	ENGLISH_NZ 0x05
	ENGLISH_EIRE 0x06
	ENGLISH_SOUTH_AFRICA 0x07
	ENGLISH_JAMAICA 0x08
	ENGLISH_CARIBBEAN 0x09
	ENGLISH_BELIZE 0x0a
	ENGLISH_TRINIDAD 0x0b
	ENGLISH_ZIMBABWE 0x0c
	ENGLISH_PHILIPPINES 0x0d
	ENGLISH_INDIA 0x10
	ENGLISH_MALAYSIA 0x11
	ENGLISH_SINGAPORE 0x12
	ESTONIAN_ESTONIA 0x01
	FAEROESE_FAROE_ISLANDS 0x01
	FILIPINO_PHILIPPINES 0x01
	FINNISH_FINLAND 0x01
	FRENCH 0x01
	FRENCH_BELGIAN 0x02
	FRENCH_CANADIAN 0x03
	FRENCH_SWISS 0x04
	FRENCH_LUXEMBOURG 0x05
	FRENCH_MONACO 0x06
	FRISIAN_NETHERLANDS 0x01
	FULAH_SENEGAL 0x02
	GALICIAN_GALICIAN 0x01
	GEORGIAN_GEORGIA 0x01
	GERMAN 0x01
	GERMAN_SWISS 0x02
	GERMAN_AUSTRIAN 0x03
	GERMAN_LUXEMBOURG 0x04
	GERMAN_LIECHTENSTEIN 0x05
	GREEK_GREECE 0x01
	GREENLANDIC_GREENLAND 0x01
	GUJARATI_INDIA 0x01
	HAUSA_NIGERIA_LATIN 0x01
	HAWAIIAN_US 0x01
	HEBREW_ISRAEL 0x01
	HINDI_INDIA 0x01
	HUNGARIAN_HUNGARY 0x01
	ICELANDIC_ICELAND 0x01
	IGBO_NIGERIA 0x01
	INDONESIAN_INDONESIA 0x01
	INUKTITUT_CANADA 0x01
	INUKTITUT_CANADA_LATIN 0x02
	IRISH_IRELAND 0x02
	ITALIAN 0x01
	ITALIAN_SWISS 0x02
	JAPANESE_JAPAN 0x01
	KANNADA_INDIA 0x01
	KASHMIRI_SASIA 0x02
	KASHMIRI_INDIA 0x02
	KAZAK_KAZAKHSTAN 0x01
	KHMER_CAMBODIA 0x01
	KICHE_GUATEMALA 0x01
	KINYARWANDA_RWANDA 0x01
	KONKANI_INDIA 0x01
	KOREAN 0x01
	KYRGYZ_KYRGYZSTAN 0x01
	LAO_LAO 0x01
	LATVIAN_LATVIA 0x01
	LITHUANIAN 0x01
	LOWER_SORBIAN_GERMANY 0x02
	LUXEMBOURGISH_LUXEMBOURG 0x01
	MACEDONIAN_MACEDONIA 0x01
	MALAY_MALAYSIA 0x01
	MALAY_BRUNEI_DARUSSALAM 0x02
	MALAYALAM_INDIA 0x01
	MALTESE_MALTA 0x01
	MAORI_NEW_ZEALAND 0x01
	MAPUDUNGUN_CHILE 0x01
	MARATHI_INDIA 0x01
	MOHAWK_MOHAWK 0x01
	MONGOLIAN_CYRILLIC_MONGOLIA 0x01
	MONGOLIAN_PRC 0x02
	NEPALI_INDIA 0x02
	NEPALI_NEPAL 0x01
	NORWEGIAN_BOKMAL 0x01
	NORWEGIAN_NYNORSK 0x02
	OCCITAN_FRANCE 0x01
	ODIA_INDIA 0x01
	ORIYA_INDIA 0x01
	PASHTO_AFGHANISTAN 0x01
	PERSIAN_IRAN 0x01
	POLISH_POLAND 0x01
	PORTUGUESE 0x02
	PORTUGUESE_BRAZILIAN 0x01
	PULAR_SENEGAL 0x02
	PUNJABI_INDIA 0x01
	PUNJABI_PAKISTAN 0x02
	QUECHUA_BOLIVIA 0x01
	QUECHUA_ECUADOR 0x02
	QUECHUA_PERU 0x03
	ROMANIAN_ROMANIA 0x01
	ROMANSH_SWITZERLAND 0x01
	RUSSIAN_RUSSIA 0x01
	SAKHA_RUSSIA 0x01
	SAMI_NORTHERN_NORWAY 0x01
	SAMI_NORTHERN_SWEDEN 0x02
	SAMI_NORTHERN_FINLAND 0x03
	SAMI_LULE_NORWAY 0x04
	SAMI_LULE_SWEDEN 0x05
	SAMI_SOUTHERN_NORWAY 0x06
	SAMI_SOUTHERN_SWEDEN 0x07
	SAMI_SKOLT_FINLAND 0x08
	SAMI_INARI_FINLAND 0x09
	SANSKRIT_INDIA 0x01
	SCOTTISH_GAELIC 0x01
	SERBIAN_BOSNIA_HERZEGOVINA_LATIN 0x06
	SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC 0x07
	SERBIAN_MONTENEGRO_LATIN 0x0b
	SERBIAN_MONTENEGRO_CYRILLIC 0x0c
	SERBIAN_SERBIA_LATIN 0x09
	SERBIAN_SERBIA_CYRILLIC 0x0a
	SERBIAN_CROATIA 0x01
	SERBIAN_LATIN 0x02
	SERBIAN_CYRILLIC 0x03
	SINDHI_INDIA 0x01
	SINDHI_PAKISTAN 0x02
	SINDHI_AFGHANISTAN 0x02
	SINHALESE_SRI_LANKA 0x01
	SOTHO_NORTHERN_SOUTH_AFRICA 0x01
	SLOVAK_SLOVAKIA 0x01
	SLOVENIAN_SLOVENIA 0x01
	SPANISH 0x01
	SPANISH_MEXICAN 0x02
	SPANISH_MODERN 0x03
	SPANISH_GUATEMALA 0x04
	SPANISH_COSTA_RICA 0x05
	SPANISH_PANAMA 0x06
	SPANISH_DOMINICAN_REPUBLIC 0x07
	SPANISH_VENEZUELA 0x08
	SPANISH_COLOMBIA 0x09
	SPANISH_PERU 0x0a
	SPANISH_ARGENTINA 0x0b
	SPANISH_ECUADOR 0x0c
	SPANISH_CHILE 0x0d
	SPANISH_URUGUAY 0x0e
	SPANISH_PARAGUAY 0x0f
	SPANISH_BOLIVIA 0x10
	SPANISH_EL_SALVADOR 0x11
	SPANISH_HONDURAS 0x12
	SPANISH_NICARAGUA 0x13
	SPANISH_PUERTO_RICO 0x14
	SPANISH_US 0x15
	SWAHILI_KENYA 0x01
	SWEDISH 0x01
	SWEDISH_FINLAND 0x02
	SYRIAC_SYRIA 0x01
	TAJIK_TAJIKISTAN 0x01
	TAMAZIGHT_ALGERIA_LATIN 0x02
	TAMAZIGHT_MOROCCO_TIFINAGH 0x04
	TAMIL_INDIA 0x01
	TAMIL_SRI_LANKA 0x02
	TATAR_RUSSIA 0x01
	TELUGU_INDIA 0x01
	THAI_THAILAND 0x01
	TIBETAN_PRC 0x01
	TIGRIGNA_ERITREA 0x02
	TIGRINYA_ERITREA 0x02
	TIGRINYA_ETHIOPIA 0x01
	TSWANA_BOTSWANA 0x02
	TSWANA_SOUTH_AFRICA 0x01
	TURKISH_TURKEY 0x01
	TURKMEN_TURKMENISTAN 0x01
	UIGHUR_PRC 0x01
	UKRAINIAN_UKRAINE 0x01
	UPPER_SORBIAN_GERMANY 0x01
	URDU_PAKISTAN 0x01
	URDU_INDIA 0x02
	UZBEK_LATIN 0x01
	UZBEK_CYRILLIC 0x02
	VALENCIAN_VALENCIA 0x02
	VIETNAMESE_VIETNAM 0x01
	WELSH_UNITED_KINGDOM 0x01
	WOLOF_SENEGAL 0x01
	XHOSA_SOUTH_AFRICA 0x01
	YAKUT_RUSSIA 0x01
	YI_PRC 0x01
	YORUBA_NIGERIA 0x01
	ZULU_SOUTH_AFRICA 0x01
}

const_ordinary! { SW: i32;
	/// [`HWND::ShowWindow`](crate::prelude::user_Hwnd::ShowWindow) `show_cmd`
	/// (`i32`).
	=>
	=>
	/// Hides the window and activates another window.
	HIDE 0
	/// Activates and displays a window. If the window is minimized or
	/// maximized the system restores it to its original size and position. An
	/// application should specify this flag when displaying the window for the
	/// first time.
	SHOWNORMAL 1
	/// Activates the window and displays it as a minimized window.
	SHOWMINIMIZED 2
	/// Activates the window and displays it as a maximized window.
	SHOWMAXIMIZED 3
	/// Displays a window in its most recent size and position. This value is
	/// similar to `SW::SHOWNORMAL` except that the window is not activated.
	SHOWNOACTIVATE 4
	/// Activates the window and displays it in its current size and position.
	SHOW 5
	/// Minimizes the specified window and activates the next top-level window
	/// in the Z order.
	MINIMIZE 6
	/// Displays the window as a minimized window. This value is similar to
	/// `SW::SHOWMINIMIZED` except the window is not activated.
	SHOWMINNOACTIVE 7
	// Displays the window in its current size and position. This value is
	// similar to `SW::SHOW` except that the window is not activated.
	SHOWNA 8
	/// Activates and displays the window. If the window is minimized or
	/// maximized the system restores it to its original size and position. An
	/// application should specify this flag when restoring a minimized window.
	RESTORE 9
	/// Sets the show state based on the SW value specified in the
	/// [`STARTUPINFO`](crate::STARTUPINFO) structure passed to the
	/// [`HPROCESS::CreateProcess`](crate::prelude::kernel_Hprocess::CreateProcess)
	/// function by the program that started the application.
	SHOWDEFAULT 10
	/// Minimizes a window even if the thread that owns the window is not
	/// responding. This flag should only be used when minimizing windows from a
	/// different thread.
	FORCEMINIMIZE 11
}

const_ordinary! { SYSTEM_POWER_CONDITION: u32;
	/// [`SYSTEM_POWER_CONDITION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-system_power_condition)
	/// enumeration (`u32`).
	=>
	=>
	Ac 0
	Dc 1
	Hot 2
	ConditionMaximum 3
}

const_bitflag! { TH32CS: u32;
	/// [`HPROCESSLIST::CreateToolhelp32Snapshot`](crate::HPROCESSLIST) `flags`
	/// (`u32`).
	=>
	=>
	/// Includes all heaps of the process.
	SNAPHEAPLIST 0x0000_0001
	/// Includes all processes of the process in the snapshot.
	SNAPPROCESS 0x0000_0002
	/// Includes all threads of the process in the snapshot.
	SNAPTHREAD 0x0000_0004
	/// Includes all modules of the process in the snapshot.
	SNAPMODULE 0x0000_0008
	/// Includes all 32-bit modules of the process when called from a 64-bit
	/// process.
	SNAPMODULE32 0x0000_0010
	/// Includes all processes and threads in the system, plus the heaps and
	/// modules of the process.
	SNAPALL Self::SNAPHEAPLIST.0 | Self::SNAPPROCESS.0 | Self::SNAPTHREAD.0 | Self::SNAPMODULE.0
	/// Indicates that the snapshot handle is to be inheritable.
	INHERIT 0x8000_0000
}

const_bitflag! { THREAD_CREATE: u32;
	/// [`HTHREAD::CreateThread`](crate::prelude::kernel_Hthread::CreateThread)
	/// `flags` (`u32`).
	///
	/// Originally has no prefix.
	=>
	=>
	/// Originally just a zero.
	RUN_IMMEDIATELY 0
	CREATE_SUSPENDED 0x0000_0004
	STACK_SIZE_PARAM_IS_A_RESERVATION 0x0001_0000
}

const_bitflag! { TOKEN: u32;
	/// [Token access rights](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-rights-for-access-token-objects).
	=>
	=>
	DELETE ACCESS_RIGHTS::DELETE.0
	READ_CONTROL ACCESS_RIGHTS::READ_CONTROL.0
	WRITE_DAC ACCESS_RIGHTS::WRITE_DAC.0
	WRITE_OWNER ACCESS_RIGHTS::WRITE_OWNER.0

	ASSIGN_PRIMARY 0x0001
	DUPLICATE 0x0002
	IMPERSONATE 0x0004
	QUERY 0x0008
	QUERY_SOURCE 0x0010
	ADJUST_PRIVILEGES 0x0020
	ADJUST_GROUPS 0x0040
	ADJUST_DEFAULT 0x0080
	ADJUST_SESSIONID 0x0100
	ALL_ACCESS_P STANDARD_RIGHTS::REQUIRED.0 | Self::ASSIGN_PRIMARY.0 | Self::DUPLICATE.0 | Self::IMPERSONATE.0 | Self::QUERY.0 | Self::QUERY_SOURCE.0 | Self::ADJUST_PRIVILEGES.0 | Self::ADJUST_GROUPS.0 | Self::ADJUST_DEFAULT.0
	ALL_ACCESS Self::ALL_ACCESS_P.0 | Self::ADJUST_SESSIONID.0
	READ STANDARD_RIGHTS::READ.0 | Self::QUERY.0
	WRITE STANDARD_RIGHTS::WRITE.0 | Self::ADJUST_PRIVILEGES.0 | Self::ADJUST_GROUPS.0 | Self::ADJUST_DEFAULT.0
	EXECUTE STANDARD_RIGHTS::EXECUTE.0
	TRUST_CONSTRAINT_MASK STANDARD_RIGHTS::READ.0 | Self::QUERY.0 | Self::QUERY_SOURCE.0
	ACCESS_PSEUDO_HANDLE Self::QUERY.0 | Self::QUERY_SOURCE.0
}

const_ordinary! { TOKEN_ELEVATION_TYPE: u32;
	/// [`TOKEN_ELEVATION_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_elevation_type)
	/// enumeration (`u32`).
	=>
	=>
	Default 1
	Full 2
	Limited 3
}

const_ordinary! { TOKEN_INFORMATION_CLASS: u32;
	/// [`TOKEN_INFORMATION_CLASS`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class)
	/// enumeration (`u32`).
	=>
	=>
	User 1
	Groups 2
	Privileges 3
	Owner 4
	PrimaryGroup 5
	DefaultDacl 6
	Source 7
	Type 8
	ImpersonationLevel 9
	Statistics 10
	RestrictedSids 11
	SessionId 12
	GroupsAndPrivileges 13
	SessionReference 14
	SandBoxInert 15
	AuditPolicy 16
	Origin 17
	ElevationType 18
	LinkedToken 19
	Elevation 20
	HasRestrictions 21
	AccessInformation 22
	VirtualizationAllowed 23
	VirtualizationEnabled 24
	IntegrityLevel 25
	UIAccess 26
	MandatoryPolicy 27
	LogonSid 28
	IsAppContainer 29
	Capabilities 30
	AppContainerSid 31
	AppContainerNumber 32
	UserClaimAttributes 33
	DeviceClaimAttributes 34
	RestrictedUserClaimAttributes 35
	RestrictedDeviceClaimAttributes 36
	DeviceGroups 37
	RestrictedDeviceGroups 38
	SecurityAttributes 39
	IsRestricted 40
	ProcessTrustLevel 41
	PrivateNameSpace 42
	SingletonAttributes 43
	BnoIsolation 44
	ChildProcessFlags 45
	IsLessPrivilegedAppContainer 46
	IsSandboxed 47
	OriginatingProcessTrustLevel 48
}

const_ordinary! { TOKEN_MANDATORY_POLICY: u32;
	/// [`TOKEN_MANDATORY_POLICY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_policy)
	/// `Policy` (`u32`).
	=>
	=>
	OFF 0x0
	NO_WRITE_UP 0x1
	NEW_PROCESS_MIN 0x2
	VALID_MASK 0x3
}

const_ordinary! { TOKEN_TYPE: u32;
	/// [`TOKEN_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_type)
	/// enumeration (`u32`).
	=>
	=>
	Primary 1
	Impersonation 2
}

const_bitflag! { TRANSACTION: u32;
	/// [`Transaction access masks`](https://learn.microsoft.com/en-us/windows/win32/ktm/transaction-access-masks)
	/// (`u32`).
	=>
	=>
	QUERY_INFORMATION 0x0001
	SET_INFORMATION 0x0002
	ENLIST 0x0004
	COMMIT 0x0008
	ROLLBACK 0x0010
	PROPAGATE 0x0020
	RIGHT_RESERVED1 0x0040
	GENERIC_READ STANDARD_RIGHTS::READ.0 | TRANSACTION::QUERY_INFORMATION.0 | ACCESS_RIGHTS::SYNCHRONIZE.0
	GENERIC_WRITE STANDARD_RIGHTS::WRITE.0 | TRANSACTION::SET_INFORMATION.0 | TRANSACTION::COMMIT.0 | TRANSACTION::ENLIST.0 | TRANSACTION::ROLLBACK.0 | TRANSACTION::PROPAGATE.0 | ACCESS_RIGHTS::SYNCHRONIZE.0
	GENERIC_EXECUTE STANDARD_RIGHTS::EXECUTE.0 | TRANSACTION::COMMIT.0 | TRANSACTION::ROLLBACK.0 | ACCESS_RIGHTS::SYNCHRONIZE.0
	ALL_ACCESS STANDARD_RIGHTS::REQUIRED.0 | TRANSACTION::GENERIC_READ.0 | TRANSACTION::GENERIC_WRITE.0 | TRANSACTION::GENERIC_EXECUTE.0
	RESOURCE_MANAGER_RIGHTS TRANSACTION::GENERIC_READ.0 | STANDARD_RIGHTS::WRITE.0 | TRANSACTION::SET_INFORMATION.0 | TRANSACTION::ENLIST.0 | TRANSACTION::ROLLBACK.0 | TRANSACTION::PROPAGATE.0 | ACCESS_RIGHTS::SYNCHRONIZE.0
}

const_bitflag! { TRANSACTION_OPT: u32;
	/// [`CrateTransaction`](crate::prelude::kernel_Htransaction::CreateTransaction)
	/// `options` (`u32`).
	///
	/// Originally has `TRANSACTION` prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DO_NOT_PROMOTE 0x0000_0001
}

const_ordinary! { USER_ACTIVITY_PRESENCE: u32;
	/// [`PowerSetting::GlobalUserPresence`](crate::PowerSetting::GlobalUserPresence)
	/// enumeration (`u32`).
	=>
	=>
	Present 0
    NotPresent 1
    Inactive 2
    Maximum 3
    Invalid Self::Maximum.0
}

const_ordinary! { VER_COND: u8;
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `condition` (`u8`).
	=>
	=>
	EQUAL 1
	GREATER 2
	GREATER_EQUAL 3
	LESS 4
	LESS_EQUAL 5
	AND 6
	OR 7
	CONDITION_MASK 7
}

const_bitflag! { VER_MASK: u32;
	/// [`VerifyVersionInfo`](crate::VerifyVersionInfo) and
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `type_mask` (`u32`).
	=>
	=>
	MINORVERSION 0x000_0001
	MAJORVERSION 0x000_0002
	BUILDNUMBER 0x000_0004
	PLATFORMID 0x000_0008
	SERVICEPACKMINOR 0x000_0010
	SERVICEPACKMAJOR 0x000_0020
	SUITENAME 0x000_0040
	PRODUCT_TYPE 0x000_0080
}

const_ordinary! { VER_NT: u8;
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wProductType` (`u8`).
	=>
	=>
	WORKSTATION 0x000_0001
	DOMAIN_CONTROLLER 0x000_0002
	SERVER 0x000_0003
}

const_bitflag! { VER_SUITE: u16;
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wSuiteMask` (`u16`).
	=>
	=>
	SMALLBUSINESS 0x0001
	ENTERPRISE 0x0002
	BACKOFFICE 0x0004
	COMMUNICATIONS 0x0008
	TERMINAL 0x0010
	SMALLBUSINESS_RESTRICTED 0x0020
	EMBEDDEDNT 0x0040
	DATACENTER 0x0080
	SINGLEUSERTS 0x0100
	PERSONAL 0x0200
	BLADE 0x0400
	EMBEDDED_RESTRICTED 0x0800
	SECURITY_APPLIANCE 0x1000
	STORAGE_SERVER 0x2000
	COMPUTE_SERVER 0x4000
	WH_SERVER 0x8000
	//MULTIUSERTS 0x00020000 // Win32 bug, truncated to zero as u16
}

const_ordinary! { VER_PLATFORM: u32;
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `dwPlatformId` (`u32`).
	=>
	=>
	WIN32s 0
	WIN32_WINDOWS 1
	WIN32_NT 2
}

const_ordinary! { WAIT: u32;
	/// [`HPROCESS::WaitForSingleObject`](crate::prelude::kernel_Hprocess::WaitForSingleObject)
	/// return value (`u32`).
	=>
	=>
	ABANDONED 0x0000_0080
	OBJECT_0 0x0000_0000
	TIMEOUT 0x0000_0102
	FAILED 0xffff_ffff
}

const_bitflag! { WC: u32;
	/// [`WideCharToMultiByte`](crate::WideCharToMultiByte) `flags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	COMPOSITECHECK 0x0000_0200
	ERR_INVALID_CHARS 0x0000_0080
	NO_BEST_FIT_CHARS 0x0000_0400
	DEFAULTCHAR 0x0000_0040
	DISCARDNS 0x0000_0010
	SEPCHARS 0x0000_0020
}

const_ordinary! { WELL_KNOWN_SID_TYPE: u32;
	/// [`WELL_KNOWN_SID_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-well_known_sid_type)
	/// enumeration (`u32`).
	=>
	=>
	Null 0
	World 1
	Local 2
	CreatorOwner 3
	CreatorGroup 4
	CreatorOwnerServer 5
	CreatorGroupServer 6
	NtAuthority 7
	Dialup 8
	Network 9
	Batch 10
	Interactive 11
	Service 12
	Anonymous 13
	Proxy 14
	EnterpriseControllers 15
	SelfSid 16
	AuthenticatedUser 17
	RestrictedCode 18
	TerminalServer 19
	RemoteLogonId 20
	LogonIds 21
	LocalSystem 22
	LocalService 23
	NetworkService 24
	BuiltinDomain 25
	BuiltinAdministrators 26
	BuiltinUsers 27
	BuiltinGuests 28
	BuiltinPowerUsers 29
	BuiltinAccountOperators 30
	BuiltinSystemOperators 31
	BuiltinPrintOperators 32
	BuiltinBackupOperators 33
	BuiltinReplicator 34
	BuiltinPreWindows2000CompatibleAccess 35
	BuiltinRemoteDesktopUsers 36
	BuiltinNetworkConfigurationOperators 37
	AccountAdministrator 38
	AccountGuest 39
	AccountKrbtgt 40
	AccountDomainAdmins 41
	AccountDomainUsers 42
	AccountDomainGuests 43
	AccountComputers 44
	AccountControllers 45
	AccountCertAdmins 46
	AccountSchemaAdmins 47
	AccountEnterpriseAdmins 48
	AccountPolicyAdmins 49
	AccountRasAndIasServers 50
	NTLMAuthentication 51
	DigestAuthentication 52
	SChannelAuthentication 53
	ThisOrganization 54
	OtherOrganization 55
	BuiltinIncomingForestTrustBuilders 56
	BuiltinPerfMonitoringUsers 57
	BuiltinPerfLoggingUsers 58
	BuiltinAuthorizationAccess 59
	BuiltinTerminalServerLicenseServers 60
	BuiltinDCOMUsers 61
	BuiltinIUsers 62
	IUser 63
	BuiltinCryptoOperators 64
	UntrustedLabel 65
	LowLabel 66
	MediumLabel 67
	HighLabel 68
	SystemLabel 69
	WriteRestrictedCode 70
	CreatorOwnerRights 71
	CacheablePrincipalsGroup 72
	NonCacheablePrincipalsGroup 73
	EnterpriseReadonlyControllers 74
	AccountReadonlyControllers 75
	BuiltinEventLogReadersGroup 76
	NewEnterpriseReadonlyControllers 77
	BuiltinCertSvcDComAccessGroup 78
	MediumPlusLabel 79
	LocalLogon 80
	ConsoleLogon 81
	ThisOrganizationCertificate 82
	ApplicationPackageAuthority 83
	BuiltinAnyPackage 84
	CapabilityInternetClient 85
	CapabilityInternetClientServer 86
	CapabilityPrivateNetworkClientServer 87
	CapabilityPicturesLibrary 88
	CapabilityVideosLibrary 89
	CapabilityMusicLibrary 90
	CapabilityDocumentsLibrary 91
	CapabilitySharedUserCertificates 92
	CapabilityEnterpriseAuthentication 93
	CapabilityRemovableStorage 94
	BuiltinRDSRemoteAccessServers 95
	BuiltinRDSEndpointServers 96
	BuiltinRDSManagementServers 97
	UserModeDrivers 98
	BuiltinHyperVAdmins 99
	AccountCloneableControllers 100
	BuiltinAccessControlAssistanceOperators 101
	BuiltinRemoteManagementUsers 102
	AuthenticationAuthorityAsserted 103
	AuthenticationServiceAsserted 104
	LocalAccount 105
	LocalAccountAndAdministrator 106
	AccountProtectedUsers 107
	CapabilityAppointments 108
	CapabilityContacts 109
	AccountDefaultSystemManaged 110
	BuiltinDefaultSystemManagedGroup 111
	BuiltinStorageReplicaAdmins 112
	AccountKeyAdmins 113
	AccountEnterpriseKeyAdmins 114
	AuthenticationKeyTrust 115
	AuthenticationKeyPropertyMFA 116
	AuthenticationKeyPropertyAttestation 117
	AuthenticationFreshKeyAuth 118
	BuiltinDeviceOwners 119
}

const_ordinary! { WIN32: u16;
	/// [`_WIN32`](https://learn.microsoft.com/en-us/windows/win32/winprog/using-the-windows-headers)
	/// version definitions (`u16`).
	=>
	=>
	WINNT_NT4 0x0400
	WINNT_WIN2K 0x0500
	WINNT_WINXP 0x0501
	WINNT_WS03 0x0502
	WINNT_WIN6 0x0600
	WINNT_VISTA 0x0600
	WINNT_WS08 0x0600
	WINNT_LONGHORN 0x0600
	WINNT_WIN7 0x0601
	WINNT_WIN8 0x0602
	WINNT_WINBLUE 0x0603
	WINNT_WINTHRESHOLD 0x0a00
	WINNT_WIN10 0x0a00

	IE_IE20 0x0200
	IE_IE30 0x0300
	IE_IE302 0x0302
	IE_IE40 0x0400
	IE_IE401 0x0401
	IE_IE50 0x0500
	IE_IE501 0x0501
	IE_IE55 0x0550
	IE_IE60 0x0600
	IE_IE60SP1 0x0601
	IE_IE60SP2 0x0603
	IE_IE70 0x0700
	IE_IE80 0x0800
	IE_IE90 0x0900
	IE_IE100 0x0A00
	IE_IE110 0x0A00

	IE_NT4 Self::IE_IE20.0
	IE_NT4SP1 Self::IE_IE20.0
	IE_NT4SP2 Self::IE_IE20.0
	IE_NT4SP3 Self::IE_IE302.0
	IE_NT4SP4 Self::IE_IE401.0
	IE_NT4SP5 Self::IE_IE401.0
	IE_NT4SP6 Self::IE_IE50.0
	IE_WIN98 Self::IE_IE401.0
	IE_WIN98SE Self::IE_IE50.0
	IE_WINME Self::IE_IE55.0
	IE_WIN2K Self::IE_IE501.0
	IE_WIN2KSP1 Self::IE_IE501.0
	IE_WIN2KSP2 Self::IE_IE501.0
	IE_WIN2KSP3 Self::IE_IE501.0
	IE_WIN2KSP4 Self::IE_IE501.0
	IE_XP Self::IE_IE60.0
	IE_XPSP1 Self::IE_IE60SP1.0
	IE_XPSP2 Self::IE_IE60SP2.0
	IE_WS03 0x0602
	IE_WS03SP1 Self::IE_IE60SP2.0
	IE_WIN6 Self::IE_IE70.0
	IE_LONGHORN Self::IE_IE70.0
	IE_WIN7 Self::IE_IE80.0
	IE_WIN8 Self::IE_IE100.0
	IE_WINBLUE Self::IE_IE100.0
	IE_WINTHRESHOLD Self::IE_IE110.0
	IE_WIN10 Self::IE_IE110.0
}

const_ordinary! { WTS: u8;
	/// [`wm::WtsSessionChange`](crate::msg::wm::WtsSessionChange) `state`
	/// (`u32`).
	=>
	=>
	CONSOLE_CONNECT 0x1
	CONSOLE_DISCONNECT 0x2
	REMOTE_CONNECT 0x3
	REMOTE_DISCONNECT 0x4
	SESSION_LOGON 0x5
	SESSION_LOGOFF 0x6
	SESSION_LOCK 0x7
	SESSION_UNLOCK 0x8
	SESSION_REMOTE_CONTROL 0x9
	SESSION_CREATE 0xa
	SESSION_TERMINATE 0xb
}

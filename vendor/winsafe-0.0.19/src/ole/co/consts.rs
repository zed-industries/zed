#![allow(non_camel_case_types)]

const_bitflag! { ADVF: u32;
	/// [`ADVF`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-advf)
	/// enumeration (`u32`).
	=>
	=>
	NODATA 1
	PRIMEFIRST 2
	ONLYONCE 4
	DATAONSTOP 64
}

const_ordinary! { CLSCTX: u32;
	/// [`CLSCTX`](https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx)
	/// enumeration (`u32`).
	=>
	=>
	/// Same process.
	///
	/// The code that creates and manages objects of this class is a DLL that
	/// runs in the same process as the caller of the function specifying the
	/// class context.
	INPROC_SERVER 0x1
	/// The code that manages objects of this class is an in-process handler.
	/// This is a DLL that runs in the client process and implements client-side
	/// structures of this class when instances of the class are accessed
	/// remotely.
	INPROC_HANDLER 0x2
	/// Different process, same computer.
	///
	/// The EXE code that creates and manages objects of this class runs on same
	/// machine but is loaded in a separate process space.
	LOCAL_SERVER 0x4
	/// Different computer.
	///
	/// A remote context. The `LocalServer32` or `LocalService` code that creates
	/// and manages objects of this class is run on a different computer.
	REMOTE_SERVER 0x10
	/// Disables the downloading of code from the directory service or the
	/// Internet. This flag cannot be set at the same time as
	/// `CLSCTX::ENABLE_CODE_DOWNLOAD`.
	NO_CODE_DOWNLOAD 0x400
	/// Specify if you want the activation to fail if it uses custom marshalling.
	NO_CUSTOM_MARSHAL 0x1000
	/// Enables the downloading of code from the directory service or the
	/// Internet. This flag cannot be set at the same time as
	/// `CLSCTX::NO_CODE_DOWNLOAD`.
	ENABLE_CODE_DOWNLOAD 0x2000
	/// The `CLSCTX::NO_FAILURE_LOG` can be used to override the logging of
	/// failures in [`CoCreateInstanceEx`](crate::CoCreateInstanceEx).
	NO_FAILURE_LOG 0x4000
	/// Disables activate-as-activator (AAA) activations for this activation only.
	DISABLE_AAA 0x8000
	/// Enables activate-as-activator (AAA) activations for this activation only.
	ENABLE_AAA 0x1_0000
	/// Begin this activation from the default context of the current apartment.
	FROM_DEFAULT_CONTEXT 0x2_0000
	/// Activate or connect to a 32-bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_X86_SERVER 0x4_0000
	/// Activate or connect to a 32-bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_32_BIT_SERVER Self::ACTIVATE_X86_SERVER.0
	/// Activate or connect to a 64 bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_64_BIT_SERVER 0x8_0000
	/// Specify this flag for Interactive User activation behavior for
	/// As-Activator servers.
	ACTIVATE_AAA_AS_IU 0x80_0000
	/// (No official docs for this entry.)
	ACTIVATE_ARM32_SERVER 0x200_0000
}

const_bitflag! { COINIT: u32;
	/// [`COINIT`](https://learn.microsoft.com/en-us/windows/win32/api/objbase/ne-objbase-coinit)
	/// enumeration (`u32`).
	=>
	=>
	/// Initializes the thread for apartment-threaded object concurrency.
	///
	/// Use this when in a thread that creates a window.
	APARTMENTTHREADED 0x2
	/// Initializes the thread for multithreaded object concurrency.
	///
	/// Use this when in a thread that doesn't create a window.
	MULTITHREADED 0x0
	/// Disables DDE for OLE1 support.
	///
	/// It's a good idea to add this flag, since it avoids some overhead
	/// associated with OLE 1.0, an obsolete technology.
	DISABLE_OLE1DDE 0x4
	/// Increase memory usage in an attempt to increase performance.
	SPEED_OVER_MEMORY 0x8
}

const_ordinary! { DROPEFFECT: u32;
	/// [`DROPEFFECT`](https://learn.microsoft.com/en-us/windows/win32/com/dropeffect-constants)
	/// constants (`u32`).
	=>
	=>
	NONE 0
	COPY 1
	MOVE 2
	LINK 4
	SCROLL 0x8000_0000
}

const_ordinary! { DVASPECT: u32;
	/// [`DVASPECT`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ne-wtypes-dvaspect)
	/// enumeration (`u32`).
	=>
	=>
	CONTENT 1
	THUMBNAIL 2
	ICON 4
	DOCPRINT 8
}

const_ordinary! { FACILITY: u32;
	/// [`HRESULT`](crate::co::HRESULT) facility (`u32`).
	=>
	=>
	NULL 0
	RPC 1
	DISPATCH 2
	STORAGE 3
	ITF 4
	WIN32 7
	WINDOWS 8
	SSPI 9
	SECURITY 9
	CONTROL 10
	CERT 11
	INTERNET 12
	MEDIASERVER 13
	MSMQ 14
	SETUPAPI 15
	SCARD 16
	COMPLUS 17
	AAF 18
	URT 19
	ACS 20
	DPLAY 21
	UMI 22
	SXS 23
	WINDOWS_CE 24
	HTTP 25
	USERMODE_COMMONLOG 26
	WER 27
	USERMODE_FILTER_MANAGER 31
	BACKGROUNDCOPY 32
	CONFIGURATION 33
	WIA 33
	STATE_MANAGEMENT 34
	METADIRECTORY 35
	WINDOWSUPDATE 36
	DIRECTORYSERVICE 37
	GRAPHICS 38
	SHELL 39
	NAP 39
	TPM_SERVICES 40
	TPM_SOFTWARE 41
	UI 42
	XAML 43
	ACTION_QUEUE 44
	PLA 48
	WINDOWS_SETUP 48
	FVE 49
	FWP 50
	WINRM 51
	NDIS 52
	USERMODE_HYPERVISOR 53
	CMI 54
	USERMODE_VIRTUALIZATION 55
	USERMODE_VOLMGR 56
	BCD 57
	USERMODE_VHD 58
	USERMODE_HNS 59
	SDIAG 60
	WEBSERVICES 61
	WINPE 61
	WPN 62
	WINDOWS_STORE 63
	INPUT 64
	EAP 66
	WINDOWS_DEFENDER 80
	OPC 81
	XPS 82
	MBN 84
	POWERSHELL 84
	RAS 83
	P2P_INT 98
	P2P 99
	DAF 100
	BLUETOOTH_ATT 101
	AUDIO 102
	STATEREPOSITORY 103
	VISUALCPP 109
	SCRIPT 112
	PARSE 113
	BLB 120
	BLB_CLI 121
	WSBAPP 122
	BLBUI 128
	USN 129
	USERMODE_VOLSNAP 130
	TIERING 131
	WSB_ONLINE 133
	ONLINE_ID 134
	DEVICE_UPDATE_AGENT 135
	DRVSERVICING 136
	DLS 153
	DELIVERY_OPTIMIZATION 208
	USERMODE_SPACES 231
	USER_MODE_SECURITY_CORE 232
	USERMODE_LICENSING 234
	SOS 160
	DEBUGGERS 176
	SPP 256
	RESTORE 256
	DMSERVER 256
	DEPLOYMENT_SERVICES_SERVER 257
	DEPLOYMENT_SERVICES_IMAGING 258
	DEPLOYMENT_SERVICES_MANAGEMENT 259
	DEPLOYMENT_SERVICES_UTIL 260
	DEPLOYMENT_SERVICES_BINLSVC 261
	DEPLOYMENT_SERVICES_PXE 263
	DEPLOYMENT_SERVICES_TFTP 264
	DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT 272
	DEPLOYMENT_SERVICES_DRIVER_PROVISIONING 278
	DEPLOYMENT_SERVICES_MULTICAST_SERVER 289
	DEPLOYMENT_SERVICES_MULTICAST_CLIENT 290
	DEPLOYMENT_SERVICES_CONTENT_PROVIDER 293
	LINGUISTIC_SERVICES 305
	AUDIOSTREAMING 1094
	ACCELERATOR 1536
	WMAAECMA 1996
	DIRECTMUSIC 2168
	DIRECT3D10 2169
	DXGI 2170
	DXGI_DDI 2171
	DIRECT3D11 2172
	DIRECT3D11_DEBUG 2173
	DIRECT3D12 2174
	DIRECT3D12_DEBUG 2175
	LEAP 2184
	AUDCLNT 2185
	WINCODEC_DWRITE_DWM 2200
	WINML 2192
	DIRECT2D 2201
	DEFRAG 2304
	USERMODE_SDBUS 2305
	JSCRIPT 2306
	PIDGENX 2561
	EAS 85
	WEB 885
	WEB_SOCKET 886
	MOBILE 1793
	SQLITE 1967
	UTC 1989
	WEP 2049
	SYNCENGINE 2050
	XBOX 2339
	GAME 2340
	PIX 2748
}

const_bitflag! { LOCKTYPE: u32;
	/// [`LOCKTYPE`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-locktype)
	/// enumeration (`u32`).
	=>
	=>
	WRITE 1
	EXCLUSIVE 2
	ONLYONCE 4
}

const_ordinary! { MKRREDUCE: u32;
	/// [How far](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-mkrreduce)
	/// a moniker should be reduced (`u32`).
	=>
	=>
	ONE (3 << 16)
	TOUSER (2 << 16)
	THROUGHUSER (1 << 16)
	ALL 0
}

const_ordinary! { MKSYS: u32;
	/// Moniker
	/// [classes](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-mksys)
	/// (`u32`).
	=>
	=>
	NONE 0
	GENERICCOMPOSITE 1
	FILEMONIKER 2
	ANTIMONIKER 3
	ITEMMONIKER 4
	POINTERMONIKER 5
	CLASSMONIKER 7
	OBJREFMONIKER 8
	SESSIONMONIKER 9
	LUAMONIKER 10
}

const_ordinary! { PICTYPE: i16;
	/// [`PICTYPE`](https://learn.microsoft.com/en-us/windows/win32/com/pictype-constants)
	/// constants (`i16`).
	=>
	=>
	UNINITIALIZED -1
	NONE 0
	BITMAP 1
	METAFILE 2
	ICON 3
	ENHMETAFILE 4
}

const_ordinary! { RPC_C_AUTHN: u32;
	/// Authentication service
	/// [constants](https://learn.microsoft.com/en-us/windows/win32/com/com-authentication-service-constants)
	/// (`u32`).
	=>
	=>
	NONE 0
	DCE_PRIVATE 1
	DCE_PUBLIC 2
	DEC_PUBLIC 4
	GSS_NEGOTIATE 9
	WINNT 10
	GSS_SCHANNEL 14
	GSS_KERBEROS 16
	DPA 17
	MSN 18
	DIGEST 21
	KERNEL 20
	NEGO_EXTENDER 30
	PKU2U 31
	LIVE_SSP 32
	LIVEXP_SSP 35
	CLOUD_AP 36
	MSONLINE 82
	MQ 100
	DEFAULT 0xffff_ffff
}

const_ordinary! { RPC_C_AUTHZ: u32;
	/// Authorization
	/// [constants](https://learn.microsoft.com/en-us/windows/win32/com/com-authorization-constants)
	/// (`u32`).
	=>
	=>
	NONE 0
	NAME 1
	DCE 2
	DEFAULT 0xffff_ffff
}

const_ordinary! { RPC_C_IMP_LEVEL: u32;
	/// Impersonation level
	/// [constants](https://learn.microsoft.com/en-us/windows/win32/com/com-impersonation-level-constants)
	/// (`u32`).
	=>
	=>
	DEFAULT 0
	ANONYMOUS 1
	IDENTIFY 2
	IMPERSONATE 3
	DELEGATE 4
}

const_ordinary! { RPC_C_QOS_CAPABILITIES: u32;
	/// [Quality of service](https://learn.microsoft.com/en-us/windows/win32/rpc/quality-of-service)
	/// capabilities (`u32`).
	=>
	=>
	DEFAULT 0x0
	MUTUAL_AUTH 0x1
	MAKE_FULLSIC 0x2
	ANY_AUTHORITY 0x4
	IGNORE_DELEGATE_FAILURE 0x8
	LOCAL_MA_HINT 0x10
	SCHANNEL_FULL_AUTH_IDENTITY 0x20
}

const_ordinary! { SEC_WINNT_AUTH_IDENTITY: u32;
	/// [`COAUTHIDENTITY`](crate::COAUTHIDENTITY) `Flags` (`u32`).
	=>
	=>
	ANSI 0x1
	UNICODE 0x2
}

const_ordinary! { SEVERITY: u8;
	/// [`HRESULT`](crate::co::HRESULT) severity (`u8`).
	=>
	=>
	SUCCESS 0
	FAILURE 1
}

const_bitflag! { STGC: u32;
	/// [`STGC`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ne-wtypes-stgc)
	/// enumeration (`u32`).
	=>
	=>
	DEFAULT 0
	OVERWRITE 1
	ONLYIFCURRENT 2
	DANGEROUSLYCOMMITMERELYTODISKCACHE 4
	CONSOLIDATE 8
}

const_bitflag! { STGM: u32;
	/// [`STGM`](https://learn.microsoft.com/en-us/windows/win32/stg/stgm-constants)
	/// enumeration (`u32`).
	=>
	=>
	READ 0x0000_0000
	WRITE 0x0000_0001
	READWRITE 0x0000_0002
	SHARE_DENY_NONE 0x0000_0040
	SHARE_DENY_READ 0x0000_0030
	SHARE_DENY_WRITE 0x0000_0020
	SHARE_EXCLUSIVE 0x0000_0010
	PRIORITY 0x0004_0000
	CREATE 0x0000_1000
	CONVERT 0x0002_0000
	FAILIFTHERE 0x0000_0000
	DIRECT 0x0000_0000
	TRANSACTED 0x0001_0000
	NOSCRATCH 0x0010_0000
	NOSNAPSHOT 0x0020_0000
	SIMPLE 0x0800_0000
	DIRECT_SWMR 0x0040_0000
	DELETEONRELEASE 0x0400_0000
}

const_ordinary! { STGMOVE: u32;
	/// [`STGMOVE`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ne-wtypes-stgmove)
	/// enumeration (`u32`).
	=>
	=>
	MOVE 0
	COPY 1
	SHALLOWCOPY 2
}

const_ordinary! { STREAM_SEEK: u32;
	/// [`STREAM_SEEK`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-stream_seek)
	/// enumeration (`u32`).
	=>
	=>
	SET 0
	CUR 1
	END 2
}

const_ordinary! { TYMED: u32;
	/// [`TYMED`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-tymed)
	/// enumeration (`u32`).
	=>
	=>
	HGLOBAL 1
	FILE 2
	ISTREAM 4
	ISTORAGE 8
	GDI 16
	MFPICT 32
	ENHMF 64
	NULL 0
}

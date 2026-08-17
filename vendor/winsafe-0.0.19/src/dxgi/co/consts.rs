#![allow(non_camel_case_types, non_upper_case_globals)]

const_bitflag! { DXGI_ENUM_MODES: u32;
	/// [`DXGI_ENUM_MODES`](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-enum-modes)
	/// enumeration (`u32`).
	=>
	=>
	INTERLACED 1
	SCALING 2
	STEREO 4
	DISABLED_STEREO 8
}

const_ordinary! { DXGI_FORMAT: u32;
	/// [`DXGI_FORMAT`](https://learn.microsoft.com/en-us/windows/win32/api/dxgiformat/ne-dxgiformat-dxgi_format)
	/// enumeration (`u32`).
	=>
	=>
	UNKNOWN 0
	R32G32B32A32_TYPELESS 1
	R32G32B32A32_FLOAT 2
	R32G32B32A32_UINT 3
	R32G32B32A32_SINT 4
	R32G32B32_TYPELESS 5
	R32G32B32_FLOAT 6
	R32G32B32_UINT 7
	R32G32B32_SINT 8
	R16G16B16A16_TYPELESS 9
	R16G16B16A16_FLOAT 10
	R16G16B16A16_UNORM 11
	R16G16B16A16_UINT 12
	R16G16B16A16_SNORM 13
	R16G16B16A16_SINT 14
	R32G32_TYPELESS 15
	R32G32_FLOAT 16
	R32G32_UINT 17
	R32G32_SINT 18
	R32G8X24_TYPELESS 19
	D32_FLOAT_S8X24_UINT 20
	R32_FLOAT_X8X24_TYPELESS 21
	X32_TYPELESS_G8X24_UINT 22
	R10G10B10A2_TYPELESS 23
	R10G10B10A2_UNORM 24
	R10G10B10A2_UINT 25
	R11G11B10_FLOAT 26
	R8G8B8A8_TYPELESS 27
	R8G8B8A8_UNORM 28
	R8G8B8A8_UNORM_SRGB 29
	R8G8B8A8_UINT 30
	R8G8B8A8_SNORM 31
	R8G8B8A8_SINT 32
	R16G16_TYPELESS 33
	R16G16_FLOAT 34
	R16G16_UNORM 35
	R16G16_UINT 36
	R16G16_SNORM 37
	R16G16_SINT 38
	R32_TYPELESS 39
	D32_FLOAT 40
	R32_FLOAT 41
	R32_UINT 42
	R32_SINT 43
	R24G8_TYPELESS 44
	D24_UNORM_S8_UINT 45
	R24_UNORM_X8_TYPELESS 46
	X24_TYPELESS_G8_UINT 47
	R8G8_TYPELESS 48
	R8G8_UNORM 49
	R8G8_UINT 50
	R8G8_SNORM 51
	R8G8_SINT 52
	R16_TYPELESS 53
	R16_FLOAT 54
	D16_UNORM 55
	R16_UNORM 56
	R16_UINT 57
	R16_SNORM 58
	R16_SINT 59
	R8_TYPELESS 60
	R8_UNORM 61
	R8_UINT 62
	R8_SNORM 63
	R8_SINT 64
	A8_UNORM 65
	R1_UNORM 66
	R9G9B9E5_SHAREDEXP 67
	R8G8_B8G8_UNORM 68
	G8R8_G8B8_UNORM 69
	BC1_TYPELESS 70
	BC1_UNORM 71
	BC1_UNORM_SRGB 72
	BC2_TYPELESS 73
	BC2_UNORM 74
	BC2_UNORM_SRGB 75
	BC3_TYPELESS 76
	BC3_UNORM 77
	BC3_UNORM_SRGB 78
	BC4_TYPELESS 79
	BC4_UNORM 80
	BC4_SNORM 81
	BC5_TYPELESS 82
	BC5_UNORM 83
	BC5_SNORM 84
	B5G6R5_UNORM 85
	B5G5R5A1_UNORM 86
	B8G8R8A8_UNORM 87
	B8G8R8X8_UNORM 88
	R10G10B10_XR_BIAS_A2_UNORM 89
	B8G8R8A8_TYPELESS 90
	B8G8R8A8_UNORM_SRGB 91
	B8G8R8X8_TYPELESS 92
	B8G8R8X8_UNORM_SRGB 93
	BC6H_TYPELESS 94
	BC6H_UF16 95
	BC6H_SF16 96
	BC7_TYPELESS 97
	BC7_UNORM 98
	BC7_UNORM_SRGB 99
	AYUV 100
	Y410 101
	Y416 102
	NV12 103
	P010 104
	P016 105
	F420_OPAQUE 106
	YUY2 107
	Y210 108
	Y216 109
	NV11 110
	AI44 111
	IA44 112
	P8 113
	A8P8 114
	B4G4R4A4_UNORM 115
	P208 130
	V208 131
	V408 132
}

const_bitflag! { DXGI_MAP: u32;
	/// [`IDXGISurface::Map`](crate::prelude::dxgi_IDXGISurface::Map)
	/// `map_flags` (`u32`).
	=>
	=>
	READ 1
	WRITE 2
	DISCARD 4
}

const_bitflag! { DXGI_MWA: u32;
	/// [`IDXGIFactory::GetWindowAssociation`](crate::prelude::dxgi_IDXGIFactory::GetWindowAssociation)
	/// `flags` (`u32`).
	=>
	=>
	NO_WINDOW_CHANGES (1 << 0)
	NO_ALT_ENTER (1 << 1)
	NO_PRINT_SCREEN (1 << 2)
}

const_ordinary! { DXGI_MODE_ROTATION: u32;
	/// [`DXGI_MODE_ROTATION`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173065(v=vs.85))
	/// enumeration (`u32`).
	=>
	=>
	UNSPECIFIED 0
	IDENTITY 1
	ROTATE90 2
	ROTATE180 3
	ROTATE270 4
}

const_ordinary! { DXGI_MODE_SCALING: u32;
	/// [`DXGI_MODE_SCALING`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173066(v=vs.85))
	/// enumeration (`u32`).
	=>
	=>
	UNSPECIFIED 0
	CENTERED 1
	STRETCHED 2
}

const_ordinary! { DXGI_MODE_SCANLINE_ORDER: u32;
	/// [`DXGI_MODE_SCANLINE_ORDER`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173067(v=vs.85))
	/// enumeration (`u32`).
	=>
	=>
	UNSPECIFIED 0
	PROGRESSIVE 1
	UPPER_FIELD_FIRST 2
	LOWER_FIELD_FIRST 3
}

const_bitflag! { DXGI_PRESENT: u32;
	/// [`DXGI_PRESENT`](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-present)
	/// enumeration (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DO_NOT_SEQUENCE 0x0000_0002
	TEST 0x0000_0001
	RESTART 0x0000_0004
	DO_NOT_WAIT 0x0000_0008
	RESTRICT_TO_OUTPUT 0x0000_0010
	STEREO_PREFER_RIGHT 0x0000_0020
	STEREO_TEMPORARY_MONO 0x0000_0040
	USE_DURATION 0x0000_0100
	ALLOW_TEARING 0x0000_0200
}

const_ordinary! { DXGI_RESIDENCY: u32;
	/// [`DXGI_RESIDENCY`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_residency)
	/// enumeration (`u32`).
	=>
	=>
	FULLY_RESIDENT 1
	RESIDENT_IN_SHARED_MEMORY 2
	EVICTED_TO_DISK 3
}

const_ordinary! { DXGI_RESOURCE_PRIORITY: u32;
	/// [`IDXGIResource::GetEvictionPriority`](crate::prelude::dxgi_IDXGIResource::GetEvictionPriority)
	/// and
	/// [`IDXGIResource::SetEvictionPriority`](crate::prelude::dxgi_IDXGIResource::SetEvictionPriority)
	/// `eviction_priority` (`u32`).
	=>
	=>
	MINIMUM 0x2800_0000
	LOW 0x5000_0000
	NORMAL 0x7800_0000
	HIGH 0xa000_0000
	MAXIMUM 0xc800_0000
}

const_bitflag! { DXGI_SWAP_CHAIN_FLAG: u32;
	/// [`DXGI_SWAP_CHAIN_FLAG`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_swap_chain_flag)
	/// enumeration (`u32`).
	=>
	=>
	NONPREROTATED 1
	ALLOW_MODE_SWITCH 2
	GDI_COMPATIBLE 4
	RESTRICTED_CONTENT 8
	RESTRICT_SHARED_RESOURCE_DRIVER 16
	DISPLAY_ONLY 32
	FRAME_LATENCY_WAITABLE_OBJECT 64
	FOREGROUND_LAYER 128
	FULLSCREEN_VIDEO 256
	YUV_VIDEO 512
	HW_PROTECTED 1024
	ALLOW_TEARING 2048
	RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS 4096
}

const_ordinary! { DXGI_SWAP_EFFECT: u32;
	/// [`DXGI_SWAP_EFFECT`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_swap_effect)
	/// enumeration (`u32`).
	=>
	=>
	DISCARD 0
	SEQUENTIAL 1
	FLIP_SEQUENTIAL 3
	FLIP_DISCARD 4
}

const_bitflag! { DXGI_USAGE: u32;
	/// [`DXGI_USAGE`](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-usage)
	/// flags (`u32`).
	=>
	=>
	SHADER_INPUT 0x0000_0010
	RENDER_TARGET_OUTPUT 0x0000_0020
	BACK_BUFFER 0x0000_0040
	SHARED 0x0000_0080
	READ_ONLY 0x0000_0100
	DISCARD_ON_PRESENT 0x0000_0200
	UNORDERED_ACCESS 0x0000_0400
}

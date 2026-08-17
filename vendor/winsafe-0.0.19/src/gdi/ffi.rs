use crate::kernel::ffi_types::*;

extern_sys! { "gdi32";
	AbortPath(HANDLE) -> BOOL
	AngleArc(HANDLE, i32, i32, u32, f32, f32) -> BOOL
	Arc(HANDLE, i32, i32, i32, i32, i32, i32, i32, i32) -> BOOL
	ArcTo(HANDLE, i32, i32, i32, i32, i32, i32, i32, i32) -> BOOL
	BeginPath(HANDLE) -> BOOL
	BitBlt(HANDLE, i32, i32, i32, i32, HANDLE, i32, i32, u32) -> BOOL
	CancelDC(HANDLE) -> BOOL
	Chord(HANDLE, i32, i32, i32, i32, i32, i32, i32, i32) -> BOOL
	CloseFigure(HANDLE) -> BOOL
	CreateBitmap(i32, i32, u32, u32, PVOID) -> HANDLE
	CreateBrushIndirect(PCVOID) -> HANDLE
	CreateCompatibleBitmap(HANDLE, i32, i32) -> HANDLE
	CreateCompatibleDC(HANDLE) -> HANDLE
	CreateFontIndirectW(PCVOID) -> HANDLE
	CreateFontW(i32, i32, i32, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32, PCSTR) -> HANDLE
	CreateHalftonePalette(HANDLE) -> HANDLE
	CreateHatchBrush(i32, u32) -> HANDLE
	CreatePalette(PCVOID) -> HANDLE
	CreatePatternBrush(HANDLE) -> HANDLE
	CreatePen(i32, i32, u32) -> HANDLE
	CreatePenIndirect(PCVOID) -> HANDLE
	CreateRectRgn(i32, i32, i32, i32) -> HANDLE
	CreateRectRgnIndirect(PVOID) -> HANDLE
	CreateRoundRectRgn(i32, i32, i32, i32, i32, i32) -> HANDLE
	CreateSolidBrush(u32) -> HANDLE
	DeleteDC(HANDLE) -> BOOL
	DeleteObject(HANDLE) -> BOOL
	Ellipse(HANDLE, i32, i32, i32, i32) -> BOOL
	EndPath(HANDLE) -> BOOL
	FillPath(HANDLE) -> BOOL
	FillRect(HANDLE, PCVOID, HANDLE) -> i32
	FillRgn(HANDLE, HANDLE, HANDLE) -> BOOL
	FlattenPath(HANDLE) -> BOOL
	FrameRgn(HANDLE, HANDLE, HANDLE, i32, i32) -> BOOL
	GdiFlush() -> BOOL
	GdiGetBatchLimit() -> u32
	GdiSetBatchLimit(u32) -> u32
	GetBkColor(HANDLE) -> u32
	GetBkMode(HANDLE) -> i32
	GetDCBrushColor(HANDLE) -> u32
	GetDCPenColor(HANDLE) -> u32
	GetDeviceCaps(HANDLE, i32) -> i32
	GetDIBits(HANDLE, HANDLE, u32, u32, PVOID, PVOID, u32) -> i32
	GetObjectW(HANDLE, i32, PVOID) -> i32
	GetStockObject(i32) -> HANDLE
	GetStretchBltMode(HANDLE) -> i32
	GetSysColorBrush(i32) -> HANDLE
	GetTextColor(HANDLE) -> u32
	GetTextExtentPoint32W(HANDLE, PCSTR, i32, PVOID) -> BOOL
	GetTextFaceW(HANDLE, i32, PSTR) -> i32
	GetTextMetricsW(HANDLE, PVOID) -> BOOL
	GetViewportExtEx(HANDLE, PVOID) -> BOOL
	GetViewportOrgEx(HANDLE, PVOID) -> BOOL
	GetWindowExtEx(HANDLE, PVOID) -> BOOL
	GetWindowOrgEx(HANDLE, PVOID) -> BOOL
	LineTo(HANDLE, i32, i32) -> BOOL
	MoveToEx(HANDLE, i32, i32, PVOID) -> BOOL
	OffsetClipRgn(HANDLE, i32, i32) -> i32
	OffsetRgn(HANDLE, i32, i32) -> i32
	PatBlt(HANDLE, i32, i32, i32, i32, u32) -> BOOL
	PathToRegion(HANDLE) -> HANDLE
	Pie(HANDLE, i32, i32, i32, i32, i32, i32, i32, i32) -> BOOL
	PolyBezier(HANDLE, PCVOID, u32) -> BOOL
	PolyBezierTo(HANDLE, PCVOID, u32) -> BOOL
	Polyline(HANDLE, PCVOID, u32) -> BOOL
	PolylineTo(HANDLE, PCVOID, u32) -> BOOL
	PtInRegion(HANDLE, i32, i32) -> BOOL
	PtVisible(HANDLE, i32, i32) -> BOOL
	RealizePalette(HANDLE) -> u32
	Rectangle(HANDLE, i32, i32, i32, i32) -> BOOL
	RectInRegion(HANDLE, PCVOID) -> BOOL
	RestoreDC(HANDLE, i32) -> BOOL
	RoundRect(HANDLE, i32, i32, i32, i32, i32, i32) -> BOOL
	SaveDC(HANDLE) -> i32
	SelectClipPath(HANDLE, i32) -> BOOL
	SelectClipRgn(HANDLE, HANDLE) -> i32
	SelectObject(HANDLE, HANDLE) -> HANDLE
	SelectPalette(HANDLE, HANDLE, BOOL) -> HANDLE
	SetArcDirection(HANDLE, i32) -> i32
	SetBkColor(HANDLE, u32) -> u32
	SetBkMode(HANDLE, i32) -> i32
	SetBrushOrgEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetDCBrushColor(HANDLE, u32) -> u32
	SetDCPenColor(HANDLE, u32) -> u32
	SetDIBits(HANDLE, HANDLE, u32, u32, PCVOID, PCVOID, u32) -> i32
	SetGraphicsMode(HANDLE, i32) -> i32
	SetStretchBltMode(HANDLE, i32) -> i32
	SetTextAlign(HANDLE, u32) -> u32
	SetTextColor(HANDLE, u32) -> u32
	SetTextJustification(HANDLE, i32, i32) -> BOOL
	SetViewportExtEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetViewportOrgEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetWindowExtEx(HANDLE, i32, i32, PVOID) -> BOOL
	SetWindowOrgEx(HANDLE, i32, i32, PVOID) -> BOOL
	StretchBlt(HANDLE, i32, i32, i32, i32, HANDLE, i32, i32, i32, i32, u32) -> BOOL
	StrokeAndFillPath(HANDLE) -> BOOL
	StrokePath(HANDLE) -> BOOL
	TextOutW(HANDLE, i32, i32, PCSTR, i32) -> BOOL
	UnrealizeObject(HANDLE) -> BOOL
	UpdateColors(HANDLE) -> BOOL
	WidenPath(HANDLE) -> BOOL
}

extern_sys! { "msimg32";
	TransparentBlt(HANDLE, i32, i32, i32, i32, HANDLE, i32, i32, i32, i32, u32) -> BOOL
}

extern_sys! { "user32";
	LoadImageW(HANDLE, PCSTR, u32, i32, i32, u32) -> HANDLE // returns GdiObjectGuard, so needs gdi feature
}

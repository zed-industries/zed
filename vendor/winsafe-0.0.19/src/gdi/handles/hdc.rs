#![allow(non_camel_case_types, non_snake_case)]

use std::any::TypeId;

use crate::co;
use crate::decl::*;
use crate::gdi::{ffi, privs::*};
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl gdi_Hdc for HDC {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HDC`](crate::HDC).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hdc: user_Hdc {
	/// [`AborthPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-abortpath)
	/// function.
	fn AbortPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::AbortPath(self.ptr()) })
	}

	/// [`AngleArc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-anglearc)
	/// function.
	fn AngleArc(&self,
		center: POINT,
		radius: u32,
		start_angle: f32,
		sweep_angle: f32,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::AngleArc(
					self.ptr(),
					center.x, center.y,
					radius, start_angle, sweep_angle,
				)
			},
		)
	}

	/// [`Arc`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arc)
	/// function.
	fn Arc(&self,
		bound: RECT,
		radial_start: POINT,
		radial_end: POINT,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::Arc(
					self.ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
					radial_start.x, radial_start.y,
					radial_end.x, radial_end.y,
				)
			},
		)
	}

	/// [`ArcTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-arcto)
	/// function.
	fn ArcTo(&self,
		bound: RECT,
		radial_start: POINT,
		radial_end: POINT,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::ArcTo(
					self.ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
					radial_start.x, radial_start.y,
					radial_end.x, radial_end.y,
				)
			},
		)
	}

	/// [`BeginPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-beginpath)
	/// function.
	fn BeginPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::BeginPath(self.ptr()) })
	}

	/// [`BitBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt)
	/// function.
	fn BitBlt(&self,
		dest_pos: POINT,
		sz: SIZE,
		hdc_src: &HDC,
		src_src: POINT,
		rop: co::ROP,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::BitBlt(
					self.ptr(),
					dest_pos.x, dest_pos.y,
					sz.cx, sz.cy,
					hdc_src.ptr(),
					src_src.x, src_src.y,
					rop.raw(),
				)
			},
		)
	}

	/// [`CancelDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-canceldc)
	/// function.
	fn CancelDC(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::CancelDC(self.ptr()) })
	}

	/// [`Chord`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-chord)
	/// function.
	fn Chord(&self,
		bounds: RECT,
		start_radial: POINT,
		end_radial: POINT,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::Chord(
					self.ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					start_radial.x, start_radial.y,
					end_radial.x, end_radial.y,
				)
			},
		)
	}

	/// [`CloseFigure`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-closefigure)
	/// function.
	fn CloseFigure(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::CloseFigure(self.ptr()) })
	}

	/// [`CreateCompatibleBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatiblebitmap)
	/// function.
	#[must_use]
	fn CreateCompatibleBitmap(&self,
		cx: i32,
		cy: i32,
	) -> SysResult<DeleteObjectGuard<HBITMAP>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateCompatibleBitmap(self.ptr(), cx, cy),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateCompatibleDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// function.
	#[must_use]
	fn CreateCompatibleDC(&self) -> SysResult<DeleteDCGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::CreateCompatibleDC(self.ptr()))
				.map(|h| DeleteDCGuard::new(h))
		}
	}

	/// [`CreateHalftonePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhalftonepalette)
	/// function.
	#[must_use]
	fn CreateHalftonePalette(&self) -> SysResult<DeleteObjectGuard<HPALETTE>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::CreateHalftonePalette(self.ptr()))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`Ellipse`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ellipse)
	/// function.
	fn Ellipse(&self, bound: RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::Ellipse(
					self.ptr(),
					bound.left, bound.top,
					bound.right, bound.bottom,
				)
			},
		)
	}

	/// [`EndPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-endpath)
	/// function.
	fn EndPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::EndPath(self.ptr()) })
	}

	/// [`FillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillpath)
	/// function.
	fn FillPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::FillPath(self.ptr()) })
	}

	/// [`FillRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	/// function.
	fn FillRect(&self, rc: RECT, hbr: &HBRUSH) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::FillRect(self.ptr(), &rc as *const _ as _, hbr.ptr()) },
		)
	}

	/// [`FillRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-fillrgn)
	/// function.
	fn FillRgn(&self, rgn: &HRGN, brush: &HBRUSH) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::FillRgn(self.ptr(), rgn.ptr(), brush.ptr()) },
		)
	}

	/// [`FlattenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-flattenpath)
	/// function.
	fn FlattenPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::FlattenPath(self.ptr()) })
	}

	/// [`FrameRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-framergn)
	/// function.
	fn FrameRgn(&self,
		rgn: &HRGN,
		brush: &HBRUSH,
		w: i32,
		h: i32,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::FrameRgn(
					self.ptr(),
					rgn.ptr(),
					brush.ptr(),
					w, h,
				)
			},
		)
	}

	/// [`GetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkcolor)
	/// function.
	#[must_use]
	fn GetBkColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetBkColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			c => Ok(unsafe { COLORREF::from_raw(c) }),
		}
	}

	/// [`GetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbkmode)
	/// function.
	#[must_use]
	fn GetBkMode(&self) -> SysResult<co::BKMODE> {
		match unsafe { ffi::GetBkMode(self.ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::BKMODE::from_raw(v) }),
		}
	}

	/// [`GetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcbrushcolor)
	/// function.
	#[must_use]
	fn GetDCBrushColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetDCBrushColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdcpencolor)
	/// function.
	#[must_use]
	fn GetDCPenColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetDCPenColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetDIBits`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdibits)
	/// function.
	///
	/// # Safety
	///
	/// If `bmpDataBuf` is smaller than needed, you'll have a buffer overflow.
	///
	/// # Examples
	///
	/// Taking a screenshot and saving to file:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let cx_screen = w::GetSystemMetrics(co::SM::CXSCREEN);
	/// let cy_screen = w::GetSystemMetrics(co::SM::CYSCREEN);
	///
	/// let hdc_screen = w::HWND::DESKTOP.GetDC()?;
	/// let hbmp = hdc_screen.CreateCompatibleBitmap(cx_screen, cy_screen)?;
	/// let hdc_mem = hdc_screen.CreateCompatibleDC()?;
	/// let _hbmp_guard = hdc_mem.SelectObject(&*hbmp)?;
	///
	/// hdc_mem.BitBlt(w::POINT::new(0, 0), w::SIZE::new(cx_screen, cy_screen),
	///     &hdc_screen, w::POINT::new(0, 0), co::ROP::SRCCOPY)?;
	///
	/// let mut bmp_obj = w::BITMAP::default();
	/// hbmp.GetObject(&mut bmp_obj)?;
	///
	/// let mut bi = w::BITMAPINFO::default();
	/// bi.bmiHeader.biWidth = cx_screen;
	/// bi.bmiHeader.biHeight = cy_screen;
	/// bi.bmiHeader.biPlanes = 1;
	/// bi.bmiHeader.biBitCount = 32;
	/// bi.bmiHeader.biCompression = co::BI::RGB;
	///
	/// let bmp_size = (bmp_obj.bmWidth * (bi.bmiHeader.biBitCount as i32) + 31)
	///     / 32 * 4 * bmp_obj.bmHeight;
	/// let mut data_buf = vec![0u8; bmp_size as _];
	///
	/// unsafe {
	///     hdc_screen.GetDIBits(&hbmp, 0, cy_screen as _,
	///         Some(&mut data_buf), &mut bi, co::DIB::RGB_COLORS)?;
	/// }
	///
	/// let mut bfh = w::BITMAPFILEHEADER::default();
	/// bfh.bfOffBits = (std::mem::size_of::<w::BITMAPFILEHEADER>()
	///     + std::mem::size_of::<w::BITMAPINFOHEADER>()) as _;
	/// bfh.bfSize = bfh.bfOffBits + (bmp_size as u32);
	///
	/// let fo = w::File::open("C:\\Temp\\foo.bmp", w::FileAccess::OpenOrCreateRW)?;
	/// fo.write(bfh.serialize())?;
	/// fo.write(bi.bmiHeader.serialize())?;
	/// fo.write(&data_buf)?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	unsafe fn GetDIBits(&self,
		hbm: &HBITMAP,
		first_scan_line: u32,
		num_scan_lines: u32,
		bmp_data_buf: Option<&mut [u8]>,
		bmi: &mut BITMAPINFO,
		usage: co::DIB,
	) -> SysResult<i32>
	{
		let ret = ffi::GetDIBits(
			self.ptr(),
			hbm.ptr(),
			first_scan_line,
			num_scan_lines,
			bmp_data_buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr() as _),
			bmi as *const _ as _,
			usage.raw(),
		);

		if unsafe { co::ERROR::from_raw(ret as _) } == co::ERROR::INVALID_PARAMETER {
			Err(co::ERROR::INVALID_PARAMETER)
		} else if ret == 0 {
			Err(GetLastError())
		} else {
			Ok(ret)
		}
	}

	/// [`GetDeviceCaps`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps)
	/// function.
	#[must_use]
	fn GetDeviceCaps(&self, index: co::GDC) -> i32 {
		unsafe { ffi::GetDeviceCaps(self.ptr(), index.raw()) }
	}

	/// [`GetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstretchbltmode)
	/// function.
	#[must_use]
	fn GetStretchBltMode(&self) -> SysResult<co::STRETCH_MODE> {
		match unsafe { ffi::GetStretchBltMode(self.ptr()) } {
			0 => Err(GetLastError()),
			sm => Ok(unsafe { co::STRETCH_MODE::from_raw(sm) }),
		}
	}

	/// [`GetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextcolor)
	/// function.
	#[must_use]
	fn GetTextColor(&self) -> SysResult<COLORREF> {
		match unsafe { ffi::GetTextColor(self.ptr()) } {
			CLR_INVALID => Err(GetLastError()),
			color => Ok(unsafe { COLORREF::from_raw(color) }),
		}
	}

	/// [`GetTextExtentPoint32`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// function.
	#[must_use]
	fn GetTextExtentPoint32(&self, text: &str) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				ffi::GetTextExtentPoint32W(
					self.ptr(),
					WString::from_str(text).as_ptr(),
					text.chars().count() as _,
					&mut sz as *mut _ as _,
				)
			},
		).map(|_| sz)
	}

	/// [`GetTextFace`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextfacew)
	/// function.
	#[must_use]
	fn GetTextFace(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(LF_FACESIZE + 1);
		match unsafe {
			ffi::GetTextFaceW(
				self.ptr(),
				buf.buf_len() as _,
				buf.as_mut_ptr(),
			)
		} {
			0 => Err(GetLastError()),
			v => Ok(v),
		}.map(|_| buf.to_string())
	}

	/// [`GetTextMetrics`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextmetricsw)
	/// function.
	fn GetTextMetrics(&self, tm: &mut TEXTMETRIC) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::GetTextMetricsW(self.ptr(), tm as *mut _ as _) },
		)
	}

	/// [`GetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportextex)
	/// function.
	#[must_use]
	fn GetViewportExtEx(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe { ffi::GetViewportExtEx(self.ptr(), &mut sz as *mut _ as _) },
		).map(|_| sz)
	}

	/// [`GetViewportOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getviewportorgex)
	/// function.
	#[must_use]
	fn GetViewportOrgEx(&self) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe { ffi::GetViewportOrgEx(self.ptr(), &mut pt as *mut _ as _) },
		).map(|_| pt)
	}

	/// [`GetWindowExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindowextex)
	/// function.
	#[must_use]
	fn GetWindowExtEx(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe { ffi::GetWindowExtEx(self.ptr(), &mut sz as *mut _ as _) },
		).map(|_| sz)
	}

	/// [`GetWindowOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getwindoworgex)
	/// function.
	#[must_use]
	fn GetWindowOrgEx(&self) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe { ffi::GetWindowOrgEx(self.ptr(), &mut pt as *mut _ as _) },
		).map(|_| pt)
	}

	/// [`AtlHiMetricToPixel`](https://learn.microsoft.com/en-us/cpp/atl/reference/pixel-himetric-conversion-global-functions?view=msvc-170#atlhimetrictopixel)
	/// function.
	///
	/// Converts HIMETRIC units to pixels. The inverse operation is
	/// [`HDC::PixelToHiMetric`](crate::prelude::gdi_Hdc::PixelToHiMetric).
	#[must_use]
	fn HiMetricToPixel(&self, x: i32, y: i32) -> (i32, i32) {
		// http://www.verycomputer.com/5_5f2f75dc2d090ee8_1.htm
		// https://forums.codeguru.com/showthread.php?109554-Unresizable-activeX-control
		(
			MulDiv(x, self.GetDeviceCaps(co::GDC::LOGPIXELSX), HIMETRIC_PER_INCH),
			MulDiv(y, self.GetDeviceCaps(co::GDC::LOGPIXELSY), HIMETRIC_PER_INCH),
		)
	}

	/// [`LineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// function.
	fn LineTo(&self, x: i32, y: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::LineTo(self.ptr(), x, y) })
	}

	/// [`MoveToEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// function.
	fn MoveToEx(&self, x: i32, y: i32, pt: Option<&mut POINT>) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::MoveToEx(
					self.ptr(),
					x, y,
					pt.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				)
			},
		)
	}

	/// [`PatBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-patblt)
	/// function.
	fn PatBlt(&self, top_left: POINT, sz: SIZE, rop: co::ROP) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::PatBlt(
					self.ptr(), top_left.x, top_left.y, sz.cx, sz.cy, rop.raw(),
				)
			},
		)
	}

	/// [`PathToRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pathtoregion)
	/// function.
	#[must_use]
	fn PathToRegion(&self) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::PathToRegion(self.ptr()))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`Pie`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// function.
	fn Pie(&self,
		bounds: RECT,
		radial_1: POINT,
		radial_2: POINT,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::Pie(
					self.ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					radial_1.x, radial_1.y,
					radial_2.y, radial_2.y,
				)
			},
		)
	}

	/// [`AtlPixelToHiMetric`](https://learn.microsoft.com/en-us/cpp/atl/reference/pixel-himetric-conversion-global-functions?view=msvc-170#atlpixeltohimetric)
	/// function.
	///
	/// Converts pixels to HIMETRIC units. The inverse operation is
	/// [`HDC::HiMetricToPixel`](crate::prelude::gdi_Hdc::HiMetricToPixel).
	#[must_use]
	fn PixelToHiMetric(&self, x: i32, y: i32) -> (i32, i32) {
		(
			MulDiv(x, HIMETRIC_PER_INCH, self.GetDeviceCaps(co::GDC::LOGPIXELSX)),
			MulDiv(y, HIMETRIC_PER_INCH, self.GetDeviceCaps(co::GDC::LOGPIXELSY)),
		)
	}

	/// [`PolyBezier`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// function.
	fn PolyBezier(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::PolyBezier(self.ptr(), vec_ptr(pts) as _, pts.len() as _)
			},
		)
	}

	/// [`PolyBezierTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// function.
	fn PolyBezierTo(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::PolyBezierTo(self.ptr(), vec_ptr(pts) as _, pts.len() as _)
			},
		)
	}

	/// [`Polyline`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// function.
	fn Polyline(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::Polyline(self.ptr(), vec_ptr(pts) as _, pts.len() as _)
			},
		)
	}

	/// [`PolylineTo`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// function.
	fn PolylineTo(&self, pts: &[POINT]) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::PolylineTo(self.ptr(), vec_ptr(pts) as _, pts.len() as _)
			},
		)
	}

	/// [`PtVisible`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// function.
	#[must_use]
	fn PtVisible(&self, x: i32, y: i32) -> SysResult<bool> {
		match unsafe { ffi::PtVisible(self.ptr(), x, y) } {
			-1 => Err(GetLastError()),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`RealizePalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-realizepalette)
	/// function.
	fn RealizePalette(&self) -> SysResult<u32> {
		match unsafe { ffi::RealizePalette(self.ptr()) } {
			GDI_ERROR => Err(GetLastError()),
			num => Ok(num),
		}
	}

	/// [`Rectangle`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// function.
	fn Rectangle(&self, bounds: RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::Rectangle(self.ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom)
			},
		)
	}

	/// [`RestoreDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// function.
	fn RestoreDC(&self, saved_dc: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::RestoreDC(self.ptr(), saved_dc) })
	}

	/// [`RoundRect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// function.
	fn RoundRect(&self, bounds: RECT, sz: SIZE) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::RoundRect(
					self.ptr(),
					bounds.left, bounds.top, bounds.right, bounds.bottom,
					sz.cx, sz.cy,
				)
			},
		)
	}

	/// [`SaveDC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// function.
	fn SaveDC(&self) -> SysResult<i32> {
		match unsafe { ffi::SaveDC(self.ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(v),
		}
	}

	/// [`SelectClipPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectclippath)
	/// function.
	fn SelectClipPath(&self, mode: co::RGN) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SelectClipPath(self.ptr(), mode.raw()) })
	}

	/// [`SelectClipRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectcliprgn)
	/// function.
	fn SelectClipRgn(&self, rgn: &HRGN) -> SysResult<co::REGION> {
		match unsafe { ffi::SelectClipRgn(self.ptr(), rgn.ptr()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::REGION::from_raw(v) }),
		}
	}

	/// [`SelectObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// function.
	///
	/// In the original C implementation, `SelectObject` returns a handle to the
	/// object being replaced. You must perform a cleanup operation, calling
	/// `SelectObject` again, passing the handle to the replaced object.
	///
	/// Here, the cleanup is performed automatically, because `SelectObject`
	/// returns a [`SelectObjectGuard`](crate::guard::SelectObjectGuard), which
	/// stores the replaced handle and calls `SelectObject` automatically when
	/// the guard goes out of scope. You must, however, keep the guard alive,
	/// otherwise the cleanup will be performed right away.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hdc: w::HDC; // initialized somewhere
	/// # let hdc = w::HDC::NULL;
	///
	/// let hpen = w::HPEN::CreatePen(
	///     co::PS::SOLID,
	///     1,
	///     w::COLORREF::new(0xff, 0x00, 0x88),
	/// )?;
	///
	/// let _pen_guard = hdc.SelectObject(&*hpen); // keep guard alive
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn SelectObject<G>(&self,
		hgdiobj: &G,
	) -> SysResult<SelectObjectGuard<'_, Self, G>>
		where G: GdiObjectSelect,
	{
		unsafe {
			ptr_to_sysresult(ffi::SelectObject(self.ptr(), hgdiobj.ptr()))
				.map(|ptr| {
					if hgdiobj.type_id() == TypeId::of::<HRGN>() {
						SelectObjectGuard::new(
							self,
							G::NULL, // regions don't need cleanup
							Some(co::REGION::from_raw(ptr as *mut _ as _)),
						)
					} else {
						SelectObjectGuard::new(
							self,
							G::from_ptr(ptr), // GDI object to cleanup
							None,
						)
					}
				})
		}
	}

	/// [`SelectPalette`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectpalette)
	/// function.
	fn SelectPalette(&self,
		hpal: &HPALETTE,
		force_bkgd: bool,
	) -> SysResult<Option<HPALETTE>>
	{
		let ptr = unsafe {
			ffi::SelectPalette(self.ptr(), hpal.ptr(), force_bkgd as _)
		};

		if ptr.is_null() {
			match GetLastError() {
				co::ERROR::SUCCESS => Ok(None),
				err => Err(err),
			}
		} else {
			Ok(Some(unsafe { HPALETTE::from_ptr(ptr) }))
		}
	}

	/// [`SetArcDirection`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setarcdirection)
	/// function.
	fn SetArcDirection(&self, dir: co::AD) -> SysResult<co::AD> {
		match unsafe { ffi::SetArcDirection(self.ptr(), dir.raw()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::AD::from_raw(v) }),
		}
	}

	/// [`SetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkcolor)
	/// function.
	fn SetBkColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetBkColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetBkMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// function.
	fn SetBkMode(&self, mode: co::BKMODE) -> SysResult<co::BKMODE> {
		match unsafe { ffi::SetBkMode(self.ptr(), mode.raw()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::BKMODE::from_raw(v) }),
		}
	}

	/// [`SetBrushOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbrushorgex)
	/// function.
	fn SetBrushOrgEx(&self, new_origin: POINT) -> SysResult<POINT> {
		let mut old_origin = POINT::default();
		bool_to_sysresult(
			unsafe {
				ffi::SetBrushOrgEx(
					self.ptr(),
					new_origin.x, new_origin.y,
					&mut old_origin as *mut _ as _,
				)
			},
		).map(|_| old_origin)
	}

	/// [`SetDCBrushColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcbrushcolor)
	/// function.
	fn SetDCBrushColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetDCBrushColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetDCPenColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdcpencolor)
	/// function.
	fn SetDCPenColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetDCPenColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetDIBits`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdibits)
	/// function.
	fn SetDIBits(&self,
		hbm: &HBITMAP,
		first_scan_line: u32,
		num_scan_lines: u32,
		dib_color_data: &[u8],
		bmi: &BITMAPINFO,
		color_use: co::DIB,
	) -> SysResult<i32>
	{
		match unsafe {
			ffi::SetDIBits(
				self.ptr(),
				hbm.ptr(),
				first_scan_line,
				num_scan_lines,
				vec_ptr(dib_color_data) as _,
				bmi as *const _ as _,
				color_use.raw(),
			)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0),
				err => Err(err),
			},
			n => Ok(n),
		}
	}

	/// [`SetGraphicsMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setgraphicsmode)
	/// function.
	fn SetGraphicsMode(&self, mode: co::GM) -> SysResult<co::GM> {
		match unsafe { ffi::SetGraphicsMode(self.ptr(), mode.raw()) } {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::GM::from_raw(v) })
		}
	}

	/// [`SetStretchBltMode`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setstretchbltmode)
	/// function.
	fn SetStretchBltMode(&self,
		mode: co::STRETCH_MODE,
	) -> SysResult<co::STRETCH_MODE>
	{
		match unsafe {
			co::ERROR::from_raw(
				ffi::SetStretchBltMode(self.ptr(), mode.raw()) as _,
			)
		} {
			co::ERROR::INVALID_PARAMETER => Err(co::ERROR::INVALID_PARAMETER),
			err_val => Ok(unsafe { co::STRETCH_MODE::from_raw(err_val.raw() as _) }),
		}
	}

	/// [`SetTextAlign`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextalign)
	/// function.
	fn SetTextAlign(&self, align: co::TA) -> SysResult<co::TA> {
		match unsafe { ffi::SetTextAlign(self.ptr(), align.raw()) } {
			GDI_ERROR => Err(GetLastError()),
			ta => Ok(unsafe { co::TA::from_raw(ta) }),
		}
	}

	/// [`SetTextColor`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextcolor)
	/// function.
	fn SetTextColor(&self, color: COLORREF) -> SysResult<COLORREF> {
		match unsafe { ffi::SetTextColor(self.ptr(), color.into()) } {
			CLR_INVALID => Err(GetLastError()),
			old => Ok(unsafe { COLORREF::from_raw(old) }),
		}
	}

	/// [`SetTextJustification`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextjustification)
	/// function.
	fn SetTextJustification(&self, extra: i32, count: i32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::SetTextJustification(self.ptr(), extra, count) },
		)
	}

	/// [`SetViewportExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportextex)
	/// function.
	fn SetViewportExtEx(&self, x: i32, y: i32) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				ffi::SetViewportExtEx(self.ptr(), x, y, &mut sz as *mut _ as _)
			},
		).map(|_| sz)
	}

	/// [`SetViewportOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setviewportorgex)
	/// function.
	fn SetViewportOrgEx(&self, x: i32, y: i32) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe {
				ffi::SetViewportOrgEx(self.ptr(), x, y, &mut pt as *mut _ as _)
			},
		).map(|_| pt)
	}

	/// [`SetWindowExtEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindowextex)
	/// function.
	fn SetWindowExtEx(&self, x: i32, y: i32) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				ffi::SetWindowExtEx(self.ptr(), x, y, &mut sz as *mut _ as _)
			},
		).map(|_| sz)
	}

	/// [`SetWindowOrgEx`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setwindoworgex)
	/// function.
	fn SetWindowOrgEx(&self, x: i32, y: i32) -> SysResult<POINT> {
		let mut pt = POINT::default();
		bool_to_sysresult(
			unsafe {
				ffi::SetWindowOrgEx(self.ptr(), x, y, &mut pt as *mut _ as _)
			},
		).map(|_| pt)
	}

	/// [`StretchBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchblt)
	/// function.
	fn StretchBlt(&self,
		pos_dest: POINT,
		sz_dest: SIZE,
		hdc_src: &HDC,
		pt_src: POINT,
		sz_src: SIZE,
		rop: co::ROP,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::StretchBlt(
					self.ptr(),
					pos_dest.x, pos_dest.y,
					sz_dest.cx, sz_dest.cy,
					hdc_src.ptr(),
					pt_src.x, pt_src.y,
					sz_src.cx, sz_src.cy,
					rop.raw(),
				)
			},
		)
	}

	/// [`StrokeAndFillPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokeandfillpath)
	/// function.
	fn StrokeAndFillPath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::StrokeAndFillPath(self.ptr()) })
	}

	/// [`StrokePath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-strokepath)
	/// function.
	fn StrokePath(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::StrokePath(self.ptr()) })
	}

	/// [`TextOut`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-textoutw)
	/// function.
	fn TextOut(&self, x: i32, y: i32, text: &str) -> SysResult<()> {
		let output = WString::from_str(text);
		bool_to_sysresult(
			unsafe {
				ffi::TextOutW(
					self.ptr(),
					x, y,
					output.as_ptr(),
					output.str_len() as _,
				)
			},
		)
	}

	/// [`TransparentBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt)
	/// function.
	fn TransparentBlt(&self,
		dest_top_left: POINT,
		dest_sz: SIZE,
		hdc_src: HDC,
		src_top_left: POINT,
		src_sz: SIZE,
		color_transparent: COLORREF,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::TransparentBlt(
					self.ptr(),
					dest_top_left.x, dest_top_left.y,
					dest_sz.cx, dest_sz.cy,
					hdc_src.ptr(),
					src_top_left.x, src_top_left.y,
					src_sz.cx, src_sz.cy,
					color_transparent.into(),
				)
			},
		)
	}

	/// [`UpdateColors`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-updatecolors)
	/// function.
	fn UpdateColors(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::UpdateColors(self.ptr()) })
	}

	/// [`WidenPath`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-widenpath)
	/// function.
	fn WidenPath(&self) -> SysResult<()>  {
		bool_to_sysresult(unsafe { ffi::WidenPath(self.ptr()) })
	}
}

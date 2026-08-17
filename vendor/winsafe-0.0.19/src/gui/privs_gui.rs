//! Global objects used within `gui` module.

use std::error::Error;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::gui::{*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Global return error originated from an event handling closure; will be taken
/// in main loop.
pub(in crate::gui) static mut QUIT_ERROR: Option<MsgError> = None;

/// Terminates the program with the given error.
pub(in crate::gui) fn post_quit_error(
	src_msg: WndMsg,
	err: Box<dyn Error + Send + Sync>,
) {
	unsafe { QUIT_ERROR = Some(MsgError::new(src_msg, err)); } // store the error, so Base::run_main_loop() can grab it
	PostQuitMessage(-1); // this -1 will be discarded in the main loop, anyway
}

//------------------------------------------------------------------------------

/// Global UI font object.
static mut UI_HFONT: Option<DeleteObjectGuard<HFONT>> = None;

/// Creates the global UI font object.
pub(in crate::gui) fn create_ui_font() -> SysResult<()> {
	let mut ncm = NONCLIENTMETRICS::default();
	unsafe {
		SystemParametersInfo(
			co::SPI::GETNONCLIENTMETRICS,
			std::mem::size_of::<NONCLIENTMETRICS>() as _,
			&mut ncm,
			co::SPIF::NoValue,
		)?;
		UI_HFONT = Some(HFONT::CreateFontIndirect(&ncm.lfMenuFont)?);
	}
	Ok(())
}

/// Frees the global UI font object.
pub(in crate::gui) fn delete_ui_font() {
	unsafe { UI_HFONT = None; } // https://users.rust-lang.org/t/why-drop-trait-not-called-when-use-global-static
}

/// Retrieves the global UI font object, or panics if not created yet.
pub(in crate::gui) fn ui_font() -> &'static HFONT {
	unsafe {
		match &UI_HFONT {
			Some(hfont) => hfont,
			None => panic!("Global UI font not created."),
		}
	}
}

//------------------------------------------------------------------------------

static mut BASE_CTRL_ID: u16 = 20_000; // in-between Visual Studio Resource Editor values

/// Returns the next sequential control ID.
pub(in crate::gui) fn auto_ctrl_id() -> u16 {
	unsafe {
		let new_id = BASE_CTRL_ID;
		BASE_CTRL_ID += 1;
		new_id
	}
}

//------------------------------------------------------------------------------

static mut DPI: POINT = POINT::new(0, 0);

/// Multiplies the given coordinates by current system DPI.
pub(in crate::gui) fn multiply_dpi(
	pt: Option<&mut POINT>,
	sz: Option<&mut SIZE>,
) -> SysResult<()>
{
	unsafe {
		if (pt.is_some() || sz.is_some()) && DPI.x == 0 { // DPI not cached yet?
			let screen_dc = HWND::NULL.GetDC()?;
			DPI.x = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSX); // cache
			DPI.y = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSY);
		}

		if let Some(pt) = pt {
			pt.x = MulDiv(pt.x, DPI.x, 96);
			pt.y = MulDiv(pt.y, DPI.y, 96);
		}
		if let Some(sz) = sz {
			sz.cx = MulDiv(sz.cx, DPI.x, 96);
			sz.cy = MulDiv(sz.cy, DPI.y, 96);
		}
	}
	Ok(())
}

/// If parent is a dialog, converts Dialog Template Units to pixels; otherwise
/// multiplies by current DPI factor.
pub(in crate::gui) fn multiply_dpi_or_dtu(
	parent_base: &Base,
	pt: Option<&mut POINT>,
	sz: Option<&mut SIZE>,
) -> SysResult<()>
{
	if parent_base.is_dialog() {
		let mut rc = RECT::default();
		pt.as_ref().map(|pt| {
			rc.left = pt.x;
			rc.top = pt.y;
		});
		sz.as_ref().map(|sz| {
			rc.right = sz.cx;
			rc.bottom = sz.cy;
		});

		parent_base.hwnd().MapDialogRect(&mut rc)?;
		let (mut pt, mut sz) = (pt, sz);
		pt.as_mut().map(|pt| {
			pt.x = rc.left;
			pt.y = rc.top;
		});
		sz.as_mut().map(|sz| {
			sz.cx = rc.right;
			sz.cy = rc.bottom;
		});

	} else {
		multiply_dpi(pt, sz)?;
	}

	Ok(())
}

//------------------------------------------------------------------------------

/// Calculates the bound rectangle to fit the text with current system font.
pub(in crate::gui) fn calc_text_bound_box(text: &str) -> SysResult<SIZE> {
	let desktop_hwnd = HWND::GetDesktopWindow();
	let desktop_hdc = desktop_hwnd.GetDC()?;
	let clone_dc = desktop_hdc.CreateCompatibleDC()?;
	let _prev_font = clone_dc.SelectObject(ui_font())?;

	let mut bounds = if text.is_empty() {
		clone_dc.GetTextExtentPoint32("Pj")? // just a placeholder to get the text height
	} else {
		clone_dc.GetTextExtentPoint32(&remove_accelerator_ampersands(text))?
	};

	if text.is_empty() {
		bounds.cx = 0; // if no text was given, return just the height
	}
	Ok(bounds)
}

/// Calculates the bound rectangle to fit the text with current system font,
/// adding a check box.
pub(in crate::gui) fn calc_text_bound_box_check(text: &str) -> SysResult<SIZE> {
	let mut bound_box = calc_text_bound_box(text)?;
	bound_box.cx += GetSystemMetrics(co::SM::CXMENUCHECK) // https://stackoverflow.com/a/1165052/6923555
		+ GetSystemMetrics(co::SM::CXEDGE);

	let cy_check = GetSystemMetrics(co::SM::CYMENUCHECK);
	if cy_check > bound_box.cy {
		bound_box.cy = cy_check; // if the check is taller than the font, use its height
	}

	Ok(bound_box)
}

fn remove_accelerator_ampersands(text: &str) -> String {
	let mut txt_no_ampersands = String::with_capacity(text.len());
	let mut last_ch = 'a'; // initial value will be skipped

	for (idx, ch) in text.char_indices() {
		if idx == 0 { // first char
			if ch != '&' {
				txt_no_ampersands.push(ch);
			}
		} else if ch != '&' || (ch == '&' && last_ch == '&') {
			txt_no_ampersands.push(ch);
		}
		last_ch = ch;
	}

	txt_no_ampersands
}

//------------------------------------------------------------------------------

/// Adjusts the position of a modeless window on parent.
pub(in crate::gui) fn adjust_modeless_pos(
	parent_base: &Base,
	mut user_pos: POINT,
) -> SysResult<POINT>
{
	// For a modeless (0,0) is the left/topmost point in parent's client area.
	multiply_dpi_or_dtu(parent_base, Some(&mut user_pos), None)?;
	let hparent = parent_base.hwnd();
	let mut parent_rc = hparent.GetClientRect()?;
	hparent.ClientToScreenRc(&mut parent_rc)?;
	user_pos.x += parent_rc.left;
	user_pos.y += parent_rc.top;
	Ok(user_pos)
}

/// Paints the themed border of an user control, if it has the proper styles.
pub(in crate::gui) fn paint_control_borders(
	hwnd: &HWND,
	wm_ncp: wm::NcPaint,
) -> AnyResult<()>
{
	hwnd.DefWindowProc(wm_ncp); // let the system draw the scrollbar for us

	let ex_style = unsafe { co::WS_EX::from_raw(hwnd.GetWindowLongPtr(co::GWLP::EXSTYLE) as _) };
	if !ex_style.has(co::WS_EX::CLIENTEDGE) // no border
		|| !IsThemeActive()
		|| !IsAppThemed()
	{
		return Ok(());
	}

	let mut rc = hwnd.GetWindowRect()?; // window outmost coordinates, including margins
	hwnd.ScreenToClientRc(&mut rc)?;
	rc.left += 2; rc.top += 2; rc.right += 2; rc.bottom += 2; // because it comes up anchored at -2,-2

	let hdc = hwnd.GetWindowDC()?;

	if let Some(htheme) = hwnd.OpenThemeData("LISTVIEW") {
		// Draw only the borders to avoid flickering.
		htheme.DrawThemeBackground(&hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.left, top: rc.top, right: rc.left + 2, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(&hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.left, top: rc.top, right: rc.right, bottom: rc.top + 2 })?;
		htheme.DrawThemeBackground(&hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.right - 2, top: rc.top, right: rc.right, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(&hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.left, top: rc.bottom - 2, right: rc.right, bottom: rc.bottom })?;
	}

	Ok(())
}

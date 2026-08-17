use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;

/// Specifies the horizontal behavior of the control when the parent window is
/// resized.
///
/// The values are analog to [`gui::Vert`](crate::gui::Vert).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Horz {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at right.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control width will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

/// Specifies the vertical behavior of the control when the parent window is
/// resized.
///
/// The values are analog to [`gui::Horz`](crate::gui::Horz).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vert {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at bottom.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control height will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

struct ChildInfo {
	hchild: HWND,
	horz: Horz,
	vert: Vert,
	rc_orig: Option<RECT>, // original coordinates relative to parent, filled at 1st WM_SIZE
}

struct Obj { // actual fields of LayoutArranger
	ctrls: UnsafeCell<Vec<ChildInfo>>,
	sz_parent_orig: UnsafeCell<Option<SIZE>>, // original parent client area, filled at WM_CREATE/INITDIALOG
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Rearranges the stored controls according to their predefined rules.
#[derive(Clone)]
pub(in crate::gui) struct LayoutArranger(Pin<Arc<Obj>>);

impl LayoutArranger {
	pub(in crate::gui) fn new() -> Self {
		Self(
			Arc::pin(
				Obj {
					ctrls: UnsafeCell::new(Vec::default()),
					sz_parent_orig: UnsafeCell::new(None),
					_pin: PhantomPinned,
				},
			),
		)
	}

	/// Adds a new child control to the internal list, so this control will have
	/// its position and size rearranged when requested.
	pub(in crate::gui) fn add_child(&self,
		hparent: &HWND,
		hchild: &HWND,
		resize_behavior: (Horz, Vert),
	) -> SysResult<()>
	{
		if *hparent == HWND::NULL || *hchild == HWND::NULL {
			panic!("Cannot add resizer entries before window/control creation.");
		}

		let (horz, vert) = resize_behavior;
		if horz != Horz::None || vert != Vert::None { // if nothing to do, don't even add it
			unsafe { &mut *self.0.ctrls.get() }.push(
				ChildInfo {
					hchild: unsafe { hchild.raw_copy() },
					horz,
					vert,
					rc_orig: None,
				},
			);
		}

		Ok(())
	}

	/// Saves the original client area of the parent window.
	pub(in crate::gui) fn save_original_client_area(&self,
		hparent: &HWND,
	) -> SysResult<()>
	{
		let rc_parent = hparent.GetClientRect()?;
		*unsafe { &mut *self.0.sz_parent_orig.get() } = Some(
			SIZE::new(rc_parent.right, rc_parent.bottom),
		);
		Ok(())
	}

	/// Rearranges all child controls to fit the new width/height of parent
	/// window.
	pub(in crate::gui) fn rearrange(&self, p: &wm::Size) -> SysResult<()> {
		let ctrls = unsafe { &mut *self.0.ctrls.get() };
		if ctrls.is_empty() // no controls
			|| p.request == co::SIZE_R::MINIMIZED { // we're minimized
			return Ok(());
		}

		let sz_parent_orig = match unsafe { &mut *self.0.sz_parent_orig.get() } {
			Some(sz) => *sz,
			None => panic!("Original parent client area was not saved."),
		};

		let mut hdwp = HDWP::BeginDeferWindowPos(ctrls.len() as _)?;

		for ctrl in ctrls.iter_mut() {
			let mut uflags = co::SWP::NOZORDER;
			if ctrl.horz == Horz::Repos && ctrl.vert == Vert::Repos { // reposition both vert & horz
				uflags |= co::SWP::NOSIZE;
			} else if ctrl.horz == Horz::Resize && ctrl.vert == Vert::Resize { // resize both vert & horz
				uflags |= co::SWP::NOMOVE;
			}

			let rc_orig = match &ctrl.rc_orig {
				Some(rc) => *rc,
				None => {
					let mut rc = ctrl.hchild.GetWindowRect()?;
					ctrl.hchild.GetParent()?.ScreenToClientRc(&mut rc)?;
					ctrl.rc_orig = Some(rc); // save control client coordinates relative to parent
					rc
				},
			};

			hdwp.DeferWindowPos(
				&ctrl.hchild,
				HwndPlace::None,
				POINT::new(
					match ctrl.horz {
						Horz::Repos => p.client_area.cx - sz_parent_orig.cx + rc_orig.left,
						_ => rc_orig.left // keep original x pos
					},
					match ctrl.vert {
						Vert::Repos => p.client_area.cy - sz_parent_orig.cy + rc_orig.top,
						_ => rc_orig.top // keep original y pos
					},
				),
				SIZE::new(
					match ctrl.horz {
						Horz::Resize => p.client_area.cx - sz_parent_orig.cx + rc_orig.right - rc_orig.left,
						_ => rc_orig.right - rc_orig.left // keep original width
					},
					match ctrl.vert {
						Vert::Resize => p.client_area.cy - sz_parent_orig.cy + rc_orig.bottom - rc_orig.top,
						_ => rc_orig.bottom - rc_orig.top // keep original height
					},
				),
				uflags,
			)?;
		}

		Ok(())
	}
}

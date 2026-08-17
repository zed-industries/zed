use std::marker::PhantomPinned;
use std::ops::Index;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::prelude::*;

struct Obj { // actual fields of RadioGroup
	parent_ptr: NonNull<Base>,
	radios: Vec<RadioButton>,
	events: RadioGroupEvents,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A group of native [`RadioButton`](crate::gui::RadioButton) controls.
#[derive(Clone)]
pub struct RadioGroup(Pin<Arc<Obj>>);

unsafe impl Send for RadioGroup {}

impl Index<usize> for RadioGroup {
	type Output = RadioButton;

	fn index(&self, i: usize) -> &Self::Output {
		&self.0.radios[i]
	}
}

impl GuiNativeControlEvents<RadioGroupEvents> for RadioGroup {
	fn on(&self) -> &RadioGroupEvents {
		if *self.index(0).hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *unsafe { self.0.parent_ptr.as_ref() }.hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl RadioGroup {
	/// Instantiates a new `RadioGroup` object, each `RadioButton` to be created
	/// on the parent window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if `opts` is empty.
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `RadioGroup` in an event closure.
	#[must_use]
	pub fn new(
		parent: &impl GuiParent,
		opts: &[RadioButtonOpts],
	) -> Self
	{
		if opts.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let parent_base_ref = unsafe { Base::from_guiparent(parent) };

		let radios = opts.iter().enumerate()
			.map(|(i, opt)| {
				let mut radio_opt = opt.manual_clone();
				if i == 0 { // first radio?
					radio_opt.window_style |= co::WS::TABSTOP | co::WS::GROUP;
				}
				RadioButton::new(parent, radio_opt)
			})
			.collect::<Vec<_>>();

		let ctrl_ids = radios.iter()
			.map(|r| r.ctrl_id()) // when the radio is created, the ctrl ID is defined
			.collect::<Vec<_>>();

		let opts_resz_s = opts.into_iter()
			.map(|opt| OptsResz::Wnd(opt.clone()))
			.collect::<Vec<_>>();

		let new_self = Self(
			Arc::pin(
				Obj {
					parent_ptr: NonNull::from(parent_base_ref),
					radios,
					events: RadioGroupEvents::new(parent_base_ref, ctrl_ids),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm(parent_base_ref.wm_create_or_initdialog(), move |_| {
			self2.create(&opts_resz_s)?;
			Ok(None) // not meaningful
		});

		new_self
	}

	/// Instantiates a new `RadioGroup` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if `ctrls` is empty.
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `RadioGroup` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrls: &[(u16, Horz, Vert)],
	) -> Self
	{
		if ctrls.is_empty() {
			panic!("RadioGroup needs at least one RadioButton.");
		}

		let parent_base_ref = unsafe { Base::from_guiparent(parent) };

		let radios = ctrls.iter()
			.map(|(ctrl_id, _, _)| RadioButton::new_dlg(parent, *ctrl_id))
			.collect::<Vec<_>>();

		let ctrl_ids = ctrls.iter()
			.map(|(ctrl_id, _, _)| *ctrl_id)
			.collect::<Vec<_>>();

		let opts_resz_s = ctrls.iter()
			.map(|(_, horz, vert)| OptsResz::Dlg((*horz, *vert)))
			.collect::<Vec<_>>();

		let new_self = Self(
			Arc::pin(
				Obj {
					parent_ptr: NonNull::from(parent_base_ref),
					radios,
					events: RadioGroupEvents::new(parent_base_ref, ctrl_ids),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_base_ref.privileged_on().wm_init_dialog(move |_| {
			self2.create(&opts_resz_s)?;
			Ok(true) // not meaningful
		});

		new_self
	}

	fn create(&self,
		opts_resz_s: &Vec<OptsResz<RadioButtonOpts>>) -> SysResult<()>
	{
		self.0.radios.iter()
			.zip(opts_resz_s.iter())
			.try_for_each(|(radio, opts_resz)|
				radio.create(opts_resz) // create each RadioButton sequentially
			)
	}

	/// Returns an iterator over the internal
	/// [`RadioButton`](crate::gui::RadioButton) slice.
	///
	/// # Example
	///
	/// Changing the text of all radio buttons to `"One"`:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let radio_group: gui::RadioGroup; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let radio_group = gui::RadioGroup::new(&wnd, &[]);
	///
	/// for single_radio in radio_group.iter() {
	///     single_radio.hwnd().SetWindowText("One");
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	pub fn iter(&self) -> std::slice::Iter<'_, RadioButton> {
		self.0.radios.iter()
	}

	/// Returns the currently checked [`RadioButton`](crate::gui::RadioButton)
	/// of this group, if any.
	#[must_use]
	pub fn checked(&self) -> Option<&RadioButton> {
		self.checked_index().map(|idx| &self.0.radios[idx])
	}

	/// Returns the index of the currently selected
	/// [`RadioButton`](crate::gui::RadioButton) of this group, if any.
	#[must_use]
	pub fn checked_index(&self) -> Option<usize> {
		self.0.radios.iter()
			.position(|radio| radio.is_selected())
	}

	/// Returns the number of [`RadioButton`](crate::gui::RadioButton) controls
	/// in this group.
	#[must_use]
	pub fn count(&self) -> usize {
		self.0.radios.len()
	}
}

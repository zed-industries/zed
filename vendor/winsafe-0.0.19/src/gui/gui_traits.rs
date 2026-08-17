use std::any::Any;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*};
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// Any window. Exposes the underlying window handle.
pub trait GuiWindow {
	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is physically created, what usually happens right
	/// before
	/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
	/// or
	/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
	/// events.
	#[must_use]
	fn hwnd(&self) -> &HWND;

	/// Converts a reference to the [`Any`](std::any::Any) trait. This is useful
	/// when storing a collection of polymorphic controls, because `Any` allows
	/// downcasting.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::sync::Arc;
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let parent: gui::WindowMain; // initialized somewhere
	/// # let parent = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let ctrls: Vec<Arc<dyn GuiNativeControl>> = vec![
	///     Arc::new( gui::Edit::new(&parent, gui::EditOpts::default()) ),
	///     Arc::new( gui::Button::new(&parent, gui::ButtonOpts::default()) ),
	/// ];
	///
	/// let edit = ctrls[0].as_any() // retrieve 1st element, which is an Edit
	///     .downcast_ref::<gui::Edit>()
	///     .expect("This Edit downcast should never fail.");
	///
	/// edit.set_text("Foo");
	/// ```
	#[must_use]
	fn as_any(&self) -> &dyn Any;
}

/// Any window which can get/set text.
pub trait GuiWindowText: GuiWindow {
	/// Sets the text by calling
	/// [`HWND::SetWindowText`](crate::prelude::user_Hwnd::SetWindowText).
	fn set_text(&self, text: &str) {
		self.hwnd().SetWindowText(text).unwrap();
	}

	/// Retrieves the text by calling
	/// [`HWND::GetWindowText`](crate::prelude::user_Hwnd::GetWindowText).
	#[must_use]
	fn text(&self) -> String {
		self.hwnd().GetWindowText().unwrap()
	}
}

/// Any window which can host child controls.
pub trait GuiParent: GuiWindow {
	/// Exposes methods to handle the basic window messages, plus timer and
	/// native control notifications.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	#[must_use]
	fn on(&self) -> &WindowEventsAll;

	/// Returns a pointer to the inner base window structure, declared
	/// internally in the library.
	///
	/// # Safety
	///
	/// **Do not use this method** – it's for internal use of the library only.
	#[must_use]
	unsafe fn as_base(&self) -> *mut std::ffi::c_void;
}

/// Allows a window to spawn new threads which can return errors, and run
/// closures in the original UI thread.
pub trait GuiThread: GuiParent {
	/// This method calls [`std::thread::spawn`], but it allows the returning of
	/// an error value. This error value will be forwarded to the original UI
	/// thread, allowing it to be caught at
	/// [`WindowMain::run_main`](crate::gui::WindowMain::run_main).
	///
	/// It's a way to ensure that, upon an unexpected error, you application
	/// will be terminated gracefully.
	///
	/// # Examples
	///
	/// The example below shows the event of a
	/// [button click](crate::gui::events::ButtonEvents::bn_clicked) which
	/// spawns a new thread.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// let btn: gui::Button;
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let btn = gui::Button::new(&wnd, gui::ButtonOpts::default());
	///
	/// btn.on().bn_clicked({
	///     let wnd = wnd.clone();
	///     move || -> w::AnyResult<()> {
	///         println!("Click event at {:#x}", w::GetCurrentThreadId());
	///
	///         wnd.spawn_new_thread({
	///             let wnd = wnd.clone();
	///             move || {
	///                 println!("This is another thread: {:#x}", w::GetCurrentThreadId());
	///                 if 1 != 2 {
	///                     Err("Unexpected condition, goodbye.".into())
	///                 } else {
	///                     Ok(())
	///                 }
	///             }
	///         });
	///
	///         Ok(())
	///     }
	/// });
	/// ```
	fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static;

	/// Runs a closure synchronously in the window's original UI thread,
	/// allowing UI updates without the risk of a deadlock.
	///
	/// # Rationale
	///
	/// If you perform a very long task in the UI thread, the UI freezes until
	/// the task is complete – this may cause the impression that your
	/// application crashed. That's why long tasks should be performed in
	/// parallel threads. However, at some point you'll want to update the UI to
	/// reflect the task progress, but if you update the UI from another thread
	/// (different from the original UI thread), the UI may deadlock, and you
	/// application crashes.
	///
	/// This is what this `run_ui_thread` does, step-by-step:
	///
	/// 1. blocks current thread;
	/// 2. switches to the window's original UI thread;
	/// 3. runs the given `FnOnce`;
	/// 4. switches back to the first thread, which is then unblocked.
	///
	/// When working in a parallel thread, you **must** call `run_ui_thread` to
	/// update the UI.
	///
	/// # Examples
	///
	/// The example below shows the event of a
	/// [button click](crate::gui::events::ButtonEvents::bn_clicked) which
	/// starts a long task in a parallel thread. As it progresses, the status is
	/// printed at the windows's titlebar.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// let btn: gui::Button;
	/// # let btn = gui::Button::new(&wnd, gui::ButtonOpts::default());
	///
	/// btn.on().bn_clicked({
	///     let wnd = wnd.clone();
	///     move || -> w::AnyResult<()> {
	///         println!("Click event at {:#x}", w::GetCurrentThreadId());
	///
	///         std::thread::spawn({
	///             let wnd = wnd.clone();
	///             move || {
	///                 println!("Parallel task starts at {:#x}", w::GetCurrentThreadId());
	///                 w::Sleep(2000);
	///
	///                 wnd.run_ui_thread({
	///                     let wnd = wnd.clone();
	///                     move || -> w::AnyResult<()> {
	///                         println!("Updating UI at {:#x}", w::GetCurrentThreadId());
	///                         wnd.hwnd().SetWindowText("Status... 50%")?;
	///                         Ok(())
	///                     }
	///                 });
	///
	///                 println!("Parallel task keeps going at {:#x}", w::GetCurrentThreadId());
	///                 w::Sleep(2000);
	///
	///                 wnd.run_ui_thread({
	///                     let wnd = wnd.clone();
	///                     move || -> w::AnyResult<()> {
	///                         println!("Updating UI at {:#x}", w::GetCurrentThreadId());
	///                         wnd.hwnd().SetWindowText("Status... 100%")?;
	///                         Ok(())
	///                     }
	///                 });
	///             }
	///         });
	///
	///         Ok(())
	///     }
	/// });
	/// ```
	fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static;
}

/// Any child window.
pub trait GuiChild: GuiWindow {
	/// Returns the control ID, which is defined at control creation.
	///
	/// The control ID should be unique within a parent.
	#[must_use]
	fn ctrl_id(&self) -> u16;
}

/// Any child window which can be focused.
pub trait GuiChildFocus: GuiChild {
	/// In a raw, ordinary window, simply calls
	/// [`HWND:SetFocus`](crate::prelude::user_Hwnd::SetFocus).
	///
	/// In a dialog window, sends a
	/// [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl) message. This is
	/// preferable to the `HWND::SetFocus` because it takes care of border
	/// highlighting, like the native [`Button`](crate::gui::Button) control
	/// needs.
	fn focus(&self) {
		let hparent = self.hwnd().GetParent().unwrap();
		let atom = hparent.GetClassLongPtr(co::GCLP::ATOM) as u16;
		if atom == WC_DIALOG { // https://stackoverflow.com/a/64437627/6923555
			hparent.SendMessage(wm::NextDlgCtl {
				hwnd_focus: HwndFocus::Hwnd(unsafe { self.hwnd().raw_copy() }),
			});
		} else {
			self.hwnd().SetFocus();
		}
	}
}

/// Any native control, which can be subclassed.
pub trait GuiNativeControl: GuiChild {
	/// Exposes the subclass events. If at least one event exists, the control
	/// will be
	/// [subclassed](https://learn.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
	///
	/// **Note:** Subclassing may impact performance, use with care.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	#[must_use]
	fn on_subclass(&self) -> &WindowEvents;
}

/// Events of a native control.
pub trait GuiNativeControlEvents<E> {
	/// Exposes the specific control events.
	///
	/// # Panics
	///
	/// Panics if the control is already created. Events must be set before
	/// control creation.
	#[must_use]
	fn on(&self) -> &E;
}

/// Any window which can be rendered inside a native [`Tab`](crate::gui::Tab)
/// control.
pub trait GuiTab {
	/// Returns a reference to the [`WindowControl`](crate::gui::WindowControl)
	/// member.
	#[must_use]
	fn as_ctrl(&self) -> &WindowControl;
}

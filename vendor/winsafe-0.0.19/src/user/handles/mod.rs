mod haccel;
mod hcursor;
mod hdc;
mod hdesk;
mod hdwp;
mod hhook;
mod hicon;
mod hinstance;
mod hmenu;
mod hmonitor;
mod hprocess;
mod hwnd;

pub mod decl {
	pub use super::haccel::HACCEL;
	pub use super::hcursor::HCURSOR;
	pub use super::hdc::HDC;
	pub use super::hdesk::HDESK;
	pub use super::hdwp::HDWP;
	pub use super::hhook::HHOOK;
	pub use super::hicon::HICON;
	pub use super::hmenu::HMENU;
	pub use super::hmonitor::HMONITOR;
	pub use super::hwnd::HWND;

	impl_handle! { HBITMAP;
		/// Handle to a
		/// [bitmap](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
	}

	impl_handle! { HBRUSH;
		/// Handle to a
		/// [brush](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush).
	}

	impl_handle! { HRGN;
		/// Handle to a
		/// [region](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hrgn)
		/// GDI object.
	}
}

pub mod traits {
	pub use super::haccel::user_Haccel;
	pub use super::hcursor::user_Hcursor;
	pub use super::hdc::user_Hdc;
	pub use super::hdesk::user_Hdesk;
	pub use super::hdwp::user_Hdwp;
	pub use super::hhook::user_Hhook;
	pub use super::hicon::user_Hicon;
	pub use super::hinstance::user_Hinstance;
	pub use super::hmenu::user_Hmenu;
	pub use super::hmonitor::user_Hmonitor;
	pub use super::hprocess::user_Hprocess;
	pub use super::hwnd::user_Hwnd;
}

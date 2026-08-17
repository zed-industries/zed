#![doc = include_str!("lib.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Declarations of macros used throughout the library.
// No macros are public.

#[macro_use] mod macros;

// Declarations of modules themselves.

#[cfg(feature = "comctl")] mod comctl;
#[cfg(feature = "dshow")] mod dshow;
#[cfg(feature = "dwm")] mod dwm;
#[cfg(feature = "dxgi")] mod dxgi;
#[cfg(feature = "gdi")] mod gdi;
#[cfg(feature = "kernel")] mod kernel;
#[cfg(feature = "mf")] mod mf;
#[cfg(feature = "ole")] mod ole;
#[cfg(feature = "oleaut")] mod oleaut;
#[cfg(feature = "shell")] mod shell;
#[cfg(feature = "taskschd")] mod taskschd;
#[cfg(feature = "user")] mod user;
#[cfg(feature = "uxtheme")] mod uxtheme;
#[cfg(feature = "version")] mod version;
#[cfg(all(feature = "comctl", feature = "gdi"))] mod comctl_gdi;
#[cfg(all(feature = "comctl", feature = "shell"))] mod comctl_shell;
#[cfg(all(feature = "gdi", feature = "mf"))] mod gdi_mf;

// The gui module itself is public.

#[cfg(feature = "gui")] pub mod gui;

// Declarations inside decl are public, placed at the root of the crate.

mod decl {
	#[cfg(feature = "comctl")] pub use super::comctl::decl::*;
	#[cfg(feature = "dshow")] pub use super::dshow::decl::*;
	#[cfg(feature = "dwm")] pub use super::dwm::decl::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::decl::*;
	#[cfg(feature = "gdi")] pub use super::gdi::decl::*;
	#[cfg(feature = "kernel")] pub use super::kernel::decl::*;
	#[cfg(feature = "mf")] pub use super::mf::decl::*;
	#[cfg(feature = "ole")] pub use super::ole::decl::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::decl::*;
	#[cfg(feature = "shell")] pub use super::shell::decl::*;
	#[cfg(feature = "taskschd")] pub use super::taskschd::decl::*;
	#[cfg(feature = "user")] pub use super::user::decl::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::decl::*;
	#[cfg(feature = "version")] pub use super::version::decl::*;
	#[cfg(all(feature = "comctl", feature = "gdi"))] pub use super::comctl_gdi::decl::*;
}
pub use decl::*;

#[cfg(feature = "kernel")]
pub mod co {
	//! Native constants.
	//!
	//! All types can be converted from/to their underlying integer type. They
	//! all implement the [`NativeConst`](crate::prelude::NativeConst) trait;
	//! those who can be combined as bitflags also implement
	//! [`NativeBitflag`](crate::prelude::NativeBitflag).
	//!
	//! Among these constant types, three are error types:
	//! [`CDERR`], [`ERROR`] and [`HRESULT`].

	#[cfg(feature = "comctl")] pub use super::comctl::co::*;
	#[cfg(feature = "dshow")] pub use super::dshow::co::*;
	#[cfg(feature = "dwm")] pub use super::dwm::co::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::co::*;
	#[cfg(feature = "gdi")] pub use super::gdi::co::*;
	#[cfg(feature = "kernel")] pub use super::kernel::co::*;
	#[cfg(feature = "mf")] pub use super::mf::co::*;
	#[cfg(feature = "ole")] pub use super::ole::co::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::co::*;
	#[cfg(feature = "shell")] pub use super::shell::co::*;
	#[cfg(feature = "taskschd")] pub use super::taskschd::co::*;
	#[cfg(feature = "user")] pub use super::user::co::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::co::*;
	#[cfg(feature = "version")] pub use super::version::co::*;
}

#[cfg(feature = "kernel")]
pub mod guard {
	//! RAII implementation for various resources, which automatically perform
	//! cleanup routines when the object goes out of scope.
	//!
	//! The guards are named after the functions they call.

	#[cfg(feature = "comctl")] pub use super::comctl::guard::*;
	#[cfg(feature = "gdi")] pub use super::gdi::guard::*;
	#[cfg(feature = "kernel")] pub use super::kernel::guard::*;
	#[cfg(feature = "ole")] pub use super::ole::guard::*;
	#[cfg(feature = "shell")] pub use super::shell::guard::*;
	#[cfg(feature = "user")] pub use super::user::guard::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::guard::*;
	#[cfg(feature = "version")] pub use super::version::guard::*;
}

#[cfg(feature = "user")]
pub mod msg {
	#![doc = include_str!("msg.md")]

	pub use super::user::messages::WndMsg;

	#[cfg(feature = "user")]
	pub mod bm {
		//! Button control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-messages),
		//! whose constants have [`BM`](crate::co::BM) and
		//! [`BCM`](crate::co::BCM) prefixes.

		pub use super::super::user::messages::bm::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::bcm::*;
	}

	#[cfg(feature = "user")]
	pub mod cb {
		//! Combo box control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-messages),
		//! whose constants have [`CB`](crate::co::CB) prefix.

		pub use super::super::user::messages::cb::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::cb::*;
	}

	#[cfg(feature = "comctl")]
	pub mod dtm {
		//! Date and time picker control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-messages),
		//! whose constants have [`DTM`](crate::co::DTM) prefix.

		pub use super::super::comctl::messages::dtm::*;
		#[cfg(feature = "gdi")] pub use super::super::comctl_gdi::messages::dtm::*;
	}

	#[cfg(feature = "user")]
	pub mod em {
		//! Edit control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages),
		//! whose constants have [`EM`](crate::co::EM) prefix.

		pub use super::super::user::messages::em::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::em::*;
	}

	#[cfg(feature = "comctl")]
	pub mod hdm {
		//! Header control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages),
		//! whose constants have [`HDM`](crate::co::HDM) prefix.

		pub use super::super::comctl::messages::hdm::*;
	}

	#[cfg(feature = "user")]
	pub mod lb {
		//! ListBox control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-messages),
		//! whose constants have [`LB`](crate::co::LB) prefix.

		pub use super::super::user::messages::lb::*;
	}

	#[cfg(feature = "comctl")]
	pub mod lvm {
		//! List view control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-messages),
		//! whose constants have [`LVM`](crate::co::LVM) prefix.

		pub use super::super::comctl::messages::lvm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod mcm {
		//! Month calendar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-messages),
		//! whose constants have [`MCM`](crate::co::MCM) prefix.

		pub use super::super::comctl::messages::mcm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod pbm {
		//! Progress bar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-progress-bar-control-reference-messages),
		//! whose constants have [`PBM`](crate::co::PBM) prefix.

		pub use super::super::comctl::messages::pbm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod sb {
		//! Status bar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-messages),
		//! whose constants have [`SB`](crate::co::SB) prefix.

		pub use super::super::comctl::messages::sb::*;
	}

	#[cfg(feature = "comctl")]
	pub mod stm {
		//! Static control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-messages),
		//! whose constants have [`STM`](crate::co::STM) prefix.

		pub use super::super::comctl::messages::stm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod tbm {
		//! Toolbar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-messages),
		//! whose constants have [`TBM`](crate::co::TBM) prefix.

		pub use super::super::comctl::messages::tbm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod tcm {
		//! Tab control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-messages),
		//! whose constants have [`TCM`](crate::co::TCM) prefix.

		pub use super::super::comctl::messages::tcm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod trbm {
		//! Trackbar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-messages),
		//! whose constants have [`TRBM`](crate::co::TRBM) prefix.
		//!
		//! Originally has `TBM` prefix.

		pub use super::super::comctl::messages::trbm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod tvm {
		//! Tree view control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-messages),
		//! whose constants have [`TVM`](crate::co::TVM) prefix.

		pub use super::super::comctl::messages::tvm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod udm {
		//! UpDown control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-messages),
		//! whose constants have [`UDM`](crate::co::UDM) prefix.

		pub use super::super::comctl::messages::udm::*;
	}

	#[cfg(feature = "user")]
	pub mod wm {
		//! Generic window
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
		//! whose constants have [`WM`](crate::co::WM) prefix.

		pub use super::super::user::messages::wm::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::wm::*;
		#[cfg(feature = "gdi")] pub use super::super::gdi::messages::wm::*;
		#[cfg(feature = "shell")] pub use super::super::shell::messages::wm::*;
	}
}

#[cfg(feature = "kernel")]
pub mod prelude {
	//! The WinSafe prelude.
	//!
	//! The purpose of this module is to alleviate imports of many common
	//! traits. To use it, add a glob import to the top of all your modules that
	//! use the library:
	//!
	//! ```rust,no_run
	//! use winsafe::prelude::*;
	//! ```

	#[cfg(feature = "comctl")] pub use super::comctl::traits::*;
	#[cfg(feature = "dshow")] pub use super::dshow::traits::*;
	#[cfg(feature = "dwm")] pub use super::dwm::traits::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::traits::*;
	#[cfg(feature = "gdi")] pub use super::gdi::traits::*;
	#[cfg(feature = "gui")] pub use super::gui::traits::*;
	#[cfg(feature = "kernel")] pub use super::kernel::traits::*;
	#[cfg(feature = "mf")] pub use super::mf::traits::*;
	#[cfg(feature = "ole")] pub use super::ole::traits::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::traits::*;
	#[cfg(feature = "shell")] pub use super::shell::traits::*;
	#[cfg(feature = "taskschd")] pub use super::taskschd::traits::*;
	#[cfg(feature = "user")] pub use super::user::traits::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::traits::*;
	#[cfg(feature = "version")] pub use super::version::traits::*;
	#[cfg(all(feature = "comctl", feature = "shell"))] pub use super::comctl_shell::traits::*;
	#[cfg(all(feature = "gdi", feature = "mf"))] pub use super::gdi_mf::traits::*;
}

#[cfg(feature = "ole")]
pub mod vt {
	//! Virtual tables of COM interfaces.

	#[cfg(feature = "dshow")] pub use super::dshow::vt::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::vt::*;
	#[cfg(feature = "mf")] pub use super::mf::vt::*;
	#[cfg(feature = "ole")] pub use super::ole::vt::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::vt::*;
	#[cfg(feature = "shell")] pub use super::shell::vt::*;
	#[cfg(feature = "taskschd")] pub use super::taskschd::vt::*;
}

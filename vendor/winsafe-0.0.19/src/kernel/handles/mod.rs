mod haccesstoken;
mod handle_traits;
mod hevent;
mod heventlog;
mod hfile;
mod hfilemap;
mod hfilemapview;
mod hfindfile;
mod hglobal;
mod hheap;
mod hinstance;
mod hkey;
mod hlocal;
mod hpipe;
mod hprocess;
mod hprocesslist;
mod hsc;
mod hservice;
mod hservicestatus;
mod hstd;
mod hthread;
mod htransaction;
mod hupdatesrc;

pub mod decl {
	pub use super::haccesstoken::HACCESSTOKEN;
	pub use super::hevent::HEVENT;
	pub use super::heventlog::HEVENTLOG;
	pub use super::hfile::HFILE;
	pub use super::hfilemap::HFILEMAP;
	pub use super::hfilemapview::HFILEMAPVIEW;
	pub use super::hfindfile::HFINDFILE;
	pub use super::hglobal::HGLOBAL;
	pub use super::hheap::HHEAP;
	pub use super::hinstance::HINSTANCE;
	pub use super::hkey::HKEY;
	pub use super::hlocal::HLOCAL;
	pub use super::hpipe::HPIPE;
	pub use super::hprocess::HPROCESS;
	pub use super::hprocesslist::HPROCESSLIST;
	pub use super::hsc::HSC;
	pub use super::hservice::HSERVICE;
	pub use super::hservicestatus::HSERVICESTATUS;
	pub use super::hstd::HSTD;
	pub use super::hthread::HTHREAD;
	pub use super::htransaction::HTRANSACTION;
	pub use super::hupdatesrc::HUPDATERSRC;

	impl_handle! { HRSRC;
		/// Handle to a
		/// [resource](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew).
		/// Originally just a `HANDLE`.
		///
		/// For an example, see
		/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	}

	impl_handle! { HRSRCMEM;
		/// Handle to a resource
		/// [memory block](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource).
		/// Originally just an `HGLOBAL`.
		///
		/// For an example, see
		/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	}
}

pub mod traits {
	pub use super::haccesstoken::kernel_Haccesstoken;
	pub use super::handle_traits::*;
	pub use super::hevent::kernel_Hevent;
	pub use super::heventlog::kernel_Heventlog;
	pub use super::hfile::kernel_Hfile;
	pub use super::hfilemap::kernel_Hfilemap;
	pub use super::hfilemapview::kernel_Hfilemapview;
	pub use super::hfindfile::kernel_Hfindfile;
	pub use super::hglobal::kernel_Hglobal;
	pub use super::hheap::kernel_Hheap;
	pub use super::hinstance::kernel_Hinstance;
	pub use super::hkey::kernel_Hkey;
	pub use super::hlocal::kernel_Hlocal;
	pub use super::hpipe::kernel_Hpipe;
	pub use super::hprocess::kernel_Hprocess;
	pub use super::hprocesslist::kernel_Hprocesslist;
	pub use super::hsc::kernel_Hsc;
	pub use super::hservice::kernel_Hservice;
	pub use super::hservicestatus::kernel_Hservicestatus;
	pub use super::hstd::kernel_Hstd;
	pub use super::hthread::kernel_Hthread;
	pub use super::htransaction::kernel_Htransaction;
	pub use super::hupdatesrc::kernel_Hupdatersrc;
}

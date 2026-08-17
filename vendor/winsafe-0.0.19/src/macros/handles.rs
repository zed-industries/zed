#![allow(unused_macros)]

/// Declares a handle.
macro_rules! impl_handle {
	(
		$name:ident;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		#[derive(PartialEq, Eq, Hash)]
		pub struct $name(*mut std::ffi::c_void);

		unsafe impl Send for $name {}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{:#010x}", self.0 as usize)
			}
		}
		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "[{:#010x} {}] {}",
					self.0 as usize, self.0 as usize, stringify!($name))
			}
		}

		impl std::fmt::LowerHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&(self.0 as usize), f)
			}
		}
		impl std::fmt::UpperHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&(self.0 as usize), f)
			}
		}

		impl crate::prelude::Handle for $name {
			const NULL: Self = Self(std::ptr::null_mut());
			const INVALID: Self = Self(-1 as _);

			unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self {
				Self(p)
			}

			unsafe fn as_mut(&mut self) -> &mut *mut std::ffi::c_void {
				&mut self.0
			}

			unsafe fn raw_copy(&self) -> Self {
				Self(self.0)
			}

			fn ptr(&self) -> *mut std::ffi::c_void {
				self.0
			}
		}
	};
}

/// Declares a handle guard which has a simple cleaner function.
macro_rules! handle_guard {
	(
		$name:ident : $handle:ty;
		$cleaner:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub struct $name {
			handle: $handle,
		}

		impl Drop for $name {
			fn drop(&mut self) {
				if let Some(h) = self.handle.as_opt() {
					unsafe { $cleaner(h.ptr()); } // ignore errors
				}
			}
		}

		impl std::ops::Deref for $name {
			type Target = $handle;

			fn deref(&self) -> &Self::Target {
				&self.handle
			}
		}

		impl std::ops::DerefMut for $name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.handle
			}
		}

		impl $name {
			/// Constructs the guard by taking ownership of the handle.
			///
			/// This method can be used as an escape hatch to interoperate with
			/// other libraries.
			///
			/// # Safety
			///
			/// Be sure the handle must be freed with the specified function at
			/// the end of scope.
			#[must_use]
			pub const unsafe fn new(handle: $handle) -> Self {
				Self { handle }
			}

			/// Ejects the underlying handle, leaving a
			/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its
			/// place.
			///
			/// Since the internal handle will be invalidated, the destructor
			/// will not run. It's your responsability to run it, otherwise
			/// you'll cause a resource leak.
			#[must_use]
			pub fn leak(&mut self) -> $handle {
				std::mem::replace(&mut self.handle, Handle::INVALID)
			}
		}
	};
}

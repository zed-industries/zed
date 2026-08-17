#![allow(unused_macros)]

/// Implements `Default` trait by zeroing all fields.
macro_rules! impl_default {
	($name:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Default for $name<$($life),*> {
			fn default() -> Self {
				unsafe { std::mem::zeroed::<Self>() }
			}
		}
	};
}

/// Implements `Default` trait by zeroing all fields. Also sets the size field
/// to struct size.
macro_rules! impl_default_with_size {
	($name:ident, $field:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Default for $name<$($life),*> {
			fn default() -> Self {
				let mut obj = unsafe { std::mem::zeroed::<Self>() };
				obj.$field = std::mem::size_of::<Self>() as _;
				obj
			}
		}
	};
}

/// Implements `IntUnderlying`, and other needed traits.
macro_rules! impl_intunderlying {
	($name:ident, $ntype:ty) => {
		unsafe impl Send for $name {}

		impl From<$name> for $ntype {
			fn from(v: $name) -> Self {
				v.0
			}
		}

		impl AsRef<$ntype> for $name {
			fn as_ref(&self) -> &$ntype {
				&self.0
			}
		}

		impl crate::prelude::IntUnderlying for $name {
			type Raw = $ntype;

			unsafe fn as_mut(&mut self) -> &mut Self::Raw {
				&mut self.0
			}
		}

		impl $name {
			/// Constructs a new object by wrapping the given integer value.
			///
			/// # Safety
			///
			/// Be sure the given value is meaningful for the actual type.
			#[must_use]
			pub const unsafe fn from_raw(v: $ntype) -> Self {
				Self(v)
			}

			/// Returns the primitive integer underlying value.
			///
			/// This method is similar to [`Into`](std::convert::Into), but it
			/// is `const`, therefore it can be used in
			/// [const contexts](https://doc.rust-lang.org/reference/const_eval.html).
			#[must_use]
			pub const fn raw(&self) -> $ntype {
				self.0
			}
		}
	};
}

/// Implements getter and setter methods for the given `BOOL` field.
macro_rules! pub_fn_bool_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the bool field.
		#[must_use]
		pub const fn $field(&self) -> bool {
			self.$field != 0
		}

		/// Sets the bool field.
		pub fn $setter(&mut self, val: bool) {
			self.$field = val as _
		}
	};
}

/// Implements getter and setter methods for the given resource ID field, stored
/// as `*mut u16`.
macro_rules! pub_fn_resource_id_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the resource ID field.
		#[must_use]
		pub fn $field(&self) -> u16 {
			self.$field as _
		}

		/// Sets the resource ID field.
		pub fn $setter(&mut self, val: u16) {
			self.$field = val as _;
		}
	};
}

/// Implements getter and setter methods for the given `*mut u16` field.
macro_rules! pub_fn_string_ptr_get_set {
	($life:lifetime, $field:ident, $setter:ident) => {
		/// Returns the string field, if any.
		#[must_use]
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }.map(|psz| {
				unsafe { WString::from_wchars_nullt(psz) }.to_string()
			})
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut WString>) {
			self.$field = buf.map_or(
				std::ptr::null_mut(),
				|buf| unsafe { buf.as_mut_ptr() },
			);
		}
	};
}

/// Implements getter and setter methods for the given `*mut u16` and `u32`
/// fields, setting pointer and its actual chars length without terminating
/// null.
macro_rules! pub_fn_string_ptrlen_get_set {
	($life:lifetime, $field:ident, $setter:ident, $length:ident) => {
		/// Returns the string field, if any.
		#[must_use]
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }.map(|psz| {
				WString::from_wchars_count(psz, self.$length as _).to_string()
			})
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut WString>) {
			self.$length = buf.as_ref().map_or(0, |buf| buf.str_len() as _);
			self.$field = buf.map_or(
				std::ptr::null_mut(),
				|buf| unsafe { buf.as_mut_ptr() },
			);
		}
	};
}

/// Implements getter and setter methods for the given `[u16; N]` field.
macro_rules! pub_fn_string_arr_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the string field.
		#[must_use]
		pub fn $field(&self) -> String {
			crate::kernel::decl::WString::from_wchars_slice(&self.$field).to_string()
		}

		/// Sets the string field.
		pub fn $setter(&mut self, text: &str) {
			crate::kernel::decl::WString::from_str(text).copy_to_slice(&mut self.$field);
		}
	};
}

/// Implements getter and setter methods for the given `*mut u16` and `i32`
/// fields, setting buffer and its size.
macro_rules! pub_fn_string_buf_get_set {
	($life:lifetime, $field:ident, $setter:ident, $raw:ident, $cch:ident) => {
		/// Returns the raw pointer to the string field, and its declared size.
		///
		/// This method can be used as an escape hatch to interoperate with
		/// other libraries.
		#[must_use]
		pub const fn $raw(&self) -> (*mut u16, i32) {
			(self.$field, self.$cch as _) // some u32 and usize exist
		}

		/// Returns the string field.
		#[must_use]
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }.map(|psz| {
				unsafe { WString::from_wchars_nullt(psz) }.to_string()
			})
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut WString>) {
			self.$cch = buf.as_ref().map_or(0, |buf| buf.buf_len() as _);
			self.$field = buf.map_or(std::ptr::null_mut(), |buf| {
				if buf.is_allocated() {
					unsafe { buf.as_mut_ptr() }
				} else {
					std::ptr::null_mut()
				}
			});
		}
	};
}

/// Implements getter and setter methods for the given pointer field.
macro_rules! pub_fn_ptr_get_set {
	($life:lifetime, $field:ident, $setter:ident, $ty:ty) => {
		/// Returns the pointer field.
		#[must_use]
		pub fn $field(&self) -> Option<&$life mut $ty> {
			unsafe { self.$field.as_mut() }
		}

		/// Sets the pointer field.
		pub fn $setter(&mut self, obj: Option<&$life mut $ty>) {
			self.$field = obj.map_or(std::ptr::null_mut(), |obj| obj);
		}
	};
}

/// Implements getter and setter methods for the given array + size fields,
/// setting buffer and its size.
macro_rules! pub_fn_array_buf_get_set {
	($life:lifetime, $field:ident, $setter:ident, $cch:ident, $ty:ty) => {
		/// Returns the array field.
		#[must_use]
		pub fn $field(&self) -> Option<&$life mut [$ty]> {
			unsafe {
				self.$field.as_mut().map(|p| {
					std::slice::from_raw_parts_mut(p, self.$cch as _)
				})
			}
		}

		/// Sets the array field.
		pub fn $setter(&mut self, buf: Option<&$life mut [$ty]>) {
			match buf {
				Some(buf) => {
					self.$field = buf as *mut _ as _;
					self.$cch = buf.len() as _;
				},
				None => {
					self.$field = std::ptr::null_mut();
					self.$cch = 0;
				},
			}
		}
	};
}

/// Implements getter and setter methods for the given `ComPtr` field.
macro_rules! pub_fn_comptr_get_set {
	($field:ident, $setter:ident, $trait:ident) => {
		/// Returns the COM object field, by cloning the underlying COM pointer.
		#[must_use]
		pub fn $field<T>(&self) -> Option<T>
			where T: $trait,
		{
			if self.$field.is_null() {
				None
			} else {
				let obj = std::mem::ManuallyDrop::new( // won't release the stored pointer
					unsafe { T::from_ptr(self.$field) },
				);
				let cloned = T::clone(&obj); // the cloned will release the stored pointer
				Some(cloned)
			}
		}

		/// Sets the COM object field, by cloning the underlying COM pointer.
		pub fn $setter<T>(&mut self, obj: Option<&T>)
			where T: $trait,
		{
			let _ = unsafe { T::from_ptr(self.$field) }; // if already set, call Release() immediately
			self.$field = match obj {
				Some(obj) => {
					let mut cloned = T::clone(obj);
					cloned.leak() // so the cloned pointer won't be released here
				},
				None => std::ptr::null_mut(),
			};
		}
	};
}

/// Implements `Drop` for a `ComPtr` field.
macro_rules! impl_drop_comptr {
	($field:ident, $name:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Drop for $name<$($life),*> {
			fn drop(&mut self) {
				if !self.$field.is_null() {
					let _ = unsafe {
						<crate::IUnknown as crate::prelude::ole_IUnknown>
							::from_ptr(self.$field) // if pointer is present, call Release() on it
					};
				}
			}
		}
	};
}

/// Implements accessor methods over `pmem` and `sz` fields of an allocated
/// memory block.
macro_rules! pub_fn_mem_block {
	() => {
		/// Returns a pointer to the allocated memory block, or null if not
		/// allocated.
		#[must_use]
		pub const fn as_ptr(&self) -> *const std::ffi::c_void {
			self.pmem
		}

		/// Returns a mutable pointer to the allocated memory block, or null if
		/// not allocated.
		#[must_use]
		pub fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
			self.pmem
		}

		/// Returns a slice over the allocated memory block.
		#[must_use]
		pub const fn as_slice(&self) -> &[u8] {
			unsafe { std::slice::from_raw_parts(self.pmem as _, self.sz) }
		}

		/// Returns a mutable slice over the allocated memory block.
		#[must_use]
		pub fn as_mut_slice(&mut self) -> &mut [u8] {
			unsafe { std::slice::from_raw_parts_mut(self.pmem as _, self.sz) }
		}

		/// Returns a slice over the allocated memory block, aligned to the given
		/// type.
		///
		/// # Safety
		///
		/// Make sure the alignment is correct.
		#[must_use]
		pub const unsafe fn as_slice_aligned<T>(&self) -> &[T] {
			std::slice::from_raw_parts(
				self.pmem as _,
				self.sz / std::mem::size_of::<T>(),
			)
		}

		/// Returns a mutable slice over the allocated memory block, aligned to the
		/// given type.
		///
		/// # Safety
		///
		/// Make sure the alignment is correct.
		#[must_use]
		pub unsafe fn as_mut_slice_aligned<T>(&mut self) -> &mut [T] {
			std::slice::from_raw_parts_mut(
				self.pmem as _,
				self.sz / std::mem::size_of::<T>(),
			)
		}

		/// Returns the size of the allocated memory block.
		#[must_use]
		pub const fn len(&self) -> usize {
			self.sz
		}
	};
}

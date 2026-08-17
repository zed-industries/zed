use super::*;

/// A registry key.
#[repr(transparent)]
#[derive(Debug)]
pub struct Key(pub(crate) HKEY);

impl Key {
    /// Creates a registry key. If the key already exists, the function opens it.
    pub fn create<T: AsRef<str>>(&self, path: T) -> Result<Self> {
        self.options().read().write().create().open(path)
    }

    /// Opens a registry key.
    pub fn open<T: AsRef<str>>(&self, path: T) -> Result<Self> {
        self.options().read().open(path)
    }

    /// Creates an `OpenOptions` object for the registry key.
    pub fn options(&self) -> OpenOptions<'_> {
        OpenOptions::new(self)
    }

    /// Constructs a registry key from an existing handle.
    ///
    /// # Safety
    ///
    /// This function takes ownership of the handle.
    /// The handle must be owned by the caller and safe to free with `RegCloseKey`.
    pub unsafe fn from_raw(handle: *mut core::ffi::c_void) -> Self {
        Self(handle)
    }

    /// Returns the underlying registry key handle.
    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        self.0
    }

    /// Changes the name of the specified registry key.
    pub fn rename<F: AsRef<str>, T: AsRef<str>>(&self, from: F, to: T) -> Result<()> {
        let result = unsafe { RegRenameKey(self.0, pcwstr(from).as_ptr(), pcwstr(to).as_ptr()) };
        win32_error(result)
    }

    /// Removes the registry keys and values of the specified key recursively.
    pub fn remove_tree<T: AsRef<str>>(&self, path: T) -> Result<()> {
        let result = unsafe { RegDeleteTreeW(self.0, pcwstr(path).as_ptr()) };
        win32_error(result)
    }

    /// Removes the registry value.
    pub fn remove_value<T: AsRef<str>>(&self, name: T) -> Result<()> {
        let result = unsafe { RegDeleteValueW(self.0, pcwstr(name).as_ptr()) };
        win32_error(result)
    }

    /// Creates an iterator of registry key names.
    pub fn keys(&self) -> Result<KeyIterator<'_>> {
        KeyIterator::new(self)
    }

    /// Creates an iterator of registry values.
    pub fn values(&self) -> Result<ValueIterator<'_>> {
        ValueIterator::new(self)
    }

    /// Sets the name and value in the registry key.
    pub fn set_u32<T: AsRef<str>>(&self, name: T, value: u32) -> Result<()> {
        self.set_bytes(name, Type::U32, &value.to_le_bytes())
    }

    /// Sets the name and value in the registry key.
    pub fn set_u64<T: AsRef<str>>(&self, name: T, value: u64) -> Result<()> {
        self.set_bytes(name, Type::U64, &value.to_le_bytes())
    }

    /// Sets the name and value in the registry key.
    pub fn set_string<T: AsRef<str>, U: AsRef<str>>(&self, name: T, value: U) -> Result<()> {
        self.set_bytes(name, Type::String, pcwstr(value).as_bytes())
    }

    /// Sets the name and value in the registry key.
    pub fn set_hstring<T: AsRef<str>>(
        &self,
        name: T,
        value: &windows_strings::HSTRING,
    ) -> Result<()> {
        self.set_bytes(name, Type::String, as_bytes(value))
    }

    /// Sets the name and value in the registry key.
    pub fn set_expand_string<T: AsRef<str>, U: AsRef<str>>(&self, name: T, value: U) -> Result<()> {
        self.set_bytes(name, Type::ExpandString, pcwstr(value).as_bytes())
    }

    /// Sets the name and value in the registry key.
    pub fn set_expand_hstring<T: AsRef<str>>(
        &self,
        name: T,
        value: &windows_strings::HSTRING,
    ) -> Result<()> {
        self.set_bytes(name, Type::ExpandString, as_bytes(value))
    }

    /// Sets the name and value in the registry key.
    pub fn set_multi_string<T: AsRef<str>>(&self, name: T, value: &[T]) -> Result<()> {
        let value = multi_pcwstr(value);
        self.set_bytes(name, Type::MultiString, value.as_bytes())
    }

    /// Sets the name and value in the registry key.
    pub fn set_value<T: AsRef<str>>(&self, name: T, value: &Value) -> Result<()> {
        self.set_bytes(name, value.ty(), value)
    }

    /// Sets the name and value in the registry key.
    pub fn set_bytes<T: AsRef<str>>(&self, name: T, ty: Type, value: &[u8]) -> Result<()> {
        unsafe { self.raw_set_bytes(pcwstr(name).as_raw(), ty, value) }
    }

    /// Gets the type for the name in the registry key.
    pub fn get_type<T: AsRef<str>>(&self, name: T) -> Result<Type> {
        let (ty, _) = unsafe { self.raw_get_info(pcwstr(name).as_raw())? };
        Ok(ty)
    }

    /// Gets the value for the name in the registry key.
    pub fn get_value<T: AsRef<str>>(&self, name: T) -> Result<Value> {
        let name = pcwstr(name);
        let (ty, len) = unsafe { self.raw_get_info(name.as_raw())? };
        let mut data = Data::new(len);
        unsafe { self.raw_get_bytes(name.as_raw(), &mut data)? };
        Ok(Value { data, ty })
    }

    /// Gets the value for the name in the registry key.
    pub fn get_u32<T: AsRef<str>>(&self, name: T) -> Result<u32> {
        Ok(self.get_u64(name)?.try_into()?)
    }

    /// Gets the value for the name in the registry key.
    pub fn get_u64<T: AsRef<str>>(&self, name: T) -> Result<u64> {
        let value = &mut [0; 8];
        let (ty, value) = unsafe { self.raw_get_bytes(pcwstr(name).as_raw(), value)? };
        from_le_bytes(ty, value)
    }

    /// Gets the value for the name in the registry key.
    pub fn get_string<T: AsRef<str>>(&self, name: T) -> Result<String> {
        self.get_value(name)?.try_into()
    }

    /// Gets the value for the name in the registry key.
    pub fn get_hstring<T: AsRef<str>>(&self, name: T) -> Result<HSTRING> {
        let name = pcwstr(name);
        let (ty, len) = unsafe { self.raw_get_info(name.as_raw())? };

        if !matches!(ty, Type::String | Type::ExpandString) {
            return Err(invalid_data());
        }

        let mut value = HStringBuilder::new(len / 2);
        unsafe { self.raw_get_bytes(name.as_raw(), value.as_bytes_mut())? };
        value.trim_end();
        Ok(value.into())
    }

    /// Gets the value for the name in the registry key.
    pub fn get_multi_string<T: AsRef<str>>(&self, name: T) -> Result<Vec<String>> {
        self.get_value(name)?.try_into()
    }

    /// Sets the name and value in the registry key.
    ///
    /// This method avoids any allocations.
    ///
    /// # Safety
    ///
    /// The `PCWSTR` pointer needs to be valid for reads up until and including the next `\0`.
    #[track_caller]
    pub unsafe fn raw_set_bytes<N: AsRef<PCWSTR>>(
        &self,
        name: N,
        ty: Type,
        value: &[u8],
    ) -> Result<()> {
        if cfg!(debug_assertions) {
            // RegSetValueExW expects string data to be null terminated.
            if matches!(ty, Type::String | Type::ExpandString | Type::MultiString) {
                debug_assert!(
                    value.get(value.len() - 2) == Some(&0),
                    "`value` isn't null-terminated"
                );
                debug_assert!(value.last() == Some(&0), "`value` isn't null-terminated");
            }
        }

        let result = unsafe {
            RegSetValueExW(
                self.0,
                name.as_ref().as_ptr(),
                0,
                ty.into(),
                value.as_ptr(),
                value.len().try_into()?,
            )
        };

        win32_error(result)
    }

    /// Gets the type and length for the name in the registry key.
    ///
    /// This method avoids any allocations.
    ///
    /// # Safety
    ///
    /// The `PCWSTR` pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn raw_get_info<N: AsRef<PCWSTR>>(&self, name: N) -> Result<(Type, usize)> {
        let mut ty = 0;
        let mut len = 0;

        let result = unsafe {
            RegQueryValueExW(
                self.0,
                name.as_ref().as_ptr(),
                null(),
                &mut ty,
                core::ptr::null_mut(),
                &mut len,
            )
        };

        win32_error(result)?;
        Ok((ty.into(), len as usize))
    }

    /// Gets the value for the name in the registry key.
    ///
    /// This method avoids any allocations.
    ///
    /// # Safety
    ///
    /// The `PCWSTR` pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn raw_get_bytes<'a, N: AsRef<PCWSTR>>(
        &self,
        name: N,
        value: &'a mut [u8],
    ) -> Result<(Type, &'a [u8])> {
        let mut ty = 0;
        let mut len = value.len().try_into()?;

        let result = unsafe {
            RegQueryValueExW(
                self.0,
                name.as_ref().as_ptr(),
                null(),
                &mut ty,
                value.as_mut_ptr(),
                &mut len,
            )
        };

        win32_error(result)?;
        Ok((ty.into(), value.get(0..len as usize).unwrap()))
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        unsafe { RegCloseKey(self.0) };
    }
}

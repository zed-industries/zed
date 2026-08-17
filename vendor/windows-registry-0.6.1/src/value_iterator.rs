use super::*;

/// An iterator of registry values.
pub struct ValueIterator<'a> {
    key: &'a Key,
    range: core::ops::Range<usize>,
    name: Vec<u16>,
    data: Data,
}

impl<'a> ValueIterator<'a> {
    pub(crate) fn new(key: &'a Key) -> Result<Self> {
        let mut count = 0;
        let mut name_max_len = 0;
        let mut value_max_len = 0;

        let result = unsafe {
            RegQueryInfoKeyW(
                key.0,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                &mut count,
                &mut name_max_len,
                &mut value_max_len,
                null_mut(),
                null_mut(),
            )
        };

        win32_error(result)?;

        Ok(Self {
            key,
            range: 0..count as usize,
            name: vec![0; name_max_len as usize + 1],
            data: Data::new(value_max_len as usize),
        })
    }
}

impl Iterator for ValueIterator<'_> {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().and_then(|index| {
            let mut ty = 0;
            let mut name_len = self.name.len() as u32;
            let mut data_len = self.data.len() as u32;

            let result = unsafe {
                RegEnumValueW(
                    self.key.0,
                    index as u32,
                    self.name.as_mut_ptr(),
                    &mut name_len,
                    core::ptr::null(),
                    &mut ty,
                    self.data.as_mut_ptr(),
                    &mut data_len,
                )
            };

            if result != 0 {
                debug_assert_eq!(result, ERROR_NO_MORE_ITEMS);
                None
            } else {
                let name = String::from_utf16_lossy(&self.name[0..name_len as usize]);
                Some((
                    name,
                    Value {
                        data: Data::from_slice(&self.data[0..data_len as usize]),
                        ty: ty.into(),
                    },
                ))
            }
        })
    }
}

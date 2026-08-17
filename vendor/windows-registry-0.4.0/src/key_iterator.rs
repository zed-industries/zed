use super::*;

/// An iterator of registry key names.
pub struct KeyIterator<'a> {
    key: &'a Key,
    range: core::ops::Range<usize>,
    name: Vec<u16>,
}

impl<'a> KeyIterator<'a> {
    pub(crate) fn new(key: &'a Key) -> Result<Self> {
        let mut count = 0;
        let mut max_len = 0;

        let result = unsafe {
            RegQueryInfoKeyW(
                key.0,
                null_mut(),
                null_mut(),
                null_mut(),
                &mut count,
                &mut max_len,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };

        win32_error(result).map(|_| Self {
            key,
            range: 0..count as usize,
            name: vec![0; max_len as usize + 1],
        })
    }
}

impl Iterator for KeyIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().and_then(|index| {
            let mut len = self.name.len() as u32;

            let result = unsafe {
                RegEnumKeyExW(
                    self.key.0,
                    index as u32,
                    self.name.as_mut_ptr(),
                    &mut len,
                    null(),
                    null_mut(),
                    null_mut(),
                    null_mut(),
                )
            };

            if result != 0 {
                debug_assert_eq!(result, ERROR_NO_MORE_ITEMS);
                None
            } else {
                Some(String::from_utf16_lossy(&self.name[0..len as usize]))
            }
        })
    }
}

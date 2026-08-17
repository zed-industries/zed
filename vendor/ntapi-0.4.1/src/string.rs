/// A wrapper over UTF16 string constants.
pub struct UTF16Const(pub &'static [u16]);
impl UTF16Const {
    #[inline]
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u16 {
        self.0.as_ptr() as *mut u16
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len() - 1
    }
}
impl AsRef<[u16]> for UTF16Const {
    #[inline]
    fn as_ref(&self) -> &[u16] {
        &self.0[..self.len()]
    }
}
impl Copy for UTF16Const {}
impl Clone for UTF16Const {
    #[inline]
    fn clone(&self) -> UTF16Const { *self }
}
/// A wrapper over UTF8 string constants.
pub struct UTF8Const(pub &'static str);
impl UTF8Const {
    #[inline]
    pub fn as_ptr(&self) -> *const i8 {
        self.0.as_ptr() as *const i8
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut i8 {
        self.0.as_ptr() as *mut i8
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len() - 1
    }
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0[..self.len()]
    }
}
impl AsRef<str> for UTF8Const {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0[..self.len()]
    }
}
impl Copy for UTF8Const {}
impl Clone for UTF8Const {
    #[inline]
    fn clone(&self) -> UTF8Const { *self }
}

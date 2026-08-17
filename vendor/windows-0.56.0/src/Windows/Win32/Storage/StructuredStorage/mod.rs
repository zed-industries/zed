#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct JET_API_PTR(pub usize);
impl JET_API_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for JET_API_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for JET_API_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for JET_API_PTR {}
impl core::fmt::Debug for JET_API_PTR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_API_PTR").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for JET_API_PTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct JET_HANDLE(pub usize);
impl JET_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for JET_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for JET_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for JET_HANDLE {}
impl core::fmt::Debug for JET_HANDLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_HANDLE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for JET_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct JET_INSTANCE(pub usize);
impl JET_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for JET_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for JET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for JET_INSTANCE {}
impl core::fmt::Debug for JET_INSTANCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_INSTANCE").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for JET_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct JET_SESID(pub usize);
impl JET_SESID {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for JET_SESID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for JET_SESID {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for JET_SESID {}
impl core::fmt::Debug for JET_SESID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_SESID").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for JET_SESID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct JET_TABLEID(pub usize);
impl JET_TABLEID {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for JET_TABLEID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl Clone for JET_TABLEID {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for JET_TABLEID {}
impl core::fmt::Debug for JET_TABLEID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_TABLEID").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for JET_TABLEID {
    type TypeKind = windows_core::CopyType;
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl windows_core::TypeKind for JET_API_PTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl windows_core::TypeKind for JET_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl windows_core::TypeKind for JET_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl windows_core::TypeKind for JET_SESID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl windows_core::TypeKind for JET_TABLEID {
    type TypeKind = windows_core::CopyType;
}

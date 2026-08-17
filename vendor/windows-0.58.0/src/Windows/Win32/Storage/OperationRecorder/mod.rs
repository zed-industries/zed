#[inline]
pub unsafe fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> super::super::Foundation::BOOL {
    windows_targets::link!("advapi32.dll" "system" fn OperationEnd(operationendparams : *const OPERATION_END_PARAMETERS) -> super::super::Foundation:: BOOL);
    OperationEnd(operationendparams)
}
#[inline]
pub unsafe fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> super::super::Foundation::BOOL {
    windows_targets::link!("advapi32.dll" "system" fn OperationStart(operationstartparams : *const OPERATION_START_PARAMETERS) -> super::super::Foundation:: BOOL);
    OperationStart(operationstartparams)
}
pub const OPERATION_END_DISCARD: OPERATION_END_PARAMETERS_FLAGS = OPERATION_END_PARAMETERS_FLAGS(1u32);
pub const OPERATION_START_TRACE_CURRENT_THREAD: OPERATION_START_FLAGS = OPERATION_START_FLAGS(1u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPERATION_END_PARAMETERS_FLAGS(pub u32);
impl windows_core::TypeKind for OPERATION_END_PARAMETERS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPERATION_END_PARAMETERS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPERATION_END_PARAMETERS_FLAGS").field(&self.0).finish()
    }
}
impl OPERATION_END_PARAMETERS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPERATION_END_PARAMETERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPERATION_END_PARAMETERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPERATION_START_FLAGS(pub u32);
impl windows_core::TypeKind for OPERATION_START_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPERATION_START_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPERATION_START_FLAGS").field(&self.0).finish()
    }
}
impl OPERATION_START_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPERATION_START_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPERATION_START_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPERATION_START_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPERATION_START_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPERATION_START_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPERATION_END_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_END_PARAMETERS_FLAGS,
}
impl windows_core::TypeKind for OPERATION_END_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPERATION_END_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPERATION_START_PARAMETERS {
    pub Version: u32,
    pub OperationId: u32,
    pub Flags: OPERATION_START_FLAGS,
}
impl windows_core::TypeKind for OPERATION_START_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPERATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

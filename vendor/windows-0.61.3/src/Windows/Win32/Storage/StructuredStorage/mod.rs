#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_API_PTR(pub usize);
impl JET_API_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_HANDLE(pub usize);
impl JET_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct JET_TABLEID(pub usize);
impl JET_TABLEID {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}

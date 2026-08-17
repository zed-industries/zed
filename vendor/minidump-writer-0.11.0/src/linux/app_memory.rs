// These entries store a list of memory regions that the client wants included
// in the minidump.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct AppMemory {
    pub ptr: usize,
    pub length: usize,
}

pub type AppMemoryList = Vec<AppMemory>;

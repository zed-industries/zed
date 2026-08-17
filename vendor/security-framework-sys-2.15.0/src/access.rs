use core_foundation_sys::base::CFTypeID;

extern "C" {
    pub fn SecAccessGetTypeID() -> CFTypeID;
}

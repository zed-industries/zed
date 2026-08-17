use metal::MTLDevice;

extern "C" {
    pub fn CGDirectDisplayCopyCurrentMetalDevice(display: u32) -> *mut MTLDevice;
}

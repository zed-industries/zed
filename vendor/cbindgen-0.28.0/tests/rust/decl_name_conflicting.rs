mod uhoh {
    enum BindingType { Buffer, NotBuffer }
}

#[repr(u32)]
pub enum BindingType { Buffer = 0, NotBuffer = 1 }

#[repr(C)]
pub struct BindGroupLayoutEntry {
    pub ty: BindingType, // This is the repr(u32) enum
}

#[no_mangle]
pub extern "C" fn root(entry: BindGroupLayoutEntry) {}

use crate::rt::async_support::raw_stream_new;
use crate::{RawStreamReader, RawStreamWriter, StreamOps};
use std::alloc::Layout;

/// Operations for `stream<()>`.
///
/// Can be combined with [`RawStreamWriter`] and [`RawStreamReader`] as created
/// through [`UnitStreamOps::new`]
#[derive(Copy, Clone)]
pub struct UnitStreamOps;

extern_wasm! {
    #[link(wasm_import_module = "$root")]
    unsafe extern "C" {
        #[link_name = "[stream-new-unit]"]
        fn unit_new() -> u64;
        #[link_name = "[async-lower][stream-write-unit]"]
        fn unit_write(stream: u32, val: *const u8, amt: usize) -> u32;
        #[link_name = "[async-lower][stream-read-unit]"]
        fn unit_read(stream: u32, val: *mut u8, amt: usize) -> u32;
        #[link_name = "[stream-cancel-read-unit]"]
        fn unit_cancel_read(stream: u32) -> u32;
        #[link_name = "[stream-cancel-write-unit]"]
        fn unit_cancel_write(stream: u32) -> u32;
        #[link_name = "[stream-drop-readable-unit]"]
        fn unit_drop_readable(stream: u32) ;
        #[link_name = "[stream-drop-writable-unit]"]
        fn unit_drop_writable(stream: u32) ;
    }
}

impl UnitStreamOps {
    /// Creates a new unit stream read/write pair.
    pub fn new() -> (RawStreamWriter<Self>, RawStreamReader<Self>) {
        unsafe { raw_stream_new(UnitStreamOps) }
    }
}

unsafe impl StreamOps for UnitStreamOps {
    type Payload = ();

    fn new(&mut self) -> u64 {
        unsafe { unit_new() }
    }
    fn elem_layout(&self) -> Layout {
        Layout::new::<()>()
    }
    fn native_abi_matches_canonical_abi(&self) -> bool {
        true
    }
    fn contains_lists(&self) -> bool {
        false
    }
    unsafe fn lower(&mut self, (): (), _dst: *mut u8) {
        unreachable!()
    }
    unsafe fn dealloc_lists(&mut self, _dst: *mut u8) {
        unreachable!()
    }
    unsafe fn lift(&mut self, _dst: *mut u8) -> Self::Payload {
        unreachable!()
    }
    unsafe fn start_write(&mut self, stream: u32, val: *const u8, amt: usize) -> u32 {
        unsafe { unit_write(stream, val, amt) }
    }
    unsafe fn start_read(&mut self, stream: u32, val: *mut u8, amt: usize) -> u32 {
        unsafe { unit_read(stream, val, amt) }
    }
    unsafe fn cancel_read(&mut self, stream: u32) -> u32 {
        unsafe { unit_cancel_read(stream) }
    }
    unsafe fn cancel_write(&mut self, stream: u32) -> u32 {
        unsafe { unit_cancel_write(stream) }
    }
    unsafe fn drop_readable(&mut self, stream: u32) {
        unsafe { unit_drop_readable(stream) }
    }
    unsafe fn drop_writable(&mut self, stream: u32) {
        unsafe { unit_drop_writable(stream) }
    }
}

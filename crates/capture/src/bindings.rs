include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn dispatch_get_main_queue() -> dispatch_queue_t {
    unsafe { &_dispatch_main_q as *const _ as dispatch_queue_t }
}

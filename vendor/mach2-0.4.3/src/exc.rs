//! This module roughly corresponds to `mach/exc.h`.

use exception_types::{exception_data_t, exception_type_t};
use kern_return::kern_return_t;
use message::{
    mach_msg_body_t, mach_msg_header_t, mach_msg_port_descriptor_t, mach_msg_type_number_t,
};
use ndr::NDR_record_t;
use port::mach_port_t;
use thread_status::thread_state_t;
use vm_types::integer_t;

pub const exc_MSG_COUNT: ::libc::c_uint = 3;

extern "C" {
    pub fn exception_raise(
        exception_port: mach_port_t,
        thread: mach_port_t,
        task: mach_port_t,
        exception: exception_type_t,
        code: exception_data_t,
        codeCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn exception_raise_state(
        exception_port: mach_port_t,
        exception: exception_type_t,
        code: exception_data_t,
        codeCnt: mach_msg_type_number_t,
        flavor: *mut ::libc::c_int,
        old_state: thread_state_t,
        old_stateCnt: mach_msg_type_number_t,
        new_state: thread_state_t,
        new_stateCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn exception_raise_state_identity(
        exception_port: mach_port_t,
        thread: mach_port_t,
        task: mach_port_t,
        exception: exception_type_t,
        code: exception_data_t,
        codeCnt: mach_msg_type_number_t,
        flavor: *mut ::libc::c_int,
        old_state: thread_state_t,
        old_stateCnt: mach_msg_type_number_t,
        new_state: thread_state_t,
        new_stateCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct __Request__exception_raise_t {
    pub Head: mach_msg_header_t,
    /* start of the kernel processed data */
    pub msgh_body: mach_msg_body_t,
    pub thread: mach_msg_port_descriptor_t,
    pub task: mach_msg_port_descriptor_t,
    /* end of the kernel processed data */
    pub NDR: NDR_record_t,
    pub exception: exception_type_t,
    pub codeCnt: mach_msg_type_number_t,
    pub code: [integer_t; 2],
}

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct __Reply__exception_raise_t {
    pub Head: mach_msg_header_t,
    pub NDR: NDR_record_t,
    pub RetCode: kern_return_t,
}

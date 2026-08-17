//! This module roughly corresponds to `mach/ndr.h`.

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct NDR_record_t {
    mig_vers: libc::c_uchar,
    if_vers: libc::c_uchar,
    reserved1: libc::c_uchar,
    mig_encoding: libc::c_uchar,
    int_rep: libc::c_uchar,
    char_rep: libc::c_uchar,
    float_rep: libc::c_uchar,
    reserved32: libc::c_uchar,
}

extern "C" {
    pub static NDR_record: NDR_record_t;
}

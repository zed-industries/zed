//! UEFI Vendor Protocols
//!
//! Many vendor protocols are not part of the official specification. But we
//! still allow importing them here, so we have a central place to collect
//! them. Note that we separate them by vendor-name, which is not the best
//! name-space but should be acceptible.

pub mod intel {
    pub mod console_control;
}

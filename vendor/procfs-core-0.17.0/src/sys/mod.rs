//! Sysctl is a means of configuring certain aspects of the kernel at run-time,
//! and the `/proc/sys/` directory is there so that you don't even need special tools to do it!
//!
//! This directory (present since 1.3.57) contains a number of files
//! and subdirectories corresponding to kernel variables.
//! These variables can be read and sometimes modified using the `/proc` filesystem,
//! and the (deprecated) sysctl(2) system call.

pub mod kernel;

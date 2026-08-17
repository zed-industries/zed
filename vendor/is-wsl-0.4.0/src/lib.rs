extern crate is_docker;
extern crate once_cell;

use once_cell::sync::OnceCell;
use std::{fs::File, io::Read};

pub fn is_wsl() -> bool {
    static CACHED_RESULT: OnceCell<bool> = OnceCell::new();

    *CACHED_RESULT.get_or_init(|| {
        if std::env::consts::OS != "linux" {
            return false;
        }

        if let Ok(os_release) = get_os_release() {
            if os_release.to_lowercase().contains("microsoft") {
                return !is_docker::is_docker();
            }
        }

        if proc_version_includes_microsoft() {
            !is_docker::is_docker()
        } else {
            false
        }
    })
}

fn proc_version_includes_microsoft() -> bool {
    match std::fs::read_to_string("/proc/version") {
        Ok(file_contents) => file_contents.to_lowercase().contains("microsoft"),
        Err(_) => false,
    }
}

// This function is copied from the sys-info crate to avoid taking a dependency on all of sys-info
// https://docs.rs/sys-info/0.9.1/src/sys_info/lib.rs.html#426-433
//
// The MIT License (MIT)
//
// Copyright (c) 2015 Siyu Wang
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
fn get_os_release() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("/proc/sys/kernel/osrelease")?.read_to_string(&mut s)?;
    s.pop(); // pop '\n'
    Ok(s)
}

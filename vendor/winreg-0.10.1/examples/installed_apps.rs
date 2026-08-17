// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#[macro_use]
extern crate serde_derive;
extern crate winreg;
use std::collections::HashMap;
use std::fmt;
use winreg::enums::*;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct InstalledApp {
    DisplayName: Option<String>,
    DisplayVersion: Option<String>,
    UninstallString: Option<String>,
}

macro_rules! str_from_opt {
    ($s:expr) => {
        $s.as_ref().map(|x| &**x).unwrap_or("")
    };
}

impl fmt::Display for InstalledApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}",
            str_from_opt!(self.DisplayName),
            str_from_opt!(self.DisplayVersion)
        )
    }
}

fn main() {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_key = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")
        .expect("key is missing");

    let apps: HashMap<String, InstalledApp> =
        uninstall_key.decode().expect("deserialization failed");

    for v in apps.values() {
        println!("{}", v);
    }
}

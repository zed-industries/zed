// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#![macro_use]

macro_rules! with_key {
    ($k:ident, $path:expr => $b:block) => {{
        let mut path = "Software\\WinRegRsTest".to_owned();
        path.push_str($path);
        let ($k, _disp) = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
            .create_subkey(&path).unwrap();
        $b
        winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .delete_subkey_all(path).unwrap();
    }}
}

// Copyright 2021, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
extern crate winreg;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    {
        // put this in a block so app_key_1 gets out of scope and doesn't prevent us
        // from loading the key again later
        let app_key_1 = RegKey::load_app_key("myhive.dat", true)?;
        app_key_1.set_value("answer", &42u32)?;
    }
    let answer: u32 = {
        // NOTE: on Windows 7 this fails with ERROR_ALREADY_EXISTS
        let app_key_2 =
            RegKey::load_app_key_with_flags("myhive.dat", KEY_READ, REG_PROCESS_APPKEY)?;
        app_key_2.get_value("answer")?
    };
    println!("The Answer is {}", answer);
    Ok(())
}

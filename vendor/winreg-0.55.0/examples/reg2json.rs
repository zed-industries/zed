// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
extern crate serde_transcode;
extern crate winreg;
use std::error::Error;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> Result<(), Box<dyn Error>> {
    let key = RegKey::predef(HKEY_CLASSES_ROOT).open_subkey("Folder")?;

    let mut deserializer = winreg::decoder::Decoder::from_key(&key)?;
    let mut serializer = serde_json::Serializer::pretty(std::io::stdout());

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    Ok(())
}

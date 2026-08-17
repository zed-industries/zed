// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use crate::enums::*;
use crate::types::FromRegValue;
use std::fmt;

/// Raw registry value
#[derive(PartialEq)]
pub struct RegValue {
    pub bytes: Vec<u8>,
    pub vtype: RegType,
}

macro_rules! format_reg_value {
    ($e:expr => $t:ident) => {
        match $t::from_reg_value($e) {
            Ok(val) => format!("{:?}", val),
            Err(_) => return Err(fmt::Error),
        }
    };
}

impl fmt::Display for RegValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_val = match self.vtype {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => format_reg_value!(self => String),
            REG_DWORD => format_reg_value!(self => u32),
            REG_QWORD => format_reg_value!(self => u64),
            _ => format!("{:?}", self.bytes), //TODO: implement more types
        };
        write!(f, "{}", f_val)
    }
}

impl fmt::Debug for RegValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegValue({:?}: {})", self.vtype, self)
    }
}

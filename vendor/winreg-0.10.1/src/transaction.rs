// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.

//! Structure for a registry transaction.
//! Part of `transactions` feature.
//!
//!```no_run
//!extern crate winreg;
//!use std::io;
//!use winreg::RegKey;
//!use winreg::enums::*;
//!use winreg::transaction::Transaction;
//!
//!fn main() {
//!    let t = Transaction::new().unwrap();
//!    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
//!    let (key, _disp) = hkcu.create_subkey_transacted("Software\\RustTransaction", &t).unwrap();
//!    key.set_value("TestQWORD", &1234567891011121314u64).unwrap();
//!    key.set_value("TestDWORD", &1234567890u32).unwrap();
//!
//!    println!("Commit transaction? [y/N]:");
//!    let mut input = String::new();
//!    io::stdin().read_line(&mut input).unwrap();
//!    input = input.trim_right().to_owned();
//!    if input == "y" || input == "Y" {
//!        t.commit().unwrap();
//!        println!("Transaction committed.");
//!    }
//!    else {
//!        // this is optional, if transaction wasn't committed,
//!        // it will be rolled back on disposal
//!        t.rollback().unwrap();
//!
//!        println!("Transaction wasn't committed, it will be rolled back.");
//!    }
//!}
//!```
#![cfg(feature = "transactions")]
use std::io;
use std::ptr;
use winapi::um::handleapi;
use winapi::um::ktmw32;
use winapi::um::winnt;

#[derive(Debug)]
pub struct Transaction {
    pub handle: winnt::HANDLE,
}

impl Transaction {
    //TODO: add arguments
    pub fn new() -> io::Result<Transaction> {
        unsafe {
            let handle = ktmw32::CreateTransaction(
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                0,
                0,
                0,
                ptr::null_mut(),
            );
            if handle == handleapi::INVALID_HANDLE_VALUE {
                return Err(io::Error::last_os_error());
            };
            Ok(Transaction { handle })
        }
    }

    pub fn commit(&self) -> io::Result<()> {
        unsafe {
            match ktmw32::CommitTransaction(self.handle) {
                0 => Err(io::Error::last_os_error()),
                _ => Ok(()),
            }
        }
    }

    pub fn rollback(&self) -> io::Result<()> {
        unsafe {
            match ktmw32::RollbackTransaction(self.handle) {
                0 => Err(io::Error::last_os_error()),
                _ => Ok(()),
            }
        }
    }

    fn close_(&mut self) -> io::Result<()> {
        unsafe {
            match handleapi::CloseHandle(self.handle) {
                0 => Err(io::Error::last_os_error()),
                _ => Ok(()),
            }
        }
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        self.close_().unwrap_or(());
    }
}

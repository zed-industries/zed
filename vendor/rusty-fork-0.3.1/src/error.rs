//-
// Copyright 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use crate::cmdline;

quick_error! {
    /// Enum for errors produced by the rusty-fork crate.
    #[derive(Debug)]
    pub enum Error {
        /// An unknown flag was encountered when examining the current
        /// process's argument list.
        ///
        /// The string is the flag that was encountered.
        UnknownFlag(flag: String) {
            display("The flag '{:?}' was passed to the Rust test \
                     process, but rusty-fork does not know how to \
                     handle it.\n\
                     If you are using the standard Rust \
                     test harness and have the latest version of the \
                     rusty-fork crate, please report a bug to\n\
                     \thttps://github.com/AltSysrq/rusty-fork/issues\n\
                     In the mean time, you can tell rusty-fork how to \
                     handle this flag by setting the environment variable \
                     `{}` to one of the following values:\n\
                     \tpass - Pass the flag (alone) to the child process\n\
                     \tpass-arg - Pass the flag and its following argument \
                     to the child process.\n\
                     \tdrop - Don't pass the flag to the child process.\n\
                     \tdrop-arg - Don't pass the flag or its following \
                     argument to the child process.",
                    flag, cmdline::env_var_for_flag(&flag))
        }
        /// A flag was encountered when examining the current process's
        /// argument list which is known but cannot be handled in any sensible
        /// way.
        ///
        /// The strings are the flag encountered and a human-readable message
        /// about why the flag could not be handled.
        DisallowedFlag(flag: String, message: String) {
            display("The flag '{:?}' was passed to the Rust test \
                     process, but rusty-fork cannot handle it; \
                     reason: {}", flag, message)
        }
        /// Spawning a subprocess failed.
        SpawnError(err: io::Error) {
            from()
            cause(err)
            display("Spawn failed: {}", err)
        }
    }
}

/// General `Result` type for rusty-fork.
pub type Result<T> = ::std::result::Result<T, Error>;

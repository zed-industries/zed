//! build.rs file to obtain the host information.

// Allow dead code in triple.rs and targets.rs for our purposes here.
#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

// Include triple.rs and targets.rs so we can parse the TARGET environment variable.
// targets.rs depends on data_model
mod data_model {
    include!("src/data_model.rs");
}
mod triple {
    include!("src/triple.rs");
}
mod targets {
    include!("src/targets.rs");
}

// Stub out `ParseError` to minimally support triple.rs and targets.rs.
mod parse_error {
    #[derive(Debug)]
    pub enum ParseError {
        UnrecognizedArchitecture(String),
        UnrecognizedVendor(String),
        UnrecognizedOperatingSystem(String),
        UnrecognizedEnvironment(String),
        UnrecognizedBinaryFormat(String),
        UnrecognizedField(String),
    }
}

use self::targets::Vendor;
use self::triple::Triple;

fn main() {
    let out_dir = PathBuf::from(
        env::var_os("OUT_DIR").expect("The OUT_DIR environment variable must be set"),
    );
    let target = env::var("TARGET").expect("The TARGET environment variable must be set");
    let triple =
        Triple::from_str(&target).unwrap_or_else(|_| panic!("Invalid target name: '{}'", target));
    let out = File::create(out_dir.join("host.rs")).expect("error creating host.rs");
    write_host_rs(out, triple).expect("error writing host.rs");
    if using_1_40() {
        println!("cargo:rustc-cfg=feature=\"rust_1_40\"");
    }
}

fn using_1_40() -> bool {
    match (|| {
        let rustc = env::var_os("RUSTC").unwrap();
        let output = Command::new(rustc).arg("--version").output().ok()?;
        let stdout = if output.status.success() {
            output.stdout
        } else {
            return None;
        };
        std::str::from_utf8(&stdout)
            .ok()?
            .split(' ')
            .nth(1)?
            .split('.')
            .nth(1)?
            .parse::<i32>()
            .ok()
    })() {
        Some(version) => version >= 40,
        None => true, // assume we're using an up-to-date compiler
    }
}

fn write_host_rs(mut out: File, triple: Triple) -> io::Result<()> {
    // The generated Debug implementation for the inner architecture variants
    // doesn't print the enum name qualifier, so import them here. There
    // shouldn't be any conflicts because these enums all share a namespace
    // in the triple string format.
    writeln!(
        out,
        r#"
#[allow(unused_imports)]
use crate::Aarch64Architecture::*;
#[allow(unused_imports)]
use crate::ArmArchitecture::*;
#[allow(unused_imports)]
use crate::CustomVendor;
#[allow(unused_imports)]
use crate::Mips32Architecture::*;
#[allow(unused_imports)]
use crate::Mips64Architecture::*;
#[allow(unused_imports)]
use crate::Riscv32Architecture::*;
#[allow(unused_imports)]
use crate::Riscv64Architecture::*;
#[allow(unused_imports)]
use crate::X86_32Architecture::*;

/// The `Triple` of the current host.
pub const HOST: Triple = Triple {{
    architecture: Architecture::{architecture:?},
    vendor: Vendor::{vendor},
    operating_system: OperatingSystem::{operating_system:?},
    environment: Environment::{environment:?},
    binary_format: BinaryFormat::{binary_format:?},
}};

impl Architecture {{
    /// Return the architecture for the current host.
    pub const fn host() -> Self {{
        Architecture::{architecture:?}
    }}
}}

impl Vendor {{
    /// Return the vendor for the current host.
    pub const fn host() -> Self {{
        Vendor::{vendor}
    }}
}}

impl OperatingSystem {{
    /// Return the operating system for the current host.
    pub const fn host() -> Self {{
        OperatingSystem::{operating_system:?}
    }}
}}

impl Environment {{
    /// Return the environment for the current host.
    pub const fn host() -> Self {{
        Environment::{environment:?}
    }}
}}

impl BinaryFormat {{
    /// Return the binary format for the current host.
    pub const fn host() -> Self {{
        BinaryFormat::{binary_format:?}
    }}
}}

impl Triple {{
    /// Return the triple for the current host.
    pub const fn host() -> Self {{
        Self {{
            architecture: Architecture::{architecture:?},
            vendor: Vendor::{vendor},
            operating_system: OperatingSystem::{operating_system:?},
            environment: Environment::{environment:?},
            binary_format: BinaryFormat::{binary_format:?},
        }}
    }}
}}"#,
        architecture = triple.architecture,
        vendor = vendor_display(&triple.vendor),
        operating_system = triple.operating_system,
        environment = triple.environment,
        binary_format = triple.binary_format,
    )?;

    Ok(())
}

fn vendor_display(vendor: &Vendor) -> String {
    match vendor {
        Vendor::Custom(custom) => format!("Custom(CustomVendor::Static({:?}))", custom.as_str()),
        known => format!("{:?}", known),
    }
}

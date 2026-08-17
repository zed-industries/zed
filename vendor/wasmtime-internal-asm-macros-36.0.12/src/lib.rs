//! This crate defines a macro named `asm_func!` which is suitable for
//! generating a single `global_asm!`-defined function.
//!
//! This macro takes care of platform-specific directives to get the symbol
//! attributes correct (e.g. ELF symbols get a size and are flagged as a
//! function) and additionally handles visibility across platforms. All symbols
//! should be visible to Rust but not visible externally outside of a `*.so`.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.

#![no_std]

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "apple")] {
        #[macro_export]
        macro_rules! asm_func {
            ($name:expr, $body:expr $(, $($args:tt)*)?) => {
                core::arch::global_asm!(
                    concat!(
                        ".p2align 4\n",
                        ".private_extern _", $name, "\n",
                        ".global _", $name, "\n",
                        "_", $name, ":\n",
                        $body,
                    ),
                    $($($args)*)?
                );
            };
        }
    } else if #[cfg(target_os = "windows")] {
        #[macro_export]
        macro_rules! asm_func {
            ($name:expr, $body:expr $(, $($args:tt)*)?) => {
                core::arch::global_asm!(
                    concat!(
                        ".def ", $name, "\n",
                        ".scl 2\n",
                        ".type 32\n",
                        ".endef\n",
                        ".global ", $name, "\n",
                        ".p2align 4\n",
                        $name, ":\n",
                        $body
                    ),
                    $($($args)*)?
                );
            };
        }
    } else {
        // Note that for now this "else" clause just assumes that everything
        // other than macOS is ELF and has the various directives here for
        // that.
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "arm")] {
                #[macro_export]
                macro_rules! elf_func_type_header {
                    ($name:tt) => (concat!(".type ", $name, ",%function\n"))
                }
            } else {
                #[macro_export]
                macro_rules! elf_func_type_header {
                    ($name:tt) => (concat!(".type ", $name, ",@function\n"))
                }
            }
        }

        #[macro_export]
        macro_rules! asm_func {
            ($name:expr, $body:expr $(, $($args:tt)*)?) => {
                core::arch::global_asm!(
                    concat!(
                        ".p2align 4\n",
                        ".hidden ", $name, "\n",
                        ".global ", $name, "\n",
                        $crate::elf_func_type_header!($name),
                        $name, ":\n",
                        $body,
                        ".size ", $name, ",.-", $name,
                    )
                    $(, $($args)*)?
                );
            };
        }
    }
}

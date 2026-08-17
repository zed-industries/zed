// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn get_arm64ec_mangled_function_name(name: &str) -> Result<Option<String>, ()> {
    let first_char = name.chars().next().unwrap();
    if first_char != '?' {
        // For non-C++ symbols, prefix the name with "#" unless it's already
        // mangled.
        if first_char == '#' {
            return Ok(None);
        }
        return Ok(Some(format!("#{name}")));
    }

    // If the name contains $$h, then it is already mangled.
    if name.contains("$$h") {
        return Ok(None);
    }

    // Ask the demangler where we should insert "$$h".
    // TODO: In LLVM, this uses the Microsoft Demangler to demangle the name to
    // find the insertion point. The demangler is massive, so rather than
    // porting it we're going to require that the name is already mangled.
    Err(())
}

pub fn get_arm64ec_demangled_function_name(name: &str) -> Option<String> {
    // For non-C++ names, drop the "#" prefix.
    let first_char = name.chars().next().unwrap();
    if first_char == '#' {
        return Some(name[1..].to_string());
    }
    if first_char != '?' {
        return None;
    }

    // Drop the ARM64EC "$$h" tag.
    match name.split_once("$$h") {
        Some((first, second)) if !second.is_empty() => Some(format!("{first}{second}")),
        _ => None,
    }
}

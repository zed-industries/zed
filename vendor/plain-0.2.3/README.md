# libplain

[![Build Status](https://travis-ci.org/randomites/plain.svg?branch=master)](https://travis-ci.org/randomites/plain)
[![Current Crates.io Version](https://img.shields.io/crates/v/plain.svg)](https://crates.io/crates/plain)
[![Current Documentation](https://docs.rs/plain/badge.svg)](https://docs.rs/plain)

A small Rust library that allows users to interpret arrays of bytes
as certain kinds of structures safely.

This crate provides an unsafe trait [`Plain`](https://docs.rs/plain/0.2.0/plain/trait.Plain.html), which the user
of the crate uses to mark types for which operations of this library are safe.
See [`Plain`](https://docs.rs/plain/0.2.0/plain/trait.Plain.html) for the contractual obligation.

Other than that, everything else in this crate is perfectly safe to use as long
as the `Plain` trait is not implemented on inadmissible types (similar to how
`Send` and `Sync` in the standard library work).

# Purpose

In low level systems development, it is sometimes necessary to
interpret locations in memory as data structures. Functions of
this crate serve to avoid pitfalls associated with that, without
having to resort to big, full-featured (de)serialization libraries.

On the other hand, this crate contains no provisions when it comes
to handling differences in encoding and byte ordering between
platforms. As such, it is entirely unsuitable for processing
external data such as file contents or network packets.

# Examples

To start using the crate, simply do `extern crate plain;`.

If you want your plain types to have methods from this crate, also include `use plain.Plain;`.

Then it's just a matter of marking the right types and using them.

```

extern crate plain;
use plain::Plain;
use std::mem;


#[repr(C)]
#[derive(Default)]
struct ELF64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

// SAFE: ELF64Header satisfies all the requirements of `Plain`.
unsafe impl Plain for ELF64Header {}

impl ELF64Header {
	fn from_bytes(buf: &[u8]) -> &ELF64Header {
			plain::from_bytes(buf).expect("The buffer is either too short or not aligned!")
		}

		fn from_mut_bytes(buf: &mut [u8]) -> &mut ELF64Header {
			plain::from_mut_bytes(buf).expect("The buffer is either too short or not aligned!")
		}

		fn copy_from_bytes(buf: &[u8]) -> ELF64Header {
			let mut h = ELF64Header::default();
			h.copy_from_bytes(buf).expect("The buffer is too short!");
			h
		}
}

# fn process_elf(elf: &ELF64Header) {}

// Conditional copying for ultimate hackery.
fn opportunistic_elf_processing(buf: &[u8]) {
	if plain::is_aligned::<ELF64Header>(buf) {
        // No copy necessary.
			let elf_ref = ELF64Header::from_bytes(buf);
			process_elf(elf_ref);
    } else {
        // Not aligned properly, copy to stack first.
			let elf = ELF64Header::copy_from_bytes(buf);
			process_elf(&elf);
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
struct ArrayEntry {
    pub name: [u8; 32],
    pub tag: u32,
    pub score: u32,
}

// SAFE: ArrayEntry satisfies all the requirements of `Plain`.
unsafe impl Plain for ArrayEntry {}

fn array_from_bytes(buf: &[u8]) -> &[ArrayEntry] {
    // NOTE: length is not a concern here,
    // since slice_from_bytes() can return empty slice.

    match plain::slice_from_bytes(buf) {
        Err(_) => panic!("The buffer is not aligned!"),
        Ok(arr) => arr,
    }
}

fn array_from_unaligned_bytes(buf: &[u8]) -> Vec<ArrayEntry> {
		let length = buf.len() / mem::size_of::<ArrayEntry>();
	let mut result = vec![ArrayEntry::default(); length];
 	(&mut result).copy_from_bytes(buf).expect("Cannot fail here.");
		result
}

# fn main() {}

```

# Comparison to [`pod`](https://crates.io/crates/pod)

[`pod`](https://crates.io/crates/pod) is another crate created to help working with plain data.
The major difference between `pod` and `plain` is scope.

`plain` currently provides only a few functions (+method wrappers) and its implementation
involves very few lines of unsafe code. It can be used in `no_std` code. Also, it doesn't
deal with [endianness](https://en.wikipedia.org/wiki/Endianness) in any way,
so it is only suitable for certain kinds of low-level work.

`pod`, on the other hand, provides a wide arsenal
of various methods, most of which may be unnecessary for a given use case.
It has dependencies on `std` as well as other crates, but among other things
it provides tools to handle endianness properly.

In short, `plain` is much, much _plainer_...


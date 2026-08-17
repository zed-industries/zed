//! Apple platform: `section$start$` and `section$end$` symbols.

/// On Apple platforms, the linker provides a pointer to the start and end of
/// the section regardless of the section's name.
#[doc(hidden)]
#[macro_export]
macro_rules! __get_section_apple {
    (movable, name=$name:tt, type=$generic_ty:ty) => {
        {
            $crate::__support::MovableBounds::new(
                $crate::__support::PtrBounds::new(
                    $crate::__address_of_symbol!(item data start $name),
                    $crate::__address_of_symbol!(item data end $name),
                ),
                $crate::__support::PtrBounds::new(
                    $crate::__address_of_symbol!(backref data start $name),
                    $crate::__address_of_symbol!(backref data end $name),
                ),
            )
        }
    };
    ($section_type:ident, name=$name:tt, type=$generic_ty:ty) => {
        {
            $crate::__support::PtrBounds::new(
                $crate::__address_of_symbol!(item data start $name),
                $crate::__address_of_symbol!(item data end $name),
            )
        }
    }
}

pub use crate::__get_section_apple as get_section;

// \x01: "do not mangle" (ref https://github.com/rust-lang/rust-bindgen/issues/2935)
crate::__def_section_name! {
    __section_name_apple,
    {
        data bare =>    ("__DATA,") __ ();
        code bare =>    ("__TEXT,") __ ();
        data section => ("__DATA,") __ (",regular,no_dead_strip");
        code section => ("__TEXT,") __ (",regular,pure_instructions");
        data start =>   ("\x01section$start$__DATA$") __ ();
        data end =>     ("\x01section$end$__DATA$") __ ();
    }
    AUXILIARY = "_";
    REFS = "_r_";
    // We use base63 for hashes. 8 characters yields 248,155,780,267,521
    // possible values. This gives us space for 5,045,539 hashes (which are
    // built on the section's raw name, location information and a hash of the
    // source text) before we have a 5% chance of collision.
    MAX_LENGTH = 16;
    HASH_LENGTH = 8;
    VALID_SECTION_CHARS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
}

const fn find_byte(bytes: &[u8], byte: u8) -> Option<usize> {
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == byte {
            return Some(i);
        }
        i += 1;
    }
    None
}

const fn write_bytes<const N: usize>(out: &mut [u8; N], mut pos: usize, bytes: &[u8]) -> usize {
    let mut i = 0;
    while i < bytes.len() {
        out[pos] = bytes[i];
        pos += 1;
        i += 1;
    }
    pos
}

const fn panic_invalid_apple_section_name(name: &str, reason: &'static str) -> ! {
    const PREFIX: &str = "link-section: invalid section name `";
    const SUFFIX: &str = "`: ";
    let mut out = [b' '; 1024];
    let mut pos = 0;
    pos = write_bytes(&mut out, pos, PREFIX.as_bytes());
    pos = write_bytes(&mut out, pos, name.as_bytes());
    pos = write_bytes(&mut out, pos, SUFFIX.as_bytes());
    write_bytes(&mut out, pos, reason.as_bytes());
    let msg = match core::str::from_utf8(&out) {
        Ok(s) => s,
        Err(_) => reason,
    };
    let msg = msg.trim_ascii();
    panic!("{}", msg);
}

pub(crate) const fn validate_apple_section_name(name: &str) {
    match validate_apple_section_name_res(name) {
        Ok(()) => {}
        Err(reason) => panic_invalid_apple_section_name(name, reason),
    }
}

const fn validate_apple_section_name_res(name: &str) -> Result<(), &'static str> {
    let bytes = name.as_bytes();
    let comma = match find_byte(bytes, b',') {
        Some(i) => i,
        None => return Err("section name must contain a comma"),
    };
    if comma == 0 {
        return Err("section name must have a segment before the comma");
    }

    let mut i = comma + 1;
    if i >= bytes.len() {
        return Err("section name must not be empty");
    }
    let mut section_len = 0;
    while i < bytes.len() {
        if bytes[i] == b',' {
            break;
        }
        if !is_valid_section_char(bytes[i]) {
            return Err("section name contains invalid character(s)");
        }
        section_len += 1;
        i += 1;
    }
    if section_len == 0 {
        return Err("Mach-O section name must not be empty");
    }
    if section_len > MAX_LENGTH {
        if cfg!(feature = "proc_macro") {
            // `hash!` shortens linker section names; const metadata keeps the raw name.
            return Ok(());
        }
        return Err("Mach-O section name must be 1 to 16 characters");
    }
    Ok(())
}

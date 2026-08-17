//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.

#![no_std]

/// Represents the possible sizes in bytes of the discriminant of a variant type in the component model
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DiscriminantSize {
    /// 8-bit discriminant
    Size1,
    /// 16-bit discriminant
    Size2,
    /// 32-bit discriminant
    Size4,
}

impl DiscriminantSize {
    /// Calculate the size of discriminant needed to represent a variant with the specified number of cases.
    pub const fn from_count(count: usize) -> Option<Self> {
        if count <= 0xFF {
            Some(Self::Size1)
        } else if count <= 0xFFFF {
            Some(Self::Size2)
        } else if count <= 0xFFFF_FFFF {
            Some(Self::Size4)
        } else {
            None
        }
    }

    /// Returns the size, in bytes, of this discriminant
    pub const fn byte_size(&self) -> u32 {
        match self {
            DiscriminantSize::Size1 => 1,
            DiscriminantSize::Size2 => 2,
            DiscriminantSize::Size4 => 4,
        }
    }
}

impl From<DiscriminantSize> for u32 {
    /// Size of the discriminant as a `u32`
    fn from(size: DiscriminantSize) -> u32 {
        size.byte_size()
    }
}

impl From<DiscriminantSize> for usize {
    /// Size of the discriminant as a `usize`
    fn from(size: DiscriminantSize) -> usize {
        match size {
            DiscriminantSize::Size1 => 1,
            DiscriminantSize::Size2 => 2,
            DiscriminantSize::Size4 => 4,
        }
    }
}

/// Represents the number of bytes required to store a flags value in the component model
pub enum FlagsSize {
    /// There are no flags
    Size0,
    /// Flags can fit in a u8
    Size1,
    /// Flags can fit in a u16
    Size2,
    /// Flags can fit in a specified number of u32 fields
    Size4Plus(u8),
}

impl FlagsSize {
    /// Calculate the size needed to represent a value with the specified number of flags.
    pub const fn from_count(count: usize) -> FlagsSize {
        if count == 0 {
            FlagsSize::Size0
        } else if count <= 8 {
            FlagsSize::Size1
        } else if count <= 16 {
            FlagsSize::Size2
        } else {
            let amt = count.div_ceil(32);
            if amt > (u8::MAX as usize) {
                panic!("too many flags");
            }
            FlagsSize::Size4Plus(amt as u8)
        }
    }
}

/// A simple bump allocator which can be used with modules
pub const REALLOC_AND_FREE: &str = r#"
    (global $last (mut i32) (i32.const 8))
    (func $realloc (export "realloc")
        (param $old_ptr i32)
        (param $old_size i32)
        (param $align i32)
        (param $new_size i32)
        (result i32)

        (local $ret i32)

        ;; Test if the old pointer is non-null
        local.get $old_ptr
        if
            ;; If the old size is bigger than the new size then
            ;; this is a shrink and transparently allow it
            local.get $old_size
            local.get $new_size
            i32.gt_u
            if
                local.get $old_ptr
                return
            end

            ;; otherwise fall through to allocate a new chunk which will later
            ;; copy data over
        end

        ;; align up `$last`
        (global.set $last
            (i32.and
                (i32.add
                    (global.get $last)
                    (i32.add
                        (local.get $align)
                        (i32.const -1)))
                (i32.xor
                    (i32.add
                        (local.get $align)
                        (i32.const -1))
                    (i32.const -1))))

        ;; save the current value of `$last` as the return value
        global.get $last
        local.set $ret

        ;; bump our pointer
        (global.set $last
            (i32.add
                (global.get $last)
                (local.get $new_size)))

        ;; while `memory.size` is less than `$last`, grow memory
        ;; by one page
        (loop $loop
            (if
                (i32.lt_u
                    (i32.mul (memory.size) (i32.const 65536))
                    (global.get $last))
                (then
                    i32.const 1
                    memory.grow
                    ;; test to make sure growth succeeded
                    i32.const -1
                    i32.eq
                    if unreachable end

                    br $loop)))


        ;; ensure anything necessary is set to valid data by spraying a bit
        ;; pattern that is invalid
        local.get $ret
        i32.const 0xde
        local.get $new_size
        memory.fill

        ;; If the old pointer is present then that means this was a reallocation
        ;; of an existing chunk which means the existing data must be copied.
        local.get $old_ptr
        if
            local.get $ret          ;; destination
            local.get $old_ptr      ;; source
            local.get $old_size     ;; size
            memory.copy
        end

        local.get $ret
    )
"#;

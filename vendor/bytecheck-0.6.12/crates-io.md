bytecheck is a type validation framework for Rust.

## bytecheck in action

```rust
use bytecheck::CheckBytes;

#[derive(CheckBytes, Debug)]
struct Test {
    a: u32,
    b: bool,
    c: char,
}

#[repr(C, align(16))]
struct Aligned<const N: usize>([u8; N]);

macro_rules! bytes {
    ($($byte:literal,)*) => {
        (&Aligned([$($byte,)*]).0 as &[u8]).as_ptr()
    };
    ($($byte:literal),*) => {
        bytes!($($byte,)*)
    };
}

fn main() {
    // This type is laid out as (u32, char, bool)
    unsafe {
        // These are valid bytes for (0, 'x', true)
        Test::check_bytes(
            bytes![
                0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8,
                1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
            ].cast(),
            &()
        ).unwrap();

        // Changing the bytes for the u32 is OK, any bytes are a valid u32
        Test::check_bytes(
            bytes![
                42u8, 16u8, 20u8, 3u8, 0x78u8, 0u8, 0u8, 0u8,
                1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
            ].cast(),
            &()
        ).unwrap();

        // Characters outside the valid ranges are invalid
        Test::check_bytes(
            bytes![
                0u8, 0u8, 0u8, 0u8, 0x00u8, 0xd8u8, 0u8, 0u8,
                1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
            ].cast(),
            &()
        ).unwrap_err();
        Test::check_bytes(
            bytes![
                0u8, 0u8, 0u8, 0u8, 0x00u8, 0x00u8, 0x11u8, 0u8,
                1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
            ].cast(),
            &()
        ).unwrap_err();

        // 0 is a valid boolean value (false) but 2 is not
        Test::check_bytes(
            bytes![
                0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8,
                0u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
            ].cast(),
            &()
        ).unwrap();
        Test::check_bytes(
            bytes![
                0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8,
                2u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
            ].cast(),
            &()
        ).unwrap_err();
    }
}
```

[![Rust Docs](https://docs.rs/lebe/badge.svg)](https://docs.rs/lebe) 
[![Crate Crate](https://img.shields.io/crates/v/lebe.svg)](https://crates.io/crates/lebe) 
![Lines of Code](https://tokei.rs/b1/github/johannesvollmer/lebe?category=code)


# LEBE
Tiny, dead simple, high performance endianness conversions with a generic API.
This crate purposefully does not have a different method, like `write_u16(my_value)`, for each primitive type. Instead, this uses generic type inference: `write(my_u16)`.  

# Purpose
This crate has exactly two purposes:
  1. Simple conversion between slices of primitives and byte arrays without unsafe code
  2. Simple and fast conversion from one endianness to the other one

The [byteorder crate](https://github.com/BurntSushi/byteorder) uses ![Lines of Code](https://tokei.rs/b1/github/BurntSushi/byteorder?category=code) for this.

This simplifies reading and writing binary data to files or network streams.


# Usage

Write values.
```rust
    use lebe::io::WriteEndian;
    use std::io::Write;
    
    fn main(){
        let mut output_bytes: Vec<u8> = Vec::new();

        let numbers: &[i32] = &[ 32, 102, 420, 594 ];
        output_bytes.write_as_little_endian(numbers.len()).unwrap();
        output_bytes.write_as_little_endian(numbers).unwrap();
    }
```

Read numbers.
```rust
    use lebe::io::ReadEndian;
    use std::io::Read;
    
    fn main(){
        let mut input_bytes: &[u8] = &[ 3, 244 ];
        let number: u16 = input_bytes.read_from_little_endian().unwrap();
    }
```

Read slices.
```rust
    use lebe::io::ReadEndian;
    use std::io::Read;
    
    fn main(){
        let mut input_bytes: &[u8] = &[ 0, 2, 0, 3, 244, 1, 0, 3, 244, 1 ];
        
        let len: u16 = input_bytes.read_from_little_endian().unwrap();
        let mut numbers = vec![ 0.0; len as usize ];
        
        input_bytes.read_from_little_endian_into(numbers.as_mut_slice()).unwrap();
    }
```

Convert slices in-place.
```rust
    use lebe::Endian;
    
    fn main(){
        let mut numbers: &[i32] = &[ 32, 102, 420, 594 ];
        numbers.convert_current_to_little_endian();
    }
```


# Why not use [byteorder](https://crates.io/crates/byteorder)?
This crate supports batch-writing slices with native speed 
where the os has the matching endianness. Writing slices in `byteorder` 
must be done manually, and may be slower than expected. 
This crate does provide u8 and i8 slice operations for completeness.
Also, the API of this crate looks simpler.

# Why not use [endianness](https://crates.io/crates/endianness)?
This crate has no runtime costs, just as `byteorder`.

# Why not use this crate?
The other crates probably have better documentation.


# Fun Facts
LEBE is made up from 'le' for little endian and 'be' for big endian.
If you say that word using english pronounciation, 
a german might think you said the german word for 'love'.

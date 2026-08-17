# scroll_derive
Macros 1.1 implementing #[derive(Pread, Pwrite)] for https://github.com/m4b/scroll

Add derive annotations to your POD seamlessly and easily:

```rust
extern crate scroll;
#[macro_use]
extern crate scroll_derive;

#[derive(Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
#[repr(C)]
struct Data {
    id: u32,
    timestamp: f64,
    arr: [u16; 2],
}

use scroll::{Pread, Pwrite, Cread, LE};

fn main (){
    let bytes = [0xefu8, 0xbe, 0xad, 0xde, 0, 0, 0, 0, 0, 0, 224, 63, 0xad, 0xde, 0xef, 0xbe];
    let data: Data = bytes.pread_with(0, LE).unwrap();
    println!("data: {data:?}");
    assert_eq!(data.id, 0xdeadbeefu32);
    let mut bytes2 = vec![0; ::std::mem::size_of::<Data>()];
    bytes2.pwrite_with(data, 0, LE).unwrap();
    let data: Data = bytes.pread_with(0, LE).unwrap();
    let data2: Data = bytes2.pread_with(0, LE).unwrap();
    assert_eq!(data, data2);

    let data: Data = bytes.cread_with(0, LE);
    assert_eq!(data, data2);
}
```

[![crates.io version]][crates.io link] [![docs-badge][]][docs]

# imagesize
Quickly probe the size of various image formats without reading the entire file.

The goal of this crate is to be able to read the dimensions of a supported image without loading unnecessary data, and without pulling in more dependencies. Most reads only require 16 bytes or less, and more complex formats take advantage of skipping junk data.

## Usage
Add the following to your Cargo.toml:
```toml
[dependencies]
imagesize = "0.14"
```

## Supported Image Formats
* Aseprite
* Avif
* BMP
* DDS
* EXR
* Farbfeld
* GIF
* HDR
* HEIC / HEIF
* ICO*
* ILBM (IFF)
* JPEG
* JPEG XL
* KTX2
* PNG
* PNM (PBM, PGM, PPM)
* PSD / PSB
* QOI
* TGA
* TIFF
* VTF
* WEBP

If you have a format you think should be added, feel free to create an issue.

*ICO files can contain multiple images, `imagesize` will give the dimensions of the largest one.

## Examples

### From a file
```rust
match imagesize::size("example.webp") {
    Ok(size) => println!("Image dimensions: {}x{}", size.width, size.height),
    Err(why) => println!("Error getting dimensions: {:?}", why)
}
```

### From a vector
```rust
let data = vec![0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x64, 0x00, 0x64, 0x00];
match imagesize::blob_size(&data) {
    Ok(size) => println!("Image dimensions: {}x{}", size.width, size.height),
    Err(why) => println!("Error getting dimensions: {:?}", why),
}
```

[crates.io link]: https://crates.io/crates/imagesize
[crates.io version]: https://img.shields.io/crates/v/imagesize.svg?style=flat-square
[docs]: https://docs.rs/imagesize
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square

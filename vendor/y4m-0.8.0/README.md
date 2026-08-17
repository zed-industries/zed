# y4m [![Build Status](https://travis-ci.com/image-rs/y4m.png?branch=master)](https://travis-ci.com/github/image-rs/y4m) [![crates.io](https://img.shields.io/crates/v/y4m.svg)](https://crates.io/crates/y4m)

YUV4MPEG2 (.y4m) Encoder/Decoder. [Format specification](https://wiki.multimedia.cx/index.php?title=YUV4MPEG2).

## Usage

Simple stream copying:

```rust
extern crate y4m;
use std::io;

let mut infh = io::stdin();
let mut outfh = io::stdout();
let mut dec = y4m::decode(&mut infh).unwrap();
let mut enc = y4m::encode(dec.get_width(), dec.get_height(), dec.get_framerate())
    .with_colorspace(dec.get_colorspace())
    .write_header(&mut outfh)
    .unwrap();
loop {
    match dec.read_frame() {
        Ok(frame) => if enc.write_frame(&frame).is_err() { break },
        _ => break,
    }
}
```

See [API documentation](https://docs.rs/y4m) for overview of all available methods. See also [this example](examples/resize.rs) on how to resize input y4m into grayscale y4m of different resolution:

```bash
cargo build --release --example resize
ffmpeg -i in.mkv -f yuv4mpegpipe - | target/release/examples/resize - 640x360 - | mpv -
```

## License

Library is licensed under [MIT](LICENSE).

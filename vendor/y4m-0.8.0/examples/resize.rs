extern crate resize;
extern crate y4m;

use resize::Pixel::Gray8;
use resize::Type::Triangle;
use std::env;
use std::fs::File;
use std::io;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        return println!("Usage: {} in.y4m WxH out.y4m", args[0]);
    }

    let mut infh: Box<dyn io::Read> = if args[1] == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(&args[1]).unwrap())
    };
    let mut decoder = y4m::decode(&mut infh).unwrap();

    if decoder.get_bit_depth() != 8 {
        panic!(
            "Unsupported bit depth {}, this example only supports 8.",
            decoder.get_bit_depth()
        );
    }
    let (w1, h1) = (decoder.get_width(), decoder.get_height());
    let dst_dims: Vec<_> = args[2].split("x").map(|s| s.parse().unwrap()).collect();
    let (w2, h2) = (dst_dims[0], dst_dims[1]);
    let mut resizer = resize::new(w1, h1, w2, h2, Gray8, Triangle);
    let mut dst = vec![0; w2 * h2];

    let mut outfh: Box<dyn io::Write> = if args[3] == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(&args[3]).unwrap())
    };
    let mut encoder = y4m::encode(w2, h2, decoder.get_framerate())
        .with_colorspace(y4m::Colorspace::Cmono)
        .write_header(&mut outfh)
        .unwrap();

    loop {
        match decoder.read_frame() {
            Ok(frame) => {
                resizer.resize(frame.get_y_plane(), &mut dst);
                let out_frame = y4m::Frame::new([&dst, &[], &[]], None);
                if encoder.write_frame(&out_frame).is_err() {
                    break;
                }
            }
            _ => break,
        }
    }
}

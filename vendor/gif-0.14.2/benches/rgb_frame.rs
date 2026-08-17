use std::{fs, io};

use criterion::{Criterion, Throughput};
use gif::{Encoder, Frame, Repeat};

const DIR: &str = "benches/samples";

fn main() {
    let mut c = Criterion::default().configure_from_args();
    let mut group = c.benchmark_group("rgb_frame");

    let dir = fs::read_dir(DIR).expect("Cant'r read dir:\n{}");

    for path in dir {
        let path = path.expect("Can't read path:\n{}").path();
        if path.extension().unwrap() != "png" {
            continue;
        }

        let mut reader = {
            let input = fs::File::open(&path).unwrap();
            let decoder = png::Decoder::new(io::BufReader::new(input));
            decoder.read_info().unwrap()
        };

        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();

        let (w, h, size) = {
            // could use try_into().unwrap() but probably no need
            (info.width as u16, info.height as u16, info.buffer_size())
        };

        //size might have to be adjusted for large images
        group
            .sample_size(50)
            .throughput(Throughput::Bytes(size as u64))
            .bench_function(
                path.file_name().unwrap().to_str().unwrap(),
                |b| match info.color_type {
                    png::ColorType::Rgb => b.iter(|| Frame::from_rgb_speed(w, h, &buf[..size], 30)),
                    png::ColorType::Rgba => {
                        b.iter(|| Frame::from_rgba_speed(w, h, &mut buf[..size], 30))
                    }
                    c => {
                        println!("Image has wrong color type: {c:?}");
                    }
                },
            );

        // actually write the image as a singe frame gif... while MSE can be used
        // for quality check, it might not be as good as visual inspection
        let mut encoder = {
            let output = fs::File::create(path.with_extension("gif")).unwrap();
            Encoder::new(output, w, h, &[]).unwrap()
        };
        encoder.set_repeat(Repeat::Finite(0)).unwrap();

        let frame = match info.color_type {
            png::ColorType::Rgb => Frame::from_rgb(w, h, &buf[..size]),
            png::ColorType::Rgba => Frame::from_rgba(w, h, &mut buf[..size]),
            _ => continue,
        };

        encoder.write_frame(&frame).unwrap();
    }
    group.finish();
    c.final_summary();
}

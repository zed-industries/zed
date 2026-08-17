
extern crate image as png;
use std::cmp::Ordering;

extern crate exr;

/// Extract all exr pixel information into pngs.
/// Writes each channel of each mip map of each layer as one grayscale png.
/// May appear black for single-color images.
pub fn main() {
    use exr::prelude::*;

    let path = "mip_maps.exr";
    let start_time = ::std::time::Instant::now();

    // load the exr file from disk with multi-core decompression
    let image = read_all_data_from_file(path)
        .expect("run example `5c_write_mip_maps` to generate this image file");

    // warning: highly unscientific benchmarks ahead!
    println!("\nloaded file in {:?}s", start_time.elapsed().as_secs_f32());
    let _ = std::fs::create_dir_all("pngs/");
    println!("writing images...");

    for (layer_index, layer) in image.layer_data.iter().enumerate() {
        let layer_name = layer.attributes.layer_name.as_ref()
            .map_or(String::from("1"), Text::to_string);

        for channel in &layer.channel_data.list {
            for (level, level_size) in layer.levels_with_resolution(&channel.sample_data) {
                let data : Vec<f32> = level.values_as_f32().collect();

                save_f32_image_as_png(&data, level_size, format!(
                    "pngs/{} ({}) {}.{}x{}.png",
                    layer_index, layer_name, channel.name,
                    level_size.width(), level_size.height(),
                ))
            }
        }
    }

    /// Save raw float data to a PNG file, doing automatic brightness adjustments per channel
    fn save_f32_image_as_png(data: &[f32], size: Vec2<usize>, name: String) {
        let mut png_buffer = png::GrayImage::new(size.width() as u32, size.height() as u32);
        let mut sorted = Vec::from(data);
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));

        // percentile normalization
        let max = sorted[7 * sorted.len() / 8];
        let min = sorted[1 * sorted.len() / 8];

        // primitive tone mapping
        let tone = |v: f32| (v - 0.5).tanh() * 0.5 + 0.5;
        let max_toned = tone(*sorted.last().unwrap());
        let min_toned = tone(*sorted.first().unwrap());

        // for each pixel, tone map the value
        for (x, y, pixel) in png_buffer.enumerate_pixels_mut() {
            let v = data[y as usize * size.0 + x as usize];
            let v = (v - min) / (max - min);
            let v = tone(v);

            let v = (v - min_toned) / (max_toned - min_toned);

            // TODO does the `image` crate expect gamma corrected data?
            *pixel = png::Luma([(v.max(0.0).min(1.0) * 255.0) as u8]);
        }

        png_buffer.save(&name).unwrap();
    }

    println!("extracted all layers to folder `./pngs/*.png`");
}


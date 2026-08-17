
// exr imports
extern crate exr;

/// Read an rgba image, increase the exposure, and then write it back.
/// Uses multi-core compression where appropriate.
///
/// All non-rgba channels and all layers except the first rgba layers will not be present in the new file.
fn main() {
    use exr::prelude::*;

    /// This is an example of a custom image type.
    /// You use your own image struct here.
    // This struct trades sub-optimal memory-efficiency for clarity,
    // because this is an example, and does not have to be perfectly efficient.
    #[derive(Debug, PartialEq)]
    struct CustomPixels { lines: Vec<Vec<RgbaF32Pixel>> }
    type RgbaF32Pixel = (f32, f32, f32, f32);

    // read the image from a file
    let mut image = read().no_deep_data()
        .largest_resolution_level()
        .rgba_channels(
            // create our custom image based on the file info
            |resolution, _channels| -> CustomPixels {
                let default_rgba_pixel = (0.0, 0.0, 0.0, 0.0);
                let default_line = vec![default_rgba_pixel; resolution.width()];
                let lines = vec![default_line; resolution.height()];
                CustomPixels { lines }
            },

            // request pixels with red, green, blue, and optionally and alpha values.
            // transfer each pixel from the file to our image
            |image, position, (r,g,b,a): RgbaF32Pixel| {

                // insert the values into our custom image
                image.lines[position.y()][position.x()] = (r,g,b,a);
            }
        )
        .first_valid_layer()
        .all_attributes()
        .from_file("generated_rgba.exr")
        .expect("run the `1_write_rgba` example to generate the required file");

    let exposure_multiplier = 2.0;

    {   // increase exposure of all pixels
        for line in &mut image.layer_data.channel_data.pixels.lines {
            for (r,g,b,_) in line {
                // you should probably check the color space and white points
                // for high quality color adjustments
                *r *= exposure_multiplier;
                *g *= exposure_multiplier;
                *b *= exposure_multiplier;
            }
        }

        // also update meta data after modifying the image
        if let Some(exposure) = &mut image.layer_data.attributes.exposure {
            println!("increased exposure from {}s to {}s", exposure, *exposure * exposure_multiplier);
            *exposure *= exposure_multiplier;
        }
    }

    // enable writing our custom pixel storage to a file
    // FIXME this should be passed as a closure to the `write_with(|x| y)` call
    impl GetPixel for CustomPixels {
        type Pixel = RgbaF32Pixel;
        fn get_pixel(&self, position: Vec2<usize>) -> Self::Pixel {
            self.lines[position.y()][position.x()]
        }
    }

    // write the image to a file
    image
        .write().to_file("rgba_exposure_adjusted.exr")
        .unwrap();

    println!("created file rgba_exposure_adjusted.exr");
}
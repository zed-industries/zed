
// exr imports
extern crate exr;

/// Read an image and print information about the image into the console.
/// This example shows how to read an image with multiple layers and specific channels.
/// This example does not include resolution levels (mipmaps or ripmaps).
fn main() {
    use exr::prelude::*;

    let image = read().no_deep_data()
        .largest_resolution_level()

        .specific_channels()
        .optional("A", f16::ONE)
        .required("Y") // TODO also accept a closure with a detailed selection mechanism
        .optional("right.Y", 0.0)
        .collect_pixels(
            |resolution, (a_channel, y_channel, y_right_channel)| {
                println!("image contains alpha channel? {}", a_channel.is_some());
                println!("image contains stereoscopic luma channel? {}", y_right_channel.is_some());
                println!("the type of luma samples is {:?}", y_channel.sample_type);

                vec![vec![(f16::ZERO, 0.0, 0.0); resolution.width()]; resolution.height()]
            },

            // all samples will be converted to f32 (you can also use the enum `Sample` instead of `f32` here to retain the original data type from the file)
            |vec, position, (a,y,yr): (f16, f32, f32)| {
                vec[position.y()][position.x()] = (a, y, yr)
            }
        )

        .all_layers()
        .all_attributes()
        .on_progress(|progress| println!("progress: {:.1}", progress*100.0))
        .from_file("custom_channels.exr")
        .expect("run example `4_write_custom_fixed_channels` to generate this image file");

    // output a random color of each channel of each layer
    for layer in &image.layer_data {
        let (alpha, luma, luma_right) = layer.channel_data.pixels.first().unwrap().first().unwrap();

        println!(
            "top left color of layer `{}`: (a, y, yr) = {:?}",
            layer.attributes.layer_name.clone().unwrap_or_default(),
            (alpha.to_f32(), luma, luma_right)
        )
    }
}
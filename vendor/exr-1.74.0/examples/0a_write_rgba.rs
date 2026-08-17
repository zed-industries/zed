extern crate exr;


/// `exr` offers a few simplified functions for the most basic use cases.
/// `write_rgb_f32_file` is a such a function, which writes a plain rgba exr file.
///
/// To write your image data, you need to specify how to retrieve a single pixel from it.
/// The closure may capture variables or generate data on the fly.
fn main() {
    use exr::prelude::*;

    // write a file, with 32-bit float precision per channel
    write_rgba_file(

        // this accepts paths or &str
        "minimal_rgba.exr",

        // image resolution is 2k
        2048, 2048,

        // generate (or lookup in your own image)
        // an f32 rgb color for each of the 2048x2048 pixels
        // (you could also create f16 values here to save disk space)
        |x,y| {
            (
                x as f32 / 2048.0, // red
                y as f32 / 2048.0, // green
                1.0 - (y as f32 / 2048.0), // blue
                1.0 // alpha
            )
        }

    ).unwrap();

    println!("created file minimal_rgb.exr");
}
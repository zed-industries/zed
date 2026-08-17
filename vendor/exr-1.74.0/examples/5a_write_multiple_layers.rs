
extern crate smallvec;
extern crate rand;
extern crate half;


// exr imports
extern crate exr;

/// Writes multiple layers into one exr file
/// Note: this may not be supported by legacy software
fn main() {
    use exr::prelude::*;
    let size = Vec2(512, 512);


    let layer1 = Layer::new(
        size,
        LayerAttributes::named("teal rgb"),
        Encoding::FAST_LOSSLESS,
        SpecificChannels::rgb(|_pos| (0_f32, 0.4_f32, 0.4_f32)),
    );

    let layer2 = Layer::new(
        size,
        LayerAttributes::named("orange rgba"),
        Encoding::FAST_LOSSLESS,
        SpecificChannels::rgba(|_pos| (0.8_f32, 0.5_f32, 0.1_f32, 1.0_f32)),
    );

    // define the visible area of the canvas
    let attributes = ImageAttributes::new(
        // the pixel section that should be shown
        IntegerBounds::from_dimensions(size)
    );

    let image = Image::empty(attributes)
        .with_layer(layer1) // add an rgb layer of type `SpecificChannels<ClosureA>`
        .with_layer(layer2); // add an rgba layer of different type, `SpecificChannels<ClosureB>`, not possible with a vector

    println!("writing image...");
    image.write().to_file("layers.exr").unwrap();

    println!("created file layers.exr");
}
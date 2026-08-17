
#[macro_use]
extern crate smallvec;
extern crate rand;
extern crate half;


// exr imports
extern crate exr;

// TODO create a dedicated reader and writer for this scenario

/// Generate an image with channel groups and write it to a file.
/// Some legacy software may group layers that contain a `.` in the layer name.
///
/// Note: This is an OpenEXR legacy strategy. OpenEXR supports layers natively since 2013.
/// Use the natively supported exrs `Layer` types instead, if possible.
///
fn main() {
    use exr::prelude::*;
    // TODO simplify handling these types of layers using read() and write()

    let size = Vec2(512, 512);

    let create_channel = |name: &str| -> AnyChannel<FlatSamples> {
        let color: f16 = f16::from_bits(rand::random::<u16>());

        AnyChannel::new(
            name,
            FlatSamples::F16(vec![color; size.area() ])
        )
    };


    // The channels have the following structure:
    //
    // - Object
    //     - Red
    //     - Green
    //     - Blue
    //     - Alpha

    // - Background
    //     - Red
    //     - Green
    //     - Blue

    let foreground_r = create_channel("Object.R");
    let foreground_g = create_channel("Object.G");
    let foreground_b = create_channel("Object.B");
    let foreground_a = create_channel("Object.A");

    let background_r = create_channel("Background.R");
    let background_g = create_channel("Background.G");
    let background_b = create_channel("Background.B");

    let layer = Layer::new(
        size,
        LayerAttributes::named("test-image"),
        Encoding::FAST_LOSSLESS,
        AnyChannels::sort(smallvec![ // the order does not actually matter
            foreground_r, foreground_g, foreground_b, foreground_a,
            background_r, background_g, background_b
        ]),
    );

    let image = Image::from_layer(layer);

    println!("writing image {:#?}", image);
    image.write().to_file("groups.exr").unwrap();

    println!("created file groups.exr");
}
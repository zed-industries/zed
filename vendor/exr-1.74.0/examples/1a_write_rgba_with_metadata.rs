
// exr imports
extern crate exr;

/// Write an rgba exr file, generating the pixel values on the fly.
/// This streams the generated pixel directly to the file,
/// never allocating the actual total pixel memory of the image.
fn main() {
    use exr::prelude::*;
    use exr::meta::attribute::*;

    // this function can generate a color for any pixel
    let generate_pixels = |position: Vec2<usize>| (
        position.x() as f32 / 2048.0, // red
        position.y() as f32 / 2048.0, // green
        1.0 - (position.y() as f32 / 2048.0), // blue
        1.0 // alpha
    );

    let mut layer_attributes = LayerAttributes::named("generated rgba main layer");
    layer_attributes.comments = Some(Text::from("This image was generated as part of an example"));
    layer_attributes.owner = Some(Text::from("The holy lambda function"));
    layer_attributes.software_name = Some(Text::from("EXRS Project"));
    layer_attributes.exposure = Some(1.0);
    layer_attributes.focus = Some(12.4);
    layer_attributes.frames_per_second = Some((60, 1));
    layer_attributes.other.insert(
        Text::from("Layer Purpose (Custom Layer Attribute)"),
        AttributeValue::Text(Text::from("This layer contains the rgb pixel data"))
    );

    let layer = Layer::new(
        (2*2048, 2*2048),
        layer_attributes,
        Encoding::SMALL_FAST_LOSSLESS, // use fast but lossy compression

        SpecificChannels::rgba(generate_pixels)
    );

    // crop away black and transparent pixels from the border, if any
    let layer = layer
        .crop_where_eq((0.0, 0.0, 0.0, 0.0))
        .or_crop_to_1x1_if_empty();

    let mut image = Image::from_layer(layer);
    image.attributes.pixel_aspect = 1.0;

    image.attributes.time_code = Some(TimeCode {
        hours: 0,
        minutes: 1,
        seconds: 59,
        frame: 29,
        ..TimeCode::default()
    });

    image.attributes.other.insert(
        Text::from("Mice Count (Custom Image Attribute)"),
        AttributeValue::I32(23333)
    );

    // write it to a file with all cores in parallel
    image.write().to_file("generated_rgba_with_meta.exr").unwrap();
    println!("created file generated_rgba_with_meta.exr");
}
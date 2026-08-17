
#[macro_use]
extern crate smallvec;
extern crate rand;
extern crate half;

use rand::Rng;

// exr imports
extern crate exr;


/// Generate a noisy image and write it to a file,
/// also attaching some meta data.
fn main() {
    use exr::prelude::*;

    fn generate_f16_vector(size: Vec2<usize>) -> Vec<f16> {
        let mut values = vec![ f16::from_f32(0.5); size.area() ];

        for _ in 0..(1024*1024/3)/4 {
            let index = rand::thread_rng().gen_range(0 .. values.len());
            let value = 1.0 / rand::random::<f32>() - 1.0;
            let value = if !value.is_normal() || value > 1000.0 { 1000.0 } else { value };
            values[index] = f16::from_f32(value);
        }

        values
    }

    let size = (1024, 512);

    let r = AnyChannel::new(
        "R", FlatSamples::F16(generate_f16_vector(size.into()))
    );

    let g = AnyChannel::new(
        "G", FlatSamples::F16(generate_f16_vector(size.into()))
    );

    let b = AnyChannel::new(
        "B", FlatSamples::F32(generate_f16_vector(size.into()).into_iter().map(f16::to_f32).collect())
    );

    let a = AnyChannel::new(
        "A", FlatSamples::F32(generate_f16_vector(size.into()).into_iter().map(f16::to_f32).collect())
    );

    let mut layer_attributes = LayerAttributes::named("test-image");
    layer_attributes.owner = Some(Text::from("It's you!"));
    layer_attributes.comments = Some(Text::from("This image was procedurally generated"));

    let layer = Layer::new(
        size,
        layer_attributes,
        Encoding::default(),
        AnyChannels::sort(smallvec![ r, g, b, a ]),
    );

    // crop away transparent pixels from the border
    let layer = layer

        // channel order is (a,b,g,r), as channels are already sorted
        .crop_where(|samples| samples[0].is_zero())

        // throw error if the image is 100% transparent pixels and should be removed
        .or_none_if_empty().expect("image is empty and cannot be cropped");

    let image = Image::from_layer(layer);

    println!("writing image {:#?}", image);

    image.write()
        .on_progress(|progress| println!("progress: {:.1}", progress*100.0))
        .to_file("noisy.exr").unwrap();

    println!("created file noisy.exr");
}
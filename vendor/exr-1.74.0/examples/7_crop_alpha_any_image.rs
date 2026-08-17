
extern crate image as png;

extern crate exr;

/// Read an arbitrary image, crop away transparent pixels,
/// then write the cropped result to another file.
pub fn main() {
    use exr::prelude::*;

    let path = "tests/images/valid/custom/oh crop.exr";

    // loads any image (excluding deep data)
    let image: FlatImage = read_all_flat_layers_from_file(path)
        .expect("this file exists in the exrs repository. download that?");

    // construct a cropped image
    let image = Image {
        attributes: image.attributes,

        // crop each layer
        layer_data: image.layer_data.into_iter().map(|layer|{
            println!("cropping layer {:#?}", layer);

            // find the alpha channel of the layer
            let alpha_channel_index = layer.channel_data.list.iter()
                .position(|channel| channel.name.eq_case_insensitive("A"));

            // if has alpha, crop it where alpha is zero
            if let Some(alpha_channel_index) = alpha_channel_index {
                layer.crop_where(|pixel: FlatSamplesPixel| pixel[alpha_channel_index].is_zero())
                    .or_crop_to_1x1_if_empty() // do not remove empty layers from image, because it could result in an image without content
                    .reallocate_cropped() // actually perform the crop operation
            }
            else {
                // return the original layer, as no alpha channel can be used for cropping
                layer
            }

        }).collect::<Layers<_>>(),
    };

    image.write().to_file("cropped.exr").unwrap();
    println!("cropped file to cropped.exr");
}


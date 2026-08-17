
extern crate rand;
extern crate half;

use std::io::{BufReader};
use std::fs::File;
use exr::block::reader::ChunksReader;

// exr imports
extern crate exr;


/// Collects the average pixel value for each channel.
/// Does not load the whole image into memory at once: only processes the image block by block.
/// On my machine, this program analyzes a 3GB file while only allocating 1.1MB.
fn main() {
    use exr::prelude::*;

    let file = BufReader::new(
        File::open("3GB.exr")
            .expect("run example `7_write_raw_blocks` to generate this image file")
    );


    // -- the following structs will hold the collected data from the image --

    /// Collect averages for each layer in the image
    #[derive(Debug)]
    struct Layer {
        #[allow(unused)] // note: is used in Debug impl
        layer_name: Option<Text>,

        data_window: IntegerBounds,

        /// Collect one average float per channel in the layer
        channels: Vec<Channel>,
    }

    /// A single channel in the layer, holds a single average value
    #[derive(Debug)]
    struct Channel {
        #[allow(unused)] // note: is used in Debug impl
        channel_name: Text,

        sample_type: SampleType, // f32, u32, or f16
        average: f32,
    }

    let start_time = ::std::time::Instant::now();


    // -- read the file, summing up the average pixel values --

    // start reading the file, extracting the meta data of the image
    let reader = exr::block::read(file, true).unwrap();

    // print progress only if it advances more than 1%
    let mut current_progress_percentage = 0;

    // create the empty data structure that will collect the analyzed results,
    // based on the extracted meta data of the file
    let mut averages = reader.headers().iter()
        // create a layer for each header in the file
        .map(|header| Layer {
            layer_name: header.own_attributes.layer_name.clone(),
            data_window: header.data_window(),

            // create a averaging channel for each channel in the file
            channels: header.channels.list.iter()
                .map(|channel| Channel {
                    channel_name: channel.name.clone(),
                    sample_type: channel.sample_type,
                    average: 0.0
                })
                .collect()
        })
        .collect::<Vec<_>>();

    // create a reader that loads only relevant chunks from the file, and also prints something on progress
    let reader = reader

        // do not worry about multi-resolution levels or deep data
        .filter_chunks(true, |meta_data, tile, block| {
            let header = &meta_data.headers[block.layer];
            !header.deep && tile.is_largest_resolution_level()
        }).unwrap()

        .on_progress(|progress|{
            let new_progress = (progress * 100.0) as usize;
            if new_progress != current_progress_percentage {
                current_progress_percentage = new_progress;
                println!("progress: {}%", current_progress_percentage)
            }
        });

    // read all pixel blocks from the image, decompressing in parallel
    reader.decompress_parallel(true, |meta_data, block|{
        let header = &meta_data.headers[block.index.layer];

        // collect all pixel values from the pixel block
        for line in block.lines(&header.channels) {
            let layer = &mut averages[line.location.layer];
            let channel = &mut layer.channels[line.location.channel];
            let channel_sample_count = layer.data_window.size.area() as f32;

            // now sum the average based on the values in this line section of pixels
            match channel.sample_type {
                SampleType::F16 => for value in line.read_samples::<f16>() {
                    channel.average += value?.to_f32() / channel_sample_count;
                },

                SampleType::F32 => for value in line.read_samples::<f32>() {
                    channel.average += value? / channel_sample_count;
                },

                SampleType::U32 => for value in line.read_samples::<u32>() {
                    channel.average += (value? as f32) / channel_sample_count;
                },
            }
        }

        Ok(())
    }).unwrap();

    println!("average values: {:#?}", averages);

    // warning: highly unscientific benchmarks ahead!
    println!("\nprocessed file in {:?}s", start_time.elapsed().as_secs_f32());
}
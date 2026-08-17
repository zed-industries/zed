//! Fuzzy testing.
//! Tries to discover panics with random bytes.
//! This test is expensive and therefore marked with `#[ignore]`. To run this test, use `cargo test -- --ignored`.

use std::panic::{catch_unwind};
use rand::rngs::{StdRng};
use rand::{Rng};

extern crate exr;
use exr::prelude::*;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write, Cursor};
use exr::image::read::read_first_rgba_layer_from_file;
use exr::image::pixel_vec::PixelVec;

fn exr_files(path: &'static str, filter: bool) -> impl Iterator<Item=PathBuf> {
    walkdir::WalkDir::new(path).into_iter().map(std::result::Result::unwrap)
        .filter(|entry| entry.path().is_file())

        .filter(move |entry| !filter || entry.path().extension() == Some(OsStr::new("exr")))
        .map(walkdir::DirEntry::into_path)
}


/// Just don't panic.
#[test]
pub fn fuzzed(){
    for ref file in exr_files("tests/images/fuzzed", false) {
        let _ = read().no_deep_data().largest_resolution_level().all_channels()
            .first_valid_layer().all_attributes().pedantic().from_file(file);

        let _ = read().no_deep_data().all_resolution_levels().all_channels()
            .all_layers().all_attributes().pedantic().from_file(file);
    }
}

/// Require an error but no panic.
#[test]
pub fn damaged(){
    let mut passed = true;

    for ref file in exr_files("tests/images/invalid", false) {
        let result = catch_unwind(move || {
            let _meta_data = MetaData::read_from_file(file, false)?;

            {
                let _minimal = read().no_deep_data()
                    .largest_resolution_level()
                    .rgba_channels(
                        |_size, _channels| (),
                        |_: &mut (), _position: Vec2<usize>, _pixel: (Sample, Sample, Sample, Sample)| {}
                    )
                    .first_valid_layer().all_attributes()
                    .from_file(&file)?;
            }

            {
                let _minimal = read().no_deep_data()
                    .largest_resolution_level() // TODO all levels
                    .rgba_channels(
                        |_size, _channels| (),
                        |_: &mut (), _position: Vec2<usize>, _pixel: (Sample, Sample, Sample, Sample)| {}
                    )
                    .all_layers().all_attributes()
                    .pedantic()
                    .from_file(&file)?;
            }

            {
                let _rgba = read_first_rgba_layer_from_file(
                    file,
                    PixelVec::<(Sample, Sample, Sample, Sample)>::constructor,
                    PixelVec::set_pixel
                )?;
            }

            {
                let _full = read_all_data_from_file(file)?;
            }

            Ok(())
        });

        // this should not panic, only err:
        passed = passed && match result {
            Ok(Err(Error::Invalid(message))) => {
                println!("✓ Recognized as invalid ({}): {:?}", message, file);
                true
            },

            Ok(Err(Error::NotSupported(message))) => {
                println!("- Unsupported ({}): {:?}", message, file);
                true
            },

            Ok(Err(Error::Io(error))) => {
                println!("✗ Unexpected IO Error: {:?}, {:?}", file, error);
                false
            },

            Err(_) => {
                println!("✗ Not recognized as invalid: {:?}", file);
                false
            },

            Ok(Ok(_)) => {
                let meta_data = MetaData::read_from_file(file, true);

                if let Err(error) = meta_data {
                    println!("✓ Recognized as invalid when pedantic ({}): {:?}", error, file);
                    true
                }
                else {
                    println!("✗ Oh no, there is nothing wrong with: {:#?}", file);
                    false
                }
            },

            _ => unreachable!(),
        };
    }

    assert!(passed, "A damaged file was not handled correctly");
}

#[test]
#[ignore]
pub fn fuzz(){
    println!("started fuzzing");
    let files: Vec<PathBuf> = exr_files("tests/images", true).collect();

    let seed = [92,1,0,130,211,8,21,70,74,4,9,5,0,23,0,3,20,25,6,5,229,30,0,34,218,0,40,7,5,2,7,0,];
    let mut random: StdRng = rand::SeedableRng::from_seed(seed);

    let mut records = File::create("tests/images/fuzzed/list.txt").unwrap();
    records.write_all(format!("seed = {:?}", seed).as_bytes()).unwrap();

    let start_index = 0; // default is 0. increase this integer for debugging a specific fuzz case
    for fuzz_index in 0 .. 1024_u64 * 2048 * 4 {

        let file_1_name = &files[random.gen_range(0 .. files.len())];
        let mutation_point = random.gen::<f32>().powi(3);
        let mutation = random.gen::<u8>();

        if fuzz_index >= start_index {
            let mut file = std::fs::read(file_1_name).unwrap();
            let index = ((mutation_point * file.len() as f32) as usize + 4) % file.len();
            file[index] = mutation;

            let file = file.as_slice();
            let result = catch_unwind(move || {
                let read_all_data = read().no_deep_data()
                    .all_resolution_levels().all_channels().all_layers().all_attributes();

                match read_all_data.from_buffered(Cursor::new(file)) {
                    Err(Error::Invalid(error)) => println!("✓ No Panic. [{}]: Invalid: {}.", fuzz_index, error),
                    Err(Error::NotSupported(error)) => println!("- No Panic. [{}]: Unsupported: {}.", fuzz_index, error),
                    _ => {},
                }
            });

            if let Err(_) = result {
                records.write_all(fuzz_index.to_string().as_bytes()).unwrap();
                records.flush().unwrap();

                let seed = seed.iter().map(|num| num.to_string()).collect::<Vec<String>>().join("-");
                let mut saved = File::create(format!("tests/images/fuzzed/fuzz_{}_{}.exr", fuzz_index, seed)).unwrap();
                saved.write_all(file).unwrap();

                println!("✗ PANIC! [{}]", fuzz_index);
            }
        }
    }
}

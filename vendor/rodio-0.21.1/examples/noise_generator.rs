//! Noise generator example demonstrating practical applications like dithering.
//! Use the "noise" feature to enable the noise generator sources.

use std::{error::Error, thread::sleep, time::Duration};

use rodio::{
    source::noise::{
        Blue, Brownian, Pink, Velvet, Violet, WhiteGaussian, WhiteTriangular, WhiteUniform,
    },
    MixerDeviceSink, Sample, Source,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let sample_rate = stream_handle.config().sample_rate();

    play_noise(
        &stream_handle,
        WhiteUniform::new(sample_rate),
        "White Uniform",
        "Testing equipment linearly, masking sounds",
    );

    play_noise(
        &stream_handle,
        WhiteGaussian::new(sample_rate),
        "White Gaussian",
        "Scientific modeling, natural processes",
    );

    play_noise(
        &stream_handle,
        WhiteTriangular::new(sample_rate),
        "White Triangular",
        "High-quality audio dithering (TPDF)",
    );

    play_noise(
        &stream_handle,
        Pink::new(sample_rate),
        "Pink",
        "Speaker testing, pleasant background sounds",
    );

    play_noise(
        &stream_handle,
        Blue::new(sample_rate),
        "Blue",
        "High-frequency emphasis, bright effects",
    );

    play_noise(
        &stream_handle,
        Violet::new(sample_rate),
        "Violet",
        "Very bright, sharp, high-frequency testing",
    );

    play_noise(
        &stream_handle,
        Brownian::new(sample_rate),
        "Brownian",
        "Muffled/distant effects, deep rumbles",
    );

    play_noise(
        &stream_handle,
        Velvet::new(sample_rate),
        "Velvet",
        "Sparse impulse generation for audio processing",
    );

    Ok(())
}

/// Helper function to play a noise type with description
fn play_noise<S>(stream_handle: &MixerDeviceSink, source: S, name: &str, description: &str)
where
    S: Source<Item = Sample> + Send + 'static,
{
    println!("{} Noise", name);
    println!("   Application: {}", description);

    stream_handle.mixer().add(
        source
            .amplify(0.12)
            .take_duration(Duration::from_millis(1500)),
    );

    sleep(Duration::from_millis(2000));
}

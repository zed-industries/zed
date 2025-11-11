use rodio::{nz, source::UniformSourceIterator, wav_to_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("airconditioning.wav")?;
    let decoder = rodio::Decoder::try_from(file)?;
    let resampled = UniformSourceIterator::new(decoder, nz!(1), nz!(16_000));

    let mut denoised = denoise::Denoiser::try_new(resampled)?;
    wav_to_file(&mut denoised, "denoised.wav")?;
    Ok(())
}

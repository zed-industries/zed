Real time streaming audio denoising using a [Dual-Signal Transformation LSTM Network for Real-Time Noise Suppression](https://arxiv.org/abs/2005.07551).

Trivial to build as it uses the native rust Candle crate for inference. Easy to integrate into any Rodio pipeline.

```rust
    # use rodio::{nz, source::UniformSourceIterator, wav_to_file};
    let file = std::fs::File::open("clips_airconditioning.wav")?;
    let decoder = rodio::Decoder::try_from(file)?;
    let resampled = UniformSourceIterator::new(decoder, nz!(1), nz!(16_000));

    let mut denoised = denoise::Denoiser::try_new(resampled)?;
    wav_to_file(&mut denoised, "denoised.wav")?;
    Result::Ok<(), Box<dyn std::error::Error>>
```

## Acknowledgements & License

The trained models in this repo are optimized versions of the models in the [breizhn/DTLN](https://github.com/breizhn/DTLN?tab=readme-ov-file#model-conversion-and-real-time-processing-with-onnx). These are licensed under MIT.

The FFT code was adapted from Datadog's [dtln-rs Repo](https://github.com/DataDog/dtln-rs/tree/main) also licensed under MIT.

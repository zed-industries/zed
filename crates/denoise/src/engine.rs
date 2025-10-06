/// use something like https://netron.app/ to inspect the models and understand
/// the flow
use std::collections::HashMap;

use candle_core::{Device, IndexOp, Tensor};
use candle_onnx::onnx::ModelProto;
use candle_onnx::prost::Message;
use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;

pub struct Engine {
    spectral_model: ModelProto,
    signal_model: ModelProto,

    fft_planner: RealFftPlanner<f32>,
    fft_scratch: Vec<Complex<f32>>,
    spectrum: [Complex<f32>; FFT_OUT_SIZE],
    signal: [f32; BLOCK_LEN],

    in_magnitude: [f32; FFT_OUT_SIZE],
    in_phase: [f32; FFT_OUT_SIZE],

    spectral_memory: Tensor,
    signal_memory: Tensor,

    in_buffer: [f32; BLOCK_LEN],
    out_buffer: [f32; BLOCK_LEN],
}

// 32 ms @ 16khz per DTLN docs: https://github.com/breizhn/DTLN
pub const BLOCK_LEN: usize = 512;
// 8 ms @ 16khz per DTLN docs.
pub const BLOCK_SHIFT: usize = 128;
pub const FFT_OUT_SIZE: usize = BLOCK_LEN / 2 + 1;

impl Engine {
    pub fn new() -> Self {
        let mut fft_planner = RealFftPlanner::new();
        let fft_planned = fft_planner.plan_fft_forward(BLOCK_LEN);
        let scratch_len = fft_planned.get_scratch_len();
        Self {
            // Models are 1.5MB and 2.5MB respectively. Its worth the binary
            // size increase not to have to distribute the models separately.
            spectral_model: ModelProto::decode(
                include_bytes!("../models/model_1_converted_simplified.onnx").as_slice(),
            )
            .expect("The model should decode"),
            signal_model: ModelProto::decode(
                include_bytes!("../models/model_2_converted_simplified.onnx").as_slice(),
            )
            .expect("The model should decode"),
            fft_planner,
            fft_scratch: vec![Complex::ZERO; scratch_len],
            spectrum: [Complex::ZERO; FFT_OUT_SIZE],
            signal: [0f32; BLOCK_LEN],

            in_magnitude: [0f32; FFT_OUT_SIZE],
            in_phase: [0f32; FFT_OUT_SIZE],

            spectral_memory: Tensor::from_slice::<_, f32>(
                &[0f32; 512],
                (1, 2, BLOCK_SHIFT, 2),
                &Device::Cpu,
            )
            .expect("Tensor has the correct dimensions"),
            signal_memory: Tensor::from_slice::<_, f32>(
                &[0f32; 512],
                (1, 2, BLOCK_SHIFT, 2),
                &Device::Cpu,
            )
            .expect("Tensor has the correct dimensions"),
            out_buffer: [0f32; BLOCK_LEN],
            in_buffer: [0f32; BLOCK_LEN],
        }
    }

    /// Add a clunk of samples and get the denoised chunk 4 feeds later
    pub fn feed(&mut self, samples: &[f32]) -> [f32; BLOCK_SHIFT] {
        /// The name of the output node of the onnx network
        /// [Dual-Signal Transformation LSTM Network for Real-Time Noise Suppression](https://arxiv.org/abs/2005.07551).
        const MEMORY_OUTPUT: &'static str = "Identity_1";

        debug_assert_eq!(samples.len(), BLOCK_SHIFT);

        // place new samples at the end of the `in_buffer`
        self.in_buffer.copy_within(BLOCK_SHIFT.., 0);
        self.in_buffer[(BLOCK_LEN - BLOCK_SHIFT)..].copy_from_slice(&samples);

        // run inference
        let inputs = self.spectral_inputs();
        let mut spectral_outputs = candle_onnx::simple_eval(&self.spectral_model, inputs)
            .expect("The embedded file must be valid");
        self.spectral_memory = spectral_outputs
            .remove(MEMORY_OUTPUT)
            .expect("The model has an output named Identity_1");
        let inputs = self.signal_inputs(spectral_outputs);
        let mut signal_outputs = candle_onnx::simple_eval(&self.signal_model, inputs)
            .expect("The embedded file must be valid");
        self.signal_memory = signal_outputs
            .remove(MEMORY_OUTPUT)
            .expect("The model has an output named Identity_1");
        let model_output = model_outputs(signal_outputs);

        // place processed samples at the start of the `out_buffer`
        // shift the rest left, fill the end with zeros. Zeros are needed as
        // the out buffer is part of the input of the network
        self.out_buffer.copy_within(BLOCK_SHIFT.., 0);
        self.out_buffer[BLOCK_LEN - BLOCK_SHIFT..].fill(0f32);
        for (a, b) in self.out_buffer.iter_mut().zip(model_output) {
            *a += b;
        }

        // samples at the front of the `out_buffer` are now denoised
        self.out_buffer[..BLOCK_SHIFT]
            .try_into()
            .expect("len is correct")
    }

    fn spectral_inputs(&mut self) -> HashMap<String, Tensor> {
        // Prepare FFT input
        let fft = self.fft_planner.plan_fft_forward(BLOCK_LEN);

        // Perform real-to-complex FFT
        let mut fft_in = self.in_buffer;
        fft.process_with_scratch(&mut fft_in, &mut self.spectrum, &mut self.fft_scratch)
            .expect("The fft should run, there is enough scratch space");

        // Generate magnitude and phase
        for ((magnitude, phase), complex) in self
            .in_magnitude
            .iter_mut()
            .zip(self.in_phase.iter_mut())
            .zip(self.spectrum)
        {
            *magnitude = complex.norm();
            *phase = complex.arg();
        }

        const SPECTRUM_INPUT: &str = "input_2";
        const MEMORY_INPUT: &str = "input_3";
        let spectrum =
            Tensor::from_slice::<_, f32>(&self.in_magnitude, (1, 1, FFT_OUT_SIZE), &Device::Cpu)
                .expect("the in magnitude has enough elements to fill the Tensor");

        let inputs = HashMap::from([
            (SPECTRUM_INPUT.to_string(), spectrum),
            (MEMORY_INPUT.to_string(), self.spectral_memory.clone()),
        ]);
        inputs
    }

    fn signal_inputs(&mut self, outputs: HashMap<String, Tensor>) -> HashMap<String, Tensor> {
        let magnitude_weight = model_outputs(outputs);

        // Apply mask and reconstruct complex spectrum
        let mut spectrum = [Complex::I; FFT_OUT_SIZE];
        for i in 0..FFT_OUT_SIZE {
            let magnitude = self.in_magnitude[i] * magnitude_weight[i];
            let phase = self.in_phase[i];
            let real = magnitude * phase.cos();
            let imag = magnitude * phase.sin();
            spectrum[i] = Complex::new(real, imag);
        }

        // Handle DC component (i = 0)
        let magnitude = self.in_magnitude[0] * magnitude_weight[0];
        spectrum[0] = Complex::new(magnitude, 0.0);

        // Handle Nyquist component (i = N/2)
        let magnitude = self.in_magnitude[FFT_OUT_SIZE - 1] * magnitude_weight[FFT_OUT_SIZE - 1];
        spectrum[FFT_OUT_SIZE - 1] = Complex::new(magnitude, 0.0);

        // Perform complex-to-real IFFT
        let ifft = self.fft_planner.plan_fft_inverse(BLOCK_LEN);
        ifft.process_with_scratch(&mut spectrum, &mut self.signal, &mut self.fft_scratch)
            .expect("The fft should run, there is enough scratch space");

        // Normalize the IFFT output
        for real in &mut self.signal {
            *real /= BLOCK_LEN as f32;
        }

        const SIGNAL_INPUT: &str = "input_4";
        const SIGNAL_MEMORY: &str = "input_5";
        let signal_input =
            Tensor::from_slice::<_, f32>(&self.signal, (1, 1, BLOCK_LEN), &Device::Cpu).unwrap();

        HashMap::from([
            (SIGNAL_INPUT.to_string(), signal_input),
            (SIGNAL_MEMORY.to_string(), self.signal_memory.clone()),
        ])
    }
}

// Both models put their outputs in the same location
fn model_outputs(mut outputs: HashMap<String, Tensor>) -> Vec<f32> {
    const NON_MEMORY_OUTPUT: &str = "Identity";
    outputs
        .remove(NON_MEMORY_OUTPUT)
        .expect("The model has this output")
        .i((0, 0))
        .and_then(|tensor| tensor.to_vec1())
        .expect("The tensor has the correct dimensions")
}

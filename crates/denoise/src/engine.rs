/// use something like https://netron.app/ to inspect the models and understand
/// the flow
use std::collections::HashMap;

use candle_core::{Device, IndexOp, Tensor};
use candle_onnx::onnx::ModelProto;
use candle_onnx::prost::Message;
use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;

pub struct Engine {
    model1: ModelProto,
    model2: ModelProto,

    fft_planner: RealFftPlanner<f32>,
    fft_scratch: Vec<Complex<f32>>,
    spectrum: [Complex<f32>; FFT_OUT_SIZE],
    signal: [f32; BLOCK_LEN],

    in_mag: [f32; FFT_OUT_SIZE],
    in_phase: [f32; FFT_OUT_SIZE],

    shared_state_model1: Tensor,
    shared_state_model2: Tensor,

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
            model1: ModelProto::decode(
                include_bytes!("../models/model_1_converted_simplified.onnx").as_slice(),
            )
            .unwrap(),
            model2: ModelProto::decode(
                include_bytes!("../models/model_2_converted_simplified.onnx").as_slice(),
            )
            .unwrap(),
            fft_planner,
            fft_scratch: vec![Complex::ZERO; scratch_len],
            spectrum: [Complex::ZERO; FFT_OUT_SIZE],
            signal: [0f32; BLOCK_LEN],

            in_mag: [0f32; FFT_OUT_SIZE],
            in_phase: [0f32; FFT_OUT_SIZE],

            shared_state_model1: Tensor::from_slice::<_, f32>(
                &[0f32; 512],
                (1, 2, BLOCK_SHIFT, 2),
                &Device::Cpu,
            )
            .unwrap(),
            shared_state_model2: Tensor::from_slice::<_, f32>(
                &[0f32; 512],
                (1, 2, BLOCK_SHIFT, 2),
                &Device::Cpu,
            )
            .unwrap(),
            out_buffer: [0f32; BLOCK_LEN],
            in_buffer: [0f32; BLOCK_LEN],
        }
    }

    /// Add a clunk of samples and get the denoised chunk 4 feeds later
    pub fn feed(&mut self, samples: &[f32]) -> [f32; BLOCK_SHIFT] {
        debug_assert_eq!(samples.len(), BLOCK_SHIFT);

        // place new samples at the end of the `in_buffer`
        self.in_buffer.copy_within(BLOCK_SHIFT.., 0);
        self.in_buffer[(BLOCK_LEN - BLOCK_SHIFT)..].copy_from_slice(&samples);

        // run inference
        let inputs = self.model_1_inputs();
        let mut outputs = candle_onnx::simple_eval(&self.model1, inputs).unwrap();
        self.shared_state_model1 = outputs.remove("Identity_1").unwrap();
        let inputs = self.model_2_inputs(outputs);
        let mut outputs = candle_onnx::simple_eval(&self.model2, inputs).unwrap();
        self.shared_state_model2 = outputs.remove("Identity_1").unwrap();
        let model_output = self.extract_output(outputs);

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

    fn model_1_inputs(&mut self) -> HashMap<String, Tensor> {
        // Prepare FFT input
        let fft = self.fft_planner.plan_fft_forward(BLOCK_LEN);

        // Perform real-to-complex FFT
        let mut fft_in = self.in_buffer;
        fft.process_with_scratch(&mut fft_in, &mut self.spectrum, &mut self.fft_scratch)
            .unwrap();

        // Generate magnitude and phase
        for ((mag, phase), complex) in self
            .in_mag
            .iter_mut()
            .zip(self.in_phase.iter_mut())
            .zip(self.spectrum)
        {
            *mag = complex.norm();
            *phase = complex.arg();
        }

        let input_2 =
            Tensor::from_slice::<_, f32>(&self.in_mag, (1, 1, FFT_OUT_SIZE), &Device::Cpu).unwrap();

        let inputs = HashMap::from([
            ("input_2".to_string(), input_2),
            ("input_3".to_string(), self.shared_state_model1.clone()),
        ]);
        inputs
    }

    fn model_2_inputs(&mut self, mut outputs: HashMap<String, Tensor>) -> HashMap<String, Tensor> {
        let out_mask: Vec<f32> = outputs
            .remove("Identity") // -> model_outputs_1[0]  ->  [1, 257]
            .unwrap()
            .i((0, 0))
            .unwrap()
            .to_vec1()
            .unwrap();

        // Apply mask and reconstruct complex spectrum
        let mut spectrum = [Complex::I; FFT_OUT_SIZE];
        for i in 0..FFT_OUT_SIZE {
            let magnitude = self.in_mag[i] * out_mask[i];
            let phase = self.in_phase[i];
            let real = magnitude * phase.cos();
            let imag = magnitude * phase.sin();
            spectrum[i] = Complex::new(real, imag);
        }

        // Handle DC component (i = 0)
        let magnitude = self.in_mag[0] * out_mask[0];
        spectrum[0] = Complex::new(magnitude, 0.0);

        // Handle Nyquist component (i = N/2)
        let magnitude = self.in_mag[FFT_OUT_SIZE - 1] * out_mask[FFT_OUT_SIZE - 1];
        spectrum[FFT_OUT_SIZE - 1] = Complex::new(magnitude, 0.0);

        // Perform complex-to-real IFFT
        let ifft = self.fft_planner.plan_fft_inverse(BLOCK_LEN);
        ifft.process_with_scratch(&mut spectrum, &mut self.signal, &mut self.fft_scratch)
            .unwrap();

        // Normalize the IFFT output
        for real in &mut self.signal {
            *real /= BLOCK_LEN as f32;
        }

        let input_4 =
            Tensor::from_slice::<_, f32>(&self.signal, (1, 1, BLOCK_LEN), &Device::Cpu).unwrap();

        HashMap::from([
            ("input_4".to_string(), input_4),
            ("input_5".to_string(), self.shared_state_model2.clone()),
        ])
    }

    fn extract_output(&mut self, mut outputs: HashMap<String, Tensor>) -> Vec<f32> {
        let out_block: Vec<f32> = outputs
            .remove("Identity")
            .unwrap()
            .i((0, 0))
            .unwrap()
            .to_vec1()
            .unwrap();

        out_block
    }
}

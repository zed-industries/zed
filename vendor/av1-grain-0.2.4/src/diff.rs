use anyhow::{ensure, Result};
use num_rational::Rational64;
use v_frame::{frame::Frame, pixel::Pixel};

use self::solver::{FlatBlockFinder, NoiseModel};
use crate::{util::frame_into_u8, GrainTableSegment};

mod solver;

const BLOCK_SIZE: usize = 32;
const BLOCK_SIZE_SQUARED: usize = BLOCK_SIZE * BLOCK_SIZE;

pub struct DiffGenerator {
    fps: Rational64,
    source_bit_depth: usize,
    denoised_bit_depth: usize,
    frame_count: usize,
    prev_timestamp: u64,
    flat_block_finder: FlatBlockFinder,
    noise_model: NoiseModel,
    grain_table: Vec<GrainTableSegment>,
}

impl DiffGenerator {
    #[must_use]
    pub fn new(fps: Rational64, source_bit_depth: usize, denoised_bit_depth: usize) -> Self {
        Self {
            frame_count: 0,
            fps,
            flat_block_finder: FlatBlockFinder::new(),
            noise_model: NoiseModel::new(),
            grain_table: Vec::new(),
            prev_timestamp: 0,
            source_bit_depth,
            denoised_bit_depth,
        }
    }

    /// Processes the next frame and adds the results to the state of this
    /// `DiffGenerator`.
    ///
    /// # Errors
    /// - If the frames do not have the same resolution
    /// - If the frames do not have the same chroma subsampling
    pub fn diff_frame<T: Pixel, U: Pixel>(
        &mut self,
        source: &Frame<T>,
        denoised: &Frame<U>,
    ) -> Result<()> {
        self.diff_frame_internal(
            &frame_into_u8(source, self.source_bit_depth),
            &frame_into_u8(denoised, self.denoised_bit_depth),
        )
    }

    /// Finalize the state of this `DiffGenerator` and return the resulting
    /// grain table segments.
    #[must_use]
    pub fn finish(mut self) -> Vec<GrainTableSegment> {
        log::debug!("Updating final parameters");
        self.grain_table.push(
            self.noise_model
                .get_grain_parameters(self.prev_timestamp, i64::MAX as u64),
        );

        self.grain_table
    }

    fn diff_frame_internal(&mut self, source: &Frame<u8>, denoised: &Frame<u8>) -> Result<()> {
        verify_dimensions_match(source, denoised)?;

        let (flat_blocks, num_flat_blocks) = self.flat_block_finder.run(&source.planes[0]);
        log::debug!("Num flat blocks: {num_flat_blocks}");

        log::debug!("Updating noise model");
        let status = self.noise_model.update(source, denoised, &flat_blocks);

        if status == NoiseStatus::DifferentType {
            let cur_timestamp = self.frame_count as u64 * 10_000_000u64 * *self.fps.denom() as u64
                / *self.fps.numer() as u64;
            log::debug!(
                "Updating parameters for times {} to {}",
                self.prev_timestamp,
                cur_timestamp
            );
            self.grain_table.push(
                self.noise_model
                    .get_grain_parameters(self.prev_timestamp, cur_timestamp),
            );
            self.noise_model.save_latest();
            self.prev_timestamp = cur_timestamp;
        }
        log::debug!("Noise model updated for frame {}", self.frame_count);
        self.frame_count += 1;

        Ok(())
    }
}

#[derive(Debug)]
enum NoiseStatus {
    Ok,
    DifferentType,
    #[allow(dead_code)]
    Error(anyhow::Error),
}

impl PartialEq for NoiseStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Error(_), &Self::Error(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

fn verify_dimensions_match(source: &Frame<u8>, denoised: &Frame<u8>) -> Result<()> {
    let res_1 = (source.planes[0].cfg.width, source.planes[0].cfg.height);
    let res_2 = (denoised.planes[0].cfg.width, denoised.planes[0].cfg.height);
    ensure!(
        res_1 == res_2,
        "Luma resolutions were not equal, {}x{} != {}x{}",
        res_1.0,
        res_1.1,
        res_2.0,
        res_2.1
    );

    let res_1 = (source.planes[1].cfg.width, source.planes[1].cfg.height);
    let res_2 = (denoised.planes[1].cfg.width, denoised.planes[1].cfg.height);
    ensure!(
        res_1 == res_2,
        "Chroma resolutions were not equal, {}x{} != {}x{}",
        res_1.0,
        res_1.1,
        res_2.0,
        res_2.1
    );

    Ok(())
}

use std::{mem::size_of, path::Path, slice};

use anyhow::{bail, ensure};
use num_rational::Rational32;
use v_frame::{
    frame::Frame,
    pixel::{ChromaSampling, Pixel},
};
use vapoursynth::{
    video_info::{Property, VideoInfo},
    vsscript::{Environment, EvalFlags},
};

use crate::decoder::VideoDetails;

const OUTPUT_INDEX: i32 = 0;

pub struct VapoursynthDecoder {
    env: Environment,
    frames_read: usize,
    total_frames: usize,
}

impl VapoursynthDecoder {
    /// # Errors
    ///
    /// - If sourcing an invalid Vapoursynth script.
    /// - If using a Vapoursynth script that contains an unsupported video
    ///   format.
    #[inline]
    pub fn new(source: &Path) -> anyhow::Result<VapoursynthDecoder> {
        let env = Environment::from_file(source, EvalFlags::SetWorkingDir)?;
        let total_frames = {
            let (node, _) = env.get_output(OUTPUT_INDEX)?;
            get_num_frames(node.info())?
        };
        Ok(Self {
            env,
            frames_read: 0,
            total_frames,
        })
    }

    /// # Errors
    ///
    /// - If sourcing an invalid Vapoursynth script.
    /// - If using a Vapoursynth script that contains an unsupported video
    ///   format.
    #[inline]
    pub fn get_video_details(&self) -> anyhow::Result<VideoDetails> {
        let (node, _) = self.env.get_output(OUTPUT_INDEX)?;
        let info = node.info();
        let (width, height) = get_resolution(info)?;
        Ok(VideoDetails {
            width,
            height,
            bit_depth: get_bit_depth(info)?,
            chroma_sampling: get_chroma_sampling(info)?,
            time_base: get_time_base(info)?,
        })
    }

    /// # Errors
    ///
    /// - If sourcing an invalid Vapoursynth script.
    /// - If using a Vapoursynth script that contains an unsupported video
    ///   format.
    /// - If a frame cannot be read.
    #[allow(clippy::transmute_ptr_to_ptr)]
    #[inline]
    pub fn read_video_frame<T: Pixel>(&mut self, cfg: &VideoDetails) -> anyhow::Result<Frame<T>> {
        const SB_SIZE_LOG2: usize = 6;
        const SB_SIZE: usize = 1 << SB_SIZE_LOG2;
        const SUBPEL_FILTER_SIZE: usize = 8;
        const FRAME_MARGIN: usize = 16 + SUBPEL_FILTER_SIZE;
        const LUMA_PADDING: usize = SB_SIZE + FRAME_MARGIN;

        if self.frames_read >= self.total_frames {
            bail!("No frames left");
        }

        let (node, _) = self.env.get_output(OUTPUT_INDEX)?;
        let vs_frame = node.get_frame(self.frames_read)?;
        self.frames_read += 1;

        let bytes = size_of::<T>();
        let mut f: Frame<T> =
            Frame::new_with_padding(cfg.width, cfg.height, cfg.chroma_sampling, LUMA_PADDING);

        // SAFETY: We are using the stride to compute the length of the data slice
        unsafe {
            f.planes[0].copy_from_raw_u8(
                slice::from_raw_parts(
                    vs_frame.data_ptr(0),
                    vs_frame.stride(0) * vs_frame.height(0),
                ),
                vs_frame.stride(0),
                bytes,
            );
            f.planes[1].copy_from_raw_u8(
                slice::from_raw_parts(
                    vs_frame.data_ptr(1),
                    vs_frame.stride(1) * vs_frame.height(1),
                ),
                vs_frame.stride(1),
                bytes,
            );
            f.planes[2].copy_from_raw_u8(
                slice::from_raw_parts(
                    vs_frame.data_ptr(2),
                    vs_frame.stride(2) * vs_frame.height(2),
                ),
                vs_frame.stride(2),
                bytes,
            );
        }
        Ok(f)
    }
}

/// Get the number of frames from a Vapoursynth `VideoInfo` struct.
fn get_num_frames(info: VideoInfo) -> anyhow::Result<usize> {
    let num_frames = {
        if Property::Variable == info.format {
            bail!("Cannot output clips with varying format");
        }
        if Property::Variable == info.resolution {
            bail!("Cannot output clips with varying dimensions");
        }
        if Property::Variable == info.framerate {
            bail!("Cannot output clips with varying framerate");
        }

        info.num_frames
    };

    ensure!(num_frames != 0, "vapoursynth reported 0 frames");

    Ok(num_frames)
}

/// Get the bit depth from a Vapoursynth `VideoInfo` struct.
fn get_bit_depth(info: VideoInfo) -> anyhow::Result<usize> {
    let bits_per_sample = {
        match info.format {
            Property::Variable => {
                bail!("Cannot output clips with variable format");
            }
            Property::Constant(x) => x.bits_per_sample(),
        }
    };

    Ok(bits_per_sample as usize)
}

/// Get the resolution from a Vapoursynth `VideoInfo` struct.
fn get_resolution(info: VideoInfo) -> anyhow::Result<(usize, usize)> {
    let resolution = {
        match info.resolution {
            Property::Variable => {
                bail!("Cannot output clips with variable resolution");
            }
            Property::Constant(x) => x,
        }
    };

    Ok((resolution.width, resolution.height))
}

/// Get the time base (inverse of frame rate) from a Vapoursynth `VideoInfo`
/// struct.
fn get_time_base(info: VideoInfo) -> anyhow::Result<Rational32> {
    match info.framerate {
        Property::Variable => bail!("Cannot output clips with varying framerate"),
        Property::Constant(fps) => Ok(Rational32::new(
            fps.denominator as i32,
            fps.numerator as i32,
        )),
    }
}

/// Get the chroma sampling from a Vapoursynth `VideoInfo` struct.
fn get_chroma_sampling(info: VideoInfo) -> anyhow::Result<ChromaSampling> {
    match info.format {
        Property::Variable => bail!("Variable pixel format not supported"),
        Property::Constant(x) => match x.color_family() {
            vapoursynth::format::ColorFamily::YUV => {
                let ss = (x.sub_sampling_w(), x.sub_sampling_h());
                match ss {
                    (1, 1) => Ok(ChromaSampling::Cs420),
                    (1, 0) => Ok(ChromaSampling::Cs422),
                    (0, 0) => Ok(ChromaSampling::Cs444),
                    _ => bail!("Unrecognized chroma subsampling"),
                }
            }
            vapoursynth::format::ColorFamily::Gray => Ok(ChromaSampling::Cs400),
            _ => bail!("Currently only YUV input is supported"),
        },
    }
}

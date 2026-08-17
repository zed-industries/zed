use std::path::Path;

use anyhow::{bail, ensure, Result};
use num_rational::Rational64;
use v_frame::{frame::Frame, plane::Plane};
use vapoursynth::{
    format::Format,
    prelude::{ColorFamily, Environment, EvalFlags, FrameRef, Property},
    video_info::VideoInfo,
};

use super::{ColorFormat, PixelFormat, VideoSource};

const OUTPUT_INDEX: i32 = 0i32;

pub(super) struct VapoursynthSource {
    env: Environment,
}

impl VapoursynthSource {
    pub(super) fn open(source: &Path) -> Result<Self> {
        let mut env = Environment::new()?;
        env.eval_file(source, EvalFlags::SetWorkingDir)?;

        Ok(VapoursynthSource { env })
    }
}

impl VideoSource for VapoursynthSource {
    fn read_frame(&mut self, frameno: usize) -> Result<Option<Frame<u8>>> {
        let (node, _) = self.env.get_output(OUTPUT_INDEX)?;
        Ok(node
            .get_frame(frameno)
            .map(|frame| Some(vs_frame_to_v_frame(&frame)))?)
    }

    fn get_frame_count(&mut self) -> Result<usize> {
        let info = vs_get_clip_info(&mut self.env)?;

        let num_frames = {
            if Property::Variable == info.format {
                bail!("Cannot handle clips with varying format");
            }
            if Property::Variable == info.resolution {
                bail!("Cannot handle clips with varying dimensions");
            }
            if Property::Variable == info.framerate {
                bail!("Cannot handle clips with varying framerate");
            }

            info.num_frames
        };

        ensure!(num_frames > 0, "vapoursynth reported 0 frames");

        Ok(num_frames)
    }

    fn get_frame_rate(&mut self) -> Result<Rational64> {
        let info = vs_get_clip_info(&mut self.env)?;

        match info.framerate {
            Property::Variable => bail!("Cannot output clips with varying framerate"),
            Property::Constant(fps) => Ok(Rational64::new(
                fps.numerator as i64,
                fps.denominator as i64,
            )),
        }
    }

    fn get_resolution(&mut self) -> Result<(u32, u32)> {
        let info = vs_get_clip_info(&mut self.env)?;

        let resolution = {
            match info.resolution {
                Property::Variable => {
                    bail!("Cannot output clips with variable resolution");
                }
                Property::Constant(x) => x,
            }
        };

        Ok((resolution.width as u32, resolution.height as u32))
    }

    fn get_pixel_format(&mut self) -> Result<ColorFormat> {
        let info = vs_get_clip_info(&mut self.env)?;

        ColorFormat::try_from(info.format)
    }
}

impl TryFrom<Property<Format<'_>>> for ColorFormat {
    type Error = anyhow::Error;

    fn try_from(format: Property<Format>) -> Result<Self, Self::Error> {
        match format {
            Property::Variable => bail!("Variable pixel format not supported"),
            Property::Constant(x) => ColorFormat::try_from(x),
        }
    }
}

impl TryFrom<Format<'_>> for ColorFormat {
    type Error = anyhow::Error;

    fn try_from(format: Format) -> Result<Self, Self::Error> {
        Ok(match format.color_family() {
            ColorFamily::Gray => ColorFormat {
                pixel_format: PixelFormat::YUV400,
                bit_depth: format.bits_per_sample(),
            },
            ColorFamily::YUV => ColorFormat {
                pixel_format: match format.sub_sampling_h() + format.sub_sampling_w() {
                    0 => PixelFormat::YUV444,
                    1 => PixelFormat::YUV422,
                    2 => PixelFormat::YUV420,
                    _ => unreachable!(),
                },
                bit_depth: format.bits_per_sample(),
            },
            _ => bail!("Only YUV clips are supported"),
        })
    }
}

fn vs_get_clip_info(env: &mut Environment) -> Result<VideoInfo> {
    // Get the output node.
    let (node, _) = env.get_output(OUTPUT_INDEX)?;

    Ok(node.info())
}

fn vs_frame_to_v_frame(in_frame: &FrameRef) -> Frame<u8> {
    let mut planes = [
        Plane::new(0, 0, 0, 0, 0, 0),
        Plane::new(0, 0, 0, 0, 0, 0),
        Plane::new(0, 0, 0, 0, 0, 0),
    ];
    let format =
        ColorFormat::try_from(in_frame.format()).expect("Color format has already been checked");
    for p in 0..format.pixel_format.planes() {
        let xdec = if p > 0 {
            format.pixel_format.subsampling().0
        } else {
            0
        };
        let ydec = if p > 0 {
            format.pixel_format.subsampling().1
        } else {
            0
        };
        let mut plane: Plane<u8> = Plane::new(
            in_frame.width(p) as usize >> xdec,
            in_frame.height(p) as usize >> ydec,
            xdec,
            ydec,
            0usize,
            0usize,
        );

        if format.bit_depth == 8 {
            let in_data: &[u8] = in_frame.plane(p).unwrap();
            assert!(plane.data.len() == in_data.len());
            plane.data_origin_mut().copy_from_slice(in_data);
        } else {
            let in_data: &[u16] = in_frame.plane(p).unwrap();
            assert!(plane.data.len() == in_data.len());
            in_data
                .iter()
                .zip(plane.data_origin_mut().iter_mut())
                .for_each(|(i, o)| {
                    *o = (i >> (format.bit_depth - 8)) as u8;
                });
        }

        planes[p] = plane;
    }
    Frame { planes }
}

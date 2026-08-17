use std::{path::Path, ptr::addr_of_mut};

use anyhow::{bail, ensure, Result};
use arrayvec::ArrayVec;
use ffmpeg_next::{
    codec::context::Context,
    decoder,
    format::{context::Input, input, Pixel},
    media::Type as MediaType,
    Error::StreamNotFound,
};
use num_rational::Rational64;
use v_frame::{frame::Frame, plane::Plane};

use super::{ColorFormat, PixelFormat, VideoSource};

pub(super) struct FfmpegSource {
    input: Input,
    decoder: decoder::Video,
    next_frameno: usize,
}

impl FfmpegSource {
    pub(super) fn open(source: &Path) -> Result<Self> {
        let input = input(&source)?;
        let video_stream = input
            .streams()
            .best(MediaType::Video)
            .ok_or(StreamNotFound)?;
        let decoder = Context::from_parameters(video_stream.parameters())?
            .decoder()
            .video()?;

        Ok(FfmpegSource {
            input,
            decoder,
            next_frameno: 0,
        })
    }
}

impl VideoSource for FfmpegSource {
    fn read_frame(&mut self, frameno: usize) -> Result<Option<Frame<u8>>> {
        ensure!(
            frameno == self.next_frameno,
            "Frame number mismatch in read_frame, ffmpeg decoder is desynced from av1-grain"
        );

        loop {
            // SAFETY: This is a really bad Rust interface from ffmpeg_next.
            // We don't let the frame escape unless it's initialized successfully.
            unsafe {
                let mut frame = ffmpeg_next::util::frame::Frame::empty();
                if self.decoder.receive_frame(&mut frame).is_ok() {
                    self.next_frameno += 1;
                    return Ok(Some(ffmpeg_frame_to_v_frame(&mut frame)));
                }
            };

            let video_stream = self
                .input
                .streams()
                .best(MediaType::Video)
                .ok_or(StreamNotFound)?;
            let video_stream_index = video_stream.index();

            let packet = self
                .input
                .packets()
                .find(|&(ref stream, _)| stream.index() == video_stream_index);
            if packet.is_none() {
                return Ok(None);
            }

            let (_, packet) = packet.unwrap();
            self.decoder.send_packet(&packet)?;
        }
    }

    fn get_frame_count(&mut self) -> Result<usize> {
        let video_stream = self
            .input
            .streams()
            .best(MediaType::Video)
            .ok_or(StreamNotFound)?;
        let video_stream_index = video_stream.index();

        let num_frames = self
            .input
            .packets()
            .filter(|&(ref stream, _)| stream.index() == video_stream_index)
            .count();
        self.input.seek(0, 0..1)?;

        ensure!(num_frames > 0, "ffmpeg reported 0 frames");

        Ok(num_frames)
    }

    fn get_frame_rate(&mut self) -> Result<Rational64> {
        let video_stream = self
            .input
            .streams()
            .best(MediaType::Video)
            .ok_or(StreamNotFound)?;
        let rate = video_stream.avg_frame_rate();
        Ok(Rational64::new(i64::from(rate.0), i64::from(rate.1)))
    }

    fn get_resolution(&mut self) -> Result<(u32, u32)> {
        let video_stream = self
            .input
            .streams()
            .best(MediaType::Video)
            .ok_or(StreamNotFound)?;

        let decoder = Context::from_parameters(video_stream.parameters())?
            .decoder()
            .video()?;

        Ok((decoder.width(), decoder.height()))
    }

    fn get_pixel_format(&mut self) -> Result<ColorFormat> {
        let video_stream = self
            .input
            .streams()
            .best(MediaType::Video)
            .ok_or(StreamNotFound)?;

        let decoder = Context::from_parameters(video_stream.parameters())?
            .decoder()
            .video()?;

        ColorFormat::try_from(decoder.format())
    }
}

impl TryFrom<Pixel> for ColorFormat {
    type Error = anyhow::Error;

    fn try_from(format: Pixel) -> Result<Self> {
        Ok(match format {
            Pixel::YUV420P | Pixel::YUVJ420P => ColorFormat {
                pixel_format: PixelFormat::YUV420,
                bit_depth: 8,
            },
            Pixel::YUV422P | Pixel::YUVJ422P => ColorFormat {
                pixel_format: PixelFormat::YUV422,
                bit_depth: 8,
            },
            Pixel::YUV444P | Pixel::YUVJ444P => ColorFormat {
                pixel_format: PixelFormat::YUV444,
                bit_depth: 8,
            },
            Pixel::YUV420P9 | Pixel::YUV420P9BE | Pixel::YUV420P9LE => ColorFormat {
                pixel_format: PixelFormat::YUV420,
                bit_depth: 9,
            },
            Pixel::YUV422P9 | Pixel::YUV422P9BE | Pixel::YUV422P9LE => ColorFormat {
                pixel_format: PixelFormat::YUV422,
                bit_depth: 9,
            },
            Pixel::YUV444P9 | Pixel::YUV444P9BE | Pixel::YUV444P9LE => ColorFormat {
                pixel_format: PixelFormat::YUV444,
                bit_depth: 9,
            },
            Pixel::YUV420P10 | Pixel::YUV420P10BE | Pixel::YUV420P10LE => ColorFormat {
                pixel_format: PixelFormat::YUV420,
                bit_depth: 10,
            },
            Pixel::YUV422P10 | Pixel::YUV422P10BE | Pixel::YUV422P10LE => ColorFormat {
                pixel_format: PixelFormat::YUV422,
                bit_depth: 10,
            },
            Pixel::YUV444P10 | Pixel::YUV444P10BE | Pixel::YUV444P10LE => ColorFormat {
                pixel_format: PixelFormat::YUV444,
                bit_depth: 10,
            },
            Pixel::YUV420P12 | Pixel::YUV420P12BE | Pixel::YUV420P12LE => ColorFormat {
                pixel_format: PixelFormat::YUV420,
                bit_depth: 12,
            },
            Pixel::YUV422P12 | Pixel::YUV422P12BE | Pixel::YUV422P12LE => ColorFormat {
                pixel_format: PixelFormat::YUV422,
                bit_depth: 12,
            },
            Pixel::YUV444P12 | Pixel::YUV444P12BE | Pixel::YUV444P12LE => ColorFormat {
                pixel_format: PixelFormat::YUV444,
                bit_depth: 12,
            },
            Pixel::YUV420P14 | Pixel::YUV420P14BE | Pixel::YUV420P14LE => ColorFormat {
                pixel_format: PixelFormat::YUV420,
                bit_depth: 14,
            },
            Pixel::YUV422P14 | Pixel::YUV422P14BE | Pixel::YUV422P14LE => ColorFormat {
                pixel_format: PixelFormat::YUV422,
                bit_depth: 14,
            },
            Pixel::YUV444P14 | Pixel::YUV444P14BE | Pixel::YUV444P14LE => ColorFormat {
                pixel_format: PixelFormat::YUV444,
                bit_depth: 14,
            },
            Pixel::YUV420P16 | Pixel::YUV420P16LE | Pixel::YUV420P16BE => ColorFormat {
                pixel_format: PixelFormat::YUV420,
                bit_depth: 16,
            },
            Pixel::YUV422P16 | Pixel::YUV422P16LE | Pixel::YUV422P16BE => ColorFormat {
                pixel_format: PixelFormat::YUV422,
                bit_depth: 16,
            },
            Pixel::YUV444P16 | Pixel::YUV444P16LE | Pixel::YUV444P16BE => ColorFormat {
                pixel_format: PixelFormat::YUV444,
                bit_depth: 16,
            },
            Pixel::GRAY8 => ColorFormat {
                pixel_format: PixelFormat::YUV400,
                bit_depth: 8,
            },
            Pixel::GRAY16BE | Pixel::GRAY16LE | Pixel::GRAY16 => ColorFormat {
                pixel_format: PixelFormat::YUV400,
                bit_depth: 16,
            },
            _ => bail!("Only YUV clips are supported"),
        })
    }
}

fn ffmpeg_frame_to_v_frame(ff_frame: &mut ffmpeg_next::util::frame::Frame) -> Frame<u8> {
    // SAFETY: We know this pointer is initialized
    let in_frame = unsafe { ffmpeg_next::util::frame::video::Video::wrap(ff_frame.as_mut_ptr()) };
    let mut planes = [
        Plane::new(0, 0, 0, 0, 0, 0),
        Plane::new(0, 0, 0, 0, 0, 0),
        Plane::new(0, 0, 0, 0, 0, 0),
    ];
    let format =
        ColorFormat::try_from(in_frame.format()).expect("Color format has already been checked");
    for p in 0..in_frame.planes() {
        let mut plane: Plane<u8> = Plane::new(
            in_frame.plane_width(p) as usize,
            in_frame.plane_height(p) as usize,
            if p > 0 {
                format.pixel_format.subsampling().0
            } else {
                0
            },
            if p > 0 {
                format.pixel_format.subsampling().1
            } else {
                0
            },
            0usize,
            0usize,
        );

        let in_data = in_frame.data(p);

        if format.bit_depth == 8 {
            assert!(plane.data.len() == in_data.len());
            plane.data_origin_mut().copy_from_slice(in_data);
        } else {
            assert!(plane.data.len() * 2 == in_data.len());
            in_data
                .chunks_exact(2)
                .zip(plane.data_origin_mut().iter_mut())
                .for_each(|(i, o)| {
                    let i = u16::from_le_bytes([i[0], i[1]]);
                    *o = (i >> (format.bit_depth - 8)) as u8;
                });
        }

        planes[p] = plane;
    }
    Frame { planes }
}

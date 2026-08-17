extern crate ffmpeg_the_third as ffmpeg;

use std::path::Path;

use anyhow::bail;
use ffmpeg::{
    codec::{decoder, packet},
    format,
    format::context,
    frame,
    media::Type,
};
use ffmpeg_the_third::threading;
use num_rational::Rational32;
use v_frame::{
    frame::Frame,
    pixel::{ChromaSampling, Pixel},
};

use crate::decoder::VideoDetails;

/// An interface that is used for decoding a video stream using ffmpeg
///
/// There have been desync issue reported with this decoder
/// on some video files. Use at your own risk!
pub struct FfmpegDecoder {
    input_ctx: context::Input,
    decoder: decoder::Video,
    pub video_details: VideoDetails,
    frameno: usize,
    stream_index: usize,
    end_of_stream: bool,
    eof_sent: bool,
}

impl FfmpegDecoder {
    /// Initialize a new ffmpeg decoder for a given input file
    ///
    /// # Errors
    ///
    /// - If ffmpeg is not available or not working on the system
    /// - If the source contains no video
    /// - If the source has an unsupported pixel format
    #[inline]
    pub fn new<P: AsRef<Path>>(input: P) -> anyhow::Result<Self> {
        ffmpeg::init()?;

        let input_ctx = format::input(&input)?;
        let input = input_ctx
            .streams()
            .best(Type::Video)
            .ok_or_else(|| anyhow::anyhow!("Could not find video stream"))?;
        let stream_index = input.index();
        let mut context = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        context.set_threading(threading::Config::kind(threading::Type::Frame));
        let mut decoder = context.decoder().video()?;
        decoder.set_parameters(input.parameters())?;

        let frame_rate = input.avg_frame_rate();
        Ok(Self {
            video_details: VideoDetails {
                width: decoder.width() as usize,
                height: decoder.height() as usize,
                bit_depth: match decoder.format() {
                    format::pixel::Pixel::YUV420P
                    | format::pixel::Pixel::YUV422P
                    | format::pixel::Pixel::YUV444P
                    | format::pixel::Pixel::YUVJ420P
                    | format::pixel::Pixel::YUVJ422P
                    | format::pixel::Pixel::YUVJ444P => 8,
                    format::pixel::Pixel::YUV420P10LE
                    | format::pixel::Pixel::YUV422P10LE
                    | format::pixel::Pixel::YUV444P10LE => 10,
                    format::pixel::Pixel::YUV420P12LE
                    | format::pixel::Pixel::YUV422P12LE
                    | format::pixel::Pixel::YUV444P12LE => 12,
                    _ => {
                        bail!("Unsupported pixel format {:?}", decoder.format());
                    }
                },
                chroma_sampling: match decoder.format() {
                    format::pixel::Pixel::YUV420P
                    | format::pixel::Pixel::YUVJ420P
                    | format::pixel::Pixel::YUV420P10LE
                    | format::pixel::Pixel::YUV420P12LE => ChromaSampling::Cs420,
                    format::pixel::Pixel::YUV422P
                    | format::pixel::Pixel::YUVJ422P
                    | format::pixel::Pixel::YUV422P10LE
                    | format::pixel::Pixel::YUV422P12LE => ChromaSampling::Cs422,
                    format::pixel::Pixel::YUV444P
                    | format::pixel::Pixel::YUVJ444P
                    | format::pixel::Pixel::YUV444P10LE
                    | format::pixel::Pixel::YUV444P12LE => ChromaSampling::Cs444,
                    _ => {
                        bail!("Unsupported pixel format {:?}", decoder.format());
                    }
                },
                time_base: Rational32::new(frame_rate.denominator(), frame_rate.numerator()),
            },
            decoder,
            input_ctx,
            frameno: 0,
            stream_index,
            end_of_stream: false,
            eof_sent: false,
        })
    }

    fn decode_frame<T: Pixel>(&self, decoded: &frame::Video) -> Frame<T> {
        const SB_SIZE_LOG2: usize = 6;
        const SB_SIZE: usize = 1 << SB_SIZE_LOG2;
        const SUBPEL_FILTER_SIZE: usize = 8;
        const FRAME_MARGIN: usize = 16 + SUBPEL_FILTER_SIZE;
        const LUMA_PADDING: usize = SB_SIZE + FRAME_MARGIN;

        let mut f: Frame<T> = Frame::new_with_padding(
            self.video_details.width,
            self.video_details.height,
            self.video_details.chroma_sampling,
            LUMA_PADDING,
        );
        let width = self.video_details.width;
        let height = self.video_details.height;
        let bit_depth = self.video_details.bit_depth;
        let bytes = if bit_depth > 8 { 2 } else { 1 };
        let (chroma_width, _) = self
            .video_details
            .chroma_sampling
            .get_chroma_dimensions(width, height);
        f.planes[0].copy_from_raw_u8(decoded.data(0), width * bytes, bytes);
        f.planes[1].copy_from_raw_u8(decoded.data(1), chroma_width * bytes, bytes);
        f.planes[2].copy_from_raw_u8(decoded.data(2), chroma_width * bytes, bytes);
        f
    }

    /// # Errors
    ///
    /// - If there are no frames remaining
    #[inline]
    pub fn read_video_frame<T: Pixel>(&mut self) -> anyhow::Result<Frame<T>> {
        // For some reason there's a crap ton of work needed to get ffmpeg to do
        // something simple, because each codec has it's own stupid way of doing
        // things and they don't all decode the same way.
        //
        // Maybe ffmpeg could have made a simple, singular interface that does this for
        // us, but noooooo.
        //
        // Reference: https://ffmpeg.org/doxygen/trunk/api-h264-test_8c_source.html#l00110
        loop {
            // This iterator is actually really stupid... it doesn't reset itself after each
            // `new`. But that solves our lifetime hell issues, ironically.
            let packet = self
                .input_ctx
                .packets()
                .next()
                .and_then(Result::ok)
                .map(|(_, packet)| packet);

            let mut packet = if let Some(packet) = packet {
                packet
            } else {
                self.end_of_stream = true;
                packet::Packet::empty()
            };

            if self.end_of_stream && !self.eof_sent {
                let _ = self.decoder.send_eof();
                self.eof_sent = true;
            }

            if self.end_of_stream || packet.stream() == self.stream_index {
                let mut decoded = frame::Video::new(
                    self.decoder.format(),
                    self.video_details.width as u32,
                    self.video_details.height as u32,
                );
                packet.set_pts(Some(self.frameno as i64));
                packet.set_dts(Some(self.frameno as i64));

                if !self.end_of_stream {
                    let _ = self.decoder.send_packet(&packet);
                }

                if self.decoder.receive_frame(&mut decoded).is_ok() {
                    let f = self.decode_frame(&decoded);
                    self.frameno += 1;
                    return Ok(f);
                } else if self.end_of_stream {
                    bail!("No frames left");
                }
            }
        }
    }
}

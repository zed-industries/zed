// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_HELPER_H_

#include <memory>

#include "media/base/media_types.h"
#include "media/media_buildflags.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace media {

#if BUILDFLAG(USE_PROPRIETARY_CODECS)
class H264ToAnnexBBitstreamConverter;
#if BUILDFLAG(ENABLE_PLATFORM_HEVC)
class H265ToAnnexBBitstreamConverter;
#endif  // BUILDFLAG(ENABLE_PLATFORM_HEVC)
namespace mp4 {
struct AVCDecoderConfigurationRecord;
#if BUILDFLAG(ENABLE_PLATFORM_HEVC)
struct HEVCDecoderConfigurationRecord;
#endif  // BUILDFLAG(ENABLE_PLATFORM_HEVC)
}  // namespace mp4
#endif  // BUILDFLAG(USE_PROPRIETARY_CODECS)

}  // namespace media

namespace blink {

// VideoDecoderHelper is a class to convert stream from MP4 format (as
// specified in ISO/IEC 14496-15) into Annex B bytestream (as specified
// in ISO/IEC 14496-10). It is a shim to the underlying
// H264ToAnnexBBitstreamConverter or H265ToAnnexBBitstreamConverter.
class MODULES_EXPORT VideoDecoderHelper {
 public:
  enum class Status : uint8_t {
    kGenericFailure,
    kDescriptionParseFailed,
    kUnsupportedCodec,
    kBitstreamConvertFailed,
    kSucceed,
    kNumEvents,
  };
  explicit VideoDecoderHelper(media::VideoType video_type);
  VideoDecoderHelper(const VideoDecoderHelper&) = delete;
  VideoDecoderHelper& operator=(const VideoDecoderHelper&) = delete;

  ~VideoDecoderHelper();

  // Create an instance of this class. Failure reason  will be reported
  // via |status_out|.
  static std::unique_ptr<VideoDecoderHelper> Create(
      media::VideoType video_type,
      const uint8_t* configuration_record,
      int configuration_record_size,
      Status* status_out);

  // Calculates needed buffer size for the bitstream converted into bytestream.
  //
  // Parameters
  //   input
  //     Pointer to buffer containing NAL units in MP4 format.
  //   input_size
  //     Size of the buffer in bytes.
  //   is_first_chunk
  //     True if this chunk is the first in the stream (follows a configure() or
  //     a flush()).
  // Returns
  //   Required buffer size for the output NAL unit buffer when converted to
  //   bytestream format, or 0 if could not determine the size of the output
  //   buffer from the data in |input|.
  uint32_t CalculateNeededOutputBufferSize(const uint8_t* input,
                                           uint32_t input_size,
                                           bool is_first_chunk) const;

  // ConvertNalUnitStreamToByteStream converts the NAL unit from MP4 format
  // to bytestream format. Client is responsible for making sure the output
  // buffer is large enough to hold the output data. Client can precalculate the
  // needed output buffer size by using CalculateNeededOutputBufferSize.
  //
  // Parameters
  //   input
  //     Pointer to buffer containing NAL units in MP4 format.
  //   input_size
  //     Size of the buffer in bytes.
  //   output
  //     Pointer to buffer where the output should be written to.
  //   output_size (i/o)
  //     Pointer to the size of the output buffer. Will contain the number of
  //     bytes written to output after successful call.
  //   is_first_chunk
  //     True if this chunk is the first in the stream (follows a configure() or
  //     a flush()).
  //
  // Returns
  //    kSucceed  if successful conversion
  //    kBitstreamConvertFailed if conversion not successful (output_size will
  //    hold the amount of converted data)
  Status ConvertNalUnitStreamToByteStream(const uint8_t* input,
                                          uint32_t input_size,
                                          uint8_t* output,
                                          uint32_t* output_size,
                                          bool is_first_chunk);

 private:
  Status Initialize(const uint8_t* configuration_record,
                    int configuration_record_size);
#if BUILDFLAG(USE_PROPRIETARY_CODECS)
  std::unique_ptr<media::H264ToAnnexBBitstreamConverter> h264_converter_;
  std::unique_ptr<media::mp4::AVCDecoderConfigurationRecord> h264_avcc_;
#if BUILDFLAG(ENABLE_PLATFORM_HEVC)
  std::unique_ptr<media::H265ToAnnexBBitstreamConverter> h265_converter_;
  std::unique_ptr<media::mp4::HEVCDecoderConfigurationRecord> h265_hvcc_;
#endif  // BUILDFLAG(ENABLE_PLATFORM_HEVC)
#endif  // BUILDFLAG(USE_PROPRIETARY_CODECS)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_HELPER_H_

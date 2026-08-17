/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_DECODER_H_
#define API_VIDEO_CODECS_VIDEO_DECODER_H_

#include <cstdint>
#include <optional>
#include <string>

#include "api/video/encoded_image.h"
#include "api/video/render_resolution.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_frame.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class RTC_EXPORT DecodedImageCallback {
 public:
  virtual ~DecodedImageCallback() {}

  virtual int32_t Decoded(VideoFrame& decodedImage) = 0;
  // Provides an alternative interface that allows the decoder to specify the
  // decode time excluding waiting time for any previous pending frame to
  // return. This is necessary for breaking positive feedback in the delay
  // estimation when the decoder has a single output buffer.
  virtual int32_t Decoded(VideoFrame& decodedImage, int64_t decode_time_ms);

  // TODO(sakal): Remove other implementations when upstream projects have been
  // updated.
  virtual void Decoded(VideoFrame& decodedImage,
                       std::optional<int32_t> decode_time_ms,
                       std::optional<uint8_t> qp);
};

class RTC_EXPORT VideoDecoder {
 public:
  struct DecoderInfo {
    // Descriptive name of the decoder implementation.
    std::string implementation_name;

    // True if the decoder is backed by hardware acceleration.
    bool is_hardware_accelerated = false;

    std::string ToString() const;
    bool operator==(const DecoderInfo& rhs) const;
    bool operator!=(const DecoderInfo& rhs) const { return !(*this == rhs); }
  };

  class Settings {
   public:
    Settings() = default;
    Settings(const Settings&) = default;
    Settings& operator=(const Settings&) = default;
    ~Settings() = default;

    // The size of pool which is used to store video frame buffers inside
    // decoder. If value isn't present some codec-default value will be used. If
    // value is present and decoder doesn't have buffer pool the value will be
    // ignored.
    std::optional<int> buffer_pool_size() const;
    void set_buffer_pool_size(std::optional<int> value);

    // When valid, user of the VideoDecoder interface shouldn't `Decode`
    // encoded images with render resolution larger than width and height
    // specified here.
    RenderResolution max_render_resolution() const;
    void set_max_render_resolution(RenderResolution value);

    // Maximum number of cpu cores the decoder is allowed to use in parallel.
    // Must be positive.
    int number_of_cores() const { return number_of_cores_; }
    void set_number_of_cores(int value);

    // Codec of encoded images user of the VideoDecoder interface will `Decode`.
    VideoCodecType codec_type() const { return codec_type_; }
    void set_codec_type(VideoCodecType value) { codec_type_ = value; }

   private:
    std::optional<int> buffer_pool_size_;
    RenderResolution max_resolution_;
    int number_of_cores_ = 1;
    VideoCodecType codec_type_ = kVideoCodecGeneric;
  };

  virtual ~VideoDecoder() = default;

  // Prepares decoder to handle incoming encoded frames. Can be called multiple
  // times, in such case only latest `settings` are in effect.
  virtual bool Configure(const Settings& settings) = 0;

  // TODO(bugs.webrtc.org/15444): Make pure virtual once all subclasses have
  // migrated to implementing this class.
  virtual int32_t Decode(const EncodedImage& input_image,
                         int64_t render_time_ms) {
    return Decode(input_image, /*missing_frame=*/false, render_time_ms);
  }

  // TODO(bugs.webrtc.org/15444): Migrate all subclasses to Decode() without
  // missing_frame and delete this.
  virtual int32_t Decode(const EncodedImage& input_image,
                         bool /* missing_frames */,
                         int64_t render_time_ms) {
    return Decode(input_image, render_time_ms);
  }

  virtual int32_t RegisterDecodeCompleteCallback(
      DecodedImageCallback* callback) = 0;

  virtual int32_t Release() = 0;

  virtual DecoderInfo GetDecoderInfo() const;

  // Deprecated, use GetDecoderInfo().implementation_name instead.
  virtual const char* ImplementationName() const;
};

inline std::optional<int> VideoDecoder::Settings::buffer_pool_size() const {
  return buffer_pool_size_;
}

inline void VideoDecoder::Settings::set_buffer_pool_size(
    std::optional<int> value) {
  buffer_pool_size_ = value;
}

inline RenderResolution VideoDecoder::Settings::max_render_resolution() const {
  return max_resolution_;
}

inline void VideoDecoder::Settings::set_max_render_resolution(
    RenderResolution value) {
  max_resolution_ = value;
}

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VIDEO_DECODER_H_

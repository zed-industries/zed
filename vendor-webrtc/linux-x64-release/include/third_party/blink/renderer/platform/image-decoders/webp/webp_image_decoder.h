// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_WEBP_WEBP_IMAGE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_WEBP_WEBP_IMAGE_DECODER_H_

#include <stddef.h>

#include "base/functional/callback.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "webp/decode.h"
#include "webp/demux.h"

class SkData;

namespace blink {

class PLATFORM_EXPORT WEBPImageDecoder final : public ImageDecoder {
 public:
  WEBPImageDecoder(AlphaOption, ColorBehavior, wtf_size_t max_decoded_bytes);
  WEBPImageDecoder(const WEBPImageDecoder&) = delete;
  WEBPImageDecoder& operator=(const WEBPImageDecoder&) = delete;
  ~WEBPImageDecoder() override;

  // ImageDecoder:
  String FilenameExtension() const override;
  const AtomicString& MimeType() const override;
  void OnSetData(scoped_refptr<SegmentReader> data) override;
  cc::YUVSubsampling GetYUVSubsampling() const override;
  int RepetitionCount() const override;
  bool FrameIsReceivedAtIndex(wtf_size_t) const override;
  base::TimeDelta FrameDurationAtIndex(wtf_size_t) const override;

 private:
  // ImageDecoder:
  void DecodeSize() override;
  wtf_size_t DecodeFrameCount() override;
  void InitializeNewFrame(wtf_size_t) override;
  void Decode(wtf_size_t) override;
  void DecodeToYUV() override;
  SkYUVColorSpace GetYUVColorSpace() const override;
  cc::ImageHeaderMetadata MakeMetadataForDecodeAcceleration() const override;

  WEBP_CSP_MODE RGBOutputMode();
  // Returns true if the image data received so far (as stored in
  // |consolidated_data_|) can potentially be decoded and rendered from YUV
  // planes.
  bool CanAllowYUVDecodingForWebP() const;
  bool HasImagePlanes() const { return image_planes_.get(); }
  bool DecodeSingleFrameToYUV(const uint8_t* data_bytes, wtf_size_t data_size);
  bool DecodeSingleFrame(const uint8_t* data_bytes,
                         wtf_size_t data_size,
                         wtf_size_t frame_index);

  // For WebP images, the frame status needs to be FrameComplete to decode
  // subsequent frames that depend on frame |index|. The reason for this is that
  // WebP uses the previous frame for alpha blending, in ApplyPostProcessing().
  //
  // Before calling this, verify that frame |index| exists by checking that
  // |index| is smaller than |frame_buffer_cache_|.size().
  bool FrameStatusSufficientForSuccessors(wtf_size_t index) override;

  bool IsDoingYuvDecode() const {
    if (image_planes_) {
      DCHECK(allow_decode_to_yuv_);
      return true;
    }
    return false;
  }

  // Provides the size of each component.
  gfx::Size DecodedYUVSize(cc::YUVIndex) const override;

  // Returns the width of each row of the memory allocation.
  wtf_size_t DecodedYUVWidthBytes(cc::YUVIndex) const override;

  void ReadColorProfile();
  bool UpdateDemuxer();

  // Set |frame_background_has_alpha_| based on this frame's characteristics.
  // Before calling this method, the caller must verify that the frame exists.
  void OnInitFrameBuffer(wtf_size_t frame_index) override;

  // When the blending method of this frame is BlendAtopPreviousFrame, the
  // previous frame's buffer is necessary to decode this frame in
  // ApplyPostProcessing, so we can't take over the data. Before calling this
  // method, the caller must verify that the frame exists.
  bool CanReusePreviousFrameBuffer(wtf_size_t frame_index) const override;

  void ApplyPostProcessing(wtf_size_t frame_index);
  void ClearFrameBuffer(wtf_size_t frame_index) override;

  void Clear();
  void ClearDecoder();

  raw_ptr<WebPIDecoder, DanglingUntriaged> decoder_ = nullptr;
  WebPDecBuffer decoder_buffer_;
  // format_flags_ and is_lossy_not_animated_no_alpha_ are set when
  // demux_state_ is greater than or equal to WEBP_DEMUX_PARSED_HEADER and
  // WebPDemuxGetI(demux_, WEBP_FF_FRAME_COUNT) returns a nonzero value.
  int format_flags_ = 0;
  bool is_lossy_not_animated_no_alpha_ = false;
  bool frame_background_has_alpha_ = false;

  raw_ptr<WebPDemuxer, DanglingUntriaged> demux_ = nullptr;
  WebPDemuxState demux_state_ = WEBP_DEMUX_PARSING_HEADER;
  bool have_parsed_current_data_ = false;
  int repetition_count_ = kAnimationLoopOnce;
  int decoded_height_ = 0;
  // Used to call UpdateBppHistogram<"WebP">() at most once to record the
  // bits-per-pixel value of the image when the image is successfully decoded.
  // Note that void(gfx::Size, size_t) is the function call signature of
  // UpdateBppHistogram<"WebP">().
  base::OnceCallback<void(gfx::Size, size_t)> update_bpp_histogram_callback_;

  typedef void (*AlphaBlendFunction)(ImageFrame&, ImageFrame&, int, int, int);
  AlphaBlendFunction blend_function_;

  // This will point to one of three things:
  // - the SegmentReader's data, if contiguous.
  // - its own copy, if not, and all data was received initially.
  // - |buffer_|, if streaming.
  sk_sp<SkData> consolidated_data_;
  Vector<char> buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_WEBP_WEBP_IMAGE_DECODER_H_

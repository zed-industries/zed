// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_SKIA_SKIA_IMAGE_DECODER_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_SKIA_SKIA_IMAGE_DECODER_BASE_H_

#include <memory>

#include "base/containers/flat_set.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/skia/include/codec/SkCodec.h"

class SkStream;

namespace blink {

class SegmentStream;

// Base class for implementing a `blink::ImageDecoder` on top of an `SkCodec`.
class PLATFORM_EXPORT SkiaImageDecoderBase : public ImageDecoder {
 public:
  static constexpr wtf_size_t kNoReadingOffset = 0;

  SkiaImageDecoderBase(AlphaOption,
                       ColorBehavior,
                       wtf_size_t max_decoded_bytes,
                       wtf_size_t reading_offset = kNoReadingOffset,
                       HighBitDepthDecodingOption bit_depth_option =
                           HighBitDepthDecodingOption::kDefaultBitDepth);

  SkiaImageDecoderBase(const SkiaImageDecoderBase&) = delete;
  SkiaImageDecoderBase& operator=(const SkiaImageDecoderBase&) = delete;
  ~SkiaImageDecoderBase() override;

  // ImageDecoder:
  void OnSetData(scoped_refptr<SegmentReader> data) final;
  int RepetitionCount() const final;
  bool FrameIsReceivedAtIndex(wtf_size_t) const final;
  base::TimeDelta FrameDurationAtIndex(wtf_size_t) const final;
  // CAUTION: SetFailed() deletes |codec_|.  Be careful to avoid
  // accessing deleted memory.
  bool SetFailed() final;

  wtf_size_t ClearCacheExceptFrame(wtf_size_t) final;

 protected:
  // OnCreateSkCodec needs to read enough of the image to get the image size.
  virtual std::unique_ptr<SkCodec> OnCreateSkCodec(std::unique_ptr<SkStream>,
                                                   SkCodec::Result* result) = 0;

 private:
  // ImageDecoder:
  void DecodeSize() final {}
  bool ImageIsHighBitDepth() final;
  wtf_size_t DecodeFrameCount() final;
  void InitializeNewFrame(wtf_size_t) final;
  void Decode(wtf_size_t) final;
  // When the disposal method of the frame is DisposeOverWritePrevious, the
  // next frame will use a previous frame's buffer as its starting state, so
  // we can't take over the data in that case. Before calling this method, the
  // caller must verify that the frame exists.
  bool CanReusePreviousFrameBuffer(wtf_size_t) const final;

  // When a frame depends on a previous frame's content, there is a list of
  // candidate reference frames. This function will find a previous frame from
  // that list which satisfies the requirements of being a reference frame
  // (kFrameComplete, not kDisposeOverwritePrevious).
  // If no frame is found, it returns kNotFound.
  wtf_size_t GetViableReferenceFrameIndex(wtf_size_t) const;

  // Calls the index of the failed frames during decoding. If all frames fail to
  // decode, call SkiaImageDecoderBase::SetFailed.
  void SetFailedFrameIndex(wtf_size_t index);

  // Returns whether decoding of the current frame has failed.
  bool IsFailedFrameIndex(wtf_size_t index) const;

  // Translates `SkCodec`'s values into Blink API values, but leaves field-based
  // statefulness to the non-internal `RepetitionCount`.
  int RepetitionCountInternal() const;

  std::unique_ptr<SkCodec> codec_;

  // |codec_| owns the SegmentStream, but we need access to it to append more
  // data as it arrives.
  raw_ptr<SegmentStream> segment_stream_ = nullptr;

  mutable int repetition_count_ = kAnimationLoopOnce;
  int prior_frame_ = SkCodec::kNoFrame;
  base::flat_set<wtf_size_t> decode_failed_frames_;

  // Offset inside `segment_stream_` where `this` decoder should start decoding
  // an image.  This is useful in scenarios where we want an `SkCodec` to decode
  // an image embedded in a middle of another data stream - one specific example
  // is PNG images embedded inside ICO or BMP images.
  const wtf_size_t reading_offset_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_SKIA_SKIA_IMAGE_DECODER_BASE_H_

// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_DECODER_CORE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_DECODER_CORE_H_

#include "base/sequence_checker.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"

namespace media {
class VideoFrame;
}  // namespace media

namespace blink {
class SegmentReader;

// A wrapper around a blink::ImageDecoder which is designed to be called on a
// specific sequence via WTF::SequenceBound.
class MODULES_EXPORT ImageDecoderCore {
 public:
  // See ImageDecoder::CreateByMimeType() for parameter definitions.
  ImageDecoderCore(String mime_type,
                   scoped_refptr<SegmentReader> data,
                   bool data_complete,
                   ColorBehavior color_behavior,
                   const SkISize& desired_size,
                   ImageDecoder::AnimationOption animation_option);
  ~ImageDecoderCore();

  struct ImageMetadata {
    // ImageDecoder::Failed(). True means the decoder encountered an error (for
    // any reason; i.e., it's not limited to just a failure to decode metadata).
    bool failed = true;

    // Only true after ImageDecoder::IsSizeAvailable() becomes true.
    bool has_size = false;

    // ImageDecoder::FrameCount(). May change as data is appended.
    uint32_t frame_count = 0u;

    // ImageDecoder::RepetitionCount(). May change as data is appended.
    uint32_t repetition_count = 0u;

    // ImageDecoder::ImageHasBothStillAndAnimatedSubImages().
    bool image_has_both_still_and_animated_sub_images = false;

    // The |data_complete| value provided during construction and AppendData().
    bool data_complete = false;
  };

  // Returns all currently known metadata.
  ImageMetadata DecodeMetadata();

  // Status values for Decode().
  enum class Status : uint8_t {
    kOk = 0,       // Image decoded okay.
    kNoImage,      // No new images or no image could currently be decoded.
    kDecodeError,  // ImageDecoder::Failed() became true.
    kIndexError,   // |frame_index| out of range when |data_complete_| is true.
    kAborted,      // Decode was aborted by caller.
  };

  struct ImageDecodeResult {
    Status status = Status::kDecodeError;

    // ImageFrame::GetStatus() == ImageFrame::kFrameComplete.
    bool complete = false;

    // Mirror of the |frame_index| value provided to Decode().
    uint32_t frame_index = 0u;

    // The decoded image (if any), will be nullptr for YUV decoded frames.
    sk_sp<SkImage> sk_image;

    // A frame wrapping |sk_image| if |sk_image| is not null, otherwise a YUV
    // frame representing the decoded image.
    scoped_refptr<media::VideoFrame> frame;
  };

  // Decodes the frame at |frame_index|, returning partial frames relative to
  // the last Decode() request if |complete_frames_only| is false. |abort_flag|
  // may be used to cancel decoding; it must outlive this Decode() call.
  std::unique_ptr<ImageDecodeResult> Decode(uint32_t frame_index,
                                            bool complete_frames_only,
                                            const base::AtomicFlag* abort_flag);

  // Calls ImageDecoder::SetData() after appending |data| to |stream_buffer_|.
  // May not be called after |data_complete| becomes true.
  void AppendData(Vector<uint8_t> data, bool data_complete);

  // Releases |decoder_|. Decode() and DecodeMetadata() may not be called until
  // Reinitialize() has been called.
  void Clear();

  // Recreates the underlying ImageDecode with the provided |animation_option|.
  void Reinitialize(ImageDecoder::AnimationOption animation_option);

  bool FrameIsDecodedAtIndexForTesting(uint32_t frame_index) const;

 private:
  void MaybeDecodeToYuv();

  // Retrieves the timestamp for |index| from |decoder_| if supported, otherwise
  // uses |timestamp_cache_| to generate a synthetic timestamp.
  base::TimeDelta GetTimestampForFrame(uint32_t index) const;

  const String mime_type_;
  const ColorBehavior color_behavior_;
  const SkISize desired_size_;

  ImageDecoder::AnimationOption animation_option_;

  // Set to true either during construction or upon AppendData() being called
  // with |data_complete| == true. Once true, never cleared.
  bool data_complete_ = false;

  // Used when data is streamed in via AppendData().
  scoped_refptr<SharedBuffer> stream_buffer_;

  // Used when all data is provided at construction time.
  scoped_refptr<SegmentReader> segment_reader_;

  // The true workhorse of this matryoshka doll of "image decoders."
  std::unique_ptr<ImageDecoder> decoder_;

  // The YUV decoders don't like to be called more than once or called after a
  // RGB frame is decoded, so store the decoded frame once we have it.
  bool have_completed_rgb_decode_ = false;
  bool have_completed_yuv_decode_ = false;
  scoped_refptr<media::VideoFrame> yuv_frame_;

  // When decode() of incomplete frames has been requested, we need to track the
  // generation id for each SkBitmap that we've handed out. So that we can defer
  // resolution of promises until a new bitmap is generated.
  HashMap<uint32_t, uint32_t, IntWithZeroKeyHashTraits<uint32_t>>
      incomplete_frames_;

  // By default, assume in order decoding and purge all decoded frames except
  // the last. If out of order decoding is detected then purging is limited to
  // only when platform memory limits are exceeded.
  bool is_decoding_in_order_ = true;
  uint32_t last_decoded_frame_ = 0u;

  // Used to generate synthetic timestamps for decoders which don't provide
  // native timestamps. The 0 position is initialized to zero at construction.
  mutable Vector<base::TimeDelta> timestamp_cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_DECODER_CORE_H_

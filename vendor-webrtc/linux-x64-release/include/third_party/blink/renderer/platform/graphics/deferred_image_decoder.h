/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DEFERRED_IMAGE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DEFERRED_IMAGE_DECODER_H_

#include <memory>
#include <optional>

#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_image.h"
#include "third_party/blink/renderer/platform/graphics/parkable_image.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "third_party/skia/include/private/SkGainmapInfo.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ImageFrameGenerator;
struct DeferredFrameData;

class PLATFORM_EXPORT DeferredImageDecoder final {
  USING_FAST_MALLOC(DeferredImageDecoder);

 public:
  static std::unique_ptr<DeferredImageDecoder> Create(
      scoped_refptr<SharedBuffer> data,
      bool data_complete,
      ImageDecoder::AlphaOption,
      ColorBehavior);

  static std::unique_ptr<DeferredImageDecoder> CreateForTesting(
      std::unique_ptr<ImageDecoder>);

  DeferredImageDecoder(const DeferredImageDecoder&) = delete;
  DeferredImageDecoder& operator=(const DeferredImageDecoder&) = delete;
  ~DeferredImageDecoder();

  String FilenameExtension() const;
  const AtomicString& MimeType() const;

  sk_sp<PaintImageGenerator> CreateGenerator();
  bool CreateGainmapGenerator(sk_sp<PaintImageGenerator>& generator,
                              SkGainmapInfo& info);

  scoped_refptr<SharedBuffer> Data();
  bool HasData() const;
  size_t DataSize() const;
  void SetData(scoped_refptr<SharedBuffer> data, bool all_data_received);

  bool IsSizeAvailable();
  bool HasEmbeddedColorProfile() const;
  gfx::Size Size() const;
  gfx::Size FrameSizeAtIndex(wtf_size_t index) const;
  wtf_size_t FrameCount();
  bool ImageIsHighBitDepth() const { return image_is_high_bit_depth_; }
  int RepetitionCount() const;
  bool FrameIsReceivedAtIndex(wtf_size_t index) const;
  base::TimeDelta FrameDurationAtIndex(wtf_size_t index) const;
  ImageOrientation OrientationAtIndex(wtf_size_t index) const;
  gfx::Size DensityCorrectedSizeAtIndex(wtf_size_t index) const;
  bool HotSpot(gfx::Point&) const;
  SkAlphaType AlphaType() const;

  // A less expensive method for getting the number of bytes thus far received
  // for the image. Checking Data()->size() involves copying bytes to a
  // SharedBuffer.
  //
  // Returns 0 if the read-write buffer has not been initialized or received
  // data.
  size_t ByteSize() const;

 private:
  explicit DeferredImageDecoder(std::unique_ptr<ImageDecoder> metadata_decoder);

  friend class DeferredImageDecoderTest;
  ImageFrameGenerator* FrameGenerator() { return frame_generator_.get(); }

  // Lazily create `frame_generator_`, if it has not been created yet.
  void ActivateLazyDecoding();
  void PrepareLazyDecodedFrames();

  // Lazily create `gainmap_`, if it has not been created yet.
  void ActivateLazyGainmapDecoding();

  void SetDataInternal(scoped_refptr<SharedBuffer> data,
                       bool all_data_received,
                       bool push_data_to_decoder);

  // Copy of the data that is passed in, used by deferred decoding.
  // Allows creating readonly snapshots that may be read in another thread.
  scoped_refptr<ParkableImage> parkable_image_;
  std::unique_ptr<ImageDecoder> metadata_decoder_;

  String filename_extension_;
  AtomicString mime_type_;
  gfx::Size size_;
  int repetition_count_;
  bool has_embedded_color_profile_ = false;
  bool all_data_received_;
  bool first_decoding_generator_created_;
  bool can_yuv_decode_;
  bool has_hot_spot_;
  bool image_is_high_bit_depth_;
  sk_sp<SkColorSpace> color_space_for_sk_images_;
  gfx::Point hot_spot_;
  const PaintImage::ContentId complete_frame_content_id_;
  std::optional<bool> incremental_decode_needed_;

  // Set to true if the image is detected to be invalid after parsing the
  // metadata.
  bool invalid_image_ = false;

  // Caches an image's metadata so it can outlive |metadata_decoder_| after all
  // data is received in cases where multiple generators are created.
  std::optional<cc::ImageHeaderMetadata> image_metadata_;

  // Caches frame state information.
  Vector<DeferredFrameData> frame_data_;
  // The number of received/complete frames in |frame_data_|. Note: This is also
  // the index of the first unreceived/incomplete frame in |frame_data_|.
  wtf_size_t received_frame_count_ = 0;
  scoped_refptr<ImageFrameGenerator> frame_generator_;

  // This is set to false when it is known that this image does not contain a
  // gainmap.
  bool might_have_gainmap_ = true;

  // Information about the gainmap image. This is initialized in
  // ActivateLazyGainmapDecoding.
  struct Gainmap {
    // The data for the gainmap. This is a subset of `parkable_image_`.
    scoped_refptr<SegmentReader> data;

    // The rendering parameters for the gainmap.
    SkGainmapInfo info;

    // Metadata read from the gainmap image.
    bool can_decode_yuv = false;
    cc::ImageHeaderMetadata image_metadata;

    // Frame generator for the gainmap image.
    scoped_refptr<ImageFrameGenerator> frame_generator;
  };
  std::unique_ptr<Gainmap> gainmap_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DEFERRED_IMAGE_DECODER_H_

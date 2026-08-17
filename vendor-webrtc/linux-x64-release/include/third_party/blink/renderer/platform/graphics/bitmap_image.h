/*
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2004, 2005, 2006 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) 2008-2009 Torch Mobile, Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BITMAP_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BITMAP_IMAGE_H_

#include <memory>
#include "base/memory/weak_ptr.h"
#include "base/notreached.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink.h"
#include "third_party/blink/renderer/platform/graphics/deferred_image_decoder.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/image-decoders/image_animation.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/size.h"

#include "third_party/skia/include/core/SkRefCnt.h"

namespace blink {

class UseCounter;

class PLATFORM_EXPORT BitmapImage final : public Image {
  friend class BitmapImageTest;
  friend class CrossfadeGeneratedImage;
  friend class GeneratedImage;
  friend class GradientGeneratedImage;
  friend class GraphicsContext;

 public:
  static scoped_refptr<BitmapImage> Create(ImageObserver* observer = nullptr,
                                           bool is_multipart = false) {
    return base::AdoptRef(new BitmapImage(observer, is_multipart));
  }

  ~BitmapImage() override;

  bool IsBitmapImage() const override { return true; }

  bool CurrentFrameHasSingleSecurityOrigin() const override;

  gfx::Size SizeWithConfig(SizeConfig) const override;
  bool GetHotSpot(gfx::Point&) const override;
  String FilenameExtension() const override;
  const AtomicString& MimeType() const override;

  SizeAvailability SetData(scoped_refptr<SharedBuffer> data,
                           bool all_data_received) override;
  SizeAvailability DataChanged(bool all_data_received) override;

  bool IsAllDataReceived() const { return all_data_received_; }
  bool HasColorProfile() const;

  void ResetAnimation() override;
  bool MaybeAnimated() override;

  void SetAnimationPolicy(mojom::blink::ImageAnimationPolicy) override;
  mojom::blink::ImageAnimationPolicy AnimationPolicy() override {
    return animation_policy_;
  }

  scoped_refptr<Image> ImageForDefaultFrame() override;

  // TODO(khushalsagar): These names are bogus, we don't know what the current
  // frame is.
  bool CurrentFrameKnownToBeOpaque() override;
  bool CurrentFrameIsComplete() override;
  bool CurrentFrameIsLazyDecoded() override;
  size_t FrameCount() override;
  PaintImage PaintImageForCurrentFrame() override;
  ImageOrientation CurrentFrameOrientation() const override;

  PaintImage PaintImageForTesting();
  void AdvanceAnimationForTesting() override {
    NOTREACHED() << "Supported only with svgs";
  }
  void SetDecoderForTesting(std::unique_ptr<DeferredImageDecoder> decoder) {
    decoder_ = std::move(decoder);
  }

  // Records the decoded image type in a UseCounter. |use_counter| may be a null
  // pointer.
  void RecordDecodedImageType(UseCounter* use_counter);

 protected:
  bool IsSizeAvailable() override;

  // TODO(khushalsagar): This is only used by MemoryCache to evict images based
  // on whether they are caching decoded data. Since the decodes are already
  // in unlocked discardable in cc/skia, this is unnecessary.
  size_t TotalFrameBytes();

 private:
  enum RepetitionCountStatus : uint8_t {
    kUnknown,    // We haven't checked the source's repetition count.
    kUncertain,  // We have a repetition count, but it might be wrong (some GIFs
                 // have a count after the image data, and will report "loop
                 // once" until all data has been decoded).
    kCertain     // The repetition count is known to be correct.
  };

  BitmapImage(const SkBitmap&, ImageObserver* = nullptr);
  BitmapImage(ImageObserver* = nullptr, bool is_multi_part = false);

  void Draw(cc::PaintCanvas*,
            const cc::PaintFlags&,
            const gfx::RectF& dst_rect,
            const gfx::RectF& src_rect,
            const ImageDrawOptions&) override;

  PaintImage CreatePaintImage();
  void UpdateSize() const;

  // Called to wipe out the entire frame buffer cache and tell the image
  // source to destroy everything; this is used when e.g. we want to free
  // some room in the image cache.
  void DestroyDecodedData() override;

  scoped_refptr<SharedBuffer> Data() override;
  bool HasData() const override;
  size_t DataSize() const override;

  // Notifies observers that the memory footprint has changed.
  void NotifyMemoryChanged();

  int RepetitionCount();

  // Whether we are ready to record UMAs related to the number of bytes in
  // images.
  bool ShouldReportByteSizeUMAs(bool data_now_completely_received);

  std::unique_ptr<DeferredImageDecoder> decoder_;
  mutable gfx::Size size_;  // The size to use for the overall image (will just
                            // be the size of the first image).
  mutable gfx::Size density_corrected_size_;

  // This caches the PaintImage created with the last updated encoded data to
  // ensure re-use of generated decodes. This is cleared each time the encoded
  // data is updated in DataChanged.
  PaintImage cached_frame_;

  // Whether or not we can play animation.
  mojom::blink::ImageAnimationPolicy animation_policy_ =
      blink::mojom::ImageAnimationPolicy::kImageAnimationPolicyAllowed;

  bool all_data_received_ : 1;  // Whether we've received all our data.
  mutable bool have_size_ : 1;  // Whether our |m_size| member variable has the
                                // final overall image size yet.
  mutable bool preferred_size_is_transposed_ : 1;  // Whether the preferred size
                                                   // uses width as height.
  bool size_available_ : 1;     // Whether we can obtain the size of the first
                                // image frame from ImageIO yet.
  bool have_frame_count_ : 1;

  bool default_frame_has_alpha_ : 1;

  RepetitionCountStatus repetition_count_status_;
  int repetition_count_;  // How many total animation loops we should do.  This
                          // will be cAnimationNone if this image type is
                          // incapable of animation.

  size_t frame_count_;

  PaintImage::AnimationSequenceId reset_animation_sequence_id_ = 0;
};

template <>
struct DowncastTraits<BitmapImage> {
  static bool AllowFrom(const Image& image) { return image.IsBitmapImage(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BITMAP_IMAGE_H_

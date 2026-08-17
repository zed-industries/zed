/*
 * Copyright (C) 2006 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) Research In Motion Limited 2009-2010. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_FRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_FRAME_H_

#include <optional>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/notreached.h"
#include "base/time/time.h"
#include "skia/ext/pmcolor_utils.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"
#include "third_party/skia/include/core/SkBitmap.h"
#include "third_party/skia/include/core/SkImage.h"
#include "third_party/skia/include/core/SkPixmap.h"
#include "third_party/skia/include/private/chromium/SkPMColor.h"
#include "ui/gfx/geometry/rect.h"

class SkImage;

namespace blink {

// ImageFrame represents the decoded image data.  This buffer is what all
// decoders write a single frame into.
class PLATFORM_EXPORT ImageFrame final {
  DISALLOW_NEW();

 public:
  enum PixelFormat { kN32, kRGBA_F16 };
  enum Status { kFrameEmpty, kFrameInitialized, kFramePartial, kFrameComplete };
  enum DisposalMethod {
    // If you change the numeric values of these, make sure you audit
    // all users, as some users may cast raw values to/from these
    // constants.
    kDisposeNotSpecified,      // Leave frame in framebuffer
    kDisposeKeep,              // Leave frame in framebuffer
    kDisposeOverwriteBgcolor,  // Clear frame to fully transparent
    kDisposeOverwritePrevious  // Clear frame to previous framebuffer contents
  };
  // Indicates how non-opaque pixels in the current frame rectangle
  // are blended with those in the previous frame.
  // Notes:
  // * GIF always uses 'BlendAtopPreviousFrame'.
  // * WebP also uses the 'BlendAtopBgcolor' option. This is useful for
  //   cases where one wants to transform a few opaque pixels of the
  //   previous frame into non-opaque pixels in the current frame.
  enum AlphaBlendSource {
    // Blend non-opaque pixels atop the corresponding pixels in the
    // initial buffer state (i.e. any previous frame buffer after having
    // been properly disposed).
    kBlendAtopPreviousFrame,

    // Blend non-opaque pixels against fully transparent (i.e. simply
    // overwrite the corresponding pixels).
    kBlendAtopBgcolor,
  };
  typedef uint32_t PixelData;
  typedef uint64_t PixelDataF16;

  ImageFrame();
  ~ImageFrame();

  // The assignment operator reads has_alpha_ (inside SetStatus()) before it
  // sets it (in SetHasAlpha()).  This doesn't cause any problems, since the
  // SetHasAlpha() call ensures all state is set correctly, but it means we
  // need to initialize has_alpha_ to some value before calling the operator
  // lest any tools complain about using an uninitialized value.
  ImageFrame(const ImageFrame& other);

  // For backends which refcount their data, this operator doesn't need to
  // create a new copy of the image data, only increase the ref count.
  ImageFrame& operator=(const ImageFrame& other);

  // These do not touch other metadata, only the raw pixel data.
  void ClearPixelData();
  void ZeroFillPixelData();
  void ZeroFillFrameRect(const gfx::Rect&);

  // Makes this frame have an independent copy of the provided image's
  // pixel data, so that modifications in one frame are not reflected in
  // the other.  Returns whether the copy succeeded.
  bool CopyBitmapData(const ImageFrame&);

  // Moves the bitmap data from the provided frame to this one, leaving the
  // provided frame empty.  Operation is successful only if bitmap data is not
  // marked as done (immutable).  Returns whether the move succeeded.
  bool TakeBitmapDataIfWritable(ImageFrame*);

  // Copies the pixel data at [(start_x, start_y), (end_x, start_y)) to the
  // same X-coordinates on each subsequent row up to but not including
  // end_y.
  void CopyRowNTimes(int start_x, int end_x, int start_y, int end_y) {
    DCHECK(pixel_format_ == kN32);
    DCHECK_LT(start_x, Width());
    DCHECK_LE(end_x, Width());
    DCHECK_LT(start_y, Height());
    DCHECK_LE(end_y, Height());
    const int row_bytes = (end_x - start_x) * sizeof(PixelData);
    const PixelData* const start_addr = GetAddr(start_x, start_y);
    for (int dest_y = start_y + 1; dest_y < end_y; ++dest_y) {
      UNSAFE_TODO(memcpy(GetAddr(start_x, dest_y), start_addr, row_bytes));
    }
  }

  // Allocates space for the pixel data. Must be called before any pixels are
  // written, and should only be called once. The specified color space may be
  // null if and only if color correct rendering is enabled. Returns true if the
  // allocation succeeded.
  bool AllocatePixelData(int new_width, int new_height, sk_sp<SkColorSpace>);

  bool HasAlpha() const { return has_alpha_; }
  PixelFormat GetPixelFormat() const { return pixel_format_; }
  const gfx::Rect& OriginalFrameRect() const { return original_frame_rect_; }
  Status GetStatus() const { return status_; }
  std::optional<base::TimeDelta> Timestamp() const { return timestamp_; }
  base::TimeDelta Duration() const { return duration_; }
  DisposalMethod GetDisposalMethod() const { return disposal_method_; }
  AlphaBlendSource GetAlphaBlendSource() const { return alpha_blend_source_; }
  bool PremultiplyAlpha() const { return premultiply_alpha_; }
  SkBitmap::Allocator* GetAllocator() const { return allocator_; }

  // Returns the bitmap that is the output of decoding.
  const SkBitmap& Bitmap() const { return bitmap_; }

  // Create SkImage from Bitmap() and return it.  This should be called only
  // if frame is complete.  The bitmap is set immutable before creating
  // SkImage to avoid copying bitmap in SkImages::RasterFromBitmap(bitmap_).
  sk_sp<SkImage> FinalizePixelsAndGetImage();

  // Returns true if the pixels changed, but the bitmap has not yet been
  // notified.
  bool PixelsChanged() const { return pixels_changed_; }
  wtf_size_t RequiredPreviousFrameIndex() const {
    return required_previous_frame_index_;
  }
  void SetHasAlpha(bool alpha);
  void SetPixelFormat(PixelFormat format) { pixel_format_ = format; }
  void SetOriginalFrameRect(const gfx::Rect& r) { original_frame_rect_ = r; }
  void SetStatus(Status);
  void SetTimestamp(base::TimeDelta timestamp) { timestamp_ = timestamp; }
  void SetDuration(base::TimeDelta duration) { duration_ = duration; }
  void SetDisposalMethod(DisposalMethod disposal_method) {
    disposal_method_ = disposal_method;
  }
  void SetAlphaBlendSource(AlphaBlendSource alpha_blend_source) {
    alpha_blend_source_ = alpha_blend_source;
  }
  void SetPremultiplyAlpha(bool premultiply_alpha) {
    premultiply_alpha_ = premultiply_alpha;
  }
  void SetMemoryAllocator(SkBitmap::Allocator* allocator) {
    allocator_ = allocator;
  }
  // The pixels_changed flag needs to be set when the raw pixel data was
  // directly modified (e.g. through a pointer or SetRGBA). The flag is usually
  // set after a batch of changes has been made.
  void SetPixelsChanged(bool pixels_changed) {
    pixels_changed_ = pixels_changed;
  }
  void SetRequiredPreviousFrameIndex(wtf_size_t previous_frame_index) {
    required_previous_frame_index_ = previous_frame_index;
  }

  inline PixelData* GetAddr(int x, int y) {
    DCHECK(pixel_format_ == kN32);
    return bitmap_.getAddr32(x, y);
  }

  inline PixelDataF16* GetAddrF16(int x, int y) {
    DCHECK(pixel_format_ == kRGBA_F16);
    SkPixmap pixmap;
    if (!bitmap_.peekPixels(&pixmap)) {
      NOTREACHED();
    }
    return pixmap.writable_addr64(x, y);
  }

  inline void SetRGBA(int x,
                      int y,
                      unsigned r,
                      unsigned g,
                      unsigned b,
                      unsigned a) {
    DCHECK(pixel_format_ == kN32);
    SetRGBA(GetAddr(x, y), r, g, b, a);
  }

  inline void SetRGBA(PixelData* dest,
                      unsigned r,
                      unsigned g,
                      unsigned b,
                      unsigned a) {
    DCHECK(pixel_format_ == kN32);
    if (premultiply_alpha_) {
      SetRGBAPremultiply(dest, r, g, b, a);
    } else {
      *dest = SkPMColorSetARGB(a, r, g, b);
    }
  }

  static inline void SetRGBAPremultiply(PixelData* dest,
                                        unsigned r,
                                        unsigned g,
                                        unsigned b,
                                        unsigned a) {
    enum FractionControl { kRoundFractionControl = 257 * 128 };

    if (a < 255) {
      unsigned alpha = a * 257;
      r = (r * alpha + kRoundFractionControl) >> 16;
      g = (g * alpha + kRoundFractionControl) >> 16;
      b = (b * alpha + kRoundFractionControl) >> 16;
    }

    *dest = SkPMColorSetARGB(a, r, g, b);
  }

  static inline void SetRGBARaw(PixelData* dest,
                                unsigned r,
                                unsigned g,
                                unsigned b,
                                unsigned a) {
    *dest = SkPMColorSetARGB(a, r, g, b);
  }

  // Blend the RGBA pixel provided by |red|, |green|, |blue| and |alpha| over
  // the pixel in |dest|, without premultiplication, and overwrite |dest| with
  // the result.
  static void BlendRGBARaw(PixelData* dest,
                           unsigned red,
                           unsigned green,
                           unsigned blue,
                           unsigned alpha);

  static void BlendRGBARawF16Buffer(PixelDataF16* dst,
                                    PixelDataF16* src,
                                    size_t num_pixels);

  // Blend the pixel in |src| over |dst| and overwrite |src| with the result.
  // This requires |src| and |dst| to not have alpha premultiplied on the rgb
  // channels.
  static void BlendSrcOverDstRaw(PixelData* src, PixelData dst);

  // Blend the RGBA pixel provided by |r|, |g|, |b|, |a| over the pixel in
  // |dest| and overwrite |dest| with the result. Premultiply the pixel values
  // before blending.
  static inline void BlendRGBAPremultiplied(PixelData* dest,
                                            unsigned r,
                                            unsigned g,
                                            unsigned b,
                                            unsigned a) {
    // If the new pixel is completely transparent, no operation is necessary
    // since |dest| contains the background pixel.
    if (a == 0x0) {
      return;
    }

    // If the new pixel is opaque, no need for blending - just write the
    // pixel.
    if (a == 0xFF) {
      SetRGBAPremultiply(dest, r, g, b, a);
      return;
    }

    PixelData src;
    SetRGBAPremultiply(&src, r, g, b, a);
    *dest = skia::BlendSrcOver(src, *dest);
  }

  static void BlendRGBAPremultipliedF16Buffer(PixelDataF16* dst,
                                              PixelDataF16* src,
                                              size_t num_pixels);

  // Blend the pixel in |src| over |dst| and overwrite |src| with the result.
  // This requires |src| and |dst| to be premultiplied already.
  static inline void BlendSrcOverDstPremultiplied(PixelData* src,
                                                  PixelData dst) {
    *src = skia::BlendSrcOver(*src, dst);
  }

  // Notifies the SkBitmap if any pixels changed and resets the flag.
  inline void NotifyBitmapIfPixelsChanged() {
    if (pixels_changed_) {
      bitmap_.notifyPixelsChanged();
    }
    pixels_changed_ = false;
  }

 private:
  int Width() const { return bitmap_.width(); }

  int Height() const { return bitmap_.height(); }

  SkAlphaType ComputeAlphaType() const;

  SkBitmap bitmap_;
  raw_ptr<SkBitmap::Allocator> allocator_ = nullptr;
  bool has_alpha_ = true;
  PixelFormat pixel_format_ = kN32;
  // This will always just be the entire buffer except for GIF or WebP
  // frames whose original rect was smaller than the overall image size.
  gfx::Rect original_frame_rect_;
  Status status_ = kFrameEmpty;
  std::optional<base::TimeDelta> timestamp_;
  base::TimeDelta duration_;
  DisposalMethod disposal_method_ = kDisposeNotSpecified;
  AlphaBlendSource alpha_blend_source_ = kBlendAtopPreviousFrame;
  bool premultiply_alpha_ = true;
  // True if the pixels changed, but the bitmap has not yet been notified.
  bool pixels_changed_ = false;

  // The frame that must be decoded before this frame can be decoded.
  // WTF::kNotFound if this frame doesn't require any previous frame.
  // This is used by ImageDecoder::ClearCacheExceptFrame(), and will never
  // be read for image formats that do not have multiple frames.
  wtf_size_t required_previous_frame_index_ = kNotFound;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_FRAME_H_

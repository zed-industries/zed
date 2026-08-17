// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BITMAP_IMAGE_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BITMAP_IMAGE_METRICS_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class UseCounter;

class PLATFORM_EXPORT BitmapImageMetrics {
  STATIC_ONLY(BitmapImageMetrics);

 public:
  // Values synced with 'DecodedImageType' in
  // src/tools/metrics/histograms/enums.xml. These values are persisted to logs.
  // Entries should not be renumbered and numeric values should never be reused.
  enum class DecodedImageType {
    kUnknown = 0,
    kJPEG = 1,
    kPNG = 2,
    kGIF = 3,
    kWebP = 4,
    kICO = 5,
    kBMP = 6,
    kAVIF = 7,
    kREMOVED_JXL = 8,
    kMaxValue = kREMOVED_JXL,
  };

  // |type| is the return value of ImageDecoder::FilenameExtension().
  static DecodedImageType StringToDecodedImageType(const WTF::String& type);

  // |type| is the return value of ImageDecoder::FilenameExtension().
  static void CountDecodedImageType(const WTF::String& type);
  // |type| is the return value of ImageDecoder::FilenameExtension().
  // |use_counter| may be a null pointer.
  static void CountDecodedImageType(const WTF::String& type,
                                    UseCounter* use_counter);
  // Report the image compression density in 0.01 bits per pixel for an image
  // with a smallest side (width or length) of |image_min_side| and total size
  // in bytes |image_size_bytes|. Only certain image types and minimum image
  // size are reported.
  static void CountDecodedImageDensity(const WTF::String& type,
                                       int image_min_side,
                                       uint64_t density_centi_bpp,
                                       size_t image_size_bytes);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BITMAP_IMAGE_METRICS_H_

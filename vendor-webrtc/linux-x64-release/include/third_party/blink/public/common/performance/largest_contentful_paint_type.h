// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PERFORMANCE_LARGEST_CONTENTFUL_PAINT_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PERFORMANCE_LARGEST_CONTENTFUL_PAINT_TYPE_H_

namespace blink {

// This enum contains the various types a potential LargestContentfulPaint
// candidate entry may have.
// These values are set in PaintTimingDetector, packed into
// page_load_metrics.mojom's LargestContentfulPaintTiming.type and finally
// reported to UKM through UKMPageLoadMetricsObserver.
enum class LargestContentfulPaintType {
  kNone = 0,
  kImage = 1 << 0,
  kText = 1 << 1,
  kAnimatedImage = 1 << 2,
  kVideo = 1 << 3,
  kDataURI = 1 << 4,
  kPNG = 1 << 5,
  kJPG = 1 << 6,
  kWebP = 1 << 7,
  kSVG = 1 << 8,
  kGIF = 1 << 9,
  kAVIF = 1 << 10,

  // kFullViewport is not yet supported.
  kFullViewport = 1 << 11,

  kAfterMouseover = 1 << 12,
  // LCP image is cross-origin with the root document.
  kCrossOrigin = 1 << 13,
};

inline constexpr LargestContentfulPaintType operator&(
    LargestContentfulPaintType a,
    LargestContentfulPaintType b) {
  return static_cast<LargestContentfulPaintType>(static_cast<uint32_t>(a) &
                                                 static_cast<uint32_t>(b));
}

inline constexpr LargestContentfulPaintType operator|(
    LargestContentfulPaintType a,
    LargestContentfulPaintType b) {
  return static_cast<LargestContentfulPaintType>(static_cast<uint32_t>(a) |
                                                 static_cast<uint32_t>(b));
}

inline LargestContentfulPaintType& operator&=(LargestContentfulPaintType& a,
                                              LargestContentfulPaintType b) {
  return a = a & b;
}

inline LargestContentfulPaintType& operator|=(LargestContentfulPaintType& a,
                                              LargestContentfulPaintType b) {
  return a = a | b;
}

inline constexpr uint64_t LargestContentfulPaintTypeToUKMFlags(
    LargestContentfulPaintType type) {
  return static_cast<uint64_t>(type);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PERFORMANCE_LARGEST_CONTENTFUL_PAINT_TYPE_H_

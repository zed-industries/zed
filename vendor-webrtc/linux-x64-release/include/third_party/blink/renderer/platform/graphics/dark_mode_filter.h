// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_FILTER_H_

#include <memory>
#include <optional>

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/graphics/dark_mode_settings.h"
#include "third_party/blink/renderer/platform/graphics/dark_mode_types.h"
#include "third_party/blink/renderer/platform/platform_export.h"

class SkPixmap;

namespace cc {
class ColorFilter;
}

namespace blink {

class DarkModeColorClassifier;
class DarkModeImageClassifier;
class DarkModeColorFilter;
class DarkModeInvertedColorCache;
class Image;

class PLATFORM_EXPORT DarkModeFilter {
 public:
  // Dark mode is disabled by default. Enable it by calling UpdateSettings()
  // with a mode other than DarkMode::kOff.
  explicit DarkModeFilter(const DarkModeSettings& settings);
  ~DarkModeFilter();

  enum class ElementRole {
    kForeground,
    kListSymbol,
    kBackground,
    kSVG,
    kBorder,
    kSelection
  };
  enum class ImageType { kNone, kIcon, kSeparator, kPhoto };

  SkColor4f InvertColorIfNeeded(const SkColor4f& color,
                                ElementRole element_role);
  SkColor4f InvertColorIfNeeded(const SkColor4f& color,
                                ElementRole role,
                                const SkColor4f& contrast_background);

  std::optional<cc::PaintFlags> ApplyToFlagsIfNeeded(
      const cc::PaintFlags& flags,
      ElementRole role,
      SkColor4f contrast_background);

  size_t GetInvertedColorCacheSizeForTesting();

  // Decides whether to apply dark mode or not.
  bool ShouldApplyFilterToImage(ImageType type) const;

  // Returns dark mode color filter based on the classification done on
  // |pixmap|. The image cannot be classified if pixmap is empty or |src| is
  // empty or |src| is larger than pixmap bounds. This function should be called
  // only if image policy is set to DarkModeImagePolicy::kFilterSmart and image
  // is classified as ImageType::kIcon or kSeparator. This API is thread-safe.
  sk_sp<cc::ColorFilter> GenerateImageFilter(const SkPixmap& pixmap,
                                             const SkIRect& src) const;

  void ApplyFilterToImage(Image* image,
                          cc::PaintFlags* flags,
                          const SkRect& src);

 private:
  struct ImmutableData {
    explicit ImmutableData(const DarkModeSettings& settings);

    DarkModeSettings settings;
    std::unique_ptr<DarkModeColorClassifier> foreground_classifier;
    std::unique_ptr<DarkModeColorClassifier> background_classifier;
    std::unique_ptr<DarkModeImageClassifier> image_classifier;
    std::unique_ptr<DarkModeColorFilter> color_filter;
    sk_sp<cc::ColorFilter> image_filter;
  };

  SkColor4f AdjustDarkenColor(const SkColor4f& color,
                              DarkModeFilter::ElementRole role,
                              const SkColor4f& contrast_background);

  bool ShouldApplyToColor(const SkColor4f& color, ElementRole role);

  // Returns dark mode color filter for images. This function should be called
  // only if image policy is set to DarkModeImagePolicy::kFilterAll or image is
  // classified as ImageType::kIcon or kSeparator. This API is thread-safe.
  sk_sp<cc::ColorFilter> GetImageFilter() const;

  DarkModeImagePolicy GetDarkModeImagePolicy() const;

  // This is read-only data and is thread-safe.
  const ImmutableData immutable_;

  // Following member is used for color classifications and is not thread-safe.
  // TODO(prashant.n): Move cache out of dark mode filter.
  std::unique_ptr<DarkModeInvertedColorCache> inverted_color_cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_FILTER_H_

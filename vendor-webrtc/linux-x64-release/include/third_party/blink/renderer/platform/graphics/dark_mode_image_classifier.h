// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_IMAGE_CLASSIFIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_IMAGE_CLASSIFIER_H_

#include <optional>
#include <vector>

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/platform/graphics/dark_mode_settings.h"
#include "third_party/blink/renderer/platform/graphics/dark_mode_types.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkPixmap.h"
#include "third_party/skia/include/core/SkRect.h"

namespace blink {

FORWARD_DECLARE_TEST(DarkModeImageClassifierTest, BlockSamples);
FORWARD_DECLARE_TEST(DarkModeImageClassifierTest, FeaturesAndClassification);

// This class is not threadsafe as the cache used for storing classification
// results is not threadsafe. So it can be used only in blink main thread.
class PLATFORM_EXPORT DarkModeImageClassifier {
 public:
  explicit DarkModeImageClassifier(
      DarkModeImageClassifierPolicy image_classifier_policy);
  ~DarkModeImageClassifier();

  struct Features {
    // True if the image is in color, false if it is grayscale.
    bool is_colorful;

    // Ratio of the number of bucketed colors used in the image to all
    // possibilities. Color buckets are represented with 4 bits per color
    // channel.
    float color_buckets_ratio;

    // How much of the image is transparent or considered part of the
    // background.
    float transparency_ratio;
    float background_ratio;
  };

  DarkModeResult Classify(const SkPixmap& pixmap, const SkIRect& src) const;

 private:
  DarkModeResult ClassifyWithFeatures(const Features& features) const;
  DarkModeResult ClassifyUsingDecisionTree(const Features& features) const;

  enum class ColorMode { kColor = 0, kGrayscale = 1 };

  std::optional<Features> GetFeatures(const SkPixmap& pixmap,
                                      const SkIRect& src) const;
  // Extracts a sample set of pixels (|sampled_pixels|), |transparency_ratio|,
  // and |background_ratio|.
  void GetSamples(const SkPixmap& pixmap,
                  const SkIRect& src,
                  std::vector<SkColor>* sampled_pixels,
                  float* transparency_ratio,
                  float* background_ratio) const;
  // Gets the |required_samples_count| for a specific |block| of the given
  // pixmap, and returns |sampled_pixels| and |transparent_pixels_count|.
  void GetBlockSamples(const SkPixmap& pixmap,
                       const SkIRect& block,
                       const int required_samples_count,
                       std::vector<SkColor>* sampled_pixels,
                       int* transparent_pixels_count) const;

  // Given |sampled_pixels|, |transparency_ratio|, and |background_ratio| for an
  // image, computes and returns the features required for classification.
  Features ComputeFeatures(const std::vector<SkColor>& sampled_pixels,
                           const float transparency_ratio,
                           const float background_ratio) const;

  // Receives sampled pixels and color mode, and returns the ratio of color
  // buckets count to all possible color buckets. If image is in color, a color
  // bucket is a 4 bit per channel representation of each RGB color, and if it
  // is grayscale, each bucket is a 4 bit representation of luminance.
  float ComputeColorBucketsRatio(const std::vector<SkColor>& sampled_pixels,
                                 const ColorMode color_mode) const;

  const DarkModeImageClassifierPolicy image_classifier_policy_;

  FRIEND_TEST_ALL_PREFIXES(DarkModeImageClassifierTest, BlockSamples);
  FRIEND_TEST_ALL_PREFIXES(DarkModeImageClassifierTest,
                           FeaturesAndClassification);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_IMAGE_CLASSIFIER_H_

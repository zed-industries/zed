// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_FONT_FEATURES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_FONT_FEATURES_H_

#include <optional>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

struct hb_feature_t;

namespace blink {

class FontDescription;

//
// Represents an OpenType font feature tag.
//
// This struct is compatible with `hb_tag_t`.
//
struct PLATFORM_EXPORT FontFeatureTag {
  constexpr FontFeatureTag() = default;
  constexpr FontFeatureTag(uint32_t tag) : tag(tag) {}
  constexpr FontFeatureTag(uint8_t c1, uint8_t c2, uint8_t c3, uint8_t c4)
      : tag((((((static_cast<uint32_t>(c1) << 8) | c2) << 8) | c3) << 8) | c4) {
  }

  operator uint32_t() const { return tag; }

  uint32_t tag = 0;
};

//
// Represents an OpenType font feature with its value.
//
struct PLATFORM_EXPORT FontFeatureValue : public FontFeatureTag {
  uint32_t value = 0;
};

//
// Represents an OpenType font feature with its value and text range.
//
// This struct has the same size and layout as `hb_feature_t`.
//
struct PLATFORM_EXPORT FontFeatureRange : public FontFeatureValue {
  // The size produced by `FromFontDescription()` for the initial style.
  static constexpr wtf_size_t kInitialSize = 1;

  // Initialize the list from |FontDescription|.
  template <wtf_size_t InlineCapacity>
  static void FromFontDescription(const FontDescription&,
                                  Vector<FontFeatureRange, InlineCapacity>&);

  // True if the list is for the initial style.
  static bool IsInitial(base::span<const FontFeatureRange>);

  uint32_t start = 0;
  uint32_t end = static_cast<uint32_t>(-1);
};

//
// Represents a list of `FontFeatureRange`.
//
class PLATFORM_EXPORT FontFeatures {
 public:
  FontFeatures() = default;
  explicit FontFeatures(base::span<const FontFeatureRange> features)
      : features_(features) {}

  // Initialize the list from |Font|.
  void Initialize(const FontDescription&);

  wtf_size_t size() const { return features_.size(); }
  bool IsEmpty() const { return features_.empty(); }

  const FontFeatureRange& operator[](wtf_size_t i) const {
    return features_[i];
  }
  explicit operator base::span<const FontFeatureRange>() { return features_; }
  const hb_feature_t* ToHarfBuzzData() const;

  std::optional<uint32_t> FindValueForTesting(uint32_t tag) const;

  void Reserve(wtf_size_t new_capacity) { features_.reserve(new_capacity); }

  void Append(const FontFeatureRange& feature) { features_.push_back(feature); }
  void Insert(const FontFeatureRange& feature) {
    features_.push_front(feature);
  }
  void AppendVector(const FontFeatures& features) {
    features_.AppendVector(features.features_);
  }

  void EraseAt(wtf_size_t position, wtf_size_t length) {
    features_.EraseAt(position, length);
  }
  void Shrink(wtf_size_t size) { features_.Shrink(size); }

  using FeatureArray = Vector<FontFeatureRange, 6>;
  using const_iterator = FeatureArray::const_iterator;
  const_iterator begin() const { return features_.begin(); }
  const_iterator end() const { return features_.end(); }

 private:
  FeatureArray features_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_FONT_FEATURES_H_

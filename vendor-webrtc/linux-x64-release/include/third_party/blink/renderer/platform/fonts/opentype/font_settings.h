// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_FONT_SETTINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_FONT_SETTINGS_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

PLATFORM_EXPORT uint32_t AtomicStringToFourByteTag(const AtomicString& tag);
PLATFORM_EXPORT AtomicString FourByteTagToAtomicString(uint32_t tag);

template <typename T>
class FontTagValuePair {
  DISALLOW_NEW();

 public:
  FontTagValuePair(uint32_t tag, T value) : tag_(tag), value_(value) {
    // ensure tag is either valid or zero
    DCHECK(tag == 0 ||
           (tag & 0xff000000) < 0x7f000000 &&
               (tag & 0xff000000) >= 0x20000000 &&
               (tag & 0xff0000) < 0x7f0000 && (tag & 0xff0000) >= 0x200000 &&
               (tag & 0xff00) < 0x7f00 && (tag & 0xff00) >= 0x2000 &&
               (tag & 0xff) < 0x7f && (tag & 0xff) >= 0x20);
  }
  FontTagValuePair(const AtomicString& tag, T value)
      : tag_(AtomicStringToFourByteTag(tag)), value_(value) {
    // ensure tag is valid
    DCHECK((tag_ & 0xff000000) < 0x7f000000 &&
           (tag_ & 0xff000000) >= 0x20000000 && (tag_ & 0xff0000) < 0x7f0000 &&
           (tag_ & 0xff0000) >= 0x200000 && (tag_ & 0xff00) < 0x7f00 &&
           (tag_ & 0xff00) >= 0x2000 && (tag_ & 0xff) < 0x7f &&
           (tag_ & 0xff) >= 0x20);
  }
  bool operator==(const FontTagValuePair& other) const {
    return tag_ == other.tag_ && value_ == other.value_;
  }

  uint32_t Tag() const { return tag_; }
  AtomicString TagString() const { return FourByteTagToAtomicString(tag_); }
  T Value() const { return value_; }

 private:
  uint32_t tag_;
  T value_;
};

template <typename T>
class FontSettings {
 public:
  FontSettings(const FontSettings&) = delete;
  FontSettings& operator=(const FontSettings&) = delete;

  void Append(const T& feature) { list_.push_back(feature); }
  wtf_size_t size() const { return list_.size(); }
  const T& operator[](wtf_size_t index) const { return list_[index]; }
  const T& at(wtf_size_t index) const { return list_.at(index); }
  bool operator==(const FontSettings& other) const {
    return list_ == other.list_;
  }
  bool operator!=(const FontSettings& other) const { return !(*this == other); }
  String ToString() const {
    StringBuilder builder;
    wtf_size_t num_features = size();
    for (wtf_size_t i = 0; i < num_features; ++i) {
      if (i > 0)
        builder.Append(",");
      builder.Append(at(i).TagString());
      builder.Append("=");
      builder.AppendNumber(at(i).Value());
    }
    return builder.ToString();
  }

  bool FindPair(uint32_t tag, T* found_pair) const {
    if (!found_pair)
      return false;

    for (auto& pair : list_) {
      if (pair.Tag() == tag) {
        *found_pair = pair;
        return true;
      }
    }
    return false;
  }

  Vector<T, 0>::const_iterator begin() const { return list_.begin(); }
  Vector<T, 0>::const_iterator end() const { return list_.end(); }
  Vector<T, 0>::iterator begin() { return list_.begin(); }
  Vector<T, 0>::iterator end() { return list_.end(); }

 protected:
  FontSettings() = default;

 private:
  Vector<T, 0> list_;
};

using FontFeature = FontTagValuePair<int>;
using FontVariationAxis = FontTagValuePair<float>;

class PLATFORM_EXPORT FontFeatureSettings
    : public FontSettings<FontFeature>,
      public RefCounted<FontFeatureSettings> {
 public:
  static scoped_refptr<FontFeatureSettings> Create() {
    return base::AdoptRef(new FontFeatureSettings());
  }

  FontFeatureSettings(const FontFeatureSettings&) = delete;
  FontFeatureSettings& operator=(const FontFeatureSettings&) = delete;

 private:
  FontFeatureSettings() = default;
};

class PLATFORM_EXPORT FontVariationSettings
    : public FontSettings<FontVariationAxis>,
      public RefCounted<FontVariationSettings> {
 public:
  static scoped_refptr<FontVariationSettings> Create() {
    return base::AdoptRef(new FontVariationSettings());
  }

  FontVariationSettings(const FontVariationSettings&) = delete;
  FontVariationSettings& operator=(const FontVariationSettings&) = delete;

  unsigned GetHash() const;

 private:
  FontVariationSettings() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_FONT_SETTINGS_H_

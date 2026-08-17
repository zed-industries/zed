// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FALLBACK_LIST_COMPOSITE_KEY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FALLBACK_LIST_COMPOSITE_KEY_H_

#include "third_party/blink/renderer/platform/fonts/alternate_font_family.h"
#include "third_party/blink/renderer/platform/fonts/font_cache_key.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"

namespace blink {

class FontDescription;

// Cache key representing a font description and font fallback list combination
// as passed into shaping. Used to look up an applicable ShapeCache instance
// from the global FontCache.
// TODO(eae,drott): Ideally this should be replaced by a combination of
// FontDescription and CSSFontSelector.
struct FallbackListCompositeKey {
  DISALLOW_NEW();

 public:
  FallbackListCompositeKey(const FontDescription& font_description)
      : hash_(font_description.StyleHashWithoutFamilyList() << 1),
        computed_size_(font_description.ComputedSize()),
        letter_spacing_(font_description.LetterSpacing()),
        word_spacing_(font_description.WordSpacing()),
        bitmap_fields_(font_description.BitmapFields()),
        auxiliary_bitmap_fields_(font_description.AuxiliaryBitmapFields()) {
    const FontFamily* current_family = &font_description.Family();
    while (current_family) {
      if (!current_family->FamilyName().empty()) {
        Add(CaseFoldingHash::GetHash(AdjustFamilyNameToAvoidUnsupportedFonts(
            current_family->FamilyName())));
        // Discriminate between quoted generic names, referring to a named
        // family, vs unquoted referring to the generic. The name itself is
        // stored identically in both cases.
        WTF::AddIntToHash(hash_, current_family->FamilyIsGeneric() ? 2u : 1u);
      }
      current_family = current_family->Next();
    }
  }
  FallbackListCompositeKey()
      : hash_(0),
        computed_size_(0),
        letter_spacing_(0),
        word_spacing_(0),
        bitmap_fields_(0),
        auxiliary_bitmap_fields_(0) {}
  FallbackListCompositeKey(WTF::HashTableDeletedValueType)
      : hash_(kDeletedValueHash),
        computed_size_(0),
        letter_spacing_(0),
        word_spacing_(0),
        bitmap_fields_(0),
        auxiliary_bitmap_fields_(0) {}

  void Add(unsigned key) {
    font_cache_keys_.push_back(key);
    // Djb2 with the first bit reserved for deleted.
    hash_ = (((hash_ << 5) + hash_) + key) << 1;
  }

  unsigned GetHash() const { return hash_; }

  bool operator==(const FallbackListCompositeKey& other) const {
    return hash_ == other.hash_ && computed_size_ == other.computed_size_ &&
           letter_spacing_ == other.letter_spacing_ &&
           word_spacing_ == other.word_spacing_ &&
           bitmap_fields_ == other.bitmap_fields_ &&
           auxiliary_bitmap_fields_ == other.auxiliary_bitmap_fields_ &&
           font_cache_keys_ == other.font_cache_keys_;
  }

  bool IsHashTableDeletedValue() const { return hash_ == kDeletedValueHash; }

 private:
  static const unsigned kDeletedValueHash = 1;
  Vector<unsigned> font_cache_keys_;
  unsigned hash_;

  float computed_size_;
  float letter_spacing_;
  float word_spacing_;
  unsigned bitmap_fields_;
  unsigned auxiliary_bitmap_fields_;
};

struct FallbackListCompositeKeyTraits
    : WTF::SimpleClassHashTraits<FallbackListCompositeKey> {
  static unsigned GetHash(const FallbackListCompositeKey& key) {
    return key.GetHash();
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FALLBACK_LIST_COMPOSITE_KEY_H_
